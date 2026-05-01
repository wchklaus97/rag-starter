//! Agent tools (rig `Tool` implementations).

use chrono::Local;
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use thiserror::Error;

/// Truncation limit for `read_file` tool output (characters, not bytes).
const READ_FILE_MAX_CHARS: usize = 100_000;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct ToolErr(pub String);

impl ToolErr {
    fn msg(s: &str) -> Self {
        Self(s.to_string())
    }
}

// --- get_current_time ---

#[derive(Deserialize)]
pub struct EmptyArgs {}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct GetCurrentTime;

impl Tool for GetCurrentTime {
    const NAME: &'static str = "get_current_time";
    type Error = ToolErr;
    type Args = EmptyArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Return the current local date and time in RFC3339 format. Use this when the user asks what time it is."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {},
                "required": [],
                "additionalProperties": false
            }),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(Local::now().to_rfc3339())
    }
}

// --- get_weather ---

#[derive(Deserialize)]
pub struct WeatherArgs {
    /// City name, e.g. "London" or "New York"
    city: String,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct GetWeather;

impl Tool for GetWeather {
    const NAME: &'static str = "get_weather";
    type Error = ToolErr;
    type Args = WeatherArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Look up current weather for a city using Open-Meteo (no API key). Pass the city name in English."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "city": { "type": "string", "description": "City name" }
                },
                "required": ["city"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let client = reqwest::Client::new();
        let geo: serde_json::Value = client
            .get("https://geocoding-api.open-meteo.com/v1/search")
            .query(&[("name", args.city.as_str()), ("count", "1")])
            .send()
            .await
            .map_err(|e| ToolErr(format!("geocoding request: {e}")))?
            .error_for_status()
            .map_err(|e| ToolErr(format!("geocoding HTTP: {e}")))?
            .json()
            .await
            .map_err(|e| ToolErr(format!("geocoding JSON: {e}")))?;

        let results = geo
            .get("results")
            .filter(|v| !v.is_null())
            .and_then(|r| r.as_array())
            .ok_or_else(|| {
                ToolErr(format!(
                    "geocoding returned no 'results' array for '{}'",
                    args.city
                ))
            })?;
        let Some(first) = results.first() else {
            return Err(ToolErr(format!("no results for city '{}'", args.city)));
        };
        let lat = first
            .get("latitude")
            .and_then(serde_json::Value::as_f64)
            .ok_or_else(|| ToolErr::msg("geocoding: missing latitude"))?;
        let lon = first
            .get("longitude")
            .and_then(serde_json::Value::as_f64)
            .ok_or_else(|| ToolErr::msg("geocoding: missing longitude"))?;
        let label: String = first
            .get("name")
            .and_then(|v| v.as_str())
            .map(str::to_owned)
            .unwrap_or_else(|| args.city.clone());

        let q: Vec<(&str, String)> = vec![
            ("latitude", lat.to_string()),
            ("longitude", lon.to_string()),
            (
                "current",
                "temperature_2m,weather_code".to_string(),
            ),
        ];
        let wx: serde_json::Value = client
            .get("https://api.open-meteo.com/v1/forecast")
            .query(&q)
            .send()
            .await
            .map_err(|e| ToolErr(format!("forecast request: {e}")))?
            .error_for_status()
            .map_err(|e| ToolErr(format!("forecast HTTP: {e}")))?
            .json()
            .await
            .map_err(|e| ToolErr(format!("forecast JSON: {e}")))?;

        let temp = wx
            .pointer("/current/temperature_2m")
            .and_then(serde_json::Value::as_f64)
            .ok_or_else(|| ToolErr::msg("forecast: missing temperature"))?;
        let code = wx
            .pointer("/current/weather_code")
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(-1);

        Ok(format!(
            "{label}: {temp:.1}°C (WMO weather code {code}) [Open-Meteo]"
        ))
    }
}

// --- read_file ---

#[derive(Deserialize)]
pub struct ReadFileArgs {
    /// Path relative to the workspace directory
    path: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ReadFile {
    root: PathBuf,
}

impl ReadFile {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn resolve(&self, rel: &str) -> Result<PathBuf, ToolErr> {
        let rel = rel.trim_start_matches('/');
        let candidate = self.root.join(rel);
        let root = self
            .root
            .canonicalize()
            .map_err(|e| ToolErr(format!("workspace root: {e}")))?;
        let abs = candidate.canonicalize().map_err(|e| {
            ToolErr(format!(
                "path {}: {e}. Stay under workspace {}",
                candidate.display(),
                root.display()
            ))
        })?;
        abs.strip_prefix(&root).map_err(|_| {
            ToolErr(format!(
                "path escapes workspace: {} is not under {}",
                abs.display(),
                root.display()
            ))
        })?;
        Ok(abs)
    }
}

impl Tool for ReadFile {
    const NAME: &'static str = "read_file";
    type Error = ToolErr;
    type Args = ReadFileArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Read a UTF-8 text file under the workspace. Pass a relative path (no leading slash required)."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "Relative path from workspace root" }
                },
                "required": ["path"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let path = self.resolve(&args.path)?;
        if !path.is_file() {
            return Err(ToolErr(format!("not a file: {}", path.display())));
        }
        let mut s = std::fs::read_to_string(&path)
            .map_err(|e| ToolErr(format!("read {}: {e}", path.display())))?;
        if s.len() > READ_FILE_MAX_CHARS {
            s.truncate(READ_FILE_MAX_CHARS);
            s.push_str("\n… [truncated at 100000 chars]");
        }
        Ok(s)
    }
}

