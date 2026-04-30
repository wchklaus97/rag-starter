# Python Internal KB Assistant Demo

This repo includes a small Python + Streamlit demo for an internal knowledge assistant. It is intentionally simple and designed as a stakeholder-friendly browser demo path.

- Upload 3-5 `.md` or `.txt` files, or use the bundled files in `demo_docs/`
- The bundled samples are **synthetic** demo text (not scraped from the web); see `demo_docs/README.md` and `demo_docs/manifest.json` for **categories**, **provenance**, and **example questions** that match each file
- Ask a question (or click an example question in the expander when using sample docs)
- Get an answer with inline citations and visible source snippets

## Setup

```bash
cd /Users/klaus_mac/Projects/04-Experiments/rag-starter

brew install uv
uv sync --group dev

cp .env.example .env
# add OPENAI_API_KEY
# optional: add ANTHROPIC_API_KEY if you want Claude answers
```

If you do not want to use `uv`, the older `python3 -m venv .venv && pip install -r requirements.txt` path still works.

## Run

```bash
uv run streamlit run streamlit_app.py
```

## Recommended Demo Flow

1. Open the app in the browser.
2. Choose `Sample docs` in the sidebar.
3. Click `Build index`.
4. Open the expander **Bundled sample docs** and click an example question to fill the question field, or type your own.
5. Click **Ask** (or use one of the manifest examples, for example: new-hire security training, Priority 1 SLA, travel approval, phishing reporting).
6. Show the answer, then open the cited source cards underneath to prove where the answer came from.

## Expected Behavior

- If the question matches the indexed files, the app answers with inline citations like `[security_policy.md#chunk0]`.
- The app also shows the retrieved source snippets below the answer so the citation trail is visible even if the model response is brief.
- If the indexed files do not contain enough evidence, the app should say that clearly instead of guessing.

## Talk Track

Use this exact line in the demo:

`I built a small internal knowledge assistant that retrieves relevant documents and answers with citations, which is a safer pattern for enterprise AI adoption.`
