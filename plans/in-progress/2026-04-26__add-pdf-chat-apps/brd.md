# BRD: Add `pdf-chat-*` Demo App Family

## Business goal

Ship a second demo family in the ose-primer template — a "chat with a PDF" suite — so
that template consumers see a credible AI/RAG reference next to the CRUD reference, and
so future AI-shaped demos inherit conventions established here (multi-provider LLM
routing, RAG over pgvector, SSE streaming end-to-end).

## Problem

The template currently demonstrates one shape of work: synchronous CRUD over Postgres
across many languages. AI-shaped demos — long-running streams, multi-provider model
routing, document ingestion, RAG retrieval — do not exist anywhere in the repo. A
maintainer cloning ose-primer to bootstrap an AI product has no reference for any of:

- How to wire OpenRouter as a single egress point for multiple LLM vendors.
- Where retrieval state belongs (separate vector DB? extension on the existing
  Postgres?).
- How to stream tokens from a Python backend through a Next.js BFF to a React UI.
- How to spec a streaming endpoint and a multipart upload endpoint in OpenAPI 3.1.
- How to test an LLM-shaped backend at three levels (unit / integration / e2e) without
  burning real LLM tokens in CI.

Every one of those decisions is reusable across future AI demos. Without a reference
implementation, each new AI demo will re-decide them inconsistently.

## Root cause

The original demo family was scoped to CRUD before any AI-shaped demos were on the
roadmap. The conventions table in `governance/development/quality/three-level-testing-standard.md`
assumes deterministic backends; the OpenAPI contract pattern in
`specs/apps/crud/contracts/` assumes synchronous request/response; the integration
docker-compose pattern assumes Postgres without extensions.

None of those patterns is wrong for CRUD — they just need a sibling reference that
shows the same level of rigour applied to a non-deterministic, streaming, RAG-shaped
workload.

## Proposed solution

Add four apps under a new `pdf-chat` family — one Python/FastAPI backend, one Next.js
frontend, two Playwright E2E suites — plus a shared `specs/apps/pdf-chat/` spec area
mirroring the structure of `specs/apps/crud/`. The backend uses pypdf for extraction,
pgvector (extension on the existing docker-compose Postgres image) for retrieval, and
OpenRouter for both embeddings and chat completions. The frontend uses the Vercel AI
SDK 5 `useChat` hook proxied through a Next.js Route Handler.

## Business impact

| Value                            | Detail                                                                                                                 |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Reference for AI workloads       | Template consumers can copy `pdf-chat-be` as the starting point for any RAG demo                                       |
| Multi-provider routing pattern   | OpenRouter as the single egress; switching from Haiku to Gemini 2.5 Flash Lite is one env var change                   |
| Vector retrieval pattern         | pgvector demonstrated as a zero-new-infra path for RAG; reuses the integration docker-compose Postgres image           |
| Streaming end-to-end pattern     | SSE from FastAPI → Next.js Route Handler → AI SDK `useChat` is documented, tested, and reproducible                    |
| OpenAPI for AI endpoints pattern | The `pdf-chat-contracts` spec shows how to describe multipart upload + SSE in OpenAPI 3.1                              |
| Persistent thread pattern        | Sessions table + messages table + RAG retrieval scoped by session — reusable shape for every thread-based chat product |
| Multi-doc retrieval pattern      | A single pgvector query joining a `session_pdfs` link table — reusable shape for any "chat with my workspace" product  |
| Guardrails reference layer       | Rate limit + content filter + token cost cap as composable middleware — reusable shape for every public AI endpoint    |
| Naming clarity preserved         | `pdf-chat-*` follows the same family-prefix pattern locked in by the recent `crud-*` rename                            |

## In scope (notable inclusions)

- **Persistent chat sessions**: chat history is stored in Postgres so conversations
  survive reload, server restart, and tab close. Demonstrates the durable-thread
  pattern needed by virtually every production chat product.
- **Multi-document conversations**: a session attaches N PDFs; retrieval queries
  pgvector across all attached PDFs. Demonstrates session-scoped RAG, not just
  doc-scoped RAG.
