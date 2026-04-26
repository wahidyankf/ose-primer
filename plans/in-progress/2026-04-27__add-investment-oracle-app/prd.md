# PRD: Add `investment-oracle` Desktop Demo

## Product overview

`investment-oracle` is a desktop application — Tauri 2 shell + React +
Vite frontend + FastAPI Python sidecar — that turns a pile of financial
report PDFs into a single, structured Markdown investment thesis the user
can edit by hand or by asking an LLM to rewrite a section. It is the
template's reference for AI-shaped workloads (RAG, streaming, agentic
editing, desktop packaging) the same way `crud-*` is the reference for
CRUD-shaped workloads.

This PRD assumes the reader has read the five prerequisite primers
([AI](../../../docs/explanation/software-engineering/ai-application-development/README.md),
[Anthropic](../../../docs/explanation/software-engineering/ai-application-development/anthropic-api.md),
[Gemini](../../../docs/explanation/software-engineering/ai-application-development/google-gemini-api.md),
[OpenAI](../../../docs/explanation/software-engineering/ai-application-development/openai-api.md),
[Perplexity](../../../docs/explanation/software-engineering/ai-application-development/perplexity-api.md))
and uses their vocabulary without redefining it.

## Personas

These are the product-perspective actors that use or consume this document.
They are not the same as the brd.md Affected Roles (which describe the
business perspective); personas describe the hats worn when operating the
product or reading this PRD.

- **Maintainer-as-user** — the template maintainer running the desktop
  application: drags PDFs into the Sources pane, creates analyses, generates
  and edits reports, switches LLM providers. Experiences the product as an
  end-user and validates the UX against the acceptance criteria.
- **Plan-executor agent** — an AI agent stepping through the delivery
  checklist. Reads this PRD to understand what "done" looks like; uses the
  Gherkin acceptance criteria to confirm correct implementation of each FR.
- **Template consumer** — a developer cloning `ose-primer` to bootstrap their
  own AI-shaped application. Reads the PRD to understand the reference design
  patterns (RAG, SSE streaming, desktop packaging, guardrails) before adapting
  them.
- **CI system** — automated GitHub Actions workflows. Executes the test suites
  whose behaviour is specified in FR-15 through FR-15d; the Gherkin scenarios
  are the machine-readable contract the CI system validates on every push.

## User stories

**US-1 (Ingest)**: As a maintainer-as-user, I want to drag one or more PDF
financial reports into the Sources pane so that the backend extracts, chunks,
and embeds them for later retrieval without my having to invoke any CLI tool.

**US-2 (Generate)**: As a maintainer-as-user, I want to click "Generate report"
after selecting a set of sources so that the backend retrieves relevant chunks
and streams a six-section Markdown investment thesis into the editor pane in
real time.

**US-3 (Manual edit)**: As a maintainer-as-user, I want to edit the generated
report directly in the Markdown editor and save my changes so that my manual
annotations are preserved as a separate revision I can inspect later.

