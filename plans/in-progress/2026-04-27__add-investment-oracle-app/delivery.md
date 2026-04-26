# Delivery Checklist: Add `pdf-chat-*` Demo App Family

> Conventions: tick each `- [ ]` to `- [x]` only after the action passes locally. Each
> phase ends with a green workspace state — `git status` clean (intentional changes
> staged), `nx affected -t typecheck lint` exits 0 for everything previously
> implemented. Commits land directly on `main` (Trunk Based Development); intermediate
> commits per phase are encouraged.

## Phase 0 — Environment setup

- [ ] **Required prerequisite reading** — read end-to-end before any other Phase 0
      checkbox: [AI Application Development](../../../docs/explanation/software-engineering/ai-application-development/README.md).
      The rest of this plan assumes its vocabulary (tokens, embeddings, RAG,
      streaming SSE, multi-provider routing, persistent sessions, guardrails, eval,
      cost). If you can't define each glossary term unprompted, do not proceed —
      you will misimplement subtle behaviour later.
- [ ] Read [tech-docs.md](./tech-docs.md) end-to-end (the AI primer is its
      pre-requisite; this document is its application to this plan)
- [ ] `git status` is clean before starting
- [ ] `npm install` succeeds
- [ ] `npm run doctor -- --fix` reports all tools OK
- [ ] Establish baseline: `npx nx affected -t typecheck lint test:quick` passes (note
      any pre-existing failures — must not be made worse by this plan)
- [ ] Confirm ports 8501 and 3501 are free on the dev machine
- [ ] Capture an `OPENROUTER_API_KEY` for local manual smoke testing (NOT committed
      anywhere; only used to run real-API E2E once)

## Phase 1 — Shared spec area scaffolding

- [ ] Create directory tree:
      `mkdir -p specs/apps/pdf-chat/{c4,be/gherkin,fe/gherkin,contracts/{paths,schemas,examples}}`
- [ ] Write `specs/apps/pdf-chat/README.md` (mirror `specs/apps/crud/README.md`,
      adjusted for the family — domains: health, pdfs, chat, test-support)
- [ ] Write `specs/apps/pdf-chat/be/README.md` (mirror `specs/apps/crud/be/README.md`)
- [ ] Write `specs/apps/pdf-chat/fe/README.md` (mirror `specs/apps/crud/fe/README.md`)
- [ ] Write `specs/apps/pdf-chat/be/gherkin/README.md` listing the BE domains
- [ ] Write `specs/apps/pdf-chat/fe/gherkin/README.md` listing the FE domains

## Phase 2 — C4 diagrams

- [ ] `specs/apps/pdf-chat/c4/README.md` — index + palette
- [ ] `specs/apps/pdf-chat/c4/context.md` — actors: End User, Operations Engineer; one
      external system (OpenRouter)
- [ ] `specs/apps/pdf-chat/c4/container.md` — Next.js, FastAPI, Postgres+pgvector,
      OpenRouter
- [ ] `specs/apps/pdf-chat/c4/component-be.md` — routers, services, infrastructure
      layers
- [ ] `specs/apps/pdf-chat/c4/component-fe.md` — pages, ts-ui-based composites,
      Route Handler proxy
- [ ] All diagrams pass `npm run lint:md` and use the accessible Mermaid palette

## Phase 3 — OpenAPI contract (`pdf-chat-contracts`)

- [ ] Write `specs/apps/pdf-chat/contracts/openapi.yaml` (root with `$ref`s into
      `paths/` and `schemas/`)
- [ ] Write `paths/health.yaml`, `paths/pdfs.yaml`, `paths/chat.yaml`
- [ ] Write `schemas/pdf.yaml` (Pdf, PdfListResponse, PdfUploadResponse),
      `schemas/chat.yaml` (ChatMessage, ChatRequest, ChatStreamFrame),
      `schemas/error.yaml` (ErrorResponse), `schemas/health.yaml`
