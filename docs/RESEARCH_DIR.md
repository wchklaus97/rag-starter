# iOS Agent Expansion: Track 1 Research Findings

This document summarizes the technical findings for upgrading the `swift-gemma4-sample` to a full agentic system with local RAG.

## 1. On-Device RAG Architecture
To implement local retrieval without a cloud backend, we will integrate the **VecturaKit** ecosystem.

### Key Components:
- **Core Library:** [VecturaKit](https://github.com/rryam/VecturaKit) - Handles the vector database storage and similarity search.
- **MLX Bridge:** [VecturaMLXKit](https://github.com/rryam/VecturaMLXKit) - Provides the `MLXEmbedder` class which uses the Apple GPU to generate embeddings from text chunks.

### Integration Strategy:
1. **Dependency:** Add `VecturaKit` and `VecturaMLXKit` to `Package.swift`.
2. **Embedding Model:** We will use a quantized BERT or MiniLM model compatible with MLX.
3. **Workflow:** 
   - User adds a folder of `.md` or `.pdf` files.
   - App background task chunks the files and generates embeddings via `MLXEmbedder`.
   - Before each Gemma 4 prompt, the top-K relevant chunks are retrieved from the local `VecturaDatabase` and injected into the system prompt.

## 2. Agentic Tool Calling
To give the Gemma 4 model "Actions" (like checking device status or sending emails), we have two paths:

### Path A: Apple App Intents (System-Native)
- **Framework:** Apple App Intents.
- **Usage:** Define `AppIntent` structs for actions.
- **Benefit:** Allows Siri and Apple Intelligence to trigger your app's functions.
- **Challenge:** Requires complex orchestration to map local LLM output tags (like `<tool_call>`) to these intents.

### Path B: In-App Agent Frameworks (GitHub)
- **Primary Candidates:** 
  - [SwiftAI (mi12labs/SwiftAI)](https://github.com/mi12labs/SwiftAI) - Highly integrated for both local and cloud models.
  - [AgentRunKit (MacPaw/AgentRunKit)](https://github.com/MacPaw/AgentRunKit) - Specifically designed for the Agent "Reasoning Loop".
- **Benefit:** Simplifies the "Call LLM -> Execute Tool -> Feed Back Results" cycle within the SwiftUI app.

## 3. Next Step Recommendations
- **Experimental SPI:** Create a prototype using `VecturaMLXKit` to embed a simple string and retrieve it.
- **Tool Protocol:** Define a `Gemma4Tool` protocol in the Swift package that wraps `AppIntents` for local model consumption.

---

## Track 2: Rust AI Agent (CLI Core)
*Objective: Optimize the core CLI agent for speed, safety, and reliability.*

### 1. Model Context Protocol (MCP) Integration
- **Official Servers:** The [modelcontextprotocol/servers](https://github.com/modelcontextprotocol/servers) repository is the definitive source.
- **Top Picks:**
    - **Filesystem Server:** For safe, structured file analysis and project indexing.
    - **Google Maps / Brave Search:** For granting the CLI agent real-world situational awareness.
- **Rust Client Implementation:** Use the `mcp-sdk-rs` or similar bindings to allow the CLI agent to "mount" these servers as tool sources.

### 2. Prompt Compression & Performance
- **Technique:** "Context Pruning" or "Summarization Forgetting".
- **Tooling:** Research the `llm-chain` crate or native Rust implementations of **RRF (Reciprocal Rank Fusion)** to prune the token window during long conversations.
- **TUI:** Use **Ratatui** for a "multi-pane" CLI experience where logs, thinking, and chat are separated visually.

---

## Track 3: Python KB Assistant (RAG Pipeline)
*Objective: Implement production-grade Hybrid Search and verifiable citations.*

### 1. Hybrid Search (BM25 + Dense Vector)
- **Concept:** Combine semantic similarity (Vector) with exact keyword matching (BM25).
- **Tooling:** Use **LlamaIndex** with a vector store that supports `enable_sparse=True` (e.g., Qdrant or Milvus).
- **Ranking:** Implement **Reciprocal Rank Fusion (RRF)** to combine the results from both retrieval methods into a single coherent context.

### 2. Streamlit Deployment & UI
- **Citations:** Implement a UI pattern in Streamlit where document source links (e.g., `[Source: document.md]`) are clickable and open a side-pane with the original text.
- **Performance:** Use `st.cache_resource` for the vector index to prevent re-loading the 100MB+ index on every script rerun.

---

## Track 4: Static Field Guide (Site & Metadata)
*Objective: Automate metadata collection and add local search capability.*

### 1. OpenRouter Metadata Automation
- **Script:** A Python script in `scripts/` using `requests` to fetch `https://openrouter.ai/api/v1/models`.
- **Transformation:** Map the JSON response to our `data/*.json` schema to automatically update model pricing and capability cards.

### 2. Local Search with Pagefind
- **Integration:** Add `npx pagefind --site "rag_model_site" --bundle-dir "pagefind"` to the `pages.yml` GitHub Action.
- **Frontend:** Embed the `PagefindUI` into the static site's navigation bar to provide instant, offline search.

---

## Track 5: Ongoing RAG landscape (manual, docs-only)
*Objective: Periodically capture RAG techniques, tooling alternatives, and news with citable sources—without coupling to the static field guide or OpenRouter automation.*

- **Runbook:** [RAG_RADAR_RUNBOOK.md](RAG_RADAR_RUNBOOK.md) — Cursor **Composer / multi-agent** plus web (or browser MCP) search, evidence rules, and git workflow.
- **Snapshot scaffold:** [RAG_RADAR_SNAPSHOT_TEMPLATE.md](RAG_RADAR_SNAPSHOT_TEMPLATE.md) — copy to a dated file or maintain `RAG_RADAR_LATEST.md` per team preference.
- **Separation from Track 4:** Track 4 uses **APIs** (OpenRouter) and **Pagefind** for `rag_model_site/`. Track 5 is **human-reviewed markdown in `docs/`** for broader landscape scanning.

---
*Date: 2026-04-30 | Status: Multi-Repo Research Complete.*
