# Planning System Overhaul

**Status**: Completed (2026-05-26)
**Created**: 2026-05-26
**Priority**: HIGH
**Scope**: Fix three remaining planning-system gaps in ose-primer, adopted from
`ose-public` [`2026-05-26__planning-system-overhaul`](https://github.com/wahidyankf/ose-public/tree/main/plans/done/2026-05-26__planning-system-overhaul)

## Context

Seven gaps in the planning system were identified in ose-public. Four of them were already
resolved in ose-primer (worktree auto-provisioning in `plan-execution.md`, mandatory grill +
Phase 0 in `plan-maker.md`, `plan-establishment-execution.md` workflow, and
`repo-setup-manager` agent). This plan adopts the three remaining gaps:

1. **RED/GREEN/REFACTOR not always separate checklist items** — the TDD convention shows a
   nested "TDD cycle" pattern but does not prohibit collapsing all three phases into one
   checkbox. A HARD RULE paragraph must make this explicit.

2. **AGENTS.md not updated** — `repo-setup-manager` is missing from the agent catalog,
   `plan-establishment-execution.md` workflow is not referenced, and `plan-maker`'s grill
   mandate is not documented.

3. **Markdown link checker flags stale links in archived content** — `plans/done/` and
   `archived/` contain frozen historical files whose internal links may be stale. Both
   markdown lint config files and the markdown quality doc need archive exclusion entries.

## Already Done in ose-primer (Skip)

The following ose-public gaps were already resolved before this plan and are NOT re-executed:

| Gap                                    | Resolution                                                                         |
| -------------------------------------- | ---------------------------------------------------------------------------------- |
| Worktree gate auto-provisioning        | `plan-execution.md` already auto-provisions (confirmed: no "non-recoverable" text) |
| `plan-maker` mandatory grill + Phase 0 | `plan-maker.md` already has Steps 1 and 8 + Phase 0 template                       |
| `plan-establishment` workflow          | `plan-establishment-execution.md` exists at `repo-governance/workflows/plan/`      |
| `repo-setup-manager` agent             | `.claude/agents/repo-setup-manager.md` exists                                      |

## In-Scope

- `repo-governance/development/workflow/test-driven-development.md` — HARD RULE + grouping-label note
- `AGENTS.md` — add `repo-setup-manager`, link `plan-establishment-execution.md`, document grill mandate
- `.markdownlintignore` — add `plans/done/` and `archived/`
- `.markdownlint-cli2.jsonc` — add `"plans/done/**"` and `"archived/**"` to ignores
- `repo-governance/development/quality/markdown.md` — archive exclusion section

## Out-of-Scope

- `plan-checker` / `plan-fixer` enforcement of the HARD RULE (future plan)
- Any changes to `plan-execution.md`, `plan-maker.md`, `plan-establishment-execution.md`,
  or `repo-setup-manager.md` (already correctly implemented)

## Approach

All changes are governance documentation plus two config file edits. No code changes,
no new agent files. `npm run generate:bindings` is only needed if any agent `.md` files
change (not expected in this plan since AGENTS.md is the canonical governance doc, not an
agent definition file).

## Documents

| Document                     | Purpose                                      |
| ---------------------------- | -------------------------------------------- |
| [brd.md](brd.md)             | Business rationale and goals                 |
| [prd.md](prd.md)             | Requirements and Gherkin acceptance criteria |
| [tech-docs.md](tech-docs.md) | Technical design with exact change content   |
| [delivery.md](delivery.md)   | TDD-shaped delivery checklist                |

## Worktree

**Path**: `worktrees/planning-system-overhaul/`

Provision before execution (run from repo root):

```bash
claude --worktree planning-system-overhaul
```

Per [plans.md §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification),
the canonical Worktree section lives in [delivery.md](delivery.md).