- [ ] Annotate the streaming response with the prose limitation note from tech-docs.md
- [ ] Write `.spectral.yaml` (camelCase rule, description-required rule)
- [ ] Write `redocly.yaml`
- [ ] Write `specs/apps/pdf-chat/contracts/project.json` with `lint`, `bundle`, `docs`
      targets (mirror `crud-contracts`)
- [ ] Write `specs/apps/pdf-chat/contracts/README.md`
- [ ] `npx nx run pdf-chat-contracts:lint` exits 0
- [ ] `npx nx run pdf-chat-contracts:bundle` produces `generated/openapi-bundled.{yaml,json}`

## Phase 4 — Gherkin scenarios (BE)

- [ ] `specs/apps/pdf-chat/be/gherkin/health/health-check.feature` (1 scenario)
- [ ] `specs/apps/pdf-chat/be/gherkin/pdfs/upload.feature` (~5 — happy, oversize, wrong
      type, empty, malformed)
- [ ] `pdfs/list.feature` (~3)
- [ ] `pdfs/delete.feature` (~3 — happy, 404, cascade chunks)
- [ ] `sessions/lifecycle.feature` (~5 — create, list, get, patch model, delete)
- [ ] `sessions/multi-pdf.feature` (~3 — attach two, attach three via patch, retrieval
      union)
- [ ] `sessions/persistence.feature` (~3 — restart preserves history; mid-stream crash;
      cascade on delete)
- [ ] `chat/streaming.feature` (~4 — SSE shape, [DONE] sentinel, mid-stream error)
- [ ] `chat/rag-retrieval.feature` (~3 — top-k, empty index, similarity ordering)
- [ ] `chat/model-selection.feature` (~3 — Haiku, Gemini, default-from-session)
- [ ] `guardrails/rate-limit.feature` (~3 — chat over limit, upload over limit, headers)
- [ ] `guardrails/content-filter.feature` (~3 — input block, output block, disabled
      flag noop)
- [ ] `guardrails/cost-cap.feature` (~3 — session cap, day cap, usage upsert)
- [ ] `test-support/test-api.feature` (~2 — db reset gated by env flag)
- [ ] All `.feature` files lint clean

## Phase 5 — Gherkin scenarios (FE)

- [ ] `specs/apps/pdf-chat/fe/gherkin/library/library-list.feature` (~3)
- [ ] `library/delete-pdf.feature` (~2)
- [ ] `sessions/sessions-list.feature` (~3 — list, last activity, delete)
- [ ] `sessions/new-session-dialog.feature` (~3 — pick PDFs, pick model, validation)
- [ ] `sessions/manage-attached-pdfs.feature` (~2 — add, remove)
- [ ] `upload/drag-drop.feature` (~3)
- [ ] `upload/validation.feature` (~3)
- [ ] `chat/chat-flow.feature` (~4 — load history on mount; persists on send)
- [ ] `chat/model-toggle.feature` (~2)
- [ ] `chat/streaming-display.feature` (~2)
- [ ] `guardrails/rate-limit-banner.feature` (~2)
- [ ] `guardrails/content-filter-feedback.feature` (~2)
- [ ] `guardrails/cost-cap-banner.feature` (~2)
- [ ] All `.feature` files lint clean

## Phase 6 — Backend project scaffold (`pdf-chat-be`)

- [ ] Create `apps/pdf-chat-be/` directory tree per tech-docs.md repository layout
- [ ] Write `pyproject.toml` runtime deps: `fastapi[standard]`, `uvicorn[standard]`,
      `sqlalchemy`, `alembic`, `psycopg2-binary`, `pgvector`, `pydantic[email]`,
      `pydantic-settings`, `python-multipart`, `pypdf`, `pdfplumber`, `httpx`,
      `sse-starlette`, `tiktoken`, `slowapi`
  - [ ] Add test stack to `pyproject.toml`: `pytest`, `pytest-bdd`, `pytest-asyncio`,
        `pytest-httpx`, `coverage[toml]`, `freezegun` (for daily-cap time-travel)
  - [ ] Add lint (`ruff`) and typecheck (`pyright`) to `pyproject.toml`
  - Note: this stack is identical to `crud-be-python-fastapi` plus the AI-specific
    deps (`sse-starlette`, `tiktoken`, `slowapi`, `pypdf`, `pdfplumber`, `pgvector`)
