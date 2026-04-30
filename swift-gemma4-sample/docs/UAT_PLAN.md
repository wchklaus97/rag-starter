# User Acceptance Testing (UAT) Plan for iOS Gemma 4

This document defines the formal UAT checklist for the `Gemma4DeviceSample` iOS application. Given the complexity of running a 4GB language model on mobile hardware, strict testing must be adhered to before approving any agent-submitted or human-authored PRs.

## 1. Deployment & Sandbox Testing

The app relies on local file loading via the iOS App sandbox's `Documents` folder.

- [ ] **Transfer Verification:** Use `xcrun devicectl device info files` to ensure the 4GB `gemma-4-e2b-it-4bit` directory exists in the container's Documents folder.
- [ ] **First Launch:** Ensure the app displays "Model loaded." and does not crash upon opening.
- [ ] **Missing Model Fallback:** If the model folder is intentionally deleted, the UI should cleanly display "Error: modelNotLoaded" when generation is attempted, instead of forcefully unwrapping and crashing.

## 2. LLM Generation Lifecycle

Ensure the MLX stream state machine is correctly tracking from input to output.

- [ ] **Cold Start Latency:** Measure the time from "Generating response" to the first streaming chunk. It should not exceed ~5 seconds on an A15+ chip.
- [ ] **Zero-Length Input:** Attempting to submit an empty or whitespace-only prompt should be caught by the UI (Send button disabled or cleanly ignored).
- [ ] **Continuous Streaming:** The response text should progressively appear on the screen, updating the `MessageBubbleView` smoothly.
- [ ] **Auto-scroll Behavior:** As chunks append to the latest message, the `ScrollViewReader` should smoothly push the view to the bottom.

## 3. Context & History Window Constraints

The application must track turn history properly.

- [ ] **Chat History Depth:** Verify that previous turns are appended to the MLX `applyChatTemplate` array correctly. The AI should remember details from 2-3 messages ago.
- [ ] **Max Tokens Enforcement:** Change `Max Tokens` to 50 in the Settings view. The generation should explicitly cut off early without throwing a framework error.

## 4. Hardware Resource Constraints (Stress Testing)

Testing how the app responds to high memory pressure is critical on mobile.

- [ ] **Rapid Sequential Generation:** Send a new prompt immediately after one finishes. Memory should plateau, not climb indefinitely.
- [ ] **Background Suspend:** Start a generation, minimize the app to the iOS home screen, and reopen it 10 seconds later. Ensure the MLX `ModelContainer` does not corrupt state.
- [ ] **Temperature Extremes:** Set `Temperature` to `2.0` (max chaos) and `0.1` (deterministic). Ensure the model generates appropriately varied outputs without triggering `mlxc` library crashes.

> [!NOTE]
> If any UAT test fails, consult the `SWIFT_MLX_BEST_PRACTICES.md` document to verify if concurrency or memory management rules have been violated.