**US-4 (LLM edit)**: As a maintainer-as-user, I want to select a report section
and type a natural-language instruction (e.g., "make the Risks section more
cautious") so that the model rewrites only that section and saves the result
as a new `llm_edit` revision.

**US-5 (Revision history)**: As a maintainer-as-user, I want to open the revision
history drawer and restore any previous version of the report so that I can
recover from a bad edit without data loss.

**US-6 (Provider swap)**: As a maintainer-as-user, I want to switch the chat
model between `claude-haiku-4-5` and `gemini-2.5-flash-lite` at any point so
that I can compare cost and quality without restarting the application.

**US-7 (Guardrails)**: As a template consumer, I want to see how cost-cap and
content-filter guardrails are implemented so that I can adapt the same patterns
in my own AI application with confidence that runaway spending and unsafe
content are blocked.

**US-8 (Smoke)**: As a CI system, I want all three test levels to consume the
same Gherkin feature files so that the acceptance criteria serve as the
single source of truth from unit mock to real-HTTP e2e without scenario
duplication.

## Functional requirements

### Sources (PDF ingest)

**FR-1**: Drag-and-drop ingestion of one or more PDF files into the
**Sources** panel; multipart upload to `POST /api/v1/sources` with size
limit 25 MB per file. Files exceeding the limit return `413` with a
structured error envelope.

**FR-2**: BE extracts text via `pypdf` (BSD-3-Clause). PyMuPDF is **banned**
(AGPL). Each page becomes a `(source_id, page, text)` tuple.

**FR-3**: BE chunks each page-text into 800-token windows with 100-token
overlap (recursive splitter operating on paragraph boundaries first, then
sentence boundaries, then character windows). Each chunk is embedded by
calling `gemini-embedding-001` with `output_dimensionality=768` and
`task_type="RETRIEVAL_DOCUMENT"`. The (chunk text, vector) row is stored in
`source_chunks` with an ivfflat index.

**FR-4**: GET `/api/v1/sources` returns the list of ingested sources. DELETE
`/api/v1/sources/{id}` removes the source and cascades to its chunks.
Sources tied to an analysis cannot be deleted; the API returns `409` until
the analysis is deleted first.

### Analyses (sessions)

**FR-5**: `POST /api/v1/analyses` creates a named analysis and attaches a
list of source ids. `GET /api/v1/analyses/{id}` returns the analysis plus
its current report (if any) and revision count. `DELETE /api/v1/analyses/{id}`
cascades to its report, revisions, and messages.

**FR-5a**: An analysis attaches **N** sources (≥ 1). Retrieval queries
pgvector across the union of chunks belonging to attached sources, ordered
by cosine distance to the embedded query, limited to top-k (default `k=8`,
configurable per call).

### Report generation

**FR-6**: `POST /api/v1/analyses/{id}/report` generates the initial report.
Streamed via SSE. The system prompt instructs the model to produce six
fixed Markdown sections:

1. Executive Summary
2. Financial Health
3. Growth and Strategy
4. Risks and Headwinds
5. Valuation Considerations
6. Recommendation (with explicit "this is not investment advice" footer)

The retrieved chunks (top-k across attached sources) are embedded as system
context with their `source_id` and `page` so the model can cite. The model
streams Markdown into the FE; on completion the BE persists the report to
`reports.content_md` and a `report_revisions` row with `kind='generation'`.

**FR-6a**: The system prompt explicitly forbids fabricated numbers — the
model is instructed to qualify any figure it cannot find in the retrieved
chunks ("the filings do not specify …").

### Manual editing

**FR-7**: The right pane is a Markdown editor (CodeMirror 6 + Markdown
mode). The user edits content directly. `PATCH /api/v1/analyses/{id}/report`
saves the current text and writes a `report_revisions` row with
`kind='manual_edit'`.

### Prompt-driven editing

**FR-8**: The user selects a section heading (one of the six) and enters a
prompt (e.g., _"Make Recommendation more cautious."_). `POST
/api/v1/analyses/{id}/report:edit` applies the edit. Streamed via SSE. The
BE sends the **selected section's current text** + the user prompt + a
system instruction ("rewrite the section per the user's request; preserve
section heading; preserve the not-investment-advice footer in the
Recommendation section") to the chosen chat model. The streamed output
replaces the section in `reports.content_md` and writes a
`report_revisions` row with `kind='llm_edit'`, `prompt_text` filled in.

### Revision history

**FR-9**: `GET /api/v1/analyses/{id}/report/revisions` returns every
revision with `kind`, `prompt_text` (nullable), `created_at`, and a snapshot
of `content_md`. The FE renders this as a side drawer with a "restore this
revision" action. Restore is itself a new revision (`kind='restore'`) — old
revisions are never overwritten.

### Provider switching

**FR-10**: The chat model is selected per analysis from a dropdown:

- `claude-haiku-4-5` (Anthropic, default)
- `gemini-2.5-flash-lite` (Google, alternative)

Embedding model is fixed at `gemini-embedding-001` regardless of chat
provider. Switching models mid-analysis is allowed; the change applies to
the next chat call.

### Configuration

**FR-11**: Required environment variables:

| Variable                    | Purpose                                                                                                                                                |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `ANTHROPIC_API_KEY`         | Anthropic Messages API authentication                                                                                                                  |
| `GOOGLE_API_KEY`            | Google `google-genai` SDK authentication (chat + embeddings)                                                                                           |
| `PERPLEXITY_API_KEY`        | Perplexity Sonar API authentication (optional; required only when web grounding is enabled)                                                            |
| `DATABASE_URL`              | Postgres connection string (`postgresql+asyncpg://...`)                                                                                                |
| `MOCK_LLM_PROVIDERS`        | When `true`, intercepts all vendor HTTP via `httpx` cassette fixtures                                                                                  |
| `INVESTMENT_ORACLE_PORT`    | Sidecar HTTP port (default 8501; Tauri shell injects a random free port at runtime)                                                                    |
| `COST_CAP_PER_ANALYSIS_USD` | Per-analysis token budget (includes Perplexity search fees); default 0.75                                                                              |
| `COST_CAP_PER_DAY_USD`      | Per-day token budget across all analyses; default 5.00                                                                                                 |
| `RESIDENCY_PROFILE`         | One of `direct-us` (default), `bedrock-jakarta-cris`, `bedrock-jakarta-in-region`, `vertex-singapore`                                                  |
| `PII_MASKING_ENABLED`       | When `true` (default), all outbound LLM text passes through `PIIMasker.mask()`. Can be `false` only when `RESIDENCY_PROFILE=bedrock-jakarta-in-region` |
| `WEB_GROUNDING_ENABLED`     | When `true`, the analysis-create form exposes the "Include latest web grounding" toggle (default `false`)                                              |

`.env.example` ships placeholder values; real keys live in user's local
`.env` (gitignored) or in OS keychain when the desktop app is run.

### Guardrails

**FR-12**: Cost cap. Each chat call records token usage in `token_usage`.
On every chat call the BE checks (a) total tokens for the analysis ≤
`COST_CAP_PER_ANALYSIS_USD` budget and (b) total tokens for the day ≤
`COST_CAP_PER_DAY_USD` budget. Violation returns `429
token_budget_exceeded` with structured error envelope; FE shows a banner.

**FR-13**: Content filter. A `ContentFilter` Protocol scans (a) the user
prompt before calling the LLM and (b) every streamed chunk before
forwarding to the FE. The shipped implementation is a regex blocklist
(`local-temp/` not committed; default rules under
`apps/investment-oracle-be/content_filter/default_rules.txt`). A real
provider can be plugged in by binding a different Protocol implementation
in the dependency-injection container.

**FR-13a**: Per-IP rate limiting via `slowapi` is wired but **opt-in**
(disabled by default for desktop, single-user). A `RATE_LIMIT_ENABLED=true`
env var turns it on, useful when the same sidecar is deployed as a server.

### Indonesian residency and PII masking

**FR-RES-1 (Residency tagging)**: Every outbound HTTP call to a chat,
embedding, or web-grounding API carries an explicit residency tag
recorded on the `token_usage` row: `direct-us` (Anthropic / Gemini /
Perplexity direct, default), `bedrock-jakarta-cris` (Bedrock Jakarta
with Global Cross-Region Inference), `bedrock-jakarta-in-region`
(Bedrock Jakarta with in-region profile, currently only Claude Opus
4.7), or `vertex-singapore` (Gemini via Vertex AI Singapore). The tag
is logged at INFO level and included in the structured error envelope
when a guardrail rejects a call.

**FR-PII-1 (PII masker, default-on)**: A `PIIMasker` Protocol intercepts
**every text payload** leaving the BE for any LLM endpoint (chat,
embedding, web grounding). The default implementation
(`IndonesianRegexMasker`) detects and replaces:

| Pattern         | Detection                                    | Placeholder format |
| --------------- | -------------------------------------------- | ------------------ |
| NIK             | 16-digit numeric (NIK validator stub)        | `[NIK_001]`        |
| NPWP            | 15-digit numeric with dashes / dots accepted | `[NPWP_001]`       |
| Phone Indonesia | `+62` or `08` prefix, 9–14 digits            | `[PHONE_001]`      |
| Email           | RFC 5322 simple shape                        | `[EMAIL_001]`      |
| Bank account    | 10–16 digits, contextual                     | `[BANK_001]`       |
| Credit card     | Luhn-checked 13–19 digits                    | `[CC_001]`         |

The reverse map is held in memory only, scoped to the current call (or
streaming session for SSE). Streamed response chunks pass through
`PIIMasker.unmask()` before being persisted to `reports.content_md` and
emitted to the FE.

**FR-PII-2 (Bypass condition)**: Masking can be disabled per call only
when `RESIDENCY_PROFILE=bedrock-jakarta-in-region`. Any other profile
attempting to bypass returns `409 masking_required_for_residency`.

**FR-PII-3 (Test surface)**: A unit-level test asserts that for every
`ChatProvider.stream()` and `Embedder.embed()` invocation, the outbound
HTTP request body contains **no** raw NIK / NPWP / phone / email /
bank / credit-card pattern when `PII_MASKING_ENABLED=true`. This is a
content-fingerprint assertion on the wire, not on the LLM response.

### Web grounding (optional, Perplexity Sonar)

**FR-WG-1 (Toggle)**: When `WEB_GROUNDING_ENABLED=true`, the
`POST /api/v1/analyses` body accepts `web_grounding: true|false`
(default `false`) and the same flag is accepted on
`POST /api/v1/analyses/{id}/report` and
`POST /api/v1/analyses/{id}/report:edit`.

**FR-WG-2 (Sonar call shape)**: When the flag is on, the BE makes one
Perplexity Sonar call **before** the chat call:

- Model: `sonar` (default) or `sonar-pro` (configurable per analysis).
- Prompt: a structured query built from the analysis sources' filer
  names and the section being generated/edited.
- Filters: `search_recency_filter: "month"` by default;
  `search_domain_filter: ["sec.gov", "wsj.com", "reuters.com",
"bloomberg.com"]` by default to bias toward authoritative sources.
- Output passed through `PIIMasker.unmask()` only if the input was
  masked (Sonar receives no PII because it operates on issuer-name
  queries, not user-provided text).

**FR-WG-3 (Citation surfacing)**: Sonar's `citations` array is stored
on the `report_revisions` row in a new JSONB column `web_citations`.
The generated/edited Markdown distinguishes citation provenance with
inline markers: `[PDF p.45]` for PDF-RAG citations and
`[Web: domain.com]` for web-grounded ones.

**FR-WG-4 (Cost-cap accounting)**: The cost cap formula adds
Perplexity's per-request search fee (estimated `$0.01` for `sonar` at
`low` search context, scaling per the [Perplexity primer](../../../docs/explanation/software-engineering/ai-application-development/perplexity-api.md#pricing-2026-q2))
on top of token cost. The UI surfaces "+ web grounding ≈ $X" before
the user clicks **Generate** so cost is informed-consent.

**FR-WG-5 (Disabled-by-default-in-tests)**: All BE tests run with
`WEB_GROUNDING_ENABLED=false` unless the test exists specifically to
exercise the Perplexity path; cassettes for the Perplexity path live
alongside Anthropic and Gemini cassettes.

### UI requirements

**FR-14**: The window opens in a permanent **split view** with a vertical
divider:

- Left pane (30 % default width, drag-resizable): **Sources panel**.
  - Drop zone for new PDFs.
  - List of ingested sources with name, page count, ingested timestamp,
    delete button.
  - Multi-select for attaching to a new analysis.
- Right pane (70 % default width): **Report editor + Prompt input**.
  - Top: persistent disclaimer banner ("Demo output, not investment advice").
  - Middle: Markdown editor (CodeMirror 6) showing the current report,
    with a "Generate report" button visible when the report is empty.
  - Bottom: prompt input + section selector + send button (used for FR-8
    LLM edits).
  - Right edge: revision history drawer trigger.
- Top bar: analysis selector (switch between analyses), model selector
  (FR-10), settings cog.

**FR-14a**: All UI primitives come from `@open-sharia-enterprise/ts-ui`.
The plan introduces no app-local Button, Input, Dialog, etc. Compositions
specific to this app (SourcesPanel, ReportEditor, PromptInput,
RevisionHistoryDrawer) live in `apps/investment-oracle-fe/src/components/`.

**FR-14b**: Dark mode is supported via the existing ts-ui theme tokens.
WCAG AA contrast on every state.

### Test strategy

**FR-15**: Backend follows the **three-level testing standard** (per
`governance/development/quality/three-level-testing-standard.md`):

| Level              | Mocks                    | Real                                                                  | Tooling                                                                             | Cacheable                                                        |
| ------------------ | ------------------------ | --------------------------------------------------------------------- | ----------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| `test:unit`        | All vendor HTTP, all I/O | Service code                                                          | pytest + pytest-bdd (Gherkin from `specs/`); ruff; pyright; coverage.py LCOV ≥ 90 % | yes                                                              |
| `test:integration` | Vendor HTTP only         | Postgres + pgvector via docker-compose; pypdf parsing of fixture PDFs | pytest + pytest-bdd; consumes same Gherkin feature files                            | no — `cache: false` per `nx.json` (real DB is non-deterministic) |
| `test:e2e`         | Vendor HTTP only         | Postgres + pgvector + real FastAPI HTTP                               | Playwright + playwright-bdd                                                         | no                                                               |

**FR-15a**: Frontend follows the **two-level testing standard** (no
integration level — this is canonical per `crud-fe-*` precedent):

| Level       | Mocks                               | Real                                         | Tooling                                                          | Cacheable |
| ----------- | ----------------------------------- | -------------------------------------------- | ---------------------------------------------------------------- | --------- |
| `test:unit` | All BE HTTP via MSW; all Tauri APIs | React component code                         | vitest + @testing-library/react; oxlint; tsc strict; LCOV ≥ 70 % | yes       |
| `test:e2e`  | None                                | `vite preview` build of FE + real FastAPI BE | Playwright + playwright-bdd                                      | no        |

**FR-15b**: TypeScript end-to-end. The FE is TypeScript-strict with no
`any`, no `@ts-ignore`. `tsc --noEmit` runs as the `typecheck` Nx target.

**FR-15c**: LLM determinism. CI must never assert on LLM-generated prose.
The four allowed assertion patterns (per the AI primer §13):

1. **Outbound request fingerprint** — assert what was sent (model id,
   message shape, retrieved chunks, tool blocks).
2. **Side effects** — assert what was written to the DB
   (`reports.content_md` non-empty, `report_revisions` row created with
   correct `kind`, `token_usage` row upserted).
3. **Structural shape** — assert the response is well-formed SSE / JSON,
   matches the contract schema.
4. **Snapshot** of the cassette response prose (only because the cassette
   itself is deterministic — never on real-vendor output).

The forbidden pattern: asserting on what the model said. Real-vendor smoke
tests live behind a workflow-dispatch flag and run weekly; they assert
**only** structural shape and HTTP 200, never content.

**FR-15d**: All three test levels consume the **same Gherkin feature
files** under `specs/apps/investment-oracle/be/gherkin/` and
`specs/apps/investment-oracle/fe/gherkin/`. Step implementations differ
(mocks vs real DB vs real HTTP) but scenarios do not.

## Acceptance criteria (Gherkin)

### Sources

```gherkin
Feature: Source ingest

  Scenario: Upload a single PDF
    Given the sidecar is running
    When the user uploads "aapl-fy2024-10k.pdf" via POST /api/v1/sources
    Then the response is 201
    And the response body contains the new source id
    And the database has one row in "sources" with the matching SHA-256
    And the database has one row per page in "source_chunks" with embedding dim 768

  Scenario: Reject oversized PDF
    When the user uploads a 30 MB PDF
    Then the response is 413
    And the response body has error code "file_too_large"

  Scenario: Reject AGPL-licensed parser dependency at build
    Given a build of investment-oracle-be is attempted with PyMuPDF added to pyproject.toml
    When the doctor scope check runs
    Then the build fails with message "PyMuPDF is AGPL — banned per BRD"
```

### Analyses and report generation

```gherkin
Feature: Analysis and report

  Scenario: Create analysis and generate report (mocked LLM)
    Given two sources have been ingested
    And MOCK_LLM_PROVIDERS is true
    When the user creates an analysis attaching both sources
    And the user requests POST /api/v1/analyses/{id}/report
    Then the response is 200 with content-type text/event-stream
    And the SSE body ends with a [DONE] sentinel
    And the database has one row in "reports" with non-empty content_md
    And the database has one row in "report_revisions" with kind="generation"
    And the database has one row in "token_usage" with non-zero input_tokens

  Scenario: Generated report contains all six structured sections
    Given a generation cassette with section markers
    When generation completes
    Then the saved report content_md contains the six section H2 headings
```

### Manual editing

```gherkin
Feature: Manual report edit

  Scenario: Save manual edit
    Given an analysis has a generated report
    When the user PATCHes /api/v1/analyses/{id}/report with new content
    Then the response is 200
    And the database has a new row in "report_revisions" with kind="manual_edit"
    And the latest revision content_md matches the patched content
```

### LLM-driven editing

```gherkin
Feature: LLM section rewrite

  Scenario: Rewrite Risks section more cautiously (mocked LLM)
    Given an analysis has a generated report
    And MOCK_LLM_PROVIDERS is true
    When the user POSTs /api/v1/analyses/{id}/report:edit with section="Risks and Headwinds" and prompt="more cautious"
    Then the response is 200 with content-type text/event-stream
    And the outbound request to the chat provider includes the current Risks section text
    And the outbound request to the chat provider includes the user prompt
    And the database has a new row in "report_revisions" with kind="llm_edit"
    And the latest revision's content_md replaces only the Risks section
```

### Provider swap

```gherkin
Feature: Provider swap

  Scenario: Switch chat model from Anthropic to Gemini
    Given an analysis exists with model="claude-haiku-4-5"
    When the user updates the analysis with model="gemini-2.5-flash-lite"
    Then subsequent chat calls go to https://generativelanguage.googleapis.com (mocked)
    And no calls are made to https://api.anthropic.com
    And embedding calls continue to use gemini-embedding-001
```

### Guardrails

```gherkin
Feature: Cost cap

  Scenario: Per-analysis budget exceeded
    Given an analysis has consumed tokens worth $0.49
    And COST_CAP_PER_ANALYSIS_USD is 0.50
    When the user requests another report:edit that would exceed the cap
    Then the response is 429 with error code "token_budget_exceeded"
    And no chat call is made
```

```gherkin
Feature: Content filter

  Scenario: Reject input matching blocklist regex
    Given the user prompt contains a blocked phrase
    When report:edit is requested
    Then the response is 400 with error code "input_blocked"
    And no chat call is made
```

### Determinism (tests assert structurally, never on prose)

```gherkin
Feature: LLM-test determinism

  Scenario: Generation test asserts outbound-request fingerprint, not content
    Given a generation cassette returns "FIXTURE_REPORT"
    When generation completes
    Then the test asserts the outbound request used model="claude-haiku-4-5"
    And the test asserts the outbound request retrieved 8 chunks
    And the test does NOT assert any text from the cassette response

  Scenario: Snapshot test of cassette response (only because cassette is deterministic)
    Given the same cassette runs in CI today and a year from now
    When generation completes
    Then the saved report content_md matches the snapshot byte-for-byte

  Scenario: ivfflat retrieval test allows result-set membership, not order
    Given chunks with similar embeddings are inserted
    When retrieval runs with top-k=4
    Then the test asserts the returned chunk ids are a subset of the expected set
    And the test does NOT assert exact row order (ivfflat is approximate)
```

### Fixture-driven smoke

```gherkin
Feature: Manual smoke against shipped fixtures

  Scenario: Ingest the four shipped 10-K PDFs
    Given the four fixture PDFs in plans/.../fixture/
    When each is uploaded via POST /api/v1/sources
    Then each upload returns 201
    And the total chunk count is between 100 and 2000
    And no source has zero chunks
```

## Out (non-requirements)

- No fine-tuning. No agentic tool-use beyond the structured prompt.
- No comparison-mode UI ("compare these two companies"). Single-analysis
  scope is enough for the demo.
- No exports (PDF, DOCX). Markdown copy-out of the editor pane is the
  manual workaround; export is a backlog item.
- No collaboration. One desktop app, one user.
- No live web search **as the only retrieval surface**. Web grounding
  via Perplexity Sonar (FR-WG) is layered on top of PDF-RAG, never
  replacing it. A Sonar-only analysis (no PDFs attached) returns `400
no_sources_attached`.
- No re-ranking, no hybrid search. Single-vector retrieval is enough for a
  demo.

## Affected files (summary)

- New apps under `apps/`: `investment-oracle-be`, `investment-oracle-fe`,
  `investment-oracle-be-e2e`, `investment-oracle-fe-e2e`
- New spec area: `specs/apps/investment-oracle/`
- New contracts project: `specs/apps/investment-oracle/contracts/`
- New AI primers: under `docs/explanation/software-engineering/ai-application-development/`
- Updated `docker-compose.integration.yml`: pgvector image already present
  for crud-be, no change required at the compose layer; new init script
  adds the `vector` extension if not present.
- Updated `package.json` scripts: `dev:investment-oracle`,
  `build:investment-oracle`.
