# Gemma4 Swift Sample Setup, Built-ins, and Workflow

This document records the working setup for `swift-gemma4-sample`.

Status: the sample now builds, loads the local Gemma 4 model, and generates text on Apple Silicon.

## English

### What Works Now

- SwiftPM release build works.
- `Gemma4SwiftCore` registers with `mlx-swift-lm`.
- The model loads from a local directory through `GEMMA4_MODEL_DIR`.
- Runtime progress logs print to stderr.
- Generated text streams to stdout.
- The local model is stored under `models/gemma-4-e2b-it-4bit`.
- The `models/` folder is ignored by git.
- Design tokens exist for a future SwiftUI shell.

### Required Setup

1. Apple Silicon Mac.
2. Swift 5.9+.
3. Xcode with Metal Toolchain installed.
4. Local model files downloaded.
5. MLX `default.metallib` available through `DYLD_FRAMEWORK_PATH`.

Install the Metal Toolchain if `xcrun metal` fails:

```bash
xcodebuild -downloadComponent MetalToolchain
```

Check Metal compiler:

```bash
xcrun -sdk macosx metal -v
```

### Model Download

The model is already downloaded locally in this workspace:

```text
swift-gemma4-sample/models/gemma-4-e2b-it-4bit
```

If downloading again, use:

```bash
cd swift-gemma4-sample

python3 -m venv /tmp/rag-hf-download-venv
/tmp/rag-hf-download-venv/bin/python -m pip install -U huggingface_hub
/tmp/rag-hf-download-venv/bin/hf download mlx-community/gemma-4-e2b-it-4bit \
  --local-dir models/gemma-4-e2b-it-4bit
```

### Build

```bash
cd swift-gemma4-sample
swift build -c release
```

### Run

Use the compiled executable directly, not `swift run`, when validating runtime. This avoids SwiftPM rebuilding and losing the MLX resource path.

```bash
cd swift-gemma4-sample
./run-local.sh
```

Equivalent manual command:

```bash
cd swift-gemma4-sample

GEMMA4_MODEL_DIR="$PWD/models/gemma-4-e2b-it-4bit" \
DYLD_FRAMEWORK_PATH="$PWD/.build/release" \
.build/release/gemma4-sample
```

Expected successful stages:

```text
Registration complete.
Model loaded.
Prompt encoded: 19 tokens.
Generating response.
First token chunk received.
Generation complete.
```

### Built-in Capabilities

- Local Gemma 4 text generation.
- Local model directory override via `GEMMA4_MODEL_DIR`.
- Runtime progress logging.
- Hugging Face download is optional because local model loading works.
- Design system docs and Swift token definitions:
  - `DESIGN.md`
  - `design-tokens.json`
  - `Sources/Gemma4Sample/DesignTokens.swift`
- Maintainability plan:
  - `MAINTAINABILITY_PLAN.md`

### Development Workflow

1. Keep the sample narrow.
2. Build first.
3. Do not require model downloads in CI.
4. Use the local model for runtime smoke tests.
5. Do not add RAG, memory, cloud inference, or agent behavior without a new plan.
6. Before adding features, extract testable helper functions from `Gemma4Sample.swift`.
7. Keep design tokens as future UI guardrails, not a product promise.

### Verification Checklist

Before saying the sample works:

- `swift build -c release` passes.
- Local model exists in `models/gemma-4-e2b-it-4bit`.
- `DYLD_FRAMEWORK_PATH="$PWD/.build/release"` is set.
- Runtime reaches `Model loaded`.
- Runtime reaches `First token chunk received`.
- Generated text prints to stdout.

### Real iPhone Wrapper

The physical-device wrapper lives in:

```text
swift-gemma4-sample/ios-app/
```

Use it only on a real iPhone or iPad. The iOS Simulator is unsupported because MLX requires real Metal GPU support.

Start with:

```text
ios-app/README.md
ios-app/DEVICE_CHECKLIST.md
```

The iPhone app source imports `Gemma4SampleCore`, the same runtime path used by the CLI sample. The first model strategy checks the app Documents folder, then the app bundle, for `gemma-4-e2b-it-4bit`.

### Known Failure Modes

- Missing Metal Toolchain:
  - symptom: `cannot execute tool 'metal' due to missing Metal Toolchain`
  - fix: `xcodebuild -downloadComponent MetalToolchain`

- Missing MLX Metal library:
  - symptom: `Failed to load the default metallib`
  - fix: ensure `default.metallib` exists in `.build/release/mlx-swift_Cmlx.bundle` and run with `DYLD_FRAMEWORK_PATH="$PWD/.build/release"`

- Missing model:
  - symptom: model loading stalls or downloads unexpectedly
  - fix: set `GEMMA4_MODEL_DIR` to the local model folder

---

## 简体中文

### 现在已经能用的部分

