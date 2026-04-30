import Foundation
import MLX
import MLXEmbedders
import MLXLMCommon
import VecturaKit

// Local replacement for rryam/VecturaMLXKit's MLXEmbedder: that package still calls
// removed `MLXEmbedders.loadModelContainer` / `ModelContainer` APIs from older mlx-swift-lm.

/// On-device embedding for VecturaKit using mlx-swift-lm's ``EmbedderModelFactory`` / ``EmbedderModelContainer``.
public actor Gemma4MLXVecturaEmbedder: VecturaEmbedder {
    private let modelContainer: EmbedderModelContainer
    private let configuration: ModelConfiguration
    private let adaptiveBatchingEnabled: Bool
    private var cachedDimension: Int?

    public init(configuration: ModelConfiguration = EmbedderRegistry.nomic_text_v1_5) async throws {
        self.configuration = configuration
        self.adaptiveBatchingEnabled = Self.resolveAdaptiveBatchingSetting()
        self.modelContainer = try await EmbedderModelFactory.shared.loadContainer(
            from: Gemma4HuggingFaceDownloader(),
            using: Gemma4TransformersTokenizerLoader(),
            configuration: configuration
        )
    }

    public var dimension: Int {
        get async throws {
            if let cached = cachedDimension {
                return cached
            }
            let testEmbedding = try await embed(text: "test")
            let dim = testEmbedding.count
            cachedDimension = dim
            return dim
        }
    }

    public func embed(texts: [String]) async throws -> [[Float]] {
        guard !texts.isEmpty else {
            throw VecturaError.invalidInput("Cannot embed empty array of texts")
        }

        for (index, text) in texts.enumerated() {
            guard !text.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty else {
                throw VecturaError.invalidInput("Text at index \(index) cannot be empty or whitespace-only")
            }
        }

        return try await modelContainer.perform { (model: EmbeddingModel, tokenizer, pooling) -> [[Float]] in
            let inputs = texts.map {
                tokenizer.encode(text: $0, addSpecialTokens: true)
            }
            let batchPlans: [EmbeddingBatchPlan]
            if self.adaptiveBatchingEnabled {
                batchPlans = EmbeddingBatchPlanner.makePlans(tokenizedInputs: inputs)
            } else {
                batchPlans = [
                    EmbeddingBatchPlan(
                        originalIndices: Array(inputs.indices),
                        maxTokenLength: inputs.map(\.count).max() ?? 0
                    ),
                ]
            }

            let padToken = EmbeddingTokenResolver.paddingTokenID(
                eosTokenID: tokenizer.eosTokenId,
                unknownTokenID: tokenizer.unknownTokenId,
                bosTokenID: tokenizer.bosToken.flatMap { tokenizer.convertTokenToId($0) }
            )

            var vectors = Array(repeating: [Float](), count: texts.count)
            var emittedVectors = 0

            for plan in batchPlans {
                let padded = stacked(
                    plan.originalIndices.map { index in
                        var tokens = inputs[index]
                        if tokens.count < plan.maxTokenLength {
                            tokens.reserveCapacity(plan.maxTokenLength)
                            tokens.append(contentsOf: repeatElement(padToken, count: plan.maxTokenLength - tokens.count))
                        }
                        return MLXArray(tokens)
                    }
                )

                let sequenceLengths = plan.originalIndices.map { inputs[$0].count }
                let maskRows = EmbeddingTokenResolver.attentionMaskRows(
                    lengths: sequenceLengths,
                    maxLength: plan.maxTokenLength
                )
                let mask = stacked(maskRows.map { MLXArray($0) })
                let tokenTypes = MLXArray.zeros(like: padded)

                let outputs = model(
                    padded,
                    positionIds: nil,
                    tokenTypeIds: tokenTypes,
                    attentionMask: mask
                )

                let pooled = pooling(
                    outputs,
                    mask: mask,
                    normalize: true,
                    applyLayerNorm: true
                )
                pooled.eval()

                let finalEmbeddings: MLXArray
                switch pooled.ndim {
                case 2:
                    finalEmbeddings = pooled
                case 3:
                    finalEmbeddings = mean(pooled, axis: 1)
                    finalEmbeddings.eval()
                default:
                    throw Gemma4EmbeddingError.unsupportedPoolingShape(pooled.shape)
                }

                let batchVectors = finalEmbeddings.map { $0.asArray(Float.self) }
                guard batchVectors.count == plan.originalIndices.count else {
                    throw Gemma4EmbeddingError.vectorCountMismatch(
                        expected: plan.originalIndices.count,
                        received: batchVectors.count
                    )
                }

                for (offset, originalIndex) in plan.originalIndices.enumerated() {
                    vectors[originalIndex] = batchVectors[offset]
                    emittedVectors += 1
                }
            }

            guard emittedVectors == texts.count else {
                throw Gemma4EmbeddingError.vectorCountMismatch(expected: texts.count, received: emittedVectors)
            }
            return vectors
        }
    }
}

