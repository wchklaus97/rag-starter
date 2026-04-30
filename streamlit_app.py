from __future__ import annotations

from datetime import datetime
import json
import os
from pathlib import Path
from typing import Any

from anthropic import Anthropic
from dotenv import load_dotenv
from llama_index.embeddings.openai import OpenAIEmbedding
from openai import OpenAI
import streamlit as st

from kb_rag import (
    DEFAULT_CHUNK_CHARS,
    DEFAULT_CHUNK_OVERLAP,
    DEFAULT_EMBEDDING_MODEL,
    DEFAULT_MIN_SCORE,
    DEFAULT_TOP_K,
    answer_with_claude,
    answer_with_openai,
    build_answer_prompt,
    format_source_excerpt,
    index_documents_with_openai,
    insufficient_context_message,
    load_demo_documents,
    load_uploaded_documents,
    retrieve_hybrid,
)


load_dotenv()

APP_DIR = Path(__file__).resolve().parent
SAMPLE_DOCS_DIR = APP_DIR / "demo_docs"
MANIFEST_PATH = SAMPLE_DOCS_DIR / "manifest.json"
MODEL_GUIDE_PATH = APP_DIR / "rag_model_site" / "data" / "models.json"
PUBLIC_EXAMPLES_PATH = APP_DIR / "rag_model_site" / "data" / "public_examples.json"


def main() -> None:
    st.set_page_config(page_title="Internal KB Assistant Demo", page_icon="📚", layout="wide")
    _ensure_state()

    st.title("Internal KB Assistant Demo")
    st.caption(
        "Upload 3-5 internal docs, ask a question, and get an answer with citations. "
        "Retrieval is hybrid: dense vectors (OpenAI embeddings) + BM25, merged with RRF. "
        "Source panel shows whether each chunk ranked in the vector index, BM25, or both."
    )
    _demo_manifest = _load_demo_manifest()
    _model_guide = _load_model_guide()
    _public_examples = _load_public_examples()

    with st.sidebar:
        st.header("Setup")
        answer_provider = st.selectbox(
            "Answer provider",
            options=["openai", "claude"],
            index=0 if os.getenv("ANSWER_PROVIDER", "openai") == "openai" else 1,
        )
        openai_key = st.text_input(
            "OpenAI API key",
            value=os.getenv("OPENAI_API_KEY", ""),
            type="password",
            help="Required for embeddings, even when Claude answers.",
        )
        anthropic_key = st.text_input(
            "Anthropic API key",
            value=os.getenv("ANTHROPIC_API_KEY", ""),
            type="password",
            help="Only required when the answer provider is Claude.",
        )
        embedding_model = st.text_input(
            "Embedding model",
            value=os.getenv("OPENAI_EMBEDDING_MODEL", DEFAULT_EMBEDDING_MODEL),
        )
        answer_model = st.text_input(
            "Answer model",
            value=_default_answer_model(answer_provider),
        )
        source_mode = st.radio("Documents", options=["Uploaded files", "Sample docs"], index=0)
        uploaded_files = st.file_uploader(
            "Upload knowledge files",
            type=["md", "txt"],
            accept_multiple_files=True,
            disabled=source_mode != "Uploaded files",
        )
        chunk_chars = st.slider("Chunk size", min_value=400, max_value=1500, value=DEFAULT_CHUNK_CHARS, step=100)
        overlap = st.slider("Chunk overlap", min_value=50, max_value=400, value=DEFAULT_CHUNK_OVERLAP, step=25)
        top_k = st.slider("Top K chunks", min_value=1, max_value=8, value=DEFAULT_TOP_K, step=1)
        min_score = st.slider(
            "Minimum similarity",
            min_value=0.0,
            max_value=1.0,
            value=float(DEFAULT_MIN_SCORE),
            step=0.05,
        )

        if st.button("Build index", type="primary", use_container_width=True):
            build_index(
                source_mode=source_mode,
                uploaded_files=uploaded_files or [],
                openai_key=openai_key,
                embedding_model=embedding_model,
                chunk_chars=chunk_chars,
                overlap=overlap,
            )

    kb_tab, model_tab = st.tabs(["Knowledge base Q&A", "Embedding model guide"])
    with kb_tab:
        _render_index_status()
        _render_sample_doc_help(_demo_manifest)
        _render_qa(
            answer_provider=answer_provider,
            openai_key=openai_key,
            anthropic_key=anthropic_key,
            embedding_model=embedding_model,
            answer_model=answer_model,
            top_k=top_k,
            min_score=min_score,
        )
    with model_tab:
        _render_model_guide(_model_guide, _public_examples)


