---
title: Gherkin Step-Keyword Cardinality — Cross-Repo Parity Decisions (2026-06-07)
description: >-
  Explanation of every decision in the 13-row deviation matrix for the
  gherkin-step-keyword-cardinality parity effort, including the four deliberate
  deviations and why each was chosen.
category: explanation
tags:
  - gherkin-step-keyword-cardinality
  - multi-repo
  - governance
  - parity
  - decision-log
---

# Gherkin Step-Keyword Cardinality — Cross-Repo Parity Decisions (2026-06-07)

This document records every decision made during the `gherkin-step-keyword-cardinality`
parity effort (2026-06-07) from the ose-primer perspective. The effort introduced a HARD
rule requiring each Gherkin `Scenario` to use exactly one primary `Given`, one `When`, and
one `Then` keyword (extras chain with `And`/`But`), enforced it across all three sibling
repositories via governance docs and a deterministic audit command, and retrofitted the
`specs/**/*.feature` corpus to conform.

The full deviation matrix lives in
[`plans/done/2026-06-07__gherkin-step-keyword-cardinality/tech-docs.md`](../../plans/done/2026-06-07__gherkin-step-keyword-cardinality/tech-docs.md).

Sibling plans in the other repositories also carry the rule and are executed in the same
parity set:

- `ose-public`: `plans/done/2026-06-07__gherkin-step-keyword-cardinality/`
- `ose-infra`: `plans/done/2026-06-07__gherkin-step-keyword-cardinality/`

## Background

Good Gherkin practice has always implied one action per scenario, but no explicit HARD rule
existed in this repo's governance. Authors and AI agents silently used repeated `When` or
`Then` keyword lines — a pattern that obscures intent, mixes multiple behaviors in one
scenario, and makes step bindings ambiguous. The `gherkin-step-keyword-cardinality` effort
makes the rule explicit and enforces it deterministically.

## Matrix Row Decisions (All 13 Rows)

### Row 1: Plan Handling

**Decision**: ose-public's existing plan updated in place; sibling repos received new plans.

**Why**: The ose-public plan was already authored and gated at Phase 0 before the parity run
began. Discarding a validated, gated plan to replace it with a fresh one wastes the work and
re-introduces authoring risk. The sibling plans (ose-primer, ose-infra) were new because no
prior plan existed for this rule in those repos.

**What was rejected**: Discarding the ose-public plan and authoring three identical plans
from scratch — rejected because the existing plan's Phase 0 baseline was already verified.

### Row 2: Linter Architecture (Deliberate Deviation)

**Decision**: ose-primer implements the audit as a standalone command
(`rhino-cli repo-governance gherkin-keyword-cardinality`) in the Rust CLI, driven by
a single Gherkin behavior contract in `specs/apps/rhino/`. ose-public and ose-infra add a
new category to their existing `audit_orchestrator.rs` pattern.

**Why**: ose-primer has no audit orchestrator. Its existing deterministic governance checks
are standalone `repo-governance` subcommands (currently only `vendor-audit`) in the single
`rhino-cli` implementation. Adding an orchestrator would be a bigger architectural
change than the new command warrants.

**What was rejected**: Adding an orchestrator layer to ose-primer — rejected because the
standalone-command pattern is already established.

**Impact on delivery**: Phase 4 is the largest phase in ose-primer's plan — it delivers the
behavior contract, the Rust implementation, and the Nx `validate:` targets all in one phase.
In ose-public, the equivalent work is a single-file addition to the orchestrator.

### Row 3: Retrofit Phases

**Decision**: All repos use linter-driven retrofit with graceful zero-offender handling.
Each retrofit phase runs the audit first; if a subtree reports zero offenders, the phase
makes no edits but still runs its gate.

**Why**: The linter is the authoritative check, not a pre-scan. The pre-scan at authoring
time found one offender in ose-primer (`specs/apps/crud/behavior/web/gherkin/layout/responsive.feature`),
but pre-scans can miss files added between authoring and execution. Running the linter at
execution time ensures completeness.

**What was rejected**: Trusting the authoring-time scan and skipping the audit at execution
time — rejected because the built linter is the authoritative check.

### Row 4: Governance Sweep

**Decision**: All repos use `repo-rules-maker`-driven governance sweep.

