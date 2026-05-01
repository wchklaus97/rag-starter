# 發布、部署與教學（總綱）

本文件係 **rag-starter** 喺「對外分享」維度上嘅完整藍本：講清楚 **策略／計劃地圖／如何部署 binaries／現有文檔索引／端到端教學同例子**。技術細節（環境變數、命令）以英文標識為準，方便複製。

---

## 1. 策略：點叫「有得部署」？

| 層級 | 內容 | 維護成本 |
| ----- | ------ | --------- |
| 本地自用 | `cargo run`／自己 build | 最低 |
| **公開預編譯程式（現採）** | GitHub Releases 上分發 Linux / macOS 二進位檔 | 低 |
| 託管服務 | SaaS／你自己嘅 server／API gateway | 高 |

呢個 repo 嘅核心係 **Rust CLI agent**，唔係典型 web app。**現行「部署」= 打標籤後由 CI build 並上載到 GitHub Release**，再配合 README 同上下面嘅教學，就可以畀人 **下載即用**。

---

## 2. 計劃與進度嘅文檔地圖（成個 roadmap 喺邊）

| 文件 | 用途 |
| ----- | ------ |
| [PLAN.md](../PLAN.md) | Rust + LLM 學習計劃：逐日 checklist，由第一個 LLM call 到 tools／RAG。 |
| [docs/RUST_LEARNING_TRACK.md](./RUST_LEARNING_TRACK.md) | **本 repo Rust CLI** 嘅精簡起跑線：環境變數表、工具列表、RAG 行為。 |
| [docs/PILOT_PLAN.md](./PILOT_PLAN.md) | Pilot／實驗方向（若適用於你嘅 fork）。 |
| [docs/UAT_CHECKLIST.md](./UAT_CHECKLIST.md) | 發布前要過嘅验收清單。 |
| **本文件** | 發布流程 + 對使用者嘅教學 + 連結整合。 |

學習者：唔使由零估——先睇 PLAN.md「點為何」，再對照 RUST_LEARNING_TRACK「跟住跑」。

---

## 3. 部署詳情（GitHub Releases）

### 3.1 觸發條件

Workflow：`.github/workflows/rust-release.yml`  
推送符合 `v*` 嘅 **Git tag**（例如 `v0.2.0`）會觸發 build。

### 3.2 產物（artifacts）

唔同 matrix 會產出嘅檔名（上載到該版本嘅 **Release Assets**）：

| 檔案 | 目標三元組／說明 |
| ----- | ---------------- |
| `rag-starter-linux` | `x86_64-unknown-linux-gnu` |
| `rag-starter-macos-arm64` | Apple Silicon (`aarch64-apple-darwin`) |
| `rag-starter-macos-x86_64` | Intel Mac (`x86_64-apple-darwin`) |

（Windows 未有 matrix；workflow 預留了 `.exe` 分支，將來可加。）

### 3.3 維護者：發布一次嘅操作例子

下列假設你已 commit 準備發布嘅程式碼係喺 **`main`**（或你希望打 tag 嘅 branch）。

```bash
# 確認乾淨、測試通過（例如 cargo test / cargo build --release）
git status
cargo build --release

# 打標籤（語意版本，例如 0.2.0）
git tag -a v0.2.0 -m "Release v0.2.0"

# 將 tag 推到遠端 —— 這會啟動 rust-release.yml
git push origin v0.2.0
```

之後：

1. 到 GitHub 該 repo → **Actions** 睇 workflow 是否綠。
2. 到 **Releases** 確認三個資產出現。
3. 將 Release 嘅 **Permanent link**（或 Releases 總頁）寫進 README／對外 README（見第 7 節）。

### 3.4 發布後驗證（例子）

選擇對應你機器嘅 binary 下載，加執行權限（類 Unix），再試跑：

```bash
chmod +x ./rag-starter-macos-arm64   # 例：換成你真係下載嘅檔名
export WORKSPACE_DIR="$PWD"
export MODEL_SOURCE=ollama           # 或 deepseek / openai
./rag-starter-macos-arm64
```

能進入對話並正常收工即表示 CI 同 binary 基本可以相信。

---

## 4. 環境前提（對「下載 binary」嘅用戶同開發者一樣要知）

程式本身唔包 LLM：**一定要** 準備其中之一：

- **Ollama**：本機跑 `ollama serve`，並 `pull` 你設定嘅模型（預設可參照 RUST_LEARNING_TRACK）。
- **DeepSeek**：`DEEPSEEK_API_KEY`
- **OpenAI**：`OPENAI_API_KEY`（如用 RAG 指定 DeepSeek，embedding 側可能自動轉 OpenAI）

