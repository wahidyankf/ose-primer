# crud-fe-ts-nextjs

Demo Frontend - Next.js 16 (App Router) implementation consuming the
[crud-be API](../crud-be-golang-gin/README.md).

## Overview

- **Framework**: Next.js 16.1 (App Router)
- **Language**: TypeScript
- **UI Library**: React 19
- **State Management**: TanStack Query v5
- **BDD Tool**: @amiceli/vitest-cucumber
- **Port**: 3301
- **Specs**: [`specs/apps/crud/fe/gherkin/`](../../specs/apps/crud/fe/gherkin/README.md)

## Prerequisites

- **Node.js 24** (managed by Volta)
- **npm 11**
- A running [crud-be backend](../crud-be-golang-gin/README.md) on port 8201 (for E2E tests)

## Nx Commands

```bash
# Start development server (localhost:3301)
nx dev crud-fe-ts-nextjs

# Production build
nx build crud-fe-ts-nextjs

# Start production server
nx run crud-fe-ts-nextjs:start

# Type checking
nx typecheck crud-fe-ts-nextjs

# Lint code (oxlint)
nx lint crud-fe-ts-nextjs

# Fast quality gate: unit tests + coverage check + specs coverage check
nx run crud-fe-ts-nextjs:test:quick

# Unit tests only
nx run crud-fe-ts-nextjs:test:unit
```

**See**: [Nx Target Standards](../../governance/development/infra/nx-targets.md) for canonical target names.

## Project Structure

```
apps/crud-fe-ts-nextjs/
├── src/
│   ├── app/                      # Next.js App Router pages and layouts
│   ├── components/               # Reusable React components
│   ├── lib/                      # Utilities, API clients, hooks
│   └── test/                     # Test utilities
├── test/
│   └── unit/                     # Unit test step definitions (BDD)
├── Dockerfile                    # Production container image
├── next.config.ts                # Next.js configuration
├── vitest.config.ts              # Vitest configuration (coverage thresholds)
├── tsconfig.json                 # TypeScript configuration
└── project.json                  # Nx targets and tags
```

## Testing

Two levels of testing consume the shared Gherkin scenarios from [`specs/apps/crud/fe/gherkin/`](../../specs/apps/crud/fe/gherkin/README.md):

| Level | Tool                        | Dependencies | Command                              | Cached? |
| ----- | --------------------------- | ------------ | ------------------------------------ | ------- |
| Unit  | @amiceli/vitest-cucumber    | All mocked   | `nx run crud-fe-ts-nextjs:test:unit` | Yes     |
| E2E   | Playwright + playwright-bdd | Full stack   | `nx run crud-fe-e2e:test:e2e`        | No      |

**Coverage**: Measured from `test:unit` only (Vitest v8). `test:quick` = `test:unit` + `rhino-cli test-coverage validate` (>=70%). Both `test:quick` and `test:unit` include `{projectRoot}/src/generated-contracts/**/*` as cache inputs so contract changes invalidate the cache.

### Unit Tests

Steps test component logic and state management with fully mocked dependencies.
No DOM rendering, no HTTP calls:

```bash
nx run crud-fe-ts-nextjs:test:unit
```

### E2E Tests

The [`crud-fe-e2e`](../crud-fe-e2e/) project provides centralized Playwright-based E2E tests
for all crud-fe frontends. Run them after starting this frontend and a backend:

```bash
# Start backend
nx dev crud-be-golang-gin

# Start this frontend (in another terminal)
nx dev crud-fe-ts-nextjs

# Run E2E tests (in another terminal)
BASE_URL=http://localhost:3301 nx run crud-fe-e2e:test:e2e
```

## Docker

Build a production container image:

```bash
docker build -t crud-fe-ts-nextjs:latest apps/crud-fe-ts-nextjs/
```

## Related Documentation

- [Three-Level Testing Standard](../../governance/development/quality/three-level-testing-standard.md) — Unit, integration, and E2E testing boundaries
- [Code Coverage Reference](../../docs/reference/code-coverage.md) — Coverage tools and thresholds
- [Project Dependency Graph](../../docs/reference/project-dependency-graph.md) — Nx dependency visualization
- [Frontend Gherkin Specs](../../specs/apps/crud/fe/gherkin/README.md) — Shared feature files (source of truth)
- [OpenAPI Contract](../../specs/apps/crud/contracts/README.md) — API contract and codegen
- [crud-fe-e2e](../crud-fe-e2e/README.md) — Centralized E2E tests for all crud-fe frontends
