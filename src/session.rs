//! Build a provider-specific `Agent` and expose a single `chat` entry point.

use crate::agent_tools::{GetCurrentTime, GetWeather, ListDir, ReadFile, RunSafeShell};
use crate::schemas::{DiffSummary, ReviewComment};
use crate::debug_ndjson;
use crate::rag::RagEngine;
use anyhow::{Context, Result};
use rig::agent::Agent;
use rig::client::{CompletionClient, Nothing, ProviderClient};
use rig::completion::{Chat, Message};
use rig::providers::openai::responses_api::ResponsesCompletionModel;
use rig::providers::{deepseek, ollama, openai};
use std::path::PathBuf;

const PREAMBLE: &str = "You are a capable assistant. You speak with light pirate flair (arr, matey) but stay accurate and concise. \
When the user asks for the current time, weather, files, directory listings, or an allowed shell command, you MUST call the matching tool — do not invent facts. \
When retrieved workspace context is provided in the prompt, use it carefully, cite it like [path#chunkN], and admit when the indexed context is insufficient.";

enum BackendAgent {
    Ollama(Agent<ollama::CompletionModel, ()>),
    DeepSeek(Agent<deepseek::CompletionModel, ()>),
    OpenAI(Agent<ResponsesCompletionModel, ()>),
}

pub struct ChatSession {
    backend: BackendAgent,
    rag: Option<RagEngine>,
}

impl ChatSession {
    /// Returns `(session, canonical_workspace)` so `chat.json` and the RAG index share the same root.
    pub async fn build(workspace: PathBuf, source: &str) -> Result<(Self, PathBuf)> {
        let run_id = std::env::var("DEBUG_RUN_ID").unwrap_or_else(|_| "pre-fix".into());
        debug_ndjson::log(
            "H1",
            "session.rs:build",
            "enter",
            serde_json::json!({
                "workspace_in": workspace.display().to_string(),
                "source": source
            }),
            &run_id,
        );

        let root = workspace
            .canonicalize()
            .or_else(|_| {
                std::fs::create_dir_all(&workspace).with_context(|| {
                    format!("create workspace directory {}", workspace.display())
                })?;
                workspace
                    .canonicalize()
                    .with_context(|| format!("canonicalize {}", workspace.display()))
            })
            .context("resolve WORKSPACE_DIR")?;
        debug_ndjson::log(
            "H1",
            "session.rs:build",
            "root_resolved",
            serde_json::json!({"root": root.display().to_string()}),
            &run_id,
        );

        let rag = match RagEngine::build(&root, source).await {
            Ok(rag) => {
                debug_ndjson::log(
                    "H1",
                    "session.rs:build",
                    "rag_ready",
                    serde_json::json!({
                        "enabled": rag.is_some(),
                        "status": rag.as_ref().map(|engine| engine.status_line())
                    }),
                    &run_id,
                );
                rag
            }
            Err(error) => {
                tracing::warn!(error = %error, "RAG initialization failed; continuing without retrieval");
                debug_ndjson::log(
                    "H1",
                    "session.rs:build",
                    "rag_err",
                    serde_json::json!({"error": error.to_string()}),
                    &run_id,
                );
                None
            }
        };

        let backend = match source {
            "ollama" => {
                let client = ollama::Client::new(Nothing).context("ollama client")?;
                let model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "qwen2.5:3b".into());
                let agent = client
                    .agent(model)
                    .preamble(PREAMBLE)
                    .tool(GetCurrentTime)
                    .tool(GetWeather)
                    .tool(ReadFile::new(root.clone()))
                    .tool(ListDir::new(root.clone()))
                    .tool(RunSafeShell::new(root.clone()))
                    .default_max_turns(24)
                    .build();
                debug_ndjson::log(
                    "H1",
                    "session.rs:build",
                    "branch_ollama",
                    serde_json::json!({}),
                    &run_id,
                );
                BackendAgent::Ollama(agent)
            }
            "deepseek" => {
                let client = deepseek::Client::from_env();
                let model =
                    std::env::var("DEEPSEEK_MODEL").unwrap_or_else(|_| "deepseek-chat".into());
                let agent = client
                    .agent(model)
                    .preamble(PREAMBLE)
                    .tool(GetCurrentTime)
                    .tool(GetWeather)
                    .tool(ReadFile::new(root.clone()))
                    .tool(ListDir::new(root.clone()))
                    .tool(RunSafeShell::new(root.clone()))
                    .default_max_turns(24)
                    .build();
                debug_ndjson::log(
                    "H1",
                    "session.rs:build",
                    "branch_deepseek",
                    serde_json::json!({}),
                    &run_id,
                );
                BackendAgent::DeepSeek(agent)
            }
            "openai" => {
                let client = openai::Client::from_env();
                let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".into());
                let agent = client
                    .agent(model)
                    .preamble(PREAMBLE)
                    .tool(GetCurrentTime)
                    .tool(GetWeather)
                    .tool(ReadFile::new(root.clone()))
                    .tool(ListDir::new(root.clone()))
                    .tool(RunSafeShell::new(root.clone()))
                    .default_max_turns(24)
                    .build();
                debug_ndjson::log(
                    "H1",
                    "session.rs:build",
                    "branch_openai",
                    serde_json::json!({}),
                    &run_id,
                );
                BackendAgent::OpenAI(agent)
            }
            other => anyhow::bail!("Unknown MODEL_SOURCE={other}. Use: ollama | deepseek | openai"),
        };

