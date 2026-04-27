# Delivery Checklist: `investment-oracle`

Step-by-step phases. Each phase ends in a verifiable artifact. Tick items
in order; push direct to `main` per Trunk Based Development
([git-push-default](../../../governance/development/workflow/git-push-default.md)).

## Phase pre-0 — Manual prerequisites

Steps that live outside this repository and must complete before Phase 0
can begin. Each item is verifiable; see
[plan README → Manual prerequisites](./README.md#manual-prerequisites-before-phase-0)
for the rationale.

### Vendor accounts and API keys

- [ ] Create an Anthropic Console account at <https://console.anthropic.com>
      and generate an API key
- [ ] Create a Google AI Studio account at <https://aistudio.google.com>
      and generate a Gemini API key
- [ ] Create a Perplexity API key at <https://www.perplexity.ai/settings/api>
- [ ] Add billing details / payment method per vendor (free tiers cover
      demo use; production will hit pay-as-you-go)
- [ ] Verify each key by hitting the corresponding `/health` or models
      endpoint:
      `curl -sH "x-api-key: $ANTHROPIC_API_KEY" -H "anthropic-version: 2023-06-01" https://api.anthropic.com/v1/models | jq .data[0].id` ;
      `curl -s "https://generativelanguage.googleapis.com/v1beta/models?key=$GOOGLE_API_KEY" | jq .models[0].name` ;
      `curl -sH "Authorization: Bearer $PERPLEXITY_API_KEY" -d '{"model":"sonar","messages":[{"role":"user","content":"ping"}]}' -H "Content-Type: application/json" https://api.perplexity.ai/chat/completions | jq .choices[0].message.content`

### Platform tooling (manual; cannot be auto-installed)

- [ ] Install Volta: `curl https://get.volta.sh | bash`
- [ ] Install Docker Desktop / OrbStack / Colima; verify `docker info`
      exits 0
- [ ] **macOS**: install Xcode Command Line Tools — `xcode-select --install`
- [ ] **Windows**: install Visual Studio Build Tools (Desktop dev with C++)
- [ ] **Linux**: `sudo apt install build-essential libwebkit2gtk-4.1-dev libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`
- [ ] Install Rust toolchain — `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` (or use the team's rustup-init); verify `rustc --version` ≥ 1.80
- [ ] Install Python 3.13 via pyenv (or distro package); verify
      `python3.13 --version`
- [ ] Install `ruff` and `pyright` (Phase 0a `doctor --fix` will also
      handle these, but verify availability up front)

### Hardware

- [ ] Verify ≥ 16 GB RAM (`vm_stat` / `free -g` / Task Manager)
- [ ] Verify ≥ 20 GB free disk on the partition holding the repo

### Network egress

- [ ] Verify outbound HTTPS to:
      `api.anthropic.com`, `generativelanguage.googleapis.com`,
      `api.perplexity.ai`, `registry.npmjs.org`, `pypi.org`,
      `crates.io`, `docker.io`, `github.com`
      via `curl -sI -o /dev/null -w "%{http_code}\n" https://<host>` for
      each (any 200/301/302/403 is fine — 0 / DNS-fail / timeout fails)

### Optional — Indonesian residency (`bedrock-jakarta-*`)

- [ ] AWS account ready with Bedrock access enabled in
      `ap-southeast-3` region
- [ ] Bedrock model access requested + approved for Claude Opus 4.7
      (in-region) and / or CRIS-eligible Claude models
- [ ] AWS access key or IAM role with `bedrock:InvokeModel` and
      `bedrock:InvokeModelWithResponseStream`

### Optional — Production deployment to Indonesian users

- [ ] Komdigi PSE Private Scope registration filed (PP 71/2019)
- [ ] UU PDP Article 56 cross-border transfer template (SCC) signed
      with each vendor, OR explicit-consent UI flow drafted
- [ ] DPIA covering Anthropic + Google + Perplexity (when
      `WEB_GROUNDING_ENABLED=true`) authored

## Phase 0 — Prerequisite reading

- [ ] Read [AI Application Development primer](../../../docs/explanation/software-engineering/ai-application-development/README.md)
- [ ] Read [Anthropic API Primer](../../../docs/explanation/software-engineering/ai-application-development/anthropic-api.md)
- [ ] Read [Google Gemini API Primer](../../../docs/explanation/software-engineering/ai-application-development/google-gemini-api.md)
- [ ] Read [OpenAI API Primer](../../../docs/explanation/software-engineering/ai-application-development/openai-api.md)
      (boundary framing only; not used in this demo)
- [ ] Read [Perplexity Sonar API Primer](../../../docs/explanation/software-engineering/ai-application-development/perplexity-api.md)
      (boundary framing only; not used in this demo)
- [ ] Read [Testing AI Applications](../../../docs/explanation/software-engineering/ai-application-development/testing-ai-apps.md)
      (cross-cutting testing playbook — directly implemented by PRD FR-15 family)
- [ ] Read this plan's [README](./README.md), [BRD](./brd.md), [PRD](./prd.md), [tech-docs](./tech-docs.md)
- [ ] Inspect the four shipped fixture PDFs in [`fixture/`](./fixture/)

## Phase 0a — Environment setup

- [ ] Install all dependencies in the repo root worktree: `npm install`
- [ ] Converge the full polyglot toolchain: `npm run doctor -- --fix`
      (required — the `postinstall` hook runs `doctor || true` and silently
      tolerates drift; see
      [Worktree Toolchain Initialization](../../../governance/development/workflow/worktree-setup.md))
- [ ] Copy environment template: `cp apps/investment-oracle-be/.env.example apps/investment-oracle-be/.env`
- [ ] Fill in `ANTHROPIC_API_KEY` and `GOOGLE_API_KEY` in the new `.env`
      (obtain from the respective vendor consoles; keep out of version control)
- [ ] Start the Postgres + pgvector service:
      `docker compose -f docker-compose.integration.yml up -d`
- [ ] Verify the service is healthy: `docker compose -f docker-compose.integration.yml ps`
- [ ] Verify existing tests pass before making changes:
      `nx affected -t test:quick`

## Phase 1 — Spec area scaffolding

- [ ] Create `specs/apps/investment-oracle/` mirroring `specs/apps/crud/`:
  - [ ] `README.md`
  - [ ] `c4/` (System Context, Container, Component diagrams as Mermaid)
  - [ ] `be/gherkin/` (one `.feature` per FR group: sources, analyses,
        report, edits, guardrails, determinism)
  - [ ] `fe/gherkin/` (one `.feature` per UX flow: ingest, generate,
        manual-edit, llm-edit, history)
  - [ ] `contracts/openapi.yaml` (OpenAPI 3.1)
  - [ ] `contracts/project.json` (Nx project; `lint` and `docs` targets)
- [ ] Validate via `npx nx run investment-oracle-contracts:lint`

## Phase 2 — Tauri 2 + React + Vite scaffolding

- [ ] `npx nx generate @nx/vite:app investment-oracle-fe --framework=react`
      (or run `npm create tauri-app@latest` and import into Nx — see Tauri docs)
- [ ] Initialise Tauri 2 in `apps/investment-oracle-fe/src-tauri/`:
  - [ ] `Cargo.toml` with `tauri = "2"`, `tauri-plugin-shell = "2"`
  - [ ] `tauri.conf.json` with `bundle.externalBin = ["binaries/investment-oracle-be"]`
  - [ ] `src/main.rs` per [tech-docs Tauri sidecar spawn snippet](./tech-docs.md#tauri-sidecar-spawn--rust)
- [ ] Wire `@open-sharia-enterprise/ts-ui` as a workspace dependency
      (mandatory; no app-local primitives)
- [ ] Verify dev mode: `npx nx run investment-oracle-fe:dev` opens a Tauri
      window pointing at the Vite dev server
- [ ] Add Nx targets: `dev`, `build`, `tauri-build`, `typecheck`, `lint`,
      `test:unit`, `test:e2e`, `codegen`, `spec-coverage`

## Phase 3 — Sidecar Python project scaffolding

- [ ] Create `apps/investment-oracle-be/` with `pyproject.toml` pinning:
  - [ ] runtime: `fastapi`, `uvicorn[standard]`, `sse-starlette`,
        `sqlalchemy[asyncio]`, `asyncpg`, `pypdf`, `anthropic==0.97.*`,
        `google-genai==1.73.*`, `httpx`, `slowapi`, `pydantic-settings`,
        `python-multipart`
  - [ ] test stack: `pytest`, `pytest-bdd`, `pytest-asyncio`,
        `pytest-httpx`, `freezegun`, `coverage`
  - [ ] lint: `ruff`
  - [ ] typecheck: `pyright`
  - [ ] packaging: `pyinstaller`
- [ ] Add Nx targets: `dev`, `build`, `typecheck`, `lint`, `test:unit`,
      `test:integration`, `test:quick`, `codegen`, `spec-coverage`,
      `pyinstaller-build`
- [ ] Confirm PyMuPDF is **not** in deps; document the ban in
      `apps/investment-oracle-be/CONTRIBUTING.md`
- [ ] Update `rhino-cli java validate-annotations` (or the equivalent
      scope check) to flag PyMuPDF if introduced — extend the existing dep
      scope check

## Phase 4 — Postgres + pgvector wiring

- [ ] Confirm `docker-compose.integration.yml` already uses
      `pgvector/pgvector:pg16` (added by the crud-be plans)
- [ ] Add `init/01-vector.sql` invoking `CREATE EXTENSION IF NOT EXISTS vector;`
      to the compose volume
- [ ] Write Alembic migration for the six tables in
      [tech-docs.md schema](./tech-docs.md#database-schema)
- [ ] Verify ivfflat index builds: `docker compose up`,
      `\d+ source_chunks` shows the index

## Phase 5 — Contract codegen

- [ ] Author `specs/apps/investment-oracle/contracts/openapi.yaml`
- [ ] Wire `npx nx run investment-oracle-contracts:lint` (Spectral)
- [ ] Wire `npx nx run investment-oracle-be:codegen` (e.g.,
      `datamodel-code-generator` or `openapi-python-client`) generating Pydantic
      request/response models into `generated-contracts/`
- [ ] Wire `npx nx run investment-oracle-fe:codegen` generating TypeScript
      types into `apps/investment-oracle-fe/src/api/generated/` via
      `openapi-typescript`
- [ ] `codegen` is a dependency of `typecheck` and `build` for both
      projects (matches crud-\* convention)

## Phase 6 — BE: domain layer

- [ ] Implement `domain/chat_provider.py` Protocol + Anthropic + Gemini
      implementations per [tech-docs.md provider abstraction](./tech-docs.md#provider-abstraction)
- [ ] Implement `domain/embedder.py` (Gemini-only) with
      `output_dimensionality=768` and `task_type` parameterisation
- [ ] Implement `domain/chunker.py` (recursive splitter on paragraph →
      sentence → character; 800/100 default)
- [ ] Implement `domain/retriever.py` (multi-source pgvector query per
      [tech-docs.md SQL](./tech-docs.md#multi-source-retrieval-sql))
- [ ] Implement `domain/content_filter.py` Protocol + regex impl; load
      rules from `content_filter/default_rules.txt`
- [ ] Implement `domain/cost_cap.py` reading `token_usage`, returning
      `BudgetState` (`under_cap` / `at_cap` / `over_cap`) before each call

## Phase 6a — BE: residency, PII masking, web grounding

- [ ] Implement `domain/pii_masker.py` Protocol + `IndonesianRegexMasker`
      default impl per [tech-docs.md PIIMasker](./tech-docs.md#piimasker-protocol).
      Detect NIK, NPWP, phone (Indonesia), email, bank account, credit
      card. Numbered placeholders (`[NIK_001]`, `[NPWP_001]`, …) with an
      in-memory reverse map scoped to the call.
- [ ] Wire `PIIMasker.mask()` into `ChatProvider.stream()` and
      `Embedder.embed()` so every outbound payload is masked when
      `PII_MASKING_ENABLED=true`. Streaming responses pass through
      `PIIMasker.unmask()` before persistence and FE emit.
- [ ] Implement `domain/residency.py`: enum of the four profiles
      (`direct-us`, `bedrock-jakarta-cris`, `bedrock-jakarta-in-region`,
      `vertex-singapore`); record the active profile on every
      `token_usage` row via the new `residency_profile` column.
- [ ] Reject `PII_MASKING_ENABLED=false` for any profile other than
      `bedrock-jakarta-in-region` with `409 masking_required_for_residency`.
- [ ] Implement `domain/web_grounder.py` Protocol + `PerplexityGrounder`
      default impl per [tech-docs.md WebGrounder](./tech-docs.md#webgrounder-protocol-optional-perplexity-layer).
      Reads `PERPLEXITY_API_KEY`. Single Sonar call with
      `search_recency_filter: "month"` and a default
      `search_domain_filter: ["sec.gov", "wsj.com", "reuters.com",
"bloomberg.com"]`.
- [ ] Wire `WebGrounder.ground()` into `ReportGenerator.generate()` and
      the LLM-edit handler when `web_grounding=true` is requested.
- [ ] Add migration for the `web_citations` JSONB column on
      `report_revisions` and the `residency_profile` + `search_fee_usd`
      columns on `token_usage` per
      [tech-docs.md schema additions](./tech-docs.md#schema-additions).
- [ ] Update `domain/cost_cap.py` to add `search_fee_usd` to the budget
      check; default `COST_CAP_PER_ANALYSIS_USD` raised from `0.50` to
      `0.75` to accommodate Perplexity.
- [ ] Unit test: assert outbound HTTP body contains no raw NIK / NPWP /
      phone / email / bank / CC pattern when `PII_MASKING_ENABLED=true`
      across all three vendor SDKs (Anthropic, Gemini, Perplexity). Use
      `pytest-httpx`'s captured request feature.

## Phase 7 — BE: API layer

- [ ] `api/health.py` — `GET /health`
- [ ] `api/sources.py` — `POST`, `GET`, `DELETE`; pypdf extraction → chunk
      → embed → store
- [ ] `api/analyses.py` — CRUD on analyses + `analysis_sources`
- [ ] `api/report.py` — `POST` (generate, SSE), `PATCH` (manual edit), `POST :edit`
      (LLM section rewrite, SSE), `GET /revisions`, `POST /revisions/{rid}:restore`
- [ ] Wire content filter and cost cap as FastAPI dependencies on every
      chat-touching route
- [ ] Wire `slowapi` rate-limit dependency, gated by `RATE_LIMIT_ENABLED`

## Phase 7a — API smoke (curl assertions)

Explicit curl invocations against every FastAPI endpoint. Run with the BE sidecar
started via `nx run investment-oracle-be:dev` (port 8501 by default).

- [ ] Health check:
      `curl -s http://localhost:8501/health | jq .`
      → expect `{"status": "ok"}` (HTTP 200)
- [ ] Upload a source PDF:
      `curl -s -o /dev/null -w "%{http_code}" -X POST http://localhost:8501/api/v1/sources -F "file=@plans/in-progress/2026-04-27__add-investment-oracle-app/fixture/aapl-fy2024-10k.pdf"`
      → expect `201`
- [ ] List sources:
      `curl -s http://localhost:8501/api/v1/sources | jq '.[].id'`
      → expect at least one source ID in the array
- [ ] Reject oversized upload (create a 30 MB dummy file first):
      `dd if=/dev/zero bs=1m count=30 | curl -s -o /dev/null -w "%{http_code}" -X POST http://localhost:8501/api/v1/sources -F "file=@/dev/stdin;filename=big.pdf"`
      → expect `413`
- [ ] Create an analysis:
      `curl -s -X POST http://localhost:8501/api/v1/analyses -H 'Content-Type: application/json' -d '{"name":"smoke","source_ids":["<id-from-above>"]}' | jq .`
      → expect `201` with an `id` field
- [ ] Get an analysis:
      `curl -s http://localhost:8501/api/v1/analyses/<id> | jq '{id,name}'`
      → expect `200` with name `"smoke"`
- [ ] Delete a source tied to an analysis:
      `curl -s -o /dev/null -w "%{http_code}" -X DELETE http://localhost:8501/api/v1/sources/<source-id>`
      → expect `409` (source in use)
- [ ] Delete an analysis:
      `curl -s -o /dev/null -w "%{http_code}" -X DELETE http://localhost:8501/api/v1/analyses/<id>`
      → expect `200`
- [ ] List revisions (requires a generated report — run after Phase 9 with a cassette):
      `curl -s http://localhost:8501/api/v1/analyses/<id>/report/revisions | jq 'length'`
      → expect `≥ 1`
- [ ] Manual edit (PATCH report):
      `curl -s -o /dev/null -w "%{http_code}" -X PATCH http://localhost:8501/api/v1/analyses/<id>/report -H 'Content-Type: application/json' -d '{"content_md":"# Edited"}'`
      → expect `200`
- [ ] Confirm all 11 endpoints documented in
      [tech-docs.md](./tech-docs.md#openapi-endpoints) have been exercised above;
      check for any 500 responses in the sidecar logs

## Phase 8 — BE: prompts

- [ ] `prompts/report_generation.md` — six-section structured prompt
      with explicit "do not invent figures" rule and the not-investment-advice
      footer requirement on the Recommendation section
- [ ] `prompts/report_edit.md` — section-rewrite prompt that preserves
      heading and the disclaimer footer
- [ ] Both prompt files are loaded at startup; tests assert that the
      outbound request system text **starts with** the file content
      (fingerprint, not prose)

## Phase 9 — BE: tests (unit, level 1 of 3)

- [ ] Author `tests/unit/` step implementations consuming the Gherkin
      files from `specs/apps/investment-oracle/be/gherkin/`
- [ ] Use `pytest-httpx` cassettes for both Anthropic and Gemini base URLs
      per [tech-docs.md cassette structure](./tech-docs.md#mock-cassette-structure)
- [ ] Assert outbound-request fingerprint, side effects, structural shape;
      never on LLM prose
- [ ] Confirm coverage ≥ 90 % via `nx run investment-oracle-be:test:quick`

## Phase 10 — BE: tests (integration, level 2 of 3)

- [ ] Author `tests/integration/` step implementations for the same
      Gherkin files
- [ ] Spin up `docker-compose.integration.yml` Postgres + pgvector; load
      the AAPL fixture PDF; assert ingest pipeline writes correct chunks
- [ ] Vendor HTTP still mocked via cassettes — only DB + parser are real
- [ ] `nx run investment-oracle-be:test:integration` passes; `cache: false`
      in `nx.json`

## Phase 11 — BE: tests (e2e, level 3 of 3)

- [ ] Create `apps/investment-oracle-be-e2e/` Playwright-bdd project
- [ ] Use `playwright.config.ts` `webServer` to launch the FastAPI sidecar
- [ ] Cassettes still in play; test the full HTTP path against a running
      uvicorn + real DB

## Phase 12 — BE: PyInstaller packaging

- [ ] Author `pyinstaller.spec` with `--onedir` build mode
- [ ] Build target: `nx run investment-oracle-be:pyinstaller-build`
      produces `dist/investment-oracle-be/` folder
- [ ] Post-build script copies the folder into
      `apps/investment-oracle-fe/src-tauri/binaries/investment-oracle-be-{target-triple}/`
- [ ] Smoke: launch the produced binary, hit `/health`, expect 200

## Phase 13 — FE: tests (unit, level 1 of 2)

- [ ] Vitest + @testing-library/react; no integration level
- [ ] All BE HTTP mocked via MSW
- [ ] All Tauri APIs mocked via `@tauri-apps/api/__mocks__`
- [ ] TypeScript-strict (`tsc --noEmit` zero errors); `oxlint` zero
- [ ] Coverage ≥ 70 % via `nx run investment-oracle-fe:test:quick`

## Phase 14 — FE: components

- [ ] `DisclaimerBanner.tsx` (sticky top, persistent)
- [ ] `SourcesPanel.tsx` (drop zone, list, multi-select, delete)
- [ ] `ReportEditor.tsx` (CodeMirror 6 + markdown mode; controlled
      component; SSE chunk append)
- [ ] `PromptInput.tsx` (section selector + prompt text + send)
- [ ] `ModelSelector.tsx` (dropdown with the two model ids)
- [ ] `RevisionHistoryDrawer.tsx` (list, restore action)
- [ ] All primitives sourced from `@open-sharia-enterprise/ts-ui`

## Phase 15 — FE: SSE client wiring

- [ ] `lib/sse-client.ts` using `@microsoft/fetch-event-source` per
      [tech-docs.md FE SSE consumer](./tech-docs.md#fe-sse-consumer--typescript)
- [ ] Component integration tests assert chunks render incrementally
      (Vitest fake timers + scripted async generator)

## Phase 16 — FE: tests (e2e, level 2 of 2)

- [ ] Create `apps/investment-oracle-fe-e2e/` Playwright-bdd project
- [ ] `playwright.config.ts` runs `vite preview` (built FE in browser
      mode) plus the FastAPI sidecar
- [ ] Cassettes still in play
- [ ] **No** Tauri-shell automated E2E — verified manually only
      (Phase 22)

## Phase 16a — Playwright MCP UI assertion

Agent-executable manual UI verification using Playwright MCP browser tools.
Run this phase against the `vite preview` build (browser mode, no Tauri shell)
after Phase 16 FE e2e tests pass.

- [ ] Start the FE preview server and BE sidecar:
      `nx run investment-oracle-fe:build` then `nx run investment-oracle-fe:preview`
      (also ensure `nx run investment-oracle-be:dev` is running)
- [ ] `browser_navigate` to `http://localhost:1420`
- [ ] `browser_snapshot` — verify the split view renders: Sources pane on the
      left, Report editor on the right with a vertical divider
- [ ] `browser_snapshot` — verify the disclaimer banner ("Demo output, not
      investment advice") is visible at the top of the window
- [ ] `browser_take_screenshot` — capture baseline layout screenshot
- [ ] `browser_console_messages` — confirm zero JavaScript errors in the console
- [ ] `browser_click` on the Sources drop zone — verify the drag-and-drop ingest
      area is interactive (no JS error thrown on click)
- [ ] `browser_snapshot` — verify the model selector dropdown is visible in the
      top bar and shows `claude-haiku-4-5` as the default selection
- [ ] `browser_network_requests` — confirm no unexpected failed network requests
      on initial page load
- [ ] Document results: note any rendering discrepancies found

## Phase 17 — Spec coverage

- [ ] `nx run investment-oracle-be:spec-coverage` exits 0
- [ ] `nx run investment-oracle-fe:spec-coverage` exits 0
- [ ] Investigate any gaps; either add a step impl or remove the scenario

## Phase 18 — CI workflows

- [ ] Author `.github/workflows/test-investment-oracle-be.yml` mirroring
      `test-crud-be.yml`
- [ ] Author `.github/workflows/test-investment-oracle-fe.yml`
- [ ] Author `.github/workflows/test-investment-oracle-be-e2e.yml`
- [ ] Author `.github/workflows/test-investment-oracle-fe-e2e.yml`
- [ ] Author `.github/workflows/build-investment-oracle-tauri.yml`
      (macOS-arm64 only on CI; Windows + Linux as separate jobs gated by
      `workflow_dispatch`)
- [ ] Author `.github/workflows/smoke-investment-oracle-real-vendor.yml`
      (workflow-dispatch + weekly schedule; sets `MOCK_LLM_PROVIDERS=false`;
      asserts only HTTP 200 + clean SSE close + at least one chunk)

## Phase 19 — Repo docs and convention pin

- [ ] Author `governance/development/pattern/llm-demo-pattern.md` codifying:
  - direct vendor SDKs over proxies
  - cassette-driven CI
  - three-level BE / two-level FE testing
  - LLM determinism strategy
  - cost-cap + content-filter guardrails
- [ ] Add `apps/investment-oracle-be/README.md` and
      `apps/investment-oracle-fe/README.md` with run / dev / test instructions
- [ ] Update root `CLAUDE.md` "Current Apps" list to include the four new
      projects
- [ ] Update `package.json` scripts: `dev:investment-oracle`,
      `build:investment-oracle`

## Phase 20 — Quality gate

- [ ] Run all 11 quality-gate commands listed in [README](./README.md#quality-gates-must-all-pass-before-merge)
- [ ] Re-run after every fix; no item ships green if any other goes red

## Phase 21 — Fix-all sweep (root-cause orientation)

- [ ] Run `npm run lint:md`, `nx affected -t lint typecheck test:quick spec-coverage`
- [ ] Fix every finding at the root cause (per the
      [Root Cause Orientation principle](../../../governance/principles/README.md));
      do **not** suppress or stub
- [ ] Re-run until clean

## Phase 22 — Manual smoke (Tauri shell + real-vendor sanity)

- [ ] Build the Tauri bundle:
      `nx run investment-oracle-fe:tauri-build` (macOS arm64)
- [ ] Launch the resulting `.app`
- [ ] Drag `fixture/aapl-fy2024-10k.pdf` into Sources; confirm ingest
      completes within ~10 s
- [ ] Drag `fixture/msft-fy2024-annual-report.pdf` into Sources
- [ ] Create an analysis attaching both; click Generate report; confirm
      six sections stream in
- [ ] Manual edit: change a sentence in Risks; confirm the saved revision
      shows in the drawer
- [ ] LLM edit: select Recommendation, prompt _"more cautious"_; confirm
      the section is rewritten and a new `llm_edit` revision row exists
- [ ] Switch model dropdown to `gemini-2.5-flash-lite`; trigger a fresh
      LLM edit; confirm outbound traffic now hits Google (use macOS
      `Console.app` or `lsof -i` while filtering by sidecar pid)
- [ ] Confirm the disclaimer banner is visible at all times
- [ ] Confirm the sidecar process exits when the Tauri window is closed

## Phase 23 — Plan-quality-gate

- [ ] Run `governance/workflows/plan/plan-quality-gate.md` over the
      authored plan: `plan-checker` → `plan-fixer` until two consecutive
      zero-strict-threshold validations
- [ ] Audit reports archived under `generated-reports/plan__*`

## Phase 24 — Commit and push

### Commit guidelines

- [ ] Confirm linear history before pushing:
      `git pull --rebase origin main`
- [ ] Stage and commit thematically — one logical concern per commit; use
      Conventional Commits format
      ([commit-messages convention](../../../governance/development/workflow/commit-messages.md)):
  - `feat(specs): ...` for spec area scaffolding
  - `feat(be): ...` for BE domain / API / prompts / packaging
  - `feat(fe): ...` for FE components / SSE wiring
  - `test(be): ...` / `test(fe): ...` for test-level changes
  - `test(e2e): ...` for e2e projects
  - `docs(governance): ...` for LLM demo pattern convention
  - `ci(investment-oracle): ...` for GitHub Actions workflows
  - `chore(plans): ...` for plan archival
- [ ] Do NOT bundle unrelated preexisting fixes with new feature commits;
      each commit touches one logical concern (specs / be / fe / be-e2e /
      fe-e2e / docs / ci) and never crosses domains

### Push and CI verification

- [ ] Push direct to `main` per Trunk Based Development
      ([git-push-default](../../../governance/development/workflow/git-push-default.md)):
      `git push origin main`
- [ ] Open the GitHub Actions tab for the `ose-primer` repository
- [ ] Monitor the following workflows triggered by the push:
  - `test-investment-oracle-be`
  - `test-investment-oracle-fe`
  - `test-investment-oracle-be-e2e`
  - `test-investment-oracle-fe-e2e`
  - `build-investment-oracle-tauri`
- [ ] Verify all CI checks pass (green status)
- [ ] If any CI check fails, fix the root cause immediately and push a
      follow-up commit; do NOT proceed until CI is green
- [ ] Move this plan to `plans/done/` once Phase 22 manual smoke passes,
      CI is green, and the push is on `main`

## Phase 25 — Post-merge

- [ ] Update `plans/ideas.md` and `plans/backlog/` with follow-up notes
      (export to PDF / DOCX, polyglot backend ports, real moderation
      provider, Linux + Windows CI lanes)
- [ ] Tag the release in the README's "What this template ships" table

---

**Reminder — root-cause orientation.** Encountering a preexisting bug,
type error, or test flake during execution is **not** a deferral: fix it
in the same sweep that touches the affected file, per the Root Cause
Orientation principle. Document any unexpected fix as a separate commit
on the same branch with its own conventional-commits scope.
