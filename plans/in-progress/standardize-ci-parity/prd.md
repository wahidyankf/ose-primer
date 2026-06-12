---
title: "PRD: Standardize CI Parity (ose-primer sibling)"
description: "Product requirements and Gherkin acceptance criteria for converging ose-primer CI to the shared Converged CI Target of the three-repo sibling set."
---

# Product Requirements — Standardize CI Parity (ose-primer sibling)

## Product Overview

A CI-plumbing convergence: ose-primer's GitHub Actions workflows, the `ci-conventions.md` governing
doc, and (conditionally) the `ci-checker` agent are brought into line with the shared **Converged CI
Target** — adding the canonical concurrency block to every workflow, adding the missing `specs-gate`
PR-gate job, confirming or aligning the scheduled cadence, and aligning governance with a `## CI
Parity Checklist`. Primer is the most converged of the three siblings, so the dimensions already at
target (current action majors, `nx affected`, the gherkin target, reusable workflows, tool-named lint
jobs, the `naming` job, the `ubuntu-latest` runner) are **confirmed only**, not changed. The
deliverable is the CI surface itself plus aligned governance, not demo-app behavior.

## Personas (hats the solo maintainer wears)

- **CI maintainer** — edits the workflow files (concurrency, specs-gate, cadence).
- **Governance author** — aligns `ci-conventions.md` and (conditionally) the `ci-checker` agent.
- **Template consumer (external)** — clones ose-primer and inherits this CI surface.
- **Consuming agents** — `ci-checker` (reads the conventions + parity checklist), `plan-checker`
  (reads this plan).

## User Stories

- As the **CI maintainer**, I want every workflow to cancel superseded runs so the GitHub-hosted
  runner is not wasted on stale commits across the full polyglot matrix.
- As the **CI maintainer**, I want the PR gate to run a `specs-gate` job so specs-structure
  regressions are blocked before merge, matching the sibling repos' coverage.
- As the **CI maintainer**, I want the scheduled cadence confirmed or aligned to the documented
  twice-daily WIB pattern so the workflows agree with the Converged CI Target.
- As the **governance author**, I want `ci-conventions.md` to carry a `## CI Parity Checklist` so the
  parity contract is explicit and `ci-checker` can audit against it.
- As the **template consumer**, I want the cloned CI surface to already meet the parity standard so I
  start from a converged baseline.

## Acceptance Criteria (Gherkin)

Each scenario uses exactly one primary `Given`, one `When`, one `Then`; extras chain with
`And`/`But` (honoring `validate:gherkin-keyword-cardinality`).

```gherkin
Scenario: Canonical concurrency added to every workflow
  Given no ose-primer workflow declares a concurrency block today
  When the concurrency phase completes
  Then every workflow that runs jobs declares the canonical concurrency block
  And cancel-in-progress is set to the canonical PR-only expression
  And actionlint reports no errors on the changed files
```

```gherkin
Scenario: Concurrency group key matches the canonical expression
  Given the Converged CI Target fixes the group key to github.workflow plus github.ref
  When the concurrency phase completes
  Then each concurrency block uses group "${{ github.workflow }}-${{ github.ref }}"
  But no workflow uses a non-canonical group key
```

```gherkin
Scenario: specs-gate job added to the PR quality gate
  Given pr-quality-gate.yml has no specs-gate job while a populated specs/ tree exists
  When the specs-gate phase completes
  Then pr-quality-gate.yml defines a specs-gate job validating the specs/ tree
  And the job runs on ubuntu-latest mirroring how ose-public wires its specs-gate
```

```gherkin
Scenario: specs-gate validator target availability confirmed before wiring
  Given primer's rhino-cli may lack the validate:specs-* Nx targets ose-public uses
  When the specs-gate phase audits apps/rhino-cli/project.json
  Then the job is wired to validators that actually exist in primer's rhino-cli
  But if the validate:specs-* targets are absent the phase records the porting decision first
```

```gherkin
Scenario: specs-gate wired into the quality-gate aggregator
  Given the quality-gate aggregator job lists the gating jobs in its needs array
  When the specs-gate phase completes
  Then the quality-gate needs array includes specs-gate
  And the aggregator failure check treats a specs-gate failure as blocking
```

```gherkin
Scenario: Scheduled cadence confirmed or aligned to twice-daily WIB
  Given the 15 test-crud-* workflows run weekly on cron "0 10 * * 5"
  When the cadence phase completes
  Then each test-crud-* workflow declares the twice-daily WIB crons "0 23 * * *" and "0 11 * * *"
  And the cadence matches the Converged CI Target table in ci-conventions.md
```

```gherkin
Scenario: Already-at-target dimensions confirmed without change
  Given primer already ships checkout@v6, nx affected, the gherkin target, reusable workflows, tool-named lint jobs, and a naming job
  When the Phase 0 baseline audit runs
  Then each already-at-target dimension is recorded as confirmed
  And no workflow edit is made for those dimensions
```

```gherkin
Scenario: ci-conventions aligned with a CI Parity Checklist section
  Given ci-conventions.md has no CI Parity Checklist section today
  When the governance phase completes
  Then ci-conventions.md contains a "## CI Parity Checklist" section enumerating the Converged CI Target invariants
  And the documented cadence no longer contradicts the workflows
```

```gherkin
Scenario: ci-checker re-synced only after parity-check edits
  Given the ci-checker agent definition may gain parity-check additions
  When .claude/agents/ci-checker.md changes
  Then npm run generate:bindings is run to re-sync platform bindings
  But if ci-checker.md is unchanged the re-sync step is a recorded no-op
```

```gherkin
Scenario: Plan's own files pass the documentation gates
  Given this plan lives three directory levels below the repo root
  When the doc-gate phase runs the link, mermaid, heading, and gherkin validators
  Then all validators exit 0 for the plan's five files
  And no committed file contains a real secret
```

## Product Scope

### In scope

- Adding the canonical `concurrency` block to all 23 workflow files (none today).
- Adding a `specs-gate` job to `pr-quality-gate.yml` and wiring it into the `quality-gate`
  aggregator `needs:` list — after confirming which validators primer's rhino-cli actually exposes.
- Confirming or aligning the 15 `test-crud-*` schedules to the twice-daily WIB cadence.
- Aligning `ci-conventions.md` + adding a `## CI Parity Checklist`; evaluating `ci-checker`;
  re-syncing platform bindings only if `ci-checker.md` changes.

### Out of scope

- Changing the runner target (kept `ubuntu-latest`; already at target).
- Bumping action majors (`actions/checkout@v6` already everywhere — confirm only).
- Adding the `validate:gherkin-keyword-cardinality` target (already present — confirm only).
- Extracting reusable workflows (already adopted — confirm only).
- Renaming lint jobs (already tool-named — primer is the reference scheme — confirm only).
- Adding a `naming` job (already present — confirm only).
- A new CI hub doc; demo-app or library behavior changes.

## Product-Level Risks

- **specs-gate target gap** — primer's rhino-cli may lack the `validate:specs-*` targets ose-public
  uses; mitigated by the Phase 2 availability audit and an explicit porting-or-substitute decision
  before the job is wired.
- **Cadence cost** — twice-daily across 15 polyglot workflows raises scheduled-run volume; mitigated
  by the Phase 3 baseline-driven decision and concurrency cancellation of re-triggers.
- **Bulk concurrency edit** — touching 23 files risks a YAML slip; mitigated by `actionlint` on all
  changed files at each gate.

## Cross-References

- WHY: [brd.md](./brd.md)
- HOW: [tech-docs.md](./tech-docs.md)
- DO: [delivery.md](./delivery.md)
  </content>
