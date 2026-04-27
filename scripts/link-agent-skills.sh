#!/usr/bin/env bash
# Link optional skill trees into .claude/skills, .cursor/skills, and .codex/skills
# so Claude Code, Cursor, and Codex all see the same packs. Safe to re-run.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

GSTACK_SRC="${GSTACK_SKILLS:-$HOME/.claude/skills/gstack}"
PROJ_SKILL_SRC="${RAG_STARTER_PROJ_SKILL:-$ROOT/skills/rag-starter}"

link_one() {
  local name="$1"
  local target="$2"
  local dest
  for dest in .claude/skills .cursor/skills .codex/skills; do
    mkdir -p "$dest"
    if [[ -e "$target" ]]; then
      ln -sfn "$target" "$dest/$name"
      echo "OK  $dest/$name -> $target"
    else
      echo "SKIP $name (missing: $target)" >&2
    fi
  done
}

if [[ ! -d "$ROOT/.git" && ! -f "$ROOT/CLAUDE.md" ]]; then
  echo "Run from repository root (expected CLAUDE.md)." >&2
  exit 1
fi

if [[ -d "$GSTACK_SRC" ]]; then
  link_one "gstack" "$GSTACK_SRC"
else
  echo "INFO: gstack not found at $GSTACK_SRC — set GSTACK_SKILLS to your pack path." >&2
fi

if [[ -d "$PROJ_SKILL_SRC" && -f "$PROJ_SKILL_SRC/SKILL.md" ]]; then
  link_one "rag-starter" "$PROJ_SKILL_SRC"
else
  echo "WARN: project skill missing at $PROJ_SKILL_SRC" >&2
fi

echo "Done. Tool-specific dirs are under: $ROOT/.claude/skills $ROOT/.cursor/skills $ROOT/.codex/skills"
