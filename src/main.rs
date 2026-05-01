//! CLI agent: persistent chat history, optional demo prompts, rig tools.

mod agent_tools;
mod chat;
mod debug_ndjson;
mod mcp_tool;
mod rag;
mod schemas;
mod session;

use anyhow::Result;
use chat::Conversation;
use session::ChatSession;
use std::io::Write;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let run_id = std::env::var("DEBUG_RUN_ID").unwrap_or_else(|_| "pre-fix".into());
    // #region agent log
    debug_ndjson::log(
        "H1",
        "main.rs:main",
        "startup",
        serde_json::json!({
            "workspace_env_set": std::env::var("WORKSPACE_DIR").is_ok(),
            "model_source": std::env::var("MODEL_SOURCE").unwrap_or_else(|_| "ollama".into()),
            "demo": std::env::var("RAG_STARTER_DEMO").unwrap_or_default(),
            "debug_ndjson": std::env::var("DEBUG_NDJSON_PATH").is_ok(),
            "mcp_stdio_tool": crate::agent_tools::mcp_stdio_tool_enabled(),
        }),
        &run_id,
    );
    // #endregion
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let workspace: PathBuf = std::env::var("WORKSPACE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().expect("cwd"));

    let source = std::env::var("MODEL_SOURCE").unwrap_or_else(|_| "ollama".into());
    let (session, workspace) = match ChatSession::build(workspace, source.as_str()).await {
        Ok(x) => {
            // #region agent log
            debug_ndjson::log(
                "H1",
                "main.rs:main",
                "build_ok",
                serde_json::json!({
                    "workspace": x.1.display().to_string(),
                    "source": source
                }),
                &run_id,
            );
            // #endregion
            x
        }
        Err(e) => {
            // #region agent log
            debug_ndjson::log(
                "H1",
                "main.rs:main",
                "build_err",
                serde_json::json!({"error": e.to_string()}),
                &run_id,
            );
            // #endregion
            return Err(e);
        }
    };

    let chat_path = workspace.join("chat.json");
    let mut conversation = Conversation::load(&chat_path)?;

    if std::env::var("RAG_STARTER_DEMO").as_deref() == Ok("1") {
        tracing::info!("running three demo prompts (Day 3 style)");
        for q in [
            "What is 2 + 2? One short sentence.",
            "Name one color. One word.",
            "Say hello in pirate speak. One short line.",
        ] {
            let history = conversation.rig_history();
            match session.chat(history, q).await {
                Ok(reply) => {
                    println!("\n[q] {q}\n[a] {reply}\n");
                    conversation.push_user(q);
                    conversation.push_assistant(reply);
                }
                Err(e) => {
                    // #region agent log
                    debug_ndjson::log(
                        "H5",
                        "main.rs:main",
                        "demo_chat_err",
                        serde_json::json!({"error": e.to_string(), "q_len": q.len()}),
                        &run_id,
                    );
                    // #endregion
                    return Err(e);
                }
            }
        }
        conversation.save(&chat_path)?;
    }

    println!("Chat session (pirate-flavored assistant with tools). Type 'quit' or 'exit' to leave.");
    println!("Commands: /summarize <path>, /review <path>");
    println!("Workspace: {}", workspace.display());
    println!("MODEL_SOURCE={source}\n");
    if let Some(status) = session.rag_status_line() {
        println!("{status}\n");
    }

    let stdin = std::io::stdin();
    loop {
        print!("you> ");
        std::io::stdout().flush().ok();
        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.eq_ignore_ascii_case("quit") || line.eq_ignore_ascii_case("exit") {
            break;
        }

        // --- Slash Commands (High Reliability Tasks) ---
        if line.starts_with("/summarize ") {
            let path_str = line.trim_start_matches("/summarize ").trim();
            let path = workspace.join(path_str);
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    println!("[task] Summarizing {}...", path_str);
                    match session.summarize_diff(&content).await {
                        Ok(summary) => {
                            let json = serde_json::to_string_pretty(&summary).unwrap_or_default();
                            println!("bot> (Structured Summary)\n{}\n", json);
                        }
                        Err(e) => eprintln!("error: failed to summarize: {e:#}"),
                    }
                }
                Err(e) => eprintln!("error: could not read file {}: {e}", path.display()),
            }
            continue;
        }

        if line.starts_with("/review ") {
            let path_str = line.trim_start_matches("/review ").trim();
            let path = workspace.join(path_str);
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    println!("[task] Drafting review for {}...", path_str);
                    match session.draft_review_comment(&content).await {
                        Ok(comment) => {
                            let json = serde_json::to_string_pretty(&comment).unwrap_or_default();
                            println!("bot> (Structured Review)\n{}\n", json);
                        }
                        Err(e) => eprintln!("error: failed to review: {e:#}"),
                    }
                }
                Err(e) => eprintln!("error: could not read file {}: {e}", path.display()),
            }
            continue;
        }
        // -----------------------------------------------

        let history = conversation.rig_history();
        match session.chat(history, line).await {
            Ok(reply) => {
                // #region agent log
                debug_ndjson::log(
                    "H2",
                    "main.rs:main",
                    "chat_ok",
                    serde_json::json!({
                        "reply_len": reply.len(),
                        "user_len": line.len()
                    }),
                    &run_id,
                );
                // #endregion
                println!("bot> {reply}\n");
                conversation.push_user(line);
                conversation.push_assistant(reply);
                conversation.save(&chat_path)?;
            }
            Err(e) => {
                // #region agent log
                debug_ndjson::log(
                    "H2",
                    "main.rs:main",
                    "chat_err",
                    serde_json::json!({"error": e.to_string()}),
                    &run_id,
                );
                // #endregion
                tracing::error!(error = %e, "chat failed");
                eprintln!("error: {e:#}");
            }
        }
    }

    conversation.save(&chat_path)?;
    Ok(())
}
