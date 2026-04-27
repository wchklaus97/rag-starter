//! Serializable conversation + conversion to `rig` chat history.

use crate::debug_ndjson;
use anyhow::{Context, Result};
use rig::completion::message::AssistantContent;
use rig::completion::Message;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Conversation {
    pub turns: Vec<(Role, String)>,
}

/// Max JSON payload we write in one shot (after pretty-print).
const MAX_CHAT_JSON_BYTES: usize = 50_000_000;

impl Conversation {
    pub fn load(path: &Path) -> Result<Self> {
        let run_id = std::env::var("DEBUG_RUN_ID").unwrap_or_else(|_| "pre-fix".into());
        if !path.exists() {
            // #region agent log
            debug_ndjson::log(
                "H4",
                "chat.rs:load",
                "missing_chat_json",
                serde_json::json!({"path": path.display().to_string()}),
                &run_id,
            );
            // #endregion
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("read {}", path.display()))?;
        match serde_json::from_str::<Self>(&text) {
            Ok(c) => {
                // #region agent log
                debug_ndjson::log(
                    "H4",
                    "chat.rs:load",
                    "parse_ok",
                    serde_json::json!({"turns": c.turns.len()}),
                    &run_id,
                );
                // #endregion
                Ok(c)
            }
            Err(e) => {
                // #region agent log
                debug_ndjson::log(
                    "H4",
                    "chat.rs:load",
                    "parse_err",
                    serde_json::json!({"error": e.to_string()}),
                    &run_id,
                );
                // #endregion
                Err(e).context("parse chat.json")
            }
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).with_context(|| format!("create {}", dir.display()))?;
        }
        let text = serde_json::to_string_pretty(self).context("serialize conversation")?;
        if text.len() > MAX_CHAT_JSON_BYTES {
            anyhow::bail!(
                "refusing to write chat.json: serialized size {} exceeds limit",
                text.len()
            );
        }
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, &text)
            .with_context(|| format!("write {}", tmp.display()))?;
        if let Err(e) = std::fs::rename(&tmp, path) {
            let _ = std::fs::remove_file(&tmp);
            return Err(e).context(format!(
                "rename {} -> {}",
                tmp.display(),
                path.display()
            ));
        }
        Ok(())
    }

    pub fn push_user(&mut self, text: impl Into<String>) {
        self.turns.push((Role::User, text.into()));
    }

    pub fn push_assistant(&mut self, text: impl Into<String>) {
        self.turns.push((Role::Assistant, text.into()));
    }

    /// Prior turns only — pass the new user line as `agent.chat(new_line, self.rig_history())`.
    pub fn rig_history(&self) -> Vec<Message> {
        self.turns
            .iter()
            .map(|(role, text)| match role {
                Role::User => Message::from(text.as_str()),
                Role::Assistant => {
                    Message::Assistant {
                        id: None,
                        content: rig::OneOrMany::one(AssistantContent::text(text.clone())),
                    }
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversation_serde_roundtrip() {
        let mut c = Conversation::default();
        c.push_user("hello");
        c.push_assistant("ahoy");
        let json = serde_json::to_string(&c).expect("serialize");
        let c2: Conversation = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(c2.turns.len(), 2);
        assert_eq!(c2.turns[0].0, Role::User);
        assert_eq!(c2.turns[1].0, Role::Assistant);
    }

    #[test]
    fn rig_history_matches_turns() {
        let mut c = Conversation::default();
        c.push_user("u");
        c.push_assistant("a");
        let h = c.rig_history();
        assert_eq!(h.len(), 2);
    }
}
