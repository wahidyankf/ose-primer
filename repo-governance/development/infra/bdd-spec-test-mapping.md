---
title: "BDD Spec-to-Test Mapping Convention"
description: Gherkin spec consumption rules for CLI apps (1:1 command mapping) and crud-be backends (three-level unit/integration/e2e)
category: explanation
subcategory: development
tags:
  - bdd
  - gherkin
  - integration-testing
  - spec-coverage
  - crud-be
---

# BDD Spec-to-Test Mapping Convention

This convention defines how Gherkin specifications are consumed across the monorepo:

- **CLI apps**: Mandatory 1:1 mapping between commands and Gherkin specs via Godog at both unit and integration test levels
- **Demo-be backends**: Three-level consumption of shared Gherkin specs (unit/integration/e2e) with different step implementations at each level

## Principles Implemented/Respected

This practice respects the following core principles:

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Every command's behavior is explicitly specified in Gherkin before implementation. No undocumented commands.

- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**: `spec-coverage validate` automatically enforces the mapping at file, scenario, and step levels.

- **[Documentation First](../../principles/content/documentation-first.md)**: Specs are written alongside or before the command implementation, serving as living documentation.

## Conventions Implemented/Respected

- **[Acceptance Criteria Convention](./acceptance-criteria.md)**: Feature files follow Gherkin standards defined there, including the HARD rule that every `Scenario` uses exactly one primary `Given`, one `When`, and one `Then` (extras chained with `And`/`But`). See [HARD Rule — Step-Keyword Cardinality](./acceptance-criteria.md#hard-rule--step-keyword-cardinality).

## 📋 CLI Apps: Command-to-Spec Mapping

### Core Rule

**Every command module must have a corresponding `@tag` in a Gherkin feature file under `specs/`.**

Infrastructure modules (`main.rs`, helpers) and parent command modules (e.g., `agents`, `docs`) that do not implement logic are exempt.

## Domain-Prefixed Subcommands

The CLI uses **subcommands** grouped by domain. The domain is the prefix in every artifact:

```
rhino-cli {domain} {action}
rhino-cli {domain} {action}
rhino-cli {domain} {action}
```

## Mapping Layers

The mapping operates at three levels:

### 1. Command to Tag (mandatory)

The `@tag` is derived from the command module filename: replace underscores with hyphens.

| Command Module              | Full Invocation          | Feature `@tag`            |
| --------------------------- | ------------------------ | ------------------------- |
| `agents_sync.rs`            | `agents sync`            | `@agents-sync`            |
| `agents_validate_sync.rs`   | `agents validate-sync`   | `@agents-validate-sync`   |
| `agents_validate_claude.rs` | `agents validate-claude` | `@agents-validate-claude` |
| `docs_validate_links.rs`    | `docs validate-links`    | `@docs-validate-links`    |
| `spec_coverage_validate.rs` | `spec-coverage validate` | `@spec-coverage-validate` |
| `doctor.rs`                 | `doctor`                 | `@doctor`                 |

### 2. Tag to Feature File (flexible)

A feature file may contain **multiple related commands** using separate `Rule` blocks with distinct `@tag` annotations. Semantically related commands (e.g., an action and its validator) can share a feature file:

```gherkin
Feature: Agent Configuration Synchronisation

  @agents-sync
  Rule: agents sync converts .claude/ configuration to .opencode/ format
    Scenario: Syncing converts agents and skills to OpenCode format
    ...

  @agents-validate-sync
  Rule: agents validate-sync confirms .claude/ and .opencode/ are equivalent
    Scenario: Directories that are in sync pass validation
    ...
```

Alternatively, a command with its own distinct domain gets its own feature file:

```
specs/apps/rhino/behavior/cli/gherkin/system/doctor.feature                       <- single @doctor tag
specs/apps/rhino/behavior/cli/gherkin/agents/agents-sync.feature                  <- @agents-sync + @agents-validate-sync
specs/apps/rhino/behavior/cli/gherkin/agents/agents-validate-claude.feature       <- single @agents-validate-claude tag
```

### 3. Unit & Integration Test to Tag (mandatory)

Each command has dedicated test files at both levels that filter scenarios by `@tag`. The same tag is used at both levels, pointing to the same feature file:

**Unit test** (runs in `test:quick`): the unit test harness loads the feature files under `specsDir` and filters scenarios to the `@agents-validate-sync` tag, invoking command logic with mocked I/O.

**Integration test** (runs in `test:integration`): the integration test harness loads the same feature files, filters to the same `@agents-validate-sync` tag, and drives the command against real `/tmp` filesystem fixtures.

## File Naming Convention

| Artifact         | Pattern                                     | Example                                                       |
| ---------------- | ------------------------------------------- | ------------------------------------------------------------- |
| Parent cmd       | `{domain}.rs`                               | `agents.rs`                                                   |
| Command module   | `{domain}_{action}.rs`                      | `agents_validate_sync.rs`                                     |
| Unit test        | `{domain}_{action}_test.rs`                 | `agents_validate_sync_test.rs`                                |
| Integration test | `{domain}_{action}_integration_test.rs`     | `agents_validate_sync_integration_test.rs`                    |
| Feature file     | `specs/{app}/cli/gherkin/{command}.feature` | `specs/apps/rhino/behavior/cli/gherkin/system/doctor.feature` |

**Unit test files** (`{domain}_{action}_test.rs`) serve dual purpose: they contain both BDD step definitions (consuming Gherkin specs) and any non-BDD pure function tests for edge cases not covered by the Gherkin scenarios. The step definitions in unit test files use mocked I/O instead of real filesystem access.

**The universal rule**: All Rust files (command, unit test, integration test) use underscores. Feature files and `@tag`s use hyphens. The `spec-coverage validate` tool normalises hyphens to underscores when matching feature stems to test files.

## ✅ Coverage Enforcement

The `spec-coverage validate` command enforces this mapping at three levels:

1. **File-level**: Every `.feature` file must have a matching `*_test.*` file
2. **Scenario-level**: Every `Scenario:` in the feature must appear as `// Scenario:` comment or `Scenario(...)` call in test code
3. **Step-level**: Every Given/When/Then step must have a matching step definition

Run the check:

```bash
rhino-cli spec-coverage validate specs/apps/rhino apps/rhino-cli
```

**Scope**: Spec-coverage enforcement is currently active for **CLI apps only** (Rust test
conventions for `rhino-cli`). Enforcement for crud-be
backends is **planned but deferred** — the tool needs enhancement to support crud-be test file
naming conventions (e.g., `HealthSteps.java` for Java) which differ
from the CLI app naming patterns the tool currently expects. This will be addressed in a follow-up plan.

## Adding a New Command

New commands are implemented in `rhino-cli`, driven by the Gherkin spec in `specs/apps/rhino/`.

**For `rhino-cli`**:

1. Create the feature file `specs/apps/rhino/{domain}/{domain}-{action}.feature`
2. Create the Rust command module in `apps/rhino-cli/src/cmd/{domain}_{action}.rs`
3. Add unit tests in the same file or `apps/rhino-cli/src/cmd/{domain}_{action}_test.rs`
4. Add integration tests in `apps/rhino-cli/tests/{domain}_{action}_integration_test.rs`
5. Verify: `rhino-cli spec-coverage validate specs/apps/rhino apps/rhino-cli`

## CLI Apps: Dual-Level Spec Consumption

`rhino-cli` consumes Gherkin specs at both the unit and integration test levels. The same feature files in `specs/apps/rhino/` serve as the contract for both levels — only the step implementations differ. `rhino-cli` uses Rust test conventions.

### Architecture

| Level       | Nx Target          | Test File Pattern                       | Step Implementation                        | Dependencies    |
| ----------- | ------------------ | --------------------------------------- | ------------------------------------------ | --------------- |
| Unit        | `test:unit`        | `{domain}_{action}_test.rs`             | Mock dependencies replace I/O              | All mocked      |
| Integration | `test:integration` | `{domain}_{action}_integration_test.rs` | Command logic against real `/tmp` fixtures | Real filesystem |

### Unit-Level Step Definitions

Unit steps call command logic directly with mocked dependencies. Injectable dependencies are overridden in step setup to inject controlled behavior without touching the real filesystem.

- Included in `cargo test` and `test:quick`
- Coverage is measured at this level (≥90% line coverage)
- Must run all Gherkin scenarios for the command's `@tag`

### Integration-Level Step Definitions

Integration steps drive commands against controlled `/tmp` filesystem fixtures. Steps create temporary directory structures, invoke the command, and assert on stdout/stderr and exit code.

- Coverage is NOT measured at this level
- Must run all Gherkin scenarios for the command's `@tag`

### Example: Same Spec, Two Step Implementations

The `@agents-validate-sync` tag lives inside `agents-sync.feature` (shared feature file) and is consumed at both levels:

```
specs/apps/rhino/behavior/cli/gherkin/agents/agents-sync.feature  (contains @agents-sync + @agents-validate-sync)
  -> Rust unit steps in:           apps/rhino-cli/src/cmd/agents_validate_sync_test.rs (or equivalent)
  -> Rust integration steps in:    apps/rhino-cli/tests/agents_validate_sync_integration_test.rs
```

## Demo-be Backend: Three-Level Spec Consumption

All 11 crud-be backends consume the **same shared Gherkin scenarios** from [`specs/apps/crud/behavior/be/gherkin/`](../../../specs/apps/crud/behavior/be/gherkin/README.md) at three test levels. The feature files are the shared contract — only the step implementations change per level.

### Shared Specs

```
specs/apps/crud/behavior/be/gherkin/
├── auth/
│   ├── login.feature
│   ├── register.feature
│   └── ...
├── users/
│   ├── list-users.feature
│   └── ...
└── ... (see gherkin README for full list)
```

### Three Levels

| Level           | Nx Target          | Step Implementations                                        | Dependencies             | What's Real            |
| --------------- | ------------------ | ----------------------------------------------------------- | ------------------------ | ---------------------- |
| **Unit**        | `test:unit`        | Call service/repository functions directly with mocked deps | All mocked               | Application logic only |
| **Integration** | `test:integration` | Call service/repository functions directly with real DB     | Real PostgreSQL (Docker) | Application + database |
| **E2E**         | `test:e2e`         | Playwright HTTP requests to running server                  | Full running server      | Everything             |

### Unit-Level Step Definitions

Unit steps call application service/repository functions directly. All dependencies (database, external APIs) are mocked via in-memory implementations or test doubles.

- No Spring context, no HTTP framework, no database connections
- Steps instantiate services with mocked repositories
- Coverage is measured at this level (≥90% line coverage)
- Must run all shared scenarios

### Integration-Level Step Definitions

Integration steps call application service/repository functions directly against a real PostgreSQL database via docker-compose. No HTTP layer.

- `docker-compose.integration.yml` starts PostgreSQL + test runner
- `Dockerfile.integration` contains language runtime + test execution
- Steps connect to PostgreSQL, run migrations, execute all shared scenarios
- Coverage is NOT measured at this level
- Must run all shared scenarios

### E2E-Level Step Definitions

E2E tests live in `apps/crud-be-e2e/` (shared Playwright suite). Steps make real HTTP requests to a running backend via `playwright-bdd`.

- Runs against any of the 11 backends
- Tests the full HTTP API contract
- Must run all shared scenarios
- Managed by `crud-be-e2e` project, not individual backends

### Validation

To verify all scenarios pass at each level for a given backend:

```bash
# Unit tests (mocked dependencies)
nx run crud-be-{lang}-{framework}:test:unit

# Integration tests (real PostgreSQL via docker-compose)
nx run crud-be-{lang}-{framework}:test:integration

# E2E tests (Playwright HTTP against running backend)
nx run crud-be-e2e:test:e2e
```

All three commands must report all scenarios passing. The Gherkin feature files serve as the single source of truth — if a scenario fails at any level, the backend is non-compliant.

## 🔗 Related Documentation

- [Acceptance Criteria Convention](./acceptance-criteria.md) - Gherkin format standards
- [Specs Directory Structure Convention](../../conventions/structure/specs-directory-structure.md) - Canonical path patterns and domain subdirectory rules
- [Three-Level Testing Standard](../quality/three-level-testing-standard.md) - Mandatory isolation boundaries for unit, integration, and E2E levels where Gherkin specs are consumed
- [Nx Target Standards](./nx-targets.md) - `test:integration` target definitions and caching rules
- [specs/README.md](../../../specs/README.md) - Spec directory organization
- [specs/apps/rhino/README.md](../../../specs/apps/rhino/README.md) - rhino-cli spec structure
- [specs/apps/crud/behavior/be/README.md](../../../specs/apps/crud/behavior/be/README.md) - Demo-be spec structure and three-level consumption
