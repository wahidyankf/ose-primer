# BRD: Add `investment-oracle` Desktop Demo

## Business goal

Ship a second demo family in the ose-primer template — a **desktop**
"investment oracle" suite — so that template consumers see a credible
AI/RAG/agentic-editing reference next to the CRUD-over-Postgres reference.
Future AI-shaped demos in this template inherit conventions established here:
direct vendor SDKs (Anthropic + Google), RAG over pgvector, SSE streaming
end-to-end, Tauri 2 + Python sidecar packaging, document-grounded report
generation with prompt-driven editing.

## Problem

The template currently demonstrates one shape of work: synchronous CRUD over
Postgres across many languages. AI-shaped demos — long-running streams,
direct multi-vendor LLM calls, document ingestion, RAG retrieval, agentic
editing of generated artifacts, desktop packaging — do not exist anywhere in
the repo. A maintainer cloning ose-primer to bootstrap an AI product today
has no reference for any of:

- How to wire Anthropic Claude **and** Google Gemini side-by-side via their
  official SDKs (no proxy in the critical path).
- Where retrieval state belongs (separate vector DB? extension on the
  existing Postgres?).
- How to stream tokens from a Python backend through a desktop shell into a
  React UI inside a single window.
- How to spec a streaming endpoint and a multipart upload endpoint in
  OpenAPI 3.1 when the consumer is a desktop client, not a browser.
- How to test an LLM-shaped backend at three levels (unit / integration /
  e2e) without burning real LLM tokens in CI.
- How to package a FastAPI sidecar inside a Tauri 2 desktop app.
- How to combine LLM-generated content with manual + LLM-driven edits in one
  reproducible artifact (the report).

Every one of those decisions is reusable across future AI demos. Without a
reference implementation, each new AI demo will re-decide them
inconsistently.

## Root cause

The original demo family was scoped to CRUD before any AI-shaped demos were
on the roadmap. The conventions table in
`governance/development/quality/three-level-testing-standard.md` assumes
deterministic backends; the OpenAPI contract pattern in
`specs/apps/crud/contracts/` assumes synchronous request/response; the
integration docker-compose pattern assumes Postgres without extensions; the
build matrix assumes web targets only.

None of those patterns is wrong for CRUD — they just need a sibling reference
that shows the same level of rigour applied to a non-deterministic, streaming,
RAG-shaped, desktop-packaged workload.

## Proposed solution

Add four projects under a new `investment-oracle` family — one Python /
FastAPI backend (sidecar binary), one Tauri 2 desktop app (Rust shell + React

- Vite + ts-ui frontend), two Playwright E2E suites — plus a shared
  `specs/apps/investment-oracle/` spec area mirroring the structure of
  `specs/apps/crud/`. The backend uses `pypdf` for extraction, pgvector
  (extension on the existing docker-compose Postgres image) for retrieval,
  Anthropic Claude Haiku 4.5 (default) and Google Gemini 2.5 Flash-Lite
  (alternate) for chat completions, and Google `gemini-embedding-001` (768
  dimensions) for embeddings — Anthropic does not offer an embedding endpoint,
  so even when chat is served by Anthropic the embedding step is served by
  Google.

The desktop shell (Tauri 2) bundles the FastAPI sidecar via PyInstaller
`--onedir` and `bundle.externalBin`, spawns it on launch, kills it on close.
The frontend renders a permanent split view: sources panel (left) and report
editor + prompt input (right). Reports start as LLM-generated Markdown and
can be edited manually or by selecting a section and prompting the model to
rewrite it; every save records a revision.

Four real-world fixture PDFs (Apple FY2024 10-K, Microsoft FY2024 annual
report, Tesla FY2024 annual report, Berkshire Hathaway FY2024 annual report,
~5.7 MB total) ship with the plan in [`fixture/`](./fixture/). Manual smoke
loads them; integration tests use the Apple PDF; E2E uses a tiny synthetic
PDF for speed.

