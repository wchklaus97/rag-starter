# rag-starter

Interactive CLI agent in Rust ([rig](https://github.com/0xPlaygrounds/rig)): pirate-flavored system prompt, **persistent** `chat.json`, **workspace RAG** over local text files, and **tools** (time, weather, read/list files under a workspace, allowlisted shell).

See [PLAN.md](./PLAN.md) for the learning roadmap.

**Ship binaries, docs, and tutorials:** [docs/SHIP_PUBLISH_AND_TEACH.md](./docs/SHIP_PUBLISH_AND_TEACH.md) describes the GitHub Releases workflow, artefact matrix, publisher checklist, and end-user tutorials (Traditional Chinese narrative + English env/command identifiers).

**AI agents (Claude Code / Cursor / Codex):** intent routing is in [CLAUDE.md](./CLAUDE.md). To mirror skills into `.claude`, `.cursor`, and `.codex` in this repo, read [docs/AGENT_SKILLS.md](./docs/AGENT_SKILLS.md) and run `bash scripts/link-agent-skills.sh` after clone.

## Installation (pre-built)

Linux x86_64 and macOS (ARM64 + x86_64) binaries are built by [.github/workflows/rust-release.yml](./.github/workflows/rust-release.yml) when you push a version tag (`v*`).

1. Open **GitHub → Releases** for this repository and download the artifact that matches your OS (`rag-starter-linux`, `rag-starter-macos-arm64`, or `rag-starter-macos-x86_64`).
2. On Unix-like systems: `chmod +x <downloaded-binary>`.
3. Set LLM/workspace env vars ([docs/RUST_LEARNING_TRACK.md](./docs/RUST_LEARNING_TRACK.md)), then run the binary.

From source: `cargo build --release` → executable at `target/release/rag-starter`.

## Onboarding Tracks

To make it easier to get started, this repository is split into three main tracks. Choose the track that best fits what you want to run or build today:

- 🦀 **[Rust Learning Track](./docs/RUST_LEARNING_TRACK.md)** — Start here for the CLI agent.
- 🐍 **[Python Demo Track](./docs/PYTHON_DEMO_TRACK.md)** — Streamlit knowledge assistant.
- 🌐 **[Model Site Track](./docs/MODEL_SITE_TRACK.md)** — Trilingual field guide site.

## 🏁 Testing & Verification

Before going live, follow the **[UAT Checklist](./docs/UAT_CHECKLIST.md)** to verify all local and mobile features.
