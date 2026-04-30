# Swift MLX Best Practices for Gemma 4

This document defines the critical architectural constraints, concurrency patterns, and MLX tokenization requirements for building the iOS device app in `swift-gemma4-sample`. 

**ALL AI AGENTS AND DEVELOPERS MUST ADHERE TO THESE GUIDELINES TO PREVENT FATAL CRASHES, SILENT FAILURES, OR MEMORY LEAKS.**

## 1. MLX Tokenization (The `<bos>` Token Rule)

> [!WARNING]
> NEVER manually format prompt strings and encode them using literal text processing (e.g. `container.encode("<start_of_turn>user...")`).

Gemma models absolutely require a Begin Of Sequence (`<bos>`) token at the start of the input. If this is missing, the model will immediately emit an `<eos>` token, resulting in a zero-token response stream that finishes instantly.

### Incorrect Method (Do NOT do this)
```swift
// WRONG: This treats special tokens as literal strings and omits <bos>.
let prompt = "<start_of_turn>user\nHello<end_of_turn>\n<start_of_turn>model\n"
let tokens = await container.encode(prompt)
```

### Correct Method
Use `applyChatTemplate` combined with message dictionaries. This leverages the MLX Swift LM native tokenizer processor, reads `tokenizer_config.json`, formats the text correctly, and injects the `<bos>` token.
```swift
// CORRECT: Lets MLX apply the ChatML template and inject <bos>.
let messages: [[String: String]] = [
    ["role": "user", "content": "Hello"]
]
let tokens = try await container.applyChatTemplate(messages: messages)
let input = LMInput(tokens: MLXArray(tokens))
```

## 2. Concurrency & `@MainActor` Boundaries

MLX generations execute on background metal threads. SwiftUI views operate on the Main thread.

> [!CAUTION]
> Updating UI state from the MLX generation loop will cause an immediate fatal crash on iOS.

All classes that conform to `ObservableObject` representing the UI state must be decorated with `@MainActor`. 
Whenever you are passing escaping closures to `onEvent` or `onChunk` from `Gemma4Runtime`, you must wrap the UI updates in `Task { @MainActor in }`.

### Correct Concurrency Bridge
```swift
_ = try await runtime.generate(
    // ...
    onChunk: { [weak self] chunk in
        Task { @MainActor in
            guard let self = self else { return }
            // Safe to update UI state here
        }
    }
)
```

## 3. Retain Cycles and Memory Management

A 4GB large language model will place the iOS device under extreme memory pressure. 

> [!IMPORTANT]
> A retain cycle that prevents the ViewModel and ModelContainer from deallocating will lead to an immediate Out-Of-Memory (OOM) crash if the user rapidly switches views.

All closures passed to the `Gemma4Runtime` MUST capture `[weak self]`. 

```swift
// MUST INCLUDE [weak self]
onEvent: { [weak self] event in
    Task { @MainActor in
        self?.append(event.message)
    }
}
```

## 4. SwiftUI Reactive Arrays

When updating a `@Published` array of objects (e.g., `messages`), ensure the objects are value types (`struct`). Mutating a property of a `struct` element in an array safely triggers `objectWillChange`.

If you need to update the last element of the chat with streaming chunks:
```swift
if let lastIndex = self.messages.indices.last {
    self.messages[lastIndex].text += chunk
}
```
*Note: Ensure your SwiftUI `ForEach` uses a stable `.id(message.id)` (like a `UUID`) to prevent visual flickering during the rapid text updates.*
