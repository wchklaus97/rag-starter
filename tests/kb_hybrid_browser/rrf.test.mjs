import test from "node:test";
import assert from "node:assert/strict";
import { reciprocalRankFusion } from "../../rag_model_site/js/hybrid_rrf.js";

test("reciprocalRankFusion matches 1/(k+rank) per list, 1-based rank", () => {
  const k = 60;
  const ranked = reciprocalRankFusion([["a", "b"], ["b", "a"]], k);
  const score = (r1, r2) => 1 / (k + r1) + 1 / (k + r2);
  const expectedA = score(1, 2);
  const expectedB = score(2, 1);
  assert.equal(expectedA, expectedB, "symmetric lists give equal fused scores");
  assert.equal(ranked[0][0], "a");
  assert.equal(ranked[1][0], "b");
  assert.equal(ranked[0][1], expectedA);
  assert.equal(ranked[1][1], expectedB);
});

test("reciprocalRankFusion ties break by id lexicographically", () => {
  const ranked = reciprocalRankFusion([["x", "y"], ["y", "x"]], 60);
  const score = (r1, r2) => 1 / (60 + r1) + 1 / (60 + r2);
  assert.equal(score(1, 2), score(2, 1));
  assert.equal(ranked[0][0], "x");
  assert.equal(ranked[1][0], "y");
  assert.equal(ranked[0][1], ranked[1][1]);
});

test("reciprocalRankFusion single list", () => {
  const ranked = reciprocalRankFusion([["z", "w"]], 10);
  assert.deepEqual(ranked, [
    ["z", 1 / 11],
    ["w", 1 / 12],
  ]);
});

test("reciprocalRankFusion asymmetric lists higher score wins", () => {
  const ranked = reciprocalRankFusion([["x"], ["y", "x"]], 60);
  assert.equal(ranked[0][0], "x");
  assert.equal(ranked[1][0], "y");
});
