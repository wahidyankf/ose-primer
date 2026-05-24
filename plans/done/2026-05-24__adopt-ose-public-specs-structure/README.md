---
title: "Adopt ose-public Specs Structure"
description: Migrate specs/ from flat-root layout to the C4-aware five-folder tree and domain-subdir CLI convention adopted in ose-public
category: plan
status: in-progress
---

# Plan: Adopt ose-public Specs Structure

**Status**: In Progress
**Owner**: Maintainer
**Started**: 2026-05-24

## Purpose

Align `ose-primer`'s `specs/` directory with the C4-aware five-folder convention
adopted in `ose-public`. The migration touches two app spec trees (`crud` and
`rhino`), the convention doc, every governance doc that references old paths,
all `project.json` Nx cache inputs, and the active in-progress plan.

## Background

`ose-public` completed the `specs-tree-uniform` plan (2026-05-23) that
established two new rules:

1. **Five-folder C4-aware tree** for every app spec root: `product/`,
   `system-context/`, `containers/`, `components/`, `behavior/`.
2. **Domain subdirectories for every surface** including CLI — the previously
   permitted flat CLI exception is retired.

`ose-primer` still uses the old flat-root layout (`be/`, `fe/`, `c4/`,
`contracts/` at root) for `crud`, and flat `behavior/cli/gherkin/*.feature`
for `rhino`. Both must be migrated.

## Approach Summary

Execute two atomic structural commits (crud migration, rhino migration), then propagate path
changes to all governance docs and project.json files via `repo-rules-maker`, then run
quality gates. All `git mv` operations and their corresponding reference updates land in
the same commit — no intermediate broken state is ever pushed.

## Git Execution Context

This plan executes directly on `main` — no worktree. Commits are pushed to `origin main`.

## Scope

| Area                                                                 | Change                                                                          |
| -------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| `specs/apps/crud/`                                                   | Flat-root → five-folder; `fe` surface renamed to `web`                          |
| `specs/apps/rhino/`                                                  | Add four missing top-level folders; group flat CLI features into domain subdirs |
| `repo-governance/conventions/structure/specs-directory-structure.md` | Replace with new C4-aware convention (via repo-rules-maker)                     |
| 9 governance docs                                                    | Update path examples and cross-links (via repo-rules-maker)                     |
| 17 `apps/crud-*/project.json`                                        | Update Nx cache-input globs and spec-coverage commands                          |
| `specs/README.md`, `README.md`                                       | Update Standard Folder Pattern section                                          |
| `plans/in-progress/add-investment-oracle-app/`                       | Update path references in active plan                                           |

## Atomic Commit Requirement

Per the migration recipe in `specs-directory-structure.md`, all file moves
(`git mv`) and every corresponding path-reference update must land in ONE
atomic commit. No intermediate commit may leave broken paths.

## Documents

- [brd.md](./brd.md) — business requirements
- [prd.md](./prd.md) — product (end-state) requirements
- [tech-docs.md](./tech-docs.md) — migration mapping and technical details
- [delivery.md](./delivery.md) — granular task checklist

## Related

- [Specs Directory Structure Convention](../../../repo-governance/conventions/structure/specs-directory-structure.md)
- [Plans Organization Convention](../../../repo-governance/conventions/structure/plans.md)
