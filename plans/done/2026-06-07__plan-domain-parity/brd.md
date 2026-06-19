# Business Requirements Document — Plan Domain Parity (ose-primer)

## Business Goal

Restore and lock in **same-or-similar quality and behavior** of the planning system
(`repo-governance/workflows/plan/`, plan-family agents, plan-family skills, and the Plans
Organization Convention) across the three sibling repositories, by adopting the merged
3-way canon into ose-primer and modernizing primer's harness-binding emitters to current
vendor conventions.

## Business Impact

### Pain Points Today

1. **Quality drift between repos.** The 2026-06-06 survey (embedded in
   [tech-docs.md](./tech-docs.md#deviation-matrix-verbatim)) measured pairwise drift of
   30–243 changed lines in plan workflows, 41–170 in plan-family agents, and 2–243 in
   plan-family skills. A plan authored in primer is validated and executed by weaker rules
   than one authored in ose-public — improvements made in one repo silently never reach
   the siblings. [Repo-grounded — survey facts, matrix]
2. **Primer's plan-establishment workflow is functionally behind.** Primer's
   `plan-establishment-execution.md` lacks the `target-stage` input present in both
   siblings, so backlog-stage plan establishment is not expressible in primer
   [Repo-grounded — `grep target-stage` returns 0 matches in primer's copy].
3. **Primer cannot invoke the parity workflow.** `plan-multi-repo-parity-planning.md`
   exists only in ose-public, so a parity pass cannot be anchored from primer
   [Repo-grounded — primer `repo-governance/workflows/plan/` contains only 3 workflows + README].
4. **Primer lacks the grilling convention.** The multi-options grilling rule is cited by
   primer's workflows README but the convention file itself does not exist in primer's
   `repo-governance/development/workflow/` [Repo-grounded — 16 files, none grilling].
5. **Deprecated OpenCode agent format.** Primer's `.opencode/agents/*.md` mirrors use
   boolean `tools` flags, which OpenCode officially deprecated in favor of the
   `permission` object [Web-cited — <https://opencode.ai/docs/agents/>, accessed
   2026-06-05 via web-researcher; excerpt: "tools boolean flags deprecated — use
   permission object instead (allow/ask/deny per tool)"].
6. **Unofficial Codex layout.** `.codex/agents/` is not a Codex-recognized convention;
   the official mechanism is `config.toml` `agents.<name>` sub-tables [Web-cited —
   <https://developers.openai.com/codex/config-reference>, accessed 2026-06-06 via
   web-researcher; excerpt: "agents.\<name\> sub-table in config.toml; config_file
   key points to agent TOML"].
7. **Overlapping in-progress plan.** `plans/in-progress/planning-system-overhaul/`
   covers a subset of this objective; two open plans for one concern splits the source of
   truth [Repo-grounded — plan folder exists; only archival items remain unchecked].

### Expected Benefits

- Plans authored in any of the three repos meet the same quality bar and follow the same
  lifecycle behavior (worktree-default authoring, two-grill establishment, strict gates).
- Sibling improvements (e.g. infra's mandatory grilling gates in the
  plan-creating-project-plans skill) become available in primer instead of being lost.
- Harness bindings conform to current vendor conventions, reducing the risk that OpenCode
  or Codex stop honoring primer's agent mirrors. _Judgment call:_ vendor deprecations
  historically precede removals, so migrating now is cheaper than migrating after a
  breaking release.
- Dual-CLI capability parity (Rust + Go) keeps the template's flagship
  "same tool, two languages" guarantee intact for bindings emission, enforced by the
  existing parity guard rather than by convention alone.

## Affected Roles

Solo-maintainer repository — the maintainer wears all hats; no sign-off ceremonies.

- **Plan author hat**: gets the restored `target-stage` input, worktree-default
  authoring, and the parity workflow invocable from primer.
- **Template consumer hat** (downstream users scaffolding from ose-primer): receive a
  planning system identical in behavior to the upstream's, plus vendor-current bindings.
- **Consuming agents**: `plan-maker`, `plan-checker`, `plan-fixer`,
  `plan-execution-checker`, `repo-setup-manager` (merged definitions);
  `repo-harness-compatibility-checker` (audited binding surface); both rhino CLIs
  (modernized emitters).

## Business-Level Success Metrics

No fabricated numeric targets. Success is defined by observable checks:

1. **Observable**: every file in matrix rows 3–16 in primer matches the merged canon
   modulo documented repo-specific divergences (verified by the per-file merge steps and
   the plan-quality-gate strict pass).
2. **Observable**: `npm run generate:bindings` succeeds via direct `cargo run` and a
   subsequent re-run produces a clean `git diff` (deterministic emission).
3. **Observable**: `validate:harness-bindings`, `validate:sync`, and both CLIs'
   `validate:cross-vendor-parity` Nx targets exit 0 after regeneration.
4. **Observable**: exactly one in-progress primer plan owns the planning-system concern
   (this one); `planning-system-overhaul` is archived with a supersession pointer.
5. **Qualitative reasoning**: future drift is managed by upstream-first editing
   discipline (row 26 deliberately drops an automated guard) — acceptable because the
   parity workflow itself is now invocable from any repo to re-converge when needed.

## Business-Scope Non-Goals

- **No automated cross-repo drift checker** (matrix row 26 — deliberate, recorded drop).
- **No change to the PR-only primer sync default itself** — this plan records a one-time,
  invoker-approved deviation (row 22); it does not rewrite the sync convention's default.
- **No new planning features** beyond the merged best-of canon and the two
  invoker-directed amendments (rows 2 and 3).
- **No Go CLI wiring into `generate:bindings`** — Rust remains the canonical script
  binary (row 21).

## Business Risks and Mitigations

| Risk                                                                                  | Mitigation                                                                                                                                            |
| ------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| Direct push to primer `main` bypasses PR review (Safety Invariant 6 deviation)        | Invoker-approved; plan files + governance docs are low-risk; full local gates + post-push CI verification; deviation recorded in the rationale doc    |
| Semantic merge drops a repo-specific primer divergence (e.g. `rhino-cli-rust` naming) | Merge steps explicitly enumerate known divergences to preserve; plan-quality-gate strict + post-merge grep checks                                     |
| Emitter format change breaks OpenCode agent loading                                   | Format follows official OpenCode docs (Web-cited); regeneration validated by `validate:sync`; manual smoke check of a regenerated mirror              |
| Codex migration breaks the one existing Codex agent (`ci-monitor-subagent`)           | Migration keeps the `agents.<name>` sub-table (already official-format) and relocates content per official reference; config verified after migration |
| ose-public canon not yet landed when this plan executes                               | Phase 1 opens with a hard sequencing gate that verifies the upstream merge has landed before any adoption step runs                                   |
| Go port diverges from Rust behavior                                                   | Existing `validate:cross-vendor-parity` guard + TDD-first tests on the Go side mirror the Rust tests                                                  |