- **Production guardrails**: per-IP rate limit, content filter on input and on streamed
  output, per-session and per-day token cost cap. Demonstrates the defensive layer that
  every public AI surface needs.

## Out of scope

- Polyglot backends (only `pdf-chat-be` Python/FastAPI ships in this plan; future
  language ports are a separate plan, mirroring the crud-be matrix).
- Authentication, authorization, multi-tenancy. (Sessions are anonymous and addressable
  by URL/UUID — adequate for a demo, deliberately not production-secure.)
- Hybrid search, re-ranking, query rewriting.
- Distributed rate limiting or distributed cost accounting (in-process counters on a
  single backend instance — same simplifying constraint as the rest of the demo set).
- Real moderation provider (the content filter ships with a regex blocklist + a swap
  point for plugging in a real provider).
- Migrating any existing demo functionality.

## Success at business level

A maintainer cloning ose-primer in mid-2026 to start a new AI product can:

1. Read `apps/pdf-chat-be/README.md` and understand the full RAG pipeline.
2. Run `npm run dev:pdf-chat-be` and `npm run dev:pdf-chat-fe` and chat with a PDF
   locally inside ten minutes from a clean clone.
3. Swap the model from Haiku to Gemini Flash Lite by changing one env var, with no code
   change.
4. Read `specs/apps/pdf-chat/contracts/openapi.yaml` and understand exactly how a
   streaming chat endpoint and a multipart upload are described.

## Prerequisite reading

Every executor and reviewer of this plan reads the repo-wide AI primer first:

- [AI Application Development](../../../docs/explanation/software-engineering/ai-application-development/README.md)

This plan does not redefine RAG, embeddings, streaming, guardrails, or any of the
vocabulary covered there.

## Affected roles

- **Maintainer (template author)**: implements the four apps, the spec tree, the
  Gherkin scenarios, the contract, and the CI workflows; signs off the quality gates;
  has read the AI primer.
- **AI agents (`plan-executor`, `swe-python-dev`, `swe-typescript-dev`,
  `swe-e2e-dev`)**: read this plan, the PRD, the tech-docs, and the AI primer to
  orient themselves; consume the delivery checklist as their step-by-step driver.
- **Template consumers (cloners)**: benefit from a copy-paste-ready RAG/chat reference
  the day this plan lands.
- **CI system**: runs `test-pdf-chat-*.yml` workflows on schedule and on workflow
  dispatch, mirroring the `test-crud-*.yml` pattern.

## Business risks

| Risk                                                                                                | Likelihood | Mitigation                                                                                                                          |
| --------------------------------------------------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| AGPL contamination from PyMuPDF if a contributor swaps the PDF parser                               | Medium     | tech-docs.md locks in `pypdf` (MIT) + `pdfplumber` (MIT) and bans PyMuPDF in a CONTRIBUTING note; doctor scope check flags new deps |
| OpenRouter model id changes break the demo (`claude-haiku-4.5` deprecated)                          | Low        | Model id is env var, not hard-coded; tech-docs lists the verification command (`GET /api/v1/models`)                                |
| LLM tokens consumed in CI cause cost surprise                                                       | Medium     | `test:quick` and `test:unit` use a fake OpenRouter via httpx mock; only manually-triggered E2E hits the real API                    |
| AI SDK 5 transport breaking change confuses contributors familiar with v4                           | Medium     | tech-docs.md links the AI SDK 5 announcement; package.json pins `ai@^5`                                                             |
| OpenAPI 3.1 cannot fully describe SSE; codegen produces incomplete types for the streaming endpoint | Low        | tech-docs.md acknowledges the limitation, hand-rolls the streaming type, and tracks 3.2 tooling readiness as a backlog item         |
| pgvector extension not enabled in default Postgres image                                            | Low        | Use `pgvector/pgvector:pg16` image in `docker-compose.integration.yml`; covered in delivery checklist                               |
| API key leakage from `.env.example` if real keys committed                                          | Low        | `.env.example` ships placeholders only; pre-commit hook + repo .gitignore catch real `.env` files                                   |
| Future AI demo plan diverges from these conventions                                                 | Medium     | Document conventions in `governance/development/pattern/llm-demo-pattern.md` (created as part of this plan)                         |
