#!/usr/bin/env python3
"""
Local OpenRouter embeddings proxy for the field guide “try embedding” playground.

- Reads OPENROUTER_API_KEY from the environment (never put keys in static HTML).
- Binds to 127.0.0.1 by default so it is not reachable from other machines.
- POST /api/embed JSON: { "model": "openai/text-embedding-3-small", "input": "hello" }
  -> forwards to https://openrouter.ai/api/v1/embeddings
- Response trims the embedding to stats + a short preview (avoid multi-megabyte JSON).

Usage:
  export OPENROUTER_API_KEY="sk-or-..."
  uv run python embed_proxy/openrouter_embed_proxy.py

Then open rag_model_site/embed-playground.html via a local static server and set the proxy URL
to http://127.0.0.1:8790 (default in the page).
"""
from __future__ import annotations

import json
import os
import sys
import urllib.error
import urllib.request
from http import HTTPStatus
from http.server import BaseHTTPRequestHandler, HTTPServer


OPENROUTER_EMBEDDINGS = "https://openrouter.ai/api/v1/embeddings"
DEFAULT_HOST = "127.0.0.1"
DEFAULT_PORT = 8790
MAX_INPUT_CHARS = 12_000
PREVIEW_DIMS = 8


def _json_response(handler: BaseHTTPRequestHandler, status: int, body: dict) -> None:
    data = json.dumps(body).encode("utf-8")
    handler.send_response(status)
    handler.send_header("Content-Type", "application/json; charset=utf-8")
    handler.send_header("Content-Length", str(len(data)))
    handler.send_header("Access-Control-Allow-Origin", "*")
    handler.end_headers()
    handler.wfile.write(data)


class Handler(BaseHTTPRequestHandler):
    server_version = "OpenRouterEmbedProxy/1.0"

    def log_message(self, fmt: str, *args) -> None:
        print(f"[{self.address_string()}] {fmt % args}", file=sys.stderr)

    def do_OPTIONS(self) -> None:
        self.send_response(HTTPStatus.NO_CONTENT)
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        self.end_headers()

    def do_GET(self) -> None:
        if self.path == "/health":
            _json_response(self, HTTPStatus.OK, {"ok": True, "service": "openrouter-embed-proxy"})
            return
        _json_response(self, HTTPStatus.NOT_FOUND, {"ok": False, "error": "not_found"})

    def do_POST(self) -> None:
        if self.path != "/api/embed":
            _json_response(self, HTTPStatus.NOT_FOUND, {"ok": False, "error": "not_found"})
            return
        api_key = os.environ.get("OPENROUTER_API_KEY", "").strip()
        if not api_key:
            _json_response(
                self,
                HTTPStatus.SERVICE_UNAVAILABLE,
                {"ok": False, "error": "proxy missing OPENROUTER_API_KEY"},
            )
            return
        try:
            length = int(self.headers.get("Content-Length", "0"))
        except ValueError:
            length = 0
        if length <= 0 or length > 1_000_000:
            _json_response(self, HTTPStatus.BAD_REQUEST, {"ok": False, "error": "invalid body"})
            return
        raw = self.rfile.read(length)
        try:
            payload = json.loads(raw.decode("utf-8"))
        except json.JSONDecodeError:
            _json_response(self, HTTPStatus.BAD_REQUEST, {"ok": False, "error": "invalid json"})
            return
        model = str(payload.get("model") or "").strip()
        inp = payload.get("input")
        if not model or not isinstance(inp, str) or not inp.strip():
            _json_response(
                self,
                HTTPStatus.BAD_REQUEST,
                {"ok": False, "error": "expected JSON { model, input } with non-empty strings"},
            )
            return
        if len(inp) > MAX_INPUT_CHARS:
            _json_response(
                self,
                HTTPStatus.BAD_REQUEST,
                {
                    "ok": False,
                    "error": f"input too long (max {MAX_INPUT_CHARS} chars)",
                },
            )
            return

        body = json.dumps({"model": model, "input": inp}).encode("utf-8")
        req = urllib.request.Request(
            OPENROUTER_EMBEDDINGS,
            data=body,
            method="POST",
            headers={
                "Authorization": f"Bearer {api_key}",
                "Content-Type": "application/json",
                "HTTP-Referer": "https://github.com",
                "X-Title": "rag-starter-embed-proxy",
            },
        )
        try:
            with urllib.request.urlopen(req, timeout=120) as resp:
                upstream = json.loads(resp.read().decode("utf-8"))
        except urllib.error.HTTPError as e:
            err_body = e.read().decode("utf-8", errors="replace")[:4000]
            _json_response(
                self,
                HTTPStatus.BAD_GATEWAY,
                {"ok": False, "error": "openrouter_http_error", "status": e.code, "detail": err_body},
            )
            return
        except urllib.error.URLError as e:
            _json_response(
                self,
                HTTPStatus.BAD_GATEWAY,
                {"ok": False, "error": "openrouter_network", "detail": str(e.reason)},
            )
            return

        data = upstream.get("data") if isinstance(upstream, dict) else None
        if not data or not isinstance(data, list):
            _json_response(
                self,
                HTTPStatus.BAD_GATEWAY,
                {"ok": False, "error": "unexpected_openrouter_shape"},
            )
            return
        first = data[0] if data else {}
        emb = first.get("embedding") if isinstance(first, dict) else None
        if not isinstance(emb, list):
            _json_response(
                self,
                HTTPStatus.BAD_GATEWAY,
                {"ok": False, "error": "missing_embedding"},
            )
            return

        preview = emb[:PREVIEW_DIMS] if len(emb) >= PREVIEW_DIMS else emb
        out = {
            "ok": True,
            "model": model,
            "dimensions": len(emb),
            "embedding_preview": preview,
            "usage": upstream.get("usage"),
        }
        _json_response(self, HTTPStatus.OK, out)


def main() -> int:
    host = os.environ.get("EMBED_PROXY_HOST", DEFAULT_HOST)
    port = int(os.environ.get("EMBED_PROXY_PORT", DEFAULT_PORT))
    httpd = HTTPServer((host, port), Handler)
    print(
        f"OpenRouter embed proxy on http://{host}:{port} "
        f"(POST /api/embed, GET /health). OPENROUTER_API_KEY required for /api/embed.",
        file=sys.stderr,
    )
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\nShutting down.", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
