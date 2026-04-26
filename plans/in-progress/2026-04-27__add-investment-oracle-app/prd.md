# PRD: Add `pdf-chat-*` Demo App Family

## Product overview

Four new Nx projects (`pdf-chat-be`, `pdf-chat-fe`, `pdf-chat-be-e2e`, `pdf-chat-fe-e2e`)
plus a shared spec tree (`specs/apps/pdf-chat/`) and a shared OpenAPI contract project
(`pdf-chat-contracts`). End-user product: upload a PDF, chat with it. The conversation
runs against either Anthropic Claude Haiku 4.5 or Google Gemini 2.5 Flash Lite,
selectable per request, both routed through OpenRouter.

## Product scope

In:

- One Python/FastAPI backend implementing PDF upload, chunking, embedding, vector
  storage, retrieval, and streaming chat completion.
- One Next.js 16 frontend with file upload UI and a chat UI streaming via Vercel AI SDK 5.
- One Playwright BE-E2E suite for backend HTTP behavior.
- One Playwright FE-E2E suite for end-user flows.
- One shared Gherkin spec tree consumed by all unit / integration / E2E levels.
- One OpenAPI 3.1 contract describing every HTTP endpoint of `pdf-chat-be`.
- One C4 diagram set for the family.
- CI workflows: `test-pdf-chat-be.yml`, `test-pdf-chat-fe.yml`, `test-pdf-chat-be-e2e.yml`,
  `test-pdf-chat-fe-e2e.yml`.
- Root `package.json` script registrations: `dev:pdf-chat-be`, `dev:pdf-chat-fe`.
- `infra/dev/pdf-chat-be/docker-compose.integration.yml` with pgvector image.

Now in scope (additions to the original cut):

- Persistent chat sessions backed by Postgres tables — survives reload, restart, tab
  close.
- Multi-document conversations — a session attaches N PDFs; RAG retrieves across all of
  them.
- Production guardrails — per-IP rate limit, content filter (input + streamed output),
  per-session + per-day token cost cap.

Out:

- Polyglot backend ports (Java, Go, Rust, etc.).
- Auth, multi-tenancy. (Sessions are anonymous, addressable by UUID.)
- Distributed rate limiting / distributed cost accounting.
- Real moderation-provider integration (regex blocklist + swap point only).

## Prerequisite reading

Every persona below who **executes** or **reviews** this plan is expected to have
read the repo-wide AI primer first:

- [AI Application Development](../../../docs/explanation/software-engineering/ai-application-development/README.md)

The acceptance criteria, functional requirements, and Gherkin scenarios all assume
the vocabulary defined there (tokens, embeddings, RAG, streaming SSE, multi-provider
routing, persistent sessions, guardrails, eval, cost).

## Personas

- **Template consumer**: clones ose-primer to bootstrap a RAG/chat product; needs an
  end-to-end working reference and a clean OpenAPI contract.
- **End user (in the running demo)**: a curious developer who wants to ask questions of
  a PDF they own.
- **AI development agent (`plan-executor` / `swe-*-dev`)**: implements features by
  reading the contract, the Gherkin specs, and the delivery checklist; required to
  have read the AI primer.
- **CI system**: runs scheduled and dispatched workflows on the four new apps.

## User stories

- As a template consumer, I want a `pdf-chat-be` and `pdf-chat-fe` to clone or copy as
  a baseline, so I can ship a RAG product without re-deciding the stack.
- As an end user, I want to drop a PDF into the upload zone and immediately chat with
  it, so I can validate the demo in under 60 seconds from a clean clone.
- As an end user, I want to switch between Claude Haiku and Gemini Flash Lite from the
  UI, so I can compare answers across providers without restarting.
- As an end user, I want responses to stream token-by-token, so I see progress
  immediately rather than waiting for full completion.
- As an AI development agent, I want a single OpenAPI 3.1 contract under
  `specs/apps/pdf-chat/contracts/`, so my codegen targets are deterministic.
- As a maintainer, I want `nx affected -t test:quick` to pass on a clean checkout
  without any real OpenRouter call, so CI cost stays bounded.

## Functional requirements

