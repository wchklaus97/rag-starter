#!/usr/bin/env python3
"""
Refresh embedding model pricing and metadata from OpenRouter into rag_model_site/data/models.json.

Requires: OPENROUTER_API_KEY in the environment (same key you use for OpenRouter API).

Uses: GET https://openrouter.ai/api/v1/embeddings/models

Preserves: ui, recommendations, testingPlan, and per-model bilingual fields (strengths, bestFor,
longNote, tags) from the existing JSON. Updates: pricePerMillionInputTokens, outputPricePerMillionTokens,
provider (from id prefix if missing), sourceUrls, and schema version metadata.

Optional: --include-all-api-models appends any API embedding id not already listed, with tag
openrouterCatalog and minimal bilingual stubs (for a full browseable catalog on the static site).
"""

from __future__ import annotations

import argparse
import json
import os
import sys
import urllib.error
import urllib.request
from datetime import datetime, timezone
from pathlib import Path

OPENROUTER_EMBEDDINGS_MODELS = "https://openrouter.ai/api/v1/embeddings/models"
ROOT = Path(__file__).resolve().parent.parent
DEFAULT_OUT = ROOT / "rag_model_site" / "data" / "models.json"
MD_ROOT = ROOT / "OPENROUTER_EMBEDDING_MODELS.md"
MD_DOCS = ROOT / "rag_model_site" / "OPENROUTER_EMBEDDING_MODELS.md"


def _fetch_json(url: str, api_key: str) -> dict:
    req = urllib.request.Request(
        url,
        headers={
            "Authorization": f"Bearer {api_key}",
            "HTTP-Referer": "https://github.com",  # OpenRouter may expect a referer
            "X-Title": "rag-starter-model-collector",
        },
    )
    with urllib.request.urlopen(req, timeout=60) as resp:
        return json.loads(resp.read().decode())


def _parse_price_per_million(pricing: dict) -> tuple[float, float]:
    """
    Return (input_per_1M, output_per_1M) in USD.
    OpenRouter public pricing often uses per-token values; for embeddings, prompt is the input.
    Heuristic: values >= 0.25 are likely already per-1M; very small values are per-token.
    """
    p = pricing.get("prompt")
    c = pricing.get("completion")
    if p is None and c is None:
        return 0.0, 0.0

    def to_per_million(x) -> float:
        if x is None:
            return 0.0
        v = float(x)
        if v == 0:
            return 0.0
        # per-token (e.g. 2e-8) -> per million
        if abs(v) < 0.01:
            return v * 1_000_000
        return v

    return to_per_million(p), to_per_million(c)


def _provider_from_id(model_id: str) -> str:
    if "/" in model_id:
        return model_id.split("/")[0].replace("-", " ").title()
    return "Unknown"


def _source_url(model: dict) -> str:
    slug = model.get("canonical_slug") or model.get("id", "")
    if not slug:
        return "https://openrouter.ai/collections/embedding-models"
    return f"https://openrouter.ai/{slug}"


def _build_api_index(data: list[dict]) -> dict[str, dict]:
    return {m["id"]: m for m in data if isinstance(m, dict) and m.get("id")}


def merge_models(curated: list[dict], api_by_id: dict[str, dict], strict: bool) -> list[dict]:
    out: list[dict] = []
    for row in curated:
        mid = row.get("id")
        if not mid:
            continue
        api = api_by_id.get(mid)
        if not api and strict:
            print(f"Warning: {mid!r} not found in API list; keeping curated values only", file=sys.stderr)
        if api:
            p_in, p_out = _parse_price_per_million(api.get("pricing") or {})
            if p_in > 0:
                row = {**row, "pricePerMillionInputTokens": p_in}
            if p_out > 0:
                row = {**row, "outputPricePerMillionTokens": p_out}
            row.setdefault("provider", _provider_from_id(mid))
            details = (api.get("links") or {}).get("details")
            primary = _source_url(api)
            urls: list = []
            if primary:
                urls.append(primary)
            if details and details not in urls:
                urls.append(details)
            if not urls:
                urls = list(row.get("sourceUrls") or [])
            if urls:
                row["sourceUrls"] = urls
        out.append(row)
    return out


def _api_blurb_en(api: dict) -> str:
    """Best-effort English blurb from OpenRouter listing (shape varies)."""
    for key in ("description", "name"):
        v = api.get(key)
        if isinstance(v, str) and v.strip():
            return v.strip()[:800]
    return ""


