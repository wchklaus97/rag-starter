import SwiftUI
import Gemma4SampleUI

struct Gemma4SettingsView: View {
    @ObservedObject var viewModel: Gemma4ConsoleViewModel
    @Environment(\.dismiss) var dismiss
    
    var body: some View {
        NavigationStack {
            Form {
                Section("Generation Parameters") {
                    VStack(alignment: .leading) {
                        HStack {
                            Text("Temperature")
                            Spacer()
                            Text(String(format: "%.1f", viewModel.temperature))
                                .foregroundStyle(DesignTokens.Semantic.accent)
                                .bold()
                        }
                        Slider(value: $viewModel.temperature, in: 0.1...2.0, step: 0.1)
                            .tint(DesignTokens.Semantic.accent)
                        Text("Controls creativity. Lower is more deterministic, higher is more creative.")
                            .font(.caption)
                            .foregroundStyle(DesignTokens.Semantic.textSecondary)
                    }
                    .padding(.vertical, DesignTokens.Spacing.xs)
                    
                    VStack(alignment: .leading) {
                        HStack {
                            Text("Max Tokens")
                            Spacer()
                            Text("\(viewModel.maxTokens)")
                                .foregroundStyle(DesignTokens.Semantic.accent)
                                .bold()
                        }
                        Slider(value: Binding(
                            get: { Float(viewModel.maxTokens) },
                            set: { viewModel.maxTokens = Int($0) }
                        ), in: 50...1024, step: 50)
                            .tint(DesignTokens.Semantic.accent)
                        Text("Maximum length of the assistant response.")
                            .font(.caption)
                            .foregroundStyle(DesignTokens.Semantic.textSecondary)
                    }
                    .padding(.vertical, DesignTokens.Spacing.xs)
                    
                    VStack(alignment: .leading) {
                        HStack {
                            Text("Top P")
                            Spacer()
                            Text(String(format: "%.2f", viewModel.topP))
                                .foregroundStyle(DesignTokens.Semantic.accent)
                                .bold()
                        }
                        Slider(value: $viewModel.topP, in: 0.0...1.0, step: 0.05)
                            .tint(DesignTokens.Semantic.accent)
                        Text("Nucleus sampling: probability mass to consider.")
                            .font(.caption)
                            .foregroundStyle(DesignTokens.Semantic.textSecondary)
                    }
                    .padding(.vertical, DesignTokens.Spacing.xs)
                }
                
                Section("About") {
                    HStack {
                        Text("Model")
                        Spacer()
                        Text("Gemma 4 (Local)")
                            .foregroundStyle(DesignTokens.Semantic.textSecondary)
                    }
                    HStack {
                        Text("Hardware")
                        Spacer()
                        Text("Apple Silicon / MLX")
                            .foregroundStyle(DesignTokens.Semantic.textSecondary)
                    }
                }
            }
            .navigationTitle("AI Settings")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
        }
    }
}
