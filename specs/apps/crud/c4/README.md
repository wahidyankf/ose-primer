# Demo Application C4 Diagrams

C4 architecture diagrams for the unified crud application (frontend + backend).

## Diagrams

| Level     | File              | What It Shows                                                                 |
| --------- | ----------------- | ----------------------------------------------------------------------------- |
| Context   | `context.md`      | The system and its four external actors                                       |
| Container | `container.md`    | Runtime containers: SPA, Static Server, REST API, Database, File Storage      |
| Component | `component-be.md` | REST API internals: handlers, middleware, services, repositories              |
| Component | `component-fe.md` | SPA internals: pages, shared components, state management, API client, guards |

## C4 Level Summary

- **Context** — answers: who uses the system and how?
- **Container** — answers: what processes run and what data stores exist?
- **Component (BE)** — answers: what are the logical building blocks inside the REST API?
- **Component (FE)** — answers: what are the logical building blocks inside the SPA?

## Implementations

The crud application is implemented in multiple languages and frameworks. All
implementations conform to the same API contract and Gherkin specifications.

### Backend Implementations (11)

| App                       | Language   | Framework    | CI Workflow                          |
| ------------------------- | ---------- | ------------ | ------------------------------------ |
| crud-be-golang-gin        | Go         | Gin          | `test-crud-be-golang-gin.yml`        |
| crud-be-java-springboot   | Java       | Spring Boot  | `test-crud-be-java-springboot.yml`   |
| crud-be-java-vertx        | Java       | Vert.x       | `test-crud-be-java-vertx.yml`        |
| crud-be-kotlin-ktor       | Kotlin     | Ktor         | `test-crud-be-kotlin-ktor.yml`       |
| crud-be-python-fastapi    | Python     | FastAPI      | `test-crud-be-python-fastapi.yml`    |
| crud-be-rust-axum         | Rust       | Axum         | `test-crud-be-rust-axum.yml`         |
| crud-be-ts-effect         | TypeScript | Effect       | `test-crud-be-ts-effect.yml`         |
| crud-be-fsharp-giraffe    | F#         | Giraffe      | `test-crud-be-fsharp-giraffe.yml`    |
| crud-be-csharp-aspnetcore | C#         | ASP.NET Core | `test-crud-be-csharp-aspnetcore.yml` |
| crud-be-clojure-pedestal  | Clojure    | Pedestal     | `test-crud-be-clojure-pedestal.yml`  |
| crud-be-elixir-phoenix    | Elixir     | Phoenix      | `test-crud-be-elixir-phoenix.yml`    |

### Frontend Implementations (3)

| App                       | Language   | Framework      | CI Workflow                          |
| ------------------------- | ---------- | -------------- | ------------------------------------ |
| crud-fe-ts-nextjs         | TypeScript | Next.js 16     | `test-crud-fe-ts-nextjs.yml`         |
| crud-fe-ts-tanstack-start | TypeScript | TanStack Start | `test-crud-fe-ts-tanstack-start.yml` |
| crud-fe-dart-flutterweb   | Dart       | Flutter Web    | `test-crud-fe-dart-flutterweb.yml`   |

### CI Workflows

- **Main CI** (`main-ci.yml`): Runs `typecheck`, `lint`, `test:quick` for all
  projects on push to `main`. Coverage is enforced locally by `rhino-cli test-coverage validate` inside `test:quick`.
- **Per-app E2E** (`test-crud-be-*.yml`, `test-crud-fe-*.yml`): Manual
  `workflow_dispatch` only (cron schedules removed to conserve resources).
  Starts full stack via Docker Compose, runs Playwright E2E tests.
- **PR Quality Gate** (`pr-quality-gate.yml`): Runs `typecheck`, `lint`,
  `test:quick` for affected projects on pull requests.

## API Contract

All implementations conform to a single OpenAPI 3.1 specification:

- **Source**: [`specs/apps/crud/contracts/openapi.yaml`](../contracts/openapi.yaml)
- **Bundled**: `specs/apps/crud/contracts/generated/openapi-bundled.yaml` (generated)
- **Nx project**: `crud-contracts` (targets: `lint`, `bundle`, `docs`)
- **Codegen**: Each implementation generates types from the bundled spec via
  its `codegen` Nx target into `generated-contracts/`

## Gherkin Specifications

All implementations consume shared Gherkin feature files. Backend and frontend
have separate spec trees with different domain coverage.

### Backend Gherkin

**Location**: [`specs/apps/crud/be/gherkin/`](../be/gherkin/README.md)

| Domain           | Feature                    | Scenarios |
| ---------------- | -------------------------- | --------- |
| admin            | admin.feature              | 6         |
| authentication   | password-login.feature     | 5         |
| authentication   | token-lifecycle.feature    | 7         |
| expenses         | attachments.feature        | 10        |
| expenses         | currency-handling.feature  | 6         |
| expenses         | expense-management.feature | 7         |
| expenses         | reporting.feature          | 6         |
| expenses         | unit-handling.feature      | 4         |
| health           | health-check.feature       | 2         |
| security         | security.feature           | 5         |
| test-support     | test-api.feature           | 2         |
| token-management | tokens.feature             | 6         |
| user-lifecycle   | registration.feature       | 6         |
| user-lifecycle   | user-account.feature       | 6         |

### Frontend Gherkin

**Location**: [`specs/apps/crud/fe/gherkin/`](../fe/gherkin/README.md)

| Domain           | Feature                    | Scenarios |
| ---------------- | -------------------------- | --------- |
| admin            | admin-panel.feature        | 6         |
| authentication   | login.feature              | 5         |
| authentication   | session.feature            | 7         |
| expenses         | attachments.feature        | 10        |
| expenses         | currency-handling.feature  | 6         |
| expenses         | expense-management.feature | 7         |
| expenses         | reporting.feature          | 6         |
| expenses         | unit-handling.feature      | 4         |
| health           | health-status.feature      | 2         |
| layout           | accessibility.feature      | 6         |
| layout           | responsive.feature         | 10        |
| security         | security.feature           | 5         |
| token-management | tokens.feature             | 6         |
| user-lifecycle   | registration.feature       | 6         |
| user-lifecycle   | user-profile.feature       | 6         |

### Three-Level Testing Standard

All implementations follow the same three-level testing pattern:

| Level                            | Scope                             | Database        | HTTP | Gherkin |
| -------------------------------- | --------------------------------- | --------------- | ---- | ------- |
| Unit (`test:unit`)               | Service-layer calls, mocked repos | Mocked          | No   | Yes     |
| Integration (`test:integration`) | Service-layer calls, real DB      | Real PostgreSQL | No   | Yes     |
| E2E (`test:e2e`)                 | Full HTTP via Playwright          | Real PostgreSQL | Yes  | Yes     |

Coverage thresholds: backends >= 90%, frontends >= 70%.

## Related

- **Parent**: [crud specs](../README.md)
- **Backend gherkin specs**: [be/gherkin/](../be/gherkin/README.md)
- **Frontend gherkin specs**: [fe/gherkin/](../fe/gherkin/README.md)
- **API contract**: [contracts/](../contracts/openapi.yaml)
- **Project dependency graph**: [docs/reference/project-dependency-graph.md](../../../../docs/reference/project-dependency-graph.md)
