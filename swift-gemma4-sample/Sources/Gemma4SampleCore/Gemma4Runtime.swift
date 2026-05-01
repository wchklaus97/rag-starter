import Foundation
import Gemma4SwiftCore
import MLX
import MLXLLM
import MLXLMCommon

public enum Gemma4RuntimeEvent: Equatable, Sendable {
    case starting
    case registering
    case registrationComplete
    case loadingModel(String)
    case modelLoaded
    case encodingPrompt
    case promptEncoded(Int)
    case generating
    case firstChunkReceived
    case generationComplete
}

public enum Gemma4ModelSource: Equatable, Sendable {
    case verifiedHuggingFace
    case localDirectory(URL)

    public var description: String {
        switch self {
        case .verifiedHuggingFace:
            return Gemma4SwiftCore.verifiedModelId
        case .localDirectory(let url):
            return "local directory \(url.path)"
        }
    }

    var configuration: ModelConfiguration {
        switch self {
        case .verifiedHuggingFace:
            return ModelConfiguration(id: Gemma4SwiftCore.verifiedModelId)
        case .localDirectory(let url):
            return ModelConfiguration(directory: url)
        }
    }
}

public struct Gemma4GenerationSettings: Equatable, Sendable {
    public var maxTokens: Int
    public var temperature: Float
    public var topP: Float

    public init(maxTokens: Int = 200, temperature: Float = 0.8, topP: Float = 0.95) {
        self.maxTokens = maxTokens
        self.temperature = temperature
        self.topP = topP
    }
}

public struct Gemma4ChatTurn: Equatable, Sendable {
    public let text: String
    public let isUser: Bool
    
    public init(text: String, isUser: Bool) {
        self.text = text
        self.isUser = isUser
    }
}

public actor Gemma4Runtime {
    private var container: ModelContainer?

    public init() {}

    public func load(
        source: Gemma4ModelSource,
        onEvent: @Sendable (Gemma4RuntimeEvent) -> Void = { _ in }
    ) async throws {
        if container != nil {
            // Model already cached — still emit the full registration sequence
            // so callers always see a consistent event log.
            onEvent(.registering)
            onEvent(.registrationComplete)
            onEvent(.modelLoaded)
            return
        }

        onEvent(.registering)
        await Gemma4Registration.registerIfNeeded().value
        onEvent(.registrationComplete)

        onEvent(.loadingModel(source.description))
        container = try await LLMModelFactory.shared.loadContainer(
            from: Gemma4HuggingFaceDownloader(),
            using: Gemma4TransformersTokenizerLoader(),
            configuration: source.configuration
        )
        onEvent(.modelLoaded)
    }

    public func generate(
        userPrompt: String,
        history: [Gemma4ChatTurn] = [],
        source: Gemma4ModelSource,
        settings: Gemma4GenerationSettings = Gemma4GenerationSettings(),
        onEvent: @Sendable (Gemma4RuntimeEvent) -> Void = { _ in },
        onChunk: @Sendable (String) -> Void = { _ in }
    ) async throws -> String {
        onEvent(.starting)
        try await load(source: source, onEvent: onEvent)

        var messages: [[String: String]] = []
        for turn in history {
            messages.append(["role": turn.isUser ? "user" : "model", "content": turn.text])
        }
        messages.append(["role": "user", "content": userPrompt])

        onEvent(.encodingPrompt)
        guard let container else {
            throw Gemma4RuntimeError.modelNotLoaded
        }

        let tokens = try await container.applyChatTemplate(messages: messages)
        let input = LMInput(tokens: MLXArray(tokens))
        onEvent(.promptEncoded(tokens.count))

        onEvent(.generating)
        let stream = try await container.generate(
            input: input,
            parameters: GenerateParameters(
                maxTokens: settings.maxTokens,
                temperature: settings.temperature,
                topP: settings.topP))

        var output = ""
        var sawFirstChunk = false
        for await event in stream {
            if case .chunk(let text) = event {
                if !sawFirstChunk {
                    onEvent(.firstChunkReceived)
                    sawFirstChunk = true
                }
                output += text
                onChunk(text)
            }
        }

        onEvent(.generationComplete)
        return output
    }

}

public enum Gemma4RuntimeError: Error, Equatable {
    case modelNotLoaded
}