**Why**: All three repos carry the `repo-rules-maker` agent. The sweep is identical in
structure: edit the canonical convention, propagate references to all Gherkin-discussing
governance docs, update the three agent prompts (`plan-maker`, `plan-checker`,
`repo-rules-checker`).

**What was rejected**: Per-repo custom sweep scripts — rejected because the agent handles
this pattern generically.

### Row 5: Skill Propagation

**Decision**: All repos manually edit the two skill packages
(`.claude/skills/plan-writing-gherkin-criteria/SKILL.md` and
`.claude/skills/plan-creating-project-plans/SKILL.md`) and re-sync bindings via
`npm run generate:bindings`.

**Why**: All three repos carry both skills and the binding generator. The rule must appear
in the skills so agents that load those skills by name receive the cardinality constraint
in their context.

**What was rejected**: Delegating skill edits to `repo-rules-maker` — rejected per explicit
requirement (delivery.md Phase 3 note: "do NOT delegate to `repo-rules-maker`"). The
separation demonstrates the rule propagates through both channels.

### Row 6: Quality-Gate Preflight (Deliberate Deviation)

**Decision**: ose-public adds the new category to its existing Step 0.5 preflight.
ose-primer (and ose-infra) must first **port** the Step 0.5 deterministic-preflight section
into their `repo-rules-quality-gate.md`, then enumerate the new category.

**Why**: ose-primer's `repo-rules-quality-gate.md` had no Step 0.5 section at all — the
preflight pattern was introduced in ose-public after ose-primer was branched. Porting it
closes the parity gap rather than wiring around it. The adaptation is necessary: ose-public
invokes a single `repo-governance audit` orchestrator; ose-primer enumerates its standalone
commands (`repo-governance vendor-audit`, `repo-governance gherkin-keyword-cardinality`)
because it has no orchestrator (row 2 deviation).

**What was rejected**: Adding only the `gherkin-keyword-cardinality` reference to
ose-primer's quality gate without porting the Step 0.5 section — rejected because that
would leave a structural gap (no preflight framing, no exit-code semantics) and perpetuate
the parity gap with ose-public.

### Row 7: CI Wiring (Deliberate Deviation)

**Decision**: CI wiring differs per repo because CI topology differs.

- ose-public: existing governance-audit CI path.
- ose-infra: `validate-markdown.yml` on a self-hosted runner.
- ose-primer: GitHub-hosted `validate-markdown.yml` + the rhino-cli integration-test job in
  `pr-quality-gate.yml`.

**Why**: CI topology is a repo-specific concern that cannot be unified. ose-infra uses a
private self-hosted runner; ose-primer uses GitHub-hosted runners. The integration-test job
in ose-primer's `pr-quality-gate.yml` covers the new command on PRs automatically because the
behavior contract is extended in Phase 4.

**What was rejected**: Unifying CI topology across repos — rejected because self-hosted
and GitHub-hosted runners have incompatible configuration.

### Row 8: Push Mode (Deliberate Deviation)

**Decision**: ose-primer's plan pushes directly to `origin main` from the worktree branch
(`git push origin HEAD:main`), bypassing the PR-only default.

