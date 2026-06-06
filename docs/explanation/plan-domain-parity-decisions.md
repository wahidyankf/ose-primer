---
title: Plan Domain Parity — Design Decisions (2026-06-06)
description: >-
  Explanation of every decision made in the 2026-06-06 plan-domain-parity
  effort from the ose-primer perspective: what was decided across all 26
  deviation-matrix rows, why each resolution was chosen, and what alternatives
  were rejected. Includes the recorded deviation from Safety Invariant 6
  (direct push to origin main, invoker-approved 2026-06-06).
category: explanation
tags:
  - plan-domain-parity
  - multi-repo
  - governance
  - harness-bindings
  - opencode
  - codex
  - decision-log
created: 2026-06-06
---

# Plan Domain Parity — Design Decisions (2026-06-06)

This document records every decision made during the `plan-domain-parity` effort
(2026-06-06, execution completed 2026-06-07). The effort aligned the
planning-system files across three sibling repositories — ose-public, ose-primer,
and ose-infra — covering fourteen governance markdown files, four AI agent
definitions, three AI skills, and the multi-harness binding surface. All 26
deviation-matrix rows were resolved in grilled sessions with the invoker on
2026-06-06 before any implementation began.

The full resolved matrix lives in
[`plans/in-progress/plan-domain-parity/tech-docs.md`](../../plans/in-progress/plan-domain-parity/tech-docs.md).

## Background

The three sibling repositories share a governance layer originally authored in
ose-public and propagated outward. Over time each repo accumulated independent
improvements — infra added mandatory grilling gates to a skill, primer added a
dual-CLI setup, ose-public renamed a convention file. The gaps compounded until a
structured comparison revealed 26 distinct deviations. This document explains the
thinking behind every resolution, from the primer's perspective.

## Primer-Specific Context

Primer carries two structures not present in ose-public:

- **Dual CLIs**: `apps/rhino-cli-rust` (canonical for binding generation) and
  `apps/rhino-cli-go` (Go implementation maintained for capability parity).
- **Naming divergence**: upstream uses `apps/rhino-cli`; primer uses
  `apps/rhino-cli-rust` and `apps/rhino-cli-go`.

These divergences affect rows 11, 20, and 21 and are explicitly preserved during
every 3-way merge described below.

## Survey Findings That Informed Decisions

The survey (empirical, 2026-06-06) established these facts before any decisions:

- `plan-quality-gate.md` is byte-identical in all three repos — no action needed.
- `plan-multi-repo-parity-planning.md` exists only in ose-public.
- Primer's `plan-establishment-execution.md` lacks the `target-stage` input field
  that ose-public and ose-infra carry (zero grep matches in the worktree).
- No grilling convention file exists in primer's
  `repo-governance/development/workflow/` (16 files verified; none match).
- `workflows/plan/` holds exactly 3 workflows + README in primer (the parity
  workflow is absent).
- The OpenCode emitter still emits the deprecated boolean `tools` flags format.
- `.codex/agents/` holds exactly one file
  (`ci-monitor-subagent.toml`), referenced from `[agents.ci-monitor-subagent]` in
  `.codex/config.toml` via `config_file = "agents/ci-monitor-subagent.toml"`.
- `.claude/agents/` holds 50 agent definitions; `.opencode/agents/` holds 50
  mirrors — 50:50 parity already present, no gap to reconcile.
- `planning-system-overhaul` has only archival items unchecked (all substantive
  delivery items ticked).

### Survey Correction (2026-06-06)

Pre-write verification against the `plan-domain-parity` worktree found one matrix
premise that needed refinement:

**Row 21 premise partially outdated.** `apps/rhino-cli-go` already ships
`agents sync` and `agents emit-bindings` commands with tests
(`cmd/agents_sync.go`, `cmd/agents_emit_bindings.go`, and 19 Go files under
`internal/agents/` including `bindings.go`, `converter.go`,
`sync_validator.go`). The remaining row-21 work is therefore **porting the
row-18/row-19 emitter changes** (permission object, Codex layout) to the Go
implementation — not building emission from scratch.

Research findings from web-research-maker (2026-06-05 to 2026-06-06) are cited
per decision where relevant.

## Decisions by Matrix Row

### Row 1 — Parity Workflow Propagation

**Decision**: propagate `plan-multi-repo-parity-planning.md` from ose-public to
primer and infra.

**Rationale**: the workflow must be invocable from any anchor repo. Keeping it
ose-public-only forces contributors in primer or infra to switch repos to invoke a
cross-repo parity sweep.

### Row 2 — Parity Workflow Grill Structure

