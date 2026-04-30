/**
 * Wire-up for kb-hybrid-demo.html — runs fully in the browser (GitHub Pages).
 * Embeddings: same Xenova model as kb_hybrid_demo.json (see meta.model_id).
 */
import { pipeline, env } from "https://esm.sh/@xenova/transformers@2.17.2";
import { runHybridSearch } from "./hybrid_search.js";

env.allowLocalModels = false;
env.allowRemoteModels = true;

const DATA_URL = "./data/kb_hybrid_demo.json";

/** @type {any} */
let corpusJson = null;
/** @type {import('@xenova/transformers').FeatureExtractionPipeline | null} */
let extractor = null;
/** @type {import('./bm25_index.js').Bm25Index | null} */
let bm25Index = null;

function $(id) {
  const el = document.getElementById(id);
  if (!el) throw new Error(`Missing #${id}`);
  return el;
}

async function loadCorpus() {
  const res = await fetch(DATA_URL);
  if (!res.ok) {
    throw new Error(`Could not load ${DATA_URL} (${res.status}). Run: npm run build:kb-demo`);
  }
  corpusJson = await res.json();
  bm25Index = null;
  const meta = corpusJson.meta || {};
  $("meta-model").textContent = meta.model_id ?? "—";
  $("meta-chunks").textContent = String(corpusJson.chunks?.length ?? 0);
  $("meta-updated").textContent = meta.generated_at ?? "—";
}

async function getExtractor() {
  if (extractor) return extractor;
  if (!corpusJson?.meta?.model_id) await loadCorpus();
  const modelId = corpusJson.meta.model_id;
  $("status-line").textContent = "Loading embedding model (downloads on first visit)…";
  extractor = await pipeline("feature-extraction", modelId);
  $("status-line").textContent = "Model ready.";
  return extractor;
}

/**
 * @param {string} text
 * @returns {Promise<number[]>}
 */
async function embedQuery(text) {
  const ext = await getExtractor();
  const output = await ext(text, { pooling: "mean", normalize: true });
  const data = /** @type {{ data: Float32Array }} */ (output);
  return Array.from(data.data);
}

function renderHits(hits) {
  const tbody = $("hits-body");
  tbody.replaceChildren();

  if (!hits.length) {
    const tr = document.createElement("tr");
    tr.innerHTML =
      '<td colspan="6" class="muted">No hits (try loosening min_vector_score in the bundle or different wording).</td>';
    tbody.appendChild(tr);
    return;
  }

  for (const h of hits) {
    const tr = document.createElement("tr");
    const snip =
      h.chunk.text.length > 160 ? `${h.chunk.text.slice(0, 160)}…` : h.chunk.text;
    tr.innerHTML = `
      <td>${escapeHtml(h.chunk.file_name)}</td>
      <td><code>${escapeHtml(h.channel)}</code></td>
      <td class="num">${fmtNum(h.fusedScore, 4)}</td>
      <td class="num">${h.vectorRank ?? "—"}</td>
      <td class="num">${h.bm25Rank ?? "—"}</td>
      <td class="snippet">${escapeHtml(snip)}</td>
    `;
    tbody.appendChild(tr);
  }
}

function escapeHtml(s) {
  return String(s)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function fmtNum(n, d) {
  if (n == null || Number.isNaN(n)) return "—";
  return Number(n).toFixed(d);
}

async function onSearch(event) {
  event?.preventDefault?.();
  const q = ($("query-input").value || "").trim();
  const errEl = $("error-line");
  errEl.textContent = "";

  if (!q) {
    errEl.textContent = "Enter a search query.";
    return;
  }

  try {
    if (!corpusJson) await loadCorpus();
    if (!corpusJson.chunks?.length) {
      errEl.textContent = "Corpus is empty.";
      return;
    }

    $("status-line").textContent = "Embedding query…";
    const qVec = await embedQuery(q);
    $("status-line").textContent = "Retrieving (BM25 + vector + RRF)…";

    const { hits, bm25Index: nextIndex } = runHybridSearch(
      corpusJson,
      qVec,
      q,
      bm25Index,
    );
    bm25Index = nextIndex;

    renderHits(hits);
    $("status-line").textContent = `Done — ${hits.length} fused hit(s).`;
  } catch (e) {
    console.error(e);
    errEl.textContent = e instanceof Error ? e.message : String(e);
    $("status-line").textContent = "";
  }
}

async function boot() {
  try {
    $("status-line").textContent = "Loading corpus JSON…";
    await loadCorpus();
    $("status-line").textContent =
      "Corpus loaded. Enter a query and click Retrieve (model loads on first search).";
  } catch (e) {
    console.error(e);
    $("error-line").textContent = e instanceof Error ? e.message : String(e);
    $("status-line").textContent = "";
  }

  $("search-form").addEventListener("submit", onSearch);
}

boot();