- [ ] Write `.python-version` (3.13)
- [ ] Write `apps/pdf-chat-be/project.json` with all mandatory targets (mirror
      `crud-be-python-fastapi`, with `cwd: apps/pdf-chat-be`, port 8501, coverage
      threshold 90)
- [ ] Write `Dockerfile.integration`
- [ ] Write `docker-compose.integration.yml` using image
      `pgvector/pgvector:pg16` (NOT vanilla `postgres:16`)
- [ ] Write `apps/pdf-chat-be/README.md`
- [ ] Add `.env.example` with all FR-11 vars
- [ ] `uv sync` succeeds inside `apps/pdf-chat-be/`

## Phase 7 — Backend codegen + Alembic migration

- [ ] Add `tags` and `implicitDependencies: ["pdf-chat-contracts", "rhino-cli"]` to
      `apps/pdf-chat-be/project.json`
- [ ] `npx nx run pdf-chat-be:codegen` produces
      `apps/pdf-chat-be/generated_contracts/__init__.py` with Pydantic v2 models
- [ ] Initialise Alembic: `alembic init alembic` from inside `apps/pdf-chat-be/`
- [ ] Author migration `0001_initial_schema.py` per tech-docs.md schema, creating in
      one revision: `pdfs`, `pdf_chunks` (with ivfflat index), `sessions`,
      `session_pdfs`, `messages`, `token_usage`, plus
      `CREATE EXTENSION IF NOT EXISTS vector;`
- [ ] `alembic upgrade head` succeeds against the docker-compose pgvector image

## Phase 8 — Backend implementation

- [ ] `src/pdf_chat_be/config.py` — pydantic-settings per tech-docs
- [ ] `src/pdf_chat_be/main.py` — FastAPI app, mount routers, CORS middleware,
      slowapi limiter, exception handlers for `RateLimitExceeded`,
      `ContentFilterBlocked`, `TokenBudgetExceeded` — each rewritten to canonical
      `ErrorResponse`
- [ ] `routers/health.py`
- [ ] `routers/pdfs.py` — POST upload, GET list, DELETE; rate-limited
- [ ] `routers/sessions.py` — POST/GET/PATCH/DELETE sessions, POST messages (SSE);
      rate-limited
- [ ] `services/pdf_extraction.py` — pypdf wrapper
- [ ] `services/chunking.py` — token-window chunker via `tiktoken`
- [ ] `services/embedding.py` — calls OpenRouter or mock based on
      `settings.mock_openrouter`
- [ ] `services/vector_store.py` — pgvector cosine top-k via SQLAlchemy, scoped by
      `session_pdfs`
- [ ] `services/rag.py` — retrieve + prompt assembly, history-aware
- [ ] `services/openrouter_chat.py` — streaming generator (mock-aware)
- [ ] `services/sessions_service.py` — session lifecycle, message persistence
- [ ] `services/rate_limiter.py` — slowapi `Limiter` + per-route decorators
- [ ] `services/content_filter.py` — `ContentFilter` Protocol +
      `RegexBlocklistFilter` + `NoopFilter`; reads blocklist file once on startup
- [ ] `services/cost_cap.py` — session + day budget assertions + UPSERT recorder
- [ ] `services/token_counter.py` — `tiktoken` for input, char-approx for output
- [ ] `infrastructure/repositories.py` — async SQLAlchemy session
- [ ] `infrastructure/openrouter_client.py` — httpx wrapper
- [ ] All modules type-clean: `npx nx run pdf-chat-be:typecheck` exits 0
- [ ] All modules lint-clean: `npx nx run pdf-chat-be:lint` exits 0

## Phase 9 — Backend tests (unit, integration)