- SwiftPM release build 可以通过。
- `Gemma4SwiftCore` 可以成功注册到 `mlx-swift-lm`。
- 模型可以通过 `GEMMA4_MODEL_DIR` 从本地目录加载。
- 运行进度会打印到 stderr。
- 生成文本会输出到 stdout。
- 本地模型放在 `models/gemma-4-e2b-it-4bit`。
- `models/` 已经被 git 忽略。
- 已经有未来 SwiftUI 用的设计 tokens。

### 必要环境

1. Apple Silicon Mac。
2. Swift 5.9+。
3. Xcode，并且安装 Metal Toolchain。
4. 已下载本地模型文件。
5. 运行时用 `DYLD_FRAMEWORK_PATH` 指向 MLX 的 build 目录。

如果 `xcrun metal` 不能用，先安装 Metal Toolchain：

```bash
xcodebuild -downloadComponent MetalToolchain
```

检查 Metal 编译器：

```bash
xcrun -sdk macosx metal -v
```

### 下载模型

当前 workspace 已经下载好了：

```text
swift-gemma4-sample/models/gemma-4-e2b-it-4bit
```

如果以后要重新下载：

```bash
cd swift-gemma4-sample

python3 -m venv /tmp/rag-hf-download-venv
/tmp/rag-hf-download-venv/bin/python -m pip install -U huggingface_hub
/tmp/rag-hf-download-venv/bin/hf download mlx-community/gemma-4-e2b-it-4bit \
  --local-dir models/gemma-4-e2b-it-4bit
```

### 构建

```bash
cd swift-gemma4-sample
swift build -c release
```

### 运行

真实验证 runtime 时，建议直接跑编译好的 executable，不要用 `swift run`。这样不容易触发 SwiftPM 重新编译，也更容易保持 MLX resource 路径正确。

```bash
cd swift-gemma4-sample
./run-local.sh
```

等价的手动命令：

```bash
cd swift-gemma4-sample

GEMMA4_MODEL_DIR="$PWD/models/gemma-4-e2b-it-4bit" \
DYLD_FRAMEWORK_PATH="$PWD/.build/release" \
.build/release/gemma4-sample
```

成功时应该看到：

```text
Registration complete.
Model loaded.
Prompt encoded: 19 tokens.
Generating response.
First token chunk received.
Generation complete.
```

### 已内建能力

- 本地 Gemma 4 文本生成。
- 通过 `GEMMA4_MODEL_DIR` 使用本地模型目录。
- 运行进度日志。
- 可以绕开黑盒下载，直接加载本地 Hugging Face 模型。
- 设计系统文件：
  - `DESIGN.md`
  - `design-tokens.json`
  - `Sources/Gemma4Sample/DesignTokens.swift`
- 维护计划：
  - `MAINTAINABILITY_PLAN.md`

### 开发工作流

1. 保持样例很小。
2. 先 build。
3. CI 不应该下载模型。
4. runtime smoke test 用本地模型。
5. 不要直接加 RAG、记忆库、云推理、agent 行为，除非先写新计划。
6. 新功能之前，先把 `Gemma4Sample.swift` 里的可测试逻辑抽出来。
7. 设计 tokens 只是未来 UI 的边界，不代表现在要做完整产品。

### 验证清单

说“它 works”之前，至少确认：

- `swift build -c release` 通过。
- `models/gemma-4-e2b-it-4bit` 存在。
- 设置了 `DYLD_FRAMEWORK_PATH="$PWD/.build/release"`。
- 运行到了 `Model loaded`。
- 运行到了 `First token chunk received`。
- stdout 打印出了生成文本。

### 真机 iPhone Wrapper

真机 wrapper 在：

```text
swift-gemma4-sample/ios-app/
```

只能用真实 iPhone 或 iPad。iOS Simulator 不支持，因为 MLX 需要真实的 Metal GPU 支持。

先看：

```text
ios-app/README.md
ios-app/DEVICE_CHECKLIST.md
```

iPhone app source 会 import `Gemma4SampleCore`，也就是 CLI sample 已经跑通的同一条 runtime 路径。第一版模型策略会先检查 app Documents，再检查 app bundle，寻找 `gemma-4-e2b-it-4bit`。

### 已知失败

- 没有 Metal Toolchain：
  - 现象：`cannot execute tool 'metal' due to missing Metal Toolchain`
  - 修复：`xcodebuild -downloadComponent MetalToolchain`

- 找不到 MLX Metal library：
  - 现象：`Failed to load the default metallib`
  - 修复：确认 `.build/release/mlx-swift_Cmlx.bundle/default.metallib` 存在，并用 `DYLD_FRAMEWORK_PATH="$PWD/.build/release"` 运行

- 没有本地模型：
  - 现象：加载卡住，或者又开始下载
  - 修复：设置 `GEMMA4_MODEL_DIR` 到本地模型目录

---

## 繁體中文

### 目前已經能用的部分

