# Delivery — Adopt Hexagonal Architecture + DDD

## Worktree

Worktree path: `worktrees/adopt-hexagonal-ddd-architecture/`

Provision before execution (run from repo root):

```bash
claude --worktree adopt-hexagonal-ddd-architecture
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Commit Guidance

Follow [Conventional Commits](https://www.conventionalcommits.org/) for every phase commit.
**Commit changes thematically**: each commit must be logically cohesive — one focused concern
per commit, never bundled across domains or phases.

- **Do not bundle governance doc changes with app structural changes** in a single commit.
- **Split different domain concerns**: Phase 1 (docs) and Phase 2 (CLI) must be separate commits
  even if both are ready at the same time. Do NOT bundle changes from different phases or apps.
- **Commit message format**: `<type>(<scope>): <imperative description>`
  - `docs(governance): ...` for Phase 1
  - `feat(rhino-cli): ...` for Phase 2
  - `feat(crud-fe): ...` for Phase 3
  - `feat(crud-be): ...` for Phase 4
  - `feat(crud-be): ...` or `chore(contracts): ...` for Phase 5
- **Do not amend published commits** — if a CI fix is needed after push, create a new commit.

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [x] Install dependencies in the root worktree: run `npm install` from
      `/Users/wkf/ose-projects/ose-primer/` (or the worktree root).
      — acceptance: command exits 0, `node_modules/` is present and synchronized.
- [x] Converge the full polyglot toolchain: run `npm run doctor -- --fix` from the repo root.
      — acceptance: command exits 0 with no unresolved drift reported.
- [x] Record a baseline test run across all non-E2E apps:
      `npx nx run-many -t test:quick --projects=rhino-cli-rust,rhino-cli-go,crud-fe-ts-nextjs,crud-fe-ts-tanstack-start,crud-fe-dart-flutterweb,crud-fs-ts-nextjs,crud-be-rust-axum,crud-be-golang-gin,crud-be-fsharp-giraffe,crud-be-ts-effect,crud-be-python-fastapi,crud-be-clojure-pedestal,crud-be-java-vertx,crud-be-java-springboot,crud-be-kotlin-ktor,crud-be-elixir-phoenix,crud-be-csharp-aspnetcore`
      — acceptance: baseline pass/fail count recorded; all preexisting failures documented
      before Phase 1 begins.
- [x] Resolve all preexisting test failures before proceeding to Phase 1.
      — acceptance: no preexisting failures remain unresolved; re-run the command above and
      confirm zero new failures.

---

## Phase 1: Governance Convention Documents

Create five convention documents under `repo-governance/development/pattern/` and update
the index README.

> **Important**: Fix ALL lint failures found during quality gates, not just those caused by
> your changes. This follows the root cause orientation principle — proactively fix
> preexisting errors encountered during work.

### Phase 1 — Step 1: hexagonal-architecture.md

- [x] Create `repo-governance/development/pattern/hexagonal-architecture.md` (_New file_)
      with the following content: title "Hexagonal Architecture", sections covering — (1)
      Overview: what hexagonal architecture is and why it is used in ose-primer; (2)
      Dependency Rule: prose description + the inward-dependency-only rule; (3) Mermaid
      diagram showing the four-zone dependency flow (domain ← application ← infrastructure
      and api/http, using the standard `flowchart LR` orientation and colour-blind-friendly
      palette from `repo-governance/conventions/formatting/diagrams.md`); (4) Terminology:
      define domain, application, port, adapter, inbound adapter, outbound adapter; (5)
      References: link to the four specialization docs.
      — acceptance: `bash test -f repo-governance/development/pattern/hexagonal-architecture.md`
      exits 0; `npx markdownlint-cli2 repo-governance/development/pattern/hexagonal-architecture.md`
      exits 0 with zero violations.
  - _Suggested executor: `docs-maker`_

### Phase 1 — Step 2: hexagonal-architecture-cli.md

- [x] Create `repo-governance/development/pattern/hexagonal-architecture-cli.md` (_New file_)
      with the following content: title "Hexagonal Architecture — CLI Apps", sections covering —
      (1) Scope: applies to `rhino-cli-rust` (Rust) and `rhino-cli-go` (Go); (2) Rust CLI
      layer table: `src/domain/`, `src/application/`, `src/infrastructure/`, `src/commands/`;
      (3) Go CLI layer table: `internal/domain/`, `internal/application/`,
      `internal/adapter/command/`, `cmd/`; (4) Notes on the Go `internal/` compiler constraint
      and why the CLI adapter is named `adapter/command/` not `infrastructure/`; (5)
      References: link to `hexagonal-architecture.md`.
      — acceptance: `bash test -f repo-governance/development/pattern/hexagonal-architecture-cli.md`
      exits 0; markdownlint exits 0 with zero violations.
  - _Suggested executor: `docs-maker`_

### Phase 1 — Step 3: hexagonal-architecture-web.md

- [x] Create `repo-governance/development/pattern/hexagonal-architecture-web.md` (_New file_)
      with the following content: title "Hexagonal Architecture — Web/FE Apps", sections
      covering — (1) Scope: applies to `crud-fe-ts-nextjs` (Next.js), `crud-fe-ts-tanstack-start`
      (TanStack Start), `crud-fe-dart-flutterweb` (Flutter Web), `crud-fs-ts-nextjs` (fullstack
      Next.js, treated as FE); (2) Standard layer table: `domain/`, `application/`,
      `infrastructure/`, `presentation/` with descriptions of what belongs in each; (3) Source
      root by app (table); (4) Note on `crud-fs-ts-nextjs` — fullstack app treated as FE, no
      DDD bounded context; (5) Note on Dart/Flutter — `lib/` is the source root, `infrastructure/`
      is used instead of Flutter community's `data/`; (6) References: link to
      `hexagonal-architecture.md`.
      — acceptance: `bash test -f repo-governance/development/pattern/hexagonal-architecture-web.md`
      exits 0; markdownlint exits 0 with zero violations.
  - _Suggested executor: `docs-maker`_

### Phase 1 — Step 4: hexagonal-architecture-be.md

- [x] Create `repo-governance/development/pattern/hexagonal-architecture-be.md` (_New file_)
      with the following content: title "Hexagonal Architecture — Backend Apps", sections
      covering — (1) Scope: applies to all 11 BE apps; (2) Bounded-context pattern: why
      `contexts/<n>/` wraps all four layers; (3) Context name: `expenses` for the CRUD demo;
      (4) Full language-specific directory layout table (all 11 apps, matching the table in
      `tech-docs.md §Canonical Layer Naming §BE Apps`); (5) `api/http/` two-level structure:
      why `api/` contains `http/` (mirrors ose-public exactly, allows future non-HTTP adapter
      siblings); (6) Language-specific notes subsection covering: F#/C# PascalCase, Go
      `internal/` wrapper, Elixir Phoenix exception (`lib/<app>_web/` IS the HTTP adapter),
      Rust `api/mod.rs` + `api/http/mod.rs`, JVM lowercase packages, Clojure namespace
      convention; (7) References: link to `hexagonal-architecture.md` and
      `openapi-contract-first.md`.
      — acceptance: `bash test -f repo-governance/development/pattern/hexagonal-architecture-be.md`
      exits 0; markdownlint exits 0 with zero violations.
  - _Suggested executor: `docs-maker`_

### Phase 1 — Step 5: openapi-contract-first.md

- [x] Create `repo-governance/development/pattern/openapi-contract-first.md` (_New file_)
      with the following content: title "OpenAPI Contract-First Development", sections
      covering — (1) Overview: contract-first means the OpenAPI spec is the source of truth,
      not generated from code; (2) Single source of truth: `specs/apps/crud/containers/contracts/openapi.yaml`
      [Repo-grounded]; (3) Bundled artifact: `specs/apps/crud/containers/contracts/generated/openapi-bundled.yaml`
      generated by `npx nx run crud-contracts:bundle`; (4) BE codegen: each BE app's `codegen`
      Nx target reads the bundled spec and generates types into `generated-contracts/`; table
      listing all 11 BE apps with their codegen command (`npx nx run <app>:codegen`); (5) FE
      consumer codegen: `crud-fe-ts-nextjs`, `crud-fe-ts-tanstack-start`, and
      `crud-fe-dart-flutterweb` consume the bundled spec; (6) Drift check: running
      `npx nx run-many -t codegen` followed by `git diff --quiet generated-contracts/` detects
      spec drift; (7) References: link to `hexagonal-architecture-be.md`.
      — acceptance: `bash test -f repo-governance/development/pattern/openapi-contract-first.md`
      exits 0; markdownlint exits 0 with zero violations.
  - _Suggested executor: `docs-maker`_

### Phase 1 — Step 6: Update pattern/README.md

- [x] Edit `repo-governance/development/pattern/README.md`: add five new entries to the
      `## Documents` section following the existing format — one line each linking to
      `hexagonal-architecture.md`, `hexagonal-architecture-cli.md`,
      `hexagonal-architecture-web.md`, `hexagonal-architecture-be.md`, and
      `openapi-contract-first.md`. Use active-voice, single-sentence descriptions consistent
      with existing entries in that file.
      — acceptance: `bash test -f repo-governance/development/pattern/README.md` exits 0;
      `grep -c "hexagonal-architecture.md" repo-governance/development/pattern/README.md`
      returns 1; markdownlint on the file exits 0.
  - _Suggested executor: `docs-maker`_

