# RAG Model Field Guide

Standalone static website for comparing RAG embedding models, plus a multilingual **AI agent framework** section (protocols, five architectural patterns, reference table, five-step chooser, trends, and audience advice). All copy lives in `data/agent_framework_guide.json` for **en / 繁中 / 简中**—the same language toggle as the embedding guide.

This folder is intentionally separate from the Rust CLI and Streamlit KB demo. It can be published by GitHub Pages as-is.

The page supports **English**, **繁體中文**, and **简体中文**.

## Files

- `index.html` — static page shell
- `styles.css` — technical lab notebook visual style
- `DESIGN_SYSTEM.md` — token architecture and component specs
- `assets/design-tokens.css` — runtime CSS variables
- `assets/design-tokens.json` — human-readable token inventory
- `app.js` — filters, language toggle, static JSON loading
- `data/models.json` — model prices, strengths, use cases, source links
- `data/public_examples.json` — public/network source examples mapped to recommended embedding models
- `data/agent_framework_guide.json` — agent stack map, wizard weights, and UI strings (educational; verify vendors independently)
- `OPENROUTER_EMBEDDING_MODELS.md` — long-form notes copied from the repo root
- `VERIFICATION.md` — local preview and QA checklist

## Local preview

```bash
cd rag_model_site
python3 -m http.server 8765
```

Open `http://127.0.0.1:8765/`.

## Refresh model data

From the repo root:

```bash
export OPENROUTER_API_KEY="sk-or-..."
uv run python scripts/collect_openrouter_models.py
```

This updates `rag_model_site/data/models.json` and syncs the long-form Markdown copy into this folder.