> See [tech-docs.md](./tech-docs.md) for component-level design and request/response
> shapes. Endpoint paths are normative; payload shapes are normative; transport details
> live in the OpenAPI contract.

### FR-1 — Upload endpoint

`POST /api/v1/pdfs` accepts `multipart/form-data` with a single `file` field of
content-type `application/pdf`, max 25 MB. On success, returns `201 Created` with
`{ "pdfId": "<uuid>", "pages": <int>, "chunks": <int> }`. On invalid input, returns
`400` with the canonical error envelope.

### FR-2 — PDF processing pipeline

After upload, the backend extracts text per page using `pypdf`, splits each page into
overlapping chunks (default 800 tokens with 100-token overlap), embeds each chunk via
OpenRouter `/api/v1/embeddings` using model `openai/text-embedding-3-small`, and
inserts rows into `pdf_chunks(id, pdf_id, page, chunk_index, text, embedding vector(1536))`.
Pipeline is synchronous within the upload request.

### FR-3 — List uploaded PDFs

`GET /api/v1/pdfs` returns `{ "pdfs": [{ "pdfId": "<uuid>", "filename": "...",
"pages": <int>, "chunks": <int>, "uploadedAt": "<ISO-8601>" }] }`.

### FR-4 — Delete PDF

`DELETE /api/v1/pdfs/{pdfId}` removes the PDF and all its chunks. Returns `204`.
Returns `404` if the PDF does not exist.

### FR-5 — Sessions and persistent chat

`POST /api/v1/sessions` creates a chat session with body
`{ "title": "...", "pdfIds": ["<uuid>", ...], "model": "<openrouter-model-id>" }`.
Returns `201` with `{ "sessionId": "<uuid>", "title": "...", "pdfIds": [...],
"model": "...", "createdAt": "..." }`. At least one `pdfId` is required and every id
must reference an uploaded PDF or the request returns `400`.

`GET /api/v1/sessions` returns the list of sessions ordered by `createdAt desc`.
`GET /api/v1/sessions/{sessionId}` returns the session metadata plus the full message
history (oldest-first array of `{ "role", "content", "createdAt" }`).
`DELETE /api/v1/sessions/{sessionId}` removes the session and its messages (PDFs
remain).
`PATCH /api/v1/sessions/{sessionId}` accepts `{ "title"?, "model"?, "pdfIds"? }` for
in-place updates (e.g., adding/removing a PDF mid-conversation).

`POST /api/v1/sessions/{sessionId}/messages` accepts `{ "content": "..." }` and streams
a `text/event-stream` response. Each event is `data: {"delta": "<token-text>"}`,
terminated by `data: [DONE]`. The backend persists the user message **before** calling
OpenRouter and persists the assembled assistant message **after** the stream
completes. The full prior message history of the session is loaded as context (subject
to token budget).

### FR-5a — Multi-document RAG retrieval

For any chat call, the backend retrieves top-k=4 chunks via pgvector cosine-similarity
search **scoped to the union of all PDFs attached to the session**, not a single PDF.
Retrieval SQL joins `pdf_chunks` to a `session_pdfs(session_id, pdf_id)` link table.
The model id used is the value stored on the session (last `PATCH` wins); the request
body cannot override per-message.

### FR-5b — Production guardrails

Three guardrail layers run on every `POST /api/v1/sessions/{id}/messages`:

1. **Rate limit**: per-IP token-bucket via `slowapi`. Default
   `RATE_LIMIT_PER_MINUTE=20` over the chat endpoint, `60` over upload, `120` over
   reads. Exceeding returns `429` with the canonical `ErrorResponse` and a
   `Retry-After` header.
2. **Content filter**: a single `services/content_filter.py` runs over (a) the
   incoming user message and (b) the assembled assistant message before the SSE stream
   completes. Default implementation is a regex blocklist seeded from
   `tests/fixtures/blocklist.txt`. Disabled via `ENABLE_CONTENT_FILTER=false` for tests
   and demos. Filter hits return `422` (input) or terminate the stream with
   `data: {"error": "content_filter_blocked"}` followed by `data: [DONE]`. The
   service is structured so a real moderation provider (OpenRouter or Anthropic) can
   replace the regex implementation without touching callers.
