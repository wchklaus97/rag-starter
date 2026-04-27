//! Debug-mode NDJSON logger (session fe9600). Do not log secrets.

// #region agent log
const LOG_PATH: &str = "/Users/klaus_mac/Projects/04-Experiments/rag-starter/.cursor/debug-fe9600.log";
const SESSION_ID: &str = "fe9600";

pub fn log(
    hypothesis_id: &str,
    location: &str,
    message: &str,
    data: serde_json::Value,
    run_id: &str,
) {
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
