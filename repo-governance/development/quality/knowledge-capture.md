---
title: "Knowledge Capture Convention"
description: Standards for the transient learnings.md running log accrued during plan execution, the open-ended principle-based triage matrix, the two mandatory safety gates, and the mandatory explicit "none" escape
category: explanation
subcategory: development
tags:
  - knowledge-capture
  - plans
  - learnings
  - triage
  - quality
created: 2026-07-06
---

# Knowledge Capture Convention

A plan is **not complete** until everything generalizable it taught the repo is routed to a durable
home -- or explicitly discarded with a reason. A plan that lands its delivery checklist but leaves its
learnings undecided in a transient file is only partially archived: the next plan will rediscover the
same workaround, the same wrong assumption, the same tool quirk, because nothing durable caught it.

## Principles Implemented/Respected

- **[Root Cause Orientation](../../principles/general/root-cause-orientation.md)**: A learning that
  is not routed to a durable home is a root cause left unaddressed -- the next plan hits the same
  wall because nothing in the repo remembers the first encounter. This convention converts one-off
  discoveries into standing repo memory.

- **[Documentation First](../../principles/content/documentation-first.md)**: The triage rubric
  treats documentation, agents, skills, and conventions as first-class destinations for captured
  knowledge -- not an afterthought bolted on after code ships.

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: The
  running log is a single markdown file with one entry shape. The triage rubric is one table plus
  one litmus question. No separate tracking system, ticket queue, or database is introduced.

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**:
  Every entry reaches an explicit terminal state -- routed inline, filed as backlog, or discarded
  with a one-line reason. "We'll remember" is never an acceptable implicit state.

- **[Deliberate Problem-Solving](../../principles/general/deliberate-problem-solving.md)**: Routing
  decisions are made once, deliberately, at a fixed checkpoint (the Knowledge Capture phase) rather
  than scattered ad hoc across a plan's lifetime.

## Conventions Implemented/Respected

- **[Plans Organization Convention](../../conventions/structure/plans.md)**: `learnings.md` is a
  plan-folder artifact alongside `README.md`, `brd.md`, `prd.md`, `tech-docs.md`, and `delivery.md`.
  This convention defines what `learnings.md` contains and how it is triaged; the Plans Organization
  Convention defines where it sits in the plan folder and its lifecycle through `backlog/` →
  `in-progress/` → `done/`.

- **[Feature Change Completeness Convention](./feature-change-completeness.md)**: Both conventions
  share the same code-routing discipline -- code lands with its companion specs, never bare. This
  convention extends that discipline to learnings: a code-homed learning becomes its own
  `plans/backlog/` plan, never an inline commit riding on an unrelated plan's scope.

- **[Regression Test Mandate](./regression-test-mandate.md)**: A learning that surfaces a bug in
  already-shipped code routes to a `plans/backlog/` fix plan that itself must satisfy the
  Regression Test Mandate -- a reproducing test in the same commit as the fix.

- **[Post-Mortems Convention](../../conventions/structure/post-mortems.md)**: When a learning
  describes a failure or incident severe enough to warrant a post-mortem, the triage rubric routes
  it there by cross-reference. This convention decides WHEN that routing applies; the Post-Mortems
  Convention remains the single source of truth for post-mortem structure and content.

- **[No Secrets in Committed Files Convention](../../conventions/security/no-secrets-in-committed-files.md)**:
  The secret/sensitivity safety gate (below) applies this convention's iron rule to every captured
  learning before it is routed anywhere.

## The Rule

**Every substantive plan MUST end its `delivery.md` with a Knowledge Capture phase that triages the
plan's `learnings.md` running log to durable homes before archival.** Archival is blocked until
every entry reaches a terminal state, or the plan carries the explicit
`No generalizable learnings -- <reason>` escape.

## The Transient `learnings.md` Running Log

Every substantive plan carries a `learnings.md` file, sibling to `delivery.md`, created empty (or
with a scaffold comment) at Environment Setup:

```
plans/in-progress/add-rate-limiting/
├── README.md
├── brd.md
├── prd.md
├── tech-docs.md
├── delivery.md
└── learnings.md          # transient running log -- triaged before archival
```

**Append during execution, not reconstructed from memory afterward.** The moment an executor
notices a workaround invented, a wrong assumption corrected, a tool or CLI quirk discovered, or any
insight that passes the litmus test below, it appends an entry immediately -- not at the end of the
plan when the details have faded.

**Entry shape**:

```markdown
## Learning: <one-line summary>

- **Context**: what was being done when this surfaced
- **Observation**: what was noticed (sanitized -- see the secret/sensitivity gate below)
- **Why it might generalize**: the litmus reasoning
```

## The Triage Rubric: Open-Ended, Principle-Based Routing

The rubric is deliberately **open-ended** -- it names candidate durable homes, not an exhaustive
enumeration. Route to whichever surface owns that kind of knowledge:

