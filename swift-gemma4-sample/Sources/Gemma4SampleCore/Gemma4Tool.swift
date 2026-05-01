import Foundation

/// Tool contract for the on-device agent (future: map model `<tool_call>` JSON to concrete actions).
public protocol Gemma4Tool: Sendable {
    var identifier: String { get }
    var displayName: String { get }
    /// Short description for system prompts or settings UI.
    var summary: String { get }

    /// Run with JSON arguments from the model; return a string to append to the conversation as tool output.
    func run(argumentsJSON: String) async throws -> String
}

/// Registers tools by `identifier` for orchestration layers (SwiftAI / AgentRunKit / custom loops).
public actor Gemma4ToolRegistry {
    private var tools: [String: any Gemma4Tool] = [:]

    public init() {}

    public func register(_ tool: some Gemma4Tool) {
        tools[tool.identifier] = tool
    }

    public func tool(identifier: String) -> (any Gemma4Tool)? {
        tools[identifier]
    }

    public var allIdentifiers: [String] {
        tools.keys.sorted()
    }

    public func runTool(identifier: String, argumentsJSON: String) async throws -> String {
        guard let tool = tools[identifier] else {
            throw Gemma4ToolError.unknownTool(identifier)
        }
        return try await tool.run(argumentsJSON: argumentsJSON)
    }
}

public enum Gemma4ToolError: Error, Equatable {
    case unknownTool(String)
}

/// Minimal no-op tool for wiring tests and UI without App Intents.
public struct Gemma4EchoTool: Gemma4Tool {
    public let identifier = "echo"
    public let displayName = "Echo"
    public let summary = "Returns the tool arguments unchanged (stub)."

    public init() {}

    public func run(argumentsJSON: String) async throws -> String {
        argumentsJSON
    }
}