完整環境表見 [RUST_LEARNING_TRACK.md § Run](./RUST_LEARNING_TRACK.md#run)。

---

## 5. 教學：由零到有用嘅對話（分兩條線）

### 線 A｜開發者：由 source 跑（教學用）

適合要跟 PLAN.md／改程式嘅人。

```bash
git clone <你的-rag-starter-repo-URL>.git
cd rag-starter
export WORKSPACE_DIR="$PWD"

# Optional: 示範用三條自動 prompt（PLAN Day 3 感覺）
export RAG_STARTER_DEMO=1

cargo run
```

成功標準：

- Terminal 進入對話；可以問「而家幾點？」睇時間 tool；`quit` 退出後 `WORKSPACE_DIR/chat.json` 有更新。

### 線 B｜使用者：只用 Release binary（對外發布嘅目標受眾）

1. **下載**：GitHub → Releases → 揀對應平台嘅資產。  
2. **解壓／放進 PATH（可選）**：例如 `mkdir -p ~/bin && mv rag-starter-macos-arm64 ~/bin/rag-starter && chmod +x ~/bin/rag-starter`。  
3. **工作目錄**：揀你想畀 agent 當「沙盒」嘅資料夾（可以係專案、筆記庫）。

```bash
cd /path/to/your/workspace
export WORKSPACE_DIR="$PWD"

# llm：
export MODEL_SOURCE=ollama
export OLLAMA_MODEL=qwen2.5:3b

# （可選）開 RAG：
export RAG_ENABLE=1
export RAG_REINDEX=0

rag-starter   # 或 ./rag-starter-macos-arm64
```

### 5.1 具體「例子對話」（教學劇本）

用呢啲係課堂式步驟，可以逐條俾新人跟：

| 步驟 | 你做嘅輸入（例子） | 預期學習點 |
| ----- | ------------------- | ---------- |
| 1 | 「Hello」 | Model 會應答；確認網絡／Ollama 正常 |
| 2 | 「what time is it?」或「而家幾點？」 | 觸發 `get_current_time` |
| 3 | 「What's the weather in Hong Kong?」 | 觸發 `get_weather` |
| 4 | 將一份 `.md` 放入 `WORKSPACE_DIR`，問「Summarize NOTES.md」 | `read_file` + 對 workspace 嘅理解 |
| 5 | 開 `RAG_ENABLE=1`，問索引相關問題 | Retriever：應見 `[path#chunkN]` 式引用 |

### 5.2 進階：與 PLAN 對齊嘅「自己做」練習

- PLAN.md Phase 3 每日任務係 **設計上嘅課題**；實際本 repo `src/` 已實現多數 tools／RAG。  
- **教學建議**：學員先完成「線 B」嘅劇本，再回看 PLAN 「Day 16+」嘅閱讀任務 **唔改程式** 先講結構；要 coding 嘅 lab 就由 fork 開始加新 tool。

---

## 6. 安全與密鑰（必睇嘅文檔）

發布嘅文檔要同 **唔好洩漏 key** 一齊講：[docs/SECURITY_AND_TOKENS.md](./SECURITY_AND_TOKENS.md)。

---

## 7. README 對外段落範例（複製即用）

發布新版本時，可以放喺 README 嘅「Installation」區：

```markdown
## Installation (pre-built)

Pre-built binaries for Linux x86_64 and macOS (ARM64 / x86_64) are attached to [GitHub Releases](https://github.com/<OWNER>/<REPO>/releases).

1. Download the asset matching your OS.
2. `chmod +x rag-starter-*` (Unix).
3. Set `WORKSPACE_DIR` and LLM credentials (see [docs/RUST_LEARNING_TRACK.md](docs/RUST_LEARNING_TRACK.md)).
4. Run the binary.

Build from source: `cargo build --release` (binary at `target/release/rag-starter`).
```

將 `<OWNER>/<REPO>` 換成真實路徑。

---

## 8. 發布 checklist（短文版）

- [ ] `main`（或發布分支）已合併、本地 `cargo build --release` 通過。
- [ ] [UAT_CHECKLIST](./UAT_CHECKLIST.md) 關鍵項已過（視乎你 scope）。
- [ ] 打 annotated tag `vX.Y.Z` 並 push。
- [ ] CI 綠燈；Releases 有三個 artefacts。
- [ ] README Releases 鏈結正確；如有 breaking change 寫進 Release notes。
- [ ] 對外講師／同學：轉發本文件 §5 教學劇本 + RUST_LEARNING_TRACK 環境表。

---

## 9. 同其他 track 嘅關係

| Track | 文檔 | 備註 |
| ----- | ------ | ------ |
| Python demo | [PYTHON_DEMO_TRACK.md](./PYTHON_DEMO_TRACK.md) | Streamlit／Python 維度嘅部署唔同 CLI。 |
| Model site | [MODEL_SITE_TRACK.md](./MODEL_SITE_TRACK.md) | 靜態站／另一套發布鏈。 |
| 分離多個 public repo | [SYNC_SPLIT_REPOS.md](./SYNC_SPLIT_REPOS.md) | 若正式「部署」發生喺 **split repo**，教學要改鏈結同 Releases 嘅 **owner**。 |

CLI 發布嘅「唯一真相」以 **你呢個載有 `rust-release.yml` 嘅 repo** 為準；split 後要喺對應 repo 複製／同步 workflow。

---

**維護提醒**：程式行為一改（環境名、binary 名、matrix），要同步改 **`.github/workflows/rust-release.yml`、`docs/RUST_LEARNING_TRACK.md`、本文 §3／§7** ，先算「規劃同執行都有文檔寫落」嘅閉環。
