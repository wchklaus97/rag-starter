# RAG radar runbook (manual Composer workflow)

Use this when you want to refresh **RAG-related techniques, tooling alternatives, and news** and record them **only in `docs/`** (not the static field guide). This complements **Track 4** in [RESEARCH_DIR.md](RESEARCH_DIR.md), which automates **OpenRouter metadata** and **Pagefind** for `rag_model_site/`.

**Composer** (including Composer-style multi-step work in Cursor) is an **in-IDE** workflow: it does not run on GitHub Actions. Sessions here end with a **human review** and a **git commit**.

---

## Preconditions

- **Repo context:** This monorepo spans experiments and split public apps; canonical shipping guidance is in [AGENTS.md](../AGENTS.md).
- **Skill routing:** If you use gstack-style skills, intent routing is described in [CLAUDE.md](../CLAUDE.md).
- **Research map:** Tracks 1–4 live in [RESEARCH_DIR.md](RESEARCH_DIR.md). Use the radar snapshot to **extend or challenge** those tracks with dated evidence—not to replace committed product specs without review.
- **Output location:** Copy [RAG_RADAR_SNAPSHOT_TEMPLATE.md](RAG_RADAR_SNAPSHOT_TEMPLATE.md) to a new file (e.g. `docs/RAG_RADAR_2026-Q2.md`) or maintain a single rolling `docs/RAG_RADAR_LATEST.md` if your team prefers one file.

---

## Session recipe (Composer / multi-agent)

1. **Scope the time window** in the prompt (e.g. “last 90 days” or “since last snapshot date”). Ask for **primary sources** and **concrete URLs**, not vibes.
2. **Split work** when useful: one thread for **retrieval / evals**, another for **vector DB / licensing**, another for **model/provider news**. Merge into one snapshot file to avoid duplication.
3. **Enforce citations in the draft:** For each bullet, the model should either paste the exact headline + URL or mark the item **unverified** if only secondary coverage exists.
4. **Map to this repo (one line each):** Under “Implications,” tie findings to Track 2 (Rust CLI / MCP), Track 3 (Python KB / hybrid search), Track 4 (field guide metadata), or Track 1 (on-device RAG)—or **none** if out of scope.
5. **Stop condition:** Close the session when the snapshot sections are filled, duplicates are merged, and every non–“unverified” claim has URL + access date.

Example prompt fragment:

> Search the open web and official docs for changes in RAG retrieval, hybrid / sparse search, citation-quality benchmarks, and major vector DB releases in [WINDOW]. For each claim: URL, access date (today), one-sentence implication for repo tracks in RESEARCH_DIR.md. Mark gaps as unverified.

---

## Search strategy

- **Prefer:** Vendor changelogs, official documentation, peer-reviewed or arXiv preprints with a stable link, and well-known framework release notes (e.g. LlamaIndex, LangChain, Haystack) when the claim is about **their** behavior.
- **Deprioritize:** Anonymous posts, unverifiable “X leaked” threads, and SEO listicles without primary links.
- **Dedupe:** Same story from three blogs counts as **one** finding; keep the **closest-to-primary** URL.
- **Date-stamp:** Write the **access date** (when you verified the page), not only the article date.

---

## Output contract (every finding)

| Field | Required |
|--------|----------|
| Short title | Yes |
| URL | Yes (unless row is explicitly **unverified**) |
| Access date | Yes (`YYYY-MM-DD`) |
| Implication for this monorepo | One line (or “none / out of scope”) |
| Track(s) touched | Optional: 1–4 or “cross-cutting” |

---

## Anti-hallucination

- Do not add a bullet **without** a link to supporting text (or mark **unverified** and explain what would confirm it).
- If sources disagree, note the disagreement and link both, or omit the claim.
- Prefer quoting a **short factual fragment** from the source (license change, metric, API shape) over paraphrase-heavy summaries.

---

## Closing steps (human)

1. Read the new or updated snapshot as a **human editor**; trim hype and fix vague wording.
2. From repo root: `git diff docs/`
3. Commit, e.g. `docs: RAG radar snapshot 2026-04`
4. Optional: add one line at the top of the snapshot pointing to the **previous** snapshot file for continuity.

---

## What this runbook is not

- **Not** a replacement for `scripts/collect_openrouter_models.py` or Pagefind CI (Track 4).
- **Not** permission to copy full articles; **link and summarize** only.
- **Not** automated crawling; periodic **manual** sessions only unless you later add a separate CI/RSS design.
