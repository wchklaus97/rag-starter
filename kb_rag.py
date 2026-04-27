from __future__ import annotations

from dataclasses import dataclass, replace
import math
from pathlib import Path
from typing import Any, Iterable, Sequence


ALLOWED_EXTENSIONS = {".md", ".txt"}
DEFAULT_CHUNK_CHARS = 900
DEFAULT_CHUNK_OVERLAP = 200
DEFAULT_TOP_K = 4
DEFAULT_MIN_SCORE = 0.20
DEFAULT_EMBEDDING_MODEL = "text-embedding-3-small"


@dataclass(frozen=True)
class SourceDocument:
    path: str
    text: str


@dataclass(frozen=True)
class ChunkSlice:
    start_char: int
    end_char: int
    text: str


@dataclass(frozen=True)
class IndexedChunk:
    label: str
    file_name: str
    source_path: str
    chunk_index: int
    start_char: int
    end_char: int
    text: str
    embedding: list[float] | None = None


@dataclass(frozen=True)
class SearchHit:
    score: float
    chunk: IndexedChunk


def is_supported_file(path: str | Path) -> bool:
    candidate = Path(path)
    return candidate.suffix.lower() in ALLOWED_EXTENSIONS and not candidate.name.startswith(".")


def load_uploaded_documents(files: Sequence[Any]) -> list[SourceDocument]:
    documents: list[SourceDocument] = []
    for file in files:
        name = Path(getattr(file, "name", "uploaded.txt")).name
        if not is_supported_file(name):
            continue

        raw = _read_uploaded_file(file)
        text = _normalize_text(raw.decode("utf-8", errors="ignore"))
        if text.strip():
            documents.append(SourceDocument(path=name, text=text))
    return documents


def load_demo_documents(path: str | Path) -> list[SourceDocument]:
    root = Path(path)
    if not root.exists():
        raise FileNotFoundError(f"Demo document path does not exist: {root}")

    documents: list[SourceDocument] = []
    for candidate in sorted(root.iterdir()):
        if not candidate.is_file() or not is_supported_file(candidate):
            continue
        text = _normalize_text(candidate.read_text(encoding="utf-8", errors="ignore"))
        if text.strip():
            documents.append(SourceDocument(path=candidate.name, text=text))
    return documents


def chunk_text(
    text: str,
    chunk_chars: int = DEFAULT_CHUNK_CHARS,
    overlap: int = DEFAULT_CHUNK_OVERLAP,
) -> list[ChunkSlice]:
    if not text.strip():
        return []

    chunk_chars = max(1, chunk_chars)
    overlap = min(overlap, max(0, chunk_chars - 1))
    chars = list(text)
    step = max(1, chunk_chars - overlap)
    chunks: list[ChunkSlice] = []
    start = 0

    while start < len(chars):
        end = min(len(chars), start + chunk_chars)
        chunk_value = "".join(chars[start:end]).strip()
        if chunk_value:
            chunks.append(ChunkSlice(start_char=start, end_char=end, text=chunk_value))
        if end == len(chars):
            break
        start += step

    return chunks


def build_chunks(
    documents: Sequence[SourceDocument],
    chunk_chars: int = DEFAULT_CHUNK_CHARS,
    overlap: int = DEFAULT_CHUNK_OVERLAP,
) -> list[IndexedChunk]:
    indexed_chunks: list[IndexedChunk] = []
    for document in documents:
        for chunk_index, chunk in enumerate(chunk_text(document.text, chunk_chars, overlap)):
            indexed_chunks.append(
                IndexedChunk(
                    label=f"{document.path}#chunk{chunk_index}",
                    file_name=document.path,
                    source_path=document.path,
                    chunk_index=chunk_index,
                    start_char=chunk.start_char,
                    end_char=chunk.end_char,
                    text=chunk.text,
                )
            )
    return indexed_chunks


def embed_texts(
    client: Any,
    texts: Sequence[str],
    model: str = DEFAULT_EMBEDDING_MODEL,
) -> list[list[float]]:
    if not texts:
        return []
    response = client.embeddings.create(model=model, input=list(texts))
    return [list(item.embedding) for item in response.data]


def embed_query(client: Any, question: str, model: str = DEFAULT_EMBEDDING_MODEL) -> list[float]:
    response = client.embeddings.create(model=model, input=question)
    return list(response.data[0].embedding)


