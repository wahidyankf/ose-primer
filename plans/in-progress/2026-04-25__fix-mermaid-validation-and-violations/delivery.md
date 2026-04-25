# Delivery — Fix Mermaid Validation and Violations

## Environment Setup

- [ ] Install dependencies: `npm install`
- [ ] Converge the full polyglot toolchain: `npm run doctor -- --fix` (required — the
      `postinstall` hook runs `doctor || true` and silently tolerates drift)
- [ ] Verify Go is available: `go version` (rhino-cli requires Go to run the validator)
- [ ] Verify existing tests pass before making changes:
      `npx nx affected -t test:quick`

## Pre-Work

- [ ] Confirm baseline: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep -c "^\(✗\|⚠\)"` — note
      the returned number is the count of violation/warning **lines** (not affected files);
      expected to be approximately 247 lines (179 width_exceeded + 56 label_too_long +
      12 complex_diagram). The README.md reports 107 **files** affected, which is a different count.
- [ ] Save full audit to `local-temp/mermaid-audit-baseline.txt` for reference during fixes

## Phase 0 — Direction-Aware Validator + Threshold Update (rhino-cli)

### 0a — `validator.go` changes

- [ ] Add `import "math"` to `validator.go`
- [ ] Change `DefaultValidateOptions()`: `MaxWidth: 3 → 4`, `MaxDepth: 5 → math.MaxInt`
- [ ] Add direction-aware `horizontal`/`vertical` selection in `ValidateBlocks` per
      tech-docs.md Phase 0 spec (switch on `diagram.Direction`, LR/RL → swap axes)

### 0b — `validator_test.go` changes

- [ ] Fix 3 tests broken by MaxWidth 3→4 (see tech-docs.md "Existing tests that break"):
  - `"width at limit+1 violation"` → switch to `span5depth3Source`
  - `"both exceeded warning only"` → use explicit `ValidateOptions{MaxWidth:3,MaxDepth:5}`
  - `"width only exceeded violation"` → use explicit opts or span=5 source
- [ ] Add `"width exactly at new limit 4 no violation"` (span=4, TD, defaultOpts)
- [ ] Add new LR direction test cases (6 cases per table in tech-docs.md Phase 0)
- [ ] Add new test sources: `span5depth3Source`, `lrDepth6Span2Source`,
      `lrSpan5Depth2Source`, `lrDepth4Span6Source`

### 0c — `docs_validate_mermaid.go` CLI flag defaults

- [ ] Change `--max-width` default: `3 → 4`
- [ ] Change `--max-depth` default: `5 → 0` (document: 0 = no limit)
- [ ] In `runValidateMermaid`: map `MaxDepth ≤ 0 → math.MaxInt` before building opts

### 0d — Verify and commit

- [ ] Run: `npx nx run rhino-cli:lint` → must pass
- [ ] Run: `npx nx run rhino-cli:typecheck` → must pass
- [ ] Run: `npx nx run rhino-cli:test:unit` → must pass
- [ ] Run: `npx nx run rhino-cli:test:quick` → must pass (coverage ≥ 90%)
- [ ] Commit: `fix(rhino-cli): direction-aware width check, MaxWidth=4, no depth limit`
- [ ] Re-audit: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | tee local-temp/mermaid-audit-phase0.txt`
- [ ] Update Phase 1 batch file lists from the re-audit output
      (LR-only-tall files may disappear; new LR-deeply-chained files may appear;
      diagrams with old span=4 violations now pass)
- [ ] Update the Violation Summary table in README.md with new counts

## Batch 1 — TypeScript (18 files)

Files: `docs/explanation/software-engineering/programming-languages/typescript/`

- [ ] Fix `typescript/README.md`
- [ ] Fix `typescript/anti-patterns.md`
- [ ] Fix `typescript/best-practices.md`
- [ ] Fix `typescript/concurrency-and-parallelism.md`
- [ ] Fix `typescript/domain-driven-design.md`
- [ ] Fix `typescript/error-handling.md`
- [ ] Fix `typescript/finite-state-machine.md`
- [ ] Fix `typescript/functional-programming.md`
- [ ] Fix `typescript/idioms.md`
- [ ] Fix `typescript/interfaces-and-types.md`
- [ ] Fix `typescript/linting-and-formatting.md`
- [ ] Fix `typescript/memory-management.md`
- [ ] Fix `typescript/modules-and-dependencies.md`
- [ ] Fix `typescript/performance.md`
- [ ] Fix `typescript/security.md`
- [ ] Fix `typescript/test-driven-development.md`
- [ ] Fix `typescript/type-safety.md`
- [ ] Fix `typescript/web-services.md`
- [ ] Validate: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep "typescript/"` → no output
- [ ] Commit: `fix(docs): fix mermaid violations in typescript/ docs (batch 1/10)`

## Batch 2 — Python (15 files)

Files: `docs/explanation/software-engineering/programming-languages/python/`

- [ ] Fix `python/README.md`
- [ ] Fix `python/anti-patterns.md`
- [ ] Fix `python/best-practices.md`
- [ ] Fix `python/classes-and-protocols.md`
- [ ] Fix `python/concurrency-and-parallelism.md`
- [ ] Fix `python/domain-driven-design.md`
- [ ] Fix `python/error-handling.md`
- [ ] Fix `python/finite-state-machine.md`
- [ ] Fix `python/idioms.md`
- [ ] Fix `python/linting-and-formatting.md`
- [ ] Fix `python/modules-and-dependencies.md`
- [ ] Fix `python/performance.md`
- [ ] Fix `python/security.md`
- [ ] Fix `python/test-driven-development.md`
- [ ] Fix `python/web-services.md`
- [ ] Validate: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep "python/"` → no output
- [ ] Commit: `fix(docs): fix mermaid violations in python/ docs (batch 2/10)`

