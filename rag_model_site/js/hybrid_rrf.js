/**
 * Reciprocal Rank Fusion — same scoring as kb_rag.reciprocal_rank_fusion:
 * score(id) += 1 / (k + rank) for each ranked list (rank is 1-based).
 * @param {string[][]} rankedIds - ordered id lists (best rank first)
 * @param {number} [rrfK=60]
 * @returns {Array<[string, number]>} sorted by fused score descending, then id
 */
export function reciprocalRankFusion(rankedIds, rrfK = 60) {
  const k = Math.max(1, rrfK);
  /** @type {Record<string, number>} */
  const fused = {};
  for (const ids of rankedIds) {
    ids.forEach((id, idx) => {
      const rank = idx + 1;
      fused[id] = (fused[id] ?? 0) + 1 / (k + rank);
    });
  }
  return Object.entries(fused).sort((a, b) => {
    if (b[1] !== a[1]) return b[1] - a[1];
    return a[0].localeCompare(b[0]);
  });
}
