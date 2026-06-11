# standardize-app-spec-trees (ose-primer)

## Context

The `specs/apps/<family>/` tree in this repository uses a **bare-surface** behavior naming
scheme: `specs/apps/crud/behavior/be/`, `specs/apps/crud/behavior/web/`, and
`specs/apps/rhino/behavior/cli/`. Across the three-repo ecosystem (ose-public, ose-primer,
ose-infra) this scheme is being standardized to a **flat product-surface** scheme where every
behavior directory states both its product and its perspective in one self-describing token:
`specs/apps/<family>/behavior/<product>-<surface>/gherkin/`.

This plan covers the **ose-primer** share of that ecosystem-wide standardization: rename this
repo's two families (`crud`, `rhino`) to the flat product-surface scheme, rewire every consumer,
codify the rule in the shared convention (byte-identical to the ose-public sibling plan), update
the `specs-checker`/`specs-maker` agents, and record the rationale.

This is a **planning artifact**. It is authored now and executed later via the
[plan-execution workflow](../../../repo-governance/workflows/plan/plan-execution.md).

## Scope

### In scope

- Adopt the LOCKED flat product-surface naming scheme (decision 1 of the shared decisions brief).
- Active remediation (`git mv` + consumer rewiring) for the two ose-primer families:
  - `crud`: `behavior/be` → `behavior/crud-be`, `behavior/web` → `behavior/crud-web`.
  - `rhino`: `behavior/cli` → `behavior/rhino-cli`.
- Amend `repo-governance/conventions/structure/specs-directory-structure.md` with the flat
  product-surface rule, the `be`-over-`api` rule, and worked examples. The amendment **text** is
  byte-identical to the ose-public sibling plan's amendment (conventions are bidirectional/identity
  in the primer-sync classifier).
- Update `.claude/agents/specs-checker.md` and `.claude/agents/specs-maker.md`; re-sync platform
  bindings via `npm run generate:bindings`.
- Write the rationale doc at
  `docs/explanation/standardize-app-spec-trees-parity-decisions.md`.

### Out of scope

- Renaming any family in ose-public or ose-infra (each repo owns its own families).
- Multi-product family consolidation (`ose-app` + `ose-platform` → `ose`) — ose-primer has no
  multi-product family.
- Any `api` → `be` rename — ose-primer has **no** `api` behavior surface (only `be`/`web`/`cli`).
- Renaming Nx **projects** or app directories under `apps/` (only the `specs/` behavior dirs and
  their consumers move).
- Contracts project rename — ose-primer has no contracts-project rename in this parity round.

## Approach Summary

Each family is restructured behind a `git mv` of its behavior directory, followed by a sweep of
every consumer that references the old path (`project.json` spec-coverage commands and inputs,
`*-e2e` playwright configs, app/specs READMEs, the `rhino-cli` Rust unit-test path defaults, the
generated `.features-gen/` tree, and governance/docs cross-references). The convention amendment
and agent updates promote the new scheme to the enforced standard. Delivery is **main-to-main**
(docs-and-structure change, low risk) — a recorded deviation from the ose-primer Sync Convention
PR-only default (Safety Invariant 6), justified in `tech-docs.md`.

## Documents

- [brd.md](./brd.md) — Business Requirements (WHY).
- [prd.md](./prd.md) — Product Requirements (WHAT) with Gherkin acceptance criteria.
- [tech-docs.md](./tech-docs.md) — Architecture, full file-impact map, cross-repo deviation matrix,
  delivery-mode deviation justification.
- [delivery.md](./delivery.md) — Phased, gated delivery checklist.

## Sibling Plans

This plan is one leg of a coordinated three-repo parity effort. The sibling plans:

- **ose-public**: `plans/in-progress/standardize-app-spec-trees/README.md`
- **ose-infra**: `plans/in-progress/standardize-app-spec-trees/README.md`

The cross-repo deviation matrix (in [tech-docs.md](./tech-docs.md)) records where the three plans
align and where each repo deviates.