## Batch 3 — Go (11 files)

Files: `docs/explanation/software-engineering/programming-languages/golang/`

- [ ] Fix `golang/README.md`
- [ ] Fix `golang/api-standards.md`
- [ ] Fix `golang/code-quality-standards.md`
- [ ] Fix `golang/concurrency-standards.md`
- [ ] Fix `golang/ddd-standards.md`
- [ ] Fix `golang/dependency-standards.md`
- [ ] Fix `golang/design-patterns.md`
- [ ] Fix `golang/error-handling-standards.md`
- [ ] Fix `golang/performance-standards.md`
- [ ] Fix `golang/security-standards.md`
- [ ] Fix `golang/type-safety-standards.md`
- [ ] Validate: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep "golang/"` → no output
- [ ] Commit: `fix(docs): fix mermaid violations in golang/ docs (batch 3/10)`

## Batch 4 — JVM Spring Boot (10 files)

Files: `docs/explanation/software-engineering/platform-web/tools/jvm-spring-boot/`

- [ ] Fix `jvm-spring-boot/README.md`
- [ ] Fix `jvm-spring-boot/configuration.md`
- [ ] Fix `jvm-spring-boot/data-access.md`
- [ ] Fix `jvm-spring-boot/dependency-injection.md`
- [ ] Fix `jvm-spring-boot/domain-driven-design.md`
- [ ] Fix `jvm-spring-boot/observability.md`
- [ ] Fix `jvm-spring-boot/performance.md`
- [ ] Fix `jvm-spring-boot/rest-apis.md`
- [ ] Fix `jvm-spring-boot/security.md`
- [ ] Fix `jvm-spring-boot/testing.md`
- [ ] Validate: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep "jvm-spring-boot/"` → no output
- [ ] Commit: `fix(docs): fix mermaid violations in jvm-spring-boot/ docs (batch 4/10)`

## Batch 5 — Elixir Phoenix (8 files)

Files: `docs/explanation/software-engineering/platform-web/tools/elixir-phoenix/`

- [ ] Fix `elixir-phoenix/channels.md`
- [ ] Fix `elixir-phoenix/contexts.md`
- [ ] Fix `elixir-phoenix/data-access.md`
- [ ] Fix `elixir-phoenix/deployment.md`
- [ ] Fix `elixir-phoenix/liveview.md`
- [ ] Fix `elixir-phoenix/observability.md`
- [ ] Fix `elixir-phoenix/performance.md`
- [ ] Fix `elixir-phoenix/testing.md`
- [ ] Validate: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep "elixir-phoenix/"` → no output
- [ ] Commit: `fix(docs): fix mermaid violations in elixir-phoenix/ docs (batch 5/10)`

## Batch 6 — React (8 files)

Files: `docs/explanation/software-engineering/platform-web/tools/fe-react/`

- [ ] Fix `fe-react/README.md`
- [ ] Fix `fe-react/component-architecture.md`
- [ ] Fix `fe-react/data-fetching.md`
- [ ] Fix `fe-react/hooks.md`
- [ ] Fix `fe-react/performance.md`
- [ ] Fix `fe-react/routing.md`
- [ ] Fix `fe-react/security.md`
- [ ] Fix `fe-react/state-management.md`
- [ ] Validate: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep "fe-react/"` → no output
- [ ] Commit: `fix(docs): fix mermaid violations in fe-react/ docs (batch 6/10)`

## Batch 7 — Next.js (6 files)

Files: `docs/explanation/software-engineering/platform-web/tools/fe-nextjs/`

