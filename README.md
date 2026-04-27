# rag-starter

Interactive CLI agent in Rust ([rig](https://github.com/0xPlaygrounds/rig)): pirate-flavored system prompt, **persistent** `chat.json`, **workspace RAG** over local text files, and **tools** (time, weather, read/list files under a workspace, allowlisted shell).

See [PLAN.md](./PLAN.md) for the learning roadmap.

**AI agents (Claude Code / Cursor / Codex):** intent routing is in [CLAUDE.md](./CLAUDE.md). To mirror skills into `.claude`, `.cursor`, and `.codex` in this repo, read [docs/AGENT_SKILLS.md](./docs/AGENT_SKILLS.md) and run `bash scripts/link-agent-skills.sh` after clone.

## Prerequisites

Pick one backend:

- **Ollama** (default): `brew install ollama`, then `ollama serve` and `ollama pull qwen2.5:3b`
- **DeepSeek**: API key → `export DEEPSEEK_API_KEY=...`
- **OpenAI**: API key → `export OPENAI_API_KEY=...`

## Run

```bash
cd /Users/klaus_mac/Projects/04-Experiments/rag-starter

# Optional: workspace root for file tools + chat.json (defaults to current directory)
export WORKSPACE_DIR="$PWD"

# Optional: RUST_LOG=debug
cargo run
```

Environment:

| Variable | Default | Meaning |
| ---------- | --------- | --------- |
| `MODEL_SOURCE` | `ollama` | `ollama` \| `deepseek` \| `openai` |
| `OLLAMA_MODEL` | `qwen2.5:3b` | Ollama model name |
| `DEEPSEEK_MODEL` | `deepseek-chat` | DeepSeek model id |
| `OPENAI_MODEL` | `gpt-4o-mini` | OpenAI model id |
| `WORKSPACE_DIR` | current dir | Sandbox for `read_file`, `list_directory`, `run_safe_shell`; `chat.json` is stored here |
| `RAG_STARTER_DEMO` | unset | Set to `1` to run three demo prompts once at startup (PLAN Day 3 style), then continue to the REPL |
| `RAG_ENABLE` | `1` | Enable semantic retrieval over workspace files |
| `RAG_REINDEX` | `0` | Force rebuild of `rag_index.json` on startup |
| `RAG_EMBEDDING_SOURCE` | `MODEL_SOURCE` except `deepseek -> openai` | `openai` or `ollama` |
| `OPENAI_EMBEDDING_MODEL` | `text-embedding-3-small` | OpenAI embedding model used for RAG |
| `OLLAMA_EMBEDDING_MODEL` | `nomic-embed-text` | Ollama embedding model used for RAG |
| `RAG_CHUNK_CHARS` | `900` | Chunk size for indexed workspace text |
| `RAG_CHUNK_OVERLAP` | `200` | Overlap between adjacent chunks |
| `RAG_TOP_K` | `4` | Max retrieved chunks per question |
| `RAG_MIN_SCORE` | `0.20` | Minimum cosine similarity for retrieved chunks |

Type `quit` or `exit` to stop. History is saved to `WORKSPACE_DIR/chat.json` after each reply.

## Workspace RAG

On startup the app indexes text-like files in the workspace (`.md`, `.txt`, `.rs`, `.toml`, `.json`, `.yaml`, `.yml`), writes embeddings to `WORKSPACE_DIR/rag_index.json`, and reuses that index until the workspace files or RAG config change.

Each chat turn:

1. Embeds the user question.
2. Retrieves the most relevant workspace chunks.
3. Injects them into the prompt.
4. Instructs the model to cite them like `[path#chunkN]` and say when the indexed context is insufficient.

## Tools (Phase 3)

| Tool | Purpose |
| ------ | --------- |
| `get_current_time` | Local time (RFC3339) |
| `get_weather` | Open-Meteo (city name) |
| `read_file` | Read UTF-8 text under workspace (relative path) |
| `list_directory` | List names in a directory under workspace |
| `run_safe_shell` | Runs only: `ls`, `pwd`, `date`, `echo`, `whoami`, `uname` (cwd = workspace; no pipes/redirection) |

## First-time compile

The first `cargo build` downloads and compiles many crates (including `rig-core`); later builds are fast.

## Troubleshooting

- **Connection refused** with Ollama: start `ollama serve` and ensure the model is pulled.
- **Tool / max turns errors**: the agent uses `default_max_turns(24)` for multi-step tool use; increase in `session.rs` if needed.

## Internal KB Assistant Demo

This repo now also includes a small Python + Streamlit demo for an internal knowledge assistant.
It is intentionally simple:

- Upload 3-5 `.md` or `.txt` files, or use the bundled files in `demo_docs/`
- The bundled samples are **synthetic** demo text (not scraped from the web); see `demo_docs/README.md` and `demo_docs/manifest.json` for **categories**, **provenance**, and **example questions** that match each file
- Ask a question (or click an example question in the expander when using sample docs)
- Get an answer with inline citations and visible source snippets

The Rust CLI app is still here. This Streamlit app is a separate demo path for stakeholder-friendly browser demos.

### Setup

```bash
cd /Users/klaus_mac/Projects/04-Experiments/rag-starter

brew install uv
uv sync --group dev

cp .env.example .env
# add OPENAI_API_KEY
# optional: add ANTHROPIC_API_KEY if you want Claude answers
```

If you do not want to use `uv`, the older `python3 -m venv .venv && pip install -r requirements.txt` path still works.

### Run

```bash
uv run streamlit run streamlit_app.py
```

### Recommended Demo Flow

1. Open the app in the browser.
2. Choose `Sample docs` in the sidebar.
3. Click `Build index`.
4. Open the expander **Bundled sample docs** and click an example question to fill the question field, or type your own.
5. Click **Ask** (or use one of the manifest examples, for example: new-hire security training, Priority 1 SLA, travel approval, phishing reporting).
6. Show the answer, then open the cited source cards underneath to prove where the answer came from.

### Expected Behavior

- If the question matches the indexed files, the app answers with inline citations like `[security_policy.md#chunk0]`.
- The app also shows the retrieved source snippets below the answer so the citation trail is visible even if the model response is brief.
- If the indexed files do not contain enough evidence, the app should say that clearly instead of guessing.

### Talk Track

Use this exact line in the demo:

`I built a small internal knowledge assistant that retrieves relevant documents and answers with citations, which is a safer pattern for enterprise AI adoption.`

## RAG model field guide (GitHub Pages)

A static, trilingual field guide for OpenRouter **embedding** models lives as a standalone mini-project under [`rag_model_site/`](./rag_model_site/): RAG use cases, price hints, public example sources, and links to model pages. It supports **English**, **繁體中文**, and **简体中文**. Long-form reference: [`OPENROUTER_EMBEDDING_MODELS.md`](./OPENROUTER_EMBEDDING_MODELS.md) (a copy is in [`rag_model_site/OPENROUTER_EMBEDDING_MODELS.md`](./rag_model_site/OPENROUTER_EMBEDDING_MODELS.md) for the published site).

### Local preview

```bash
cd rag_model_site
python3 -m http.server 8765
# open http://127.0.0.1:8765/
```

Use a local server (not `file://`) so `data/models.json` loads.

### Refresh model prices from OpenRouter

Requires an OpenRouter API key (`OPENROUTER_API_KEY`). This updates USD fields and `sourceUrls` in `rag_model_site/data/models.json` and re-copies the root Markdown into `rag_model_site/` for Pages.

```bash
export OPENROUTER_API_KEY="sk-or-..."
uv run python scripts/collect_openrouter_models.py
# optional: --dry-run  # print JSON without writing
```

The Streamlit **Embedding model guide** tab also reads `rag_model_site/data/public_examples.json`, which maps public/network data sources to suggested embedding models. These are source suggestions only; check the license and terms before copying content into `demo_docs/`.

### Publish on GitHub Pages

1. Push to GitHub. Under **Settings → Pages**, set the source to **GitHub Actions** (or deploy from the `gh-pages` branch if you use a different flow).
2. The workflow [`.github/workflows/pages.yml`](./.github/workflows/pages.yml) uploads the [`rag_model_site/`](./rag_model_site/) folder to Pages.
3. After deploy, your site is the Pages URL (often `https://<user>.github.io/<repo>/`).

**First-time setup:** in the repo’s Pages settings, choose **GitHub Actions** as the build source if prompted.

### Verification

See [rag_model_site/VERIFICATION.md](rag_model_site/VERIFICATION.md) for a manual and optional Playwright checklist.
