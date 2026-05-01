import test from "node:test";
import assert from "node:assert/strict";
import { Bm25Index } from "../../rag_model_site/js/bm25_index.js";

test("Bm25Index ranks doc with query term higher", () => {
  const ix = new Bm25Index([
    { id: "1", text: "the quick brown fox jumps" },
    { id: "2", text: "lazy dog sleeps all day" },
  ]);
  const hits = ix.search("fox", 5);
  assert.equal(hits[0].id, "1");
  assert.ok(hits[0].score > 0);
});

test("Bm25Index no match returns empty", () => {
  const ix = new Bm25Index([{ id: "a", text: "hello world" }]);
  const hits = ix.search("zzznomatch", 5);
  assert.equal(hits.length, 0);
});
