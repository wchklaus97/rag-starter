# Gemma4 Device Sample

This folder contains the source skeleton for a physical-device iOS SwiftUI wrapper around the working `swift-gemma4-sample` runtime.

It is not simulator-supported. MLX requires real Apple GPU support.

## What This App Does

- Shows a small runtime console.
- Lets you type a prompt.
- Loads Gemma 4 through `Gemma4SampleCore`.
- Streams generated text into the UI.
- Shows status logs for registration, model load, prompt encoding, first token, and completion.

## Developer & Agent Guidelines

If you are an AI Agent or developer working on this codebase, **you must read and follow these critical documents before modifying any Swift code:**

- [Swift MLX Best Practices](../docs/SWIFT_MLX_BEST_PRACTICES.md): Mandatory rules for MLX tokenization, `@MainActor` concurrency, and preventing memory leaks.
- [User Acceptance Testing (UAT) Plan](../docs/UAT_PLAN.md): Formal testing procedures for model loading, stream performance, and hardware constraints.

## Xcode Setup

1. Open Xcode.
2. Create a new **iOS App** project named `Gemma4DeviceSample`.
3. Set interface to **SwiftUI**.
4. Set minimum iOS to **17.0**.
5. Add the local Swift package:

   ```text
   /Users/klaus_mac/Projects/04-Experiments/rag-starter/swift-gemma4-sample
   ```

6. Link the app target to the package products:
   
   ```text
   Gemma4SampleCore
   Gemma4SampleUI
   ```

7. Replace the generated app files with the files in:

   ```text
   ios-app/Gemma4DeviceSample/
   ```

8. Use `Info.plist` from this folder or copy these keys into the app target:

   ```text
   UIFileSharingEnabled = YES
   LSSupportsOpeningDocumentsInPlace = YES
   ```

9. Select a real iPhone or iPad, not a simulator.
10. Configure signing with your Apple ID/team.
11. Build and run.

## Model Loading Strategy

First version looks for the model folder in two places:

1. App Documents:

   ```text
   Documents/gemma-4-e2b-it-4bit
   ```

2. App bundle resources:

   ```text
   Bundle/gemma-4-e2b-it-4bit
   ```

Do not commit model files.

The model is large, around 3.4 GB in the current local download. For early device testing, the cleanest path is:

1. Install the app once.
2. Open terminal and use the `devicectl` utility to copy `gemma-4-e2b-it-4bit` into the app Documents folder:
   ```bash
   xcrun devicectl device info files \
     --device "Your Device Name" \
     --domain com.your-bundle-id \
     upload /path/to/models/gemma-4-e2b-it-4bit Documents/gemma-4-e2b-it-4bit
   ```
3. Relaunch the app.
4. Press **Generate**.

Bundling the model into the app can work for local testing, but it makes builds slow and app size huge. Use Documents first.

## Expected Success Log

The UI should show:

```text
Registering Gemma4SwiftCore.
Registration complete.
Loading model from local directory ...
Model loaded.
Encoding prompt.
Prompt encoded: N tokens.
Generating.
First token chunk received.
Generation complete.
```

## Known Limits

- No iOS Simulator support.
- No cloud inference.
- No RAG.
- No memory store.
- No App Intents.
- No App Store packaging.
- Physical-device smoke testing still must be done manually in Xcode.
