//! Debug-mode NDJSON logger (session fe9600). Do not log secrets.

// #region agent log
const LOG_PATH: &str = "/Users/klaus_mac/Projects/04-Experiments/rag-starter/.cursor/debug-fe9600.log";
const SESSION_ID: &str = "fe9600";

use once_cell::sync::Lazy;
use regex::Regex;

static SECRET_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"sk-[a-zA-Z0-9]{48}").unwrap(), // OpenAI-like
        Regex::new(r"sk-proj-[a-zA-Z0-9_-]{30,}").unwrap(), // OpenAI project
        Regex::new(r"xkeyshf-[a-zA-Z0-9]{40,}").unwrap(), // DeepSeek/HuggingFace-like
    ]
});

fn sanitize_secrets(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::String(s) => {
            for re in SECRET_PATTERNS.iter() {
                if re.is_match(s) {
                    *s = re.replace_all(s, "[REDACTED_SECRET]").to_string();
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                sanitize_secrets(v);
            }
        }
        serde_json::Value::Object(obj) => {
            for v in obj.values_mut() {
                sanitize_secrets(v);
            }
        }
        _ => {}
    }
}

pub fn log(
    hypothesis_id: &str,
    location: &str,
    message: &str,
    mut data: serde_json::Value,
    run_id: &str,
) {
    sanitize_secrets(&mut data);
    if let Some(parent) = std::path::Path::new(LOG_PATH).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let payload = serde_json::json!({
        "sessionId": SESSION_ID,
        "hypothesisId": hypothesis_id,
        "location": location,
        "message": message,
        "data": data,
        "timestamp": ts,
        "runId": run_id
    });
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)
    {
        let _ = std::io::Write::write_all(&mut f, format!("{payload}\n").as_bytes());
    }
}
// #endregion
