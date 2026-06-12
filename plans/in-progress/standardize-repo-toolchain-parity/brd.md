# BRD — Standardize Repo Toolchain Parity (ose-primer)

This Business Requirements Document explains **why** the toolchain standardization exists. The
**what** (features, scope, acceptance criteria) lives in [prd.md](./prd.md); the **how** lives in
[tech-docs.md](./tech-docs.md).

## Business Goal

Make the **repository toolchain** of `ose-primer` and its two sibling repos (`ose-public`,
`ose-infra`) converge to a single **fixed Converged Toolchain Target** — CI workflows, git hooks,
the `rhino-cli` CLI (architecture + command surface + Nx target names), and the governing docs all
functionally identical except for recorded per-repo deviations — so that a contributor (or AI agent)
who understands one repo's toolchain understands all three. As the public polyglot template,
`ose-primer` is the showcase scaffolding teams copy when building their own Sharia-compliant
products, so its toolchain must read identically to the upstream it derives from.

For CI/hooks/target-naming/docs (workstreams A/B/E/F) there is **no single anchor repo**: each repo
closes only its own gaps and the plans are **parallel-safe**. For the rhino-cli architecture and
command surface (workstreams C/D) the convergence is **reference-first**: `ose-public` is the
reference that authors first, and `ose-primer` ports from it.

## Business Rationale

Toolchain drift between sibling repos is a slow, compounding tax across six surfaces:

- **CI cognitive load (A)** — every divergence (a concurrency block here, none there; a `specs-gate`
  job here, none there; a gate on PR here but also on push-to-main there; ad-hoc workflow file names +
  `name:` fields + job ids) is a thing a maintainer must hold in their head per repo. This repo is
  solo-maintained, so the load lands on one person. [Judgment call]
- **Wasted compute (A)** — ose-primer has **no concurrency cancellation** on any of its ~23 workflows,
  so superseded pushes keep burning CI minutes; the converged block cancels in-progress PR runs only.
  This is ose-primer's largest A gap. [Repo-grounded: `grep -l "concurrency:" .github/workflows/*.yml`
  → 0]
- **Missing governance gate (A)** — ose-primer carries the `naming` validator job but **no
  `specs-gate` job**, so the BDD spec-tree structural checks (`specs:tree`/`counts`/`links`/`adoption`)
  do not gate its PRs the way the converged target requires. [Repo-grounded — `specs-gate` absent from
  `pr-quality-gate.yml`; `naming` job present]
- **Under-gated main pushes (A)** — ose-primer's `pr-quality-gate.yml` triggers on `pull_request`
  only, so a direct worktree-to-main push (the repo's TBD norm) skips the full gate; the converged
  target also gates `push` to `main`. Its per-language `test-crud-*` app schedulers run **weekly**
  (`0 10 * * 5`) — a recorded portfolio cadence the converged target keeps (ose-primer runs no
  governance sweep needing 2× WIB). [Repo-grounded — `pr-quality-gate.yml` `on: pull_request` only;
  `test-crud-*` cron `0 10 * * 5`]
- **Hook lifecycle drift (B)** — the three repos' `commit-msg`/`pre-commit`/`pre-push` hooks differ
  in build flags, lint-staged wiring, and which conditional validators run, so the local pre-flight
  contract differs per repo. [Judgment call]
- **rhino-cli architecture drift (C)** — ose-primer's CLI still carries a partial/placeholder
  hexagonal layout with a residual `src/internal/` tree; testing IO-bound logic means reaching through
  to the filesystem/process layer. A hexagonal core (pure domain + injected ports) makes the CLI
  testable and identical across repos, and folds in the salvaged `migrate-rhino-cli-to-hexagonal`
  design. ose-primer ports `ose-public`'s reference crate structure. [Repo-grounded —
  `apps/rhino-cli/src/internal/` still present alongside the hexagonal dirs]
- **rhino-cli command-surface drift (D)** — ose-primer is missing the `Specs` and `Ddd` subcommands
  that the union superset (and the siblings) carry, so the CLI is not a drop-in across repos
  (ose-primer already carries `Java` + `Contracts`). [Repo-grounded — current set has `SpecCoverage`
  but neither `Specs` nor `Ddd`]
- **Target-naming drift (E)** — governance/validation/lint targets use ad-hoc `validate:*` / `lint:*`
  / `fmt:check` names rather than the canonical `{domain}:{work}` scheme; `env:validate` does not
  follow the `{domain}:{work}` `env:validation` form; and `spec-coverage` is spelled inconsistently
  with the `:`-delimited lifecycle targets. [Repo-grounded — `apps/rhino-cli/project.json` carries
  `env:validate` and `spec-coverage`]
