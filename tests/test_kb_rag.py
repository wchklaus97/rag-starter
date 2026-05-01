from __future__ import annotations

import json
from pathlib import Path

from llama_index.core import MockEmbedding

from kb_rag import (
    IndexedChunk,
    SearchHit,
    SourceDocument,
    build_answer_prompt,
    build_chunks,
    chunk_text,
    cosine_similarity,
    insufficient_context_message,
    load_demo_documents,
    reciprocal_rank_fusion,
    retrieve,
    retrieve_hybrid,
)


def test_chunk_text_preserves_overlap() -> None:
    chunks = chunk_text("abcdefghij", chunk_chars=4, overlap=2)
    assert [chunk.text for chunk in chunks] == ["abcd", "cdef", "efgh", "ghij"]


def test_build_chunks_creates_stable_labels() -> None:
    documents = [SourceDocument(path="policy.md", text="a" * 450)]
    chunks = build_chunks(documents, chunk_chars=300, overlap=100)

    assert chunks[0].label == "policy.md#chunk0"
    assert chunks[1].label == "policy.md#chunk1"
    assert chunks[0].file_name == "policy.md"


def test_demo_manifest_matches_files_on_disk() -> None:
    repo = Path(__file__).resolve().parent.parent
    manifest_path = repo / "demo_docs" / "manifest.json"
    raw = json.loads(manifest_path.read_text(encoding="utf-8"))
    demo_dir = repo / "demo_docs"
    loaded = {Path(d.path).name for d in load_demo_documents(demo_dir)}
    for entry in raw.get("documents", []):
        fname = entry.get("file")
        assert fname in loaded, f"manifest lists {fname} but load_demo_documents did not find it"


def test_load_demo_documents_filters_hidden_and_unsupported_files(tmp_path: Path) -> None:
    (tmp_path / "policy.md").write_text("Allowed markdown file", encoding="utf-8")
    (tmp_path / "notes.txt").write_text("Allowed text file", encoding="utf-8")
    (tmp_path / ".secret.md").write_text("Should be ignored", encoding="utf-8")
    (tmp_path / "slides.pdf").write_text("Unsupported", encoding="utf-8")

    documents = load_demo_documents(tmp_path)

    assert [doc.path for doc in documents] == ["notes.txt", "policy.md"]


def test_cosine_similarity_prefers_nearer_vector() -> None:
    query = [1.0, 0.0]
    near = [0.9, 0.1]
    far = [0.0, 1.0]

    assert cosine_similarity(query, near) > cosine_similarity(query, far)


def test_retrieve_returns_sorted_top_k_hits() -> None:
    chunks = [
        IndexedChunk(
            label="a.md#chunk0",
            file_name="a.md",
            source_path="a.md",
            chunk_index=0,
            start_char=0,
            end_char=10,
            text="alpha",
            embedding=[1.0, 0.0],
        ),
        IndexedChunk(
            label="b.md#chunk0",
            file_name="b.md",
            source_path="b.md",
            chunk_index=0,
            start_char=0,
            end_char=10,
            text="beta",
            embedding=[0.8, 0.2],
        ),
        IndexedChunk(
            label="c.md#chunk0",
            file_name="c.md",
            source_path="c.md",
            chunk_index=0,
            start_char=0,
            end_char=10,
            text="gamma",
            embedding=[0.0, 1.0],
        ),
    ]

    hits = retrieve([1.0, 0.0], chunks, top_k=2, min_score=0.10)

    assert [hit.chunk.file_name for hit in hits] == ["a.md", "b.md"]
    assert hits[0].score >= hits[1].score


def test_build_answer_prompt_includes_citation_labels() -> None:
    hit = SearchHit(
        score=0.91,
        chunk=IndexedChunk(
            label="policy.md#chunk2",
            file_name="policy.md",
            source_path="policy.md",
            chunk_index=2,
            start_char=0,
            end_char=20,
            text="Manager approval is required.",
            embedding=[1.0, 0.0],
        ),
    )

    prompt = build_answer_prompt("Do I need approval?", [hit])

    assert "[policy.md#chunk2 | score=0.910]" in prompt
    assert "Cite retrieved evidence inline like [filename#chunkN]." in prompt


def test_empty_retrieval_supports_low_context_message() -> None:
    hits = retrieve([1.0, 0.0], [], top_k=4, min_score=0.20)

    assert hits == []
    assert "don't have enough indexed context" in insufficient_context_message()


def test_reciprocal_rank_fusion_orders_overlap() -> None:
    fused = reciprocal_rank_fusion([["a", "b"], ["b", "c"]], rrf_k=60)
    ids = [i for i, _ in fused]
    assert ids[0] == "b"


def test_retrieve_hybrid_marks_vector_and_bm25_ranks() -> None:
    emb = MockEmbedding(embed_dim=16)
    chunks = [
        IndexedChunk(
            label="doc.md#chunk0",
            file_name="doc.md",
            source_path="doc.md",
            chunk_index=0,
            start_char=0,
            end_char=10,
            text="quantum encryption standards",
            embedding=list(emb.get_text_embedding("quantum encryption standards")),
        ),
        IndexedChunk(
            label="doc.md#chunk1",
            file_name="doc.md",
            source_path="doc.md",
            chunk_index=1,
            start_char=10,
            end_char=20,
            text="lunch menu sandwiches",
            embedding=list(emb.get_text_embedding("lunch menu sandwiches")),
        ),
    ]
    hits = retrieve_hybrid(
        "encryption quantum",
        chunks,
        emb,
        top_k=2,
        min_vector_score=0.0,
        vector_candidate_k=4,
        bm25_candidate_k=4,
    )
    assert hits
    assert hits[0].vector_rank is not None or hits[0].bm25_rank is not None
    assert "fused_rrf=" in build_answer_prompt("q", hits)


def test_build_answer_prompt_hybrid_includes_fused_header() -> None:
    hit = SearchHit(
        score=0.0321,
        chunk=IndexedChunk(
            label="policy.md#chunk2",
            file_name="policy.md",
            source_path="policy.md",
            chunk_index=2,
            start_char=0,
            end_char=20,
            text="Manager approval is required.",
            embedding=[1.0, 0.0],
        ),
        vector_rank=1,
        bm25_rank=2,
        vector_similarity=0.88,
        bm25_score=0.41,
    )
    prompt = build_answer_prompt("Do I need approval?", [hit])
    assert "fused_rrf=0.0321" in prompt
    assert "vec_rank=1" in prompt
    assert "bm25_rank=2" in prompt
