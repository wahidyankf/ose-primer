---
title: Project Dependency Graph
description: Complete reference for Nx project dependencies, implicit dependencies, and workspace-level spec inputs
category: reference
tags:
  - nx
  - dependencies
  - architecture
  - monorepo
---

# Project Dependency Graph

Complete reference for how projects depend on each other in the Nx monorepo.
Run `nx graph` to visualize this interactively.

## 📋 Dependency Mechanisms

Nx tracks project relationships through three mechanisms:

### 1. `implicitDependencies` (Project-Level)

Declared in `project.json`. When the dependency project changes, `nx affected`
flags the dependent project for re-testing.

```json
"implicitDependencies": ["crud-contracts", "rhino-cli"]
```

### 2. `dependsOn` (Task-Level)

Declared per target in `project.json`. Controls execution order — the dependency
task runs before the dependent task. Cross-project `dependsOn` (e.g.,
`crud-contracts:bundle`) also creates an implicit project edge.

```json
"codegen": {
  "dependsOn": ["crud-contracts:bundle"]
}
```

### 3. `inputs` with `{workspaceRoot}` (File-Level)

Declared per target. When matched files change, the target's cache is
invalidated and `nx affected` flags the project.

```json
"inputs": [
  "default",
  "{workspaceRoot}/specs/apps/crud/be/gherkin/**/*.feature"
]
```

## 🏗️ Visual Dependency Graph

The full dependency graph is shown across three focused diagrams below.

### Shared Infrastructure

```mermaid
graph LR
  AKC[rhino-cli]:::cli
  OPC[rhino-cli]:::cli
  RC[rhino-cli]:::cli
  HC[golang-commons]:::lib
  GC[golang-commons]:::lib

  AKC --> HC
  AKC --> RC
  OPC --> HC
  OPC --> RC
  HC --> RC
  HC --> GC
  RC --> GC

  classDef lib fill:#029E73,stroke:#016B4E,color:#FFFFFF
  classDef cli fill:#DE8F05,stroke:#A56A04,color:#FFFFFF
```

### Demo Backends

```mermaid
graph LR
  BEE2E[crud-be-e2e]:::e2e
  BE[crud-be-*]:::backend
  CLOJURE[crud-be-clojure-pedestal]:::backend
  COC[clojure-openapi-codegen]:::lib
  ELIXIR[crud-be-elixir-phoenix]:::backend
  EOC[elixir-openapi-codegen]:::lib
  RC[rhino-cli]:::cli
  DC[crud-contracts]:::spec

  BEE2E --> BE
  BEE2E --> CLOJURE
  BEE2E --> ELIXIR
  BE --> DC
  BE --> RC
  CLOJURE --> COC
  CLOJURE --> DC
  CLOJURE --> RC
  ELIXIR --> EOC
  ELIXIR --> DC
  ELIXIR --> RC
  COC --> DC
  COC --> RC
  EOC --> DC
  EOC --> RC

  classDef spec fill:#0173B2,stroke:#01537F,color:#FFFFFF
  classDef lib fill:#029E73,stroke:#016B4E,color:#FFFFFF
  classDef cli fill:#DE8F05,stroke:#A56A04,color:#FFFFFF
  classDef backend fill:#CA9161,stroke:#977048,color:#FFFFFF
  classDef e2e fill:#0173B2,stroke:#01537F,color:#FFFFFF
```

### Demo Frontends

```mermaid
graph LR
  FEE2E[crud-fe-e2e]:::e2e
  NEXTJS[crud-fe-ts-nextjs]:::frontend
  TANSTACK[crud-fe-ts-tanstack-start]:::frontend
  FLUTTER[crud-fe-dart-flutterweb]:::frontend
  OLE[crud-fe-e2e]:::e2e
  OLW[crud-fe-ts-nextjs]:::site
  RC[rhino-cli]:::cli
  DC[crud-contracts]:::spec

  FEE2E --> NEXTJS
  FEE2E --> TANSTACK
  FEE2E --> FLUTTER
  FEE2E --> DC
  OLE --> OLW
  OLW --> RC
  NEXTJS --> DC
  NEXTJS --> RC
  TANSTACK --> DC
  TANSTACK --> RC
  FLUTTER --> DC
  FLUTTER --> RC

  classDef spec fill:#0173B2,stroke:#01537F,color:#FFFFFF
  classDef cli fill:#DE8F05,stroke:#A56A04,color:#FFFFFF
  classDef site fill:#CC78BC,stroke:#9A5A8E,color:#FFFFFF
  classDef frontend fill:#808080,stroke:#606060,color:#FFFFFF
  classDef e2e fill:#0173B2,stroke:#01537F,color:#FFFFFF
```

**Legend**:

- Blue: Specs / E2E tests
- Green: Libraries
- Orange: CLI tools
- Purple: Web sites
- Brown: Demo backends
- Gray: Demo frontends

## Shared Infrastructure Projects

These projects are dependencies of many other projects.

### crud-contracts

**Location**: `specs/apps/crud/contracts/`

