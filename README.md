# rag-starter

Interactive CLI agent in Rust ([rig](https://github.com/0xPlaygrounds/rig)): pirate-flavored system prompt, **persistent** `chat.json`, **workspace RAG** over local text files, and **tools** (time, weather, read/list files under a workspace, allowlisted shell).

See [PLAN.md](./PLAN.md) for the learning roadmap.

**AI agents (Claude Code / Cursor / Codex):** intent routing is in [CLAUDE.md](./CLAUDE.md). To mirror skills into `.claude`, `.cursor`, and `.codex` in this repo, read [docs/AGENT_SKILLS.md](./docs/AGENT_SKILLS.md) and run `bash scripts/link-agent-skills.sh` after clone.

## Onboarding Tracks

To make it easier to get started, this repository is split into three main tracks. Choose the track that best fits what you want to run or build today:

- 🦀 **[Rust Learning Track](file:///Users/klaus_mac/Projects/04-Experiments/rag-starter/docs/RUST_LEARNING_TRACK.md)** — Start here for the CLI agent.
- 🐍 **[Python Demo Track](file:///Users/klaus_mac/Projects/04-Experiments/rag-starter/docs/PYTHON_DEMO_TRACK.md)** — Streamlit knowledge assistant.
- 🌐 **[Model Site Track](file:///Users/klaus_mac/Projects/04-Experiments/rag-starter/docs/MODEL_SITE_TRACK.md)** — Trilingual field guide site.

## 🏁 Testing & Verification
Before going live, follow the **[UAT Checklist](file:///Users/klaus_mac/Projects/04-Experiments/rag-starter/docs/UAT_CHECKLIST.md)** to verify all local and mobile features.
