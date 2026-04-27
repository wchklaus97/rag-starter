# Bundled sample documents (Streamlit demo)

## What these are

The `.md` and `.txt` files here are **synthetic, demo-only** “internal policy” style content (fictional company, example addresses and contacts). They were **written for this repository** so the internal KB demo works out of the box.

They are **not**:

- A dump from a public website or API  
- Proprietary or customer data  
- Guaranteed to match any real company’s rules  

## Machine-readable list

`manifest.json` lists each file, a **category** (for the UI and suggested questions), and **example questions** that match the text so retrieval demos behave well.

## Using public or licensed content instead

If you want “real public” text:

- Prefer sources with **clear licenses** (e.g. **CC0**, **public domain**, or your own company docs).  
- Government open publications, public-domain books, and your own policies are common choices.  
- After swapping files, update `manifest.json` (titles, categories, `example_questions`, and the `provenance` blurb) and click **Build index** again in the app.

## Why not auto-fetch from the web?

Automatic crawling mixes **copyright**, **terms of use**, and **stale/irrelevant** text. For a small demo, **a few hand-picked files** (like these, or your own) are easier to reason about and safer to show in stakeholder demos.
