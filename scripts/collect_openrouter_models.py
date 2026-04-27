#!/usr/bin/env python3
"""
Refresh embedding model pricing and metadata from OpenRouter into rag_model_site/data/models.json.

Requires: OPENROUTER_API_KEY in the environment (same key you use for OpenRouter API).

Uses: GET https://openrouter.ai/api/v1/embeddings/models

Preserves: ui, recommendations, testingPlan, and per-model bilingual fields (strengths, bestFor,
longNote, tags) from the existing JSON. Updates: pricePerMillionInputTokens, outputPricePerMillionTokens,
provider (from id prefix if missing), sourceUrls, and schema version metadata.
"""

from __future__ import annotations

import argparse
import json
import os
import sys
import urllib.error
import urllib.request
from datetime import date
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
    args = parser.parse_args()
    output, dry, strict, no_sync = args.output, args.dry_run, args.strict, args.no_sync_md

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
    payload["models"] = merge_models(models, api_by_id, strict=strict)
    payload["version"] = payload.get("version", 1)
    payload["updated"] = str(date.today())
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