- [ ] `tests/unit/steps/` step defs (pytest-bdd) consuming
      `specs/apps/pdf-chat/be/gherkin/**/*.feature`, including
      `sessions/`, `chat/`, and `guardrails/` domains
- [ ] Unit fixtures mock OpenRouter via `pytest-httpx` and pypdf via patch
- [ ] `tests/integration/steps/` consume the same features against real
      Postgres+pgvector via docker-compose; OpenRouter still mocked
- [ ] `tests/fixtures/sample.pdf` (small public-domain PDF, e.g., RFC 2119 text)
- [ ] `tests/fixtures/sample-b.pdf` (second small PDF, used for multi-PDF tests)
- [ ] `tests/fixtures/openrouter.json` cassette per tech-docs
- [ ] `tests/fixtures/blocklist.txt` (test-only patterns; never seeds prod)
- [ ] Daily-cap tests use `freezegun` to advance virtual `usage_date`
- [ ] Rate-limit tests reset slowapi's in-memory store between scenarios
- [ ] `npx nx run pdf-chat-be:test:unit` exits 0
- [ ] `npx nx run pdf-chat-be:test:quick` reports ≥90% line coverage
- [ ] `npx nx run pdf-chat-be:test:integration` exits 0

## Phase 10 — Frontend project scaffold (`pdf-chat-fe`)

- [ ] Create `apps/pdf-chat-fe/` directory tree per tech-docs
- [ ] `package.json` deps: `next@^16`, `react@^19`, `react-dom@^19`,
      `ai@^5`, `@ai-sdk/react@^3` (v3.x ships aligned with `ai@^5`; verify
      `npm info @ai-sdk/react versions` if install fails with peer-dep conflict),
      `@open-sharia-enterprise/ts-ui: "*"`,
      `@open-sharia-enterprise/ts-ui-tokens: "*"`, `@tailwindcss/postcss`,
      `tailwindcss`. Dev deps mirror `crud-fe-ts-nextjs`
- [ ] `next.config.ts`, `tsconfig.json`, `oxlint.json` mirror `crud-fe-ts-nextjs`
- [ ] `vitest.config.ts` mirror `crud-fe-ts-nextjs`
- [ ] `apps/pdf-chat-fe/project.json` with mandatory targets, port 3501, coverage 70
- [ ] `apps/pdf-chat-fe/README.md`
- [ ] `apps/pdf-chat-fe/.env.example`
- [ ] `npm install` resolves the workspace links to ts-ui

## Phase 11 — Frontend ts-ui wiring + globals

- [ ] `src/app/globals.css` imports `@open-sharia-enterprise/ts-ui-tokens/src/tokens.css`
      on the first line
- [ ] `src/app/layout.tsx` includes the global CSS and renders `<html lang="en">`
- [ ] Verify Nx graph: `npx nx graph` shows `pdf-chat-fe → ts-ui` and
      `pdf-chat-fe → ts-ui-tokens`
- [ ] Spot-check: searching `apps/pdf-chat-fe/src` for `function Button|function Input|function Card|function Label|function Dialog|function Alert` returns zero matches (ban on local primitives)

## Phase 12 — Frontend implementation (composites + pages)

- [ ] `src/components/UploadZone.tsx` — composes `Card`, `Button`, `Alert` from ts-ui
- [ ] `src/components/ChatTranscript.tsx` — composes `Card`; hydrates from session
      detail; renders streaming-token deltas appended to the last message
- [ ] `src/components/ChatComposer.tsx` — composes `Input`, `Button`
- [ ] `src/components/ModelSelector.tsx` — composes `Button`, `Label`; PATCHes the
      session on change
- [ ] `src/components/SessionList.tsx` — composes `Card`, `Button`
- [ ] `src/components/NewSessionDialog.tsx` — composes `Dialog`, `Button`, `Label`,
      `Input`; multi-select PDFs + model
- [ ] `src/components/AttachedPdfManager.tsx` — composes `Card`, `Button`; add/remove
      PDFs from a session via PATCH