- SwiftPM release build 可以通過。
- `Gemma4SwiftCore` 可以成功註冊到 `mlx-swift-lm`。
- 模型可以透過 `GEMMA4_MODEL_DIR` 從本機目錄載入。
- 執行進度會輸出到 stderr。
- 生成文字會輸出到 stdout。
- 本機模型放在 `models/gemma-4-e2b-it-4bit`。
- `models/` 已經被 git 忽略。
- 已經有未來 SwiftUI 可用的設計 tokens。

### 必要環境

1. Apple Silicon Mac。
2. Swift 5.9+。
3. Xcode，並且安裝 Metal Toolchain。
4. 已下載本機模型檔案。
5. 執行時用 `DYLD_FRAMEWORK_PATH` 指向 MLX 的 build 目錄。

如果 `xcrun metal` 不能用，先安裝 Metal Toolchain：

```bash
xcodebuild -downloadComponent MetalToolchain
```

檢查 Metal 編譯器：

```bash
xcrun -sdk macosx metal -v
```

### 下載模型

目前 workspace 已經下載好了：

```text
swift-gemma4-sample/models/gemma-4-e2b-it-4bit
```

如果之後要重新下載：

```bash
cd swift-gemma4-sample

python3 -m venv /tmp/rag-hf-download-venv
/tmp/rag-hf-download-venv/bin/python -m pip install -U huggingface_hub
/tmp/rag-hf-download-venv/bin/hf download mlx-community/gemma-4-e2b-it-4bit \
  --local-dir models/gemma-4-e2b-it-4bit
```

### 建置

```bash
cd swift-gemma4-sample
swift build -c release
```

### 執行

真實驗證 runtime 時，建議直接跑編譯好的 executable，不要用 `swift run`。這樣比較不會觸發 SwiftPM 重新編譯，也比較容易保持 MLX resource 路徑正確。

```bash
cd swift-gemma4-sample
./run-local.sh
```

等價的手動命令：

```bash
cd swift-gemma4-sample

GEMMA4_MODEL_DIR="$PWD/models/gemma-4-e2b-it-4bit" \
DYLD_FRAMEWORK_PATH="$PWD/.build/release" \
.build/release/gemma4-sample
```

成功時應該看到：

```text
Registration complete.
Model loaded.
Prompt encoded: 19 tokens.
Generating response.
First token chunk received.
Generation complete.
```

### 已內建能力

- 本機 Gemma 4 文字生成。
- 透過 `GEMMA4_MODEL_DIR` 使用本機模型目錄。
- 執行進度日誌。
- 可以避開黑盒下載，直接載入本機 Hugging Face 模型。
- 設計系統文件：
  - `DESIGN.md`
  - `design-tokens.json`
  - `Sources/Gemma4Sample/DesignTokens.swift`
- 維護計畫：
  - `MAINTAINABILITY_PLAN.md`

### 開發工作流

1. 保持範例很小。
2. 先 build。
3. CI 不應該下載模型。
4. runtime smoke test 使用本機模型。
5. 不要直接加入 RAG、記憶庫、雲端推理、agent 行為，除非先寫新計畫。
6. 新功能之前，先把 `Gemma4Sample.swift` 裡可測試的邏輯抽出來。
7. 設計 tokens 只是未來 UI 的邊界，不代表現在要做完整產品。

### 驗證清單

說「它 works」之前，至少確認：

- `swift build -c release` 通過。
- `models/gemma-4-e2b-it-4bit` 存在。
- 設定了 `DYLD_FRAMEWORK_PATH="$PWD/.build/release"`。
- 執行到了 `Model loaded`。
- 執行到了 `First token chunk received`。
- stdout 印出了生成文字。

### 真機 iPhone Wrapper

真機 wrapper 在：

```text
swift-gemma4-sample/ios-app/
```

只能用真實 iPhone 或 iPad。iOS Simulator 不支援，因為 MLX 需要真實的 Metal GPU 支援。

先看：

```text
ios-app/README.md
ios-app/DEVICE_CHECKLIST.md
```

iPhone app source 會 import `Gemma4SampleCore`，也就是 CLI sample 已經跑通的同一條 runtime 路徑。第一版模型策略會先檢查 app Documents，再檢查 app bundle，尋找 `gemma-4-e2b-it-4bit`。

### 已知失敗

- 沒有 Metal Toolchain：
  - 現象：`cannot execute tool 'metal' due to missing Metal Toolchain`
  - 修復：`xcodebuild -downloadComponent MetalToolchain`

- 找不到 MLX Metal library：
  - 現象：`Failed to load the default metallib`
  - 修復：確認 `.build/release/mlx-swift_Cmlx.bundle/default.metallib` 存在，並用 `DYLD_FRAMEWORK_PATH="$PWD/.build/release"` 執行

- 沒有本機模型：
  - 現象：載入卡住，或者又開始下載
  - 修復：設定 `GEMMA4_MODEL_DIR` 到本機模型目錄