extension Gemma4MLXVecturaEmbedder {
    private static func resolveAdaptiveBatchingSetting() -> Bool {
        guard let rawValue = ProcessInfo.processInfo.environment["VECTURA_MLX_ADAPTIVE_BATCHING"] else {
            return true
        }
        let normalized = rawValue.trimmingCharacters(in: .whitespacesAndNewlines).lowercased()
        switch normalized {
        case "0", "false", "no", "off":
            return false
        default:
            return true
        }
    }
}

private enum Gemma4EmbeddingError: Error {
    case unsupportedPoolingShape([Int])
    case vectorCountMismatch(expected: Int, received: Int)
}

private struct EmbeddingBatchPlan {
    let originalIndices: [Int]
    let maxTokenLength: Int
}

private enum EmbeddingBatchPlanner {
    static let defaultMaxBatchSize = 32
    static let minimumPaddingReduction = 0.15

    static func makePlans(
        tokenizedInputs: [[Int]],
        maxBatchSize: Int = defaultMaxBatchSize,
        minimumPaddingReduction: Double = minimumPaddingReduction
    ) -> [EmbeddingBatchPlan] {
        precondition(maxBatchSize > 0, "maxBatchSize must be greater than zero")
        guard !tokenizedInputs.isEmpty else {
            return []
        }

        let lengths = tokenizedInputs.map(\.count)
        let allIndices = Array(lengths.indices)
        let singlePlan = [
            EmbeddingBatchPlan(
                originalIndices: allIndices,
                maxTokenLength: lengths.max() ?? 0
            ),
        ]

        guard tokenizedInputs.count > maxBatchSize else {
            return singlePlan
        }

        let sortedIndices = allIndices.sorted { lhs, rhs in
            lengths[lhs] < lengths[rhs]
        }

        var candidatePlans: [EmbeddingBatchPlan] = []
        candidatePlans.reserveCapacity((sortedIndices.count + maxBatchSize - 1) / maxBatchSize)

        var start = 0
        while start < sortedIndices.count {
            let end = min(start + maxBatchSize, sortedIndices.count)
            let batchIndices = Array(sortedIndices[start ..< end])
            let maxTokenLength = batchIndices.reduce(into: 0) { currentMax, index in
                currentMax = max(currentMax, lengths[index])
            }
            candidatePlans.append(EmbeddingBatchPlan(
                originalIndices: batchIndices,
                maxTokenLength: maxTokenLength
            ))
            start = end
        }

        let baselinePaddedTokenCount = paddedTokenCount(for: singlePlan)
        guard baselinePaddedTokenCount > 0 else {
            return singlePlan
        }

        let candidatePaddedTokenCount = paddedTokenCount(for: candidatePlans)
        let reduction = Double(baselinePaddedTokenCount - candidatePaddedTokenCount)
            / Double(baselinePaddedTokenCount)

        guard reduction >= minimumPaddingReduction else {
            return singlePlan
        }
        return candidatePlans
    }

    static func paddedTokenCount(for plans: [EmbeddingBatchPlan]) -> Int {
        plans.reduce(into: 0) { total, plan in
            total += plan.maxTokenLength * plan.originalIndices.count
        }
    }
}

private enum EmbeddingTokenResolver {
    static func paddingTokenID(
        eosTokenID: Int?,
        unknownTokenID: Int?,
        bosTokenID: Int?
    ) -> Int {
        eosTokenID ?? unknownTokenID ?? bosTokenID ?? 0
    }

    static func attentionMaskRows(lengths: [Int], maxLength: Int) -> [[Bool]] {
        lengths.map { length in
            let clampedLength = min(max(0, length), maxLength)
            let paddingCount = max(0, maxLength - clampedLength)
            return Array(repeating: true, count: clampedLength)
                + Array(repeating: false, count: paddingCount)
        }
    }
}