def build_index(
    *,
    source_mode: str,
    uploaded_files: list[object],
    openai_key: str,
    embedding_model: str,
    chunk_chars: int,
    overlap: int,
) -> None:
    if not openai_key:
        st.error("Add an OpenAI API key first. The app needs it to build embeddings.")
        return

    try:
        documents = (
            load_demo_documents(SAMPLE_DOCS_DIR)
            if source_mode == "Sample docs"
            else load_uploaded_documents(uploaded_files)
        )
    except Exception as error:  # pragma: no cover - UI error path
        st.error(f"Could not load documents: {error}")
        return

    if not documents:
        st.error("No supported `.md` or `.txt` files were found to index.")
        return

    try:
        client = OpenAI(api_key=openai_key)
        indexed_chunks = index_documents_with_openai(
            client,
            documents,
            embedding_model=embedding_model,
            chunk_chars=chunk_chars,
            overlap=overlap,
        )
    except Exception as error:  # pragma: no cover - UI error path
        st.error(f"Index build failed: {error}")
        return

    st.session_state.indexed_chunks = indexed_chunks
    st.session_state.document_count = len(documents)
    st.session_state.built_at = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    st.session_state.last_answer = ""
    st.session_state.last_hits = []
    st.success(f"Indexed {len(documents)} files into {len(indexed_chunks)} chunks.")


def _load_demo_manifest() -> dict[str, Any] | None:
    if not MANIFEST_PATH.is_file():
        return None
    try:
        return json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None


