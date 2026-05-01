<!-- /autoplan restore point: ~/.gstack/projects/rag-starter/main-autoplan-restore-swift-gemma4-sample-20260429.md -->
# Swift Gemma4 Sample Maintainability Plan

## Goal

Turn `swift-gemma4-sample` from a one-off SwiftPM sample into a small, maintained reference project for on-device Gemma 4 experiments.

The project should stay narrow:

- prove `Gemma4SwiftCore` integration
- document local runtime constraints
- provide a future SwiftUI design system
- keep feature additions behind explicit workflow and checklist gates
- avoid becoming a vague agent product

## Current State

- `Package.swift` defines a SwiftPM executable target.
- `Sources/Gemma4Sample/Gemma4Sample.swift` runs the upstream quick-start flow.
- `DESIGN.md` defines UI direction for a future SwiftUI shell.
- `design-tokens.json` and `DesignTokens.swift` define primitive, semantic, and component tokens.
- `README.md` documents build and run commands.

## Proposed Additions

### 1. Feature Governance Docs

Add lightweight governance docs under `swift-gemma4-sample/docs/`, but keep the set small:

- `MAINTAINING.md`: feature inventory, checklist, workflow, release checks, and scope rules in one file.
- `DECISIONS.md`: append-only architecture decision log, created only when the first durable decision appears.

Do not create four separate governance docs yet. For a small sample, that is more process than product.

### 2. Testing Policy

Define the minimum checks now:

- `swift build -c release`
- `swift test` once reusable pure logic exists
- manual run only when model weights are available

The sample should not require downloading 1.5 GB weights just to pass normal CI.

Before adding new features, extract testable seams from `Gemma4Sample.swift`:

- generation settings construction
- prompt text construction
- expected failure classification

Do not make tests depend on model-weight downloads.

### 3. Scope Rules

Allowed:

- local Gemma 4 text generation sample
- prompt formatting examples using `Gemma4PromptFormatter`
- future SwiftUI shell using `DesignTokens`
- docs explaining Apple Silicon, MLX, and model download behavior
- compile-only CI for API drift detection
- troubleshooting docs for common first-run failures

Not allowed without a new plan:

- full autonomous agent behavior
- cloud inference
- cross-app control
- persistent memory store
- RAG system
- production iOS app claims

### 4. Design System Guardrails

Use `DESIGN.md` as the source of truth.

Future UI code should use:

- `DesignTokens.Primitive`
- `DesignTokens.Semantic`
- `DesignTokens.Component`

Avoid hardcoded colors and spacing in SwiftUI views once a UI target exists.

Before adding a real SwiftUI UI, decide whether `DesignTokens.swift` stays in the executable target or moves into a small UI support target. Do not let the CLI sample quietly become a UI product without a new plan.

### 5. Developer Experience

Keep the first successful path short:

```bash
cd swift-gemma4-sample
swift build
swift run gemma4-sample
```

Document that first run may download model weights and may take a long time.

Add a first-run transcript before broad sharing:

- expected commands
- expected timing ranges
- expected download behavior
- expected output shape
- cache and disk-space notes
- recovery steps for network, cache, memory, and unsupported hardware failures

### 6. CI Strategy

Add CI now if it can avoid model-weight downloads.

First CI version should run:

```bash
swift build -c release
```

When tests are introduced, add:

```bash
swift test
```

## Success Criteria

- A new maintainer can understand the project in under 5 minutes.
- Adding a new feature requires passing the checklist.
- The sample stays focused on `Gemma4SwiftCore`, not a product concept.
- Build verification remains fast and does not require model downloads.
- Future SwiftUI work has tokens and design rules before UI code grows.
- CI catches compile/API drift without downloading model weights.
- Expected first-run failures have cause and fix guidance.

## /autoplan Review Report

### Summary

Reviewed by CEO, Design, Eng, and DX lenses. Codex CLI was unavailable, so outside voice ran in subagent-only mode.

Consensus: the direction is right, but the original plan over-invested in governance documents and under-invested in CI, test seams, and first-run failure guidance.

### CEO Review

Stance: scope reduction.

Findings:

- High: process maintainability was ahead of sample usefulness. The sample must first be boring, runnable, and clear.
- Medium: SwiftUI design-system scope is useful only as a guardrail. Actual UI work needs a separate plan.
- Medium: the maintainer contract needs to name what stays current: `Gemma4SwiftCore` API use, prompt formatter, model ID, Apple Silicon constraints, first-run download behavior.

Auto-decision: replace four governance docs with `docs/MAINTAINING.md` plus `docs/DECISIONS.md` only when durable decisions exist.

### Design Review

Scores:

- Hierarchy: 6/10
- States: 4/10
- Tokens: 7/10
- Accessibility: 3/10
- Specificity: 7/10

Findings:

- High: future SwiftUI boundary is loose because `DesignTokens.swift` lives in the executable target.
- High: local-runtime states are incomplete: first-run download, cached model, unsupported device, cancelled generation, partial output, memory pressure.
- Medium: token naming is slightly split between `DESIGN.md`, JSON, and Swift.

Auto-decision: keep the quiet local console design language, but defer real UI implementation until there is a separate UI plan.

### Engineering Review

Architecture:

```text
Developer / CI
  |
  | swift build -c release
  v
SwiftPM Package
  |
  +-- executable target: Gemma4Sample
        |
        +-- DesignTokens.swift
        |     future SwiftUI-only concern
        |
        +-- Gemma4Sample.swift
              |
              +-- Gemma4Registration.registerIfNeeded()
              +-- LLMModelFactory.shared.loadContainer(...)
              |     may download/cache model weights
              |
              +-- Gemma4PromptFormatter.userTurn(...)
              +-- container.encode(...)
              +-- container.generate(...)
                    |
                    +-- stream chunks to stdout
                    +-- errors to stderr, exit(1)
```

Findings:

- P1: CI should not wait. Compile-only CI is the main API-drift guard.
- P1: `main` currently mixes registration, load, prompt formatting, encode, generate, stream, and process exit. Add test seams before new features.
- P2: four docs are too much for this sample. Use one operational maintaining doc.
- P2: first-run failure modes need explicit docs.

Failure modes to cover:

- unsupported Apple Silicon or MLX runtime
- Hugging Face/network failure
- interrupted or corrupt model cache
- disk pressure
- memory pressure during load
- upstream API/model ID changes
- prompt formatter API drift
- streaming error after partial output

### DX Review

Scores:

- Time to hello world: 5/10
- Docs: 6/10
- Commands: 7/10
- Errors/debugging: 3/10
- Examples: 4/10
- Upgrade path: 3/10
- Maintainer workflow: 7/10
- Overall: 5.4/10

Findings:

- First-run uncertainty is the biggest DX problem. A 1.5 GB download needs timing and recovery guidance.
- Error messages are too generic today.
- Upgrade path for `Swift-gemma4-core` and MLX dependencies is not defined.

Auto-decision: add first-run transcript, troubleshooting, and upstream upgrade checklist before promoting this sample.

### Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | CEO | Reduce governance docs to `MAINTAINING.md` plus optional `DECISIONS.md` | Mechanical | Explicit over clever | Four docs are more process than this sample needs | Four separate governance docs |
| 2 | CEO | Keep full agent/RAG/memory/product concepts out of scope | Mechanical | DRY and scope control | This sample exists to prove `Gemma4SwiftCore`, not become an agent product | Product-style expansion |
| 3 | Design | Keep design system as future guardrail, not UI implementation mandate | Taste | Bias toward action | Tokens are useful, but UI product work is premature | Building SwiftUI shell now |
| 4 | Eng | Add compile-only CI now | Mechanical | Choose completeness | CI catches dependency/API drift without model downloads | Deferring CI |
| 5 | Eng | Require test seams before new features | Mechanical | Explicit over clever | Pure helpers make future tests possible without model weights | Keeping all logic in `main` |
| 6 | DX | Add first-run transcript and troubleshooting before broad sharing | Mechanical | User impact | Developers need to know what they will see during long downloads | README-only happy path |

### NOT In Scope

- autonomous agent behavior
- cloud inference
- cross-app control
- persistent memory store
- RAG system
- production iOS app claims
- full SwiftUI shell without a new UI plan

### Final Recommendation

Approve this revised plan. Next implementation should create `docs/MAINTAINING.md`, add compile-only CI, and improve README first-run guidance before adding any new runtime feature.