        Ok((Self { backend, rag }, root))
    }

    pub fn rag_status_line(&self) -> Option<String> {
        self.rag.as_ref().map(|rag| rag.status_line())
    }

    pub async fn chat(&self, history: Vec<Message>, user_message: &str) -> Result<String> {
        let run_id = std::env::var("DEBUG_RUN_ID").unwrap_or_else(|_| "pre-fix".into());
        debug_ndjson::log(
            "H3",
            "session.rs:chat",
            "enter",
            serde_json::json!({
                "history_len": history.len(),
                "prompt_len": user_message.len(),
                "rag_enabled": self.rag.is_some()
            }),
            &run_id,
        );

        let prompt = if let Some(rag) = &self.rag {
            match rag.prepare_prompt(user_message).await {
                Ok(prepared) => {
                    let hits = prepared
                        .hits
                        .iter()
                        .map(|hit| format!(
                            "{}#chunk{}:{:.3}",
                            hit.document.path, hit.document.chunk_index, hit.score
                        ))
                        .collect::<Vec<_>>();
                    debug_ndjson::log(
                        "H3",
                        "session.rs:chat",
                        "rag_context",
                        serde_json::json!({
                            "hit_count": prepared.hits.len(),
                            "hits": hits
                        }),
                        &run_id,
                    );
                    prepared.prompt
                }
                Err(error) => {
                    tracing::warn!(error = %error, "RAG query failed; falling back to raw prompt");
                    debug_ndjson::log(
                        "H3",
                        "session.rs:chat",
                        "rag_query_err",
                        serde_json::json!({"error": error.to_string()}),
                        &run_id,
                    );
                    user_message.to_string()
                }
            }
        } else {
            user_message.to_string()
        };

        let res = match &self.backend {
            BackendAgent::Ollama(agent) => agent.chat(prompt.as_str(), history).await,
            BackendAgent::DeepSeek(agent) => agent.chat(prompt.as_str(), history).await,
            BackendAgent::OpenAI(agent) => agent.chat(prompt.as_str(), history).await,
        };

        match &res {
            Ok(output) => {
                debug_ndjson::log(
                    "H3",
                    "session.rs:chat",
                    "rig_ok",
                    serde_json::json!({"out_len": output.len()}),
                    &run_id,
                );
            }
            Err(error) => {
                debug_ndjson::log(
                    "H3",
                    "session.rs:chat",
                    "rig_err",
                    serde_json::json!({"error": error.to_string()}),
                    &run_id,
                );
            }
        }

        res.map_err(anyhow::Error::from)
    }

    /// High-reliability task: Summarize a code diff into a structured schema.
    pub async fn summarize_diff(&self, diff_text: &str) -> Result<DiffSummary> {
        let prompt = format!(
            "Analyze the following code diff and provide a structured summary in JSON format.\n\
            DO NOT include any text before or after the JSON block.\n\n\
            Diff:\n{}\n\n\
            JSON Schema:\n\
            {{ \"summary\": \"string\", \"files_changed\": [\"string\"], \"risk_level\": \"Low|Medium|High|Critical\", \"suggested_reviewers\": [\"string\"], \"reasoning\": \"string\" }}",
            diff_text
        );

        let response = self.chat(vec![], &prompt).await?;
        
        // Basic JSON extraction from markdown if present
        let json_str = if let Some(start) = response.find('{') {
            if let Some(end) = response.rfind('}') {
                &response[start..=end]
            } else {
                &response
            }
        } else {
            &response
        };

        serde_json::from_str(json_str).context("failed to parse DiffSummary from agent output")
    }

    /// High-reliability task: Draft a review comment into a structured schema.
    pub async fn draft_review_comment(&self, context: &str) -> Result<ReviewComment> {
        let prompt = format!(
            "Draft a code review comment for the following context and provide it in JSON format.\n\
            DO NOT include any text before or after the JSON block.\n\n\
            Context:\n{}\n\n\
            JSON Schema:\n\
            {{ \"file\": \"string\", \"line\": number, \"comment\": \"string\", \"severity\": \"Note|Warning|Error\" }}",
            context
        );

        let response = self.chat(vec![], &prompt).await?;
        
        let json_str = if let Some(start) = response.find('{') {
            if let Some(end) = response.rfind('}') {
                &response[start..=end]
            } else {
                &response
            }
        } else {
            &response
        };

        serde_json::from_str(json_str).context("failed to parse ReviewComment from agent output")
    }
}