// --- list_dir ---

#[derive(Deserialize)]
pub struct ListDirArgs {
    /// Directory relative to workspace; use "." for workspace root
    path: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListDir {
    root: PathBuf,
}

impl ListDir {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn resolve(&self, rel: &str) -> Result<PathBuf, ToolErr> {
        ReadFile::new(self.root.clone()).resolve(rel)
    }
}

impl Tool for ListDir {
    const NAME: &'static str = "list_directory";
    type Error = ToolErr;
    type Args = ListDirArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "List file and directory names in a folder under the workspace. Use path \".\" for the workspace root."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "Relative directory path" }
                },
                "required": ["path"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let path = self.resolve(&args.path)?;
        if !path.is_dir() {
            return Err(ToolErr(format!("not a directory: {}", path.display())));
        }
        let mut names: Vec<String> = std::fs::read_dir(&path)
            .map_err(|e| ToolErr(format!("read_dir: {e}")))?
            .filter_map(std::result::Result::ok)
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        if names.is_empty() {
            Ok("(empty directory)".to_string())
        } else {
            Ok(names.join("\n"))
        }
    }
}

// --- run_safe_shell ---

#[derive(Deserialize)]
pub struct ShellArgs {
    /// Full command line; first token must be an allowlisted program
    command: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RunSafeShell {
    root: PathBuf,
}

impl RunSafeShell {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

const ALLOWED: &[&str] = &["ls", "pwd", "date", "echo", "whoami", "uname"];

impl Tool for RunSafeShell {
    const NAME: &'static str = "run_safe_shell";
    type Error = ToolErr;
    type Args = ShellArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: format!(
                "Run a very limited shell command. Allowed first words only: {}. Runs with working directory = workspace. No pipes, no cd, no rm.",
                ALLOWED.join(", ")
            ),
            parameters: json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string", "description": "e.g. 'ls -la' or 'date'" }
                },
                "required": ["command"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let root = self
            .root
            .canonicalize()
            .map_err(|e| ToolErr(format!("workspace: {e}")))?;

        let cmdline = args.command.trim();
        if cmdline.is_empty() {
            return Err(ToolErr::msg("empty command"));
        }
        if cmdline.contains('|') || cmdline.contains('>') || cmdline.contains('<') {
            return Err(ToolErr::msg("redirection and pipes are not allowed"));
        }

        let mut parts = cmdline.split_whitespace();
        let Some(bin) = parts.next() else {
            return Err(ToolErr::msg("empty command"));
        };
        if !ALLOWED.contains(&bin) {
            return Err(ToolErr(format!(
                "command '{bin}' not in allowlist {:?}",
                ALLOWED
            )));
        }

        let mut c = tokio::process::Command::new(bin);
        for a in parts {
            c.arg(a);
        }
        c.current_dir(&root);
        let out = c
            .output()
            .await
            .map_err(|e| ToolErr(format!("spawn: {e}")))?;
        let status = out.status;
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        Ok(format!(
            "exit={}\nstdout:\n{stdout}\nstderr:\n{stderr}",
            status
        ))
    }
}

// --- MCP stdio (optional; see `RAG_STARTER_MCP_STDIO_JSON`) ---

/// True when `RAG_STARTER_MCP_STDIO_JSON` is set to a valid non-empty JSON string array.
pub fn mcp_stdio_tool_enabled() -> bool {
    crate::mcp_tool::stdio_argv_from_env().is_some()
}

#[derive(Deserialize)]
pub struct CallMcpStdioArgs {
    /// MCP server tool name
    tool_name: String,
    /// Optional JSON object of arguments for that tool
    #[serde(default)]
    arguments: Option<serde_json::Value>,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct CallMcpStdioTool;

impl Tool for CallMcpStdioTool {
    const NAME: &'static str = "call_mcp_stdio_tool";
    type Error = ToolErr;
    type Args = CallMcpStdioArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Call a tool on an MCP server running over stdio. The server command is configured by the operator via RAG_STARTER_MCP_STDIO_JSON (JSON array: program + args). Pass the MCP tool name and an optional JSON object of arguments.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "tool_name": { "type": "string", "description": "Name of the tool on the MCP server" },
                    "arguments": { "type": "object", "description": "Optional tool arguments (plain JSON object)", "additionalProperties": true }
                },
                "required": ["tool_name"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let Some(argv) = crate::mcp_tool::stdio_argv_from_env() else {
            return Err(ToolErr::msg(
                "MCP stdio is not configured: set RAG_STARTER_MCP_STDIO_JSON to a JSON array of strings.",
            ));
        };

        let arguments = match args.arguments {
            None => None,
            Some(serde_json::Value::Object(map)) => Some(map),
            Some(other) => {
                return Err(ToolErr(format!(
                    "arguments must be a JSON object or omitted; got {}",
                    serde_json::to_string(&other).unwrap_or_else(|_| "opaque value".into())
                )));
            }
        };

        crate::mcp_tool::call_stdio_tool_once(argv, args.tool_name, arguments)
            .await
            .map_err(ToolErr)
    }
}
