# Delivery — Fix Mermaid Violations

## Pre-Work

- [ ] Confirm baseline: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep -c "^\(✗\|⚠\)"` returns 107
- [ ] Save full audit to `local-temp/mermaid-audit-baseline.txt` for reference during fixes

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
- [ ] Commit: `fix(docs): fix mermaid violations in typescript/ docs (batch 1/11)`

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
- [ ] Commit: `fix(docs): fix mermaid violations in python/ docs (batch 2/11)`

## Batch 3 — Go (10 files)

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
- [ ] Commit: `fix(docs): fix mermaid violations in golang/ docs (batch 3/11)`

## Batch 4 — JVM Spring Boot (9 files)

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
- [ ] Commit: `fix(docs): fix mermaid violations in jvm-spring-boot/ docs (batch 4/11)`

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
- [ ] Commit: `fix(docs): fix mermaid violations in elixir-phoenix/ docs (batch 5/11)`

## Batch 6 — React (7 files)

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
- [ ] Commit: `fix(docs): fix mermaid violations in fe-react/ docs (batch 6/11)`

## Batch 7 — Next.js (6 files)

Files: `docs/explanation/software-engineering/platform-web/tools/fe-nextjs/`

- [ ] Fix `fe-nextjs/README.md`
- [ ] Fix `fe-nextjs/app-router.md`
- [ ] Fix `fe-nextjs/data-fetching.md`
- [ ] Fix `fe-nextjs/middleware.md`
- [ ] Fix `fe-nextjs/performance.md`
- [ ] Fix `fe-nextjs/rendering.md`
- [ ] Validate: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep "fe-nextjs/"` → no output
- [ ] Commit: `fix(docs): fix mermaid violations in fe-nextjs/ docs (batch 7/11)`

## Batch 8 — Elixir Language (5 files)

Files: `docs/explanation/software-engineering/programming-languages/elixir/`

- [ ] Fix `elixir/README.md`
- [ ] Fix `elixir/ddd-standards.md`
- [ ] Fix `elixir/otp-application.md`
- [ ] Fix `elixir/otp-genserver.md`
- [ ] Fix `elixir/otp-supervisor.md`
- [ ] Fix `elixir/protocols-behaviours-standards.md`
- [ ] Validate: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep "/elixir/"` → no output
- [ ] Commit: `fix(docs): fix mermaid violations in elixir/ docs (batch 8/11)`

## Batch 9 — C4 Architecture (5 files)

Files: `docs/explanation/software-engineering/architecture/c4-architecture-model/`

- [ ] Fix `c4-architecture-model/README.md`
- [ ] Fix `c4-architecture-model/bounded-context-visualization.md`
- [ ] Fix `c4-architecture-model/diagram-standards.md`
- [ ] Fix `c4-architecture-model/notation-standards.md`
- [ ] Fix `c4-architecture-model/nx-workspace-visualization.md`
- [ ] Validate: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep "c4-architecture-model/"` → no output
- [ ] Commit: `fix(docs): fix mermaid violations in c4-architecture-model/ docs (batch 9/11)`

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
- [ ] Validate: `go run ./apps/rhino-cli/main.go docs validate-mermaid 2>&1 | grep -E "(c-sharp|clojure|f-sharp|java/|kotlin|rust/|jvm-spring/|organize-work|system-architecture)"` → no output
- [ ] Commit: `fix(docs): fix mermaid violations in remaining docs (batch 10/10)`

## Final Validation

- [ ] Run full validator: `go run ./apps/rhino-cli/main.go docs validate-mermaid`
- [ ] Confirm zero lines starting with `✗` (warnings tolerated)
- [ ] Confirm `[width_exceeded]` count = 0
- [ ] Confirm `[label_too_long]` count = 0
- [ ] Run pre-push hook manually: `npx nx affected -t typecheck lint test:quick spec-coverage`
- [ ] Push to `main`

## Plan Archival

- [ ] Move `plans/in-progress/2026-04-25__fix-mermaid-violations/` → `plans/done/`
- [ ] Update `plans/in-progress/README.md` — remove entry
- [ ] Update `plans/done/README.md` — add entry
