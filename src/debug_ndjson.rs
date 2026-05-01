//! Optional NDJSON logger for debug sessions. Do not log secrets.
//!
//! When `DEBUG_NDJSON_PATH` is unset or empty, [`log`](log) is a no-op.

// #region agent log
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

fn log_path() -> Option<std::path::PathBuf> {
    let raw = std::env::var("DEBUG_NDJSON_PATH").ok()?;
    let t = raw.trim();
    if t.is_empty() {
        None
    } else {
        Some(std::path::PathBuf::from(t))
    }
}

fn session_id() -> String {
    std::env::var("DEBUG_SESSION_ID")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "rag-starter".to_string())
}

pub fn log(
    hypothesis_id: &str,
    location: &str,
    message: &str,
    mut data: serde_json::Value,
    run_id: &str,
) {
    let Some(path) = log_path() else {
        return;
    };
    sanitize_secrets(&mut data);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let payload = serde_json::json!({
        "sessionId": session_id(),
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
        .open(&path)
    {
        let _ = std::io::Write::write_all(&mut f, format!("{payload}\n").as_bytes());
    }
}
// #endregion
