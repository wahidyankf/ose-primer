---
title: "Specs Directory Structure Convention"
description: "Canonical directory structure for specs/ using a C4-aware five-folder layout (product, system-context, containers, components, behavior) and domain subdirectories for every surface"
category: explanation
subcategory: conventions
tags:
  - conventions
  - specs
  - gherkin
  - directory-structure
  - organization
  - c4-aware
  - five-folder
  - openapi
---

# Specs Directory Structure Convention

The `specs/` directory contains all behavioral specifications (Gherkin feature files), architectural diagrams (C4), and API contracts (OpenAPI) for the monorepo. This convention codifies the canonical directory structure that all projects must follow.

## Principles Implemented/Respected

This convention implements the following core principles:

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: The directory structure communicates spec scope through path segments. Reading a path like `specs/apps/crud/behavior/be/gherkin/expenses/expense-management.feature` immediately reveals the project, C4 layer, surface, domain, and feature without any external metadata or configuration.

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: The five-folder layout maps directly to C4 model levels, giving each concern (product, context, containers, components, behavior) a dedicated home. Domain subdirectories apply uniformly to all surfaces, including CLI, eliminating special-case rules.

- **[Documentation First](../../principles/content/documentation-first.md)**: The specs directory serves as living documentation of system behavior. Gherkin features describe what the system does in human-readable language, C4 diagrams describe architectural context, and OpenAPI contracts describe API surfaces.

## Conventions Implemented/Respected

This convention implements/respects the following conventions:

- **[Specs-Application Sync Convention](../../development/quality/specs-application-sync.md)**: The directory structure enables bidirectional sync between specs and application code. The path pattern mirrors the app/lib structure in the workspace.

- **[BDD Spec-Test Mapping](../../development/infra/bdd-spec-test-mapping.md)**: The Gherkin directory structure directly supports the mapping between feature files and test implementations across all three testing levels.

- **[Three-Level Testing Standard](../../development/quality/three-level-testing-standard.md)**: All three test levels (unit, integration, E2E) consume the same Gherkin specs from this directory structure. Only step implementations differ.

## Purpose

This convention establishes the canonical directory layout for the `specs/` directory. It defines how Gherkin feature files, C4 architecture diagrams, and OpenAPI contracts are organized across apps and libs using a C4-aware five-folder layout, ensuring consistency, discoverability, and correct tool integration.

## Scope

### What This Convention Covers

- **Gherkin feature file placement** for apps (BE, web, CLI) and libs
- **Domain subdirectory rules** for grouping related feature files across all surfaces
- **C4 diagram placement** within the five-folder structure per app
- **OpenAPI contract placement** within the `containers/` folder per app
- **README.md index files** at each navigational level

### What This Convention Does NOT Cover

- **Gherkin writing standards** (covered by [Acceptance Criteria Convention](../../development/infra/acceptance-criteria.md))
- **C4 diagram content** (covered by C4 model documentation within each project)
- **OpenAPI spec authoring** (covered by contract project documentation)
- **Test implementation** (covered by [Three-Level Testing Standard](../../development/quality/three-level-testing-standard.md))

## Canonical Path Pattern

### Five-Folder C4-Aware Layout

Every app spec root uses the following five top-level folders:

```
specs/apps/{app-name}/
├── product/            # PM-first product documentation
├── system-context/     # C4 Level 1 — System Context
├── containers/         # C4 Level 2 — Container diagram + API contracts
├── components/         # C4 Level 3 — Component diagrams per surface
└── behavior/           # Behavioral specs (Gherkin) per surface
```

### Canonical Behavior Path

The canonical path pattern for Gherkin feature files is:

```
specs/apps/{app-name}/behavior/{surface}/gherkin/{domain}/{feature}.feature
```

Where:

- **`{surface}`** = `be`, `web`, or `cli`
- **`{domain}`** = business domain grouping folder (e.g., `expenses/`, `authentication/`, `health/`)
- **`{feature}`** = feature file name describing the behavior

The `fe` surface identifier is retired. Use `web` for all frontend Gherkin specs.

### Domain Subdirectory Rules

Domain subdirectories are required under `gherkin/` for **all surfaces**, including CLI. There is no flat-structure exception.

**BE specs** use domain subdirectories:

```
specs/apps/crud/behavior/be/gherkin/expenses/expense-management.feature
specs/apps/crud/behavior/be/gherkin/expenses/attachments.feature
specs/apps/crud/behavior/be/gherkin/authentication/password-login.feature
```

**Web specs** use domain subdirectories:

```
specs/apps/crud/behavior/web/gherkin/accessibility/accessibility.feature
specs/apps/crud/behavior/web/gherkin/authentication/google-login.feature
```

**CLI specs** also use domain subdirectories:

```
specs/apps/rhino/behavior/cli/gherkin/system/doctor.feature
specs/apps/rhino/behavior/cli/gherkin/env/env-backup.feature
specs/apps/rhino/behavior/cli/gherkin/spec-coverage/spec-coverage-validate.feature
```

Group CLI feature files by command group or functional area. A domain folder may contain one or many feature files.

**Lib specs** use package or module subdirectories under `gherkin/` (no surface segment because libs do not have BE/web/CLI layers):

```
specs/libs/golang-commons/gherkin/testutil/capture-stdout.feature
specs/libs/golang-commons/gherkin/timeutil/timestamp.feature
specs/libs/ts-ui/gherkin/button/button.feature
specs/libs/ts-ui/gherkin/dialog/dialog.feature
specs/libs/golang-commons/gherkin/links/check-links.feature
```

## Full Directory Structure

The complete `specs/` directory follows this layout:

```
specs/
├── README.md
├── apps/
│   └── {app-name}/
│       ├── README.md
│       ├── product/                         # PM-first product documentation
│       │   ├── README.md
│       │   └── overview.md
│       ├── system-context/                  # C4 Level 1 — System Context diagram
│       │   ├── README.md
│       │   └── context.md
│       ├── containers/                      # C4 Level 2 — Container diagram + API contracts
│       │   ├── README.md
│       │   ├── container.md
│       │   └── contracts/                   # OpenAPI specs (if applicable)
│       │       ├── README.md
│       │       └── openapi.yaml
│       ├── components/                      # C4 Level 3 — Component diagrams
│       │   ├── README.md
│       │   ├── be/
│       │   │   ├── README.md
│       │   │   └── component-be.md
│       │   └── web/
│       │       ├── README.md
│       │       └── component-web.md
│       └── behavior/                        # Behavioral specs (Gherkin)
│           ├── README.md
│           ├── be/
│           │   ├── README.md
│           │   └── gherkin/
│           │       ├── README.md
│           │       └── {domain}/            # Domain subdirs (required)
│           │           └── {feature}.feature
│           ├── web/
│           │   ├── README.md
│           │   └── gherkin/
│           │       ├── README.md
│           │       └── {domain}/            # Domain subdirs (required)
│           │           └── {feature}.feature
│           └── cli/
│               └── gherkin/
│                   ├── README.md
│                   └── {domain}/            # Domain subdirs (required for CLI too)
│                       └── {command}.feature
├── libs/
│   └── {lib-name}/
│       ├── README.md
│       └── gherkin/
│           └── {package}/
│               └── {feature}.feature
└── apps-labs/
    └── README.md
```

### Which Projects Have Which Directories

Not every project has all directories. The presence of `product/`, `system-context/`, `containers/`, `components/`, or specific surface directories under `behavior/` depends on the project:

- **`product/`**: Present for projects with PM-authored product documentation
- **`system-context/`**: Present for multi-layer app groups with a defined system boundary
- **`containers/`**: Present for apps with container-level architecture documentation; `contracts/` is nested here for apps with OpenAPI contract specs
- **`components/`**: Present for apps with component-level C4 diagrams; `be/` and `web/` subdirectories created only for layers that exist
- **`behavior/`**: Present for all apps with Gherkin specs; surface subdirectories (`be/`, `web/`, `cli/`) created only for layers that exist

## README Index Files

Each navigational directory level should contain a `README.md` file that indexes its contents. This follows the same GitHub compatibility pattern used throughout the repository (see [File Naming Convention](./file-naming.md)).

README files serve as entry points when browsing the specs directory on GitHub, providing context about what specifications exist at each level.

## Adding New Specs

### Adding Specs for a New Project

1. Create the project directory under `specs/apps/{name}/` or `specs/libs/{name}/`
2. Create a `README.md` at the project level
3. Create the five top-level folders (`product/`, `system-context/`, `containers/`, `components/`, `behavior/`) as needed for the project, each with its own `README.md`
4. Under `behavior/`, create the applicable surface directories (`be/`, `web/`, `cli/`) with `gherkin/` subdirectories
5. For `containers/`, add `contracts/` with the OpenAPI structure if the app exposes an API
6. For `components/`, add `be/` and `web/` subdirectories matching the app's surface coverage

### Adding a Feature File to an Existing Project

1. Identify the correct surface (`be`, `web`, or `cli`)
2. Navigate to `behavior/{surface}/gherkin/`
3. For all surfaces (BE, web, CLI): place the file in the appropriate domain subdirectory, creating the domain folder if it does not exist
4. Update the relevant `README.md` index file