def minimal_row_from_api(api: dict) -> dict:
    """Hand-written bilingual fields omitted: safe defaults for browse-only catalog rows."""
    mid = api.get("id")
    if not mid:
        raise ValueError("api model missing id")
    p_in, p_out = _parse_price_per_million(api.get("pricing") or {})
    details = (api.get("links") or {}).get("details")
    primary = _source_url(api)
    urls: list = []
    if primary:
        urls.append(primary)
    if details and details not in urls:
        urls.append(details)
    blurb = _api_blurb_en(api)
    strength_en = (
        blurb
        if blurb
        else "Imported from OpenRouter GET /v1/embeddings/models. Edit strengths, bestFor, and longNote in models.json for richer copy."
    )
    strength_zh = (
        blurb
        if blurb
        else "來自 OpenRouter GET /v1/embeddings/models。可在 models.json 編寫更完整的優勢與說明。"
    )
    strength_hans = (
        blurb
        if blurb
        else "来自 OpenRouter GET /v1/embeddings/models。可在 models.json 补充更完整说明。"
    )
    bf_en = "Retrieval / RAG via OpenRouter embeddings API."
    bf_zh = "透過 OpenRouter 嵌入 API 做檢索／RAG。"
    bf_hans = "通过 OpenRouter 嵌入 API 做检索／RAG。"
    row: dict = {
        "id": mid,
        "provider": _provider_from_id(mid),
        "pricePerMillionInputTokens": float(p_in) if p_in > 0 else 0.0,
        "outputPricePerMillionTokens": float(p_out) if p_out > 0 else 0.0,
        "tags": ["openrouterCatalog"],
        "strengths": {"en": strength_en, "zh": strength_zh, "zh_hans": strength_hans},
        "bestFor": {"en": bf_en, "zh": bf_zh, "zh_hans": bf_hans},
        "longNote": {
            "en": "Catalog-only row. Compare pricing on OpenRouter before production.",
            "zh": "僅目錄列。正式使用前請在 OpenRouter 確認價格與條款。",
            "zh_hans": "仅目录行。正式使用前请在 OpenRouter 确认价格和条款。",
        },
        "sourceUrls": urls or ["https://openrouter.ai/collections/embedding-models"],
    }
    return row


def merge_models_with_optional_catalog(
    curated: list[dict],
    api_by_id: dict[str, dict],
    strict: bool,
    include_all_api_models: bool,
) -> list[dict]:
    base = merge_models(curated, api_by_id, strict=strict)
    if not include_all_api_models:
        return base
    seen = {r["id"] for r in base if r.get("id")}
    extras: list[dict] = []
    for mid in sorted(api_by_id.keys()):
        if mid in seen:
            continue
        extras.append(minimal_row_from_api(api_by_id[mid]))
    if extras:
        print(
            f"Appending {len(extras)} API-only embedding model(s) (tag: openrouterCatalog).",
            file=sys.stderr,
        )
    return base + extras


def main() -> int:
    parser = argparse.ArgumentParser(description="Refresh OpenRouter embedding data into models.json")
    parser.add_argument(
        "--output",
        type=Path,
        default=DEFAULT_OUT,
        help="Path to models.json (default: rag_model_site/data/models.json)",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Fetch and print a summary; do not write the file",
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="Warn when a curated model id is missing from the API",
    )
    parser.add_argument(
        "--no-sync-md",
        action="store_true",
        help="Do not copy OPENROUTER_EMBEDDING_MODELS.md from repo root to rag_model_site/",
    )
    parser.add_argument(
        "--include-all-api-models",
        action="store_true",
        help="Append every embedding model returned by the API that is not already in models.json "
        "(tag: openrouterCatalog; minimal bilingual stubs).",
    )
    args = parser.parse_args()
    output, dry, strict, no_sync, include_all = (
        args.output,
        args.dry_run,
        args.strict,
        args.no_sync_md,
        args.include_all_api_models,
    )

    api_key = os.environ.get("OPENROUTER_API_KEY", "").strip()
    if not api_key:
        print("Error: set OPENROUTER_API_KEY in the environment.", file=sys.stderr)
        return 1

    try:
        res = _fetch_json(OPENROUTER_EMBEDDINGS_MODELS, api_key)
    except urllib.error.HTTPError as e:
        print(f"HTTP error: {e.code} {e.reason}", file=sys.stderr)
        return 1
    except urllib.error.URLError as e:
        print(f"Request failed: {e}", file=sys.stderr)
        return 1

    data = res.get("data") or []
    api_by_id = _build_api_index(data)
    if not data:
        print("Warning: API returned no models", file=sys.stderr)

    if not output.is_file():
        print(f"Error: {output} not found. Create a base file first.", file=sys.stderr)
        return 1

    payload = json.loads(output.read_text(encoding="utf-8"))
    models = payload.get("models") or []
    payload["models"] = merge_models_with_optional_catalog(
        models, api_by_id, strict=strict, include_all_api_models=include_all
    )
    payload["version"] = payload.get("version", 1)
    payload["updated"] = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    if include_all:
        payload["sourceNote"] = (
            "Refreshed from OpenRouter GET /v1/embeddings/models; includes API-only rows tagged "
            "openrouterCatalog (minimal copy). Hand-maintained fields preserved for curated models."
        )
    else:
        payload["sourceNote"] = (
            "Refreshed from OpenRouter GET /v1/embeddings/models. Bilingual copy is hand-maintained in this file."
        )

    if not no_sync and MD_ROOT.is_file() and not dry:
        MD_DOCS.write_text(MD_ROOT.read_text(encoding="utf-8"), encoding="utf-8")
        print(f"Synced {MD_ROOT.name} -> {MD_DOCS.relative_to(ROOT)}")

    if dry:
        print(json.dumps(payload, indent=2)[:8000])
        if len(json.dumps(payload)) > 8000:
            print("\n... (truncated; full JSON would be written on write)")
        return 0

    output.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"Wrote {output} ({len(payload['models'])} models)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
