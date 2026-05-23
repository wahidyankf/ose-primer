---
title: "PRD: Adopt ose-public Specs Structure"
description: End-state requirements for the specs/ migration
category: plan
---

# PRD: Adopt ose-public Specs Structure

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

`repo-governance/conventions/structure/specs-directory-structure.md` reflects
the C4-aware five-folder tree, the `behavior/<surface>/gherkin/<domain>/`
canonical path, and the retired CLI flat-structure exception. All path examples
use new paths.

## End State: project.json files

All `apps/crud-*/project.json` Nx cache inputs and `spec-coverage` commands
use `behavior/be/gherkin` and `behavior/web/gherkin` (not `be/gherkin` /
`fe/gherkin`).

## Non-Goals

- Adding real content to `product/`, `system-context/`, `containers/`,
  `components/` for `crud` — placeholder READMEs are sufficient for this plan.
- Migrating `specs/apps-labs/` — no app specs exist there.
- Migrating library specs (`specs/libs/`) — already compliant.
