---
title: "PRD: Adopt ose-public Specs Structure"
description: End-state requirements for the specs/ migration
category: plan
---

# PRD: Adopt ose-public Specs Structure

## Product Overview

This migration reorganizes `ose-primer`'s `specs/` directory from a flat-root
layout (`be/`, `fe/`, `c4/`, `contracts/` at app root) to the C4-aware five-folder
layout (`product/`, `system-context/`, `containers/`, `components/`, `behavior/`)
adopted in `ose-public`. The migration also regroups the `rhino` CLI gherkin
feature files from a flat directory into domain subdirectories. The result is a
`specs/` tree that downstream forks can inherit directly without requiring any
further structural migration.

## Personas

| Persona                                                                       | Interest in this migration                                                        |
| ----------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| **Downstream fork author**                                                    | Clones `ose-primer` and gets the correct five-folder baseline immediately         |
| **ose-primer maintainer**                                                     | Executes the migration without CI breakage; keeps atomic commits and green gates  |
| **AI agents** (`specs-checker`, `specs-maker`, `specs-fixer`) [Repo-grounded] | Read updated convention and path examples; no stale guidance in agent definitions |

## Product Risks

| Risk                                                                                             | Mitigation                                                                    |
| ------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------- |
| README skeleton files generated with incorrect relative link depths (too shallow or too deep)    | Use R3 template with per-nesting-level adjustment note; validate with lint:md |
| Nx cache inputs in `project.json` still point to old paths after migration, breaking CI silently | Phase 2.8 grep verification confirms zero old-path references before push     |
| `docs-checker` or `repo-rules-checker` finds stale path references in governance docs post-push  | Phase 4 governance propagation (repo-rules-maker) sweeps all referenced files |
| Partial migration leaves broken cross-links between spec files and governance docs               | Atomic commit discipline — all moves and path sweeps land in a single commit  |

## End State: crud spec tree

```
specs/apps/crud/
├── README.md
├── product/                    # (placeholder — future PM-first content)
├── system-context/
│   ├── README.md
│   └── context.md              # moved from c4/context.md
├── containers/
│   ├── README.md
│   ├── container.md            # moved from c4/container.md
│   └── contracts/              # moved from contracts/
├── components/
│   ├── README.md
│   ├── be/
│   │   ├── README.md
│   │   └── component-be.md     # moved from c4/component-be.md
│   └── web/
│       ├── README.md
│       └── component-web.md    # moved + renamed from c4/component-fe.md
└── behavior/
    ├── README.md
    ├── be/                     # moved from be/
    │   ├── README.md
    │   └── gherkin/
    └── web/                    # moved + renamed from fe/
        ├── README.md
        └── gherkin/
```

No `c4/`, `be/`, `fe/`, or `contracts/` at root.

## End State: rhino spec tree

```
specs/apps/rhino/
├── README.md
├── product/
│   ├── README.md
│   └── overview.md
├── system-context/
│   ├── README.md
│   └── context.md
├── containers/
│   ├── README.md
│   └── container.md
├── components/
│   ├── README.md
│   └── cli/
│       ├── README.md
│       └── component-cli.md
└── behavior/
    ├── README.md
    └── cli/
        └── gherkin/
            ├── README.md
            ├── agents/          # agents-*.feature
            ├── contracts/       # contracts-*.feature
            ├── docs/            # docs-validate-*.feature
            ├── env/             # env-*.feature
            ├── git/             # git-pre-commit.feature
            ├── java/            # java-validate-annotations.feature
            ├── repo-governance/ # repo-governance-vendor-audit.feature
            ├── spec-coverage/   # spec-coverage-validate.feature
            ├── system/          # doctor.feature
            ├── test-coverage/   # test-coverage-*.feature
            └── workflows/       # workflows-validate-naming.feature
```

No flat `*.feature` files directly under `behavior/cli/gherkin/`.

## End State: convention doc

`repo-governance/conventions/structure/specs-directory-structure.md` [Repo-grounded]
reflects the C4-aware five-folder tree, the `behavior/<surface>/gherkin/<domain>/`
canonical path, and the retired CLI flat-structure exception. All path examples
use new paths.

## End State: project.json files

All `apps/crud-*/project.json` [Repo-grounded] Nx cache inputs and `spec-coverage`
commands use `behavior/be/gherkin` and `behavior/web/gherkin` (not `be/gherkin` /
`fe/gherkin`).

## Non-Goals

- Adding real content to `product/`, `system-context/`, `containers/`,
  `components/` for `crud` — placeholder READMEs are sufficient for this plan.
- Migrating `specs/apps-labs/` — no app specs exist there.
- Migrating library specs (`specs/libs/`) — already compliant.

## User Stories

As a downstream fork author cloning `ose-primer`,
I want the `specs/` tree to already follow the C4-aware five-folder layout,
so that I don't have to run a second structural migration after forking.

As the `ose-primer` maintainer executing this migration,
I want all file moves, path sweeps, and project.json updates to land in a single
atomic commit per phase,
so that no intermediate state breaks CI between moves and reference updates.

As the `specs-checker` agent [Repo-grounded] validating a downstream fork's specs directory,
I want the convention doc (`specs-directory-structure.md`) [Repo-grounded] to describe
the C4-aware five-folder layout with domain subdirectories,
so that my validation logic references correct canonical paths and does not flag
compliant directories as violations.

## Acceptance Criteria

```gherkin
Feature: C4-aware specs tree for crud

  Scenario: No flat-root artifacts remain in crud
    Given the adopt-ose-public-specs-structure plan has been executed
    When I run `find specs/apps/crud -maxdepth 1 -type d`
    Then the output does not contain "be", "fe", "c4", or "contracts"
    And the output contains "behavior", "system-context", "containers", "components"

  Scenario: Nx spec-coverage targets resolve after migration
    Given the project.json files have been updated
    When I run `npx nx run crud-be-golang-gin:spec-coverage`
    Then the command exits 0
    And no "no such file" errors appear in the output

  Scenario: No flat CLI feature files remain in rhino
    Given the rhino gherkin tree has been regrouped
    When I run `find specs/apps/rhino/behavior/cli/gherkin -maxdepth 1 -name "*.feature"`
    Then the output is empty
```