3. **Token cost cap**: the backend tracks input + output tokens per session and per
   calendar day in a `token_usage(session_id, date, input_tokens, output_tokens)`
   table. Defaults `MAX_TOKENS_PER_SESSION=200000`, `MAX_TOKENS_PER_DAY=2000000`.
   Exceeding either returns `429` with `error.code = "token_budget_exceeded"`.
   Counters are read-modify-write inside a single SQL `UPDATE` (no Redis).

### FR-6 — Health endpoint

`GET /health` returns `{ "status": "ok" }` with `200`.

### FR-7 — Frontend upload UI

`pdf-chat-fe` ships a single page with:

- A drag-and-drop zone with click-to-browse fallback.
- Visible file size limit (25 MB) and accepted type (PDF).
- Upload progress state.
- After upload, navigation to `/` (home), where the PDF appears in the library
  and the user can create a session to start chatting.

### FR-8 — Frontend chat UI (session-scoped)

`pdf-chat-fe` `/chat/[sessionId]` renders:

- The session title (editable) and the list of attached PDFs with add/remove controls.
- A model selector toggling between **Claude Haiku 4.5** and **Gemini 2.5 Flash Lite**;
  selecting persists via `PATCH /api/v1/sessions/{id}`.
- A chat transcript hydrated from `GET /api/v1/sessions/{id}` on mount, with
  role-labeled bubbles and streaming-token rendering for new messages.
- A composer at the bottom with `Enter` to send, `Shift+Enter` for newline.
- A "back to home" link.
- A surfaced banner on `429` responses explaining whether the rate limit, daily token
  budget, or session token budget was exceeded, with a `Retry-After` countdown when
  applicable.