- [ ] `src/components/GuardrailBanner.tsx` — composes `Alert`; renders 429 / 422 /
      stream-embedded error frames with a `Retry-After` countdown
- [ ] `src/app/page.tsx` — Home: PDFs section + Sessions section, "New session" button
- [ ] `src/app/chat/[sessionId]/page.tsx` — session-scoped chat page; loads message
      history on mount; wires `useChat` to `/api/chat` with `sessionId` in body
- [ ] `src/app/api/chat/route.ts` — Route Handler SSE proxy per tech-docs (forwards to
      `POST /api/v1/sessions/{id}/messages`, propagates `429`/`422` and `Retry-After`)
- [ ] `src/lib/api.ts` — typed client wrapping fetch over generated-contracts
      (sessions CRUD + post-message)
- [ ] `src/lib/env.ts` — read `PDF_CHAT_BE_URL`, `NEXT_PUBLIC_DEFAULT_MODEL`
- [ ] `npx nx run pdf-chat-fe:codegen` exits 0
- [ ] `npx nx run pdf-chat-fe:typecheck` exits 0
- [ ] `npx nx run pdf-chat-fe:lint` exits 0

## Phase 13 — Frontend tests (unit only — no integration level)

> The frontend ships **two** test levels: `test:unit` (this phase) and `test:e2e`
> (Phase 15). There is no `pdf-chat-fe:test:integration` target — integration
> concerns are covered by `pdf-chat-be:test:integration` plus the two e2e suites.

- [ ] Component unit tests for each composite (vitest + @testing-library/react)
- [ ] Optional vitest-cucumber wiring of `specs/apps/pdf-chat/fe/gherkin` if FE
      Gherkin coverage is required by `spec-coverage`
- [ ] Mock the Route Handler / fetch boundary in component tests; do not hit FastAPI
- [ ] All FE source files are `.ts` / `.tsx` (TypeScript end-to-end; no `.js` /
      `.jsx` in `apps/pdf-chat-fe/src/`)
- [ ] `npx nx run pdf-chat-fe:test:quick` exits 0 with ≥70% coverage
- [ ] If a primitive is missing in ts-ui and you found yourself reaching for one: add
      it to `libs/ts-ui` first via `swe-ui-maker`, land that change in a separate
      commit, then proceed (do **not** inline the primitive here)

## Phase 14 — BE E2E (`pdf-chat-be-e2e`)

- [ ] Scaffold `apps/pdf-chat-be-e2e/` mirroring `crud-be-e2e/`
- [ ] `playwright.config.ts` boots `pdf-chat-be` with `MOCK_OPENROUTER=true` via
      `webServer`
- [ ] BDD glue under `tests/steps/` consumes `specs/apps/pdf-chat/be/gherkin/`
- [ ] Add `apps/pdf-chat-be-e2e/project.json` mandatory targets
- [ ] `implicitDependencies: ["pdf-chat-be", "pdf-chat-contracts"]`
- [ ] `npx nx run pdf-chat-be-e2e:test:e2e` exits 0
- [ ] `npx nx run pdf-chat-be-e2e:spec-coverage` exits 0

## Phase 15 — FE E2E (`pdf-chat-fe-e2e`)

- [ ] Scaffold `apps/pdf-chat-fe-e2e/` mirroring `crud-fe-e2e/`
- [ ] `playwright.config.ts` boots BOTH `pdf-chat-be` (with `MOCK_OPENROUTER=true`)
      and `pdf-chat-fe` via `webServer`
- [ ] BDD glue under `tests/steps/` consumes `specs/apps/pdf-chat/fe/gherkin/`
- [ ] Add `apps/pdf-chat-fe-e2e/project.json` mandatory targets
- [ ] `implicitDependencies: ["pdf-chat-fe", "pdf-chat-be", "pdf-chat-contracts"]`
- [ ] Tests use a fixture PDF and assert that streamed tokens render in the DOM
- [ ] `npx nx run pdf-chat-fe-e2e:test:e2e` exits 0
- [ ] `npx nx run pdf-chat-fe-e2e:spec-coverage` exits 0

