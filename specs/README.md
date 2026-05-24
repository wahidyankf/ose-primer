# Specs

Gherkin acceptance specifications for applications in this monorepo.

## What This Is

This directory holds executable specifications written in Gherkin — the shared language between
business stakeholders, developers, and QA engineers. These specs describe _what_ each app does,
not _how_ it is implemented.

## Why Specs Live Here

Acceptance specs belong at the monorepo root rather than inside app directories because:

- **Stakeholder access** — business owners and QA engineers read specs without navigating app internals
- **Shared ownership** — Three Amigos (business + development + QA) collectively own these files
- **Clear separation** — specs define behavior; implementation tests live inside the apps

## Testing Layers

| Layer                      | Location           | Purpose                               | When it runs            |
| -------------------------- | ------------------ | ------------------------------------- | ----------------------- |
| Acceptance specs (Gherkin) | `specs/`           | Define behavior from user perspective | CI full suite           |
| Unit / integration tests   | `apps/*/src/test/` | Verify internal implementation        | Pre-push (`test:quick`) |
| E2E tests                  | `apps/*-e2e/`      | Verify flows against running system   | CI E2E suite            |

## App Specs

- **[crud/](./apps/crud/README.md)** — CRUD application specifications
  (platform-agnostic Gherkin — see [be/gherkin](./apps/crud/behavior/be/gherkin/README.md) and [web/gherkin](./apps/crud/behavior/web/gherkin/README.md) for details)
- **[rhino/](./apps/rhino/README.md)** — Repository management CLI specifications (Go, godog)

## Experimental App Specs

- **[apps-labs/](./apps-labs/README.md)** — Specs for framework evaluations, POCs, and tech stack
  comparisons; graduates to `apps/` when the implementation is promoted

## Library Specs

- **[golang-commons/](./libs/golang-commons/)** — Shared Go utility specifications (godog)

## Standard Folder Pattern

Each application domain follows the C4-aware five-folder layout under `specs/apps/{domain}/`:

```
specs/apps/{domain}/
├── README.md               # Describes app, BDD framework, and feature organization
├── product/                # Product-level docs (vision, stakeholders, personas)
├── system-context/         # C4 Level 1: context diagrams
├── containers/             # C4 Level 2: container diagrams + API contracts
│   └── contracts/          # OpenAPI 3.1 contract spec (bundled + source files)
├── components/             # C4 Level 3: component diagrams per surface
│   ├── be/                 # Backend component diagram
│   └── web/                # Frontend component diagram (or cli/, etc.)
└── behavior/               # Gherkin acceptance specs, by surface
    ├── be/gherkin/         # Backend acceptance specs (.feature files)
    ├── web/gherkin/        # Frontend acceptance specs (.feature files)
    └── cli/gherkin/        # CLI acceptance specs (.feature files, if applicable)
```

Where `{surface}` in `behavior/{surface}/gherkin/` is one of:

- `be` — backend service (REST API, GraphQL, etc.)
- `web` — frontend application (Next.js, Flutter, etc.)
- `cli` — CLI tool (Go, etc.)

**Contracts** live at `specs/apps/{domain}/containers/contracts/` and are the source of truth for
API contracts shared between frontend and backend. The `{domain}-contracts` Nx project lints and
bundles the spec; downstream apps consume it via their `codegen` target.

**C4 diagrams** are distributed across `system-context/`, `containers/`, and `components/` per C4
model level. See the [Specs Directory Structure Convention](../repo-governance/conventions/structure/specs-directory-structure.md)
for the normative rules.

## Standards

All feature files follow the BDD standards:

- [BDD Standards](../docs/explanation/software-engineering/development/behavior-driven-development-bdd/README.md) —
  framework requirements, Three Amigos process, coverage rules
- [Gherkin Standards](../docs/explanation/software-engineering/development/behavior-driven-development-bdd/gherkin-standards.md) —
  feature file structure, naming, ubiquitous language
- [Scenario Standards](../docs/explanation/software-engineering/development/behavior-driven-development-bdd/scenario-standards.md) —
  scenario independence, naming, assertions
- [Spec-to-Test Mapping](../repo-governance/development/infra/bdd-spec-test-mapping.md) —
  mandatory 1:1 mapping between CLI commands and feature file `@tags`

## Adding Specs

1. Choose the appropriate subdirectory: `specs/apps/` for production-bound applications,
   `specs/apps-labs/` for experimental/POC applications, `specs/libs/` for libraries
2. Create a folder matching the project name: `specs/apps/[app-name]/` or `specs/libs/[lib-name]/`
3. Add a `README.md` describing the project, BDD framework, and feature file organization
4. Organize `.feature` files by bounded context or user journey (kebab-case names)
5. Update this README with a link to the new folder
