# Adopt Hexagonal Architecture + DDD

**Status**: In Progress
**Plan identifier**: `adopt-hexagonal-ddd-architecture`
**Created**: 2026-05-26

## Context

The ose-primer monorepo contains 19 apps spanning 11+ languages. Current app structures are
language-idiomatic but architecturally inconsistent: some apps have partial hexagonal layers,
others are flat, and none enforce bounded-context DDD separation. This creates cognitive overhead
when contributors navigate across apps, and makes the monorepo less useful as a template for
production OSE-style repos.

This plan establishes hexagonal architecture + Domain-Driven Design (DDD) as the canonical
pattern across all non-E2E apps, producing both governance convention documents and concrete
structural changes in each app.

## Scope

**In scope** (17 apps):

- 2 CLI apps: `rhino-cli-rust`, `rhino-cli-go`
- 4 FE apps: `crud-fe-ts-nextjs`, `crud-fe-ts-tanstack-start`, `crud-fe-dart-flutterweb`, `crud-fs-ts-nextjs`
- 11 BE apps: `crud-be-rust-axum`, `crud-be-golang-gin`, `crud-be-fsharp-giraffe`, `crud-be-ts-effect`,
  `crud-be-python-fastapi`, `crud-be-clojure-pedestal`, `crud-be-java-vertx`, `crud-be-java-springboot`,
  `crud-be-kotlin-ktor`, `crud-be-elixir-phoenix`, `crud-be-csharp-aspnetcore`
- 5 governance convention documents in `repo-governance/development/pattern/`

**Out of scope**:

- `apps/crud-fe-e2e` — E2E test runner, no hexagonal structure applies
- `apps/crud-be-e2e` — E2E test runner, no hexagonal structure applies
- Application logic rewrites (layer structure only; existing business logic stays in place)
- Database migration or schema changes

## Documents

| Document                       | Purpose                                                         |
| ------------------------------ | --------------------------------------------------------------- |
| [brd.md](./brd.md)             | Business rationale, success metrics, risks                      |
| [prd.md](./prd.md)             | Product requirements, user stories, Gherkin acceptance criteria |
| [tech-docs.md](./tech-docs.md) | Architecture, layer naming table, design decisions              |
| [delivery.md](./delivery.md)   | Phased delivery checklist                                       |

## Quick Reference: Layer Naming by App Type

| App type           | Layer 1 (innermost)             | Layer 2                              | Layer 3                                 | Layer 4 (outermost)               |
| ------------------ | ------------------------------- | ------------------------------------ | --------------------------------------- | --------------------------------- |
| CLI (Rust)         | `domain/`                       | `application/`                       | `infrastructure/`                       | `commands/`                       |
| CLI (Go)           | `internal/domain/`              | `internal/application/`              | `internal/adapter/command/`             | `cmd/`                            |
| FE (TS/Dart)       | `domain/`                       | `application/`                       | `infrastructure/`                       | `presentation/`                   |
| BE (all except Go) | `contexts/<n>/domain/`          | `contexts/<n>/application/`          | `contexts/<n>/infrastructure/`          | `contexts/<n>/api/http/`          |
| BE (Go)            | `internal/contexts/<n>/domain/` | `internal/contexts/<n>/application/` | `internal/contexts/<n>/infrastructure/` | `internal/contexts/<n>/api/http/` |

See [tech-docs.md](./tech-docs.md) for the full language-specific naming table.
