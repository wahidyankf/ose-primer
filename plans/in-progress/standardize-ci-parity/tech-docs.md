---
title: "Tech Docs: Standardize CI Parity (ose-primer sibling)"
description: "Converged CI Target and Deviation Matrix (verbatim), current-state, and design decisions for converging ose-primer CI to the shared three-repo target."
---

# Technical Documentation — Standardize CI Parity (ose-primer sibling)

## Architecture Context

ose-primer's CI surface consists of **23 workflow files** under `.github/workflows/` plus composite
actions under `.github/actions/`, all targeting the GitHub-hosted `ubuntu-latest` runner
[Repo-grounded]. The surface is the richest of the three siblings because ose-primer is the
**polyglot template**:

- **`pr-quality-gate.yml`** — the PR gate, with a full polyglot job set: `detect`, `format`,
  `typescript`, `golang`, `jvm`, `dotnet`, `python`, `rust`, `elixir`, `clojure`, `dart`,
  `markdown`, `naming`, `env-validate`, `hadolint`, `shellcheck`, `actionlint`, and the
  `quality-gate` aggregator [Repo-grounded].
- **7 `_reusable-*.yml`** — `_reusable-backend-{coverage,e2e,integration,lint,spec-coverage,
typecheck}.yml` and `_reusable-frontend-e2e.yml` (the reusable-workflow pattern is adopted)
  [Repo-grounded].
- **15 `test-crud-*.yml`** — per-language demo-app scheduled workflows (11 backends + 3 frontends +
  1 fullstack), each carrying a `schedule:` trigger [Repo-grounded].
- **`validate-markdown.yml`** — markdown validation [Repo-grounded].

| Surface area               | Status at authoring time                                                                        |
| -------------------------- | ----------------------------------------------------------------------------------------------- |
| `actions/checkout` major   | `@v6` on all 33 references [Repo-grounded]                                                      |
| Non-TS PR-gate semantics   | `nx affected` (`nx run-many` count is 0) [Repo-grounded]                                        |
| Gherkin cardinality target | `validate:gherkin-keyword-cardinality` present in `apps/rhino-cli/project.json` [Repo-grounded] |
| Reusable workflows         | 7 `_reusable-*.yml` present [Repo-grounded]                                                     |
| Lint-gate jobs             | `shellcheck`, `hadolint`, `actionlint` (tool-named — primer is the reference) [Repo-grounded]   |
| `naming` job               | present [Repo-grounded]                                                                         |
| Concurrency                | **none** — 0 of 23 files declare a `concurrency` block [Repo-grounded]                          |
| `specs-gate` job           | **absent** while `specs/` is populated [Repo-grounded]                                          |
| Scheduled cadence          | weekly `cron: "0 10 * * 5"` on all 15 `test-crud-*` [Repo-grounded]                             |
| Runner                     | `ubuntu-latest` everywhere [Repo-grounded]                                                      |

<!-- SHARED CANONICAL BLOCKS for the 3-repo standardize-ci-parity sibling set.
     Embed VERBATIM in each repo's plan tech-docs.md. Authored 2026-06-12. -->

## Converged CI Target (shared across the three-repo sibling set)

