# RAG Model Field Guide

Standalone static website for comparing RAG embedding models, plus a multilingual **AI agent framework** section (protocols, five architectural patterns, reference table, five-step chooser, trends, and audience advice). All copy lives in `data/agent_framework_guide.json` for **en / 繁中 / 简中**—the same language toggle as the embedding guide.

This folder is intentionally separate from the Rust CLI and Streamlit KB demo. It can be published by GitHub Pages as-is.

The page supports **English**, **繁體中文**, and **简体中文**.

Use the header **Map** control (§) to open a **guide map** overlay: it shows where you are on the page, highlights the matching section as you scroll, and links to each major section (including the agent framework block when that content is loaded).

## Files

- `index.html` — static page shell
- `embed-playground.html` — optional local UI for `POST /api/v1/embeddings` via [`../embed_proxy/README.md`](../embed_proxy/README.md) (not used on GitHub Pages without your own proxy host)
- `styles.css` — technical lab notebook visual style
- `DESIGN_SYSTEM.md` — token architecture and component specs
- `assets/design-tokens.css` — runtime CSS variables
- `assets/design-tokens.json` — human-readable token inventory
- `app.js` — filters, language toggle, static JSON loading
- `data/models.json` — model prices, strengths, use cases, source links; top-level **`updated`** is UTC (`YYYY-MM-DD` or `YYYY-MM-DDTHH:MM:SSZ`) for the footer and methodology “snapshot” line
- `data/public_examples.json` — public/network source examples mapped to recommended embedding models
- `data/agent_framework_guide.json` — agent stack map, wizard weights, and UI strings (educational; verify vendors independently)
- `data/kb_hybrid_demo.json` — CI-generated bundle for **client-side hybrid retrieval** on [`kb-hybrid-demo.html`](kb-hybrid-demo.html): BM25 + cosine (Xenova embeddings, same model at build time and in the browser) + reciprocal rank fusion. Not the same embedding space as the Streamlit/OpenAI path in `kb_rag.py`.
- `OPENROUTER_EMBEDDING_MODELS.md` — long-form notes copied from the repo root
- `VERIFICATION.md` — local preview and QA checklist

## Local preview

```bash
cd rag_model_site
python3 -m http.server 8765
```

Open `http://127.0.0.1:8765/` (if the port is in use, pick another port, e.g. `8766`).

## Site search (Pagefind)

GitHub Actions runs Pagefind before upload (see `.github/workflows/pages.yml`). For local dev, from the **repo root**:

```bash
npx --yes pagefind --site rag_model_site --output-subdir pagefind
```

Then reload the site; the header search box indexes static HTML under `rag_model_site/`. The `pagefind/` output is gitignored.

Multilingual **guides hub** stubs for GitHub Pages-style paths: `/guides/`, `/en/guides/`, `/zh/guides/`, `/zh-hans/guides/` (static `index.html` in each folder).

## Hybrid RAG demo (browser-only)

[`kb-hybrid-demo.html`](kb-hybrid-demo.html) runs retrieval **entirely in the visitor’s browser**: it loads `data/kb_hybrid_demo.json` (chunks + embeddings) and uses **Transformers.js** (via CDN) to embed the query with the **same** model as the build step. The corpus is `demo_docs/` in this repo, chunked similarly to `kb_rag.chunk_text` (see `scripts/build_kb_hybrid_demo.mjs`).

**Regenerate the JSON** (from repo root, after `npm install`):

```bash
npm run build:kb-demo
```

**Run unit tests** (RRF + BM25, from repo root):

```bash
npm run test:kb-hybrid
```

GitHub Actions runs `build:kb-demo` before Pagefind on each Pages deploy.

**Limits:** fixed corpus only; retrieval-only (no in-browser LLM answer); Xenova/MiniLM space is parallel to—not merged with—OpenAI embeddings used by the Streamlit stack.

From the repo root:

```bash
export OPENROUTER_API_KEY="sk-or-..."
uv run python scripts/collect_openrouter_models.py
# optional: --include-all-api-models  # merge in every id from OpenRouter's embeddings list not already in models.json
```

This updates `rag_model_site/data/models.json` and syncs the long-form Markdown copy into this folder.
