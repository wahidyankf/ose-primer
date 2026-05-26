# Hexagonal Architecture — Web/FE Apps

Canonical hexagonal layer conventions for the four frontend apps in ose-primer.

See [Hexagonal Architecture](./hexagonal-architecture.md) for shared principles and
terminology that apply across all app types.

## Scope

| App                              | Framework                      | Source Root |
| -------------------------------- | ------------------------------ | ----------- |
| `apps/crud-fe-ts-nextjs`         | Next.js (TypeScript)           | `src/`      |
| `apps/crud-fe-ts-tanstack-start` | TanStack Start (TypeScript)    | `src/`      |
| `apps/crud-fe-dart-flutterweb`   | Flutter Web (Dart)             | `lib/`      |
| `apps/crud-fs-ts-nextjs`         | Next.js fullstack (TypeScript) | `src/`      |

All four apps use identical layer names under their source root.

## Layer Directories

| Layer          | Directory         | Purpose                                                        |
| -------------- | ----------------- | -------------------------------------------------------------- |
| Domain         | `domain/`         | Types, entities, business rules with no framework dependencies |
| Application    | `application/`    | Use cases, state management, port interfaces                   |
| Infrastructure | `infrastructure/` | External calls — API clients, local storage, browser APIs      |
| Presentation   | `presentation/`   | UI components, pages, routes, layout                           |

**Full paths by app:**

| App                         | Domain        | Application        | Infrastructure        | Presentation        |
| --------------------------- | ------------- | ------------------ | --------------------- | ------------------- |
| `crud-fe-ts-nextjs`         | `src/domain/` | `src/application/` | `src/infrastructure/` | `src/presentation/` |
| `crud-fe-ts-tanstack-start` | `src/domain/` | `src/application/` | `src/infrastructure/` | `src/presentation/` |
| `crud-fe-dart-flutterweb`   | `lib/domain/` | `lib/application/` | `lib/infrastructure/` | `lib/presentation/` |
| `crud-fs-ts-nextjs`         | `src/domain/` | `src/application/` | `src/infrastructure/` | `src/presentation/` |

## Notes

### `crud-fs-ts-nextjs` — Fullstack App Treated as FE

`crud-fs-ts-nextjs` is a fullstack Next.js app that uses React components alongside
Next.js API routes. For hexagonal layering purposes it is treated as a frontend app: it
generates its own OpenAPI contract internally and does not consume a separate backend
service. DDD bounded-context structure does not apply.

### Dart/Flutter — `infrastructure/` Not `data/`

The Flutter community often names the data-access layer `data/`. ose-primer uses
`infrastructure/` instead to maintain consistent hexagonal terminology across all apps
regardless of language. This makes cross-language navigation predictable.

## References

- [Hexagonal Architecture](./hexagonal-architecture.md) — shared principles and dependency rule

## Principles Implemented/Respected

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)** —
  All four frontend apps use the same four layer names (`domain/`, `application/`,
  `infrastructure/`, `presentation/`), making cross-framework navigation uniform.
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)** —
  The `presentation/` layer is named explicitly rather than conflated with the application layer,
  keeping UI concerns visibly separated from use-case logic.

## Conventions Implemented/Respected

- **[Hexagonal Architecture](./hexagonal-architecture.md)** — This document specializes the
  shared dependency rule and terminology for web/frontend application structure (Next.js,
  TanStack Start, Flutter Web).