### Phase 1 — Local Quality Gates

- [x] Run markdownlint on all five new files and the updated README:
      `npx markdownlint-cli2 "repo-governance/development/pattern/hexagonal-architecture*.md" "repo-governance/development/pattern/openapi-contract-first.md" "repo-governance/development/pattern/README.md"`
      — acceptance: exits 0 with zero violations.
- [x] Run affected lint: `npx nx affected -t lint`
      — acceptance: exits 0.
- [x] Run spec-coverage: `npx nx affected -t spec-coverage`
      — acceptance: exits 0; no new failures vs Phase 0 baseline.
- [x] Fix ALL failures, including preexisting issues encountered during lint.

### Phase 1 — Commit

- [x] Stage and commit all Phase 1 files:
      `git add repo-governance/development/pattern/hexagonal-architecture.md repo-governance/development/pattern/hexagonal-architecture-cli.md repo-governance/development/pattern/hexagonal-architecture-web.md repo-governance/development/pattern/hexagonal-architecture-be.md repo-governance/development/pattern/openapi-contract-first.md repo-governance/development/pattern/README.md`
      then commit with message:
      `docs(governance): add hexagonal architecture + openapi-contract-first conventions`
      — acceptance: `git status` shows clean working tree for those files; commit hash recorded.

### Phase 1 — Post-Push CI Verification

- [x] Push: `git push origin HEAD:main`
- [x] Monitor these workflows: `pr-quality-gate`, `pr-validate-links`.
- [x] Verify ALL CI checks pass — no exceptions.
- [x] If any CI check fails, fix immediately and push a follow-up commit.
- [x] Do NOT proceed to Phase 2 until CI is fully green.

---

## Phase 2: CLI App Layer Directories

Add canonical hexagonal layer directories to the two CLI apps.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes.

### Phase 2 — Step 1: rhino-cli-rust layers

- [x] Create directory `apps/rhino-cli-rust/src/domain/` with a `.gitkeep` file:
      `mkdir -p apps/rhino-cli-rust/src/domain && touch apps/rhino-cli-rust/src/domain/.gitkeep`
      — acceptance: `bash test -f apps/rhino-cli-rust/src/domain/.gitkeep` exits 0.
  - _Suggested executor: `swe-rust-dev`_

- [x] Create directory `apps/rhino-cli-rust/src/application/` with a `.gitkeep` file:
      `mkdir -p apps/rhino-cli-rust/src/application && touch apps/rhino-cli-rust/src/application/.gitkeep`
      — acceptance: `bash test -f apps/rhino-cli-rust/src/application/.gitkeep` exits 0.
  - _Suggested executor: `swe-rust-dev`_

