#!/usr/bin/env node
/**
 * Build rag_model_site/data/kb_hybrid_demo.json from demo_docs/
 * Chunking mirrors kb_rag.chunk_text / build_chunks; embeddings use the same
 * Xenova model as the browser demo (query + chunk vectors must match).
 *
 * Schema:
 *   meta: { model_id, rrf_k, chunk_chars, chunk_overlap, vector_candidate_k,
 *           bm25_candidate_k, min_vector_score, default_top_k, generated_at }
 *   chunks: [{ id, file_name, source_path, chunk_index, start_char, end_char, text, embedding }]
 */

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { pipeline } from "@xenova/transformers";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.join(__dirname, "..");

const DEFAULT_CHUNK_CHARS = 900;
const DEFAULT_CHUNK_OVERLAP = 200;
const DEFAULT_RRF_K = 60;
const DEFAULT_VECTOR_CANDIDATE_K = 24;
const DEFAULT_BM25_CANDIDATE_K = 24;
const DEFAULT_MIN_SCORE = 0.2;
const DEFAULT_TOP_K = 4;
const MODEL_ID = "Xenova/all-MiniLM-L6-v2";

const ALLOWED = new Set([".md", ".txt"]);

function isSupportedFile(filePath) {
  const base = path.basename(filePath);
  if (base.startsWith(".")) return false;
  const ext = path.extname(filePath).toLowerCase();
  return ALLOWED.has(ext);
}

function normalizeText(text) {
  return text.replace(/\r\n/g, "\n").replace(/\r/g, "\n").trim();
}

function chunkText(text, chunkChars = DEFAULT_CHUNK_CHARS, overlap = DEFAULT_CHUNK_OVERLAP) {
  const normalized = normalizeText(text);
  if (!normalized) return [];

  chunkChars = Math.max(1, chunkChars);
  overlap = Math.min(overlap, Math.max(0, chunkChars - 1));
  const chars = [...normalized];
  const step = Math.max(1, chunkChars - overlap);
  /** @type {{ start_char: number, end_char: number, text: string }[]} */
  const chunks = [];
  let start = 0;

  while (start < chars.length) {
    const end = Math.min(chars.length, start + chunkChars);
    const chunkValue = chars.slice(start, end).join("").trim();
    if (chunkValue) {
      chunks.push({ start_char: start, end_char: end, text: chunkValue });
    }
    if (end === chars.length) break;
    start += step;
  }

  return chunks;
}

function loadDemoDocuments(rootDir) {
  const dir = path.resolve(rootDir);
  if (!fs.existsSync(dir)) {
    throw new Error(`Demo document path does not exist: ${dir}`);
  }
  /** @type {{ path: string, text: string }[]} */
  const documents = [];
  for (const name of fs.readdirSync(dir).sort()) {
    const candidate = path.join(dir, name);
    if (!fs.statSync(candidate).isFile() || !isSupportedFile(candidate)) continue;
    const raw = fs.readFileSync(candidate, "utf8");
    const text = normalizeText(raw);
    if (text) documents.push({ path: name, text });
  }
  return documents;
}

function buildChunks(documents, chunkChars, overlap) {
  /** @type {object[]} */
  const out = [];
  for (const document of documents) {
    const slices = chunkText(document.text, chunkChars, overlap);
    slices.forEach((slice, chunkIndex) => {
      const label = `${document.path}#chunk${chunkIndex}`;
      out.push({
        id: label,
        file_name: document.path,
        source_path: document.path,
        chunk_index: chunkIndex,
        start_char: slice.start_char,
        end_char: slice.end_char,
        text: slice.text,
      });
    });
  }
  return out;
}

/**
 * @param {Awaited<ReturnType<typeof pipeline>>} extractor
 * @param {string[]} texts
 */
async function embedBatch(extractor, texts) {
  /** @type {number[][]} */
  const embeddings = [];
  const batchSize = 4;
  for (let i = 0; i < texts.length; i += batchSize) {
    const batch = texts.slice(i, i + batchSize);
    for (const text of batch) {
      const output = await extractor(text, {
        pooling: "mean",
        normalize: true,
      });
      const data = /** @type {{ data: Float32Array }} */ (output);
      embeddings.push(Array.from(data.data));
    }
  }
  return embeddings;
}

async function main() {
  const demoDocsDir = path.join(ROOT, "demo_docs");
  const outPath = path.join(ROOT, "rag_model_site", "data", "kb_hybrid_demo.json");

  const documents = loadDemoDocuments(demoDocsDir);
  const chunkStructs = buildChunks(
    documents,
    DEFAULT_CHUNK_CHARS,
    DEFAULT_CHUNK_OVERLAP,
  );

  if (!chunkStructs.length) {
    console.warn("No chunks produced from demo_docs; writing empty bundle.");
    fs.mkdirSync(path.dirname(outPath), { recursive: true });
    fs.writeFileSync(
      outPath,
      JSON.stringify({
        meta: {
          model_id: MODEL_ID,
          rrf_k: DEFAULT_RRF_K,
          chunk_chars: DEFAULT_CHUNK_CHARS,
          chunk_overlap: DEFAULT_CHUNK_OVERLAP,
          vector_candidate_k: DEFAULT_VECTOR_CANDIDATE_K,
          bm25_candidate_k: DEFAULT_BM25_CANDIDATE_K,
          min_vector_score: DEFAULT_MIN_SCORE,
          default_top_k: DEFAULT_TOP_K,
          generated_at: new Date().toISOString(),
          demo_docs_root: "demo_docs",
        },
        chunks: [],
      }),
    );
    return;
  }

  console.error("Loading embedding model (first run may download weights)…");
  const extractor = await pipeline("feature-extraction", MODEL_ID);
  const texts = chunkStructs.map((c) => c.text);
  const embeddings = await embedBatch(extractor, texts);

  const chunks = chunkStructs.map((c, i) => ({
    ...c,
    embedding: embeddings[i],
  }));

  const payload = {
    meta: {
      model_id: MODEL_ID,
      rrf_k: DEFAULT_RRF_K,
      chunk_chars: DEFAULT_CHUNK_CHARS,
      chunk_overlap: DEFAULT_CHUNK_OVERLAP,
      vector_candidate_k: DEFAULT_VECTOR_CANDIDATE_K,
      bm25_candidate_k: DEFAULT_BM25_CANDIDATE_K,
      min_vector_score: DEFAULT_MIN_SCORE,
      default_top_k: DEFAULT_TOP_K,
      generated_at: new Date().toISOString(),
      demo_docs_root: "demo_docs",
    },
    chunks,
  };

  fs.mkdirSync(path.dirname(outPath), { recursive: true });
  fs.writeFileSync(outPath, JSON.stringify(payload));
  console.error(
    `Wrote ${chunks.length} chunks to ${path.relative(ROOT, outPath)}`,
  );
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
