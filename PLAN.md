<!-- /autoplan restore point: /Users/klaus_mac/.gstack/projects/rag-starter/unknown-autoplan-restore-20260427-014144.md -->
# Rust + LLM Learning Plan

> **Goal:** Build a personal AI agent in Rust that can use tools (read files, call APIs, do tasks).
> **Pace:** ~30 min/day. Total: ~6 weeks.
> **Starting point:** New to Rust, comfortable with the borrow checker fights, no LLM experience.
>
> **Ship & teach:** If you maintain or share this CLI, keep release flow and tutorials in sync with [docs/SHIP_PUBLISH_AND_TEACH.md](./docs/SHIP_PUBLISH_AND_TEACH.md).

---

## How to use this plan

- **One small thing per day.** If you finish in 10 minutes, stop. Don't binge.
- **Check the box when done.** Momentum matters more than speed.
- **Stuck for >20 min?** Move on. Come back tomorrow. Or ask me.
- **Don't read ahead.** It will make you anxious. Trust the order.

---

## Phase 0 — Setup (Day 0, one-time)

Pick **one** model source. You can switch later.

- [ ] **Option A — Local Ollama (free, private, slower):**
  ```bash
  brew install ollama
  ollama serve         # leave running in a terminal
  ollama pull qwen2.5:3b
  ```
- [ ] **Option B — DeepSeek API (cheap, fast, online):**
  - Get a key at https://platform.deepseek.com
  - `export DEEPSEEK_API_KEY="sk-..."` in your shell rc file
- [ ] **Option C — OpenAI (most reliable, costs $):**
  - Get a key at https://platform.openai.com
  - `export OPENAI_API_KEY="sk-..."`

✅ **Done when:** you can answer "which model am I going to call?" without hesitation.

---

## Phase 1 — First successful LLM call (Week 1, ~5 days)

The point of this week is **proving you can do it**. Nothing fancy.

- [ ] **Day 1** — `cargo run` prints a sentence from the model. (The scaffold is already done for you — see `README.md`.)
- [ ] **Day 2** — Change the model's `preamble` (system prompt). Make it answer in pirate English. Notice that *behavior changes from one line of text*.
- [ ] **Day 3** — Hardcode 3 different prompts in `main`, run them in a row. Notice each call is independent — the model has no memory.
- [ ] **Day 4** — Replace the hardcoded prompt with `std::io::stdin().read_line(...)`. Now you can type questions.
- [ ] **Day 5** — Wrap Day 4 in a `loop {}` so it keeps asking until you type `quit`. Congratulations: you have a chatbot.

✅ **Phase 1 done when:** you can have a multi-turn (but stateless) conversation in your terminal.

---

## Phase 2 — Rust fundamentals you actually need (Weeks 2–3, ~10 days)

These are the only Rust concepts you need to learn LLM-specific code. Skip the rest of "Rust by Example" for now.