- [x] Create directory `apps/rhino-cli-rust/src/infrastructure/` with a `.gitkeep` file:
      `mkdir -p apps/rhino-cli-rust/src/infrastructure && touch apps/rhino-cli-rust/src/infrastructure/.gitkeep`
      — acceptance: `bash test -f apps/rhino-cli-rust/src/infrastructure/.gitkeep` exits 0.
  - _Suggested executor: `swe-rust-dev`_

- [x] Verify `apps/rhino-cli-rust/src/commands/` already exists (it does):
      `bash test -d apps/rhino-cli-rust/src/commands`
      — acceptance: exits 0. [Repo-grounded: `apps/rhino-cli-rust/src/commands/` exists]

### Phase 2 — Step 2: rhino-cli-go layers

- [x] Create directory `apps/rhino-cli-go/internal/domain/` with a `.gitkeep` file:
      `mkdir -p apps/rhino-cli-go/internal/domain && touch apps/rhino-cli-go/internal/domain/.gitkeep`
      — acceptance: `bash test -f apps/rhino-cli-go/internal/domain/.gitkeep` exits 0.
  - _Suggested executor: `swe-golang-dev`_

- [x] Create directory `apps/rhino-cli-go/internal/application/` with a `.gitkeep` file:
      `mkdir -p apps/rhino-cli-go/internal/application && touch apps/rhino-cli-go/internal/application/.gitkeep`
      — acceptance: `bash test -f apps/rhino-cli-go/internal/application/.gitkeep` exits 0.
  - _Suggested executor: `swe-golang-dev`_

- [x] Create directory `apps/rhino-cli-go/internal/adapter/command/` with a `.gitkeep` file:
      `mkdir -p apps/rhino-cli-go/internal/adapter/command && touch apps/rhino-cli-go/internal/adapter/command/.gitkeep`
      — acceptance: `bash test -f apps/rhino-cli-go/internal/adapter/command/.gitkeep` exits 0.
  - _Suggested executor: `swe-golang-dev`_

- [x] Verify `apps/rhino-cli-go/cmd/` already exists:
      `bash test -d apps/rhino-cli-go/cmd`
      — acceptance: exits 0. [Repo-grounded: `apps/rhino-cli-go/cmd/` exists]

### Phase 2 — Local Quality Gates

- [x] Run affected tests for CLI apps:
      `npx nx run-many -t test:quick --projects=rhino-cli-rust,rhino-cli-go`
      — acceptance: exits 0; same pass count as Phase 0 baseline.
- [x] Run affected lint: `npx nx affected -t lint`
      — acceptance: exits 0.
- [x] Run spec-coverage: `npx nx affected -t spec-coverage`
      — acceptance: exits 0; no new failures vs Phase 0 baseline.
- [x] Fix ALL failures.

### Phase 2 — Commit

- [x] Stage and commit all Phase 2 `.gitkeep` files:
      `git add apps/rhino-cli-rust/src/domain/.gitkeep apps/rhino-cli-rust/src/application/.gitkeep apps/rhino-cli-rust/src/infrastructure/.gitkeep apps/rhino-cli-go/internal/domain/.gitkeep apps/rhino-cli-go/internal/application/.gitkeep apps/rhino-cli-go/internal/adapter/command/.gitkeep`
      then commit:
      `feat(rhino-cli): add canonical hexagonal layer directories`
      — acceptance: `git status` clean for those files.

### Phase 2 — Post-Push CI Verification

- [x] Push: `git push origin HEAD:main`
- [x] Monitor these workflows: `pr-quality-gate`.
      (No separate per-CLI CI workflows exist — `pr-quality-gate` covers lint and typecheck.)
- [x] Verify ALL CI checks pass.
- [x] Fix immediately if any fail.
- [x] Do NOT proceed to Phase 3 until CI is fully green.

---

## Phase 3: FE App Layer Directories

Add canonical hexagonal layer directories to the four FE apps.

> **Important**: Fix ALL failures found during quality gates.

### Phase 3 — Step 1: crud-fe-ts-nextjs layers

- [x] Create `apps/crud-fe-ts-nextjs/src/domain/.gitkeep`:
      `mkdir -p apps/crud-fe-ts-nextjs/src/domain && touch apps/crud-fe-ts-nextjs/src/domain/.gitkeep`
      — acceptance: `bash test -f apps/crud-fe-ts-nextjs/src/domain/.gitkeep` exits 0.

- [x] Create `apps/crud-fe-ts-nextjs/src/application/.gitkeep`:
      `mkdir -p apps/crud-fe-ts-nextjs/src/application && touch apps/crud-fe-ts-nextjs/src/application/.gitkeep`
      — acceptance: `bash test -f apps/crud-fe-ts-nextjs/src/application/.gitkeep` exits 0.

- [x] Create `apps/crud-fe-ts-nextjs/src/infrastructure/.gitkeep`:
      `mkdir -p apps/crud-fe-ts-nextjs/src/infrastructure && touch apps/crud-fe-ts-nextjs/src/infrastructure/.gitkeep`
      — acceptance: `bash test -f apps/crud-fe-ts-nextjs/src/infrastructure/.gitkeep` exits 0.

- [x] Create `apps/crud-fe-ts-nextjs/src/presentation/.gitkeep`:
      `mkdir -p apps/crud-fe-ts-nextjs/src/presentation && touch apps/crud-fe-ts-nextjs/src/presentation/.gitkeep`
      — acceptance: `bash test -f apps/crud-fe-ts-nextjs/src/presentation/.gitkeep` exits 0.
  - _Suggested executor: `swe-typescript-dev`_

### Phase 3 — Step 2: crud-fe-ts-tanstack-start layers

