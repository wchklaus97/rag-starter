// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "Gemma4Sample",
    platforms: [
        .macOS(.v14),
        .iOS(.v17),
    ],
    products: [
        .executable(name: "gemma4-sample", targets: ["Gemma4Sample"]),
        .library(name: "Gemma4SampleCore", targets: ["Gemma4SampleCore"]),
        .library(name: "Gemma4SampleUI", targets: ["Gemma4SampleUI"]),
    ],
    dependencies: [
        .package(
            url: "https://github.com/yejingyang8963-byte/Swift-gemma4-core.git",
            from: "0.1.0"),
        .package(
            url: "https://github.com/gonzalezreal/swift-markdown-ui",
            from: "2.4.0"),
    ],
    targets: [
        .target(
            name: "Gemma4SampleCore",
            dependencies: [
                .product(name: "Gemma4SwiftCore", package: "Swift-gemma4-core"),
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
