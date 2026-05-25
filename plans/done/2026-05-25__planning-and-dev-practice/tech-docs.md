# Technical Design — Planning and Dev Practice Improvement

## Architecture Overview

This plan produces one new skill file and a set of governance document updates:

| Artifact          | Action                        | Path                                                               |
| ----------------- | ----------------------------- | ------------------------------------------------------------------ |
| Grill-me skill    | CREATE                        | `.claude/skills/grill-me/SKILL.md`                                 |
| Plan checker      | UPDATE                        | `.claude/agents/plan-checker.md` (add Step 5g)                     |
| Plan quality gate | UPDATE                        | `repo-governance/workflows/plan/plan-quality-gate.md`              |
| Planning workflow | UPDATE                        | `repo-governance/workflows/plan/plan-execution.md`                 |
| TDD convention    | UPDATE                        | `repo-governance/development/workflow/test-driven-development.md`  |
| Agent mirror      | REGENERATE                    | `.opencode/agents/plan-checker.md` via `npm run generate:bindings` |
| Governance docs   | UPDATE via `repo-rules-maker` | Multiple — see Phase 3                                             |

No application code changes. No new dependencies. No migrations. The only non-markdown effect is the
mechanical regeneration of the `plan-checker` OpenCode mirror, which is byte-derived from the edited
`.claude/agents/plan-checker.md` [Repo-grounded:
`repo-governance/conventions/structure/multi-harness-binding.md` → "AD4 — Mechanical Generation Over
Hand-Maintenance"].

## Skill File: `.claude/skills/grill-me/SKILL.md`

The skill is a single markdown file with YAML frontmatter. Claude Code discovers skills via the
`.claude/skills/{name}/SKILL.md` path convention, and OpenCode reads the same file natively
[Repo-grounded: `AGENTS.md` → "OpenCode reads `.claude/skills/{name}/SKILL.md` natively"].

### Final content

```markdown
---
name: grill-me
description: >
  Interview the user relentlessly about a plan or design, presenting choices one at a
  time until shared understanding is reached. Resolves every branch of the decision
  tree. Use when the user wants to stress-test a plan, get grilled on their design,
  or mentions "grill me".
---

# Grill Me

Stress-test plans and designs through relentless, structured questioning before
implementation begins.

## When to activate

Activate when:

- User says "grill me", "challenge my plan", "stress-test this", "interrogate my design",
  or any close variant
- A new plan is being created and design decisions remain open
- A design review is requested before committing to implementation

## Process

Interview the user about every aspect of the plan until shared understanding is reached.
Walk down each branch of the decision tree, resolving dependencies one-by-one.

**Rules:**

1. Ask questions **one at a time** — never bundle multiple questions in one message
2. Present **2-4 concrete options** with trade-off descriptions per question
3. **Mark the recommended option** clearly, e.g. `**(Recommended)**`
4. **Explore the codebase first** — if a question can be answered by reading existing
   files, read them instead of asking
5. Continue until all branches are resolved

## Question format

Structure each question like this:

> **[Question]**
>
> - **Option A**: [description] — [trade-off]
> - **Option B**: [description] — [trade-off] **(Recommended)**
> - **Option C**: [description] — [trade-off]
>
> **Recommendation**: Option B because [specific reason grounded in this context].

## After the grilling

When all decision tree branches are resolved:

1. Summarize every decision made and its rationale
2. Confirm shared understanding explicitly
3. Signal readiness to proceed to plan writing or implementation
```

## Related Files to Update

The following files reference planning skills, conventions, or TDD practices and need
updating when grill-me is adopted [Repo-grounded: each path verified via `ls`]:

### `.claude/agents/plan-checker.md` (Step 5g logic)

Add a new validation step — **Step 5g — Harness-Neutrality Scan** — as a new numbered section
after the existing "Step 5f — Anti-Hallucination Scan" (currently section 12 in
`plan-checker.md`). The step is CRITICAL and fires only when a plan touches agents, skills, rules,
or governance docs. It verifies:

1. Agent definitions follow multi-harness-binding conventions
   [Repo-grounded: `repo-governance/conventions/structure/multi-harness-binding.md`]
2. Agent mirrors are regenerated via `npm run generate:bindings`, not hand-written
   [Repo-grounded: `AGENTS.md` → "`generate:bindings`"]
3. Skill body is plain markdown — no harness-specific syntax
4. No OpenCode skill mirror is manually created (OpenCode reads `.claude/skills/` natively)
5. Governance doc changes live outside any "Platform Binding Examples" heading unless
   intentionally vendor-specific [Repo-grounded:
   `repo-governance/conventions/structure/governance-vendor-independence.md`]

The check is **conditional**: if a plan does not touch agents, skills, or `repo-governance/`,
the step is skipped and no findings are generated.

### `repo-governance/workflows/plan/plan-quality-gate.md` (Step 5g reference)

Add a `Step 5g` entry to the "Validation scope (per `plan-checker` Steps 0-7 + 5b/5c/5d/5e/5f)"
list and a matching bullet to the "Plan-Specific Validation" section, describing the conditional
harness-neutrality scan. This keeps the workflow doc's documented scope in sync with the
`plan-checker` step it delegates to.

### `repo-governance/workflows/plan/plan-execution.md`

Add a reference to grill-me in the `**When to use**:` description. When a plan is being
created or refined, grill-me should be mentioned as the skill to invoke for
stress-testing design decisions before execution.

### `repo-governance/development/workflow/test-driven-development.md`

The document already mandates TDD and already includes an "Applying TDD to Plans" section with
RED-GREEN-REFACTOR delivery-checklist examples [Repo-grounded:
`repo-governance/development/workflow/test-driven-development.md` → "Applying TDD to Plans"].
Strengthen that section by adding a `### TDD Shape for Delivery Checklists` subsection containing
the explicit command + acceptance-criterion three-substep template from §TDD Shape for Delivery
Checklists below.

### Repo-Rules Propagation via `repo-rules-maker`

Run `repo-rules-maker` to propagate the new grill-me convention across any governance
layer documents that reference planning skills or planning conventions. The agent will
identify any gaps in coverage and create or update rules accordingly.

## Design Decisions

### Why choice-based questions?

Open-ended questions allow vague, non-committal answers. Options with named trade-offs force
explicit engagement with each decision. This aligns with the superpowers `brainstorming` skill's
multiple-choice principle
[Web-cited: https://github.com/obra/superpowers/blob/main/skills/brainstorming/SKILL.md,
accessed: 2026-05-25, excerpt: "Prefer multiple choice questions when possible, but open-ended is
fine too"] and with Claude Code's `AskUserQuestion` pattern which requires concrete options with
descriptions.

### Why "one question at a time"?

From mattpocock's original grill-me skill
[Web-cited: https://github.com/mattpocock/skills/blob/main/skills/productivity/grill-me/SKILL.md,
accessed: 2026-05-25, excerpt: "Ask the questions one at a time."]:
"Ask the questions one at a time." Batching questions allows the user to answer shallowly.
Sequential questions force engagement with each decision branch before moving to the next.

### Why no OpenCode mirror?

`AGENTS.md` documents that OpenCode reads `.claude/skills/{name}/SKILL.md` natively
[Repo-grounded]. No mirror needed. Manually creating an `.opencode/skill/` mirror would be flagged
by the `validate:sync` "No Synced Skill Mirror" check [Repo-grounded: `CLAUDE.md`].

### Why the harness-neutrality check in plan-checker (Step 5g)?

Plans that add or modify agents, skills, or governance docs have direct impact on
multi-harness compatibility. The existing quality gate checks structure, TDD shape, and
anti-hallucination — but has no harness-specific validation. Adding Step 5g to `plan-checker`
(where the other 5x steps live) closes this gap at the plan level rather than relying solely on the
post-execution `repo-harness-compatibility-quality-gate`
[Repo-grounded: `repo-governance/workflows/repo/repo-harness-compatibility-quality-gate.md`]. Early
detection is cheaper than post-execution remediation. The two gates are complementary, not
redundant: Step 5g reviews plan documents before execution; the compatibility gate audits the
committed binding surfaces after execution.

### Why edit both `plan-checker.md` and `plan-quality-gate.md`?

In this repository the actual validation logic for Steps 5b–5f lives in `.claude/agents/plan-checker.md`,
and `plan-quality-gate.md` documents the scope it delegates to [Repo-grounded:
`repo-governance/workflows/plan/plan-quality-gate.md` → "Validation scope (per `plan-checker`
Steps 0-7 + 5b/5c/5d/5e/5f)"]. To make Step 5g an executable check rather than a dangling reference,
the step logic lands in `plan-checker.md` and the scope reference lands in `plan-quality-gate.md`.

### Why `repo-rules-maker` + `repo-rules-quality-gate`?

Adding a new planning convention without updating governance docs creates inconsistency
across the six-layer hierarchy [Repo-grounded: `repo-governance/repository-governance-architecture.md`].
`repo-rules-maker` identifies affected docs and creates/updates rules. `repo-rules-quality-gate`
verifies coherence.

## Rollback

This plan makes no application code changes. All deliverables are governance docs, an agent
definition edit (plus its regenerated mirror), and a skill file. Rollback is a series of
`git revert` commits or manual deletions:

1. **Remove grill-me skill**: `rm -rf .claude/skills/grill-me/` and commit
   `chore(skills): remove grill-me skill (rollback)`.
2. **Revert `plan-checker.md`**: Remove the Step 5g (Harness-Neutrality Scan) section that was added
   by this plan, then run `npm run generate:bindings` to regenerate the OpenCode mirror.
3. **Revert `plan-quality-gate.md`**: Remove the Step 5g scope reference added by this plan. Verify
   `npm run lint:md` passes after revert.
4. **Revert `plan-execution.md`**: Remove the grill-me reference added in Step 2.2.
5. **Revert `test-driven-development.md`**: Remove the `### TDD Shape for Delivery Checklists`
   subsection added by Step 2.3.
6. **Revert repo-rules propagation**: If `repo-rules-maker` updated governance docs, revert those
   commits individually using `git revert <commit-sha>` — one commit per domain per the thematic
   commit guidance.
7. **Verify coherence**: After all reverts, run `npm run lint:md` and `repo-rules-quality-gate` to
   confirm the repo is in a consistent state.

No database migrations, no runtime config changes, and no dependency updates are involved — all
changes are markdown edits, one directory removal, and one mechanical mirror regeneration.

## TDD Shape for Delivery Checklists

All code delivery steps in plan checklists must follow this three-substep pattern,
adapted from `repo-governance/development/workflow/test-driven-development.md` [Repo-grounded]:

```text
- [ ] **RED**: Write failing test for [specific behavior]
      — command: `nx run [project]:test:unit`
      — acceptance: test fails with `[expected error message]`
- [ ] **GREEN**: Implement `[function/component]` in `[file path]`
      — command: `nx run [project]:test:unit`
      — acceptance: test passes, no other tests broken
- [ ] **REFACTOR**: Clean up [specific concern] in `[file path]`
      — command: `nx run [project]:test:unit`
      — acceptance: all tests still pass, code is cleaner
```

Non-code steps (doc edits, config changes, file creation) do not require RED-GREEN-REFACTOR.
They use direct action + acceptance criterion instead.
