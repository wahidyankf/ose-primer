---
title: "BRD: Standardize CI Parity (ose-primer sibling)"
description: "Business rationale for converging ose-primer's CI to the shared Converged CI Target of the three-repo standardize-ci-parity sibling set."
---

# Business Requirements — Standardize CI Parity (ose-primer sibling)

## Business Goal

Bring ose-primer's GitHub Actions CI into line with the shared **Converged CI Target** of the
three-repo `standardize-ci-parity` sibling set, so that `ose-public`, `ose-infra`, and `ose-primer`
share one current, governed, predictable CI standard. A single shared standard is what lets the
maintainer reason about all three repos' pipelines without holding three divergent mental models —
and, because ose-primer is the **public polyglot template** teams clone to build their own
Sharia-compliant products, its CI is also the reference others inherit.

## Dependency Position (business framing)

- This is the **ose-primer sibling** of a three-repo set. It converges to a **static Converged CI
  Target** (recorded verbatim in [tech-docs.md](./tech-docs.md)), not to another repo's plan output.
- **No anchor repo, no cross-plan ordering.** This plan depends on **no sibling plan** and is
  **safe to run in parallel** with the `ose-public` and `ose-infra` siblings.
- ose-primer normally receives content from `ose-public` via propagation PRs; **this plan is the
  exception** and executes directly in ose-primer.

## Business Impact

### Pain points addressed (grounded in the baseline)

- **No concurrency guard on any workflow.** Zero of ose-primer's 23 workflow files declare a
  `concurrency` block [Repo-grounded — `grep -l concurrency: .github/workflows/*.yml` returns
  nothing across all 23 files]. Superseded PR runs and re-triggered scheduled runs are never
  auto-cancelled, wasting GitHub-hosted runner minutes across the full polyglot job matrix.
- **Missing specs governance gate on PRs.** `pr-quality-gate.yml` runs no `specs-gate` job even
  though ose-primer ships a populated `specs/` tree (`specs/apps/crud`, `specs/apps/rhino`,
  `specs/libs/ts-ui`, `specs/libs/golang-commons`) [Repo-grounded — `specs/` exists; no specs-gate
  job in the workflow]. Specs-structure regressions can merge unblocked today.
- **Unconfirmed scheduled cadence.** All 15 `test-crud-*` workflows run on `cron: "0 10 * * 5"` —
  **once weekly, Friday 10:00 UTC (17:00 WIB)** [Repo-grounded — `grep -A2 schedule:
  .github/workflows/test-crud-*.yml` shows the identical weekly cron in all 15 files]. The Converged
  CI Target specifies a **twice-daily WIB cadence** (`0 23 * * *` + `0 11 * * *`) for scheduled
  test/deploy workflows; the current weekly cadence **diverges**.
- **Governance not yet aligned.** `ci-conventions.md` has no `## CI Parity Checklist` section and
  does not yet record the Converged CI Target invariants [Repo-grounded — `grep "CI Parity
  Checklist" repo-governance/development/infra/ci-conventions.md` returns 0].

### Expected benefits

- One shared, current CI standard across all three repos — lower cognitive load [Judgment call].
- The public template's CI demonstrates the parity standard others inherit when they clone it.
- A complete PR-gate governance surface catches specs regressions before merge.
- Canonical concurrency cancels superseded runs, conserving hosted runner capacity across the rich
  polyglot matrix.

## Affected Roles (hats the solo maintainer wears)

Solo-maintainer repo — no sign-off ceremonies.

- **CI maintainer** — owns the workflow files and the concurrency / specs-gate / cadence changes.
- **Governance author** — owns `ci-conventions.md` and the `ci-checker` agent definition.
- **Template consumer (external)** — teams cloning ose-primer inherit this CI surface; the parity
  standard is what they start from.
- **Consuming agents** — `ci-checker` reads the aligned conventions + parity checklist;
  `plan-checker` reads this plan's own files.

## Business-Level Success Metrics

- **Concurrency coverage** — every workflow that runs jobs carries a `concurrency` block with the
  canonical group key [observable fact — verifiable via `grep -L 'concurrency:'
  .github/workflows/*.yml` listing none].
- **PR-gate specs coverage** — `pr-quality-gate.yml` runs a `specs-gate` job and the
  `quality-gate` aggregator lists it in `needs:` [observable fact — verifiable in the workflow file].
- **Cadence alignment** — the `test-crud-*` schedules match the twice-daily WIB cadence (or the plan
  records an explicit, justified decision to keep the existing cadence) [observable fact].
- **Governance alignment** — `ci-conventions.md` carries a `## CI Parity Checklist` section and no
  longer omits the Converged CI Target invariants [observable fact].
- **DoD** — the full 5-doc plan passes plan-quality-gate strict, is pushed to ose-primer
  `origin/main`, and CI is green [observable fact].

## Business-Scope Non-Goals

- **Changing the runner target** — `ubuntu-latest` is kept (already at target; identical to public).
- **Adding or removing template capabilities** — the polyglot demo apps and their per-language
  workflows are not changed beyond concurrency + cadence plumbing.
- **A new CI hub doc** — governance is aligned in place in the existing `ci-conventions.md`.
- **Changing application or demo-app behavior** — this plan touches CI plumbing, governance docs,
  and (conditionally) one agent definition only.

## Business Risks and Mitigations

| Risk                                                                      | Likelihood | Mitigation                                                                                                                 |
| ------------------------------------------------------------------------- | ---------- | -------------------------------------------------------------------------------------------------------------------------- |
| The `specs-gate` job has no `validate:specs-*` Nx target to run in primer | Medium     | Phase 2 verifies target availability first; if absent, the phase ports the targets or gates on the existing spec-coverage. |
| Cadence change increases scheduled-run cost across 15 polyglot workflows  | Medium     | Phase 3 decides cadence from the baseline; twice-daily is still bounded and concurrency cancels re-triggers.               |
| Adding concurrency to 23 files introduces a YAML syntax error             | Low        | Each phase runs `actionlint` on all changed workflows before its gate passes.                                              |
| Governance doc drifts from the actual workflows again                     | Low        | The `## CI Parity Checklist` makes the parity contract explicit and checkable by `ci-checker`.                             |

## Cross-References

- WHAT / acceptance criteria: [prd.md](./prd.md)
- HOW / Converged CI Target + deviation matrix: [tech-docs.md](./tech-docs.md)
- DO / phased checklist: [delivery.md](./delivery.md)
  </content>
