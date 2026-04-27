//! Lightweight workspace RAG: chunk local text files, embed, persist, and retrieve.

use anyhow::{Context, Result};
use rig::client::embeddings::EmbeddingsClient;
use rig::client::{Nothing, ProviderClient};
use rig::embeddings::{Embedding, EmbeddingModel, EmbeddingsBuilder};
use rig::providers::{ollama, openai};
use rig::{Embed, OneOrMany};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

const INDEX_FILE_NAME: &str = "rag_index.json";
const MAX_FILE_CHARS: usize = 200_000;
const DEFAULT_CHUNK_CHARS: usize = 900;
const DEFAULT_CHUNK_OVERLAP: usize = 200;
const DEFAULT_TOP_K: usize = 4;
const DEFAULT_MIN_SCORE: f64 = 0.20;
const DEFAULT_OPENAI_MODEL: &str = openai::TEXT_EMBEDDING_3_SMALL;
const DEFAULT_OLLAMA_MODEL: &str = "nomic-embed-text";
const ALLOWED_EXTENSIONS: &[&str] = &["md", "txt", "rs", "toml", "json", "yaml", "yml"];
const IGNORED_DIRS: &[&str] = &[".git", ".cursor", "target", "node_modules"];
const IGNORED_FILES: &[&str] = &["chat.json", INDEX_FILE_NAME];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RagConfig {
    pub enabled: bool,
    pub reindex: bool,
    pub embedding_source: String,
    pub embedding_model: String,
    pub chunk_chars: usize,
    pub chunk_overlap: usize,
    pub top_k: usize,
    pub min_score: f64,
}

