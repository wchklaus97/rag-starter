import Foundation

#if canImport(AppIntents)
import AppIntents

/// Bridge layer: map Gemma tool calls to system **App Intents** (Calendar, Reminders, etc.).
/// Implement app-target `AppIntent` types, then invoke them from `Gemma4Tool` conformers.
///
/// This type is a placeholder until each intent is defined in the app bundle.
public enum Gemma4AppIntentBridge {
    /// Hook for a future `AppIntent.perform()` dispatch from parsed model output.
    public static var isAppIntentsAvailable: Bool { true }

    /// Placeholder — replace with real intent selection + parameter filling from `argumentsJSON`.
    public static func performPlaceholder() async throws -> String {
        "Gemma4AppIntentBridge: define AppIntent types in the app target and dispatch from Gemma4Tool."
    }
}

#else

public enum Gemma4AppIntentBridge {
    public static var isAppIntentsAvailable: Bool { false }

    public static func performPlaceholder() async throws -> String {
        throw Gemma4AppIntentBridgeError.appIntentsUnavailable
    }
}

public enum Gemma4AppIntentBridgeError: Error {
    case appIntentsUnavailable
}

#endif