## Business impact

| Value                                | Detail                                                                                                                        |
| ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| Reference for AI workloads           | Template consumers can copy `investment-oracle-be` as the starting point for any RAG / report-generation demo                 |
| Direct multi-vendor pattern          | Anthropic SDK + Google `google-genai` SDK side-by-side; switching from Haiku to Flash-Lite is one config change, no proxy hop |
| Vector retrieval pattern             | pgvector demonstrated as a zero-new-infra path for RAG; reuses the integration docker-compose Postgres image                  |
| Streaming end-to-end pattern         | SSE from FastAPI → Tauri sidecar HTTP → React (`@microsoft/fetch-event-source`) is documented, tested, and reproducible       |
| OpenAPI for AI endpoints pattern     | The `investment-oracle-contracts` spec shows how to describe multipart upload + SSE in OpenAPI 3.1                            |
| Desktop packaging pattern            | Tauri 2 + PyInstaller `--onedir` sidecar pattern documented end-to-end, including kill-on-close and platform binary suffixes  |
| Document-grounded generation pattern | Structured-section prompt + RAG retrieval + streaming response is reusable for any "draft me a report from these docs" tool   |
| Agentic editing pattern              | Manual Markdown edit + section-scoped LLM rewrite + revision history is the shape of every modern LLM document UX             |
| Test strategy under non-determinism  | Three-level BE testing with `MOCK_LLM_PROVIDERS=true` cassette injection is the playbook for every AI demo                    |
| Naming clarity preserved             | `investment-oracle-*` follows the same family-prefix pattern locked in by the recent `crud-*` rename                          |
| Indonesian residency awareness       | Every outbound LLM call carries an explicit residency tag; the demo documents the Bedrock Jakarta upgrade path                |
| PII masking layer                    | A `PIIMasker` Protocol scrubs Indonesian PII (NIK, NPWP, phone, email, bank, credit card) before any cross-border LLM call    |
| Web-grounding pattern                | Optional Perplexity Sonar call layers public-web context on top of private-PDF RAG; demonstrates the third vendor lane        |

## In scope (notable inclusions)

- **Desktop packaging**: Tauri 2 + Rust shell + React + Vite + ts-ui +
  Python sidecar via PyInstaller `--onedir`. macOS .app/.dmg, Windows .msi,
  Linux .AppImage targets configured; CI ships only macOS arm64 to keep the
  matrix small.
- **Direct vendor SDKs**: Anthropic + Google in the critical path; no
  OpenRouter or other proxy. Vendor abstraction lives in a small
  `ChatProvider` Protocol so swapping or adding vendors is mechanical.
- **Document-grounded report generation**: structured-section prompt,
  retrieved chunks across multiple PDFs, SSE-streamed Markdown response.
- **Hybrid editing**: manual Markdown edit **and** prompt-driven section
  rewrite. Both produce revision rows.
- **Multi-document analyses**: an analysis attaches N source PDFs;
  retrieval queries pgvector across all attached sources. Demonstrates
  session-scoped RAG, not just doc-scoped RAG.
- **Production guardrails**: cost cap (per-analysis and per-day) and content
  filter on input + streamed output, both retained from the template's
  prior AI plan. Per-IP rate limit is opt-in (single-user desktop) but the
  Protocol is wired so a deployed-as-server variant inherits it.
- **Indonesian residency tagging + PII masking**: every outbound LLM call
  is tagged with a residency posture (`direct-us`, `bedrock-jakarta-cris`,
  `bedrock-jakarta-in-region`, `vertex-singapore`); a `PIIMasker` Protocol
  with a default Indonesian-regex implementation strips NIK / NPWP /
  phone / email / bank / credit-card patterns from any text leaving the
  BE for a non-on-shore endpoint, with a numbered-placeholder reverse
  map so streamed responses unmask correctly before display.
