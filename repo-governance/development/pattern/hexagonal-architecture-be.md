---
title: Hexagonal Architecture + DDD — Backend Apps
description: Hexagonal architecture with DDD bounded contexts for backend apps — F#/Giraffe directory layouts, language-specific idioms, and inter-context isolation rules
category: explanation
subcategory: development
tags:
  - architecture
  - hexagonal
  - ddd
  - fsharp
  - backend
created: 2026-05-26
---

# Hexagonal Architecture — Backend Apps

Canonical hexagonal layer conventions for all 11 backend apps in ose-primer. Backend apps
use Domain-Driven Design (DDD) bounded contexts to organize layers.

See [Hexagonal Architecture](./hexagonal-architecture.md) for shared principles and
terminology. See [OpenAPI Contract-First Development](./openapi-contract-first.md) for
API contract conventions.

## Scope

All 11 BE apps: `crud-be-rust-axum`, `crud-be-golang-gin`, `crud-be-fsharp-giraffe`,
`crud-be-ts-effect`, `crud-be-python-fastapi`, `crud-be-clojure-pedestal`,
`crud-be-java-vertx`, `crud-be-java-springboot`, `crud-be-kotlin-ktor`,
`crud-be-elixir-phoenix`, `crud-be-csharp-aspnetcore`.

## Bounded-Context Pattern

Each BE app wraps its hexagonal layers inside a bounded-context directory named
`contexts/<name>/`. A bounded context owns its slice of the domain model, application
layer, infrastructure adapters, and HTTP adapter. Using an explicit context directory
makes the DDD structure visible on disk and allows multiple contexts to coexist in the
same app as the domain grows.

The CRUD demo uses a single bounded context named **`expenses`** (matching the primary
domain entity). All 11 apps use the same context name, making cross-language navigation
predictable: the path shape is identical in every language.

## `api/http/` Two-Level Structure

The outermost inbound adapter is always two directory levels: `api/` containing `http/`.
This matches the convention from ose-public exactly and allows future non-HTTP adapter
siblings (e.g. `api/grpc/`, `api/graphql/`) to be added without restructuring.

## Full Language-Specific Directory Layout

| App                         | Root for layers                   | Domain                                   | Application                                   | Infrastructure                                   | HTTP adapter                           |
| --------------------------- | --------------------------------- | ---------------------------------------- | --------------------------------------------- | ------------------------------------------------ | -------------------------------------- |
| `crud-be-rust-axum`         | `src/`                            | `contexts/expenses/domain/`              | `contexts/expenses/application/`              | `contexts/expenses/infrastructure/`              | `contexts/expenses/api/http/`          |
| `crud-be-golang-gin`        | (app root)                        | `internal/contexts/expenses/domain/`     | `internal/contexts/expenses/application/`     | `internal/contexts/expenses/infrastructure/`     | `internal/contexts/expenses/api/http/` |
| `crud-be-fsharp-giraffe`    | `src/DemoBeFsgi/`                 | `Contexts/Expenses/Domain/`              | `Contexts/Expenses/Application/`              | `Contexts/Expenses/Infrastructure/`              | `Contexts/Expenses/Api/Http/`          |
| `crud-be-ts-effect`         | `src/`                            | `contexts/expenses/domain/`              | `contexts/expenses/application/`              | `contexts/expenses/infrastructure/`              | `contexts/expenses/api/http/`          |
| `crud-be-python-fastapi`    | `src/crud_be_python_fastapi/`     | `contexts/expenses/domain/`              | `contexts/expenses/application/`              | `contexts/expenses/infrastructure/`              | `contexts/expenses/api/http/`          |
| `crud-be-clojure-pedestal`  | `src/crud_be_cjpd/`               | `contexts/expenses/domain/`              | `contexts/expenses/application/`              | `contexts/expenses/infrastructure/`              | `contexts/expenses/api/http/`          |
| `crud-be-java-vertx`        | `src/main/java/com/demobejavx/`   | `contexts/expenses/domain/`              | `contexts/expenses/application/`              | `contexts/expenses/infrastructure/`              | `contexts/expenses/api/http/`          |
| `crud-be-java-springboot`   | `src/main/java/com/demobejasb/`   | `contexts/expenses/domain/`              | `contexts/expenses/application/`              | `contexts/expenses/infrastructure/`              | `contexts/expenses/api/http/`          |
| `crud-be-kotlin-ktor`       | `src/main/kotlin/com/demobektkt/` | `contexts/expenses/domain/`              | `contexts/expenses/application/`              | `contexts/expenses/infrastructure/`              | `contexts/expenses/api/http/`          |
| `crud-be-elixir-phoenix`    | `lib/`                            | `crud_be_exph/contexts/expenses/domain/` | `crud_be_exph/contexts/expenses/application/` | `crud_be_exph/contexts/expenses/infrastructure/` | `crud_be_exph_web/` (generated)        |
| `crud-be-csharp-aspnetcore` | `src/DemoBeCsas/`                 | `Contexts/Expenses/Domain/`              | `Contexts/Expenses/Application/`              | `Contexts/Expenses/Infrastructure/`              | `Contexts/Expenses/Api/Http/`          |