**Decision**: amend all copies of `plan-multi-repo-parity-planning.md` to add a
two-grill + web-research step: Survey → Matrix → First Grill (hard gate) →
web-research-maker (conditional) → Second Grill (post-research) → Author → Gate →
Deliver.

**Rationale**: the invoker required this pattern (2026-06-06) to mirror the
structure already established in `plan-establishment-execution.md`. Decisions that
depend on external tool-convention research must not be locked in before the
research runs.

### Row 3 — plan-establishment-execution Merge; Worktree Default; target-stage

**Decision**: perform a 3-way best-of merge across all three repos. The merged
version keeps the `target-stage` input that primer's copy lacked. The merged
version also adds a **new default behavior**: plans are authored inside a dedicated
worktree (`worktrees/<identifier>/`), provisioned if absent via
`git worktree add -b <identifier> worktrees/<identifier> main` followed by
`npm install` and `npm run doctor -- --fix`. After delivery the worktree is removed
with `git worktree remove`.

**Rationale**: the invoker directed both the worktree default and the push
mechanics (HEAD pushed to the confirmed push target, defaulting to `origin main`).
The `target-stage` field is retained because ose-public and ose-infra already use
it; dropping it would be a regression.

### Row 4 — plan-execution.md Drift

**Decision**: 3-way best-of merge; each repo's agent-selection lists are preserved
verbatim because they reference repo-specific agents.

**Rationale**: the merge captures improvements from all repos while keeping
repo-specific content that would be wrong if overwritten.

### Row 5 — workflows/plan/README.md Index

**Decision**: align the index post-propagation so all three repos list the same
four workflows.

**Rationale**: follows from row 1 — the workflow now exists in all repos and must
appear in all three indexes.

### Row 6 — execution-modes.md Drift

**Decision**: 3-way best-of merge.

**Rationale**: the file had substantive divergence (40–102 changed lines) with no
repo-specific content that needed preservation.

### Row 7 — plan-maker Agent Drift

**Decision**: 3-way best-of merge; repo-specific cross-references (primer app and
CLI names) preserved.

**Rationale**: merge the improvements, keep the repo-specific links.

### Row 8 — plan-checker Agent Drift

**Decision**: 3-way best-of merge.

**Rationale**: no repo-specific content; straightforward merge.

### Row 9 — plan-fixer Agent Drift

**Decision**: 3-way best-of merge.

**Rationale**: same as row 8.

### Row 10 — plan-execution-checker Agent Drift

**Decision**: 3-way best-of merge.

**Rationale**: same as row 8.

### Row 11 — repo-setup-manager Primer Three-Line Drift

**Decision**: keep primer's three-line deviation if it reflects the primer-specific
`rhino-cli-rust` naming; merge the remainder.

**Rationale**: primer uses `apps/rhino-cli-rust` (not `apps/rhino-cli`). Those
three lines naming the Rust CLI are intentional. Overwriting them would break
primer's setup sequence.

### Row 12 — plan-creating-project-plans Skill Drift; Infra Grilling Gates

**Decision**: 3-way best-of merge **including infra's mandatory grilling gates**.
The infra improvement — requiring grilling to be documented and verified before
plan authoring proceeds — is adopted across all three repos.

**Rationale**: infra independently developed a stronger enforcement mechanism.
Importing an improvement from a sibling repo is consistent with the bidirectional
content-flow model.

### Row 13 — plan-writing-gherkin-criteria Skill Drift

**Decision**: 3-way merge (trivial — only 2–10 changed lines).

**Rationale**: the divergence was minor and non-structural.

### Row 14 — grill-me Skill Drift

**Decision**: 3-way best-of merge.

**Rationale**: 25–52 changed lines with no repo-specific content.

### Row 15 — Grilling Convention Naming

**Decision**: the merged content lands as `grilling-with-options.md` in all three
repos. Infra renames its existing `grilling.md` to `grilling-with-options.md` and
runs a full link sweep. Primer gains the file for the first time.

**Rationale**: ose-public already named the file `grilling-with-options.md` and
all ose-public workflows and `AGENTS.md` cite that exact name. Renaming toward
infra's shorter name would require a larger sweep across ose-public's link graph;
confining the sweep to infra is less disruptive.

**Rejected alternative**: use infra's name `grilling.md` everywhere — rejected
because it forces a larger sweep across ose-public.

### Row 16 — conventions/structure/plans.md Drift

**Decision**: 3-way best-of merge (107–125 changed lines).

**Rationale**: no repo-specific content; the merged version captures accumulated
improvements.

### Row 17 — Harness Binding Coverage Audit

**Decision**: perform a full repo-wide binding audit — all agents checked against
`.opencode/`, `.amazonq/`, and `.codex/`; `validate:harness-bindings` (or
equivalent) must pass with zero findings.

