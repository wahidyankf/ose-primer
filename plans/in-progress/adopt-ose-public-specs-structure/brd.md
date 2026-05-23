---
title: "BRD: Adopt ose-public Specs Structure"
description: Business requirements for aligning ose-primer specs layout with ose-public convention
category: plan
---

# BRD: Adopt ose-public Specs Structure

## Problem Statement

`ose-primer` diverges from `ose-public`'s canonical `specs/` layout. New forks
cloning `ose-primer` inherit the old flat-root convention and require a second
migration when adopting `ose-public` governance. The convention doc
(`specs-directory-structure.md`) and all governance docs still reference the
old paths, creating documentation drift.

## Business Requirements

| ID   | Requirement                                                                                                           |
| ---- | --------------------------------------------------------------------------------------------------------------------- |
| BR-1 | `ose-primer` spec tree must match `ose-public`'s C4-aware five-folder layout so forks start from the correct baseline |
| BR-2 | Convention doc must be the single source of truth — no stale path examples                                            |
| BR-3 | All Nx cache inputs and `spec-coverage` commands must resolve after migration                                         |
| BR-4 | The active `add-investment-oracle-app` plan must reference the new paths so the new app is scaffolded correctly       |
| BR-5 | Migration must not break any pre-push or CI hooks — one atomic commit preserves history integrity                     |

## Success Criteria

- `find specs/apps/crud -maxdepth 1 -type d` returns only the five canonical dirs plus `README.md` — no `be/`, `fe/`, `c4/`, `contracts/` at root
- `find specs/apps/rhino/behavior/cli/gherkin -maxdepth 1 -name "*.feature"` returns empty (all features moved to domain subdirs)
- `grep -r "specs/apps/crud/be/gherkin\|specs/apps/crud/fe/gherkin\|specs/apps/crud/c4" --include="*.json" apps/` returns empty
- `npm run lint:md` passes on all updated files
