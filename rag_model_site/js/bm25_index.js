/**
 * Okapi BM25 over chunked documents (tokenize: lowercase alphanumeric splits).
 */

const DEFAULT_K1 = 1.5;
const DEFAULT_B = 0.75;

function tokenize(text) {
  return String(text)
    .toLowerCase()
    .match(/[a-z0-9]+/g) ?? [];
}

export class Bm25Index {
  /**
   * @param {Array<{ id: string, text: string }>} docs
   */
  constructor(docs) {
    this.docIds = docs.map((d) => d.id);
    this.docFreq = new Map();
    /** @type {Map<string, Map<string, number>>} docId -> term -> count */
    this.termFreqs = new Map();
    this.docLens = [];

    let totalLen = 0;
    for (const doc of docs) {
      const terms = tokenize(doc.text);
      const len = terms.length;
      this.docLens.push(len);
      totalLen += len;

      const tf = new Map();
      for (const t of terms) {
        tf.set(t, (tf.get(t) ?? 0) + 1);
      }
      this.termFreqs.set(doc.id, tf);

      const seen = new Set(tf.keys());
      for (const t of seen) {
        this.docFreq.set(t, (this.docFreq.get(t) ?? 0) + 1);
      }
    }

    this.avgdl = docs.length ? totalLen / docs.length : 0;
    this.N = docs.length;
  }

  /**
   * @param {string} query
   * @param {number} topK
   * @returns {{ id: string, score: number }[]}
   */
  search(query, topK) {
    const qTerms = tokenize(query);
    if (!qTerms.length || !this.N) return [];

    const k1 = DEFAULT_K1;
    const b = DEFAULT_B;
    const scores = new Map();

    for (const term of qTerms) {
      const df = this.docFreq.get(term) ?? 0;
      if (df === 0) continue;
      const idf = Math.log(1 + (this.N - df + 0.5) / (df + 0.5));

      for (let i = 0; i < this.docIds.length; i++) {
        const id = this.docIds[i];
        const tfMap = this.termFreqs.get(id);
        const f = tfMap?.get(term) ?? 0;
        if (f === 0) continue;
        const dl = this.docLens[i];
        const denom = f + k1 * (1 - b + (b * dl) / (this.avgdl || 1));
        const score = idf * ((f * (k1 + 1)) / denom);
        scores.set(id, (scores.get(id) ?? 0) + score);
      }
    }

    const ranked = [...scores.entries()]
      .map(([id, score]) => ({ id, score }))
      .sort((a, b) => b.score - a.score);

    return ranked.slice(0, Math.max(1, topK));
  }
}
