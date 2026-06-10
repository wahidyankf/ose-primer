---
title: Applications & Containers
description: Application inventory and C4 Level 2 container diagram
category: reference
tags:
  - architecture
  - applications
  - c4-model
---

# Applications & Containers

Application inventory and C4 Level 2 container diagram for the platform.

## 📋 Applications Inventory

The platform consists of 19 applications across multiple technology stacks.

### Backend Services (`apps/crud-be-*`)

| App                         | Language / Framework | Build Command                        |
| --------------------------- | -------------------- | ------------------------------------ |
| `crud-be-clojure-pedestal`  | Clojure + Pedestal   | `nx build crud-be-clojure-pedestal`  |
| `crud-be-csharp-aspnetcore` | C# + ASP.NET Core    | `nx build crud-be-csharp-aspnetcore` |
| `crud-be-elixir-phoenix`    | Elixir + Phoenix     | `nx build crud-be-elixir-phoenix`    |
| `crud-be-fsharp-giraffe`    | F# + Giraffe         | `nx build crud-be-fsharp-giraffe`    |
| `crud-be-golang-gin`        | Go + Gin             | `nx build crud-be-golang-gin`        |
| `crud-be-java-springboot`   | Java + Spring Boot   | `nx build crud-be-java-springboot`   |
| `crud-be-java-vertx`        | Java + Vert.x        | `nx build crud-be-java-vertx`        |
| `crud-be-kotlin-ktor`       | Kotlin + Ktor        | `nx build crud-be-kotlin-ktor`       |
| `crud-be-python-fastapi`    | Python + FastAPI     | `nx build crud-be-python-fastapi`    |
| `crud-be-rust-axum`         | Rust + Axum          | `nx build crud-be-rust-axum`         |
| `crud-be-ts-effect`         | TypeScript + Effect  | `nx build crud-be-ts-effect`         |

All backend services implement the same OpenAPI contract (`specs/apps/crud/containers/contracts/`). Each is an independent deployable REST API.

### Frontend Applications (`apps/crud-fe-*`)

| App                         | Language / Framework        | Build Command                        |
| --------------------------- | --------------------------- | ------------------------------------ |
| `crud-fe-dart-flutterweb`   | Dart + Flutter Web          | `nx build crud-fe-dart-flutterweb`   |
| `crud-fe-ts-nextjs`         | TypeScript + Next.js        | `nx build crud-fe-ts-nextjs`         |
| `crud-fe-ts-tanstack-start` | TypeScript + TanStack Start | `nx build crud-fe-ts-tanstack-start` |

### Fullstack Application

| App                 | Language / Framework | Build Command                |
| ------------------- | -------------------- | ---------------------------- |
| `crud-fs-ts-nextjs` | TypeScript + Next.js | `nx build crud-fs-ts-nextjs` |

### E2E Test Suites

| App           | Purpose                                   | Run Command                   |
| ------------- | ----------------------------------------- | ----------------------------- |
| `crud-be-e2e` | End-to-end tests for all `crud-be-*` APIs | `nx run crud-be-e2e:test:e2e` |
| `crud-fe-e2e` | End-to-end tests for `crud-fe-*` apps     | `nx run crud-fe-e2e:test:e2e` |

### CLI Tools

| App         | Language | Purpose                          | Build Command        |
| ----------- | -------- | -------------------------------- | -------------------- |
| `rhino-cli` | Rust     | Repository management automation | `nx build rhino-cli` |

## 🏗️ C4 Level 2: Container Diagram

Shows the high-level technical building blocks (containers) of the system. In C4 terminology, a "container" is a deployable/executable unit (web app, API, CLI, etc.), not a Docker container.

