---
title: "BRD: Adopt ose-public Specs Structure"
description: Business requirements for aligning ose-primer specs layout with ose-public convention
category: plan
---

# BRD: Adopt ose-public Specs Structure

## Business Goal

Align `ose-primer`'s `specs/` directory tree with `ose-public`'s C4-aware five-folder
layout so that downstream forks start from the correct baseline and require no second
structural migration after cloning.

## Business Impact

**Pain points without this migration:**

- Downstream fork authors must run a second structural migration after cloning
  `ose-primer`, adding unnecessary friction to project setup.
- The convention doc (`specs-directory-structure.md`) and all governance docs reference
  stale paths, causing documentation drift between `ose-primer` and `ose-public`.
- AI agents (`specs-checker`, `specs-maker`, `specs-fixer`) operate from stale path
  guidance embedded in their definitions, leading to incorrect suggestions.

**Expected benefits after migration:**

- No second migration required — forks start from the correct five-folder baseline.
- Convention doc and all governance docs become accurate and current.
- AI agents read updated convention and path examples with no stale guidance.

## Business Non-Goals

- This migration does NOT add real content to the `product/`, `system-context/`,
  `containers/`, or `components/` folders for `crud` — placeholder READMEs are
  sufficient for this structural plan.
- This migration does NOT migrate `specs/apps-labs/` — no app specs exist there.
- This migration does NOT migrate library specs (`specs/libs/`) — already compliant.
- This migration does NOT introduce new business features or application behavior.

## Affected Roles

| Role                                                                          | Impact                                                                 |
| ----------------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| **ose-primer maintainer**                                                     | Executes migration; must ensure atomic commits and CI green            |
| **Downstream fork authors**                                                   | Inherit the correct baseline; no second migration needed after cloning |
| **AI agents** (`specs-checker`, `specs-maker`, `specs-fixer`) [Repo-grounded] | Read updated convention and path examples; no stale guidance           |

## Business Requirements

| ID   | Requirement                                                                                                           |
| ---- | --------------------------------------------------------------------------------------------------------------------- |
| BR-1 | `ose-primer` spec tree must match `ose-public`'s C4-aware five-folder layout so forks start from the correct baseline |
| BR-2 | Convention doc must be the single source of truth — no stale path examples                                            |
| BR-3 | All Nx cache inputs and `spec-coverage` commands must resolve after migration                                         |
| BR-4 | The active `add-investment-oracle-app` plan must reference the new paths so the new app is scaffolded correctly       |
| BR-5 | Migration must not break any pre-push or CI hooks — one atomic commit preserves history integrity                     |

## Business Risks

| Risk                                                                         | Likelihood | Mitigation                                                                                                 |
| ---------------------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------- |
| Partial migration commit breaks `spec-coverage` Nx targets mid-push          | Medium     | Atomic-commit rule: all `git mv` + path sweeps land in one commit; never push between move and sweep       |
| Missed path reference causes CI failure on first post-push run               | Medium     | Phase 5 quality gate runs `spec-coverage` for sample apps and does a repo-wide stale-path grep before push |
| ose-public convention doc drifts after plan is written                       | Low        | Phase 0 pre-flight checks the convention against this plan's gap inventory before any file moves           |
| Active `add-investment-oracle-app` plan becomes inconsistent after migration | Low        | Phase 4 propagation explicitly targets the three in-progress plan files                                    |

## Success Criteria

- `find specs/apps/crud -maxdepth 1 -type d` returns only the five canonical dirs plus `README.md` — no `be/`, `fe/`, `c4/`, `contracts/` at root
- `find specs/apps/rhino/behavior/cli/gherkin -maxdepth 1 -name "*.feature"` returns empty (all features moved to domain subdirs)
- `grep -r "specs/apps/crud/be/gherkin\|specs/apps/crud/fe/gherkin\|specs/apps/crud/c4" --include="*.json" apps/` returns empty
- `npm run lint:md` passes on all updated files
