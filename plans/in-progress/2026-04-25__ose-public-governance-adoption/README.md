# Plan: ose-public Governance Adoption (Apr 2026)

## Overview

Adopt three governance improvements that landed in `ose-public` and are directly applicable to `ose-primer` as a standalone template repo:

| #   | Change                                                                                                     | Source commits                                                  |
| --- | ---------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| A   | **git-push-default convention** — explicit opt-in-PR, no-unsolicited-PR rule for plan agents               | `9abf43f4a`                                                     |
| B   | **no-date-metadata convention** — strip manual date fields from all non-website markdown, git is the truth | `76304d7b8`                                                     |
| C   | **rhino-cli `docs validate-mermaid`** — new command + internal/mermaid package + Nx target + pre-push wire | `655e0f048`, `4c8397b88`, `1474dcf73`, `17b8a3a0d`, `c684b789d` |

All three are `propagate`-direction candidates under the ose-primer sync classifier (generic governance patterns, not ose-public-specific content).

## Scope

Single-repo: `ose-primer` only. No parent gitlink bump required — changes are self-contained.

## Documents

- [Business Rationale](./brd.md) — why each change belongs in ose-primer
- [Product Requirements](./prd.md) — acceptance criteria per change, Gherkin scenarios
- [Technical Approach](./tech-docs.md) — exact files, commands, porting notes per change
- [Delivery Checklist](./delivery.md) — step-by-step execution checklist

## Key Constraints

- Module path in `ose-primer/apps/rhino-cli/go.mod` is `github.com/wahidyankf/ose-public/apps/rhino-cli` — **no import path changes needed** when porting Go code.
- `ose-primer` has no website app — date-metadata carve-out for `apps/` content does not apply; all files get stripped.
- Pre-push hook wires `validate:mermaid` Nx target only when `.md` files are in the push range.
- After any `.claude/agents/` changes, run `npm run sync:claude-to-opencode` before committing.