- [ ] **Day 6** — Understand `async`, `.await`, and why `#[tokio::main]` exists. (Read: [Tokio Tutorial – Hello Tokio](https://tokio.rs/tokio/tutorial/hello-tokio).)
- [ ] **Day 7** — Understand `Result<T, E>` and the `?` operator. Try removing one `?` from `main.rs` and read the compiler error.
- [ ] **Day 8** — Understand `anyhow::Result` (what we're using). Why is it nicer than `Result<T, Box<dyn Error>>`?
- [ ] **Day 9** — Add a `struct Conversation { messages: Vec<String> }`. Push every user prompt + model reply into it.
- [ ] **Day 10** — Learn `serde::{Serialize, Deserialize}`. Make `Conversation` serializable. Save it to `chat.json` on quit.
- [ ] **Day 11** — Load `chat.json` on startup. You now have **persistent memory**.
- [ ] **Day 12** — Move the chat code out of `main.rs` into `chat.rs`. Learn `mod` and `pub`.
- [ ] **Day 13** — Add proper logging with `tracing` + `tracing-subscriber`. See every LLM call clearly.
- [ ] **Day 14** — Add a `.env` file for API keys with the `dotenvy` crate.
- [ ] **Day 15** — Buffer day. Refactor whatever feels ugly. Read your own code.

✅ **Phase 2 done when:** your CLI chatbot remembers conversations across runs and the code is clean.

---

## Phase 3 — Tools & Agent (Weeks 4–6, ~15 days)

This is the actual goal. Each tool you add makes the agent more useful.

- [ ] **Day 16** — Read [rig's tool example](https://github.com/0xPlaygrounds/rig/blob/main/rig-core/examples/agent_with_tools.rs). Don't write code, just read and re-read until the structure makes sense.
- [ ] **Day 17** — Add tool #1: `get_current_time()`. No arguments. Returns a `String`.
- [ ] **Day 18** — Test it. Ask "what time is it?" — the model should call your tool. If it doesn't, your description is bad. Improve it.
- [ ] **Day 19** — Add tool #2: `get_weather(city: String)`. Use the free [Open-Meteo API](https://open-meteo.com/) with `reqwest`. No API key needed.
- [ ] **Day 20** — Handle errors in `get_weather`. What if the city doesn't exist? Return an error string the model can understand.
- [ ] **Day 21** — Buffer day. Polish, debug, take a breath.
- [ ] **Day 22** — Add tool #3: `read_file(path: String)`. Validate the path stays inside a safe directory (don't let the model read `/etc/passwd`).
- [ ] **Day 23** — Add tool #4: `list_directory(path: String)`.
- [ ] **Day 24** — Add tool #5: `write_file(path: String, contents: String)`. Now the model can take action, not just observe.
- [ ] **Day 25** — Combine everything: ask "summarize the markdown files in `~/notes/` and save the summary to `summary.md`". Watch it work (or fail interestingly).
- [ ] **Day 26** — Add a confirmation prompt before any `write_file`. **Never give an agent silent write access.**
- [ ] **Day 27** — Add tool #6: `web_search(query: String)`. Use [Tavily](https://tavily.com/) (free tier) or DuckDuckGo HTML.
- [ ] **Day 28** — Add tool #7: `run_shell(command: String)` — but only with an allowlist of safe commands (`ls`, `cat`, `git status`).
- [ ] **Day 29** — Refactor: each tool in its own file under `src/tools/`.
- [ ] **Day 30** — Polish: add a nice startup banner, color the model's responses with `colored`, write a real `README.md`.

✅ **Phase 3 done when:** you have a CLI agent that can answer questions, read your notes, search the web, and write files — and you understand every line of code.

---

## What comes after (optional, pick one)

- **Make it a web app:** wrap it in `axum`, serve a chat UI.
- **Add RAG:** index your `~/notes/` with `rig-lancedb`, let the agent search them semantically. (This is what `rag-starter` was originally going to be — now you'll have the foundation to build it properly.)
- **Add MCP:** expose your tools as an [MCP server](https://modelcontextprotocol.io/) so Cursor and Claude Desktop can use them.
- **Go fully local:** swap Ollama for `mistral.rs` so the model runs in-process, pure Rust.

---

## Things you do NOT need to learn yet

You will read about these online and feel guilty. Don't.

- ❌ Training or fine-tuning models
- ❌ GPU / CUDA programming
- ❌ Quantization internals (GGUF, AWQ, etc.)
- ❌ LangChain, LangGraph, multi-agent orchestration
- ❌ Vector database internals (HNSW, IVF, etc.)
- ❌ Building your own embedding model

Build the agent first. These will make sense (or be irrelevant) later.

---

## When you get stuck

1. Read the compiler error **out loud**. Rust errors are usually correct.
2. `cargo check` is faster than `cargo build` for catching errors.
3. Search the rig examples: https://github.com/0xPlaygrounds/rig/tree/main/rig-core/examples
4. Ask me. Paste the error. I'll explain it.

---

**Today's task:** Phase 0 + Phase 1 Day 1. See `README.md`.

---

## Autoplan Review (2026-04-27)

### Phase 0 Intake
- Base branch fallback: `main` (git metadata unavailable in this workspace snapshot).
- UI scope: `no` (plan is CLI/learning focused).
- DX scope: `yes` (heavy developer workflow and onboarding content).
- Design doc discovered and reviewed: `/Users/klaus_mac/.gstack/projects/rag-starter/klaus_mac-unknown-design-20260427-014852.md`.

### Step 0 Outputs (CEO track)

#### 0A) Premise challenge
- The current document is strong as a learning roadmap, but weak as a startup execution artifact.
- Core gap: customer pain and adoption proof are implicit, not operationalized.
- Premise gate passed: user agreed to startup framing + workflow-first wedge + safety-first sequencing.

#### 0B) Existing code leverage map
| Sub-problem | Existing implementation leverage |
| --- | --- |
| Interactive agent loop | `src/main.rs` |
| RAG retrieval/indexing | `src/rag.rs` |
| Model/session wiring | `src/session.rs` |
| Python KB demo path | `streamlit_app.py`, `kb_rag.py` |
| Model reference site | `rag_model_site/` |

#### 0C) Dream-state diagram
```
CURRENT (learning roadmap only)
    -> THIS PLAN (workflow-first startup wedge + safety/test gates)
        -> 12-MONTH IDEAL (reliable, differentiated workflow product with measurable retention and distribution moat)
```

#### 0C-bis) Alternatives table
| Approach | Effort | Risk | Why it exists |
| --- | --- | --- | --- |
| A. Workflow-first wedge | S-M | Low-Med | Fastest path to demand evidence |
| B. Architecture-first platform | M-L | Med | Better long-term structure, slower validation |
| C. Concierge-first validation | S | Med | Fastest market signal, less reusable code |

#### 0D) Mode-specific scope decisions (SELECTIVE_EXPANSION)
- Accepted: tighten startup framing, add measurable outcomes, add safety/test sequencing.
- Deferred to `TODOS.md`: security policy depth, deploy target commitment, schema formalization, pilot sourcing ops.

#### 0E) Temporal interrogation
- Hour 1 risk: setup friction and unclear first measurable win.
- Hour 6+ risk: feature creep into broad-agent scope before proving one workflow.
- 6-month regret risk: building a rich “agent playground” with weak retention and no wedge moat.

#### 0F) Mode confirmation
- Mode used: `SELECTIVE_EXPANSION`.

### CEO Review Findings
- Critical: problem statement is curriculum-centric, not user-outcome-centric.
- Critical: missing explicit adoption and demand premises.
- High: unsafe capabilities are sequenced too early relative to guardrails.
- High: plan/repo drift may cause duplicate implementation effort.

### CEO Dual Voices Consensus
- Codex voice: unavailable (`codex-cli 0.104.0` cannot execute required default model in this environment).
- Claude subagent voice: completed.

| Dimension | Claude | Codex | Consensus |
| --- | --- | --- | --- |
| Premises valid? | Needs tightening | N/A | partial |
| Right problem to solve? | Partially | N/A | partial |
| Scope calibration correct? | Too broad early | N/A | partial |
| Alternatives explored enough? | Yes, after revision | N/A | partial |
| Competitive/market risks covered? | Under-covered | N/A | partial |
| 6-month trajectory sound? | Needs wedge discipline | N/A | partial |

### Error & Rescue Registry
| Risk | Rescue |
| --- | --- |
| No hard demand proof | Run 5-user pilot with count-based thresholds |
| Setup friction stalls Day 1 | Add realistic Day 0.5 smoke path and fallback |
| Unsafe tool behavior | Gate with approval + schema validation + audit logs |
| Docs ambiguity | Split onboarding tracks and anchor quickstart path |

### Failure Modes Registry
| Failure mode | Severity | Mitigation |
| --- | --- | --- |
| Kitchen-sink agent scope | High | Lock to one workflow wedge |
| Premature mutating tools | High | Safety layer before write/shell |
| Undefined reliability metric | High | Determinism + observability contract |
| Pilot fails due user sourcing, not product | Med | Pre-commit pilot cohort before run week |

### NOT in scope (for this revision)
- Full multi-workflow orchestration.
- Generic agent framework abstraction.
- Enterprise auth/integration matrix.
- Full GTM strategy beyond wedge validation.

### What already exists
- Tooling, session management, and retrieval foundations already live in repo.
- A stakeholder-facing Streamlit path already exists for demo surfaces.
- Static model-site work already provides content/presentation channel.

### Dream-state delta
- Before: strong educational sequence, weak startup validation path.
- After review: startup wedge is explicit, validation criteria defined, safety and DX sequencing tightened.

### Eng Review Findings
- P0-equivalent concern: mutation/shell safety sequencing.
- P1: test strategy needs explicit unit/integration/security matrix.
- P1: conversation schema should be typed and versioned early.
- P1: observability/config should move earlier in plan.
- P2: hidden complexity in shell/search assumptions.

### Eng Dual Voices Consensus
- Codex voice: unavailable (same environment limitation).
- Claude subagent voice: completed.

| Dimension | Claude | Codex | Consensus |
| --- | --- | --- | --- |
| Architecture sound? | Needs sequencing refactor | N/A | partial |
| Test coverage sufficient? | Not yet | N/A | partial |
| Performance risks addressed? | Partially | N/A | partial |
| Security threats covered? | Not enough | N/A | partial |
| Error paths handled? | Incomplete | N/A | partial |
| Deployment risk manageable? | Needs explicit target | N/A | partial |

### Architecture ASCII Diagram (targeted)
```
[User]
  -> [CLI Workflow Entry]
      -> [Deterministic Tool Layer]
          -> [Validation + Approval Gate]
              -> [Execution Log + Metrics]
                  -> [Pilot Evaluation]
```

### Test Diagram (codepath -> coverage)
| Codepath / flow | Test type | Status |
| --- | --- | --- |
| Onboarding first run | smoke/integration | required |
| Tool allowlist + path guard | unit/security regression | required |
| Approval-required side effects | integration | required |
| Reliability log completeness | integration | required |
| Pilot KPI aggregation | unit/integration | required |

Test plan artifact: `/Users/klaus_mac/.gstack/projects/rag-starter/klaus_mac-unknown-test-plan-20260427-020051.md`.

### DX Review Findings
- P1: onboarding source-of-truth is ambiguous across Rust/Python/site tracks.
- P1: TTHW assumption is optimistic without first-run compile budget.
- P1: plan/repo capability drift is not explicitly marked (already-built vs extension work).
- P2: error UX and upgrade ladders need explicit contracts.

### DX Dual Voices Consensus
- Codex voice: unavailable (same environment limitation).
- Claude subagent voice: completed.

| Dimension | Claude | Codex | Consensus |
| --- | --- | --- | --- |
| TTHW < 5 min? | No (first run heavier) | N/A | partial |
| API/CLI naming guessable? | Mostly | N/A | partial |
| Error messages actionable? | Partially | N/A | partial |
| Docs findable & complete? | Fragmented | N/A | partial |
| Upgrade path safe? | Under-specified | N/A | partial |
| Dev environment friction-free? | Needs track split | N/A | partial |

### Developer Journey Map (DX)
| Stage | Current friction | Target fix |
| --- | --- | --- |
| Discover | Too many parallel tracks | Explicit track split |
| Setup | First compile surprises | Day 0.5 timing + fallback |
| First run | Ambiguous baseline | “already done vs extension” tags |
| Configure | Model/env confusion risk | one canonical config path |
| Build | Scope creep | wedge-first guardrails |
| Validate | weak metric definitions | count-based pilot KPIs |
| Debug | low error guidance | What/Why/Next format |
| Deploy | target undecided | choose one deploy path |
| Iterate | no kill criteria loop | enforce go/no-go gates |

### Developer Empathy Narrative
“I can follow the roadmap, but I need one clear lane and one clear win. If setup takes longer than expected and docs fork me into three directions, I lose momentum. If the plan tells me exactly what to run, what success looks like, and what not to build yet, I ship.”

### DX Scorecard (0-10)
- Getting started: 6.5
- Setup clarity: 6.0
- Docs navigation: 5.5
- Error guidance: 6.0
- Workflow ergonomics: 7.0
- Safety clarity: 6.5
- Deployment path: 5.5
- Iteration loop: 7.0

Overall DX score: 6.3/10.  
TTHW assessment: current 30-90 min (first run dependent), target <30 min guided first success.

### DX Implementation Checklist
- [ ] Split docs by path (Rust learning / Python demo / model site).
- [ ] Add Day 0.5 smoke test with realistic time budget.
- [ ] Mark “baseline already implemented” vs “exercise extension”.
- [ ] Add error UX contract and recovery commands.
- [ ] Add concrete upgrade ladders with rollback notes.

### Cross-Phase Themes
- **Theme 1: Narrow wedge before breadth** (CEO + Eng + DX).
- **Theme 2: Safety and reliability before autonomy** (CEO + Eng).
- **Theme 3: Onboarding clarity is product risk, not documentation polish** (Eng + DX).

### Deferred to TODOS.md
- Token handling policy.
- Deployment target lock.
- Output schema formalization.
- Pilot user pre-commit channel.
- README track split.

<!-- AUTONOMOUS DECISION LOG -->
## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | CEO | Treat plan as startup artifact | Mechanical | P1 | User asked startup-mode autoplan; business framing needed | Keep as pure learning checklist |
| 2 | CEO | Workflow-first wedge | Mechanical | P1/P2 | Fastest path to demand evidence | Broad personal-agent scope first |
| 3 | CEO | Keep selective expansion | Taste | P3/P5 | Tightens without full rewrite | Full expansion rewrite |
| 4 | CEO | Defer enterprise auth depth | Taste | P2/P3 | Outside immediate wedge, but tracked | Pull into first sprint |
| 5 | CEO | Require premise gate | Mechanical | Skill gate | Non-auto-decided requirement | Auto-assume premises |
| 6 | Eng | Move safety before mutation | Mechanical | P1/P5 | Prevents irreversible risk debt | Add write/shell first |
| 7 | Eng | Add explicit test artifact | Mechanical | P1 | Required output and execution guard | Implicit/manual testing only |
| 8 | Eng | Count-based pilot thresholds | Mechanical | P3/P5 | Small sample avoids misleading percentages | Percentage-only KPI at n=5 |
| 9 | Eng | Keep existing code leverage | Mechanical | P4 | Avoid duplicate implementation effort | Rebuild already-working paths |
| 10 | DX | Track-split docs requirement | Mechanical | P5 | Reduces onboarding ambiguity quickly | Keep mixed-path README |
| 11 | DX | TTHW realism update | Mechanical | P3 | Better trust and planning | Keep optimistic estimate |
| 12 | DX | Skip design phase | Mechanical | Scope gate | No UI scope detected from plan | Run full UI design review |
| 13 | Global | Codex degraded to unavailable | Mechanical | Bias to action | CLI/model mismatch blocked execution | Stop whole pipeline |
| 14 | Global | Continue with subagent-only outside voice | Mechanical | P6 | Preserve independent review signal | Single reviewer only |
| 15 | Global | Create `TODOS.md` for deferred scope | Mechanical | P2 | Preserve defer list with accountability | Leave deferred items unstated |