- **Optional web grounding via Perplexity Sonar**: a `WebGrounder`
  Protocol (default impl: `PerplexityGrounder`) supplements PDF-RAG
  context with live web-grounded context (recent news, sentiment,
  material developments). Sonar is opt-in per analysis or per LLM edit;
  citations are surfaced in the report as `[Web: domain.com]` markers
  alongside `[PDF page N]` markers from RAG.
- **Permanent disclaimer banner**: "Demo output, not investment advice"
  ribbon stays visible at the top of the window.
- **Shipped fixture PDFs**: four real 10-K-class financial reports in
  `plans/in-progress/.../fixture/` for manual smoke. Underlying filings are
  public per SEC policy; the fixture README documents source URLs and
  hashes.

## Out of scope

- Polyglot backends (only `investment-oracle-be` Python/FastAPI ships in
  this plan; future language ports are a separate plan).
- Authentication, authorization, multi-tenancy. (Sessions and analyses are
  local to one machine; the sidecar binds to `127.0.0.1` only.)
- Hybrid search, re-ranking, query rewriting.
- Distributed rate limiting or distributed cost accounting (in-process
  counters on a single backend instance — same simplifying constraint as
  the rest of the demo set).
- Real moderation provider (the content filter ships with a regex blocklist
  - a swap point for plugging in a real provider).
- Tauri-shell automated E2E. The shell is verified by manual smoke;
  Playwright drives `vite preview` (FE in browser mode) and direct FastAPI
  calls (BE).
- Production-grade investment research. The disclaimer is permanent; there
  are no DCF calculators, no live price feeds, no portfolio tools.
- Migrating any existing demo functionality.

## Success at business level

A maintainer cloning ose-primer in mid-2026 to start a new AI product can:

1. Read `apps/investment-oracle-be/README.md` and understand the full RAG
   pipeline plus the two-vendor abstraction.
2. Read `apps/investment-oracle-fe/README.md` and understand the Tauri shell,
   sidecar wiring, and split-view UX.
3. Run `npm run dev:investment-oracle` to launch the desktop app locally
   inside ten minutes from a clean clone (assumes Docker for Postgres +
   pgvector is already running per the standard CRUD-demo setup).
4. Drag one of the shipped fixture PDFs onto the sources pane and watch a
   structured Markdown report stream into the right pane.
5. Swap the chat model from Anthropic Haiku to Gemini Flash-Lite by
   changing one config dropdown, with no code change.
6. Read `specs/apps/investment-oracle/contracts/openapi.yaml` and understand
   exactly how a streaming chat endpoint and a multipart upload are
   described.

## Prerequisite reading

Every executor and reviewer of this plan reads the repo-wide AI primer
**and** the four vendor primers and the testing companion first:

- [AI Application Development](../../../docs/explanation/software-engineering/ai-application-development/README.md)
- [Anthropic API Primer](../../../docs/explanation/software-engineering/ai-application-development/anthropic-api.md)
- [Google Gemini API Primer](../../../docs/explanation/software-engineering/ai-application-development/google-gemini-api.md)
- [OpenAI API Primer](../../../docs/explanation/software-engineering/ai-application-development/openai-api.md)
  (read for boundary framing — OpenAI is not used in this demo)
- [Perplexity Sonar API Primer](../../../docs/explanation/software-engineering/ai-application-development/perplexity-api.md)
  (Perplexity is used for optional web grounding (FR-WG); read for the Sonar API shape)
- [Testing AI Applications](../../../docs/explanation/software-engineering/ai-application-development/testing-ai-apps.md)
  (cross-cutting testing playbook — implemented by PRD FR-15 family)

This plan does not redefine RAG, embeddings, streaming, guardrails, or any of
the vendor-specific vocabulary covered there.

## Affected roles

- **Maintainer (template author)**: implements the four projects, the spec
  tree, the Gherkin scenarios, the contract, and the CI workflows; signs
  off the quality gates; has read the AI primer and all four vendor
  primers and the testing companion.
