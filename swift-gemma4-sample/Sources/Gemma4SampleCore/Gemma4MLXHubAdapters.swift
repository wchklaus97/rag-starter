import Foundation
import HuggingFace
import MLXLMCommon
import Tokenizers

enum Gemma4MLXHubError: Error {
    case invalidRepositoryID(String)
}

/// Hub snapshot download for mlx-swift-lm ``Downloader``.
struct Gemma4HuggingFaceDownloader: MLXLMCommon.Downloader {
    private let client: HubClient

    init(client: HubClient = .default) {
        self.client = client
    }

    func download(
        id: String,
        revision: String?,
        matching patterns: [String],
        useLatest _: Bool,
        progressHandler: @escaping @Sendable (Progress) -> Void
    ) async throws -> URL {
        guard let repoID = HuggingFace.Repo.ID(rawValue: id) else {
            throw Gemma4MLXHubError.invalidRepositoryID(id)
        }

        return try await client.downloadSnapshot(
            of: repoID,
            revision: revision ?? "main",
            matching: patterns,
            progressHandler: { @MainActor progress in
                progressHandler(progress)
            }
        )
    }
}

/// Loads tokenizers via swift-transformers for mlx-swift-lm.
struct Gemma4TransformersTokenizerLoader: MLXLMCommon.TokenizerLoader {
    func load(from directory: URL) async throws -> any MLXLMCommon.Tokenizer {
        let upstream = try await AutoTokenizer.from(modelFolder: directory)
        return Gemma4HuggingFaceTokenizerBridge(upstream)
    }
}

struct Gemma4HuggingFaceTokenizerBridge: MLXLMCommon.Tokenizer {
    private let upstream: any Tokenizers.Tokenizer

    init(_ upstream: any Tokenizers.Tokenizer) {
        self.upstream = upstream
    }

    func encode(text: String, addSpecialTokens: Bool) -> [Int] {
        upstream.encode(text: text, addSpecialTokens: addSpecialTokens)
    }

    func decode(tokenIds: [Int], skipSpecialTokens: Bool) -> String {
        upstream.decode(tokens: tokenIds, skipSpecialTokens: skipSpecialTokens)
    }

    func convertTokenToId(_ token: String) -> Int? {
        upstream.convertTokenToId(token)
    }

    func convertIdToToken(_ id: Int) -> String? {
        upstream.convertIdToToken(id)
    }

    var bosToken: String? { upstream.bosToken }
    var eosToken: String? { upstream.eosToken }
    var unknownToken: String? { upstream.unknownToken }

    func applyChatTemplate(
        messages: [[String: any Sendable]],
        tools: [[String: any Sendable]]?,
        additionalContext: [String: any Sendable]?
    ) throws -> [Int] {
        do {
            return try upstream.applyChatTemplate(
                messages: messages,
                tools: tools,
                additionalContext: additionalContext
            )
        } catch Tokenizers.TokenizerError.missingChatTemplate {
            throw MLXLMCommon.TokenizerError.missingChatTemplate
        }
    }
}