- [x] Create `apps/crud-fe-ts-tanstack-start/src/domain/.gitkeep`:
      `mkdir -p apps/crud-fe-ts-tanstack-start/src/domain && touch apps/crud-fe-ts-tanstack-start/src/domain/.gitkeep`
      — acceptance: `bash test -f apps/crud-fe-ts-tanstack-start/src/domain/.gitkeep` exits 0.

- [x] Create `apps/crud-fe-ts-tanstack-start/src/application/.gitkeep`:
      `mkdir -p apps/crud-fe-ts-tanstack-start/src/application && touch apps/crud-fe-ts-tanstack-start/src/application/.gitkeep`
      — acceptance: `bash test -f apps/crud-fe-ts-tanstack-start/src/application/.gitkeep` exits 0.

- [x] Create `apps/crud-fe-ts-tanstack-start/src/infrastructure/.gitkeep`:
      `mkdir -p apps/crud-fe-ts-tanstack-start/src/infrastructure && touch apps/crud-fe-ts-tanstack-start/src/infrastructure/.gitkeep`
      — acceptance: `bash test -f apps/crud-fe-ts-tanstack-start/src/infrastructure/.gitkeep` exits 0.

- [x] Create `apps/crud-fe-ts-tanstack-start/src/presentation/.gitkeep`:
      `mkdir -p apps/crud-fe-ts-tanstack-start/src/presentation && touch apps/crud-fe-ts-tanstack-start/src/presentation/.gitkeep`
      — acceptance: `bash test -f apps/crud-fe-ts-tanstack-start/src/presentation/.gitkeep` exits 0.
  - _Suggested executor: `swe-typescript-dev`_

### Phase 3 — Step 3: crud-fe-dart-flutterweb layers

- [x] Create `apps/crud-fe-dart-flutterweb/lib/domain/.gitkeep`:
      `mkdir -p apps/crud-fe-dart-flutterweb/lib/domain && touch apps/crud-fe-dart-flutterweb/lib/domain/.gitkeep`
      — acceptance: `bash test -f apps/crud-fe-dart-flutterweb/lib/domain/.gitkeep` exits 0.

- [x] Create `apps/crud-fe-dart-flutterweb/lib/application/.gitkeep`:
      `mkdir -p apps/crud-fe-dart-flutterweb/lib/application && touch apps/crud-fe-dart-flutterweb/lib/application/.gitkeep`
      — acceptance: `bash test -f apps/crud-fe-dart-flutterweb/lib/application/.gitkeep` exits 0.

- [x] Create `apps/crud-fe-dart-flutterweb/lib/infrastructure/.gitkeep`:
      `mkdir -p apps/crud-fe-dart-flutterweb/lib/infrastructure && touch apps/crud-fe-dart-flutterweb/lib/infrastructure/.gitkeep`
      — acceptance: `bash test -f apps/crud-fe-dart-flutterweb/lib/infrastructure/.gitkeep` exits 0.

- [x] Create `apps/crud-fe-dart-flutterweb/lib/presentation/.gitkeep`:
      `mkdir -p apps/crud-fe-dart-flutterweb/lib/presentation && touch apps/crud-fe-dart-flutterweb/lib/presentation/.gitkeep`
      — acceptance: `bash test -f apps/crud-fe-dart-flutterweb/lib/presentation/.gitkeep` exits 0.
  - _Suggested executor: `swe-dart-dev`_

### Phase 3 — Step 4: crud-fs-ts-nextjs layers

- [x] Create `apps/crud-fs-ts-nextjs/src/domain/.gitkeep`:
      `mkdir -p apps/crud-fs-ts-nextjs/src/domain && touch apps/crud-fs-ts-nextjs/src/domain/.gitkeep`
      — acceptance: `bash test -f apps/crud-fs-ts-nextjs/src/domain/.gitkeep` exits 0.

- [x] Create `apps/crud-fs-ts-nextjs/src/application/.gitkeep`:
      `mkdir -p apps/crud-fs-ts-nextjs/src/application && touch apps/crud-fs-ts-nextjs/src/application/.gitkeep`
      — acceptance: `bash test -f apps/crud-fs-ts-nextjs/src/application/.gitkeep` exits 0.

- [x] Create `apps/crud-fs-ts-nextjs/src/infrastructure/.gitkeep`:
      `mkdir -p apps/crud-fs-ts-nextjs/src/infrastructure && touch apps/crud-fs-ts-nextjs/src/infrastructure/.gitkeep`
      — acceptance: `bash test -f apps/crud-fs-ts-nextjs/src/infrastructure/.gitkeep` exits 0.

- [x] Create `apps/crud-fs-ts-nextjs/src/presentation/.gitkeep`:
      `mkdir -p apps/crud-fs-ts-nextjs/src/presentation && touch apps/crud-fs-ts-nextjs/src/presentation/.gitkeep`
      — acceptance: `bash test -f apps/crud-fs-ts-nextjs/src/presentation/.gitkeep` exits 0.
  - _Suggested executor: `swe-typescript-dev`_

### Phase 3 — Local Quality Gates

- [x] Run affected tests for FE apps:
      `npx nx run-many -t test:quick --projects=crud-fe-ts-nextjs,crud-fe-ts-tanstack-start,crud-fe-dart-flutterweb,crud-fs-ts-nextjs`
      — acceptance: exits 0; same pass count as baseline.
- [x] Run affected typecheck: `npx nx affected -t typecheck`
      — acceptance: exits 0.
- [x] Run affected lint: `npx nx affected -t lint`
      — acceptance: exits 0.
- [x] Run spec-coverage: `npx nx affected -t spec-coverage`
      — acceptance: exits 0; no new failures vs Phase 0 baseline.
- [x] Fix ALL failures.

### Phase 3 — Commit

