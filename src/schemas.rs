use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffSummary {
    pub summary: String,
    pub files_changed: Vec<String>,
    pub risk_level: RiskLevel,
    pub suggested_reviewers: Vec<String>,
    pub reasoning: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Severity {
    Note,
    Warning,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewComment {
    pub file: String,
    pub line: Option<u32>,
    pub comment: String,
    pub severity: Severity,
}
