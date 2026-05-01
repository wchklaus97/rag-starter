import Darwin
import Foundation
import Gemma4SampleCore

@main
enum Gemma4SampleMain {
    private static let startedAt = Date()

    static func main() async {
        do {
            let config = Gemma4SampleConfig(environment: ProcessInfo.processInfo.environment)
            
            if config.usesRemoteModel {
                log("No GEMMA4_MODEL_DIR set. First run may download about 1.5 GB from Hugging Face.")
            }

            let runtime = Gemma4Runtime()
            _ = try await runtime.generate(
                userPrompt: "Tell me a short story about a curious fox.",
                source: config.modelSource,
                onEvent: { event in
                    log(event.message)
                },
                onChunk: { text in
                    print(text, terminator: "")
                })
            print()
        } catch {
            log("Error: \(error)")
            exit(1)
        }
    }

    private static func log(_ message: String) {
        let elapsed = Date().timeIntervalSince(startedAt)
        fputs(String(format: "[gemma4-sample +%.1fs] %@\n", elapsed, message), stderr)
        fflush(stderr)
    }
}