- [x] Stage all Phase 3 `.gitkeep` files and commit:
      `git add apps/crud-fe-ts-nextjs/src/domain/.gitkeep apps/crud-fe-ts-nextjs/src/application/.gitkeep apps/crud-fe-ts-nextjs/src/infrastructure/.gitkeep apps/crud-fe-ts-nextjs/src/presentation/.gitkeep apps/crud-fe-ts-tanstack-start/src/domain/.gitkeep apps/crud-fe-ts-tanstack-start/src/application/.gitkeep apps/crud-fe-ts-tanstack-start/src/infrastructure/.gitkeep apps/crud-fe-ts-tanstack-start/src/presentation/.gitkeep apps/crud-fe-dart-flutterweb/lib/domain/.gitkeep apps/crud-fe-dart-flutterweb/lib/application/.gitkeep apps/crud-fe-dart-flutterweb/lib/infrastructure/.gitkeep apps/crud-fe-dart-flutterweb/lib/presentation/.gitkeep apps/crud-fs-ts-nextjs/src/domain/.gitkeep apps/crud-fs-ts-nextjs/src/application/.gitkeep apps/crud-fs-ts-nextjs/src/infrastructure/.gitkeep apps/crud-fs-ts-nextjs/src/presentation/.gitkeep`
      then commit:
      `feat(crud-fe): add canonical hexagonal layer directories to all FE apps`
      — acceptance: `git status` clean for those files.

### Phase 3 — Post-Push CI Verification

- [x] Push: `git push origin HEAD:main`
- [x] Monitor these workflows: `pr-quality-gate`, `test-crud-fe-ts-nextjs`,
      `test-crud-fe-ts-tanstack-start`, `test-crud-fe-dart-flutterweb`, `test-crud-fs-ts-nextjs`.
- [x] Verify ALL CI checks pass.
- [x] Fix immediately if any fail.
- [x] Do NOT proceed to Phase 4 until CI is fully green.

---

## Phase 4: BE App Bounded-Context Directories

Add bounded-context hexagonal directories to all 11 BE apps. Single context name:
`expenses`. No existing code is moved — directories only.

> **Important**: Fix ALL failures found during quality gates.
>
> **Note on two-level HTTP structure**: For all BE apps except Elixir, the outermost inbound
> adapter is `api/http/` — two directory levels. Create both `api/` (with its own `.gitkeep`
> or `mod.rs` in Rust) and `api/http/` (with `.gitkeep`). For Elixir, `lib/crud_be_exph_web/`
> already exists and IS the HTTP adapter layer; no new `api/http/` directory is created there.

### Phase 4 — Step 1: crud-be-rust-axum

All four directories under `src/contexts/expenses/`. Note Rust requires `mod.rs` stubs at
`api/` and `api/http/` for the compiler; use `.gitkeep` at all other levels.

- [x] `mkdir -p apps/crud-be-rust-axum/src/contexts/expenses/domain && touch apps/crud-be-rust-axum/src/contexts/expenses/domain/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-rust-axum/src/contexts/expenses/domain/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-rust-axum/src/contexts/expenses/application && touch apps/crud-be-rust-axum/src/contexts/expenses/application/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-rust-axum/src/contexts/expenses/application/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-rust-axum/src/contexts/expenses/infrastructure && touch apps/crud-be-rust-axum/src/contexts/expenses/infrastructure/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-rust-axum/src/contexts/expenses/infrastructure/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-rust-axum/src/contexts/expenses/api && touch apps/crud-be-rust-axum/src/contexts/expenses/api/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-rust-axum/src/contexts/expenses/api/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-rust-axum/src/contexts/expenses/api/http && touch apps/crud-be-rust-axum/src/contexts/expenses/api/http/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-rust-axum/src/contexts/expenses/api/http/.gitkeep` exits 0.
- [x] Run `npx nx run crud-be-rust-axum:test:quick` — acceptance: exits 0, same pass count as baseline.
  - _Suggested executor: `swe-rust-dev`_

### Phase 4 — Step 2: crud-be-golang-gin

All four paths under `internal/contexts/expenses/`.

- [x] `mkdir -p apps/crud-be-golang-gin/internal/contexts/expenses/domain && touch apps/crud-be-golang-gin/internal/contexts/expenses/domain/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-golang-gin/internal/contexts/expenses/domain/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-golang-gin/internal/contexts/expenses/application && touch apps/crud-be-golang-gin/internal/contexts/expenses/application/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-golang-gin/internal/contexts/expenses/application/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-golang-gin/internal/contexts/expenses/infrastructure && touch apps/crud-be-golang-gin/internal/contexts/expenses/infrastructure/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-golang-gin/internal/contexts/expenses/infrastructure/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-golang-gin/internal/contexts/expenses/api && touch apps/crud-be-golang-gin/internal/contexts/expenses/api/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-golang-gin/internal/contexts/expenses/api/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-golang-gin/internal/contexts/expenses/api/http && touch apps/crud-be-golang-gin/internal/contexts/expenses/api/http/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-golang-gin/internal/contexts/expenses/api/http/.gitkeep` exits 0.
- [x] Run `npx nx run crud-be-golang-gin:test:quick` — acceptance: exits 0, same pass count as baseline.
  - _Suggested executor: `swe-golang-dev`_

### Phase 4 — Step 3: crud-be-fsharp-giraffe

All paths under `apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/` using PascalCase.

- [x] `mkdir -p "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Domain" && touch "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Domain/.gitkeep"`
      — acceptance: `bash test -f "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Domain/.gitkeep"` exits 0.
- [x] `mkdir -p "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Application" && touch "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Application/.gitkeep"`
      — acceptance: `bash test -f "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Application/.gitkeep"` exits 0.
- [x] `mkdir -p "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Infrastructure" && touch "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Infrastructure/.gitkeep"`
      — acceptance: `bash test -f "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Infrastructure/.gitkeep"` exits 0.
- [x] `mkdir -p "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Api" && touch "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Api/.gitkeep"`
      — acceptance: `bash test -f "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Api/.gitkeep"` exits 0.
- [x] `mkdir -p "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Api/Http" && touch "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Api/Http/.gitkeep"`
      — acceptance: `bash test -f "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Api/Http/.gitkeep"` exits 0.
- [x] Run `npx nx run crud-be-fsharp-giraffe:test:quick` — acceptance: exits 0, same pass count as baseline.
  - _Suggested executor: `swe-fsharp-dev`_

