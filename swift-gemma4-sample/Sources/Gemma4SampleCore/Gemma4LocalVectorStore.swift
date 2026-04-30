import Foundation
import MLXEmbedders
import MLXLMCommon
import VecturaKit

/// One retrieved chunk for injecting into a Gemma system prompt (local RAG).
public struct Gemma4RetrievedChunk: Equatable, Sendable {
    public let id: UUID
    public let text: String
    public let score: Float

    public init(id: UUID, text: String, score: Float) {
        self.id = id
        self.text = text
        self.score = score
    }
}

/// Thin wrapper around [VecturaKit](https://github.com/rryam/VecturaKit) with MLX embeddings via mlx-swift-lm (`Gemma4MLXVecturaEmbedder`).
public actor Gemma4LocalVectorStore {
    private let kit: VecturaKit

    /// Opens or creates a vector store under `directoryURL` using the default MLX embedding model (Nomic v1.5 via `Gemma4MLXVecturaEmbedder`).
    public static func open(
        name: String = "gemma4-local-rag",
        directoryURL: URL,
        embeddingConfiguration: ModelConfiguration = EmbedderRegistry.nomic_text_v1_5
    ) async throws -> Gemma4LocalVectorStore {
        let embedder = try await Gemma4MLXVecturaEmbedder(configuration: embeddingConfiguration)
        let config = try VecturaConfig(
            name: name,
            directoryURL: directoryURL,
            dimension: nil,
            searchOptions: .init(
                defaultNumResults: 10,
                minThreshold: 0,
                hybridWeight: 0.5
            )
        )
        let kit = try await VecturaKit(config: config, embedder: embedder)
        return Gemma4LocalVectorStore(kit: kit)
    }

    private init(kit: VecturaKit) {
        self.kit = kit
    }

    public func addDocuments(texts: [String]) async throws -> [UUID] {
        try await kit.addDocuments(texts: texts)
    }

    public func search(query: String, numResults: Int = 5, threshold: Float? = nil) async throws
        -> [Gemma4RetrievedChunk]
    {
        let rows = try await kit.search(query: .text(query), numResults: numResults, threshold: threshold)
        return rows.map { Gemma4RetrievedChunk(id: $0.id, text: $0.text, score: $0.score) }
    }

    /// Concatenates retrieved chunks into a single context block for prompting.
    public static func buildContextBlock(from chunks: [Gemma4RetrievedChunk]) -> String {
        chunks.enumerated().map { index, chunk in
            "[\(index + 1)] (score: \(String(format: "%.3f", chunk.score)))\n\(chunk.text)"
        }.joined(separator: "\n\n")
    }
}