| Learning shape                                           | Candidate durable home                                                                                          |
| -------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| A rule the repo should always enforce                    | `repo-governance/conventions/` or `repo-governance/development/`                                                |
| A workflow procedure gap                                 | `repo-governance/workflows/`                                                                                    |
| An agent behavior gap or missing capability              | `.claude/agents/<name>.md`                                                                                      |
| A reusable technique or reference pattern                | `.claude/skills/<name>/SKILL.md` (or a reference module)                                                        |
| A user-facing or contributor-facing doc gap              | `docs/how-to/`, `docs/reference/`, or `docs/explanation/`                                                       |
| A code defect or missing test                            | `apps/`, `libs/`, or `specs/` -- via a `plans/backlog/` follow-up plan (see the code-routing rule below)        |
| A failure or incident worth a durable narrative          | `docs/explanation/post-mortems/` per the [Post-Mortems Convention](../../conventions/structure/post-mortems.md) |
| A future-work idea, richer than a one-liner but not yet plan-ready | `plans/ideas/` as a two-pager brief -- fold into an existing brief if one already covers the same area (see the [Ideas Folder convention](../../conventions/structure/plans.md#ideas-folder-two-pagers)) |
| Not generalizable -- true only for this plan's specifics | Discard with a one-line reason                                                                                  |

**Litmus test**: would a durable surface (a convention, an agent prompt, a skill, a test) **catch
this automatically next time** if it existed? If yes, the learning survives and is routed. If the
answer is "only if someone happens to remember," the learning does not generalize -- discard it
with a one-line reason rather than routing noise.

## The Code-Routing Downstream Rule

**A learning whose durable home is `apps/`, `libs/`, or `specs/` is NEVER landed inline in the
current plan's commits.** It is filed as a separate `plans/backlog/<slug>/` follow-up
plan instead. The current plan's scope was grilled and gated already; smuggling unrelated code
changes into it under the label "knowledge capture" reopens scope that was already closed.

**The sole carve-out**: a bug, lint failure, or test failure that blocks THIS plan's own delivery
checklist is fixed inline as ordinary Root Cause Orientation work -- that is not a deferred
learning, it is the current plan doing its job.

## Routing Timing: Destination-Aware (Inline vs. Backlog)

Not every non-code routing waits for a new plan. Timing depends on the destination and the size of
the change:

- **Small non-code routings** (a one-paragraph convention clarification, a short agent-prompt
  tweak, a missing cross-reference) land **inline**, in the current plan's own commits, as part of
  the Knowledge Capture phase itself.
- **Large non-code routings** (a new convention section, a new skill, a restructured workflow) and
  **all code routings** (per the rule above) become a `plans/backlog/<slug>/` follow-up
  plan -- never squeezed into the current plan's remaining scope.
- **`plans/ideas/` two-pager**: a future-work idea that is not yet plan-ready becomes a two-pager
  filed **inline** in the current plan's own commit/PR (creating one `plans/ideas/<slug>.md` is a
  small doc edit). Distinguish from `backlog/`: a learning that is **already plan-ready** goes straight
  to a `plans/backlog/<slug>/` follow-up plan; a promising-but-unripe idea that still needs its own
  pitch/triage goes to `plans/ideas/`. Fold into an existing two-pager rather than duplicating. Any
  eventual code work still flows through a full backlog plan when the two-pager is promoted, carrying
  the code-routing rule above in full.

## The Two Safety Gates (HARD -- run before routing)

Both gates run on **every surviving entry**, before it is routed anywhere.

### 1. Secret/Sensitivity Gate

Sanitize any credential, token, connection string, internal hostname, or personally identifying
detail out of the entry before it is written anywhere -- replace with a `<placeholder>` token. If
an entry cannot be sanitized without losing the meaning that made it worth capturing, discard it
rather than risk a leak. This gate applies the
[No Secrets in Committed Files Convention](../../conventions/security/no-secrets-in-committed-files.md)
to captured learnings specifically.

### 2. Repo-Relevance Gate

Verify the learning belongs in the repo it is about to land in. A learning about this repo's own
governance, conventions, agents, skills, or code belongs here. A learning about a sibling repo's
infrastructure, private configuration, or repo-specific constraints does NOT cross-route into this
repo -- and the inverse applies when working across sibling repos: content scoped to one repo's
private concerns never leaks into a public sibling.

## Mandatory + Explicit "None" Escape

If execution genuinely surfaced no generalizable learning, `learnings.md` records the explicit
escape instead of individual entries:

```markdown
No generalizable learnings -- <one-line reason>
```

This is a **mandatory** record, not merely permitted. Silent absence (an empty `learnings.md` with
no escape line and no entries) is not equivalent -- it leaves a checker unable to distinguish
"nothing to capture" from "capture was skipped."

## Exemptions

Pure-docs plans and trivial plans (a one-line rename, a single broken-link fix) MAY skip a
populated `learnings.md` -- the explicit "none" escape (or an equivalent note in `delivery.md`)
satisfies the requirement without inventing insight from a change that had none to offer. This
mirrors the exemption pattern in the
[Feature Change Completeness Convention](./feature-change-completeness.md).

## Anti-Theater Guardrails

The practice fails in two opposite directions; both are guarded against:

- **Under-capture theater**: declaring "no learnings" reflexively to skip the phase, when the
  execution transcript plainly surfaced a workaround or correction. `plan-execution-checker` and
  `plan-fixer` cross-check the delivery transcript against the "none" claim.
- **Over-capture theater**: routing every trivial, plan-specific detail as if it were a durable
  learning, flooding conventions and agent prompts with noise that fails the litmus test. The
  litmus test above exists specifically to filter this out -- "interesting to me right now" is not
  the bar; "would a durable surface catch this automatically" is.

## The Transient-Log Caveat

`learnings.md` is committed and moves with the plan folder through its lifecycle
(`backlog/` → `in-progress/` → `done/`), but it is **never the system of record**. Once every entry
reaches a terminal state, the durable homes it routed to -- not the log itself -- are what future
work depends on. `learnings.md` MAY be deleted from `plans/done/` at any later date without loss,
because everything worth keeping already moved out.

## What Gets Validated

- **[`plan-maker`](../../../.claude/agents/plan-maker.md)**: emits the `learnings.md` scaffold and
  the Knowledge Capture phase template in every substantive new plan.
- **[`plan-checker`](../../../.claude/agents/plan-checker.md)**: flags a substantive plan whose
  `delivery.md` lacks a Knowledge Capture phase (or an explicit "none" record) as a MEDIUM finding.
- **[`plan-execution-checker`](../../../.claude/agents/plan-execution-checker.md)**: cross-checks
  the "none" claim against the execution transcript (under-capture guardrail) and verifies no
  code-homed learning landed inline (code-routing guardrail).
- **[`plan-fixer`](../../../.claude/agents/plan-fixer.md)**: scaffolds a missing Knowledge Capture
  phase or `learnings.md` file when `plan-checker` flags its absence.

## Examples

### PASS: Small learning routed inline

While executing a rate-limiting plan, an executor discovers the repo's `nx affected` command
silently skips a project when its `project.json` lacks a `tags` array entry. They append the entry
to `learnings.md`, and during Knowledge Capture route it as a small inline addition -- a one-line
clarification in `repo-governance/development/infra/nx-targets.md` -- landed in the current plan's
own commits.

### PASS: Larger learning filed as backlog

The same plan's executor notices the repo has no convention describing when a new Nx project
needs a `release` tag. This is a new convention section, not a one-liner. Knowledge Capture routes
it as a `plans/backlog/document-nx-release-tagging/` follow-up plan rather than
expanding the current plan's scope.

### PASS: Discarded with reason

An executor notices the plan's specific test fixture used a three-item array. This detail is true
only for this plan's specific test data -- it fails the litmus test (no durable surface would need
to "catch" a specific fixture choice). Knowledge Capture discards it: "Fixture array size was an
arbitrary test-data choice, not a generalizable pattern."