- **Governance drift (F)** — a rule documented in one repo's conventions but not another's quietly
  rots; ose-primer is **missing `cross-language-lint-strictness.md` entirely**, so the cross-language
  warning-threshold standard the upstream documents has no counterpart here. Without a propagation +
  quality-gate step the docs fall out of sync with the toolchain they describe. [Repo-grounded —
  `repo-governance/development/quality/cross-language-lint-strictness.md` absent in ose-primer]
- **State diagrams escape the render-width gate (G)** — the `mermaid:validation` discipline keeps
  diagrams readable on mobile viewports and in narrow PDF/doc columns, but it currently applies
  **only to flowcharts**: an 11-state `stateDiagram-v2 direction LR` chain renders far too wide for
  mobile yet sails through the gate, while an equivalent flowchart is blocked. State diagrams are an
  unguarded escape hatch from a discipline the repo already invests in. [Repo-grounded:
  `apps/rhino-cli/src/internal/mermaid/parser.rs` header regex matches only `flowchart|graph`, so
  `parse_diagram` returns count `0` for non-flowchart headers — see the `non_flowchart_returns_zero_count`
  test]

## Business Impact

### Pain Points Addressed

- A maintainer reading any sibling repo's CI/hooks/CLI cannot assume it matches `ose-primer`.
- ose-primer wastes CI minutes (no cancel-in-progress on any of its ~23 workflows).
- ose-primer under-gates its PRs (no `specs-gate` job) and its governance scheduler cadence drifts.
- ose-primer's rhino-cli is hard to unit-test (residual `src/internal/` IO-coupled layout) and is
  missing the `Specs` and `Ddd` union commands.
- Target names, `env:validate`, and `spec-coverage` spelling diverge from the canonical scheme.
- Governance docs drift from the toolchain (`cross-language-lint-strictness.md` is missing entirely)
  without a propagation + quality gate.

### Expected Benefits

- **One mental model** of the whole toolchain across all three repos (minus recorded deviations).
- **Faster, fully-gated CI** in ose-primer (concurrency cancel-in-progress on every workflow +
  `specs-gate` gating PRs).
- **Identical, testable rhino-cli** — same hexagonal architecture and same union command surface
  everywhere.
- **Canonical target names** (`{domain}:{work}`, `env:validation`, `specs:coverage`) across the family.
- **Self-healing governance** — docs propagated (incl. the newly-created
  `cross-language-lint-strictness.md`) and quality-gated so they stay in sync.
- **State diagrams held to the same render-width discipline as flowcharts** — the readability
  benefit the flowchart rule already earns transfers directly to state diagrams. _Judgment call:
  the state width/label rule is identical to the trusted flowchart rule._
- **Parity locked by a machine-checked golden corpus** — one identical state-diagram fixture set
  (`.md` + expected violation JSON) committed to all three repos makes future Mermaid-rule parity
  automatic rather than a three-way manual reconciliation.
- **A trustworthy template** teams can copy knowing it matches the upstream toolchain exactly.

## Affected Roles

Solo-maintainer repository — the roles below are **hats the maintainer wears** and **agents that
consume the outputs**:

- **CI maintainer hat** — edits the workflows and `ci-conventions.md`.
- **Toolchain/CLI maintainer hat** — performs the rhino-cli hexagonal port and command ports.
- **Template-author hat** — keeps ose-primer faithful to the upstream toolchain teams copy from.
- **`ci-checker` / `ci-fixer` agents** — validate/fix projects against `ci-conventions.md`.
- **`repo-rules-maker` / `repo-rules-checker` / `repo-rules-fixer` agents** — propagate the doc
  changes and run the final repo-rules quality gate.
- **`plan-checker` / `plan-execution-checker` agents** — validate this plan and its execution.

## Business-Level Success Metrics (per workstream)