This is the **fixed end-state** every sibling plan converges to. It is a **static
specification** — not a moving target produced by another plan — so **no plan depends on
another finishing first**. Each repo converges independently and **all three plans are safe
to execute in parallel**. The three plans embed this same target verbatim; per-repo
differences are recorded in the [Deviation Matrix](#deviation-matrix).

There is **no single anchor repo**. The target is the best-of-breed union across the three
pipelines as of 2026-06-12: `ose-primer` already ships the tool-named lint jobs and the
gherkin target; `ose-public` already ships current action majors; `ose-infra` already runs
`nx affected`. Each repo leads on some dimensions and trails on others.

Sibling plans (same slug in each repo):

- `ose-public` — `plans/in-progress/standardize-ci-parity/`
  (<https://github.com/wahidyankf/ose-public/tree/main/plans/in-progress/standardize-ci-parity>)
- `ose-infra` (private) — `plans/in-progress/standardize-ci-parity/`
- `ose-primer` — `plans/in-progress/standardize-ci-parity/`
  (<https://github.com/wahidyankf/ose-primer/tree/main/plans/in-progress/standardize-ci-parity>)

| Dimension                              | Converged target end-state                                                                                                                             |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `actions/checkout` major               | `@v6`                                                                                                                                                  |
| Non-TS PR-gate test semantics          | `nx affected` (single-project governance gates such as `specs-gate` may keep `run-many`)                                                               |
| `validate:gherkin-keyword-cardinality` | Nx target present **and** wired into the markdown validator workflow                                                                                   |
| Reusable-workflow pattern              | adopted (`_reusable-*.yml` + thin callers)                                                                                                             |
| Concurrency                            | canonical block on every workflow: `group: ${{ github.workflow }}-${{ github.ref }}`, `cancel-in-progress: ${{ github.event_name == 'pull_request' }}` |
| Lint-gate jobs                         | three **tool-named** jobs: `shellcheck`, `hadolint`, `actionlint`                                                                                      |
| Governance jobs                        | `naming` (where `.claude/agents/` exists) + `specs-gate` (where `specs/` exists)                                                                       |
| Scheduled cadence                      | twice-daily WIB — `0 23 * * *` (06:00 WIB) and `0 11 * * *` (18:00 WIB) — for scheduled test/deploy workflows                                          |
| `ci-conventions.md`                    | carries a `## CI Parity Checklist` enumerating the invariants above and recording the deviations                                                       |

### Convergence status per repo (baseline 2026-06-12)

| Dimension                              | ose-public                                  | ose-infra                           | ose-primer                           |
| -------------------------------------- | ------------------------------------------- | ----------------------------------- | ------------------------------------ |
| `checkout@v6`                          | done                                        | gap — `@v4` → bump                  | done                                 |
| Non-TS `nx affected`                   | gap — `run-many` → affected                 | done                                | done                                 |
| `gherkin-keyword-cardinality` target   | gap — add + wire                            | done                                | done                                 |
| Reusable workflows                     | done                                        | gap — extract monolith              | done                                 |
| Concurrency (canonical, all workflows) | gap — add (0 today)                         | gap — add pr-gate + align 3 drifted | gap — add (0 today)                  |
| Lint jobs tool-named                   | gap — rename `shell`/`dockerfile`/`actions` | gap — split `infra-lint`            | done — reference scheme              |
| `naming` + `specs-gate`                | done — both                                 | gap — add both                      | gap — has `naming`; add `specs-gate` |
| Scheduled cadence 2× WIB               | done                                        | gap — align 1× → 2×                 | confirm/align per-language workflows |

Legend: _done_ = already at target (confirm only) · _gap_ = closed by this repo's plan.

## Deviation Matrix

Intentional per-repo differences — **recorded, not converged**. Each respects a genuine
per-repo constraint.

| Deviation                                             | ose-public                                  | ose-infra                                | ose-primer                                                             | Rationale                                                                                             |
| ----------------------------------------------------- | ------------------------------------------- | ---------------------------------------- | ---------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| Runner target                                         | `ubuntu-latest`                             | `[self-hosted, linux, ose-infra-runner]` | `ubuntu-latest`                                                        | infra needs warm Docker/Terraform/Ansible + on-prem reach; public/primer use ephemeral hosted runners |
| Language matrix                                       | TS + Go + F#/.NET + Rust                    | TS + Go + Rust                           | full polyglot (TS, Go, JVM, .NET, Python, Rust, Elixir, Clojure, Dart) | detection follows each repo's real portfolio; primer is the polyglot template                         |
| `npm` install flag                                    | `npm ci`                                    | `npm ci --ignore-scripts`                | `npm ci`                                                               | self-hosted hardening on the persistent infra runner                                                  |
| `setup-docker` composite                              | absent                                      | present                                  | absent                                                                 | hosted runners ship Docker; self-hosted must warm it                                                  |
| Rust toolchain action                                 | `actions-rust-lang/setup-rust-toolchain@v1` | `dtolnay/rust-toolchain@stable`          | `actions-rust-lang/setup-rust-toolchain@v1`                            | existing infra composite; kept to avoid churn                                                         |
| IaC lint job (`iac-lint`: terraform/ansible/yamllint) | absent                                      | present                                  | absent                                                                 | infra-only — terraform/ansible/yaml surface exists only in ose-infra                                  |

<!-- END SHARED CANONICAL BLOCKS -->

## Current State — ose-primer Specific

### Already at the Converged CI Target (confirm only — no action)

| Dimension                  | Evidence (authoring time)                                                                     |
| -------------------------- | --------------------------------------------------------------------------------------------- |
| `actions/checkout@v6`      | [Repo-grounded — 33 references, all `@v6`; no `@v4`/`@v5` remain]                             |
| Non-TS `nx affected`       | [Repo-grounded — `nx affected` appears 9× in `pr-quality-gate.yml`; `nx run-many` count is 0] |
| Gherkin cardinality target | [Repo-grounded — `validate:gherkin-keyword-cardinality` in `apps/rhino-cli/project.json`]     |
| Reusable workflows         | [Repo-grounded — 7 `_reusable-*.yml`]                                                         |
| Tool-named lint jobs       | [Repo-grounded — `shellcheck`, `hadolint`, `actionlint` jobs; primer is the reference scheme] |
| `naming` job               | [Repo-grounded — `naming` job runs `validate:naming-agents` + `validate:naming-workflows`]    |
| `ubuntu-latest` runner     | [Repo-grounded — every job `runs-on: ubuntu-latest`]                                          |

### Gaps this plan closes

| #   | Gap                                                                                              | Evidence                                                                                                | Convergence action        |
| --- | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------- | ------------------------- |
| G1  | No `concurrency` block on any of the 23 workflow files                                           | [Repo-grounded — `grep -L 'concurrency:' .github/workflows/*.yml` lists all 23]                         | Phase 1 — add canonical   |
| G2  | `pr-quality-gate.yml` has no `specs-gate` job though `specs/` is populated                       | [Repo-grounded — no specs-gate job; `specs/apps/crud`, `specs/apps/rhino`, `specs/libs/*` exist]        | Phase 2 — add + wire      |
| G3  | The 15 `test-crud-*` workflows run **weekly** (`cron: "0 10 * * 5"`), not twice-daily WIB        | [Repo-grounded — identical weekly cron in all 15 files]                                                 | Phase 3 — align to 2× WIB |
| G4  | `ci-conventions.md` has no `## CI Parity Checklist` and omits the Converged CI Target invariants | [Repo-grounded — `grep "CI Parity Checklist" repo-governance/development/infra/ci-conventions.md` is 0] | Phase 4 — add + align     |

### specs-gate target-availability caveat (drives Decision D2)

ose-public's `specs-gate` job runs
`npx nx run-many -t validate:specs-adoption validate:specs-tree validate:specs-counts
validate:specs-links --projects=rhino-cli`, and ose-public's `apps/rhino-cli/project.json` **defines
all four `validate:specs-*` targets** [Repo-grounded — ose-public].

ose-primer's `apps/rhino-cli/project.json` **does NOT define any `validate:specs-*` target**
[Repo-grounded — primer's rhino-cli exposes `validate:gherkin-keyword-cardinality`,
`validate:heading-hierarchy`, `validate:links`, `validate:mermaid`, `validate:naming-agents`,
`validate:naming-workflows`, `validate:cross-vendor-parity`, `validate:harness-bindings`,
`validate:repo-governance-vendor-audit`, and `validate:repo-governance-vendor-audit` — but no
`validate:specs-tree` / `-links` / `-counts` / `-adoption`]. Primer does already validate gherkin
**step coverage** against `specs/` via the `spec-coverage` Nx target, which runs in the PR gate today
inside the per-language jobs [Repo-grounded — `spec-coverage` referenced 9× in `pr-quality-gate.yml`].

**Consequence:** the primer `specs-gate` job **cannot mirror ose-public's command verbatim** because
the targets do not exist. Phase 2 therefore audits target availability first, then takes one of two
recorded paths (D2): **(2a)** port the `validate:specs-*` targets into primer's rhino-cli and wire the
specs-gate to them (full structural parity), or **(2b)** wire a primer `specs-gate` to the specs
validators that DO exist (gherkin `spec-coverage` over the `specs/` tree) and record the
target-porting as a follow-up. The plan does not pre-commit to one; the decision is made from the
Phase 2 audit and recorded in the implementation notes. This is the plan's one genuine open question
(see [§Open Questions](#open-questions)).

## Design Decisions

### D1 — Canonical concurrency expression on every workflow

Adopt the Converged CI Target block on all 23 workflows (none have one today):
`group: ${{ github.workflow }}-${{ github.ref }}` and
`cancel-in-progress: ${{ github.event_name == 'pull_request' }}`. The PR-only cancel expression means
scheduled `test-crud-*` runs are not cancelled by ref-collision (a `schedule` event keeps
`cancel-in-progress` false), while PR pushes cancel superseded runs. **Decision:** one canonical
expression, top-level, after `permissions:`, in every workflow that runs jobs.

### D2 — specs-gate mirrors ose-public's job shape, wired to validators that exist in primer

The specs-gate job structure mirrors ose-public's (`runs-on: ubuntu-latest`, `setup-node` +
`setup-rust`, a single `npx nx ...` validate step, added to the `quality-gate` aggregator `needs:`).
The **command** is resolved by the Phase 2 availability audit per the caveat above (path 2a port, or
path 2b wire-to-existing). **Decision:** same job shape as ose-public; command resolved from the
audit, with the chosen path recorded.

### D3 — Cadence aligned to twice-daily WIB (baseline shows divergence)

The baseline grep shows all 15 `test-crud-*` workflows on `cron: "0 10 * * 5"` — **weekly, Friday
10:00 UTC**, which **diverges** from the Converged CI Target's twice-daily WIB cadence. **Decision:**
align each `test-crud-*` schedule to the two canonical crons `0 23 * * *` (06:00 WIB) and
`0 11 * * *` (18:00 WIB), replacing the single weekly cron. (Were the baseline already twice-daily WIB,
this phase would degrade to confirm-only; it is not, so an alignment phase is included.)

### D4 — Governance aligned in place, not a new hub

Align `repo-governance/development/infra/ci-conventions.md` to the Converged CI Target and add a
`## CI Parity Checklist` enumerating the invariants + recording the deviations. ose-primer does **not**
have a `cross-language-lint-strictness.md` doc (that is ose-public-only), so the parity checklist
references primer's own lint governance generically rather than cross-linking a public-only path;
creating a dedicated primer lint-strictness doc is flagged as an eval, not done here. Evaluate
`ci-checker` for parity-check additions; re-sync bindings via `npm run generate:bindings` **only if**
`.claude/agents/ci-checker.md` changes. **Decision:** in-place governance alignment.

## File Impact

| Path                                                        | Change                                                                | Type             |
| ----------------------------------------------------------- | --------------------------------------------------------------------- | ---------------- |
| `.github/workflows/pr-quality-gate.yml`                     | add concurrency; add `specs-gate` job; wire into `quality-gate` needs | config (YAML)    |
| `.github/workflows/validate-markdown.yml`                   | add canonical concurrency                                             | config (YAML)    |
| `.github/workflows/_reusable-*.yml` (7 files)               | add canonical concurrency                                             | config (YAML)    |
| `.github/workflows/test-crud-*.yml` (15 files)              | add canonical concurrency; align schedule to 2× WIB                   | config (YAML)    |
| `apps/rhino-cli/project.json` _(conditional — path 2a)_     | add `validate:specs-*` targets if porting them for the specs-gate     | config (JSON)    |
| `repo-governance/development/infra/ci-conventions.md`       | align to Converged CI Target; add `## CI Parity Checklist`            | governance doc   |
| `.claude/agents/ci-checker.md` _(conditional)_              | optional parity-check additions                                       | agent definition |
| `.opencode/agents/ci-checker.md` _(generated, conditional)_ | re-synced via `npm run generate:bindings` if `ci-checker.md` changes  | generated        |

_Conditional_ markers flag files touched only on a specific Phase 2 / Phase 4 decision path.

## Testing Strategy

CI YAML and governance docs are validated, not unit-tested. Per the non-code carve-out in
`repo-governance/development/workflow/test-driven-development.md`:

- **Config (YAML/JSON) steps** use a RED→GREEN→REFACTOR shape where RED is a `grep`/`actionlint`
  assertion that the desired state is absent, GREEN is the YAML/JSON edit, REFACTOR is `actionlint`
  cleanup. The Gherkin acceptance criteria in [prd.md](./prd.md) are the source of the RED assertions.
- **Governance-doc and agent-definition edits** use DIRECT-ACTION + acceptance (no RED/GREEN/
  REFACTOR), per the non-code carve-out.
- **Workflow correctness** is verified with `actionlint` on every changed workflow; the specs-gate
  command (once wired) is verified to run against primer's `specs/` tree.

## Rollback

Each phase is an independent commit on `main`. Rollback = `git revert <commit>` for the offending
phase. The bulk concurrency edit (Phase 1) and the cadence edit (Phase 3) each touch many files but
are mechanical and revert cleanly; the specs-gate phase (Phase 2) reverts to no specs-gate in one
revert.

## Open Questions

- **specs-gate validator source (D2)** — primer's rhino-cli lacks the `validate:specs-*` targets
  ose-public uses. Phase 2 must decide between porting those targets (2a) or wiring the specs-gate to
  primer's existing gherkin `spec-coverage` over `specs/` (2b). This is the only material decision the
  plan defers to execution; it is resolved from the Phase 2 availability audit, not pre-committed here.

## Cross-References

- WHY: [brd.md](./brd.md)
- WHAT: [prd.md](./prd.md)
- DO: [delivery.md](./delivery.md)
  </content>
