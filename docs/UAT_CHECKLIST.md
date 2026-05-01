# User Acceptance Testing (UAT) Checklist

Follow this checklist to verify that the `rag-starter` repository is fully functional and ready for pilot users.

## 1. iOS Mobile App 📱

### [ ] Multi-Turn Conversation (Memory)
- **Action**: Ask "What is the capital of Japan?"
- **Action**: Follow up with "What is the best way to get around there?"
- **Success**: The assistant understands that "there" refers to Japan (Tokyo).

### [ ] LLM Parameter Control
- **Action**: Open Settings (gear icon), set **Temperature** to `0.1`. Ask a complex question twice.
- **Success**: The answers are nearly identical (deterministic).
- **Action**: Set **Temperature** to `1.8`. Ask the same question.
- **Success**: The answer is significantly more varied/creative.

### [ ] Settings Persistence
- **Action**: Change **Max Tokens** to `100`, close the app entirely, and reopen.
- **Success**: Open Settings; the value should still be `100`.

### [ ] UI/UX & Auto-Scrolling
- **Action**: Send a long prompt.
- **Success**: The chat view automatically scrolls to the bottom as tokens stream in.
- **Success**: Message bubbles use the correct colors (Accent for user, Surface for bot).

---

## 2. Rust CLI Agent 🦀

### [ ] Log Sanitization (Security)
- **Action**: Run the agent and send a message containing a fake key: `My key is sk-proj-1234567890abcdef1234567890abcdef12345678`.
- **Action**: Open `.cursor/debug-fe9600.log`.
- **Success**: Search for the key. It should be replaced with `[REDACTED_SECRET]`.

### [ ] Structured Output (Slash Commands)
- **Action**: Type `/summarize src/main.rs`.
- **Success**: The agent returns a pretty-printed JSON object with fields like `summary`, `risk_level`, etc.
- **Action**: Type `/review src/session.rs`.
- **Success**: The agent returns a JSON object with a structured review comment.

### [ ] RAG (Retrieval Augmented Generation)
- **Action**: Ask a question about the `PILOT_PLAN.md` (e.g., "What is the pilot pre-commit mechanism?").
- **Success**: The agent cites the document in the logs or response (e.g., `[docs/PILOT_PLAN.md#chunk0]`).

---

## 3. Infrastructure & Deployment 🚀

### [ ] Build Verification
- **Action**: Run `cargo check`.
- **Success**: Exit code `0`.

### [ ] Release Pipeline
- **Action**: Create a local tag `git tag v0.1.1-test`.
- **Action**: (Optional) Push the tag if you have a remote set up.
- **Success**: GitHub Actions should trigger the "Rust Release" workflow and build Mac/Linux binaries.

---

## 4. Documentation 📚
- **Action**: Open root `README.md`.
- **Success**: Verify links to `RUST_LEARNING_TRACK`, `PYTHON_DEMO_TRACK`, and `MODEL_SITE_TRACK` are working.
