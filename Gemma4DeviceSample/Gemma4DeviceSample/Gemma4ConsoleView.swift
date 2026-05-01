import SwiftUI
import Gemma4SampleUI
import MarkdownUI

struct Gemma4ConsoleView: View {
    @StateObject private var viewModel = Gemma4ConsoleViewModel()
    @State private var showSettings = false

    var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                ScrollViewReader { proxy in
                    ScrollView {
                        VStack(alignment: .leading, spacing: DesignTokens.Spacing.m) {
                            statusCard
                            
                            ForEach(viewModel.messages) { message in
                                MessageBubbleView(message: message)
                                    .id(message.id)
                            }
                            
                            logsCard
                        }
                        .padding(DesignTokens.Spacing.l)
                    }
                    .onChange(of: viewModel.messages.last?.text) { _, _ in
                        if let lastId = viewModel.messages.last?.id {
                            withAnimation {
                                proxy.scrollTo(lastId, anchor: .bottom)
                            }
                        }
                    }
                }
                
                promptInputArea
            }
            .navigationTitle("Gemma 4")
            .background(DesignTokens.Semantic.background)
            .toolbar {
                ToolbarItem(placement: .topBarTrailing) {
                    Button {
                        showSettings = true
                    } label: {
                        Image(systemName: "gearshape.fill")
                            .foregroundStyle(DesignTokens.Semantic.accent)
                    }
                }
            }
            .sheet(isPresented: $showSettings) {
                Gemma4SettingsView(viewModel: viewModel)
            }
        }
    }

    private var statusCard: some View {
        VStack(alignment: .leading, spacing: DesignTokens.Spacing.s) {
            Text("Real-device runtime")
                .font(.headline)
                .foregroundStyle(DesignTokens.Semantic.textPrimary)
            Text("Simulator is unsupported. Run this on a physical iPhone or iPad with the model folder available.")
                .font(.footnote)
                .foregroundStyle(DesignTokens.Semantic.textSecondary)
            Text(viewModel.isWorking ? "Working" : "Idle")
                .font(.caption.weight(.semibold))
                .foregroundStyle(.white)
                .padding(.horizontal, DesignTokens.Component.StatusPill.horizontalPadding)
                .padding(.vertical, DesignTokens.Component.StatusPill.verticalPadding)
                .background(viewModel.isWorking ? DesignTokens.Component.StatusPill.loading : DesignTokens.Component.StatusPill.ready)
                .clipShape(Capsule())
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(DesignTokens.Spacing.l)
        .background(DesignTokens.Semantic.surface)
        .clipShape(RoundedRectangle(cornerRadius: DesignTokens.Radius.large))
    }

    private var promptInputArea: some View {
        VStack(spacing: 0) {
            Divider()
            HStack(alignment: .bottom, spacing: DesignTokens.Spacing.s) {
                TextField("Ask something...", text: $viewModel.prompt, axis: .vertical)
                    .textFieldStyle(.plain)
                    .padding(DesignTokens.Spacing.s)
                    .background(DesignTokens.Semantic.surface)
                    .clipShape(RoundedRectangle(cornerRadius: DesignTokens.Radius.small))
                    .overlay(
                        RoundedRectangle(cornerRadius: DesignTokens.Radius.small)
                            .stroke(DesignTokens.Semantic.border)
                    )
                    .lineLimit(1...5)
                
                Button {
                    viewModel.generate()
                } label: {
                    Image(systemName: "arrow.up.circle.fill")
                        .font(.system(size: 32))
                        .foregroundStyle(viewModel.canGenerate ? DesignTokens.Semantic.accent : DesignTokens.Semantic.textMuted)
                }
                .disabled(!viewModel.canGenerate)
            }
            .padding(DesignTokens.Spacing.m)
            .background(DesignTokens.Semantic.background)
        }
    }

    private var logsCard: some View {
        DisclosureGroup {
            VStack(alignment: .leading, spacing: DesignTokens.Spacing.xs) {
                ForEach(viewModel.logs.indices, id: \.self) { index in
                    Text(viewModel.logs[index])
                        .font(.caption2.monospaced())
                        .foregroundStyle(DesignTokens.Semantic.textSecondary)
                }
            }
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(.top, DesignTokens.Spacing.s)
        } label: {
            Text("Runtime Logs")
                .font(.caption)
                .foregroundStyle(DesignTokens.Semantic.textSecondary)
        }
        .padding(DesignTokens.Spacing.m)
        .background(DesignTokens.Semantic.surface.opacity(0.5))
        .clipShape(RoundedRectangle(cornerRadius: DesignTokens.Radius.medium))
    }
}

struct MessageBubbleView: View {
    let message: GemmaMessage
    
    var body: some View {
        HStack {
            if message.isUser { Spacer(minLength: 40) }
            
            Markdown(message.text.isEmpty ? "..." : message.text)
                .markdownTheme(.gemma)
                .padding(DesignTokens.Component.MessageBubble.padding)
                .foregroundStyle(message.isUser ? .white : DesignTokens.Semantic.textPrimary)
                .background(message.isUser ? DesignTokens.Component.MessageBubble.userBackground : DesignTokens.Component.MessageBubble.assistantBackground)
                .clipShape(RoundedRectangle(cornerRadius: DesignTokens.Component.MessageBubble.radius))
                .shadow(color: .black.opacity(0.05), radius: 2, y: 1)
            
            if !message.isUser { Spacer(minLength: 40) }
        }
    }
}

extension Theme {
    static let gemma = Theme()
        .text {
            ForegroundColor(DesignTokens.Semantic.textPrimary)
            FontSize(16)
        }
        .code {
            FontFamilyVariant(.monospaced)
            FontSize(14)
            BackgroundColor(DesignTokens.Semantic.background.opacity(0.1))
        }
        .codeBlock { configuration in
            configuration.label
                .padding(DesignTokens.Spacing.s)
                .background(DesignTokens.Semantic.background.opacity(0.1))
                .clipShape(RoundedRectangle(cornerRadius: DesignTokens.Radius.small))
                .markdownMargin(top: .zero, bottom: DesignTokens.Spacing.s)
        }
}

#Preview {
    Gemma4ConsoleView()
}
