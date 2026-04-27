# OpenRouter embed proxy (local playground)

The static field guide on GitHub Pages **does not** send your API key from the browser. To try
live embeddings against OpenRouter:

1. Export your key and start the proxy (listens on **127.0.0.1:8790** by default):

   ```bash
   export OPENROUTER_API_KEY="sk-or-..."
   uv run python embed_proxy/openrouter_embed_proxy.py
   ```

2. Serve `rag_model_site/` over HTTP (not `file://`) and open **`embed-playground.html`**.

3. Confirm the **Proxy base URL** is `http://127.0.0.1:8790`, pick a model, enter text, **Send**.

## Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| GET | `/health` | Liveness JSON |
| POST | `/api/embed` | Body: `{"model":"openai/text-embedding-3-small","input":"hello"}` |

The proxy strips full embedding vectors from the JSON it returns (dimension + first values + usage) so responses stay small.

## Security

- Default bind: **127.0.0.1** only.
- Do **not** expose this process to the public internet without authentication and HTTPS.
- For production demos, deploy a proper serverless function or backend that stores secrets in environment variables.