**Rationale**: the invoker chose maximal scope. A partial audit would leave gaps
that the triple-harness compatibility goal requires to be closed.

**Primer note**: the 50:50 `.claude`/`.opencode` parity was already verified
empirically against the worktree on 2026-06-06 — no gap to reconcile before
regeneration.

### Row 18 — OpenCode Emitter Format

**Decision**: modernize the rhino-cli OpenCode emitters in primer from the
deprecated boolean `tools` flags format to the `permission` object format
(`allow`/`ask`/`deny` per tool). In primer this means updating both
`apps/rhino-cli-rust/src/internal/agents/converter.rs` and
`apps/rhino-cli-go/internal/agents/converter.go`. After the code changes,
regenerate all 50 `.opencode/agents/*.md` mirrors.

**Rationale**: the OpenCode official documentation deprecates the boolean flags
form in favor of the `permission` object. Source:
<https://opencode.ai/docs/agents/> (accessed 2026-06-05).

**Implementation note**: tools not listed in the Claude frontmatter are **omitted**
from the `permission` block rather than emitted as `deny`. Emitting blanket `deny`
entries would require enumerating OpenCode's full tool universe, which is a moving
target. Omission is the minimal faithful translation — OpenCode's own defaults
apply for unlisted tools.

### Row 19 — .codex/agents/ Directory Removal

**Decision**: migrate per-agent Codex configuration from
`.codex/agents/<name>.toml` into `config.toml` `agents.<name>` sub-tables; stop
emitting or maintaining `.codex/agents/` as an official directory.

**Rationale**: the OpenAI Codex CLI official documentation documents only
`config_file` and `description` as sub-table keys. The `.codex/agents/` per-agent
directory pattern is not an officially recognized Codex convention.
Source: <https://developers.openai.com/codex/config-reference> (accessed
2026-06-06).

**Primer-specific**: neither primer CLI emits `.codex/agents/` today. The one
hand-maintained file (`ci-monitor-subagent.toml`) is migrated into the
`[agents.ci-monitor-subagent]` sub-table. The expected-binding-dirs lists in both
CLI implementations keep `.codex` (the directory itself remains; only the
`agents/` subdirectory is removed).

### Row 20 — generate:bindings Invocation Alignment

**Decision**: align primer to invoke the rhino-cli Rust binary directly via
`cargo run --manifest-path apps/rhino-cli-rust/Cargo.toml`. This replaces the
existing `nx run rhino-cli-rust:build && ./apps/rhino-cli-rust/dist/rhino-cli`
pattern in `package.json`.

**Rationale**: uniform invocation simplifies cross-repo maintenance. The accepted
trade-off is losing the Nx build-cache wrapper around the rhino-cli-rust
compilation step.

**Primer-specific**: the manifest path is `apps/rhino-cli-rust/Cargo.toml`
(upstream uses `apps/rhino-cli/Cargo.toml`). The alignment covers the full script
family: `generate:bindings`, `sync:agents`, `sync:skills`, `sync:dry-run`,
`validate:sync`, `validate:claude`, and `validate:harness-bindings`.

### Row 21 — Primer Dual-CLI Emitters

**Decision**: the Rust CLI (`apps/rhino-cli-rust`) remains canonical in the
`generate:bindings` script. The bindings emission capability is **already present**
in the Go CLI (survey correction: `agents sync` and `emit-bindings` already ship in
`apps/rhino-cli-go`). The remaining work is porting the row-18 and row-19 emitter
changes (permission object, Codex layout) to the Go implementation so capability
parity holds after the Rust emitter changes. The parity guard is validated via
`nx run rhino-cli-{rust,go}:validate:cross-vendor-parity` and is **not** wired into
the `generate:bindings` script.

**Rationale**: the invoker confirmed in the second grill session that the Go port
scope was appropriate and that the script should stay Rust-canonical. Separating
the parity guard from the generation script keeps scripts deterministic and avoids
circular validation.

### Row 22 — Primer Direct-Push Deviation (Safety Invariant 6)

**Decision**: accepted deviation. The primer plan pushes directly to
`origin main` from its worktree (`worktrees/plan-domain-parity/`, branch
`plan-domain-parity`) rather than following the PR-only sync default that applies
to upstream-to-downstream content propagation.

**Rationale**: the invoker explicitly approved this deviation (invoker grill,
2026-06-06). Plan files are low-risk content — they do not affect production
deployments. The deviation is recorded in three places: matrix row 22 in
`tech-docs.md`, the plan README's Deviation Notice, and this document.

