# RAG Model Field Guide (GitHub Pages)

A static, trilingual field guide for OpenRouter **embedding** models lives as a standalone mini-project under `rag_model_site/`: RAG use cases, price hints, public example sources, and links to model pages. It supports **English**, **繁體中文**, and **简体中文**. 

Long-form reference: `OPENROUTER_EMBEDDING_MODELS.md` (a copy is in `rag_model_site/OPENROUTER_EMBEDDING_MODELS.md` for the published site).

## Local preview

```bash
cd rag_model_site
python3 -m http.server 8765
# open http://127.0.0.1:8765/
```

Use a local server (not `file://`) so `data/models.json` loads.

## Refresh model prices from OpenRouter

Requires an OpenRouter API key (`OPENROUTER_API_KEY`). This updates USD fields and `sourceUrls` in `rag_model_site/data/models.json` and re-copies the root Markdown into `rag_model_site/` for Pages.

```bash
export OPENROUTER_API_KEY="sk-or-..."
uv run python scripts/collect_openrouter_models.py
# optional: --dry-run  # print JSON without writing
# optional: --include-all-api-models  # append OpenRouter embedding ids not yet in models.json (minimal copy + openrouterCatalog tag)
```

**Live embedding playground (local only):** the static Pages site does not expose your API key. Use `embed_proxy/README.md` to run a localhost proxy, then open `rag_model_site/embed-playground.html` via a local server (same as the field guide preview).

The Streamlit **Embedding model guide** tab also reads `rag_model_site/data/public_examples.json`, which maps public/network data sources to suggested embedding models. These are source suggestions only; check the license and terms before copying content into `demo_docs/`.

## Publish on GitHub Pages

1. **One-time (required):** in the repository go to **Settings → Pages → Build and deployment**, set **Source** to **GitHub Actions** (not “Deploy from a branch”) and save. If this step is skipped, the **Deploy GitHub Pages** workflow fails at *Setup Pages* with `Get Pages site failed` / `Not Found` until Pages exists.
2. Push to `main` (or run the workflow manually: **Actions → Deploy GitHub Pages → Run workflow**).
3. The workflow `.github/workflows/pages.yml` uploads the `rag_model_site/` folder to Pages.
4. When the run succeeds, the site URL is `https://wchklaus97.github.io/rag-starter/` (or **Settings → Pages** shows the public URL).  
   *A separate mirror repo (e.g. `ai-rag-agent-guide-site`) is optional; it does not share this repo’s Pages site.*

## Verification

See `rag_model_site/VERIFICATION.md` for a manual and optional Playwright checklist.

## Client-side hybrid retrieval demo

`/kb-hybrid-demo.html` (source: `rag_model_site/kb-hybrid-demo.html`) demonstrates BM25 + dense retrieval + reciprocal rank fusion with **no backend**. The corpus and embeddings are baked into `rag_model_site/data/kb_hybrid_demo.json` at deploy time (`npm run build:kb-demo` in CI). Model: **Xenova/all-MiniLM-L6-v2** (Transformers.js in the browser). This is **teaching/parity on the corpus**, not a substitute for the OpenAI-embedding Streamlit flow.
