import Foundation
import SwiftUI
import Combine
import Gemma4SampleCore

struct GemmaMessage: Identifiable {
    let id = UUID()
    var text: String
    let isUser: Bool
}

@MainActor
final class Gemma4ConsoleViewModel: ObservableObject {
    @Published var prompt = ""
    @Published var messages: [GemmaMessage] = []
    @Published var logs: [String] = ["Ready. Use a real iPhone or iPad."]
    @Published var isWorking = false

    // LLM Parameters
    @Published var temperature: Float = 0.8 { didSet { saveSettings() } }
    @Published var maxTokens: Int = 200 { didSet { saveSettings() } }
    @Published var topP: Float = 0.95 { didSet { saveSettings() } }

    private let runtime = Gemma4Runtime()

    init() {
        loadSettings()
    }

    var canGenerate: Bool {
        !isWorking && !prompt.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
    }

    func generate() {
        guard canGenerate else { return }

        let userPrompt = prompt
        prompt = ""
        isWorking = true
        
        // Prepare history BEFORE adding the new turn
        let history = messages.suffix(10).map { 
            Gemma4ChatTurn(text: $0.text, isUser: $0.isUser) 
        }
        
        messages.append(GemmaMessage(text: userPrompt, isUser: true))
        messages.append(GemmaMessage(text: "", isUser: false))
        
        append("Starting generation (T=\(temperature), Max=\(maxTokens), History=\(history.count)).")

        Task { @MainActor in
            do {
                let source = try ModelLocation.firstAvailableModelSource()
                let settings = Gemma4GenerationSettings(
                    maxTokens: maxTokens,
                    temperature: temperature,
                    topP: topP
                )

                _ = try await runtime.generate(
                    userPrompt: userPrompt,
                    history: history,
                    source: source,
                    settings: settings,
                    onEvent: { [weak self] event in
                        Task { @MainActor in
                            self?.append(event.message)
                        }
                    },
                    onChunk: { [weak self] chunk in
                        Task { @MainActor in
                            guard let self = self else { return }
                            if let lastIndex = self.messages.indices.last {
                                self.messages[lastIndex].text += chunk
                            }
                        }
                    })

                append("Done.")
            } catch {
                append("Error: \(error)")
                if let lastIndex = messages.indices.last {
                    messages[lastIndex].text += "\n[Error: \(error)]"
                }
            }

            isWorking = false
        }
    }

    private func append(_ message: String) {
        let timestamp = Date.formattedNow
        logs.append("[\(timestamp)] \(message)")
    }

    private func loadSettings() {
        let defaults = UserDefaults.standard
        if defaults.object(forKey: "temperature") != nil {
            temperature = defaults.float(forKey: "temperature")
        }
        if defaults.object(forKey: "maxTokens") != nil {
            maxTokens = defaults.integer(forKey: "maxTokens")
        }
        if defaults.object(forKey: "topP") != nil {
            topP = defaults.float(forKey: "topP")
        }
    }

    private func saveSettings() {
        let defaults = UserDefaults.standard
        defaults.set(temperature, forKey: "temperature")
        defaults.set(maxTokens, forKey: "maxTokens")
        defaults.set(topP, forKey: "topP")
    }
}

private extension Date {
    static var formattedNow: String {
        let formatter = DateFormatter()
        formatter.dateFormat = "HH:mm:ss"
        return formatter.string(from: Date())
    }
}