- A surfaced inline message on content-filter blocks ("This message was blocked by the
  content filter").

### FR-9 — Home page (library + sessions)

`pdf-chat-fe` `/` is split into two sections:

- **PDFs**: list of uploaded PDFs with filename, page count, and delete.
- **Sessions**: list of chat sessions with title, attached PDF count, last activity
  timestamp, and delete; a "New session" button opens a dialog to pick PDFs and a
  model, then creates the session and routes to `/chat/[sessionId]`.

### FR-10 — Route Handler proxy

The chat stream is consumed by the frontend through a Next.js Route Handler at
`/api/chat`, which forwards the SSE response from `pdf-chat-be` unchanged. The Route
Handler exists to keep the backend URL out of the browser and to let `useChat` consume
a same-origin endpoint.

### FR-11 — Backend env-var contract

The backend reads:

- `OPENROUTER_API_KEY` (required).
- `OPENROUTER_BASE_URL` (default `https://openrouter.ai/api/v1`).
- `DATABASE_URL` (required, Postgres with pgvector).
- `OPENROUTER_DEFAULT_MODEL` (default `anthropic/claude-haiku-4.5`).
- `OPENROUTER_EMBEDDING_MODEL` (default `openai/text-embedding-3-small`).
- `MAX_UPLOAD_MB` (default `25`).
- `RAG_TOP_K` (default `4`).
- `RATE_LIMIT_CHAT_PER_MINUTE` (default `20`).
- `RATE_LIMIT_UPLOAD_PER_MINUTE` (default `60`).
- `RATE_LIMIT_READ_PER_MINUTE` (default `120`).
- `ENABLE_CONTENT_FILTER` (default `true`; tests set `false`).
- `CONTENT_FILTER_BLOCKLIST_PATH` (default
  `tests/fixtures/blocklist.txt`).
- `MAX_TOKENS_PER_SESSION` (default `200000`).
- `MAX_TOKENS_PER_DAY` (default `2000000`).

`.env.example` ships with placeholders, never real keys.

### FR-12a — Shared UI library (ts-ui)

`pdf-chat-fe` consumes `@open-sharia-enterprise/ts-ui` (source: `libs/ts-ui/`) for all
UI primitives. `pdf-chat-fe/package.json` declares `@open-sharia-enterprise/ts-ui` and
`@open-sharia-enterprise/ts-ui-tokens` as dependencies. `globals.css` imports the token
sheet on its first line. No `Button`, `Input`, `Card`, `Label`, `Dialog`, or `Alert` is
re-implemented inside `pdf-chat-fe`. Demo-specific composites (UploadZone,
ChatTranscript, ChatComposer, ModelSelector) live in
`apps/pdf-chat-fe/src/components/` and compose `ts-ui` primitives. If a needed
primitive does not exist in `ts-ui`, it must be added to `libs/ts-ui` first via
`swe-ui-maker` and landed in its own commit before the consumer code references it.

### FR-12 — Mandatory Nx targets

Both `pdf-chat-be` and `pdf-chat-fe` ship the seven mandatory targets defined in
`governance/development/infra/nx-targets.md` plus `dev`/`start`. The two E2E projects
ship the targets defined in the existing `crud-be-e2e` and `crud-fe-e2e` patterns.

### FR-13 — Coverage thresholds

`pdf-chat-be` enforces ≥90% line coverage in `test:quick`. `pdf-chat-fe` enforces ≥70%.
Validation goes through `rhino-cli test-coverage validate`, identical to the CRUD apps.

### FR-14 — Backend testing (three levels)

`pdf-chat-be` tests at **three levels** with a single shared Gherkin set under
`specs/apps/pdf-chat/be/gherkin/`. Test framework: **pytest + pytest-bdd** at all
three levels. Lint: **ruff**. Typecheck: **pyright**. (Identical stack to
`crud-be-python-fastapi`.)

- `test:unit` — pytest + pytest-bdd; OpenRouter and pypdf both mocked; mocked
  in-memory repositories. Coverage measured here (≥90%).
- `test:integration` — pytest + pytest-bdd in docker-compose with real Postgres +
  pgvector; OpenRouter mocked via `pytest-httpx` cassette; same Gherkin features as
  `test:unit`, different step implementations.
- `test:e2e` — Playwright HTTP against running backend (driven by
  `pdf-chat-be-e2e`); OpenRouter routed through `MOCK_OPENROUTER=true` by default,
  optionally real OpenRouter on `workflow_dispatch`.

### FR-14a — Frontend testing (two levels)

`pdf-chat-fe` tests at **two levels** — there is no `test:integration` for the
frontend. Source language is **TypeScript** end-to-end (no JavaScript). Lint:
**oxlint** with `--jsx-a11y-plugin`. Typecheck: **`tsc --noEmit`**. (Identical
stack to `crud-fe-ts-nextjs`.)

- `test:unit` — vitest + @testing-library/react + jsdom; Route Handler proxy and
  generated-contracts client mocked at the fetch boundary. Coverage measured here
  (≥70%).
- `test:e2e` — Playwright + playwright-bdd against a running BE + FE pair (driven
  by `pdf-chat-fe-e2e`); BE booted with `MOCK_OPENROUTER=true` to avoid real LLM
  cost in CI.

Frontend integration concerns (real backend contract, real DB) are validated by
`pdf-chat-be:test:integration` plus the two e2e suites; replicating an
integration tier on the FE adds no signal.

### FR-14b — LLM test determinism

Every test target that runs on PR / on push must be **fully deterministic and
offline** despite calling LLM-shaped code paths. Concretely:

- All unit and integration tests run with `MOCK_OPENROUTER=true`. No outbound
  HTTPS request to `openrouter.ai` is made.
- Tests assert on **what the backend sends** (request fingerprint: model id,
  prompt content, retrieved chunks, message history) and on **side effects**
  (DB state, persisted messages, token-usage upserts, response status, SSE
  frame shape) — never on returned LLM prose.
- Snapshot tests are used for assembled prompts; diffs require deliberate
  human review.
- Embedding fixtures are deterministic vectors; retrieval similarity ordering
  is reproducible. Where pgvector ivfflat approximation could affect ordering,
  tests assert on the result **set** not the order, or force a sequential scan.
- `freezegun` pins `datetime.now()` for daily-cap tests. `tiktoken` is pinned
  in `pyproject.toml`.
- Real-LLM smoke runs only on `workflow_dispatch`; its assertions are
  structural only (status, Content-Type, ≥1 frame, well-formed envelope).

See [tech-docs.md § Test determinism strategy](./tech-docs.md#test-determinism-strategy-the-llm-problem)
for the four allowed assertion patterns and the one anti-pattern (asserting on
LLM output prose) that is forbidden in this plan's test set.

### FR-15 — Spec consumption

Every `pdf-chat-be` test target lists `specs/apps/pdf-chat/be/gherkin/**/*.feature` as
an Nx cache input. Same on the frontend with `specs/apps/pdf-chat/fe/gherkin/`.

### FR-16 — Contract codegen

`pdf-chat-be:codegen` runs `datamodel-codegen` against the bundled OpenAPI; output goes
to `apps/pdf-chat-be/generated_contracts/`. `pdf-chat-fe:codegen` runs
`@hey-api/openapi-ts` against the same bundle into
`apps/pdf-chat-fe/src/generated-contracts/`. The streaming endpoint is hand-typed
because OpenAPI 3.1 cannot fully describe SSE; the contract documents this with a
prose annotation.

### FR-17 — CI workflow files

Four workflows are added under `.github/workflows/`:

- `test-pdf-chat-be.yml`
- `test-pdf-chat-fe.yml`
- `test-pdf-chat-be-e2e.yml`
- `test-pdf-chat-fe-e2e.yml`

Each follows the existing `test-crud-*.yml` pattern (workflow_dispatch + scheduled,
runs `lint`, `typecheck`, `test:quick`, `spec-coverage`, plus E2E for the e2e
workflows).

### FR-18 — `MOCK_OPENROUTER` flag

When `MOCK_OPENROUTER=true`, the backend serves canned chat responses and embeddings
from a fixture file (`apps/pdf-chat-be/tests/fixtures/openrouter.json`). This keeps
unit, integration, and CI E2E free of real LLM cost. Production runs leave the flag
unset.

## Product risks

| Risk                                                                                                | Likelihood | Mitigation                                                                                            |
| --------------------------------------------------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------------------------------- |
| Streaming SSE breaks behind reverse proxies or in older browsers                                    | Medium     | Smoke-test in Chrome + Firefox; document the `X-Accel-Buffering: no` header in tech-docs              |
| Vector dimension mismatch (`text-embedding-3-small` produces 1536, schema hard-codes another value) | Low        | Schema explicitly types `vector(1536)`; integration test asserts insert succeeds                      |
| OpenRouter rate limiting in E2E test runs                                                           | Medium     | `MOCK_OPENROUTER=true` is the default in CI; real-API E2E is workflow_dispatch only                   |
| UI choice of model not propagated end-to-end                                                        | Low        | Gherkin scenarios assert the selected model id is what the backend forwards to OpenRouter             |
| Codegen for streaming endpoint generates wrong types                                                | Low        | tech-docs hand-typed shape; the OpenAPI streaming response has a `x-streaming: sse` extension comment |
| Plan author's port choices collide with existing `crud-*` ports                                     | Low        | 8501 / 3501 are checked against `apps/*/project.json`; no collision                                   |

## Acceptance criteria

```gherkin
Feature: pdf-chat demo family is added end-to-end

  Background:
    Given all delivery items in delivery.md are completed
    And `npm install` has been run on a clean checkout

  Scenario: Nx workspace recognises all four pdf-chat projects and the contracts project
    When I run "npx nx graph --file=/tmp/nx-graph-output.json"
    Then the graph contains projects "pdf-chat-be", "pdf-chat-fe", "pdf-chat-be-e2e", "pdf-chat-fe-e2e", and "pdf-chat-contracts"
    And no project named "pdf-chat-be" appears with empty targets

  Scenario: OpenAPI contract lints clean
    When I run "npx nx run pdf-chat-contracts:lint"
    Then exit code is 0
    And "specs/apps/pdf-chat/contracts/generated/openapi-bundled.yaml" exists

  Scenario: Backend codegen succeeds and produces Pydantic models
    When I run "npx nx run pdf-chat-be:codegen"
    Then exit code is 0
    And "apps/pdf-chat-be/generated_contracts/__init__.py" exists
    And the file contains a class "PdfUploadResponse"

  Scenario: Frontend codegen succeeds and produces TypeScript types
    When I run "npx nx run pdf-chat-fe:codegen"
    Then exit code is 0
    And the directory "apps/pdf-chat-fe/src/generated-contracts" exists

  Scenario: Backend test:quick passes with coverage gate
    When I run "npx nx run pdf-chat-be:test:quick"
    Then exit code is 0
    And the rhino-cli coverage validator reports >=90.00% line coverage

  Scenario: Frontend test:quick passes with coverage gate
    When I run "npx nx run pdf-chat-fe:test:quick"
    Then exit code is 0
    And the rhino-cli coverage validator reports >=70.00% line coverage

  Scenario: Backend integration tests bring up Postgres with pgvector
    Given the docker-compose.integration.yml uses a pgvector-enabled image
    When I run "npx nx run pdf-chat-be:test:integration"
    Then exit code is 0
    And every Gherkin scenario under specs/apps/pdf-chat/be/gherkin runs

  Scenario: BE E2E suite runs against a live backend
    Given the backend is started with MOCK_OPENROUTER=true
    When I run "npx nx run pdf-chat-be-e2e:test:e2e"
    Then exit code is 0

  Scenario: FE E2E suite uploads a PDF and chats with it
    Given the backend and frontend are both started with MOCK_OPENROUTER=true
    When I run "npx nx run pdf-chat-fe-e2e:test:e2e"
    Then exit code is 0
    And the test transcript contains a streamed assistant message

  Scenario: Spec coverage passes for both apps
    When I run "npx nx run pdf-chat-be:spec-coverage"
    And  I run "npx nx run pdf-chat-fe:spec-coverage"
    Then both invocations exit 0

  Scenario: Sessions persist chat history across server restarts
    Given a session with two prior message turns
    When the backend process is restarted
    And the frontend GETs "/api/v1/sessions/{id}"
    Then the response contains both prior turns in original order
    And every message has a non-null "createdAt"

  Scenario: Multi-document retrieval queries chunks across all attached PDFs
    Given a session attaches PDFs "manual-a.pdf" and "manual-b.pdf"
    And the recorded pgvector query is captured
    When the user posts a message
    Then the SQL retrieval filter is "WHERE pdf_id IN (:a, :b)"
    And the top-k result set contains chunks from both PDFs

  Scenario: PATCH a session adds a third PDF mid-conversation
    Given a session attaches one PDF
    When PATCH "/api/v1/sessions/{id}" sets pdfIds to three uuids
    Then GET "/api/v1/sessions/{id}" returns three pdfIds
    And the next chat call retrieves chunks across all three

  Scenario: Rate limit blocks excessive chat requests
    Given RATE_LIMIT_CHAT_PER_MINUTE=2
    When the same IP posts three messages within one minute
    Then the third response status is 429
    And the response has a "Retry-After" header
    And the response body's "error.code" is "rate_limit_exceeded"

  Scenario: Content filter rejects a blocklisted input message
    Given ENABLE_CONTENT_FILTER=true with the test blocklist loaded
    When the user posts a message containing a blocklisted token
    Then the response status is 422
    And the response body's "error.code" is "content_filter_blocked"
    And no OpenRouter request is made

  Scenario: Content filter terminates a streamed assistant message that violates the blocklist
    Given the OpenRouter mock cassette streams a blocklisted token
    When the frontend reads the SSE stream
    Then a frame "data: {\"error\":\"content_filter_blocked\"}" arrives before "data: [DONE]"
    And the persisted assistant message is recorded as "blocked"

  Scenario: Session token budget cap returns 429
    Given a session with token_usage already at MAX_TOKENS_PER_SESSION
    When the user posts a new message
    Then the response status is 429
    And the response body's "error.code" is "token_budget_exceeded"
    And the assistant message is not persisted

  Scenario: Daily token budget cap returns 429
    Given today's daily token_usage is already at MAX_TOKENS_PER_DAY
    When any session in the workspace posts a new message
    Then the response status is 429
    And the response body's "error.code" is "token_budget_exceeded"

  Scenario: Streaming chat endpoint emits incremental tokens
    Given the backend is running with MOCK_OPENROUTER=true and a known fixture
    When the frontend posts a message to "/api/chat" for an uploaded PDF
    Then the response Content-Type is "text/event-stream"
    And at least two "data:" frames arrive before "data: [DONE]"

  Scenario: Model selector switches the upstream OpenRouter model
    Given an uploaded PDF "rfc-2119.pdf"
    When the user selects "Gemini 2.5 Flash Lite" and submits a prompt
    Then the recorded OpenRouter request body contains "google/gemini-2.5-flash-lite"
    And does not contain "anthropic/claude-haiku-4.5"

  Scenario: Upload rejects oversized files
    When the user uploads a 30 MB file
    Then the response status is 413
    And the response body conforms to the canonical ErrorResponse schema

  Scenario: PDF deletion removes all chunks
    Given an uploaded PDF with 12 chunks
    When the user deletes the PDF via "DELETE /api/v1/pdfs/{pdfId}"
    Then the response status is 204
    And a database query for "SELECT count(*) FROM pdf_chunks WHERE pdf_id=:id" returns 0

  Scenario: Markdown quality gate passes
    When I run "npm run lint:md" at the repo root
    Then exit code is 0

  Scenario: Tests assert on outbound request, not on LLM output prose
    Given a unit test exercising the chat endpoint with MOCK_OPENROUTER=true
    When the test inspects the recorded outbound request to OpenRouter
    Then the assertion is on model id, prompt content, retrieved chunks, or message history
    And no assertion is on the prose of the streamed assistant response

  Scenario: Snapshot test catches prompt-assembly regressions
    Given a known session and a known retrieval result set
    When the chat handler assembles the prompt before calling OpenRouter
    Then the assembled messages array matches the committed snapshot byte-for-byte
    And changes require deliberate snapshot update with human review

  Scenario: Daily-cap test uses freezegun
    Given a test scenario advancing past midnight
    When freezegun freezes time to "2026-04-26T23:59:00Z" then "2026-04-27T00:00:00Z"
    Then the daily token_usage row for 2026-04-27 is fresh (zero)
    And the prior day's row is preserved

  Scenario: Retrieval test asserts on chunk-id set, not order, when ivfflat is in play
    Given five seeded chunks across two PDFs
    When integration retrieval runs through the ivfflat index
    Then the test asserts the result set contains the expected chunk ids
    And the test does not assert on a specific chunk order
    And a separate test forces enable_indexscan=off to validate the deterministic ordering

  Scenario: No real OpenRouter call happens during test:quick
    Given MOCK_OPENROUTER is unset by the developer
    When I run "npx nx run pdf-chat-be:test:quick"
    Then no outbound HTTPS connection to "openrouter.ai" is made
    And the test still exits 0

  Scenario: Four CI workflow files exist
    When I list ".github/workflows/"
    Then files exist named "test-pdf-chat-be.yml", "test-pdf-chat-fe.yml", "test-pdf-chat-be-e2e.yml", "test-pdf-chat-fe-e2e.yml"
    And each file references the correct Nx project name

  Scenario: pdf-chat-be is reachable on its assigned port
    Given the backend is started with "npx nx run pdf-chat-be:dev"
    When I curl "http://localhost:8501/health"
    Then the response status is 200
    And the response body equals "{\"status\":\"ok\"}"

  Scenario: pdf-chat-fe is reachable on its assigned port
    Given the frontend is started with "npx nx run pdf-chat-fe:dev"
    When I open "http://localhost:3501/" in a browser
    Then the page renders the upload zone

  Scenario: pdf-chat-fe consumes ts-ui as its UI base
    Given pdf-chat-fe/package.json is staged
    Then dependencies include "@open-sharia-enterprise/ts-ui"
    And dependencies include "@open-sharia-enterprise/ts-ui-tokens"
    And no source file under apps/pdf-chat-fe/src defines a local "Button", "Input", "Card", "Label", "Dialog", or "Alert" component
    And at least one source file imports from "@open-sharia-enterprise/ts-ui"

  Scenario: Nx graph shows the ts-ui edge
    When I run "npx nx graph --file=/tmp/nx-graph-output.json"
    Then "pdf-chat-fe" depends on "ts-ui"
    And "pdf-chat-fe" depends on "ts-ui-tokens"

  Scenario: Affected gate captures the new apps
    Given a one-line edit in apps/pdf-chat-be/src/main.py
    When I run "npx nx affected -t typecheck lint test:quick"
    Then "pdf-chat-be" appears in the affected list
    And exit code is 0
```