- [ ] Fix `fe-nextjs/README.md`
- [ ] Fix `fe-nextjs/app-router.md`
- [ ] Fix `fe-nextjs/data-fetching.md`
- [ ] Fix `fe-nextjs/middleware.md`
- [ ] Fix `fe-nextjs/performance.md`
- [ ] Fix `fe-nextjs/rendering.md`
- [ ] Validate: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep "fe-nextjs/"` → no output
- [ ] Commit: `fix(docs): fix mermaid violations in fe-nextjs/ docs (batch 7/10)`

## Batch 8 — Elixir Language (6 files)

Files: `docs/explanation/software-engineering/programming-languages/elixir/`

- [ ] Fix `elixir/README.md`
- [ ] Fix `elixir/ddd-standards.md`
- [ ] Fix `elixir/otp-application.md`
- [ ] Fix `elixir/otp-genserver.md`
- [ ] Fix `elixir/otp-supervisor.md`
- [ ] Fix `elixir/protocols-behaviours-standards.md`
- [ ] Validate: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep "programming-languages/elixir/"` → no output
- [ ] Commit: `fix(docs): fix mermaid violations in elixir/ docs (batch 8/10)`

## Batch 9 — C4 Architecture (5 files)

Files: `docs/explanation/software-engineering/architecture/c4-architecture-model/`

- [ ] Fix `c4-architecture-model/README.md`
- [ ] Fix `c4-architecture-model/bounded-context-visualization.md`
- [ ] Fix `c4-architecture-model/diagram-standards.md`
- [ ] Fix `c4-architecture-model/notation-standards.md`
- [ ] Fix `c4-architecture-model/nx-workspace-visualization.md`
- [ ] Validate: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep "c4-architecture-model/"` → no output
- [ ] Commit: `fix(docs): fix mermaid violations in c4-architecture-model/ docs (batch 9/10)`

## Batch 10 — Remaining errors (14 files)

- [ ] Fix `docs/explanation/software-engineering/programming-languages/c-sharp/README.md`
- [ ] Fix `docs/explanation/software-engineering/programming-languages/clojure/README.md`
- [ ] Fix `docs/explanation/software-engineering/programming-languages/f-sharp/README.md`
- [ ] Fix `docs/explanation/software-engineering/programming-languages/java/README.md`
- [ ] Fix `docs/explanation/software-engineering/programming-languages/kotlin/README.md`
- [ ] Fix `docs/explanation/software-engineering/programming-languages/rust/README.md`
- [ ] Fix `docs/explanation/software-engineering/platform-web/tools/jvm-spring/README.md`
- [ ] Fix `docs/explanation/software-engineering/platform-web/tools/jvm-spring/web-mvc.md`
- [ ] Fix `docs/explanation/software-engineering/development/README.md`
- [ ] Fix `docs/how-to/organize-work.md`
- [ ] Fix `docs/reference/system-architecture/README.md`
- [ ] Fix `docs/reference/system-architecture/applications.md`
- [ ] Fix `docs/reference/system-architecture/components.md`
- [ ] Fix `docs/reference/system-architecture/deployment.md`
- [ ] Validate: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep -E "(c-sharp|clojure|f-sharp|java/|kotlin|rust/|jvm-spring/|organize-work|system-architecture|software-engineering/development)"` → no output
- [ ] Commit: `fix(docs): fix mermaid violations in remaining docs (batch 10/10)`

## Final Validation

> **Important**: Fix ALL failures found during quality gates, not just those caused by
> your changes. This follows the root cause orientation principle — proactively fix
> preexisting errors encountered during work. Do not defer or mention-and-skip existing
> issues.

- [ ] Run full validator: `go run ./apps/rhino-cli/main.go docs validate-mermaid`
- [ ] Confirm zero lines starting with `✗` (warnings tolerated)
- [ ] Confirm `[width_exceeded]` count = 0
- [ ] Confirm `[label_too_long]` count = 0
- [ ] Confirm prd.md Scenarios 1–4 (direction-aware discrimination) are covered by new
      test cases in `validator_test.go`
- [ ] Run affected quality gates: `npx nx affected -t typecheck lint test:quick spec-coverage`
- [ ] Fix ALL failures found — including preexisting issues not caused by your changes

### Commit Guidelines

- Follow Conventional Commits format: `<type>(<scope>): <description>`
- Keep diagram fixes in `fix(docs):` commits; stage and commit separately any
  unrelated preexisting fixes discovered during the batch work
- Do not bundle unrelated concerns in a single commit
- One commit per batch is the default; split only if unrelated changes exist

### Post-Push Verification

- [ ] Push changes to `main`
- [ ] ose-primer has no GitHub Actions CI — verify locally after push:
  - `npm run lint:md` (markdown linting)
  - `npx nx affected -t typecheck lint test:quick spec-coverage`
- [ ] Confirm no regressions introduced by diagram changes before closing the plan

## Plan Archival

- [ ] Verify ALL delivery checklist items are ticked
- [ ] Verify ALL quality gates pass (local)
- [ ] Move `plans/in-progress/2026-04-25__fix-mermaid-validation-and-violations/` → `plans/done/` via `git mv`
- [ ] Update `plans/in-progress/README.md` — remove entry
- [ ] Update `plans/done/README.md` — add entry with completion date
- [ ] Commit: `chore(plans): move fix-mermaid-validation-and-violations to done`
