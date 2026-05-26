# PRD — Adopt Hexagonal Architecture + DDD

## Product Overview

Five governance convention documents and structural changes to 17 non-E2E apps establish
hexagonal architecture and DDD as the canonical pattern across the ose-primer monorepo.
Each app receives the correct layer directories for its type and language. All 11 BE apps
receive bounded-context structure and wired OpenAPI codegen targets.

## Personas

- **Template author (maintainer)**: Authors convention documents and applies the pattern
  consistently across all apps. Needs a single authoritative reference for each language.
- **Downstream consumer (agent or human)**: Forks ose-primer and expects the structural
  template to be correct. Needs to find the right layer directory without guessing.

## User Stories

### US-1: Governance convention documents

As a template author, I want five governance documents describing hexagonal architecture
for all app types, so that I have a single authoritative reference when applying the pattern
to any supported language.

### US-2: CLI app layer structure

As a downstream consumer, I want each CLI app to have its canonical hexagonal layers
established on disk, so that I can place domain logic, use-case orchestration, and
infrastructure adapters in the correct directories.

### US-3: FE app layer structure

As a downstream consumer, I want each FE app to have its canonical hexagonal layers
established on disk, so that I can separate UI rendering concerns from application logic
and external API calls.

### US-4: BE app bounded-context structure

As a downstream consumer, I want each BE app to have its canonical bounded-context hexagonal
structure established on disk, so that I can place domain models, application services,
infrastructure adapters, and HTTP handlers in the correct directories under the correct context.

### US-5: OpenAPI codegen wiring

As a downstream consumer, I want all BE apps to have Nx codegen targets that generate typed
clients from their OpenAPI specs into the three primary FE apps, so that FE code always has
type-safe contracts that match the actual server contract.

## Acceptance Criteria

### AC-1: Governance documents created

```gherkin
Scenario: All five governance convention documents exist and pass quality gates
  Given the plan has been executed
  When I list files under repo-governance/development/pattern/
  Then hexagonal-architecture.md exists
  And hexagonal-architecture-be.md exists
  And hexagonal-architecture-web.md exists
  And hexagonal-architecture-cli.md exists
  And openapi-contract-first.md exists
  When I run markdownlint on each document
  Then all five documents pass with zero violations
```

### AC-2: CLI Rust app layer structure

```gherkin
Scenario: rhino-cli-rust has canonical hexagonal layer directories
  Given the plan has been executed
  When I inspect apps/rhino-cli-rust/src/
  Then the directory domain/ exists
  And the directory application/ exists
  And the directory infrastructure/ exists
  And the directory commands/ exists
```

### AC-3: CLI Go app layer structure

```gherkin
Scenario: rhino-cli-go has canonical hexagonal layer directories
  Given the plan has been executed
  When I inspect apps/rhino-cli-go/
  Then the directory internal/domain/ exists
  And the directory internal/application/ exists
  And the directory internal/adapter/command/ exists
  And the directory cmd/ exists
```

### AC-4: FE TypeScript apps layer structure

```gherkin
Scenario: crud-fe-ts-nextjs has canonical hexagonal layer directories
  Given the plan has been executed
  When I inspect apps/crud-fe-ts-nextjs/src/
  Then the directory domain/ exists
  And the directory application/ exists
  And the directory infrastructure/ exists
  And the directory presentation/ exists

Scenario: crud-fe-ts-tanstack-start has canonical hexagonal layer directories
  Given the plan has been executed
  When I inspect apps/crud-fe-ts-tanstack-start/src/
  Then the directory domain/ exists
  And the directory application/ exists
  And the directory infrastructure/ exists
  And the directory presentation/ exists

Scenario: crud-fs-ts-nextjs has canonical hexagonal layer directories (treated as FE)
  Given the plan has been executed
  When I inspect apps/crud-fs-ts-nextjs/src/
  Then the directory domain/ exists
  And the directory application/ exists
  And the directory infrastructure/ exists
  And the directory presentation/ exists
```

