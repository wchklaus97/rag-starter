#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MODEL_DIR="${GEMMA4_MODEL_DIR:-$ROOT_DIR/models/gemma-4-e2b-it-4bit}"
BUILD_DIR="$ROOT_DIR/.build/release"
BINARY="$BUILD_DIR/gemma4-sample"
METALLIB="$BUILD_DIR/mlx-swift_Cmlx.bundle/default.metallib"

log() {
  printf '[run-local] %s\n' "$*" >&2
}

fail() {
  printf '[run-local] ERROR: %s\n' "$*" >&2
  exit 1
}

cd "$ROOT_DIR"

if [[ ! -d "$MODEL_DIR" ]]; then
  fail "model directory not found: $MODEL_DIR

Download it with:
  python3 -m venv /tmp/rag-hf-download-venv
  /tmp/rag-hf-download-venv/bin/python -m pip install -U huggingface_hub
  /tmp/rag-hf-download-venv/bin/hf download mlx-community/gemma-4-e2b-it-4bit --local-dir models/gemma-4-e2b-it-4bit"
fi

if [[ ! -x "$BINARY" ]]; then
  log "release binary not found. Building..."
  swift build -c release
fi

if [[ ! -f "$METALLIB" ]]; then
  fail "MLX Metal library not found: $METALLIB

This command-line sample needs the MLX SwiftPM resource bundle.
If the Metal compiler is missing, run:
  xcodebuild -downloadComponent MetalToolchain

Then rebuild or recreate the MLX metallib before running again."
fi

log "model: $MODEL_DIR"
log "binary: $BINARY"
log "metallib: $METALLIB"
log "starting local Gemma 4 generation..."

GEMMA4_MODEL_DIR="$MODEL_DIR" \
DYLD_FRAMEWORK_PATH="$BUILD_DIR" \
"$BINARY"