The OpenAPI 3.1 specification consumed by all demo apps for type generation.

- **Dependents**: All 11 `crud-be-*` backends + all 3 `crud-fe-*` frontends + 2 E2E suites + 2 codegen libs (18 total)
- **Mechanism**: `implicitDependencies` + `codegen.dependsOn: ["crud-contracts:bundle"]`
- **Spec input**: `{workspaceRoot}/specs/apps/crud/contracts/generated/openapi-bundled.yaml`

### rhino-cli

**Location**: `apps/rhino-cli/`

Repository management CLI used by most projects for coverage validation
(`test-coverage validate`), spec coverage (`spec-coverage validate`),
contract post-processing (`contracts java-clean-imports`, `contracts dart-scaffold`),
and annotation validation (`java validate-annotations`).

- **Dependents**: 22 projects (all demo apps, CLI tools, libs, crud-fe-ts-nextjs)
- **Mechanism**: `implicitDependencies`
- **Own dependency**: `golang-commons`
- **Note**: `golang-commons` does NOT depend on `rhino-cli` to avoid a circular
  dependency. Changes to `rhino-cli`'s coverage algorithm are caught by the
  main CI running `--all`.

### golang-commons

**Location**: `libs/golang-commons/`

Shared Go utilities (time formatting, test helpers, output capture).

- **Dependents**: `rhino-cli`, `golang-commons`, `rhino-cli`, `rhino-cli`
- **Mechanism**: Go module `replace` directives + `implicitDependencies`

## 📊 Project Dependency Table

### Demo Backends

All demo backends share the same dependency pattern.

| Project                   | Dependencies                                                                      | Spec Inputs                 |
| ------------------------- | --------------------------------------------------------------------------------- | --------------------------- |
| crud-be-clojure-pedestal  | clojure-openapi-codegen, crud-contracts, rhino-cli                                | contracts/\*, be/gherkin/\* |
| crud-be-csharp-aspnetcore | crud-contracts, rhino-cli                                                         | contracts/\*, be/gherkin/\* |
| crud-be-elixir-phoenix    | crud-contracts, elixir-cabbage, elixir-gherkin, elixir-openapi-codegen, rhino-cli | contracts/\*, be/gherkin/\* |
| crud-be-fsharp-giraffe    | crud-contracts, rhino-cli                                                         | contracts/\*, be/gherkin/\* |
| crud-be-golang-gin        | crud-contracts, rhino-cli                                                         | contracts/\*, be/gherkin/\* |
| crud-be-java-springboot   | crud-contracts, rhino-cli                                                         | contracts/\*, be/gherkin/\* |
| crud-be-java-vertx        | crud-contracts, rhino-cli                                                         | contracts/\*, be/gherkin/\* |
| crud-be-kotlin-ktor       | crud-contracts, rhino-cli                                                         | contracts/\*, be/gherkin/\* |
| crud-be-python-fastapi    | crud-contracts, rhino-cli                                                         | contracts/\*, be/gherkin/\* |
| crud-be-rust-axum         | crud-contracts, rhino-cli                                                         | contracts/\*, be/gherkin/\* |
| crud-be-ts-effect         | crud-contracts, rhino-cli                                                         | contracts/\*, be/gherkin/\* |

**Spec input paths**:

- `contracts/*` = `{workspaceRoot}/specs/apps/crud/contracts/generated/openapi-bundled.yaml` (codegen)
- `be/gherkin/*` = `{workspaceRoot}/specs/apps/crud/be/gherkin/**/*.feature` (test:unit, test:quick)

### Demo Frontends

| Project                   | Dependencies              | Spec Inputs                 |
| ------------------------- | ------------------------- | --------------------------- |
| crud-fe-dart-flutterweb   | crud-contracts, rhino-cli | contracts/\*, fe/gherkin/\* |
| crud-fe-ts-nextjs         | crud-contracts, rhino-cli | contracts/\*, fe/gherkin/\* |
| crud-fe-ts-tanstack-start | crud-contracts, rhino-cli | contracts/\*, fe/gherkin/\* |

**Spec input paths**:

- `contracts/*` = `{workspaceRoot}/specs/apps/crud/contracts/generated/openapi-bundled.yaml` (codegen)
- `fe/gherkin/*` = `{workspaceRoot}/specs/apps/crud/fe/gherkin/**/*.feature` (test:unit, test:quick)

### E2E Test Projects

| Project     | Dependencies                               | Spec Inputs                                  |
| ----------- | ------------------------------------------ | -------------------------------------------- |
| crud-be-e2e | all 11 crud-be-\* backends, crud-contracts | be/gherkin/\* (typecheck, test:quick)        |
| crud-fe-e2e | all 3 crud-fe-\* frontends, crud-contracts | fe/gherkin/\* (typecheck, test:quick)        |
| crud-fe-e2e | crud-fe-ts-nextjs                          | crud-fe-ts-nextjs/\* (typecheck, test:quick) |