### Phase 4 — Step 4: crud-be-ts-effect

All paths under `apps/crud-be-ts-effect/src/`.

- [x] `mkdir -p apps/crud-be-ts-effect/src/contexts/expenses/domain && touch apps/crud-be-ts-effect/src/contexts/expenses/domain/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-ts-effect/src/contexts/expenses/domain/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-ts-effect/src/contexts/expenses/application && touch apps/crud-be-ts-effect/src/contexts/expenses/application/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-ts-effect/src/contexts/expenses/application/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-ts-effect/src/contexts/expenses/infrastructure && touch apps/crud-be-ts-effect/src/contexts/expenses/infrastructure/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-ts-effect/src/contexts/expenses/infrastructure/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-ts-effect/src/contexts/expenses/api && touch apps/crud-be-ts-effect/src/contexts/expenses/api/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-ts-effect/src/contexts/expenses/api/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-ts-effect/src/contexts/expenses/api/http && touch apps/crud-be-ts-effect/src/contexts/expenses/api/http/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-ts-effect/src/contexts/expenses/api/http/.gitkeep` exits 0.
- [x] Run `npx nx run crud-be-ts-effect:test:quick` — acceptance: exits 0, same pass count as baseline.
  - _Suggested executor: `swe-typescript-dev`_

### Phase 4 — Step 5: crud-be-python-fastapi

All paths under `apps/crud-be-python-fastapi/src/crud_be_python_fastapi/`.

- [x] `mkdir -p apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/domain && touch apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/domain/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/domain/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/application && touch apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/application/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/application/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/infrastructure && touch apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/infrastructure/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/infrastructure/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/api && touch apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/api/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/api/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/api/http && touch apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/api/http/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-python-fastapi/src/crud_be_python_fastapi/contexts/expenses/api/http/.gitkeep` exits 0.
- [x] Run `npx nx run crud-be-python-fastapi:test:quick` — acceptance: exits 0, same pass count as baseline.
  - _Suggested executor: `swe-python-dev`_

### Phase 4 — Step 6: crud-be-clojure-pedestal

All paths under `apps/crud-be-clojure-pedestal/src/crud_be_cjpd/`.

- [x] `mkdir -p apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/domain && touch apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/domain/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/domain/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/application && touch apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/application/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/application/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/infrastructure && touch apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/infrastructure/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/infrastructure/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/api && touch apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/api/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/api/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/api/http && touch apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/api/http/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-clojure-pedestal/src/crud_be_cjpd/contexts/expenses/api/http/.gitkeep` exits 0.
- [x] Run `npx nx run crud-be-clojure-pedestal:test:quick` — acceptance: exits 0, same pass count as baseline.
  - _Suggested executor: `swe-clojure-dev`_

### Phase 4 — Step 7: crud-be-java-vertx

All paths under `apps/crud-be-java-vertx/src/main/java/com/demobejavx/`.

- [x] `mkdir -p apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/domain && touch apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/domain/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/domain/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/application && touch apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/application/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/application/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/infrastructure && touch apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/infrastructure/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/infrastructure/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/api && touch apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/api/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/api/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/api/http && touch apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/api/http/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-java-vertx/src/main/java/com/demobejavx/contexts/expenses/api/http/.gitkeep` exits 0.
- [x] Run `npx nx run crud-be-java-vertx:test:quick` — acceptance: exits 0, same pass count as baseline.
  - _Suggested executor: `swe-java-dev`_

### Phase 4 — Step 8: crud-be-java-springboot

All paths under `apps/crud-be-java-springboot/src/main/java/com/demobejasb/`.

- [x] `mkdir -p apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/domain && touch apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/domain/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/domain/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/application && touch apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/application/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/application/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/infrastructure && touch apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/infrastructure/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/infrastructure/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/api && touch apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/api/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/api/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/api/http && touch apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/api/http/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-java-springboot/src/main/java/com/demobejasb/contexts/expenses/api/http/.gitkeep` exits 0.
- [x] Run `npx nx run crud-be-java-springboot:test:quick` — acceptance: exits 0, same pass count as baseline.
  - _Suggested executor: `swe-java-dev`_

### Phase 4 — Step 9: crud-be-kotlin-ktor

All paths under `apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/`.

- [x] `mkdir -p apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/domain && touch apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/domain/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/domain/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/application && touch apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/application/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/application/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/infrastructure && touch apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/infrastructure/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/infrastructure/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/api && touch apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/api/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/api/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/api/http && touch apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/api/http/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/contexts/expenses/api/http/.gitkeep` exits 0.
- [x] Run `npx nx run crud-be-kotlin-ktor:test:quick` — acceptance: exits 0, same pass count as baseline.
  - _Suggested executor: `swe-kotlin-dev`_

### Phase 4 — Step 10: crud-be-elixir-phoenix

Inner DDD layers under `apps/crud-be-elixir-phoenix/lib/crud_be_exph/contexts/expenses/`.
The `lib/crud_be_exph_web/` directory already exists and IS the HTTP adapter layer — no
`api/http/` directory is created.

- [x] `mkdir -p apps/crud-be-elixir-phoenix/lib/crud_be_exph/contexts/expenses/domain && touch apps/crud-be-elixir-phoenix/lib/crud_be_exph/contexts/expenses/domain/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-elixir-phoenix/lib/crud_be_exph/contexts/expenses/domain/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-elixir-phoenix/lib/crud_be_exph/contexts/expenses/application && touch apps/crud-be-elixir-phoenix/lib/crud_be_exph/contexts/expenses/application/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-elixir-phoenix/lib/crud_be_exph/contexts/expenses/application/.gitkeep` exits 0.
- [x] `mkdir -p apps/crud-be-elixir-phoenix/lib/crud_be_exph/contexts/expenses/infrastructure && touch apps/crud-be-elixir-phoenix/lib/crud_be_exph/contexts/expenses/infrastructure/.gitkeep`
      — acceptance: `bash test -f apps/crud-be-elixir-phoenix/lib/crud_be_exph/contexts/expenses/infrastructure/.gitkeep` exits 0.