**Why**: The invoker explicitly selected `worktree-to-main` (the mode name at the time of this
2026-06-07 decision; the same direct-push-to-`origin-main` mode was later renamed
`worktree-to-origin-main` in the canonical four-mode Delivery Mode vocabulary — see
[Plans Organization Convention §Delivery Mode](../../repo-governance/conventions/structure/plans.md#delivery-mode))
for the entire parity set. ose-primer's own
[git-push-default convention](../../repo-governance/development/workflow/git-push-default.md)
also defaults to direct main push (no PR unless explicitly requested). This aligns with
ose-primer's Trunk Based Development workflow.

**What was rejected**: Creating a PR for ose-primer — rejected by invoker explicit
selection and by ose-primer's own default convention.

### Row 9: Linter Scan Scope

**Decision**: All repos scan `**/*.feature` minus a shared exclusion set: build outputs
(`bin/`, `build/`, `target/`, `dist/`, `node_modules/`), `worktrees/`, `archived/`,
and BDD-library self-test fixtures (`libs/elixir-cabbage/test/features/`,
`libs/elixir-gherkin/test/fixtures/`).

**Why**: Future feature files outside `specs/` would still be caught by a repo-wide scan.
The Elixir BDD library fixtures test the Gherkin parser itself and may deliberately use
unusual keyword patterns — exempting them prevents false positives.

**Net effect today**: In ose-primer the exclusion set reduces the scan scope to
`specs/**/*.feature` (58 files at authoring), but the broader pattern future-proofs
the command.

**What was rejected**: Scoping the scan to `specs/**` only — rejected because the
repo-wide pattern is more robust for future feature files placed outside `specs/`.

### Row 10: Rationale Doc Location

**Decision**: All repos create `docs/explanation/gherkin-step-keyword-cardinality-parity-decisions.md`.

**Why**: Matches the precedent set by the `plan-domain-parity` effort
([`docs/explanation/plan-domain-parity-decisions.md`](./plan-domain-parity-decisions.md)),
which used the same location. Consistency makes these decision logs discoverable.

**What was rejected**: Placing the rationale doc in the plan folder — rejected because plan
folders are temporary (archived to `done/`), while `docs/explanation/` contains permanent
documentation.

### Row 11: Research

**Decision**: Web research skipped in all repos.

**Why**: The change is purely internal — a governance rule, two CLI implementations, and
keyword-only spec edits. No external library versions, APIs, or standards are claimed.
All factual claims in the plan carry `[Repo-grounded]`, `[Judgment call]`, or
`[Unverified]` labels.

**What was rejected**: Running web research — rejected because there is nothing to research
externally.

### Row 12: Stage and Gate Mode

**Decision**: All repos use `in-progress` stage, `strict` gate mode, double-zero required.

**Why**: The parity set executes immediately after planning. Strict mode means the
`repo-rules-quality-gate` workflow must report zero deterministic findings and zero
confirmed AI-judgment findings on two consecutive runs before the plan is considered done.

**What was rejected**: Relaxed gate mode — rejected because the rule is a HARD rule and
its enforcement must be provably complete before archival.

### Row 13: Markdown-Gherkin Coverage

**Decision**: No deterministic markdown linter covers Gherkin fences in plan docs. Plan-doc
Gherkin (` ```gherkin ` fences in `prd.md`, etc.) is caught by `plan-checker` AI judgment
criteria and by `repo-rules-checker` judgment criteria during quality-gate sweeps. Active
plans (`plans/in-progress/`, `plans/backlog/`) are manually swept at execution time.
`plans/done/` is exempt as immutable archive.

**Why**: Deterministic parsing of markdown fences is out of scope for the linter (which
scans `.feature` files only). AI judgment via `plan-checker` and `repo-rules-checker` is
sufficient for plan-doc Gherkin because plan docs are authored and reviewed interactively.
Archived plans are immutable history — retrofitting them would be pure churn with no
quality benefit.

**What was rejected**: Extending the linter to parse markdown code fences — rejected by
invoker decision 2026-06-07 as out of scope.

## Summary of Deliberate Deviations

Four of the thirteen rows are deliberate deviations from the common cross-repo baseline:

| Row | Dimension              | ose-primer deviation                                                                   |
| --- | ---------------------- | -------------------------------------------------------------------------------------- |
| 2   | Linter architecture    | Standalone Rust CLI command + Gherkin behavior contract; no orchestrator               |
| 6   | Quality-gate preflight | Port Step 0.5 section first, then enumerate category (siblings had no Step 0.5)        |
| 7   | CI wiring              | GitHub-hosted runners; new step in `validate-markdown.yml`; parity job auto-covers PRs |
| 8   | Push mode              | Direct push to `origin main`; no PR (invoker-approved, matches repo default)           |

All four deviations are recorded in the plan's deviation matrix and are intentional — none
is a silent divergence.

## Related Documentation

- [HARD Rule — Step-Keyword Cardinality](../../repo-governance/development/infra/acceptance-criteria.md#hard-rule--step-keyword-cardinality) - The canonical rule text
- [BDD Spec-to-Test Mapping Convention](../../repo-governance/development/infra/bdd-spec-test-mapping.md) - How Gherkin connects to tests
- [Plan Domain Parity — Design Decisions](./plan-domain-parity-decisions.md) - Precedent for this document format
- [Plan](../../plans/done/2026-06-07__gherkin-step-keyword-cardinality/tech-docs.md) - Full deviation matrix and design decisions