def index_documents_with_openai(
    client: Any,
    documents: Sequence[SourceDocument],
    embedding_model: str = DEFAULT_EMBEDDING_MODEL,
    chunk_chars: int = DEFAULT_CHUNK_CHARS,
    overlap: int = DEFAULT_CHUNK_OVERLAP,
) -> list[IndexedChunk]:
    chunks = build_chunks(documents, chunk_chars=chunk_chars, overlap=overlap)
    if not chunks:
        return []

    embeddings = embed_texts(client, [chunk.text for chunk in chunks], model=embedding_model)
    return [
        replace(chunk, embedding=embedding)
        for chunk, embedding in zip(chunks, embeddings, strict=True)
    ]


def cosine_similarity(left: Sequence[float], right: Sequence[float]) -> float:
    if not left or not right or len(left) != len(right):
        return -1.0

    dot = sum(l * r for l, r in zip(left, right))
    left_norm = math.sqrt(sum(value * value for value in left))
    right_norm = math.sqrt(sum(value * value for value in right))

    if left_norm == 0.0 or right_norm == 0.0:
        return -1.0

    return dot / (left_norm * right_norm)


def retrieve(
    question_embedding: Sequence[float],
    chunks: Sequence[IndexedChunk],
    top_k: int = DEFAULT_TOP_K,
    min_score: float = DEFAULT_MIN_SCORE,
) -> list[SearchHit]:
    hits: list[SearchHit] = []
    for chunk in chunks:
        if chunk.embedding is None:
            continue
        score = cosine_similarity(question_embedding, chunk.embedding)
        if score >= min_score:
            hits.append(SearchHit(score=score, chunk=chunk))

    hits.sort(key=lambda hit: (-hit.score, hit.chunk.source_path, hit.chunk.chunk_index))
    return hits[: max(1, top_k)]


def build_answer_prompt(question: str, hits: Sequence[SearchHit]) -> str:
    lines = ["User question:", question, "", "Retrieved context:"]
    if hits:
        for hit in hits:
            lines.append(f"[{hit.chunk.label} | score={hit.score:.3f}]")
            lines.append(hit.chunk.text)
            lines.append("")
    else:
        lines.append("No strongly relevant indexed context was found.")
        lines.append("")

    lines.extend(
        [
            "Answer rules:",
            "- Answer from the retrieved context first.",
            "- Cite retrieved evidence inline like [filename#chunkN].",
            "- If the retrieved context is insufficient, say so clearly instead of guessing.",
            "- Do not cite anything that is not in the retrieved context list.",
        ]
    )
    return "\n".join(lines).strip()


def answer_with_openai(client: Any, prompt: str, model: str) -> str:
    response = client.responses.create(model=model, input=prompt)
    output_text = getattr(response, "output_text", "")
    if output_text:
        return output_text.strip()
    raise ValueError("OpenAI response did not include output_text")


def answer_with_claude(client: Any, prompt: str, model: str) -> str:
    message = client.messages.create(
        model=model,
        max_tokens=700,
        messages=[{"role": "user", "content": prompt}],
    )
    text_blocks = [block.text for block in message.content if getattr(block, "type", "") == "text"]
    if text_blocks:
        return "\n".join(text_blocks).strip()
    raise ValueError("Claude response did not include text content")


def format_source_excerpt(text: str, max_chars: int = 280) -> str:
    normalized = " ".join(text.split())
    if len(normalized) <= max_chars:
        return normalized
    return normalized[: max_chars - 3].rstrip() + "..."


def insufficient_context_message() -> str:
    return "I don't have enough indexed context in these files to answer confidently."


def _normalize_text(text: str) -> str:
    return text.replace("\r\n", "\n").replace("\r", "\n").strip()


def _read_uploaded_file(file: Any) -> bytes:
    if hasattr(file, "getvalue"):
        return file.getvalue()
    if hasattr(file, "read"):
        raw = file.read()
        if isinstance(raw, bytes):
            return raw
        if isinstance(raw, str):
            return raw.encode("utf-8")
    raise TypeError("Uploaded file must provide getvalue() or read()")


__all__ = [
    "ALLOWED_EXTENSIONS",
    "DEFAULT_CHUNK_CHARS",
    "DEFAULT_CHUNK_OVERLAP",
    "DEFAULT_MIN_SCORE",
    "DEFAULT_TOP_K",
    "DEFAULT_EMBEDDING_MODEL",
    "ChunkSlice",
    "IndexedChunk",
    "SearchHit",
    "SourceDocument",
    "answer_with_claude",
    "answer_with_openai",
    "build_answer_prompt",
    "build_chunks",
    "chunk_text",
    "cosine_similarity",
    "embed_query",
    "embed_texts",
    "format_source_excerpt",
    "index_documents_with_openai",
    "insufficient_context_message",
    "is_supported_file",
    "load_demo_documents",
    "load_uploaded_documents",
    "retrieve",
]