### PASS: Explicit "none" escape

A plan that only renames a file records, per the Exemptions section:
`No generalizable learnings -- single-file rename with no design decisions.`

### FAIL: Silent absence

A plan's `learnings.md` is empty with no escape line, even though the plan's execution transcript
shows the executor discovered and worked around a flaky test. `plan-execution-checker` flags this
as under-capture theater -- the transcript contradicts the silent "nothing to see here."

### FAIL: Code landed inline instead of backlogged

An executor discovers a bug in an unrelated library function while executing a plan about a
different feature. Rather than filing a `plans/backlog/` fix plan, they patch the library function
directly in the current plan's commits and label it "knowledge capture." This violates the
code-routing rule -- the fix reopens scope the plan was never gated for, and the fix is not a
regression fix for something THIS plan's checklist depends on.

### FAIL: Secret leaked via an unsanitized entry

An entry in `learnings.md` reads "the staging DB connection string
`postgres://admin:hunter2@staging-db:5432/app` doesn't support SSL." The secret/sensitivity gate
was skipped. This is a leaked credential the moment the commit lands -- rotate immediately and
never write raw credentials into any captured entry.

## Related Documentation

- [Plans Organization Convention](../../conventions/structure/plans.md) -- Plan-folder structure,
  lifecycle, and where `learnings.md` sits alongside the other plan documents
- [Feature Change Completeness Convention](./feature-change-completeness.md) -- The companion
  two-path (direct-change / planned-change) discipline this convention's code-routing rule mirrors
- [Regression Test Mandate](./regression-test-mandate.md) -- Governs the reproducing test required
  when a routed learning becomes a bug-fix backlog plan
- [Post-Mortems Convention](../../conventions/structure/post-mortems.md) -- The durable home for a
  learning that describes a failure or incident severe enough to warrant one
- [No Secrets in Committed Files Convention](../../conventions/security/no-secrets-in-committed-files.md) --
  The iron rule the secret/sensitivity gate applies to every captured entry
- [`plan-maker`](../../../.claude/agents/plan-maker.md), [`plan-checker`](../../../.claude/agents/plan-checker.md),
  [`plan-execution-checker`](../../../.claude/agents/plan-execution-checker.md),
  [`plan-fixer`](../../../.claude/agents/plan-fixer.md) -- The four agents that emit, validate, and
  fix Knowledge Capture compliance across the plan lifecycle