impl RagConfig {
    pub fn from_env(chat_source: &str) -> Self {
        let enabled = env_bool("RAG_ENABLE", true);
        let reindex = env_bool("RAG_REINDEX", false);
        let chunk_chars = env_usize("RAG_CHUNK_CHARS", DEFAULT_CHUNK_CHARS).max(200);
        let chunk_overlap = env_usize("RAG_CHUNK_OVERLAP", DEFAULT_CHUNK_OVERLAP)
            .min(chunk_chars.saturating_sub(1));
        let top_k = env_usize("RAG_TOP_K", DEFAULT_TOP_K).max(1);
        let min_score = env_f64("RAG_MIN_SCORE", DEFAULT_MIN_SCORE);

        let default_source = match chat_source {
            "ollama" => "ollama",
            "openai" => "openai",
            _ => "openai",
        };
        let embedding_source =
            std::env::var("RAG_EMBEDDING_SOURCE").unwrap_or_else(|_| default_source.to_string());
        let embedding_model = match embedding_source.as_str() {
            "ollama" => {
                std::env::var("OLLAMA_EMBEDDING_MODEL").unwrap_or_else(|_| DEFAULT_OLLAMA_MODEL.into())
            }
            _ => std::env::var("OPENAI_EMBEDDING_MODEL")
                .unwrap_or_else(|_| DEFAULT_OPENAI_MODEL.into()),
        };

        Self {
            enabled,
            reindex,
            embedding_source,
            embedding_model,
            chunk_chars,
            chunk_overlap,
            top_k,
            min_score,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceSnapshot {
    pub path: String,
    pub size: u64,
    pub modified_ms: u128,
}

#[derive(Embed, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChunkDocument {
    pub id: String,
    pub path: String,
    pub chunk_index: usize,
    pub start_char: usize,
    pub end_char: usize,
    #[embed]
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedChunk {
    pub document: ChunkDocument,
    pub embeddings: OneOrMany<Embedding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedIndex {
    pub config: RagConfig,
    pub sources: Vec<SourceSnapshot>,
    pub chunks: Vec<PersistedChunk>,
}

#[derive(Debug, Clone)]
pub struct SearchHit {
    pub score: f64,
    pub document: ChunkDocument,
    pub matched_text: String,
}

#[derive(Debug, Clone)]
pub struct PreparedPrompt {
    pub prompt: String,
    pub hits: Vec<SearchHit>,
}

pub struct RagEngine {
    config: RagConfig,
    index_path: PathBuf,
    index: PersistedIndex,
    backend: EmbeddingBackend,
}

impl RagEngine {
    pub async fn build(root: &Path, chat_source: &str) -> Result<Option<Self>> {
        let config = RagConfig::from_env(chat_source);
        if !config.enabled {
            return Ok(None);
        }

        let backend = EmbeddingBackend::from_config(&config)?;
        let root = root.to_path_buf();
        let index_path = root.join(INDEX_FILE_NAME);
        let sources = snapshot_workspace(&root)?;

        let index = if !config.reindex {
            match load_index_if_fresh(&index_path, &config, &sources)? {
                Some(index) => index,
                None => build_index(&backend, &root, &config, sources.clone(), &index_path).await?,
            }
        } else {
            build_index(&backend, &root, &config, sources.clone(), &index_path).await?
        };

        Ok(Some(Self {
            config,
            index_path,
            index,
            backend,
        }))
    }

    pub fn status_line(&self) -> String {
        format!(
            "RAG: {} chunks from {} files via {}:{} -> {}",
            self.index.chunks.len(),
            self.index.sources.len(),
            self.config.embedding_source,
            self.config.embedding_model,
            self.index_path.display()
        )
    }

    pub async fn prepare_prompt(&self, user_message: &str) -> Result<PreparedPrompt> {
        let hits = self.search(user_message).await?;
        Ok(PreparedPrompt {
            prompt: format_prompt(user_message, &hits),
            hits,
        })
    }

    pub async fn search(&self, query: &str) -> Result<Vec<SearchHit>> {
        let query_embedding = self
            .backend
            .embed_query(query)
            .await
            .context("embed query for RAG")?;

        let mut hits = self
            .index
            .chunks
            .iter()
            .filter_map(|chunk| {
                best_match(&query_embedding, &chunk.embeddings).map(|(score, matched_text)| SearchHit {
                    score,
                    document: chunk.document.clone(),
                    matched_text,
                })
            })
            .filter(|hit| hit.score >= self.config.min_score)
            .collect::<Vec<_>>();

        hits.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(Ordering::Equal)
                .then_with(|| a.document.path.cmp(&b.document.path))
                .then_with(|| a.document.chunk_index.cmp(&b.document.chunk_index))
        });
        hits.truncate(self.config.top_k);
        Ok(hits)
    }
}

enum EmbeddingBackend {
    OpenAI(openai::EmbeddingModel),
    Ollama(ollama::EmbeddingModel),
}

impl EmbeddingBackend {
    fn from_config(config: &RagConfig) -> Result<Self> {
        match config.embedding_source.as_str() {
            "ollama" => {
                // `from_env()` panics without OLLAMA_API_BASE_URL; align with `session` via `Client::new(Nothing)` + optional URL.
                let client = match std::env::var("OLLAMA_API_BASE_URL") {
                    Ok(url) => ollama::Client::builder()
                        .api_key(Nothing)
                        .base_url(&url)
                        .build()
                        .context("ollama RAG client (OLLAMA_API_BASE_URL)")?,
                    Err(_) => ollama::Client::new(Nothing).context("ollama RAG client")?,
                };
                Ok(Self::Ollama(client.embedding_model(config.embedding_model.clone())))
            }
            "openai" => {
                let client = openai::Client::from_env();
                Ok(Self::OpenAI(client.embedding_model(config.embedding_model.clone())))
            }
            other => anyhow::bail!(
                "unsupported RAG_EMBEDDING_SOURCE={other}. Use openai or ollama"
            ),
        }
    }

    async fn embed_query(&self, query: &str) -> Result<Embedding> {
        match self {
            Self::OpenAI(model) => embed_text(model, query).await,
            Self::Ollama(model) => embed_text(model, query).await,
        }
    }

    async fn embed_documents(&self, docs: Vec<ChunkDocument>) -> Result<Vec<(ChunkDocument, OneOrMany<Embedding>)>> {
        match self {
            Self::OpenAI(model) => embed_documents(model, docs).await,
            Self::Ollama(model) => embed_documents(model, docs).await,
        }
    }
}

async fn embed_text<M>(model: &M, query: &str) -> Result<Embedding>
where
    M: EmbeddingModel,
{
    model
        .embed_text(query)
        .await
        .map_err(anyhow::Error::from)
}

async fn embed_documents<M>(
    model: &M,
    docs: Vec<ChunkDocument>,
) -> Result<Vec<(ChunkDocument, OneOrMany<Embedding>)>>
where
    M: EmbeddingModel + Clone,
{
    EmbeddingsBuilder::new(model.clone())
        .documents(docs)
        .context("prepare RAG chunks for embedding")?
        .build()
        .await
        .map_err(anyhow::Error::from)
}

fn load_index_if_fresh(
    index_path: &Path,
    config: &RagConfig,
    sources: &[SourceSnapshot],
) -> Result<Option<PersistedIndex>> {
    if !index_path.exists() {
        return Ok(None);
    }
    let text = std::fs::read_to_string(index_path)
        .with_context(|| format!("read {}", index_path.display()))?;
    let index: PersistedIndex =
        serde_json::from_str(&text).with_context(|| format!("parse {}", index_path.display()))?;
    if &index.config == config && index.sources == sources {
        Ok(Some(index))
    } else {
        Ok(None)
    }
}

async fn build_index(
    backend: &EmbeddingBackend,
    root: &Path,
    config: &RagConfig,
    sources: Vec<SourceSnapshot>,
    index_path: &Path,
) -> Result<PersistedIndex> {
    let documents = load_chunk_documents(root, config)?;
    let embedded = backend
        .embed_documents(documents)
        .await
        .context("build RAG embeddings")?;

    let chunks = embedded
        .into_iter()
        .map(|(document, embeddings)| PersistedChunk { document, embeddings })
        .collect::<Vec<_>>();

    let index = PersistedIndex {
        config: config.clone(),
        sources,
        chunks,
    };
    save_index(index_path, &index)?;
    Ok(index)
}

fn save_index(index_path: &Path, index: &PersistedIndex) -> Result<()> {
    let text = serde_json::to_string_pretty(index).context("serialize RAG index")?;
    let tmp = index_path.with_extension("json.tmp");
    std::fs::write(&tmp, text).with_context(|| format!("write {}", tmp.display()))?;
    std::fs::rename(&tmp, index_path)
        .with_context(|| format!("rename {} -> {}", tmp.display(), index_path.display()))?;
    Ok(())
}

fn load_chunk_documents(root: &Path, config: &RagConfig) -> Result<Vec<ChunkDocument>> {
    let mut docs = Vec::new();
    for snapshot in snapshot_workspace(root)? {
        let full_path = root.join(&snapshot.path);
        let text = std::fs::read_to_string(&full_path)
            .with_context(|| format!("read {}", full_path.display()))?;
        let truncated = truncate_for_embedding(&text);
        for (chunk_index, chunk) in chunk_text(&truncated, config.chunk_chars, config.chunk_overlap)
            .into_iter()
            .enumerate()
        {
            docs.push(ChunkDocument {
                id: format!("{}#chunk{}", snapshot.path, chunk_index),
                path: snapshot.path.clone(),
                chunk_index,
                start_char: chunk.start_char,
                end_char: chunk.end_char,
                text: chunk.text,
            });
        }
    }
    Ok(docs)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChunkSlice {
    start_char: usize,
    end_char: usize,
    text: String,
}

fn chunk_text(text: &str, chunk_chars: usize, overlap: usize) -> Vec<ChunkSlice> {
    if text.trim().is_empty() {
        return Vec::new();
    }

    let chars = text.chars().collect::<Vec<_>>();
    let len = chars.len();
    let step = chunk_chars.saturating_sub(overlap).max(1);
    let mut start = 0usize;
    let mut chunks = Vec::new();

    while start < len {
        let end = (start + chunk_chars).min(len);
        let chunk_text = chars[start..end].iter().collect::<String>().trim().to_string();
        if !chunk_text.is_empty() {
            chunks.push(ChunkSlice {
                start_char: start,
                end_char: end,
                text: chunk_text,
            });
        }
        if end == len {
            break;
        }
        start = start.saturating_add(step);
    }

    chunks
}

fn truncate_for_embedding(text: &str) -> String {
    let mut chars = text.chars().collect::<Vec<_>>();
    if chars.len() <= MAX_FILE_CHARS {
        return text.to_string();
    }
    chars.truncate(MAX_FILE_CHARS);
    chars.into_iter().collect()
}

fn snapshot_workspace(root: &Path) -> Result<Vec<SourceSnapshot>> {
    let mut acc = Vec::new();
    collect_workspace_files(root, root, &mut acc)?;
    acc.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(acc)
}

fn collect_workspace_files(root: &Path, dir: &Path, acc: &mut Vec<SourceSnapshot>) -> Result<()> {
    for entry in std::fs::read_dir(dir).with_context(|| format!("read_dir {}", dir.display()))? {
        let entry = entry.with_context(|| format!("read entry in {}", dir.display()))?;
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        if entry
            .file_type()
            .with_context(|| format!("file_type {}", path.display()))?
            .is_dir()
        {
            if should_skip_dir(&file_name) {
                continue;
            }
            collect_workspace_files(root, &path, acc)?;
            continue;
        }

        if should_skip_file(&file_name, &path) {
            continue;
        }

        let relative = path
            .strip_prefix(root)
            .with_context(|| format!("strip prefix {}", path.display()))?
            .to_string_lossy()
            .into_owned();
        let metadata = std::fs::metadata(&path).with_context(|| format!("metadata {}", path.display()))?;
        let modified_ms = metadata
            .modified()
            .ok()
            .and_then(|ts| ts.duration_since(UNIX_EPOCH).ok())
            .map(|dur| dur.as_millis())
            .unwrap_or(0);

        acc.push(SourceSnapshot {
            path: relative,
            size: metadata.len(),
            modified_ms,
        });
    }

    Ok(())
}

fn should_skip_dir(name: &str) -> bool {
    name.starts_with('.') || IGNORED_DIRS.contains(&name)
}

fn should_skip_file(name: &str, path: &Path) -> bool {
    if name.starts_with('.') || IGNORED_FILES.contains(&name) {
        return true;
    }
    let Some(ext) = path.extension().and_then(|ext| ext.to_str()) else {
        return false;
    };
    !ALLOWED_EXTENSIONS.contains(&ext)
}

fn best_match(query_embedding: &Embedding, embeddings: &OneOrMany<Embedding>) -> Option<(f64, String)> {
    embeddings
        .iter()
        .map(|embedding| {
            (
                cosine_similarity(&query_embedding.vec, &embedding.vec),
                embedding.document.clone(),
            )
        })
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal))
}

fn cosine_similarity(left: &[f64], right: &[f64]) -> f64 {
    if left.is_empty() || right.is_empty() || left.len() != right.len() {
        return -1.0;
    }

    let mut dot = 0.0;
    let mut left_norm = 0.0;
    let mut right_norm = 0.0;

    for (l, r) in left.iter().zip(right.iter()) {
        dot += l * r;
        left_norm += l * l;
        right_norm += r * r;
    }

    if left_norm == 0.0 || right_norm == 0.0 {
        return -1.0;
    }

    dot / (left_norm.sqrt() * right_norm.sqrt())
}

fn format_prompt(user_message: &str, hits: &[SearchHit]) -> String {
    let mut prompt = String::new();
    prompt.push_str("User question:\n");
    prompt.push_str(user_message);
    prompt.push_str("\n\nWorkspace retrieval:\n");

    if hits.is_empty() {
        prompt.push_str("No strongly relevant indexed workspace context was found.\n");
    } else {
        for hit in hits {
            prompt.push_str(&format!(
                "[{}#chunk{} | score={:.3}]\n{}\n\n",
                hit.document.path, hit.document.chunk_index, hit.score, hit.matched_text
            ));
        }
    }

    prompt.push_str(
        "Answer rules:\n\
- If the retrieved workspace context is relevant, use it first.\n\
- Cite workspace evidence inline like [path#chunkN].\n\
- If the workspace context is insufficient for the question, say so clearly.\n\
- If the question does not need workspace context, answer normally and use tools when appropriate.\n",
    );
    prompt
}

fn env_bool(name: &str, default: bool) -> bool {
    match std::env::var(name) {
        Ok(value) => matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "on"),
        Err(_) => default,
    }
}

