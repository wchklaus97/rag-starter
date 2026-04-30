# Sync `rag-starter` → split public repos

Use this when the monorepo is ahead of [rust-ai-agent](https://github.com/wchklaus97/rust-ai-agent), [internal-kb-assistant](https://github.com/wchklaus97/internal-kb-assistant), or [ai-rag-agent-guide-site](https://github.com/wchklaus97/ai-rag-agent-guide-site). **Do not copy monorepo-only trees** (`.claude`, `.cursor`, `.codex`, `embed_proxy/`, `skills/`, root `docs/` except as noted, `PLAN.md`, etc.) unless you intentionally want them public.

**Powershell / path note:** examples use `RAG=.../rag-starter`; adjust for your machine.

---

## 0) Preconditions

- Monorepo `main` is what you want to publish (commit or stash local WIP).
- Clone each split repo on the side, work on a branch, open a PR.

```bash
export RAG="$PWD"   # run from rag-starter root, or set absolute path
```

---

## 1) `rust-ai-agent`

### Copy (from monorepo → split repo root)

| Source (`$RAG/…`) | Destination (split repo root) |
|-------------------|-------------------------------|
| `src/` | `src/` |
| `Cargo.lock` | `Cargo.lock` |
| `.env.example` | `.env.example(review)` |

Treat `Cargo.toml` as **edit**, not blind overwrite (see below).

### Rename / edit checklist

1. **`Cargo.toml`** — after taking monorepo’s dependency **content**, set:
   - `[package] name = "rust-ai-agent"` (not `rag-starter`).
2. **`.gitignore`** — keep split-repo ignore (or merge); monorepo may list extra paths.
3. **`README.md`** — split repo already describes **rust-ai-agent**; refresh **only** if env vars or run instructions changed in monorepo.
4. Optional: document `DEBUG_RUN_ID` / `debug_ndjson` if you keep `src/debug_ndjson.rs`.

### Verify

```bash
cargo fmt --check
cargo clippy
cargo test   # if you add tests later
```

---

## 2) `internal-kb-assistant`

### Copy

| Source | Destination |
|--------|-------------|
| `streamlit_app.py` | `streamlit_app.py` |
| `kb_rag.py` | `kb_rag.py` |
| `requirements.txt` | `requirements.txt` |
| `demo_docs/` | `demo_docs/` |
| `tests/` | `tests/` (skip `__pycache__`) |
| `.env.example` | `.env.example` (merge if split had extra keys) |

### `pyproject.toml` — merge, do not use monorepo file as-is

Keep **split** identity and pytest config; align **dependencies** with monorepo `[project]`:

```toml
[project]
name = "internal-kb-assistant"
version = "0.1.0"
description = "Streamlit internal knowledge-base demo with RAG and citations."
readme = "README.md"
requires-python = ">=3.11"
dependencies = [
  "anthropic",
  "numpy",
  "openai",
  "python-dotenv",
  "streamlit",
]

[dependency-groups]
dev = [
  "pytest",
]

[tool.pytest.ini_options]
pythonpath = ["."]
testpaths = ["tests"]
```

Then regenerate lock in **that** repo:

```bash
cd /path/to/internal-kb-assistant
uv lock
```

### Verify

```bash
uv sync --group dev
uv run pytest
uv run streamlit run streamlit_app.py   # smoke
```

---

## 3) `ai-rag-agent-guide-site`

### Copy

| Source | Destination |
|--------|-------------|
| `rag_model_site/` (entire tree) | `rag_model_site/` |
| `OPENROUTER_EMBEDDING_MODELS.md` | `OPENROUTER_EMBEDDING_MODELS.md` |
| `scripts/collect_openrouter_models.py` | `scripts/collect_openrouter_models.py` |

**Do not** copy `scripts/link-agent-skills.sh` — monorepo-only.

### `pyproject.toml` — keep **split** file

The published site repo uses minimal deps:

```toml
[project]
name = "ai-rag-agent-guide-site"
version = "0.1.0"
description = "Static OpenRouter embedding models field guide (GitHub Pages)."
readme = "README.md"
requires-python = ">=3.11"
dependencies = []

[dependency-groups]
dev = []
```

Only change this if you add a real Python dependency to the **site** repo.

### CI / Pages

- Diff `.github/workflows/pages.yml` against monorepo if you changed build roots or artifact paths.
- `README.md` in the **site** repo may differ from `rag_model_site/README.md`; update the **repo** README only if user-facing clone/setup changed.

### Verify

```bash
# optional: regenerate models data if you use OpenRouter API locally
uv run python scripts/collect_openrouter_models.py   # when documented in site README
# open rag_model_site/index.html or rely on GitHub Actions Pages build
```

---

## 4) `swift-gemma4-sample`

### Copy

| Source | Destination |
|--------|-------------|
| `swift-gemma4-sample/` (entire tree) | `/` (root of split repo) |

**Do not** copy `swift-gemma4-sample/models/` — models should never be pushed to GitHub.
**Do not** copy `swift-gemma4-sample/.build/` or `swift-gemma4-sample/.swiftpm/` — these are generated directories.

### `.gitignore` — enforce strict exclusions

Ensure the public repository has a strict `.gitignore` to prevent accidental model uploads:

```text
# Models
models/
gemma-4-e2b-it-4bit/
*.safetensors
*.bin

# Swift/Xcode
.build/
.swiftpm/
xcuserdata/
DerivedData/
*.xcworkspace/
```

### Verify

```bash
cd /path/to/swift-gemma4-sample
swift test
xcodebuild -scheme Gemma4DeviceSample -destination 'generic/platform=iOS Simulator,name=iPhone 15' build  # Just to ensure it compiles, even though it doesn't run on simulators.
```

---

## 5) Hub `ai-rag-workspace`

Update links / submodule pins / “last updated” copy if you changed what each repo is for. No automatic sync from this monorepo.

---

## 6) Quick `rsync` patterns (optional)

From `rag-starter` root, **examples** (adjust `DEST`):

```bash
# Rust agent
rsync -a --delete src/ "$DEST_RUST/src/"
cp Cargo.lock "$DEST_RUST/"
# then hand-merge Cargo.toml package name

# KB assistant — no --delete on demo_docs if you want to keep split-only files
rsync -a streamlit_app.py kb_rag.py requirements.txt "$DEST_KB/"
rsync -a --delete --exclude '__pycache__' tests/ "$DEST_KB/tests/"
rsync -a demo_docs/ "$DEST_KB/demo_docs/"

# Field guide
rsync -a --delete rag_model_site/ "$DEST_SITE/rag_model_site/"
cp OPENROUTER_EMBEDDING_MODELS.md "$DEST_SITE/"
rsync -a scripts/collect_openrouter_models.py "$DEST_SITE/scripts/"

# Swift iOS App
rsync -a --delete --exclude 'models/' --exclude '.build/' --exclude '.swiftpm/' swift-gemma4-sample/ "$DEST_SWIFT/"
```

`--delete` removes files in the destination that no longer exist in the monorepo; use only when you intend **full mirror** of that subtree.

---

## 7) After sync

- Bump `CHANGELOG` / version if the split repos use them.
- **PR per repo**; don’t force-push `main` without review.
- Re-run the diff procedure (clone split `main` vs `$RAG`) before merge if you need a clean bill of health.

See also root **`AGENTS.md`** (“canonical source of truth” for where to ship).
