# Security and Token Handling Policy

This document outlines the security protocols for handling API keys, tokens, and sensitive data within the `rag-starter` repository.

## 1. Secret Storage
- **No Hardcoding**: API keys, bearer tokens, and other credentials MUST NOT be hardcoded in source code or committed to the repository.
- **Environment Variables**: Use `.env` files for local development. This repo is configured with `dotenvy` to load these.
- **Gitignore**: The `.env` file and any files in `.cursor/` (where logs are stored) are excluded from version control via `.gitignore`.

## 2. Log Sanitization
- **Debug Logs**: The `debug_ndjson` logger automatically redacts sensitive patterns before writing to disk.
- **Patterns Redacted**:
    - OpenAI API Keys (`sk-...`)
    - DeepSeek API Keys
    - Anthropic API Keys
    - Generic Bearer Tokens
- **Manual Audits**: Even with automatic sanitization, avoid passing raw sensitive data in the `message` or `data` fields of a log call whenever possible.

## 3. Redaction Standards
- Redacted strings should be replaced with a clear marker: `[REDACTED_SECRET]`.
- Partial redaction (showing the first 4 characters) is permitted only when necessary for troubleshooting, e.g., `sk-proj-ABCD...`.

## 4. Revocation and Rotation
- If a secret is accidentally committed or exposed in a log:
    1. **Revoke** the key immediately in the provider's dashboard (OpenAI, DeepSeek, etc.).
    2. **Rotate** the key by generating a new one and updating your `.env` file.
    3. **Purge** history if committed to Git (using `git filter-repo` or similar tools).

## 5. Mutating Workflows
- Before implementing tools that can mutate external state (e.g., `write_file`, `send_email`, `execute_unrestricted_shell`), an additional security review of the tool's access scope is required.
