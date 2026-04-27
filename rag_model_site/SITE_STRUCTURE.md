# Site structure (RAG field guide, static)

This site is **static HTML + JSON** on **GitHub Pages** (no app router). The information architecture mirrors a **docs-style guides hub** (similar in spirit to locale-prefixed `/[locale]/guides` on a Next.js app): one long field guide, plus a **hierarchical entry point** so visitors can jump by topic.

## URLs

| Path | Role |
|------|------|
| `/` or `index.html` | Main field guide (embeddings, agent chooser, methodology). |
| `/guides/` | **Guides hub** — grouped cards linking to in-page anchors on the main guide. |
| `/en/guides/` | Redirect to `/guides/` (English hub). |
| `/zh/guides/` | Redirect to `/?lang=zh` (Traditional Chinese UI on the main page). |
| `/zh-hans/guides/` | Redirect to `/?lang=zh_hans` (Simplified Chinese UI on the main page). |

## Query

- `?lang=en` | `?lang=zh` | `?lang=zh_hans` — sets display language and persists in `localStorage` (see `app.js` `loadLang` / `saveLang`).

## Grouping (hub → anchors)

1. **Quick orientation** — `#decision-heading`, `#filter-heading`, `#model-grid`
2. **RAG & embeddings** — `#rag-embeddings`, `#public-examples-heading`, `#method-heading`
3. **Agent frameworks** — `#agent-guide`
4. **Reference** — `#sources-heading`

## Compared to a full multi-page product site

- We do **not** duplicate every article as a separate route; the “single long page + hub” pattern keeps the static site small and easy to maintain.
- Adding real per-locale **hub copy** (e.g. `zh/guides/` as its own page) would require either duplicate HTML or a small build step — optional follow-up.
