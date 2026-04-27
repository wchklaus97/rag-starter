# RAG model site — verification

The published site is static: it only loads `data/models.json` and `data/public_examples.json`; it has **no** runtime calls to OpenRouter or other APIs. Use this checklist before you point others at GitHub Pages.

## Local preview

From the repository root:

```bash
cd rag_model_site
python3 -m http.server 8765
```

Open `http://127.0.0.1:8765/` in a browser. (Opening `index.html` as a `file://` URL may block `fetch()` for `models.json` — use a local server.)

## Quick checks

1. **EN / 繁中 / 简中** — Toggle all three languages. Headings, filters, search placeholder, cards, public examples, and footer should switch. Reload the page: language choice should persist (localStorage).
2. **Filters** — Click each chip including “All”. Card count should change; with no match, the empty state appears.
3. **Search** — Type part of a model id (e.g. `bge` or `qwen`) and confirm cards filter.
4. **Sources** — On each card, “View on OpenRouter” (or 在 OpenRouter 檢視) should open a valid OpenRouter model or collection page.
5. **Markdown** — The link to `OPENROUTER_EMBEDDING_MODELS.md` in the “Sources & reference” section should download or display the file on GitHub Pages.
6. **Responsive** — Narrow the window to ~360px: layout should not overflow horizontally; model grid should stack.
7. **Guide map (Map / §)** — Open the header control: overlay lists sections, “current focus” matches scroll, one row is highlighted, **Escape** and backdrop close the dialog; section links jump and close the overlay. When agent content is loaded, the **Agent frameworks** row appears.
8. **No network dependency** — In DevTools → Network, confirm there are no XHR/fetch calls except `data/models.json`, `data/public_examples.json`, and fonts from Google (optional: self-host fonts later to remove that request).

## Optional: Playwright

If you use Playwright, script the same flows: language toggle, filter chips, and search, plus screenshot the hero and one model card for regression baselines. Run against `http://127.0.0.1:8765/` with the same `python3 -m http.server` as above.

## Optional: refresh data

After running `scripts/collect_openrouter_models.py` (with `OPENROUTER_API_KEY`), re-run the local preview and spot-check that prices in cards match your expectations and that the “Data snapshot” line updated.