- [x] Verify `apps/crud-be-elixir-phoenix/lib/crud_be_exph_web/` exists as the HTTP adapter:
      `bash test -d apps/crud-be-elixir-phoenix/lib/crud_be_exph_web`
      — acceptance: exits 0. [Repo-grounded: `lib/crud_be_exph_web/` exists]
- [x] Run `npx nx run crud-be-elixir-phoenix:test:quick` — acceptance: exits 0, same pass count as baseline.
  - _Suggested executor: `swe-elixir-dev`_

### Phase 4 — Step 11: crud-be-csharp-aspnetcore

All paths under `apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/` using PascalCase.

- [x] `mkdir -p "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Domain" && touch "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Domain/.gitkeep"`
      — acceptance: `bash test -f "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Domain/.gitkeep"` exits 0.
- [x] `mkdir -p "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Application" && touch "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Application/.gitkeep"`
      — acceptance: `bash test -f "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Application/.gitkeep"` exits 0.
- [x] `mkdir -p "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Infrastructure" && touch "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Infrastructure/.gitkeep"`
      — acceptance: `bash test -f "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Infrastructure/.gitkeep"` exits 0.
- [x] `mkdir -p "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Api" && touch "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Api/.gitkeep"`
      — acceptance: `bash test -f "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Api/.gitkeep"` exits 0.
- [x] `mkdir -p "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Api/Http" && touch "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Api/Http/.gitkeep"`
      — acceptance: `bash test -f "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Api/Http/.gitkeep"` exits 0.
- [x] Run `npx nx run crud-be-csharp-aspnetcore:test:quick` — acceptance: exits 0, same pass count as baseline.
  - _Suggested executor: `swe-csharp-dev`_

### Phase 4 — Local Quality Gates

- [x] Run full BE test suite to verify zero regressions:
      `npx nx run-many -t test:quick --projects=crud-be-rust-axum,crud-be-golang-gin,crud-be-fsharp-giraffe,crud-be-ts-effect,crud-be-python-fastapi,crud-be-clojure-pedestal,crud-be-java-vertx,crud-be-java-springboot,crud-be-kotlin-ktor,crud-be-elixir-phoenix,crud-be-csharp-aspnetcore`
      — acceptance: exits 0; same pass count as Phase 0 baseline.
- [x] Run affected typecheck: `npx nx affected -t typecheck`
      — acceptance: exits 0.
- [x] Run affected lint: `npx nx affected -t lint`
      — acceptance: exits 0.
- [x] Run spec-coverage: `npx nx affected -t spec-coverage`
      — acceptance: exits 0; no new failures vs Phase 0 baseline.
- [x] Fix ALL failures.

### Phase 4 — Commit

- [x] Stage all Phase 4 `.gitkeep` files with `git add apps/crud-be-*/` and commit:
      `feat(crud-be): add bounded-context hexagonal layer directories to all 11 BE apps`
      — acceptance: `git status` shows all Phase 4 `.gitkeep` files tracked.

### Phase 4 — Post-Push CI Verification

- [x] Push: `git push origin HEAD:main`
- [x] Monitor these workflows: `pr-quality-gate`, `test-crud-be-rust-axum`, `test-crud-be-golang-gin`,
      `test-crud-be-fsharp-giraffe`, `test-crud-be-ts-effect`, `test-crud-be-python-fastapi`,
      `test-crud-be-clojure-pedestal`, `test-crud-be-java-vertx`, `test-crud-be-java-springboot`,
      `test-crud-be-kotlin-ktor`, `test-crud-be-elixir-phoenix`, `test-crud-be-csharp-aspnetcore`.
- [x] Verify ALL CI checks pass.
- [x] Fix immediately if any fail.
- [x] Do NOT proceed to Phase 5 until CI is fully green.

---

## Phase 5: OpenAPI Contract Infrastructure Verification

Verify the existing `codegen` Nx target for each BE app, document the wiring in
`openapi-contract-first.md` (already created in Phase 1), and confirm FE consumer
codegen targets work.

> **Important**: Fix ALL failures found during verification.

### Phase 5 — Step 1: Verify BE codegen targets

For each of the 11 BE apps, confirm the `codegen` target is present in `project.json` and
runs successfully.

- [x] Verify `crud-be-rust-axum` codegen target exists:
      `cat apps/crud-be-rust-axum/project.json | python3 -c "import json,sys; d=json.load(sys.stdin); assert 'codegen' in d['targets'], 'codegen missing'"`
      — acceptance: exits 0 (no assertion error). [Repo-grounded: codegen target confirmed]

- [x] Verify `crud-be-golang-gin` codegen target exists:
      `cat apps/crud-be-golang-gin/project.json | python3 -c "import json,sys; d=json.load(sys.stdin); assert 'codegen' in d['targets'], 'codegen missing'"`
      — acceptance: exits 0. [Repo-grounded: codegen target confirmed]

- [x] For each of the remaining 9 BE apps, run the same one-liner (replacing the app name):
      `crud-be-fsharp-giraffe`, `crud-be-ts-effect`, `crud-be-python-fastapi`,
      `crud-be-clojure-pedestal`, `crud-be-java-vertx`, `crud-be-java-springboot`,
      `crud-be-kotlin-ktor`, `crud-be-elixir-phoenix`, `crud-be-csharp-aspnetcore`
      — acceptance: all 9 exit 0.

### Phase 5 — Step 2: Verify FE consumer codegen targets

- [x] Verify `crud-fe-ts-nextjs` codegen target exists and `generated-contracts/` is populated:
      `cat apps/crud-fe-ts-nextjs/project.json | python3 -c "import json,sys; d=json.load(sys.stdin); assert 'codegen' in d['targets'], 'codegen missing'"` AND
      `bash test -d apps/crud-fe-ts-nextjs/src/generated-contracts`
      — acceptance: both exit 0. [Repo-grounded: directory exists]