## Language-Specific Notes

### F# and C# — PascalCase Directories

F# and C# use PascalCase directory names (`Contexts/Expenses/Domain/`) to match .NET
and F# naming conventions. Every other app uses lowercase.

### Go — `internal/` Wrapper

Go's `internal/` wrapper is compiler-enforced: code inside `internal/` cannot be
imported by packages outside the module. All four bounded-context layers for
`crud-be-golang-gin` live under `internal/contexts/expenses/`.

### Elixir Phoenix — `_web/` Exception

Phoenix generates `lib/<app>/` and `lib/<app>_web/` at project creation. The
`lib/crud_be_exph_web/` directory already exists and IS the HTTP adapter layer — it
is the `api/http/` equivalent for this app. Creating a separate `api/http/` directory
would conflict with Phoenix's generated structure.

Inner DDD layers (`domain/`, `application/`, `infrastructure/`) are added under
`lib/crud_be_exph/contexts/expenses/`. The `_web/` module is not duplicated.

### Rust — `api/` and `api/http/` Both Exist

For Rust apps using Axum, both `api/` and `api/http/` directories are created. In a
real implementation each would contain a `mod.rs` to register the module. During the
initial scaffold, each level holds a `.gitkeep` placeholder.

### JVM (Java, Kotlin) — Lowercase Package Directories

Java and Kotlin package names are lowercase; directories follow naturally. New
bounded-context directories are siblings to existing packages — no existing packages are
renamed.

### Clojure — Underscore Namespace Convention

Clojure namespaces use underscores for directory separators. The source root
`src/crud_be_cjpd/` is the module root; bounded-context directories follow Clojure
namespace conventions with lowercase underscored paths.

## References

- [Hexagonal Architecture](./hexagonal-architecture.md) — shared principles and dependency rule
- [OpenAPI Contract-First Development](./openapi-contract-first.md) — API contract and codegen

## Principles Implemented/Respected

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)** —
  A single bounded-context pattern (`contexts/<name>/`) is applied identically across all 11
  backend apps, making cross-language navigation predictable without per-app conventions.
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)** —
  DDD bounded-context directories make the domain boundary visible on disk; no implicit coupling
  between bounded contexts is permitted.
- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)** —
  Contract types and server stubs are generated from the OpenAPI spec by Nx codegen targets,
  eliminating hand-written drift between spec and implementation.

## Conventions Implemented/Respected

- **[Hexagonal Architecture](./hexagonal-architecture.md)** — This document specializes the
  shared dependency rule and terminology for backend DDD bounded-context structure.
- **[OpenAPI Contract-First Development](./openapi-contract-first.md)** — Generated contract
  types land in `generated-contracts/` inside each BE app's adapter zone, not the domain.