### AC-5: FE Dart/Flutter app layer structure

```gherkin
Scenario: crud-fe-dart-flutterweb has canonical hexagonal layer directories
  Given the plan has been executed
  When I inspect apps/crud-fe-dart-flutterweb/lib/
  Then the directory domain/ exists
  And the directory application/ exists
  And the directory infrastructure/ exists
  And the directory presentation/ exists
```

### AC-6: BE apps bounded-context structure — Rust/Axum

```gherkin
Scenario: crud-be-rust-axum has bounded-context hexagonal structure
  Given the plan has been executed
  When I inspect apps/crud-be-rust-axum/src/
  Then the directory contexts/expenses/domain/ exists
  And the directory contexts/expenses/application/ exists
  And the directory contexts/expenses/infrastructure/ exists
  And the directory contexts/expenses/api/http/ exists
```

### AC-7: BE apps bounded-context structure — Go/Gin

```gherkin
Scenario: crud-be-golang-gin has bounded-context hexagonal structure
  Given the plan has been executed
  When I inspect apps/crud-be-golang-gin/
  Then the directory internal/contexts/expenses/domain/ exists
  And the directory internal/contexts/expenses/application/ exists
  And the directory internal/contexts/expenses/infrastructure/ exists
  And the directory internal/contexts/expenses/api/http/ exists
```

### AC-8: BE apps bounded-context structure — F#/Giraffe

```gherkin
Scenario: crud-be-fsharp-giraffe has bounded-context hexagonal structure
  Given the plan has been executed
  When I inspect apps/crud-be-fsharp-giraffe/src/DemoBeFsgi/
  Then the directory Contexts/Expenses/Domain/ exists
  And the directory Contexts/Expenses/Application/ exists
  And the directory Contexts/Expenses/Infrastructure/ exists
  And the directory Contexts/Expenses/Api/Http/ exists
```

### AC-9: BE apps bounded-context structure — TypeScript/Effect

```gherkin
Scenario: crud-be-ts-effect has bounded-context hexagonal structure
  Given the plan has been executed
  When I inspect apps/crud-be-ts-effect/src/
  Then the directory contexts/expenses/domain/ exists
  And the directory contexts/expenses/application/ exists
  And the directory contexts/expenses/infrastructure/ exists
  And the directory contexts/expenses/api/http/ exists
```

### AC-10: BE apps bounded-context structure — Python/FastAPI

```gherkin
Scenario: crud-be-python-fastapi has bounded-context hexagonal structure
  Given the plan has been executed
  When I inspect apps/crud-be-python-fastapi/src/crud_be_python_fastapi/
  Then the directory contexts/expenses/domain/ exists
  And the directory contexts/expenses/application/ exists
  And the directory contexts/expenses/infrastructure/ exists
  And the directory contexts/expenses/api/http/ exists
```

### AC-11: BE apps bounded-context structure — Clojure/Pedestal

```gherkin
Scenario: crud-be-clojure-pedestal has bounded-context hexagonal structure
  Given the plan has been executed
  When I inspect apps/crud-be-clojure-pedestal/src/crud_be_cjpd/
  Then the directory contexts/expenses/domain/ exists
  And the directory contexts/expenses/application/ exists
  And the directory contexts/expenses/infrastructure/ exists
  And the directory contexts/expenses/api/http/ exists
```

### AC-12: BE apps bounded-context structure — Java/Vert.x

```gherkin
Scenario: crud-be-java-vertx has bounded-context hexagonal structure
  Given the plan has been executed
  When I inspect apps/crud-be-java-vertx/src/main/java/com/demobejavx/
  Then the directory contexts/expenses/domain/ exists
  And the directory contexts/expenses/application/ exists
  And the directory contexts/expenses/infrastructure/ exists
  And the directory contexts/expenses/api/http/ exists
```