E2E projects use `bddgen` to generate TypeScript from `.feature` files in
`test:quick` and `typecheck`. Gherkin spec inputs ensure cache invalidation
when feature files change.

### Hugo Sites

| Project           | Dependencies | Spec Inputs |
| ----------------- | ------------ | ----------- |
| crud-fs-ts-nextjs | rhino-cli    | (none)      |

Hugo sites depend on their CLI tools for content automation (link checking).
The CLI tools are built via `dependsOn` in `links:check` and `test:quick` targets.

### Next.js Content Platforms

| Project           | Dependencies | Spec Inputs |
| ----------------- | ------------ | ----------- |
| crud-fs-ts-nextjs | rhino-cli    | (none)      |

crud-fs-ts-nextjs depends on rhino-cli for link validation.

### CLI Tools

| Project   | Dependencies                              | Spec Inputs                     |
| --------- | ----------------------------------------- | ------------------------------- |
| rhino-cli | golang-commons, golang-commons, rhino-cli | rhino-cli/\* (test:integration) |
| rhino-cli | golang-commons, golang-commons, rhino-cli | rhino-cli/\* (test:integration) |
| rhino-cli | golang-commons                            | rhino-cli/\* (test:integration) |

### demo

| Project           | Dependencies | Spec Inputs                             |
| ----------------- | ------------ | --------------------------------------- |
| crud-fe-ts-nextjs | rhino-cli    | crud-fe-ts-nextjs/\* (test:integration) |

### Libraries

| Project                 | Dependencies              | Spec Inputs                          |
| ----------------------- | ------------------------- | ------------------------------------ |
| golang-commons          | (none)                    | golang-commons/\* (test:integration) |
| golang-commons          | golang-commons, rhino-cli | golang-commons/\* (test:integration) |
| elixir-gherkin          | rhino-cli                 | (none)                               |
| elixir-cabbage          | elixir-gherkin, rhino-cli | (none)                               |
| elixir-openapi-codegen  | crud-contracts, rhino-cli | (none)                               |
| clojure-openapi-codegen | crud-contracts, rhino-cli | (none)                               |

### Specs

| Project        | Dependencies | Spec Inputs                                 |
| -------------- | ------------ | ------------------------------------------- |
| crud-contracts | (none)       | (self — project root is the spec directory) |

## Spec Directory Mapping

All Gherkin specs and API contracts live under `specs/` and are consumed via
`{workspaceRoot}` inputs.

| Spec Directory                | Consumed By                    | Targets                          |
| ----------------------------- | ------------------------------ | -------------------------------- |
| `specs/apps/crud/contracts/`  | all 14 demo apps               | codegen                          |
| `specs/apps/crud/be/gherkin/` | 11 demo backends + crud-be-e2e | test:unit, test:quick, typecheck |
| `specs/apps/crud/fe/gherkin/` | 3 demo frontends + crud-fe-e2e | test:unit, test:quick, typecheck |
| `specs/apps/rhino/`           | rhino-cli                      | test:integration                 |
| `specs/apps/crud/`            | rhino-cli, crud-fs-ts-nextjs   | test:integration                 |
| `specs/apps/crud/`            | rhino-cli, crud-fs-ts-nextjs   | test:integration                 |
| `specs/libs/golang-commons/`  | golang-commons                 | test:integration                 |
| `specs/libs/golang-commons/`  | golang-commons                 | test:integration                 |

## Design Decisions

### Why `golang-commons` does not depend on `rhino-cli`

`golang-commons` uses `rhino-cli` in its `test:quick` target for coverage
validation, but declaring this dependency would create a circular dependency:
`golang-commons -> rhino-cli -> golang-commons`. The risk is minimal because
`rhino-cli` coverage algorithm changes are rare and are caught by the main CI
workflow which runs `--all` projects.

### Why contracts use `implicitDependencies` instead of just `dependsOn`

Task-level `dependsOn: ["crud-contracts:bundle"]` controls execution order
(codegen runs after bundle), but does NOT make the project appear in
`nx affected` when the OpenAPI spec changes. Adding `crud-contracts` to
`implicitDependencies` ensures that spec changes trigger re-testing of all
consuming apps.

### Why E2E projects need spec inputs

E2E projects (`crud-be-e2e`, `crud-fe-e2e`, `crud-fe-e2e`) use
`bddgen` to generate TypeScript from `.feature` files in their `test:quick`
and `typecheck` targets. Without spec inputs, feature file changes would not
invalidate the cache, causing stale generated code.

## 🔗 Related Documentation

- [Monorepo Structure Reference](./monorepo-structure.md) - Folder organization and file formats
- [Nx Configuration Reference](./nx-configuration.md) - Workspace configuration options
- [Nx Target Standards](../../governance/development/infra/nx-targets.md) - Canonical target names and caching rules
- [Three-Level Testing Standard](../../governance/development/quality/three-level-testing-standard.md) - Unit, integration, and E2E testing requirements
- [Code Coverage Reference](./code-coverage.md) - Coverage measurement, tools, and thresholds