- [x] Verify `crud-fe-ts-tanstack-start` codegen target exists and `generated-contracts/` is populated:
      `cat apps/crud-fe-ts-tanstack-start/project.json | python3 -c "import json,sys; d=json.load(sys.stdin); assert 'codegen' in d['targets'], 'codegen missing'"` AND
      `bash test -d apps/crud-fe-ts-tanstack-start/src/generated-contracts`
      — acceptance: both exit 0. [Repo-grounded: directory exists]

- [x] Verify `crud-fe-dart-flutterweb` codegen target exists:
      `cat apps/crud-fe-dart-flutterweb/project.json | python3 -c "import json,sys; d=json.load(sys.stdin); assert 'codegen' in d['targets'], 'codegen missing'"`
      — acceptance: exits 0. If the target is absent, add it following the pattern in
      `apps/crud-fe-ts-nextjs/project.json` using the Dart OpenAPI generator.

### Phase 5 — Step 3: Smoke-test bundled spec exists

- [x] Verify the canonical OpenAPI bundled artifact is present:
      `bash test -f specs/apps/crud/containers/contracts/generated/openapi-bundled.yaml`
      — acceptance: exits 0. [Repo-grounded: file exists]

- [x] If the bundled file is stale, regenerate it:
      `npx nx run crud-contracts:bundle`
      — acceptance: exits 0; `git diff --quiet specs/apps/crud/containers/contracts/generated/`
      exits 0 (no drift).

### Phase 5 — Local Quality Gates

- [x] Run affected tests: `npx nx affected -t test:quick`
      — acceptance: exits 0; same pass count as baseline.
- [x] Run affected lint: `npx nx affected -t lint`
      — acceptance: exits 0.
- [x] Run spec-coverage: `npx nx affected -t spec-coverage`
      — acceptance: exits 0; no new failures vs Phase 0 baseline.
- [x] Fix ALL failures.

### Phase 5 — Commit

- [x] If any `project.json` was updated in Phase 5, commit:
      `feat(crud-be): verify and document openapi codegen targets for all 11 BE apps`
      — acceptance: `git status` shows only expected project.json changes.

### Phase 5 — Post-Push CI Verification

- [x] Push: `git push origin HEAD:main`
- [x] Monitor these workflows: `pr-quality-gate`, all 11 `test-crud-be-*` workflows,
      `test-crud-fe-dart-flutterweb` (if codegen wiring was added).
- [x] Verify ALL CI checks pass — no exceptions.
- [x] Fix immediately if any fail.
- [x] Do NOT proceed to Plan Archival until CI is fully green.

---

## Final Acceptance Verification

Before archiving, run the complete end-to-end acceptance check.

### All-app test run (zero regressions)

- [x] Run `npx nx run-many -t test:quick --projects=rhino-cli-rust,rhino-cli-go,crud-fe-ts-nextjs,crud-fe-ts-tanstack-start,crud-fe-dart-flutterweb,crud-fs-ts-nextjs,crud-be-rust-axum,crud-be-golang-gin,crud-be-fsharp-giraffe,crud-be-ts-effect,crud-be-python-fastapi,crud-be-clojure-pedestal,crud-be-java-vertx,crud-be-java-springboot,crud-be-kotlin-ktor,crud-be-elixir-phoenix,crud-be-csharp-aspnetcore`
      — acceptance: exits 0; same pass count as Phase 0 baseline; no new failures introduced.

### Governance documents exist

- [x] `bash test -f repo-governance/development/pattern/hexagonal-architecture.md` exits 0.
- [x] `bash test -f repo-governance/development/pattern/hexagonal-architecture-cli.md` exits 0.
- [x] `bash test -f repo-governance/development/pattern/hexagonal-architecture-web.md` exits 0.
- [x] `bash test -f repo-governance/development/pattern/hexagonal-architecture-be.md` exits 0.
- [x] `bash test -f repo-governance/development/pattern/openapi-contract-first.md` exits 0.

### Layer directories exist (sampling)

- [x] `bash test -d apps/rhino-cli-rust/src/domain` exits 0.
- [x] `bash test -d apps/rhino-cli-go/internal/adapter/command` exits 0.
- [x] `bash test -d apps/crud-fe-ts-nextjs/src/presentation` exits 0.
- [x] `bash test -d apps/crud-fe-dart-flutterweb/lib/presentation` exits 0.
- [x] `bash test -d apps/crud-be-rust-axum/src/contexts/expenses/api/http` exits 0.
- [x] `bash test -d apps/crud-be-golang-gin/internal/contexts/expenses/api/http` exits 0.
- [x] `bash test -d "apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/Contexts/Expenses/Api/Http"` exits 0.
- [x] `bash test -d apps/crud-be-elixir-phoenix/lib/crud_be_exph/contexts/expenses/domain` exits 0.
- [x] `bash test -d "apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/Contexts/Expenses/Api/Http"` exits 0.

---

## Plan Archival

- [x] Verify ALL delivery checklist items above are ticked.
- [x] Verify ALL quality gates pass (local + CI).
- [x] Rename and move:
      `git mv plans/in-progress/adopt-hexagonal-ddd-architecture plans/done/$(date +%Y-%m-%d)__adopt-hexagonal-ddd-architecture`
      — acceptance: folder appears under `plans/done/` with today's date prefix.
- [x] Update `plans/in-progress/README.md` — remove the `adopt-hexagonal-ddd-architecture`
      entry from the Active Plans list.
- [x] Update `plans/done/README.md` — add an entry for `adopt-hexagonal-ddd-architecture`
      with the completion date and a one-line summary.
- [x] Update `plans/README.md` if it lists in-progress plans (check and update if present).
- [x] Commit:
      `chore(plans): move adopt-hexagonal-ddd-architecture to done`
      — acceptance: `git status` clean; plan folder visible under `plans/done/`.
- [x] Push: `git push origin HEAD:main`