**Safety Invariant 6 context**: Safety Invariant 6 of the
`plan-multi-repo-parity-planning` workflow requires that every mutation reaching
ose-primer flow through a worktree + branch + draft PR. Worktree-to-main execution
of a self-contained plan inside primer does not cross the upstream-to-downstream
boundary; the convention's intent (preventing accidental overwrites of downstream
customizations) is not violated by this execution. The deviation is still recorded
because the workflow text does not carve out this case explicitly. The approval
provenance is: invoker grill, 2026-06-06.

**Safety Invariant 6** is named verbatim in the `plan-multi-repo-parity-planning`
workflow source in ose-public
(`repo-governance/workflows/plan/plan-multi-repo-parity-planning.md`, ~line 161).

### Row 23 — Primer planning-system-overhaul Plan Superseded

**Decision**: the primer parity plan absorbs the remaining items from the
in-progress `planning-system-overhaul` plan. The overhaul plan is closed and
archived with a pointer to the parity plan.

**Rationale**: both plans addressed the same gap — primer's planning system lagging
ose-public's. Running two concurrent plans targeting the same files would produce
conflicts. A single source of truth (the parity plan) prevents duplicate effort and
conflicting resolutions.

**Primer-specific**: the overhaul plan's substantive delivery items were already
all ticked (verified empirically against `delivery.md` lines 216–232). What this
plan absorbs is the close-out: supersession pointer, archival `git mv` with
completion-date prefix, README index updates, and orphan-reference sweep.

**Rejected alternative**: run the overhaul plan to completion before the parity
plan — rejected because the overhaul was already superseded in scope by the more
comprehensive parity effort.

### Row 24 — Rationale Doc Location

**Decision**: the rationale document (`docs/explanation/plan-domain-parity-decisions.md`)
is placed in all three repos at the same relative path.

**Rationale**: uniform placement makes cross-repo navigation predictable. This
document is the primer instantiation of that decision.

### Row 25 — Slug, Stage, Gate

**Decision**: slug `plan-domain-parity`, stage `plans/in-progress/`, gate
`plan-quality-gate.md` strict double-zero.

**Rationale**: standard plan metadata. The strict double-zero gate requires zero
open checklist items and zero outstanding review comments before delivery is
accepted.

### Row 26 — Drift Guard Deliberately Dropped

**Decision**: no automated cross-repo drift checker is added. The upstream-first
editing discipline is left implicit — contributors edit in ose-public first and
propagate via the established sync agents.

**Rationale**: the invoker decided against adding tooling for this. The decision is
recorded explicitly so that future contributors understand the absence of a drift
guard is deliberate, not an oversight. Anyone reviewing this doc and finding
ose-public, primer, and infra drifting again should not assume a drift guard will
catch it — they should initiate a new parity effort.

**Rejected alternative**: add a `validate:cross-repo-drift` Nx target or CI step —
rejected on complexity grounds. The invoker judged the maintenance burden of such a
checker (keeping file lists current, handling intentional deviations) higher than
the benefit given the low frequency of parity sweeps.

## Research Citations

All web research performed by web-research-maker on 2026-06-05 to 2026-06-06:

- **OpenCode agents format** (boolean `tools` → `permission` object, rows 17 and
  18): <https://opencode.ai/docs/agents/> (accessed 2026-06-05)
- **OpenCode skills** (`.claude/skills/<name>/SKILL.md` read natively; no mirror
  needed): <https://opencode.ai/docs/skills/> (accessed 2026-06-05)
- **Amazon Q Developer CLI** (`.amazonq/rules/` + `.amazonq/cli-agents/*.json`;
  does not read AGENTS.md natively; generated bridge mechanism is correct):
  <https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-custom-agents.html>
  (accessed 2026-06-06)
- **OpenAI Codex CLI** (reads AGENTS.md natively via directory-walk; `.codex/agents/`
  not official; official path is `config.toml` `agents.<name>` sub-tables with
  `config_file` and `description` keys; rows 19 and 20):
  <https://developers.openai.com/codex/guides/agents-md> (accessed 2026-06-06),
  <https://developers.openai.com/codex/config-reference> (accessed 2026-06-06)
- **Multi-repo sync prior art** (no OSS tool performs 3-way semantic merge of
  hand-edited governance docs; manual semantic 3-way merge is the justified
  approach): surveyed repo-file-sync-action, cruft, copier, and symlink approaches
  (accessed 2026-06-06)

## Relation to Other Documents

- [Technical Documentation (tech-docs.md)](../../plans/in-progress/plan-domain-parity/tech-docs.md) —
  full embedded matrix, design decisions D1–D7, file impact table, testing
  strategy, and rollback plan
- [Plan README](../../plans/in-progress/plan-domain-parity/README.md) — delivery
  checklist and phase structure
- [Platform Bindings Reference](../reference/platform-bindings.md) — full catalog
  of binding directories affected by rows 17–20
