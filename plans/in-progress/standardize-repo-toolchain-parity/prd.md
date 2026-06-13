# PRD — Standardize Repo Toolchain Parity (ose-primer)

This Product Requirements Document specifies **what** gets built. The **why** lives in
[brd.md](./brd.md); the **how** lives in [tech-docs.md](./tech-docs.md). The Gherkin scenarios below
are the source of the first failing verification assertions in [delivery.md](./delivery.md).

## Product Overview

A set of verifiable changes across **seven workstreams (A–G)** that close ose-primer's gaps against the fixed
**Converged Toolchain Target** shared with `ose-public` and `ose-infra`, except the recorded per-repo
deviations. A/B/E/F are parallel-safe (no single anchor); C/D/G are reference-first (ose-public leads,
ose-primer ports):

1. **A — CI**: canonical concurrency on every workflow (ose-primer's main A gap); add a `specs-gate`
   CI job; add the **full quality gate on `push` to `main`** (today `pull_request`-only); workflow
   file/`name:`/job-id naming onto the canonical BLOCK 1-A scheme; keep the `test-crud-*` app
   schedulers weekly (ose-primer runs no governance sweep). Confirm-only: `nx affected` per-language
   jobs, tool-named lint jobs, and the gherkin keyword-cardinality target + CI wiring.
2. **B — Hooks**: `commit-msg`/`pre-commit`/`pre-push` converge to the canonical BLOCK 1-B lifecycle
   (pre-commit gains `test:quick` = format+lint+typecheck+test:unit; pre-push = `specs:coverage` +
   `test-coverage`).
3. **H — Test Lifecycle Architecture**: three-level testing (unit/integration/e2e) sharing the same
   `.feature` files; `test:unit` (mocked) at pre-commit via `test:quick`; `specs:coverage` +
   `test-coverage` at pre-push; **`test:integration`+`test:e2e` CRON-only** per app-group at **1×/day**
   (ose-primer); `specs:coverage` enforces all scenarios across all three levels; heavy-test workflow
   `test-and-deploy-{app-group}-development.yml` only — **ose-primer builds NO staging container and
   runs NO staging test (no staging area)**; prod deploy manual.
4. **C — rhino-cli architecture (PORT)**: placeholder layout → full hexagonal, behavior-frozen by a
   golden-master CLI suite, mirroring `ose-public`'s reference.
5. **D — rhino-cli commands + scope-based regroup (PORT)**: rationalize + regroup (`docs`→`md`,
   `agents`→`harness`, `java`→`lang`; fold `spec-coverage`/`ddd`/`contracts`/`gherkin`→`specs`; new
   `convention`; `docs` reserved) + uniform grammar; port the **`specs` structural set** (ose-primer
   already has `Java` + `Contracts` → now `lang` + `specs` codegen; `SpecCoverage` folds into `specs`).
6. **E — Target naming**: `{domain}:{work}` rename (incl. `env:validate`→`env:validation`) +
   `spec-coverage`→`specs:coverage` repo-wide.
7. **F — Governance**: **create** `cross-language-lint-strictness.md` (missing in primer); update all
   other related docs, run `repo-rules-maker`, then `repo-rules-quality-gate` until clean.
8. **G — Mermaid state-diagram validation (PORT)**: add the `state.rs` front-end + width/label rules
   for state diagrams to the migrated Mermaid slice, mirror `ose-public`'s shared golden corpus
   byte-identical, and clean up every violating state diagram repo-wide. Depends on workstream C.

## Personas

Solo-maintainer repo — hats the maintainer wears plus consuming agents:

- **CI maintainer** — wants all three repos' pipelines to behave identically.
- **Toolchain/CLI maintainer** — wants a testable, identical rhino-cli across repos.
- **Contributor / AI agent** — pushes changes and expects superseded runs cancelled and PRs
  gated by the full validator set including `specs-gate`.
- **`ci-checker` / `repo-rules-*` agents** — validate the toolchain and propagate/gate the docs.
- **Template-author hat** — keeps ose-primer faithful to the upstream toolchain teams copy from.

## User Stories

- **US-1 (A)** — As a CI maintainer, I want every workflow to declare a concurrency block so that
  superseded PR runs are cancelled and ose-primer's CI minutes are not wasted (its main A gap).
- **US-2 (A)** — As a CI maintainer, I want a `specs-gate` CI job added so that the BDD spec-tree
  structural checks gate ose-primer's PRs like the converged target requires.
- **US-3 (A)** — As a CI maintainer, I want every workflow file name, `name:` field, and job id on the
  canonical BLOCK 1-A scheme so that the workflow graph reads identically across repos — with the
  `Quality gate` required-check name kept (any required-check rename paired with a branch-protection
  update).
- **US-3b (A)** — As a CI maintainer, I want the full quality gate to also run on `push` to `main`
  (today `pull_request`-only) so that direct worktree-to-main pushes are gated identically to PRs.
- **US-3c (A)** — As a CI maintainer, I want to confirm ose-primer's already-converged A dimensions
  (`nx affected`, tool-named lint jobs, the gherkin target + CI wiring) and keep the `test-crud-*`
  schedulers weekly so that no regression is introduced while closing the real gaps.
- **US-6 (B)** — As a toolchain maintainer, I want the git hooks to match the canonical lifecycle so
  that the local pre-flight contract is identical across repos.
- **US-7 (C)** — As a toolchain maintainer, I want rhino-cli ported to the full hexagonal layout with
  its behavior frozen so that the CLI is testable and identical to `ose-public`'s reference.
- **US-8 (D)** — As a toolchain maintainer, I want the command surface **regrouped by scope**
  (`docs`→`md`, `agents`→`harness`, `java`→`lang`; `ddd`/`contracts`/`spec-coverage`/`gherkin` folded
  into `specs`; new `convention`) under one **uniform `<group> [<language>] <verb> [<object>]` grammar**,
  and the missing **`specs` structural set** added, so that the CLI is the union superset and drop-in
  across repos.
- **US-9 (E)** — As a toolchain maintainer, I want every governance target renamed to `{domain}:{work}`
  (incl. `env:validate`→`env:validation`) and `spec-coverage`→`specs:coverage` so that target names
  are canonical everywhere.
- **US-10 (F)** — As a governance maintainer, I want `cross-language-lint-strictness.md` created and
  all related docs updated, propagated by `repo-rules-maker`, and passed through the
  `repo-rules-quality-gate` so that the docs never drift from the toolchain.
- **US-12 (G)** — As a documentation author, I want over-wide state diagrams and long state /
  transition labels flagged by `mermaid:validation`, so that my state diagrams stay readable on
  mobile just like my flowcharts, and the golden corpus locks identical behavior across the three
  repos.

## Acceptance Criteria (Gherkin)

Each scenario uses exactly one primary `Given`, one `When`, one `Then`; extras chain with
`And`/`But`.

```gherkin
Scenario: Baseline recorded and golden-master captured before work begins
  Given ose-primer has no upstream prerequisite plan to verify
  When Phase 0 runs its baseline and golden-master gate
  Then the affected baseline is recorded with every preexisting failure resolved
  And a golden-master CLI corpus for every rhino-cli subcommand is recorded
  And a re-run of the capture produces a byte-identical corpus
```

```gherkin
Scenario: Non-TS PR-gate jobs already use nx affected
  Given pr-quality-gate.yml already runs nx affected for every per-language job
  When the affected-semantics dimension is confirmed
  Then each per-language job invokes nx affected and not nx run-many
  And the inline NX_BASE/NX_HEAD env vars remain set on each affected job
  But no per-language job needs conversion in ose-primer
```

```gherkin
Scenario: Canonical concurrency block added to every workflow
  Given ose-primer workflows currently declare no concurrency group
  When the concurrency block is added to the PR gate, validator, and scheduled workflows
  Then each targeted workflow declares a concurrency group keyed by workflow and PR number or ref
  And cancel-in-progress is true only for pull_request events
```

```gherkin
Scenario: Lint-gate jobs already follow the tool-named scheme
  Given pr-quality-gate.yml already declares the lint jobs shellcheck and hadolint
  When the lint-gate job scheme is confirmed
  Then the lint jobs are tool-named shellcheck, hadolint, and actionlint
  And quality-gate.needs references the tool-named jobs
  But no lint job needs renaming in ose-primer
```

```gherkin
Scenario: A specs-gate job is added and the workflow naming is canonical
  Given pr-quality-gate.yml carries the naming job but no specs-gate job
  When the specs-gate addition and BLOCK 1-A naming convergence is applied
  Then pr-quality-gate.yml declares a specs-gate job running the specs structural checks
  And every workflow file is kebab-case with a Title-Case name and kebab-case job ids
  And the Quality gate required-check job name is unchanged
  But ose-primer keeps its full polyglot language rows untouched
```

```gherkin
Scenario: Gherkin keyword-cardinality validator already runs in CI
  Given ose-primer already wires the gherkin keyword-cardinality validator into CI
  When the validator dimension is confirmed
  Then the markdown validator workflow invokes the gherkin keyword-cardinality target
  And the target passes against the current repository tree
  But the target is renamed to the canonical specs:gherkin-cardinality-validation name in Phase 10
```

```gherkin
Scenario: Full quality gate runs on push to main
  Given pr-quality-gate.yml triggers on pull_request only today
  When the push-to-main full gate is added
  Then the full quality gate runs on push to the main branch
  And a direct worktree-to-main push is gated identically to a pull request
```

```gherkin
Scenario: Git hooks converge to the canonical lifecycle
  Given the ose-primer hooks differ from the BLOCK 1-B canonical lifecycle
  When the hook convergence is applied
  Then commit-msg, pre-commit, and pre-push match the canonical lifecycle
  And every target the hooks invoke exists under its canonical name
```

```gherkin
Scenario: rhino-cli ports to the full hexagonal architecture with behavior frozen
  Given rhino-cli carries a placeholder hexagonal layout with a residual src/internal tree
  When the hexagonal port is applied feature by feature
  Then rhino-cli has src/domain, src/application, src/infrastructure, and src/commands layers
  And the golden-master CLI corpus is byte-identical to the Phase 0 baseline
  And the layout matches the reference layout ose-public authored
```

```gherkin
Scenario: rhino-cli subcommands are regrouped by scope and renamed to the uniform grammar
  Given rhino-cli uses old groups and hyphenated forms like docs validate-mermaid and agents emit-bindings
  When the Phase 9a scope-based regroup and Phase 9b uniform-grammar rename are applied
  Then every subcommand reads uniform like md validate mermaid and harness emit amazonq
  And the docs group is reserved and the convention group exists
  And no caller (project.json, hooks, package.json, docs) invokes an old group or hyphenated subcommand
  And the golden-master corpus is re-captured for the renamed surface
  But env init/backup/restore/validate and git pre-commit stay unchanged
```

```gherkin
Scenario: rhino-cli exposes the regrouped union command superset
  Given rhino-cli is missing the specs structural set under the regrouped surface
  When the scope-based regroup and the specs structural-set port are applied
  Then rhino-cli --help lists the md, convention, harness, specs, and lang groups
  And the specs group carries adoption, counts, tree, links, coverage, bc, ul, and gherkin-cardinality
  And SpecCoverage, Ddd, Contracts, and the gherkin validator are folded into specs
  And the command surface matches the regrouped union superset shared across the three repos
```

```gherkin
Scenario: Governance targets renamed to the canonical scheme
  Given the governance targets use ad-hoc validate/lint/fmt names, env:validate, and spec-coverage
  When the {domain}:{work} rename is applied repo-wide
  Then every governance, validation, lint, and check target uses the {domain}:{work} scheme
  And env:validate is renamed to env:validation
  And spec-coverage is renamed to specs:coverage in every project.json
  But every caller (hooks, workflows, package.json) references only the new names
```

```gherkin
Scenario: Governance docs pass the repo-rules quality gate
  Given the related docs have been updated for the converged toolchain
  When repo-rules-maker propagates the changes and the repo-rules-quality-gate workflow runs
  Then all related docs reflect the converged toolchain
  And the repo-rules-quality-gate workflow reports clean before the plan is marked done
```

```gherkin
Scenario: Full toolchain green after push
  Given all phase changes are committed and pushed to origin main
  When GitHub Actions runs the standardized workflows
  Then all CI checks pass with zero failures
  And the new specs-gate job and the concurrency-equipped workflows ran and are green
  And the specs:gherkin-cardinality-validation step is present and green
```

### Workstream H — Test Lifecycle Architecture acceptance criteria

```gherkin
Scenario: The three test levels share one set of feature specs
  Given an app has Gherkin .feature files under specs
  When test:unit, test:integration, and test:e2e run for that app
  Then all three levels execute the same .feature scenarios
  And test:unit uses mocks while test:integration uses same-container deps and test:e2e may use any deps
  And specs:coverage fails if any scenario is unimplemented in any of the three levels
```

```gherkin
Scenario: Heavy tests run only from CRON
  Given test:integration and test:e2e are heavy
  When the pre-commit, pre-push, PR gate, and push-to-main stages run
  Then none of those stages invoke test:integration or test:e2e
  And the heavy tests run only from the scheduled per-app-group workflows
  But pre-commit still runs test:unit via test:quick
```

```gherkin
Scenario: Heavy-test workflow exists per app-group at the ose-primer cadence
  Given each app-group is a deployable family from the Nx project graph
  When the heavy-test workflow is created for ose-primer
  Then test-and-deploy-{app-group}-development.yml runs integration and e2e using Dockerfile or local deps
  And the cadence is 1x per day for ose-primer
  But ose-primer builds no staging container and runs no test-{app-group}-staging.yml
```

### Workstream G — Mermaid state-diagram validation acceptance criteria

> These scenarios (ported from the folded `mermaid-state-diagram-validation` plan) become the first
> failing tests in Phase 8. Each uses exactly one primary `Given`, one `When`, one `Then`; extras
> chain with `And`/`But`. The Phase 8 target name is still `validate:mermaid` (the rename to
> `mermaid:validation` is Phase 10); the underlying `docs validate-mermaid` CLI command is unchanged at
> Phase 8 (Phase 9 later regroups it to `md validate mermaid`).

```gherkin
Feature: State diagram width validation

  Background:
    Given the validator default options use max_width 4 and max_label_len 30
    And state diagrams are in scope of validate-mermaid

  Scenario: Over-wide LR state chain is flagged width_exceeded
    Given a stateDiagram-v2 with "direction LR" and 11 sequential states
    When validate-mermaid parses the block
    Then a "width_exceeded" violation is reported for that block
    And the reported width is 11

  Scenario: Compliant narrow state chain passes
    Given a stateDiagram-v2 with "direction TB" and 3 sequential states
    When validate-mermaid parses the block
    Then no "width_exceeded" violation is reported for that block
```

```gherkin
Feature: State diagram label validation

  Background:
    Given the validator default options use max_label_len 30

  Scenario: A state display label over 30 characters is flagged
    Given a state declared as 'state "this label is far longer than thirty chars" as X'
    When validate-mermaid checks the state display label
    Then a "label_too_long" violation is reported for state X

  Scenario: A transition-edge label over 30 characters is flagged
    Given a transition "A --> B : this transition label exceeds thirty characters"
    When validate-mermaid checks the transition-edge label
    Then a "label_too_long" violation is reported for that edge

  Scenario: A short colon label passes
    Given a state declared as "Pending : awaiting input"
    When validate-mermaid checks the state display label
    Then no "label_too_long" violation is reported for that state
```

```gherkin
Feature: State diagram structure-to-node mapping

  Scenario: Pseudostates and stereotype states count as nodes
    Given a stateDiagram-v2 whose widest rank holds "[*]", a "<<choice>>" state, a "<<fork>>" state, and a "<<join>>" state plus one more
    When validate-mermaid computes rank width
    Then "[*]" and the stereotype states each count toward the rank width
    And a "width_exceeded" violation is reported because the rank holds 5 nodes

  Scenario: Composite state is treated as a subgraph
    Given a stateDiagram-v2 containing a composite "state Outer { Inner1 --> Inner2 }"
    When validate-mermaid parses the block
    Then the composite "Outer" is recorded as a subgraph
    And the subgraph-density warning applies to its inner contents
```

```gherkin
Feature: State diagram free text is not misparsed

  Scenario: Notes, comments and concurrency separators are skipped
    Given a stateDiagram-v2 containing a "note right of X ... end note", a "%% comment", and a "--" concurrency separator
    When validate-mermaid parses the block
    Then the note text is exempt from the label rule
    And the "%%" comment line produces no node
    But the "--" separator produces neither a node nor a transition
```

```gherkin
Feature: Flowchart behavior is preserved

  Scenario: Existing flowchart validation is unchanged
    Given the pre-existing flowchart unit test suite
    When the Mermaid slice gains the state front-end
    Then every pre-existing flowchart test still passes
    And no flowchart violation codes change
```

```gherkin
Feature: Legacy v1 state diagram header is recognized

  Scenario: stateDiagram v1 header is in scope
    Given a legacy "stateDiagram" (v1) block of 11 sequential states with "direction LR"
    When validate-mermaid parses the block
    Then a "width_exceeded" violation is reported
    But the "TD" direction value is rejected as invalid for state diagrams
```

## Product Scope

### In Scope

- **A** — `pr-quality-gate.yml` (add `specs-gate` job; add concurrency; add push-to-main full gate);
  workflow file/`name:`/job-id naming onto the BLOCK 1-A scheme across all workflows (`Quality gate`
  kept); `validate-markdown.yml` (concurrency; confirm the existing gherkin validator step);
  `validate-env.yml` + `test-crud-*.yml` (concurrency; `test-crud-*` schedulers stay weekly).
  Confirm-only: per-language `nx affected`, tool-named lint jobs, gherkin target. ose-primer carries
  **no image-publishing workflow** (recorded deviation).
- **B** — `.husky/commit-msg`, `.husky/pre-commit`, `.husky/pre-push` converge to BLOCK 1-B
  (pre-commit `test:quick` = format+lint+typecheck+test:unit; pre-push = `specs:coverage` +
  `test-coverage`).
- **H** — three-level test targets (`test:unit`/`test:integration`/`test:e2e`) in `apps/*/project.json`
  sharing `.feature` files; `test:unit` mocked at pre-commit; **integration/e2e CRON-only** via the
  `test-and-deploy-{app-group}-development.yml` heavy-test workflow at **1×/day** (ose-primer); `specs:coverage`
  extended to enforce all three levels; **NO staging workflow / NO staging container for ose-primer**.
- **C** — port `apps/rhino-cli/src/` to the full hexagonal layout (from primer's placeholder layout),
  golden-master-frozen, mirroring `ose-public`'s reference.
- **D** — **scope-based regroup** (`docs`→`md`, `agents`→`harness`, `java`→`lang`; fold
  `spec-coverage`/`ddd`/`contracts`/`gherkin`→`specs`; new `convention`; `docs` reserved) + uniform
  grammar; add the missing **`specs` structural set** to rhino-cli; fold `SpecCoverage` into `specs`.
- **E** — `{domain}:{work}` rename in `apps/rhino-cli/project.json` (incl.
  `env:validate`→`env:validation`); `spec-coverage`→`specs:coverage` in every app/lib `project.json`;
  update all callers.
- **F** — **create** `cross-language-lint-strictness.md`; update all other BLOCK 6 governance docs;
  run `repo-rules-maker`; run `repo-rules-quality-gate`.
- **G** — add the `state.rs` front-end to the migrated Mermaid slice (`stateDiagram-v2` +
  `stateDiagram` v1); width rule + label rule (state display labels AND transition labels); `[*]` /
  stereotype counting; composite-as-subgraph; `direction` ∈ `TB|BT|LR|RL`; mirror `ose-public`'s
  shared golden corpus byte-identical; aggressive repo-wide state-diagram cleanup (incl. `plans/done/`);
  document the rule in `diagrams.md` + `markdown.md`/`repository-validation.md`.
- `.claude/agents/ci-checker.md` / `repo-rules-*` edits if warranted.

### Out of Scope

- Converging the runner target (recorded deviation — ose-primer stays `ubuntu-latest`).
- Adding an image-publishing workflow (recorded deviation — ose-primer ships no images).
- The siblings' own A/B/E/F gaps; ose-primer **ports** ose-public's C/D/G reference in its own repo.
- New toolchain capabilities, deploy targets, or Nx Cloud changes beyond parity.

## Product-Level Risks

- **Hexagonal behavior drift** — mitigated by the golden-master CLI suite that byte-freezes the
  output surface at every feature group and phase gate.
- **Target-rename caller breakage** (incl. `env:validate`→`env:validation`) — mitigated by the Phase 10
  caller-sweep and the Phase 6/10 sequencing so hooks never reference an unrenamed target between phases.
- **`ose-public` C/D/G reference not yet landed** — ose-primer's C/D/G phases port from it; they run
  only after the reference is done, while A/B/E/F proceed independently.
- **Preexisting Gherkin cardinality violations** — fixed in-plan (root-cause orientation), not waived.
- **Concurrency over-cancellation** — mitigated by the canonical group key that only cancels on PR
  events.
- **State arrow vs concurrency separator (G)** — `-->` could be matched after the `--` concurrency
  separator, mis-classifying transitions; mitigated by matching `-->` BEFORE `--` (pinned grammar
  fact) with a golden fixture covering a `--` separator inside a composite.
- **State note free-text misparse (G)** — a note's free text could be parsed as a state, producing a
  false `label_too_long`; mitigated by a fixture with a long multiline note that must produce zero
  violations.
- **Branch-protection required-check rename (A)** — renaming a required-check job (e.g. `Quality gate`)
  silently breaks the merge gate because GitHub keys required checks by job name; mitigated by keeping
  `Quality gate` unchanged and pairing any required-check rename with a `[HUMAN]` branch-protection
  settings update (Phase 1) before the gate is relied upon.