### Adding Specs for a New Lib

1. Create `specs/libs/{lib-name}/`
2. Create a `README.md` at the lib level
3. Create `gherkin/` directly under the lib name (no surface segment)
4. Create package subdirectories under `gherkin/` matching the lib's module structure

## Keeping `specs/apps/rhino/` in Sync

The `specs/apps/rhino/` tree documents rhino-cli behavior: Gherkin scenarios under `behavior/`, container and component descriptions, and README claims. This tree has two update sources:

- **Behavior-driven development**: when rhino-cli gains new functionality, contributors update `specs/apps/rhino/` first (spec-first), then update the CLI implementation to match.
- **Harness convention changes**: when the `repo-harness-compatibility-quality-gate` workflow detects that an upstream harness changed a convention that rhino-cli emits, the `repo-harness-compatibility-fixer` agent edits the affected `specs/apps/rhino/` files as part of the same fix pass — preserving Given-When-Then structure (and the one-each keyword rule: exactly one primary `Given`, one `When`, one `Then` per scenario — see [HARD Rule — Step-Keyword Cardinality](../../development/infra/acceptance-criteria.md#hard-rule--step-keyword-cardinality)) and recording each touched spec file in the fix report.

Both sources are additive: the spec remains the authoritative description of rhino-cli behavior regardless of which update path triggered the change.

## Enforcement

### Automated Validation

The `rhino-cli spec-coverage validate` command validates that all Gherkin feature files under `specs/` have corresponding test implementations. It uses recursive globs (`**/*.feature`) to discover feature files, so it works correctly with all surfaces and nested domain subdirectory structures.

The `spec-coverage` target runs as part of the pre-push hook for projects that have it configured. It ensures specs and application code stay synchronized.

### Manual Verification

When reviewing changes to the `specs/` directory, verify:

- [ ] Feature files follow the canonical path pattern for their surface type
- [ ] BE and web specs use domain subdirectories (never flat under `gherkin/`)
- [ ] CLI specs use domain subdirectories under `gherkin/` (not flat)
- [ ] Lib specs use package subdirectories under `gherkin/`
- [ ] README.md index files exist at each navigational level
- [ ] New projects include the five-folder scaffolding appropriate to their scope
- [ ] No `fe/` surface paths exist (use `web/` instead)

## Migration Path

This convention supersedes the flat-root layout (`be/`, `fe/`, `c4/`, `contracts/` at app root)
used in earlier versions. The migration recipe is documented in:

- **ose-public**: `plans/done/2026-05-24__specs-tree-uniform/`
- **ose-primer adoption**: `plans/in-progress/adopt-ose-public-specs-structure/` (2026-05-24)

### Flat-Root to C4-Aware Mapping Table

| Old path (flat-root)                          | New path (C4-aware)                             |
| --------------------------------------------- | ----------------------------------------------- |
| `{app}/be/gherkin/`                           | `{app}/behavior/be/gherkin/`                    |
| `{app}/fe/gherkin/`                           | `{app}/behavior/web/gherkin/`                   |
| `{app}/c4/context.md`                         | `{app}/system-context/context.md`               |
| `{app}/c4/container.md`                       | `{app}/containers/container.md`                 |
| `{app}/c4/component-be.md`                    | `{app}/components/be/component-be.md`           |
| `{app}/c4/component-fe.md`                    | `{app}/components/web/component-web.md`         |
| `{app}/contracts/`                            | `{app}/containers/contracts/`                   |
| `{app}/behavior/cli/gherkin/*.feature` (flat) | `{app}/behavior/cli/gherkin/{domain}/*.feature` |

## Related Documentation

- [Specs-Application Sync Convention](../../development/quality/specs-application-sync.md) - Bidirectional sync between specs and application code
- [BDD Spec-Test Mapping](../../development/infra/bdd-spec-test-mapping.md) - How specs map to test implementations
- [Three-Level Testing Standard](../../development/quality/three-level-testing-standard.md) - Unit, integration, and E2E testing levels consuming these specs
- [Acceptance Criteria Convention](../../development/infra/acceptance-criteria.md) - Gherkin writing standards for feature files
- [File Naming Convention](./file-naming.md) - General file naming patterns (README.md exception applies here)
- [Plans Organization Convention](./plans.md) - Similar convention for plans/ directory structure
- [Multi-Harness Binding Convention](./multi-harness-binding.md) - Harness-compatibility fixer's obligation to keep `specs/apps/rhino/` consistent with catalog and binding changes

---