- **AI agents (`plan-executor`, `swe-python-dev`, `swe-typescript-dev`,
  `swe-rust-dev`, `swe-e2e-dev`)**: read this plan, the PRD, the tech-docs,
  and the six primers to orient themselves; consume the delivery checklist
  as their step-by-step driver.
- **Template consumers (cloners)**: benefit from a copy-paste-ready
  AI-desktop reference the day this plan lands.
- **CI system**: runs `test-investment-oracle-*.yml` workflows on schedule
  and on workflow dispatch, mirroring the `test-crud-*.yml` pattern. Real
  vendor calls only on workflow dispatch with budget awareness.

## Business risks

| Risk                                                                                                          | Likelihood | Mitigation                                                                                                                                                                 |
| ------------------------------------------------------------------------------------------------------------- | ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AGPL contamination from PyMuPDF if a contributor swaps the PDF parser                                         | Medium     | tech-docs.md locks in `pypdf` (BSD-3-Clause) + optional `pdfplumber` (MIT) and bans PyMuPDF in a CONTRIBUTING note; doctor scope check flags new deps                      |
| Anthropic / Google model id changes break the demo (`claude-haiku-4-5` deprecated, Sonnet 4.5 already legacy) | Medium     | Model ids are config values, not hard-coded; tech-docs lists each vendor's current-models endpoint                                                                         |
| LLM tokens consumed in CI cause cost surprise                                                                 | Medium     | `test:quick` and `test:unit` use `httpx` mock cassettes for both vendors; only manually-triggered E2E hits real APIs                                                       |
| Tauri sidecar packaging breaks on a platform we don't CI (Linux, Windows)                                     | Medium     | CI ships macOS arm64 only; tech-docs.md documents per-platform binary suffix rules and known PyInstaller gotchas with heavy Python ML deps                                 |
| AI SDK breaking changes confuse contributors familiar with older versions                                     | Medium     | tech-docs.md pins `anthropic@0.97`, `@anthropic-ai/sdk@0.90`, `google-genai@1.73`, `@google/genai`@npm-latest; vendor primers reference exact docs URLs                    |
| OpenAPI 3.1 cannot fully describe SSE; codegen produces incomplete types for the streaming endpoint           | Low        | tech-docs.md acknowledges the limitation, hand-rolls the streaming type, and tracks 3.2 tooling readiness as a backlog item                                                |
| pgvector extension not enabled in default Postgres image                                                      | Low        | Use `pgvector/pgvector:pg16` image in `docker-compose.integration.yml`; covered in delivery checklist                                                                      |
| API key leakage from `.env.example` if real keys committed                                                    | Low        | `.env.example` ships placeholders only; pre-commit hook + repo `.gitignore` catch real `.env` files                                                                        |
| Investment-research output mistaken for advice                                                                | High       | Permanent disclaimer banner in FE; system prompt instructs the model to refuse direct buy/sell recommendations; PRD + tech-docs reinforce the framing                      |
| Personal data crosses border to US (Anthropic / Perplexity) or Singapore (Gemini fallback)                    | High       | `PIIMasker` always-on for `direct-us` and `vertex-singapore` routes; UU PDP Article 56 SCC-or-consent posture documented; pre-/post-transfer report template ships in repo |
| Indonesian-residency upgrade path (Bedrock Jakarta) not exercised in CI                                       | Medium     | tech-docs.md documents the Bedrock route; manual smoke phase exercises it; CI lane added in a follow-up plan                                                               |
| Perplexity Sonar adds non-trivial cost (per-token + per-search fee)                                           | Medium     | Web grounding is opt-in per analysis; cost cap accounts for Perplexity per-request fee; UI surfaces the "+web grounding" cost delta before generation                      |
| Future AI demo plan diverges from these conventions                                                           | Medium     | Document conventions in `governance/development/pattern/llm-demo-pattern.md` (created as part of this plan)                                                                |