fn env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(default)
}

fn env_f64(name: &str, default: f64) -> f64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse::<f64>().ok())
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_text_uses_overlap() {
        let chunks = chunk_text("abcdefghij", 4, 2);
        let texts = chunks.into_iter().map(|chunk| chunk.text).collect::<Vec<_>>();
        assert_eq!(texts, vec!["abcd", "cdef", "efgh", "ghij"]);
    }

    #[test]
    fn cosine_similarity_ranks_expected_document() {
        let query = Embedding {
            document: "query".into(),
            vec: vec![1.0, 0.0],
        };
        let near = Embedding {
            document: "near".into(),
            vec: vec![0.9, 0.1],
        };
        let far = Embedding {
            document: "far".into(),
            vec: vec![0.0, 1.0],
        };

        let near_score = cosine_similarity(&query.vec, &near.vec);
        let far_score = cosine_similarity(&query.vec, &far.vec);
        assert!(near_score > far_score);
    }

    #[test]
    fn prompt_includes_citation_format() {
        let prompt = format_prompt(
            "What does this repo do?",
            &[SearchHit {
                score: 0.91,
                document: ChunkDocument {
                    id: "README.md#chunk0".into(),
                    path: "README.md".into(),
                    chunk_index: 0,
                    start_char: 0,
                    end_char: 10,
                    text: "rag starter".into(),
                },
                matched_text: "rag starter".into(),
            }],
        );

        assert!(prompt.contains("[README.md#chunk0 | score=0.910]"));
        assert!(prompt.contains("Cite workspace evidence inline like [path#chunkN]."));
    }
}
