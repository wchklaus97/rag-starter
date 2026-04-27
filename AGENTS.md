# Agent instructions (Claude / Cursor / Codex)

## Canonical source of truth (avoid drift)

- **Shipping changes for the split public apps:** work in the dedicated GitHub repos (`rust-ai-agent`, `internal-kb-assistant`, `ai-rag-agent-guide-site`) and their PRs. See the hub: [ai-rag-workspace](https://github.com/wchklaus97/ai-rag-workspace).
- **This monorepo (`rag-starter`):** use for local experiments, the combined tree, and syncing **into** those repos on purpose. Do not assume this repo and the public copies match unless you have just copied or re-split.

- **Intent routing (which skill to use):** see **`CLAUDE.md`** in this repository root.
- **How `.claude`, `.cursor`, and `.codex` skill folders are wired:** see **`docs/AGENT_SKILLS.md`**.
- **After clone:** optional skill aliases for all three tools:  
  `bash scripts/link-agent-skills.sh`  
  (symlinks are gitignored; requires `~/.claude/skills/gstack` or set `GSTACK_SKILLS`.)

The committed **project skill** lives in **`skills/rag-starter/SKILL.md`**.
