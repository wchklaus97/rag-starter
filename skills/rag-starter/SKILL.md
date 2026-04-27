---
name: rag-starter
description: >-
  RAG + Rust agent monorepo: field guide static site in rag_model_site/, CLI agent
  in src/. Use for navigation, design tokens, and CLAUDE.md skill routing. Link this
  skill from .claude, .cursor, and .codex per docs/AGENT_SKILLS.md.
---

# rag-starter (this repository)

## Layout

- **Rust REPL / agent:** crate root, `src/`, `chat.json` / `rag_index.json` in workspace
- **Field guide (static site):** `rag_model_site/` — tokens in `assets/design-tokens.css`, content in `data/*.json`
- **Split public repos (outside this tree):** see hub `https://github.com/wchklaus97/ai-rag-workspace` if you use the split

## Skill routing (authoritative)

Read **`CLAUDE.md`** at the repository root. It maps user intents to gstack skills (ship, qa, investigate, design-review, document-release, …).

## Conventions

- Prefer **design tokens** (`var(--s-*)` / `var(--color-*)`) in the field guide; see `assets/design-tokens.css`.
- Do not commit machine-local symlinks under `.claude/skills`, `.cursor/skills`, `.codex/skills` — use `scripts/link-agent-skills.sh` after clone.

## When to use this skill

- “Where is X in this repo?”
- “Apply the same workflow as CLAUDE.md” while staying inside **rag-starter** paths
- Quick orientation for Codex / Cursor / Claude when the global **gstack** pack is also linked