def _load_model_guide() -> dict[str, Any] | None:
    if not MODEL_GUIDE_PATH.is_file():
        return None
    try:
        return json.loads(MODEL_GUIDE_PATH.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None


def _load_public_examples() -> dict[str, Any] | None:
    if not PUBLIC_EXAMPLES_PATH.is_file():
        return None
    try:
        return json.loads(PUBLIC_EXAMPLES_PATH.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None


def _localized(value: Any, lang: str) -> str:
    if isinstance(value, dict):
        return str(value.get(lang) or value.get("zh") or value.get("en") or "")
    return str(value or "")


def _render_sample_doc_help(manifest: dict[str, Any] | None) -> None:
    if not manifest:
        return
    prov = manifest.get("provenance") or {}
    categories = manifest.get("categories") or {}
    with st.expander("Bundled sample docs — where they come from, categories, example questions", expanded=False):
        st.info(prov.get("summary", ""))
        st.caption("More detail: open `demo_docs/README.md` in the repository (provenance and public-domain sources).")
        if prov.get("if_you_want_public_sources"):
            st.caption(prov["if_you_want_public_sources"])

        st.subheader("By category (after you build index with **Sample docs**)")
        for doc in manifest.get("documents", []):
            file_name = doc.get("file", "")
            cat_id = doc.get("category", "")
            cat_label = categories.get(cat_id, cat_id)
            title = doc.get("title", file_name)
            st.markdown(f"**{title}** (`{file_name}`) — *{cat_label}*")
            questions = doc.get("example_questions") or []
            if not questions:
                continue
            cols = st.columns(min(2, max(1, len(questions))))
            for i, q in enumerate(questions):
                if cols[i % len(cols)].button(
                    q,
                    key=f"ex_{file_name}_{i}",
                    use_container_width=True,
                ):
                    st.session_state.kb_question = q
                    st.rerun()
        st.caption("Click a question to copy it into the “Ask your knowledge base” field below, then press **Ask**.")


def _render_model_guide(model_guide: dict[str, Any] | None, public_examples: dict[str, Any] | None) -> None:
    st.subheader("Embedding model guide")
    st.caption(
        "This is the same model data used by the GitHub Pages field guide. "
        "Use it to choose the embedding model before building a KB index."
    )

    if not model_guide:
        st.warning("Could not load `rag_model_site/data/models.json`.")
        return

    lang = st.radio(
        "Guide language",
        options=["en", "zh", "zh_hans"],
        format_func=lambda code: {"en": "English", "zh": "繁體中文", "zh_hans": "简体中文"}[code],
        horizontal=True,
    )

    updated = model_guide.get("updated", "unknown")
    st.info(
        "Embeddings are priced mainly by input tokens. The returned vector is not billed "
        f"like chat output tokens. Data snapshot: `{updated}`."
    )

    recommendations = model_guide.get("recommendations", {}).get(lang) or model_guide.get("recommendations", {}).get("en", [])
    if recommendations:
        st.markdown("**Quick choices**")
        for row in recommendations:
            st.markdown(f"- **{row.get('if', '')}** → `{row.get('then', '')}`")

    models = model_guide.get("models", [])
    if not models:
        st.warning("No models found in `rag_model_site/data/models.json`.")
        return

    rows = []
    for model in models:
        rows.append(
            {
                "Model": model.get("id", ""),
                "Provider": model.get("provider", ""),
                "Input $/1M": model.get("pricePerMillionInputTokens", 0),
                "Output $/1M": model.get("outputPricePerMillionTokens", 0),
                "Best for": _localized(model.get("bestFor"), lang),
                "Tags": ", ".join(model.get("tags", [])),
            }
        )
    st.dataframe(rows, use_container_width=True, hide_index=True)

    with st.expander("Model notes"):
        for model in models:
            st.markdown(f"**`{model.get('id', '')}`**")
            st.write(_localized(model.get("longNote"), lang))
            links = model.get("sourceUrls") or []
            if links:
                st.caption("Sources: " + " · ".join(links))

    _render_public_examples(public_examples, lang)

    st.caption(
        "Static website version: run `cd rag_model_site && python3 -m http.server 8765`, "
        "then open `http://127.0.0.1:8765/`."
    )


def _render_public_examples(public_examples: dict[str, Any] | None, lang: str) -> None:
    if not public_examples:
        return

    with st.expander("Network/public example sources mapped to embedding models"):
        st.caption(_localized(public_examples.get("note"), lang))
        rows = []
        for item in public_examples.get("examples", []):
            rows.append(
                {
                    "Source": _localized(item.get("title"), lang),
                    "Category": _localized(item.get("category"), lang),
                    "Good for": _localized(item.get("goodFor"), lang),
                    "Recommended models": ", ".join(item.get("recommendedModels", [])),
                    "License note": _localized(item.get("licenseNote"), lang),
                    "URL": item.get("sourceUrl", ""),
                }
            )
        if rows:
            st.dataframe(rows, use_container_width=True, hide_index=True)
        for item in public_examples.get("examples", []):
            st.markdown(f"**{_localized(item.get('title'), lang)}**")
            st.write(_localized(item.get("why"), lang))
            st.caption(f"Source: {item.get('sourceUrl', '')}")


def _render_index_status() -> None:
    col1, col2, col3 = st.columns(3)
    col1.metric("Files indexed", st.session_state.document_count)
    col2.metric("Chunks", len(st.session_state.indexed_chunks))
    col3.metric("Last indexed", st.session_state.built_at or "Not built")


def _render_qa(
    *,
    answer_provider: str,
    openai_key: str,
    anthropic_key: str,
    embedding_model: str,
    answer_model: str,
    top_k: int,
    min_score: float,
) -> None:
    with st.form("qa_form"):
        st.text_input("Ask your knowledge base", key="kb_question")
        submitted = st.form_submit_button("Ask", use_container_width=True)

    if submitted:
        question = (st.session_state.get("kb_question") or "").strip()
        if not st.session_state.indexed_chunks:
            st.error("Build an index first.")
            return
        if not question:
            st.error("Enter a question first.")
            return
        if not openai_key:
            st.error("Add an OpenAI API key first.")
            return
        if answer_provider == "claude" and not anthropic_key:
            st.error("Add an Anthropic API key to use Claude for answers.")
            return

        try:
            embed_model = OpenAIEmbedding(
                api_key=openai_key,
                model=embedding_model,
            )
            hits = retrieve_hybrid(
                question,
                st.session_state.indexed_chunks,
                embed_model,
                top_k=top_k,
                min_vector_score=min_score,
            )
            if not hits:
                answer = insufficient_context_message()
            else:
                prompt = build_answer_prompt(question, hits)
                answer = _answer_question(
                    provider=answer_provider,
                    prompt=prompt,
                    answer_model=answer_model,
                    openai_key=openai_key,
                    anthropic_key=anthropic_key,
                )
        except Exception as error:  # pragma: no cover - UI error path
            st.error(f"Question failed: {error}")
            return

        st.session_state.last_answer = answer
        st.session_state.last_hits = hits

    if st.session_state.last_answer:
        st.subheader("Answer")
        st.write(st.session_state.last_answer)

        st.subheader("Sources used")
        if not st.session_state.last_hits:
            st.info("No relevant source chunks were retrieved for the last answer.")
        else:
            for hit in st.session_state.last_hits:
                channels = []
                if hit.vector_rank is not None:
                    channels.append("vector")
                if hit.bm25_rank is not None:
                    channels.append("BM25")
                channel_tag = "+".join(channels) if channels else "unknown"
                title = f"{hit.chunk.label} | RRF={hit.score:.4f} | channels={channel_tag}"
                with st.expander(title):
                    st.caption(f"File: {hit.chunk.file_name}")
                    c1, c2 = st.columns(2)
                    with c1:
                        st.markdown(
                            "**Vector** — "
                            + (
                                f"rank {hit.vector_rank}, sim {hit.vector_similarity:.3f}"
                                if hit.vector_rank is not None and hit.vector_similarity is not None
                                else "not in top‑K (or below min similarity)"
                            )
                        )
                    with c2:
                        st.markdown(
                            "**BM25** — "
                            + (
                                f"rank {hit.bm25_rank}, score {hit.bm25_score:.3f}"
                                if hit.bm25_rank is not None and hit.bm25_score is not None
                                else "not in top‑K"
                            )
                        )
                    st.write(format_source_excerpt(hit.chunk.text, max_chars=400))


def _answer_question(
    *,
    provider: str,
    prompt: str,
    answer_model: str,
    openai_key: str,
    anthropic_key: str,
) -> str:
    if provider == "claude":
        client = Anthropic(api_key=anthropic_key)
        return answer_with_claude(client, prompt, answer_model)

    client = OpenAI(api_key=openai_key)
    return answer_with_openai(client, prompt, answer_model)


def _default_answer_model(provider: str) -> str:
    if provider == "claude":
        return os.getenv("ANTHROPIC_ANSWER_MODEL", "claude-3-5-sonnet-latest")
    return os.getenv("OPENAI_ANSWER_MODEL", "gpt-4o-mini")


def _ensure_state() -> None:
    st.session_state.setdefault("indexed_chunks", [])
    st.session_state.setdefault("document_count", 0)
    st.session_state.setdefault("built_at", "")
    st.session_state.setdefault("last_answer", "")
    st.session_state.setdefault("last_hits", [])
    st.session_state.setdefault("kb_question", "")


if __name__ == "__main__":
    main()
