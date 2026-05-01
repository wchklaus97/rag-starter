//! One-shot MCP client over stdio (child process). Used when `RAG_STARTER_MCP_STDIO_JSON` is set.

use rmcp::model::{CallToolRequestParams, CallToolResult, RawContent, ResourceContents};
use rmcp::service::ServiceExt;
use rmcp::transport::TokioChildProcess;

/// JSON array of strings: `["executable", "arg1", ...]`. Must be non-empty; first element is the program.
pub fn stdio_argv_from_env() -> Option<Vec<String>> {
    let raw = std::env::var("RAG_STARTER_MCP_STDIO_JSON").ok()?;
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let v: Vec<String> = serde_json::from_str(trimmed).ok()?;
    if v.is_empty() || v.iter().any(|s| s.is_empty()) {
        return None;
    }
    Some(v)
}

fn format_call_result(result: CallToolResult) -> Result<String, String> {
    let mut parts = Vec::new();
    for block in &result.content {
        match &block.raw {
            RawContent::Text(t) => parts.push(t.text.clone()),
            RawContent::Resource(r) => match &r.resource {
                ResourceContents::TextResourceContents { text, .. } if !text.is_empty() => {
                    parts.push(text.clone());
                }
                _ => parts.push("[embedded resource]".to_string()),
            },
            RawContent::Image(_) => parts.push("[image content omitted]".to_string()),
            RawContent::Audio(_) => parts.push("[audio content omitted]".to_string()),
            RawContent::ResourceLink(r) => parts.push(format!("resource: {}", r.uri)),
        }
    }
    if let Some(ref sc) = result.structured_content {
        parts.push(format!(
            "(structured): {}",
            serde_json::to_string_pretty(sc).unwrap_or_else(|_| sc.to_string())
        ));
    }
    let mut out = parts.join("\n");
    if out.trim().is_empty() {
        out = serde_json::to_string_pretty(&result)
            .unwrap_or_else(|_| format!("{result:?}"));
    }
    if result.is_error == Some(true) {
        return Err(format!("MCP tool reported error: {out}"));
    }
    Ok(out)
}

/// Spawn argv as MCP stdio server, call one tool, then shut the client down.
pub async fn call_stdio_tool_once(
    argv: Vec<String>,
    tool_name: String,
    arguments: Option<serde_json::Map<String, serde_json::Value>>,
) -> Result<String, String> {
    let mut it = argv.into_iter();
    let program = it.next().ok_or_else(|| "MCP argv is empty".to_string())?;

    let mut cmd = tokio::process::Command::new(program);
    for a in it {
        cmd.arg(a);
    }

    let transport = TokioChildProcess::new(cmd).map_err(|e| format!("MCP transport: {e}"))?;

    let client = ()
        .serve(transport)
        .await
        .map_err(|e| format!("MCP handshake: {e}"))?;

    let params = match arguments {
        Some(map) => CallToolRequestParams::new(tool_name).with_arguments(map),
        None => CallToolRequestParams::new(tool_name),
    };

    let tool_result = client
        .call_tool(params)
        .await
        .map_err(|e| format!("MCP call_tool: {e}"))?;

    let formatted = format_call_result(tool_result)?;

    let _ = client.cancel().await;

    Ok(formatted)
}
