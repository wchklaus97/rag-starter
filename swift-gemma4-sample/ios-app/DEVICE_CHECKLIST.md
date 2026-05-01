# Real Device Checklist

Use this checklist before saying the iPhone app works.

## Device

- Real iPhone or iPad selected in Xcode.
- iOS 17 or newer.
- Enough free storage for a multi-GB model folder.
- Signing team configured.
- Simulator is not selected.

## App Target

- App target links `Gemma4SampleCore`.
- App includes the Swift files from `ios-app/Gemma4DeviceSample/`.
- `UIFileSharingEnabled` is enabled if using app Documents transfer.
- `LSSupportsOpeningDocumentsInPlace` is enabled if using app Documents transfer.

## Model Files

The device must contain:

```text
gemma-4-e2b-it-4bit/config.json
gemma-4-e2b-it-4bit/tokenizer.json
gemma-4-e2b-it-4bit/model.safetensors
```

Preferred location:

```text
App Documents/gemma-4-e2b-it-4bit
```

Alternative location:

```text
App Bundle/gemma-4-e2b-it-4bit
```

## Smoke Test

- Launch app on device.
- Enter prompt.
- Tap **Generate**.
- Confirm log reaches `Model loaded`.
- Confirm log reaches `First token chunk received`.
- Confirm generated text appears in Output.

## Failure Recovery

- If model directory is missing, copy the model into app Documents and relaunch.
- If signing fails, fix Xcode team and bundle identifier.
- If device memory pressure appears, restart the device and test with no other heavy apps open.
- If build uses Simulator, switch to a physical device.