## Phase 16 — Root npm scripts + workspace metadata

- [ ] Root `package.json` add `dev:pdf-chat-be` and `dev:pdf-chat-fe` scripts
- [ ] Root `README.md` add the new family to the "Demos" section
- [ ] `CLAUDE.md` Tech Stack section updated to list four new apps
- [ ] `CLAUDE.md` coverage threshold table includes `pdf-chat-be` (≥90%) and
      `pdf-chat-fe` (≥70%)
- [ ] `AGENTS.md` updated if the doc enumerates demo apps

## Phase 17 — CI workflow files

- [ ] Author `.github/workflows/test-pdf-chat-be.yml` mirroring
      `test-crud-be-python-fastapi.yml`
- [ ] Author `.github/workflows/test-pdf-chat-fe.yml` mirroring
      `test-crud-fe-ts-nextjs.yml`
- [ ] Author `.github/workflows/test-pdf-chat-be-e2e.yml` mirroring
      backend-e2e reusable; pass `MOCK_OPENROUTER=true`
- [ ] Author `.github/workflows/test-pdf-chat-fe-e2e.yml` mirroring
      frontend-e2e reusable; pass `MOCK_OPENROUTER=true`
- [ ] All four workflows reference the correct Nx project names
- [ ] All four workflows include `workflow_dispatch` and the same scheduled cron used
      by the crud workflows

## Phase 18 — `.claude/` and `.opencode/` sync

- [ ] No new agents required for this plan (existing `swe-python-dev`,
      `swe-typescript-dev`, `swe-e2e-dev`, `swe-ui-maker` cover the scope)
- [ ] `npm run sync:claude-to-opencode` exits 0
- [ ] Verify no skill or agent references stale `crud-` paths after the sync

## Phase 19 — Final stale-reference and naming audits

- [ ] `grep -r "demo-be-\|demo-fe-\|demo-fs-" apps/pdf-chat-* specs/apps/pdf-chat`
      returns zero matches
- [ ] `grep -r "PyMuPDF\|fitz\|pymupdf" apps/pdf-chat-be` returns zero matches
- [ ] `grep -rE "(function|const) (Button|Input|Card|Label|Dialog|Alert)" apps/pdf-chat-fe/src`
      returns zero matches (ts-ui consumption guard)
