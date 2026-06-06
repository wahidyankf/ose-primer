# Plan Domain Parity (ose-primer)

**Status**: In Progress (planning complete; execution not started)
**Created**: 2026-06-06
**Slug**: `plan-domain-parity`
**Mode**: worktree-to-main (invoker-approved deviation from the PR-only primer sync default — see [Deviation Notice](#deviation-notice) below)
**Quality gate**: plan-quality-gate, strict (double-zero)

## Context

The three sibling repositories — `ose-public` (upstream), `ose-primer` (downstream template),
and `ose-infra` (private infrastructure) — each carry a copy of the planning system:
`repo-governance/workflows/plan/`, the plan-family agents (`plan-maker`, `plan-checker`,
`plan-fixer`, `plan-execution-checker`, `repo-setup-manager`), the plan-family skills
(`plan-creating-project-plans`, `plan-writing-gherkin-criteria`, `grill-me`), and the
[Plans Organization Convention](../../../repo-governance/conventions/structure/plans.md).
These copies have drifted: a 2026-06-06 survey measured pairwise drift of 2–243 changed
lines per file, and ose-primer's `plan-establishment-execution.md` lacks the
`target-stage` input that both siblings have [Repo-grounded].

This plan covers the **ose-primer side** of a three-repo parity set produced by the
plan-multi-repo-parity-planning workflow. The invoker objective, verbatim:

> "same and similar quality and behavior of repo-governance/workflows/plan/ and its
> related agents, and skills in ose-infra, ose-primer, and ose-public"

All macro decisions were grilled and resolved with the invoker on 2026-06-06 and recorded
in a 26-row deviation matrix, embedded verbatim in [tech-docs.md](./tech-docs.md). This
plan was authored non-interactively against that matrix; the plan-maker pre-write and
post-write grills were performed as validation passes against the resolved decisions.

## Scope

### In Scope (ose-primer repository only)

1. **Adopt the merged 3-way canon** (matrix rows 3–16) into primer's copies of the plan
   workflows, plan-family agents, plan-family skills, and the Plans Organization
   Convention — including restoring the missing `target-stage` input and the new
   worktree-default authoring behavior in `plan-establishment-execution.md`.
2. **Two new governance files for primer**: the amended
   `plan-multi-repo-parity-planning.md` workflow (row 2 structure) and the merged
   `grilling-with-options.md` convention (row 15).
3. **rhino-cli emitter modernization** (rows 18–21): OpenCode `permission` object
   emission, Codex `config.toml` sub-table migration (stop using `.codex/agents/`),
   `generate:bindings` direct-cargo invocation, and porting the updated bindings-emission
   behavior to `apps/rhino-cli-go` for dual-CLI capability parity.
4. **Full repo-wide binding audit** (row 17): all agents × `.opencode`/`.amazonq`/`.codex`,
   with `validate:harness-bindings`, `validate:sync`, and both
   `validate:cross-vendor-parity` targets passing after regeneration.
5. **Supersede + absorb** `plans/in-progress/planning-system-overhaul/` (row 23).
6. **Rationale doc** `docs/explanation/plan-domain-parity-decisions.md` (rows 22, 24).
7. **Governance doc updates** touched by the above: `AGENTS.md` catalog/binding wording,
   workflow indexes, multi-harness binding docs affected by rows 18–20.

### Out of Scope

- Changes to `ose-public` or `ose-infra` files (each sibling has its own plan).
- An automated cross-repo drift guard (row 26: deliberately dropped).
- Wiring `rhino-cli-go` into the `generate:bindings` script (row 21: Rust stays canonical).
- Any planning-system feature not present in one of the three repos' current copies
  (3-way _best-of_ merge, not new invention — except the row-2/row-3 amendments the
  invoker directed).

## Sibling Plans

This plan is one of three coordinated plans — one per repository. Recommended execution
order: **ose-public first** (the merged canon lands upstream), then this plan, then
ose-infra. Each plan is self-contained with its own merge steps referencing sibling clone
paths.

| Repo                  | Plan location (within that repo)                 | Local clone path                     |
| --------------------- | ------------------------------------------------ | ------------------------------------ |
| ose-public (upstream) | `plans/in-progress/plan-domain-parity/README.md` | `/Users/wkf/ose-projects/ose-public` |
| **ose-primer (this)** | `plans/in-progress/plan-domain-parity/README.md` | `/Users/wkf/ose-projects/ose-primer` |
| ose-infra             | `plans/in-progress/plan-domain-parity/README.md` | `/Users/wkf/ose-projects/ose-infra`  |

## Deviation Notice

This plan reaches ose-primer via **direct push to `origin main` from the worktree
`worktrees/plan-domain-parity/`** — an invoker-approved, recorded deviation from the
PR-only default for mutations reaching ose-primer (Safety Invariant 6 of the
plan-multi-repo-parity-planning workflow; matrix row 22). The deviation and its
justification are documented in full in the rationale doc this plan creates
(`docs/explanation/plan-domain-parity-decisions.md`) and in
[tech-docs.md §Design Decisions](./tech-docs.md#design-decisions).

## Approach Summary

ose-public lands the merged canon first; this plan adopts it by **manual semantic 3-way
merge per file** (research confirmed no OSS tool does semantic merges of hand-edited
governance docs — see tech-docs research findings), preserving primer-specific content
(`rhino-cli-rust` naming, primer agent-selection lists, primer-only divergences in
`repo-setup-manager`). Code work in both rhino CLIs follows strict Red→Green→Refactor
TDD, gated by the existing dual-CLI parity guard.

## Plan Documents

- [brd.md](./brd.md) — WHY: business goal, pain points, success metrics, non-goals, risks
- [prd.md](./prd.md) — WHAT: user stories, Gherkin acceptance criteria, product scope
- [tech-docs.md](./tech-docs.md) — HOW: full deviation matrix (verbatim), research
  findings with citations, survey corrections, file impact, design decisions
- [delivery.md](./delivery.md) — DO: phased checklist with executor legend, Phase 0,
  TDD-shaped code steps, per-phase gates, and the mandatory `## Worktree` section

## Related Documentation

- [Plans Organization Convention](../../../repo-governance/conventions/structure/plans.md)
- [Plan Workflows Index](../../../repo-governance/workflows/plan/README.md)
- [Multi-Harness Binding Convention](../../../repo-governance/conventions/structure/multi-harness-binding.md)
- [rhino-cli Dual-Implementation Parity Convention](../../../repo-governance/conventions/structure/rhino-cli-dual-implementation-parity.md)
- [Platform Bindings Reference](../../../docs/reference/platform-bindings.md)
