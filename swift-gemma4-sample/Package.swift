// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "Gemma4Sample",
    platforms: [
        .macOS(.v15),
        .iOS(.v18),
    ],
    products: [
        .executable(name: "gemma4-sample", targets: ["Gemma4Sample"]),
        .library(name: "Gemma4SampleCore", targets: ["Gemma4SampleCore"]),
        .library(name: "Gemma4SampleUI", targets: ["Gemma4SampleUI"]),
    ],
    dependencies: [
        // Vendored fork: upstream 0.1.0 lacks `ModelTypeRegistry<LanguageModel>` after mlx-swift-lm made the registry generic.
        .package(path: "Vendor/Swift-gemma4-core"),
        .package(
            url: "https://github.com/gonzalezreal/swift-markdown-ui",
            from: "2.4.0"),
        .package(
            url: "https://github.com/rryam/VecturaKit.git",
            from: "5.0.0"),
        .package(
            url: "https://github.com/huggingface/swift-transformers",
            from: "1.3.0"),
        .package(
            url: "https://github.com/huggingface/swift-huggingface.git",
            from: "0.9.0"),
        .package(
            url: "https://github.com/ml-explore/mlx-swift-lm",
            branch: "main"),
    ],
    targets: [
        .target(
            name: "Gemma4SampleCore",
            dependencies: [
                .product(name: "Gemma4SwiftCore", package: "Swift-gemma4-core"),
                .product(name: "VecturaKit", package: "VecturaKit"),
                .product(name: "MLXEmbedders", package: "mlx-swift-lm"),
                .product(name: "MLXLMCommon", package: "mlx-swift-lm"),
                .product(name: "Tokenizers", package: "swift-transformers"),
                .product(name: "HuggingFace", package: "swift-huggingface"),
            ]),
        .target(
            name: "Gemma4SampleUI",
            dependencies: [
                "Gemma4SampleCore",
                .product(name: "MarkdownUI", package: "swift-markdown-ui"),
            ]
        ),
        .executableTarget(
            name: "Gemma4Sample",
            dependencies: [
                "Gemma4SampleCore",
            ]),
        .testTarget(
            name: "Gemma4SampleTests",
            dependencies: ["Gemma4SampleCore"]
        ),
    ]
)
