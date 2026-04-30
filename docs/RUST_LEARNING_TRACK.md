# Rust + LLM Learning Track

This guide covers the interactive CLI agent built in Rust using `rig`.

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
