# Planning and Dev Practice Improvement

**Status**: In Progress
**Created**: 2026-05-25
**Priority**: MEDIUM
**Scope**: Improve planning quality and development discipline across skill, governance, and
workflow layers

## Summary

Three improvements to planning and development practice, adapted from the `ose-public`
[`2026-05-25__planning-and-dev-practice`](https://github.com/wahidyankf/ose-public/tree/main/plans/done/2026-05-25__planning-and-dev-practice)
plan and adjusted to this repository's multi-harness, rhino-cli-based binding model:

1. **Grill-Me Skill** — a structured interrogation skill that stress-tests plans by asking one
   focused question at a time, presenting choices like Claude Code's `AskUserQuestion`, and walking
   down the decision tree until shared understanding is reached. Adapted from
   [mattpocock/skills grill-me](https://github.com/mattpocock/skills/blob/main/skills/productivity/grill-me/SKILL.md)
   with inspiration from [obra/superpowers](https://github.com/obra/superpowers) brainstorming and
   writing-plans skills.

2. **TDD Mandate** — formalize RED-GREEN-REFACTOR as the required shape for all code delivery steps
   in plan checklists, strengthening the existing
   [`test-driven-development.md`](../../../repo-governance/development/workflow/test-driven-development.md)
   convention with an explicit command/acceptance delivery-checklist template.

3. **Harness-Neutral Plan Quality Gate** — add a conditional harness-neutrality scan (Step 5g) to
   the plan quality gate. The check logic lands in `plan-checker` (where Steps 5b–5f live) and is
   referenced from `repo-governance/workflows/plan/plan-quality-gate.md`, ensuring plans that touch
   agents, skills, rules, or governance docs introduce no vendor lock-in.

## Documents

| Document                     | Purpose                                      |
| ---------------------------- | -------------------------------------------- |
| [brd.md](brd.md)             | Business rationale and goals                 |
| [prd.md](prd.md)             | Requirements and Gherkin acceptance criteria |
| [tech-docs.md](tech-docs.md) | Technical design and skill file content      |
| [delivery.md](delivery.md)   | TDD-shaped delivery checklist                |

## Worktree

**Path**: `worktrees/planning-and-dev-practice/` — see [delivery.md §Worktree](delivery.md#worktree)
for the provisioning command and convention references.