### AC-13: BE apps bounded-context structure — Java/Spring Boot

```gherkin
Scenario: crud-be-java-springboot has bounded-context hexagonal structure
  Given the plan has been executed
  When I inspect apps/crud-be-java-springboot/src/main/java/com/demobejasb/
  Then the directory contexts/expenses/domain/ exists
  And the directory contexts/expenses/application/ exists
  And the directory contexts/expenses/infrastructure/ exists
  And the directory contexts/expenses/api/http/ exists
```

### AC-14: BE apps bounded-context structure — Kotlin/Ktor

```gherkin
Scenario: crud-be-kotlin-ktor has bounded-context hexagonal structure
  Given the plan has been executed
  When I inspect apps/crud-be-kotlin-ktor/src/main/kotlin/com/demobektkt/
  Then the directory contexts/expenses/domain/ exists
  And the directory contexts/expenses/application/ exists
  And the directory contexts/expenses/infrastructure/ exists
  And the directory contexts/expenses/api/http/ exists
```

### AC-15: BE apps bounded-context structure — Elixir/Phoenix

```gherkin
Scenario: crud-be-elixir-phoenix has bounded-context hexagonal structure
  Given the plan has been executed
  When I inspect apps/crud-be-elixir-phoenix/lib/
  Then the directory crud_be_exph/contexts/expenses/domain/ exists
  And the directory crud_be_exph/contexts/expenses/application/ exists
  And the directory crud_be_exph/contexts/expenses/infrastructure/ exists
  And the directory crud_be_exph_web/ exists as the HTTP adapter layer
```

### AC-16: BE apps bounded-context structure — C#/ASP.NET Core

```gherkin
Scenario: crud-be-csharp-aspnetcore has bounded-context hexagonal structure
  Given the plan has been executed
  When I inspect apps/crud-be-csharp-aspnetcore/src/DemoBeCsas/
  Then the directory Contexts/Expenses/Domain/ exists
  And the directory Contexts/Expenses/Application/ exists
  And the directory Contexts/Expenses/Infrastructure/ exists
  And the directory Contexts/Expenses/Api/Http/ exists
```

### AC-17: OpenAPI codegen targets wired for all BE apps

```gherkin
Scenario: All 11 BE apps have Nx codegen target for OpenAPI contracts
  Given the plan has been executed
  When I read the project.json of each BE app
  Then each project.json contains a codegen target
  And running npx nx run <be-app>:codegen exits 0
  And the generated client artifacts land in generated-contracts/
```

### AC-18: No regressions

```gherkin
Scenario: All existing tests pass after structural changes
  Given a baseline test run was recorded before any changes
  When all phases of the plan have been executed
  And I run npx nx run-many -t test:quick --all
  Then all previously-passing tests continue to pass
  And no new test failures are introduced
```

## Product Scope

### In scope

- Creation of 5 governance convention documents
- Creation of canonical layer directories (with `.gitkeep` placeholder files where needed)
  for all 17 non-E2E apps
- Bounded-context structure (`contexts/expenses/`) for all 11 BE apps
- Nx `codegen` target wired for all 11 BE apps where not already present
- Nx consumer codegen targets wired in the 3 primary FE apps where not already present

### Out of scope

- Rewriting existing source files to move code into new layers
- Adding new tests beyond ensuring existing tests pass
- Changing the OpenAPI spec content itself (only wiring the targets)
- E2E apps (`crud-fe-e2e`, `crud-be-e2e`)

## Product Risks

| Risk                                                                   | Mitigation                                                                   |
| ---------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| `.gitkeep` files cause confusion about which layer should contain what | Convention documents clearly describe what belongs in each layer             |
| Elixir Phoenix layer naming diverges from rest of BE apps              | Convention document explicitly documents the Phoenix exception and rationale |
| Java package names conflict with new bounded-context directories       | New directories are added alongside existing packages; no renames in Phase 4 |
