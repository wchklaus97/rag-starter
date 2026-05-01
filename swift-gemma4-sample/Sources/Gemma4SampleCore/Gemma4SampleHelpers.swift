import Foundation

public struct Gemma4SampleConfig {
    private let environment: [String: String]
    
    public init(environment: [String: String]) {
        self.environment = environment
    }
    
    public var usesRemoteModel: Bool {
        (environment["GEMMA4_MODEL_DIR"] ?? "").isEmpty
    }
    
    public var modelSource: Gemma4ModelSource {
        if let modelDir = environment["GEMMA4_MODEL_DIR"], !modelDir.isEmpty {
            return .localDirectory(URL(fileURLWithPath: modelDir))
        }
        return .verifiedHuggingFace
    }
}

public extension Gemma4RuntimeEvent {
    var message: String {
        switch self {
        case .starting:
            return "Starting Gemma4 sample."
        case .registering:
            return "Registering Gemma4SwiftCore with mlx-swift-lm."
        case .registrationComplete:
            return "Registration complete."
        case .loadingModel(let source):
            return "Loading model: \(source)"
        case .modelLoaded:
            return "Model loaded."
        case .encodingPrompt:
            return "Encoding prompt."
        case .promptEncoded(let count):
            return "Prompt encoded: \(count) tokens."
        case .generating:
            return "Generating response."
        case .firstChunkReceived:
            return "First token chunk received. Streaming output to stdout."
        case .generationComplete:
            return "Generation complete."
        }
    }
}
