# Gemma4 on-device sample (Swift)

Small SwiftPM executable that depends on **[Swift-gemma4-core](https://github.com/yejingyang8963-byte/Swift-gemma4-core)** (`Gemma4SwiftCore`) and runs the upstream quick-start flow: register the model, load weights from Hugging Face (cached after first run), format a user turn with `Gemma4PromptFormatter`, stream tokens.

## Requirements

- **Apple Silicon** Mac or device (MLX is not supported on Intel/Linux at runtime).
- **Swift 6**, **macOS 15+** / **iOS 18+** (see `Package.swift` platforms).
- First run **downloads ~1.5 GB** model weights; ensure network access to Hugging Face.

## Build & run

From this directory:

```bash
swift build -c release
swift run -c release gemma4-sample
```

Or open `Package.swift` in Xcode, select the `gemma4-sample` scheme, and run on **My Mac (Designed for Apple Silicon)**.

## Pre-download the model

For better progress visibility, download the Hugging Face model yourself first:

```bash
python3 -m pip install -U "huggingface_hub[cli]"
huggingface-cli download mlx-community/gemma-4-e2b-it-4bit \
  --local-dir models/gemma-4-e2b-it-4bit
```

Then run the sample from that local directory:

```bash
./run-local.sh
```

The `models/` folder is ignored by git.

## Notes

- Do **not** use `tokenizer.applyChatTemplate` for Gemma 4 with this stack; use `Gemma4PromptFormatter` as in upstream docs.
- For full API, tests, and architecture details, see the **[upstream repository](https://github.com/yejingyang8963-byte/Swift-gemma4-core)**.
- UI direction and token definitions live in `DESIGN.md`, `design-tokens.json`, and `Sources/Gemma4Sample/DesignTokens.swift`.
- Runtime progress logs are written to stderr, while generated text streams to stdout.
- Working setup, built-in capabilities, workflow, and troubleshooting are documented in `docs/MAINTAINING.md` in English, simplified Chinese, and traditional Chinese.
