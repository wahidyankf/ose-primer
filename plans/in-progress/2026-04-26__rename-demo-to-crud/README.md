# Plan: Rename `demo-*` → `crud-*`

## Purpose

Rename every `demo-*` app, spec, infra, workflow, and doc to `crud-*` so the CRUD
family has a precise, purpose-scoped name before a second family (`ai-chat-*` or
similar) is introduced alongside it.

## Scope summary

| Category                               | Count        | Example change                                                  |
| -------------------------------------- | ------------ | --------------------------------------------------------------- |
| `apps/demo-*` directories              | 17           | `apps/demo-be-golang-gin` → `apps/crud-be-golang-gin`           |
| `specs/apps/demo/` tree                | 1 root move  | `specs/apps/demo/` → `specs/apps/crud/`                         |
| `infra/dev/demo-*` directories         | 15           | `infra/dev/demo-be-golang-gin` → `infra/dev/crud-be-golang-gin` |
| Nx project names (`project.json`)      | 18           | `"demo-contracts"` → `"crud-contracts"`                         |
| npm scripts (`package.json`)           | ~18 scripts  | `dev:demo-be-golang-gin` → `dev:crud-be-golang-gin`             |
| Docker DB credentials                  | 11 prefixes  | `demo_be_ggn` → `crud_be_ggn`                                   |
| `.github/workflows/test-demo-*` files  | 15 files     | `git mv` + content sweep                                        |
| Governance workflows referencing demo  | audit + fix  | `governance/workflows/`                                         |
| Documentation (`docs/`, `governance/`) | audit + fix  | all `.md` files                                                 |
| `CLAUDE.md`, `README.md`, `AGENTS.md`  | bulk replace | workspace root files                                            |

## Documents

- [Business rationale](./brd.md) — why now, what it unblocks
- [Product requirements](./prd.md) — what done looks like, Gherkin acceptance criteria
- [Technical approach](./tech-docs.md) — rename strategy, file change map, validation
- [Delivery checklist](./delivery.md) — granular step-by-step tasks
