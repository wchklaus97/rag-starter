# Agent skills: Claude Code, Cursor, and Codex

This repo can expose the **same** skill folders to **three** toolchains that use different paths.

## What we found (your machine)

| Location | Role | Notes |
|----------|------|--------|
| `~/.claude/skills/` | Claude Code (global) | Large library (gstack, adapt, ship, …). Primary copy for many workflows. |
| `~/.codex/skills/` | Codex CLI (global) | Smaller set today (e.g. `opencli`, `pdf`, `playwright-interactive`). |
| `~/.cursor/skills-cursor/` | Cursor built-ins | **Reserved** — Cursor manages this; do not add project skills here. |
| `~/.cursor/skills/` | Cursor (user, global) | Optional personal skills (per Cursor create-skill docs). |
| **Project** `.claude/skills/`, `.cursor/skills/`, `.codex/skills/` | Per-repo | Use for **this repository**; can be real folders or **symlinks** to a single canonical tree. |

## Recommended patterns

1. **Project-specific** skill (committed): `skills/rag-starter/SKILL.md` in this repo — describes layout (Rust agent, `rag_model_site/`, routing in `CLAUDE.md`).  
2. **Shared gstack (or your global pack)** (optional, not committed): symlink the same `gstack` directory into `.claude/skills/gstack`, `.cursor/skills/gstack`, and `.codex/skills/gstack` so all agents see identical commands.  
3. **Single source of truth** for routing: root **`CLAUDE.md`** (skill → intent mapping). Keep it short; long procedures stay in skills.

## Scripts in this repo

- `scripts/link-agent-skills.sh` — creates the three `skills` parents and links **(a)** the vendored `gstack` pack from your home, and **(b)** this repo’s `skills/rag-starter` into each toolchain path.  
  Override with `GSTACK_SKILLS=…` and `RAG_STARTER_ROOT=…` if paths differ.

## After linking

- **Claude Code**: loads `.claude/` from the project when you work in this root.  
- **Cursor**: project skills under `.cursor/skills/`.  
- **Codex**: project skills under `.codex/skills/` (same layout as you use globally).

Re-run the script after cloning on a new machine (symlinks are local).

## Git

Symlinks to `$HOME` are **not** committed. They are created locally and listed in `.gitignore`. The **script** and **`skills/rag-starter/SKILL.md`** are committed.