- [ ] `grep -rE "@radix-ui/[a-z]" apps/pdf-chat-fe/src` returns zero matches (must
      go through ts-ui's `radix-ui` unified import)
- [ ] `grep -r "anthropic/claude-haiku-4-5\|anthropic/claude-sonnet-4-6" apps/pdf-chat-*`
      returns zero matches (model ids use dot, not hyphen)
- [ ] No `.env` files with real keys are staged: `git status | grep "\.env"` shows
      only `.env.example`
- [ ] `grep -rE "rate_limit|content_filter|cost_cap" specs/apps/pdf-chat/be/gherkin/`
      returns matches in the `guardrails/` domain (sanity)
- [ ] `grep -rE "/api/v1/pdfs/[^/]+/chat" apps/pdf-chat-* specs/apps/pdf-chat`
      returns zero matches (the per-PDF chat endpoint was superseded by sessions)
- [ ] `grep -rE "@router.*pdfs/.*chat" apps/pdf-chat-be/src` returns zero matches
- [ ] `npm run lint:md` exits 0

## Phase 20 — Workspace validation

- [ ] `npm install` from clean state succeeds
- [ ] `npx nx graph --file=/tmp/nx-graph-output.json` includes
      `pdf-chat-be`, `pdf-chat-fe`, `pdf-chat-be-e2e`, `pdf-chat-fe-e2e`,
      `pdf-chat-contracts`
- [ ] `npx nx affected -t codegen` regenerates the new contracts and re-runs
      dependents

## Phase 21 — Quality gates

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes. This follows the root cause orientation principle — proactively fix preexisting
> errors encountered during work. Do not defer or mention-and-skip existing issues.

- [ ] `npx nx affected -t typecheck` exits 0
- [ ] `npx nx affected -t lint` exits 0
- [ ] `npx nx affected -t test:quick` exits 0 with all coverage thresholds met
- [ ] `npx nx run pdf-chat-be:test:integration` exits 0
- [ ] `npx nx run pdf-chat-be-e2e:test:e2e` exits 0
- [ ] `npx nx run pdf-chat-fe-e2e:test:e2e` exits 0
- [ ] `npx nx affected -t spec-coverage` exits 0
- [ ] `npx nx run rhino-cli:test:quick` exits 0 (template tooling regression)

## Phase 22 — Manual smoke

- [ ] Terminal A: `npm run dev:pdf-chat-be` (port 8501)
- [ ] Terminal B: `npm run dev:pdf-chat-fe` (port 3501)
- [ ] Browser: open `http://localhost:3501/`
- [ ] Upload `tests/fixtures/sample.pdf` and `tests/fixtures/sample-b.pdf`
- [ ] Click "New session"; attach **both** PDFs; pick Claude Haiku 4.5; submit
- [ ] Verify navigation to `/chat/<sessionId>` and chat composer renders ts-ui
      Button + Input
- [ ] Send a question that should retrieve from PDF B; verify streaming tokens appear
      incrementally and cite content from PDF B
- [ ] Hard-refresh the page; verify the prior turn re-renders from persisted history
- [ ] Toggle model to Gemini 2.5 Flash Lite; resend a question; verify the recorded
      OpenRouter request body contains `google/gemini-2.5-flash-lite`
- [ ] Set `RATE_LIMIT_CHAT_PER_MINUTE=1` in BE env; spam-send two messages; confirm
      the GuardrailBanner explains the 429 and shows a `Retry-After` countdown
- [ ] Set `MAX_TOKENS_PER_SESSION=1`; send a message; confirm
      `error.code = "token_budget_exceeded"` surfaces in the banner
- [ ] Set `ENABLE_CONTENT_FILTER=true` and add a known phrase to the blocklist; send
      it; confirm the inline content-filter feedback renders without persisting an
      assistant message
- [ ] Delete the session; home page reflects the change; PDFs remain
- [ ] Delete a PDF; home page reflects the change; sessions referencing that PDF
      handle the cascade gracefully (404 on the missing PDF, banner explains)

## Phase 23 — Markdown quality gate

- [ ] `npm run lint:md` exits 0
- [ ] `npm run lint:md:fix` if needed; re-run until clean

## Phase 24 — Commit and push

- [ ] Review `git status` — no unintended files staged
- [ ] Suggested split (Conventional Commits):
  - `feat(specs): add specs/apps/pdf-chat tree (c4, gherkin, contracts)`
  - `feat(apps): add pdf-chat-be Python/FastAPI demo backend with sessions, multi-pdf RAG, guardrails`
  - `feat(apps): add pdf-chat-fe Next.js 16 demo consuming ts-ui`
  - `feat(apps): add pdf-chat-be-e2e and pdf-chat-fe-e2e Playwright suites`
  - `ci(workflows): add test-pdf-chat-* GitHub Actions workflows`
  - `docs(claude): document pdf-chat demo family in CLAUDE.md`
- [ ] `git pull --rebase origin main` (skip if already up to date)
- [ ] `git push origin main`

### Post-push CI monitoring

- [ ] On GitHub Actions, manually dispatch each `test-pdf-chat-*.yml` workflow once;
      verify all four pass
- [ ] If any workflow fails: fix root cause, push follow-up commit, do not move to
      Phase 25 until green

## Phase 25 — Plan archival

- [ ] `git mv plans/in-progress/2026-04-26__add-pdf-chat-apps plans/done/2026-04-26__add-pdf-chat-apps`
- [ ] Update `plans/in-progress/README.md` — remove this plan's entry
- [ ] Update `plans/done/README.md` — add this plan's entry with completion date
- [ ] Commit: `chore(plans): move add-pdf-chat-apps to done`
