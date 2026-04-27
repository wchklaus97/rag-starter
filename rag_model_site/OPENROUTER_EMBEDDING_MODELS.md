# OpenRouter Embedding Models for RAG

> **Note:** a copy of the root `OPENROUTER_EMBEDDING_MODELS.md` for GitHub Pages. Edit the file at the repository root, then run `uv run python scripts/collect_openrouter_models.py` to sync, or copy it here manually.

OpenRouter exposes embedding models through:

```text
POST https://openrouter.ai/api/v1/embeddings
```

Embedding models are used to convert text into vectors. In a RAG app, those vectors let the app find the most relevant document chunks before asking an answer model to respond with citations.

For embeddings, pricing is usually based on input tokens only. The returned vector is not billed like generated output tokens.

## Quick Recommendation

| Situation | Recommended model |
| --- | --- |
| Cheapest useful starting point | `baai/bge-m3` or `qwen/qwen3-embedding-8b` |
| Safe default for most English RAG demos | `openai/text-embedding-3-small` |
| Higher retrieval quality over cost savings | `openai/text-embedding-3-large` |
| Multilingual knowledge bases | `baai/bge-m3` or `qwen/qwen3-embedding-8b` |
| Code search or developer docs | `mistral/codestral-embed-2505` or `qwen/qwen3-embedding-8b` |
| Google ecosystem preference | `google/gemini-embedding-001` |

## Model Summary

| Model | Approx price per 1M input tokens | Output price | Main strengths | Best usage area |
| --- | ---: | ---: | --- | --- |
| `baai/bge-m3` | `$0.01` | `$0` | Very cheap, strong multilingual retrieval, good general search quality | Low-cost RAG, multilingual docs, enterprise knowledge bases |
| `qwen/qwen3-embedding-8b` | `$0.01` | `$0` | Very cheap, strong multilingual and technical retrieval potential | Mixed English/Chinese docs, technical docs, code-adjacent RAG |
| `openai/text-embedding-3-small` | `$0.02` | `$0` | Reliable, fast, cheap, widely used | Default RAG demos, FAQ bots, internal KB assistants |
| `openai/text-embedding-3-large` | `$0.13` | `$0` | Higher semantic quality than the small model, better for difficult matching | High-accuracy retrieval, messy documents, larger production KBs |
| `google/gemini-embedding-001` | `$0.15` | `$0` | Strong general semantic embeddings, good for Google model stacks | General-purpose RAG, Google-first applications |
| `mistral/codestral-embed-2505` | `$0.15` | `$0` | Code-oriented semantic retrieval | Source code search, API docs, developer assistant RAG |
| `mistral/mistral-embed-2312` | `$0.15` | `$0` | Stable general-purpose embedding model | General RAG when using Mistral ecosystem |

Prices can change, so verify the current price on OpenRouter before production use.

## When Each Model Is Useful

### `baai/bge-m3`

Use this when cost matters and the knowledge base may contain multiple languages. It is a strong candidate for internal document search because it performs well on retrieval-style tasks and is very inexpensive.

Good fit for:

- Company policies
- HR or onboarding documents
- Multilingual support docs
- Cost-sensitive prototypes

### `qwen/qwen3-embedding-8b`

Use this when the corpus includes technical language, multilingual text, or Chinese/English mixed content. It is also one of the cheapest options, so it is worth benchmarking early.

Good fit for:

- Technical documentation
- Mixed-language knowledge bases
- Engineering notes
- Code-adjacent explanations

### `openai/text-embedding-3-small`

Use this as the safest default. It is cheap, fast, familiar, and good enough for many RAG applications. For this repo's demo-style internal knowledge assistant, this is a practical first choice.

Good fit for:

- RAG demos
- FAQ assistants
- Internal policy search
- Small to medium knowledge bases

### `openai/text-embedding-3-large`

Use this when retrieval quality is more important than embedding cost. It can help when questions are indirect, documents are messy, or small wording differences matter.

Good fit for:

- Production RAG with higher accuracy requirements
- Larger knowledge bases
- Legal, compliance, or policy retrieval
- Hard semantic matching

### `google/gemini-embedding-001`

Use this if you are already building around Gemini models or want to test Google embeddings against OpenAI and BGE/Qwen alternatives.

Good fit for:

- General-purpose RAG
- Google-centered application stacks
- Long document retrieval experiments

### `mistral/codestral-embed-2505`

Use this when the main content is source code, API documentation, SDK docs, or developer guides. Code embeddings need to understand identifiers, functions, and technical relationships better than normal text embeddings.

Good fit for:

- Repository search
- Developer assistants
- API documentation search
- Code explanation RAG

### `mistral/mistral-embed-2312`

Use this as a general-purpose embedding option when you prefer Mistral's ecosystem. It is not the cheapest option, but it is simple and stable.

Good fit for:

- General knowledge-base RAG
- Mistral-centered stacks
- Provider-diversity testing

## Practical Testing Plan

For this repo, test retrieval quality on the same set of questions before choosing permanently.

1. Start with `openai/text-embedding-3-small` as the baseline.
2. Compare against `baai/bge-m3` because it is cheaper and strong for retrieval.
3. Compare against `qwen/qwen3-embedding-8b` if the docs are technical or multilingual.
4. Try `openai/text-embedding-3-large` only if the cheaper models miss relevant chunks.
5. Use `mistral/codestral-embed-2505` if the corpus becomes mostly source code.

The best embedding model is the one that retrieves the right chunks for your actual questions, not always the largest or most expensive one.