- **A — CI parity met**: **every workflow declares a concurrency block** (ose-primer's main A gap);
  a **`specs-gate` job** runs the BDD spec-tree structural checks in CI; the **full quality gate runs
  on `push` to `main`** (today `pull_request`-only); every workflow file is kebab-case `<verb>-<noun>`
  with a Title-Case `name:` and kebab-case job ids (`Quality gate` kept); the `test-crud-*` app
  schedulers stay weekly (recorded). Confirm-only (already at target): every per-language PR-gate job
  uses `nx affected`; lint jobs are tool-named; the gherkin keyword-cardinality target runs in CI.
  [Observable — grep/diff the workflows against the CI/toolchain Parity Checklist]
- **B — Hook parity met**: `commit-msg`/`pre-commit`/`pre-push` match the BLOCK 1-B canonical
  lifecycle and reference the renamed targets. [Observable — diff the `.husky/*` hooks against
  BLOCK 1-B]
- **C — Hexagonal migration complete**: rhino-cli has the `domain`/`application`/`infrastructure`/
  `commands` layout and the golden-master CLI suite is byte-identical to the Phase 0 baseline.
  [Observable — directory layout + golden-master diff = empty]
- **D — Union command surface met**: `rhino-cli` exposes the full superset including the newly-added
  `Specs` and `Ddd` (ose-primer already carries `Java` + `Contracts`); `SpecCoverage` is folded into
  `Specs`. [Observable — `rhino-cli --help` lists all union subcommands]
- **E — Target naming met**: every governance/validation/lint/check target uses `{domain}:{work}`
  (incl. `env:validate`→`env:validation`) and `specs:coverage` repo-wide; no caller references an old
  name. [Observable — grep the project.json files, hooks, workflows, package.json]
- **F — Governance gate clean**: all related docs updated, `repo-rules-maker` propagated, and the
  `repo-rules-quality-gate` workflow reports clean. [Observable — the workflow's terminal report]
- **G — State-diagram validation met**: `mermaid:validation` flags every state diagram exceeding 4
  nodes on a rank or 30 characters in a state/transition label; zero such violations remain repo-wide
  after the aggressive cleanup; the golden corpus mirrors `ose-public`'s byte-identical violation
  output; flowchart behavior is unchanged. [Observable — run the gate over the over-wide fixtures and
  a full-repo scan; diff the committed expected-JSON against `ose-public`'s corpus]
- **All CI green after push** — the standardized toolchain passes on `origin main`. [Observable —
  GitHub Actions status]

## Business-Scope Non-Goals

- This plan does **not** converge the runner target — ose-primer stays on ephemeral hosted
  `ubuntu-latest`, an accepted, documented deviation.
- This plan does **not** add an image-publishing workflow — ose-primer is a demo template that ships
  no deployable images; the absence is a recorded deviation, not a gap.
- This plan does **not** introduce new toolchain capabilities, deploy targets, or Nx Cloud changes
  beyond parity.
- This plan does **not** perform the siblings' side of A/B/E/F (ose-public's Go-strip + `nx affected`
  convergence + lint-job rename + gherkin target add; ose-infra's version bumps, reusable-workflow
  adoption, `infra-lint` split) — each is the respective sibling plan's responsibility. For C/D/G
  ose-primer **ports from** ose-public's reference, in its own repo.

## Business Risks and Mitigations

| Risk                                                                           | Likelihood | Impact | Mitigation                                                                                                                                                                                                                         |
| ------------------------------------------------------------------------------ | ---------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ose-public`'s C/D/G reference not done before ose-primer's C/D/G phases       | Medium     | High   | C/D/G are reference-first; ose-primer ports from `ose-public`. The C/D/G phases run only after `ose-public`'s reference lands; A/B/E/F proceed independently meanwhile                                                             |
| Hexagonal migration silently changes rhino-cli output                          | Medium     | High   | Golden-master CLI suite captured in Phase 0 byte-verifies the output surface at every feature group and phase gate                                                                                                                 |
| Concurrency over-cancellation cancels a needed push/scheduled run              | Low        | Medium | The canonical group key cancels in-progress runs only on `pull_request` events; push-to-main and scheduled runs are keyed by ref and not cancelled                                                                                 |
| Target rename leaves a caller pointing at a non-existent target                | Medium     | High   | Phase 10 caller-sweep checklist + Phase 6/10 sequencing so hooks never reference an unrenamed target between phases                                                                                                                |
| `env:validate`→`env:validation` rename misses a caller                         | Medium     | Medium | Phase 10 caller-sweep greps every `.husky/*`, workflow, and `package.json` for `env:validate` before the gate                                                                                                                      |
| C/D/G port diverges from public's reference                                    | Low        | Medium | `ose-public` is the single reference crate structure + golden corpus; ose-primer copies it byte-identical; the deviation matrix records only true diffs                                                                            |
| Governance docs drift from the toolchain after edits                           | Low        | Medium | Phase 11 runs `repo-rules-maker` + the `repo-rules-quality-gate` workflow as a hard gate before the plan can finish                                                                                                                |
| State front-end (G) silently changes flowchart behavior                        | Low        | Medium | State support is a second front-end on the already-migrated, golden-frozen Mermaid slice; every flowchart test stays green before any state code lands                                                                             |
| Aggressive D-CLEAN touches `plans/done/` history                               | Low        | Low    | Explicit recorded D-CLEAN choice for maximum hygiene; edits are diagram-only and reviewed in the cleanup phase                                                                                                                     |
| Renaming a branch-protection required-check job silently breaks the merge gate | Low        | High   | GitHub keys required checks by job name; the standing decision **keeps `Quality gate` unchanged**, and any required-check rename is paired with a `[HUMAN]` branch-protection settings update (Phase 1) before relying on the gate |
