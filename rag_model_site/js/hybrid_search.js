import { reciprocalRankFusion } from "./hybrid_rrf.js";
import { Bm25Index } from "./bm25_index.js";

/**
 * @param {number[]} a
 * @param {number[]} b
 */
export function cosineSimilarity(a, b) {
  if (!a?.length || !b?.length || a.length !== b.length) return -1;
  let dot = 0;
  let na = 0;
  let nb = 0;
  for (let i = 0; i < a.length; i++) {
    dot += a[i] * b[i];
    na += a[i] * a[i];
    nb += b[i] * b[i];
  }
  if (na === 0 || nb === 0) return -1;
  return dot / (Math.sqrt(na) * Math.sqrt(nb));
}

/**
 * @param {object} data - parsed kb_hybrid_demo.json
 * @param {number[]} queryEmbedding
 * @param {string} queryText
 * @param {Bm25Index} [bm25Index] - reuse across queries if corpus unchanged
 */
export function runHybridSearch(data, queryEmbedding, queryText, bm25Index = null) {
  const meta = data.meta || {};
  const minVectorScore = meta.min_vector_score ?? 0.2;
  const vectorCandidateK = meta.vector_candidate_k ?? 24;
  const bm25CandidateK = meta.bm25_candidate_k ?? 24;
  const rrfK = meta.rrf_k ?? 60;
  const topK = meta.default_top_k ?? 4;

  /** @type {import('./bm25_index.js').Bm25Index} */
  let index = bm25Index;
  if (!index) {
    index = new Bm25Index(
      data.chunks.map((c) => ({ id: c.id, text: c.text })),
    );
  }

  const byId = Object.fromEntries(data.chunks.map((c) => [c.id, c]));

  const vectorScored = data.chunks
    .map((c) => ({
      id: c.id,
      sim: cosineSimilarity(queryEmbedding, c.embedding),
    }))
    .filter((x) => x.sim >= minVectorScore)
    .sort((a, b) => b.sim - a.sim)
    .slice(0, Math.max(1, vectorCandidateK));

  const vectorIds = vectorScored.map((x) => x.id);
  const vectorMeta = Object.fromEntries(vectorScored.map((x) => [x.id, x.sim]));

  const bm25Hits = index.search(queryText, bm25CandidateK);
  const bm25Ids = bm25Hits.map((h) => h.id);
  const bm25Meta = Object.fromEntries(bm25Hits.map((h) => [h.id, h.score]));

  if (!vectorIds.length && !bm25Ids.length) {
    return { hits: [], bm25Index: index };
  }

  const fused = reciprocalRankFusion([vectorIds, bm25Ids], rrfK).slice(
    0,
    Math.max(1, topK),
  );

  const vecRankById = Object.fromEntries(vectorIds.map((id, i) => [id, i + 1]));
  const bm25RankById = Object.fromEntries(bm25Ids.map((id, i) => [id, i + 1]));

  const hits = fused.map(([id, fusedScore]) => {
    const chunk = byId[id];
    if (!chunk) return null;
    const vr = vecRankById[id];
    const br = bm25RankById[id];
    let channel = "vector+BM25";
    if (vr !== undefined && br === undefined) channel = "vector";
    else if (vr === undefined && br !== undefined) channel = "BM25";
    return {
      id,
      fusedScore,
      channel,
      vectorRank: vr ?? null,
      bm25Rank: br ?? null,
      vectorSimilarity: vectorMeta[id] ?? null,
      bm25Score: bm25Meta[id] ?? null,
      chunk,
    };
  }).filter(Boolean);

  return { hits, bm25Index: index };
}
