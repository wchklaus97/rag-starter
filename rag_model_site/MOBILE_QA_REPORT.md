# Mobile / responsive QA (field guide)

**Date:** 2026-04-28  
**Method:** Cursor IDE browser (automated) + CSS review. Not a replacement for real devices.

## Environments

| URL | Result |
|-----|--------|
| **Live:** `https://wchklaus97.github.io/ai-rag-agent-guide-site/` | Home loads; language switch works; **no `/guides/` yet** (404 until you deploy latest `rag_model_site/`). |
| **Local:** `http://127.0.0.1:8765/` (e.g. `python3 -m http.server 8765` from `rag_model_site/`) | **Guides hub** and **`?lang=`** behave as designed. |

## Viewports tested

- **390×844** — typical phone width (home on production).
- **320×568** — narrow phone (guides hub locally).

## Checklist

| # | Check | Result |
|---|--------|--------|
| 1 | **Viewport** `width=device-width`, no desktop-only fixed widths on main flow | Pass (`index.html` / `guides/index.html`). |
| 2 | **Header** stacks on small screens (language + theme reachable) | Pass — `@media (max-width: 36rem)` single-column header. |
| 3 | **Model / example / hub grids** don’t force horizontal page scroll on narrow screens | **Fixed in CSS** using `minmax(min(100%, …), 1fr)`; verify after deploy. |
| 4 | **Safe area** (notch / home indicator) | **Pass** — `viewport-fit=cover` + `env(safe-area-inset-*)` on `body`. |
| 5 | **Language workflow** (buttons + `?lang=zh`) | Pass on prod + local after data load; **繁中** activates Traditional UI; hub shows **指南總覽** when `?lang=zh` (local). |
| 6 | **In-page nav** (RAG / Agent) | Present when `inpage-nav` is shown; links work in snapshot. |
| 7 | **Agent wizard / tables** | Wide tables use `.table-scroll` (horizontal scroll); acceptable pattern on mobile. |
| 8 | **Console (JS errors)** | No app errors; only Cursor browser noise about dialog overrides. |
| 9 | **Touch targets** — primary controls | **Mostly pass.** Theme button ~2.4rem; **filter chips and EN/繁中/简中** are a bit under 44px — acceptable for desktop-first doc; consider larger tap padding if mobile is primary. |
| 10 | **Guides hub language links** | **Improved** — `guide-hub__lang-link` min-height `2.75rem` so inline lang picks are easier to tap. |

## Issues / follow-ups (non-blocking)

1. **Deploy** so GitHub Pages includes `guides/`, `en|zh|zh-hans/guides/`, and latest CSS (grid + safe-area + hub lang links).
2. **i18n gap (pre-existing):** e.g. “Filters” / “Sources & reference” section labels may stay English in Traditional Chinese — optional JSON follow-up.
3. **Real devices:** re-check on iOS Safari + Chrome Android (font, 100vh, overscroll).

## Quick manual retest

```bash
cd rag_model_site && python3 -m http.server 8765
```

Open `http://127.0.0.1:8765/guides/` and `http://127.0.0.1:8765/index.html?lang=zh`, narrow the window to 320px, confirm no horizontal scrollbar on the page chrome and that hub cards stack in one column.