```mermaid
graph LR
    subgraph "Frontend Applications"
        FE_NEXTJS[crud-fe-ts-nextjs<br/>Next.js App]
        FE_TANSTACK[crud-fe-ts-tanstack-start<br/>TanStack Start App]
        FE_DART[crud-fe-dart-flutterweb<br/>Flutter Web App]
        FS_NEXTJS[crud-fs-ts-nextjs<br/>Next.js Fullstack]
    end

    subgraph "Backend APIs - polyglot"
        BE_CLOJURE[crud-be-clojure-pedestal<br/>Clojure API]
        BE_CSHARP[crud-be-csharp-aspnetcore<br/>C# API]
        BE_ELIXIR[crud-be-elixir-phoenix<br/>Elixir API]
        BE_FSHARP[crud-be-fsharp-giraffe<br/>F# API]
        BE_GO[crud-be-golang-gin<br/>Go API]
        BE_JASB[crud-be-java-springboot<br/>Java Spring Boot API]
        BE_JAVX[crud-be-java-vertx<br/>Java Vert.x API]
        BE_KOTLIN[crud-be-kotlin-ktor<br/>Kotlin API]
        BE_PYTHON[crud-be-python-fastapi<br/>Python API]
        BE_RUST[crud-be-rust-axum<br/>Rust API]
        BE_TS[crud-be-ts-effect<br/>TypeScript API]
    end

    subgraph "E2E Test Suites"
        FE_E2E[crud-fe-e2e<br/>Playwright FE E2E]
        BE_E2E[crud-be-e2e<br/>Playwright BE E2E]
    end

    subgraph "CLI Tools"
        RHINO_RUST[rhino-cli<br/>Rust CLI]
    end

    subgraph "Shared Infrastructure"
        NX[Nx Workspace<br/>Build Orchestration]
        CONTRACT[OpenAPI Contract<br/>specs/apps/crud/]
    end

    FE_NEXTJS -->|calls| BE_FSHARP
    FE_TANSTACK -->|calls| BE_FSHARP
    FE_DART -->|calls| BE_FSHARP
    FE_E2E -->|tests| FE_NEXTJS
    BE_E2E -->|tests| BE_FSHARP

    CONTRACT -.->|defines API for| BE_CLOJURE
    CONTRACT -.->|defines API for| BE_CSHARP
    CONTRACT -.->|defines API for| BE_ELIXIR
    CONTRACT -.->|defines API for| BE_FSHARP
    CONTRACT -.->|defines API for| BE_GO
    CONTRACT -.->|defines API for| BE_JASB
    CONTRACT -.->|defines API for| BE_JAVX
    CONTRACT -.->|defines API for| BE_KOTLIN
    CONTRACT -.->|defines API for| BE_PYTHON
    CONTRACT -.->|defines API for| BE_RUST
    CONTRACT -.->|defines API for| BE_TS

    NX -.->|manages| FE_NEXTJS
    NX -.->|manages| BE_FSHARP
    NX -.->|manages| RHINO_RUST

    style FE_NEXTJS fill:#0077b6,stroke:#03045e,color:#ffffff
    style FE_TANSTACK fill:#0077b6,stroke:#03045e,color:#ffffff
    style FE_DART fill:#0077b6,stroke:#03045e,color:#ffffff
    style FS_NEXTJS fill:#0077b6,stroke:#03045e,color:#ffffff
    style BE_CLOJURE fill:#e76f51,stroke:#9d0208,color:#ffffff
    style BE_CSHARP fill:#e76f51,stroke:#9d0208,color:#ffffff
    style BE_ELIXIR fill:#e76f51,stroke:#9d0208,color:#ffffff
    style BE_FSHARP fill:#e76f51,stroke:#9d0208,color:#ffffff
    style BE_GO fill:#e76f51,stroke:#9d0208,color:#ffffff
    style BE_JASB fill:#e76f51,stroke:#9d0208,color:#ffffff
    style BE_JAVX fill:#e76f51,stroke:#9d0208,color:#ffffff
    style BE_KOTLIN fill:#e76f51,stroke:#9d0208,color:#ffffff
    style BE_PYTHON fill:#e76f51,stroke:#9d0208,color:#ffffff
    style BE_RUST fill:#e76f51,stroke:#9d0208,color:#ffffff
    style BE_TS fill:#e76f51,stroke:#9d0208,color:#ffffff
    style FE_E2E fill:#457b9d,stroke:#1d3557,color:#ffffff
    style BE_E2E fill:#457b9d,stroke:#1d3557,color:#ffffff
    style RHINO_RUST fill:#2a9d8f,stroke:#264653,color:#ffffff
    style NX fill:#6a4c93,stroke:#22223b,color:#ffffff
    style CONTRACT fill:#6a4c93,stroke:#22223b,color:#ffffff
```

## 🔄 Application Interactions

**Contract-First Design:**

All backend services (`crud-be-*`) implement the same OpenAPI 3.1 contract defined in
`specs/apps/crud/containers/contracts/`. They are independently deployable and
interchangeable — frontends can point to any backend.

**Frontend ↔ Backend:**

- Frontend apps (`crud-fe-*`, `crud-fs-ts-nextjs`) call backend REST APIs
- All backends expose the same endpoints per the OpenAPI contract

**E2E Test Suites:**

- `crud-be-e2e` — tests backend APIs against the OpenAPI contract
- `crud-fe-e2e` — tests frontend user flows via Playwright

**CLI Tools:**

- `rhino-cli` — repository management automation (Rust implementation)

**Build-Time Dependencies:**

- All applications managed by Nx workspace
- Backend apps consume code from `libs/` (codegen, commons)
- Shared OpenAPI contract consumed by backend apps and E2E suites

## 🔗 Related Documentation

- [Monorepo Structure Reference](../monorepo-structure.md) — Folder layout and naming conventions
- [Project Dependency Graph](../project-dependency-graph.md) — Nx dependency relationships
- [C4 Architecture Model](../../explanation/software-engineering/architecture/c4-architecture-model/README.md) — C4 diagram standards
