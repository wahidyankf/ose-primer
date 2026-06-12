---
title: "Delivery: Standardize CI Parity (ose-primer sibling)"
description: "Phased execution checklist — Phases 0–5 converging ose-primer CI to the shared Converged CI Target of the three-repo sibling set."
---

# Delivery Checklist — Standardize CI Parity (ose-primer sibling)

> **Parallel-safe**: this plan is **independently runnable** and **parallel-safe** with both sibling
> plans (`ose-public`, `ose-infra`). It depends on no other plan — the Converged CI Target is a static
> spec (see [tech-docs.md](./tech-docs.md#converged-ci-target-shared-across-the-three-repo-sibling-set)).
> Start it whenever; nothing gates it on a sibling.
>
> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret handling).
> `[HUMAN → AI]`: a human performs a gated action, then hands control back to the agent on an
> observable signal.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (safe-to-stop state + resume command). A phase is not complete until its
> gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Worktree path: **`worktrees/standardize-ci-parity/`** (ose-primer's gitignored worktree path per
[Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md),
mirroring ose-public). Provision with `claude --worktree standardize-ci-parity` (or
`git worktree add worktrees/standardize-ci-parity`), then **`npm install` AND
`npm run doctor -- --fix`** inside the worktree before any work (worktree toolchain init). All work
happens on `main` via the worktree; merge to `main` by **direct push — no PR** (Trunk Based
Development). ose-primer normally receives upstream content via propagation PRs, but this parity plan
executes directly in primer.

See [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification)
for the declared-path requirement this section satisfies.

---

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.
> This follows the root cause orientation principle — proactively fix preexisting errors encountered
> during work. Commit preexisting fixes separately with appropriate conventional commit messages.

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

**Parallel-safe note**: this is the ose-primer sibling of the three-repo `standardize-ci-parity` set.
It converges to a **static** Converged CI Target and depends on **no** sibling plan — proceed without
blocking on `ose-public` or `ose-infra`. The shared target is recorded verbatim in
[tech-docs.md](./tech-docs.md#converged-ci-target-shared-across-the-three-repo-sibling-set).

- [ ] [AI] Provision and enter the worktree per `## Worktree`: `claude --worktree standardize-ci-parity`
      (or `git worktree add worktrees/standardize-ci-parity`); confirm with
      `git rev-parse --abbrev-ref HEAD && git status --porcelain` — acceptance: execution root is
      `worktrees/standardize-ci-parity/`, branch `main`, tree clean.
- [ ] [AI] Install dependencies in the worktree: `npm install` — acceptance: exits 0,
      `node_modules/` synchronized.
- [ ] [AI] Converge the toolchain: `npm run doctor -- --fix` — acceptance: exits 0 with no
      unresolved drift.
- [ ] [AI] Verify `actionlint` is available (workflow linter used in every later phase): run
      `actionlint --version` — acceptance: exits 0 and prints a version. If missing, install via
      `brew install actionlint`.
- [ ] [AI] **Baseline the already-done dimensions (confirm only — no edits)**: record that
      `actions/checkout@v6` is on all references (`grep -rc 'actions/checkout@v4' .github/workflows/`
      = 0), `nx run-many` count in `pr-quality-gate.yml` is 0 (so `nx affected` confirmed),
      `validate:gherkin-keyword-cardinality` exists in `apps/rhino-cli/project.json`, 7
      `_reusable-*.yml` exist, the `shellcheck` / `hadolint` / `actionlint` jobs exist, and the
      `naming` job exists — acceptance: each recorded as "confirmed at target" in the implementation
      notes.
- [ ] [AI] **Baseline the gap dimensions**: record `grep -L 'concurrency:' .github/workflows/*.yml`
      (expect: all 23 files), the absence of a `specs-gate` job in `pr-quality-gate.yml`, the
      `test-crud-*` schedule lines (`grep -A2 'schedule:' .github/workflows/test-crud-*.yml`; expect:
      `cron: "0 10 * * 5"` weekly in all 15), and `grep -c 'CI Parity Checklist'
repo-governance/development/infra/ci-conventions.md` (expect: 0) — acceptance: gap baseline
      recorded.
- [ ] [AI] **specs-gate availability audit (drives Phase 2)**: check whether primer's rhino-cli
      defines `validate:specs-*` targets — run
      `grep -oE '"validate:specs[a-z-]*"' apps/rhino-cli/project.json | sort -u` — acceptance: the
      result (expected: none in primer) is recorded, and the Phase 2 path (2a port vs 2b
      wire-to-existing) is noted as pending the Phase 2 decision.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `git status --porcelain` is empty and branch is `main` in the worktree — expected: clean.
- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
- [ ] [AI] `actionlint --version` exits 0.
- [ ] [AI] Already-done dimensions recorded as confirmed; gap baseline recorded (concurrency 0/23,
      no specs-gate, weekly cron in 15 files, no CI Parity Checklist).
- [ ] [AI] specs-gate validator availability audited and recorded.

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no CI files
> changed yet. Safe to stop indefinitely. To resume: re-run
> `git status --porcelain && actionlint --version`.

---

## Phase 1: Concurrency — Add Canonical Block to ALL Workflows

Canonical block (D1): `group: ${{ github.workflow }}-${{ github.ref }}` and
`cancel-in-progress: ${{ github.event_name == 'pull_request' }}`. Add at top level, after
`permissions:`, in every workflow that runs jobs (all 23).

- [ ] [AI] **RED**: assert no workflow declares a concurrency block — run
      `grep -L 'concurrency:' .github/workflows/*.yml | wc -l` — acceptance: returns `23` (all files
      lack the block), proving the work is needed.
- [ ] [AI] **GREEN**: add the canonical `concurrency` block to every workflow file under
      `.github/workflows/` (the 1 PR gate, 7 `_reusable-*`, 15 `test-crud-*`, and
      `validate-markdown.yml`), each with the exact canonical group key + PR-only cancel expression —
      command: `grep -L 'concurrency:' .github/workflows/*.yml | wc -l` — acceptance: returns `0`
      (no file lacks the block).
  - _Suggested executor: ci-fixer_
- [ ] [AI] **GREEN**: confirm every block is canonical (not a hand-divergent variant) — command:
      `grep -c 'github.workflow }}-' .github/workflows/*.yml` — acceptance: every file shows ≥ 1
      match for the canonical group key.
- [ ] [AI] **REFACTOR**: run `actionlint .github/workflows/*.yml` — acceptance: exits 0 with no new
      findings introduced by the concurrency additions.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `grep -L 'concurrency:' .github/workflows/*.yml` lists no files — expected: empty.
- [ ] [AI] Every concurrency block uses the canonical group key + PR-only cancel expression — verify:
      `grep -c "github.event_name == 'pull_request'" .github/workflows/*.yml` shows a match per file.
- [ ] [AI] `actionlint .github/workflows/*.yml` exits 0.
- [ ] [AI] Commit created: `ci(workflows): add canonical concurrency to all workflows`.

> **Pause Safety**: all 23 workflows now share one concurrency expression and lint clean. Safe to
> stop. To resume: `grep -L 'concurrency:' .github/workflows/*.yml` (must be empty).

---

## Phase 2: Add specs-gate Job to pr-quality-gate.yml

specs-gate mirrors ose-public's job shape (D2): `runs-on: ubuntu-latest`, `setup-node` +
`setup-rust`, one `npx nx ...` validate step, added to the `quality-gate` aggregator `needs:`. The
**command** is resolved by the availability audit below.

- [ ] [AI] **Decide the specs-gate command (resolve the open question)**: from the Phase 0 audit,
      choose **path 2a** (port the `validate:specs-*` targets into `apps/rhino-cli/project.json` and
      wire the job to `npx nx run-many -t validate:specs-adoption validate:specs-tree
validate:specs-counts validate:specs-links --projects=rhino-cli`, matching ose-public) **or
      path 2b** (wire the job to primer's existing specs validation over the `specs/` tree, e.g. the
      gherkin `spec-coverage` target, and record target-porting as a follow-up) — acceptance: the
      chosen path and rationale are recorded in the implementation notes.
- [ ] [AI] **(Path 2a only) GREEN**: add the `validate:specs-*` targets to
      `apps/rhino-cli/project.json` and confirm each runs — command:
      `grep -oE '"validate:specs[a-z-]*"' apps/rhino-cli/project.json | sort -u` — acceptance: the
      four targets are present and `npx nx run rhino-cli:validate:specs-tree` (and siblings) exit 0.
- [ ] [AI] **RED**: assert no specs-gate job exists — run
      `grep -cE '^\s+specs-gate:' .github/workflows/pr-quality-gate.yml` — acceptance: returns `0`.
- [ ] [AI] **GREEN**: add a `specs-gate` job to `.github/workflows/pr-quality-gate.yml` on
      `ubuntu-latest` with `setup-node` + `setup-rust` running the command chosen above — command:
      `grep -A8 'specs-gate:' .github/workflows/pr-quality-gate.yml` — acceptance: shows the job
      running the chosen specs validator(s) on `ubuntu-latest`, mirroring ose-public's specs-gate
      shape.
  - _Suggested executor: ci-fixer_
- [ ] [AI] **GREEN**: wire `specs-gate` into the `quality-gate` aggregator's `needs:` array (and its
      `contains(needs.*.result, 'failure')` check covers it implicitly) — command:
      `grep -A20 'quality-gate:' .github/workflows/pr-quality-gate.yml | grep -c 'specs-gate'` —
      acceptance: returns ≥ 1 (specs-gate listed in `needs:`).
- [ ] [AI] **REFACTOR**: run `actionlint .github/workflows/pr-quality-gate.yml` — acceptance:
      exits 0.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `pr-quality-gate.yml` defines a `specs-gate` job — verify:
      `grep -c 'specs-gate:' .github/workflows/pr-quality-gate.yml` ≥ 1.
- [ ] [AI] `specs-gate` is listed in the `quality-gate` aggregator `needs:` — verify:
      `grep -A20 'quality-gate:' .github/workflows/pr-quality-gate.yml | grep -c 'specs-gate'` ≥ 1.
- [ ] [AI] The chosen specs validator command exists and runs (path 2a targets present, or path 2b
      target confirmed) — verify per the recorded path.
- [ ] [AI] `actionlint .github/workflows/pr-quality-gate.yml` exits 0.
- [ ] [AI] Commit created: `ci(pr-gate): add specs-gate governance job`.

> **Pause Safety**: the PR gate now covers specs governance and lints clean; the `naming` job was
> already present (confirmed Phase 0). Safe to stop. To resume:
> `grep -c 'specs-gate:' .github/workflows/pr-quality-gate.yml`.

---

## Phase 3: Align Scheduled Cadence to Twice-Daily WIB

Baseline finding (Phase 0): all 15 `test-crud-*` workflows run **weekly** on `cron: "0 10 * * 5"`,
which **diverges** from the Converged CI Target. Target crons (D3): 06:00 WIB = `0 23 * * *`
(previous-day UTC) and 18:00 WIB = `0 11 * * *`.

- [ ] [AI] **RED**: assert the weekly single-cron cadence across all 15 files — run
      `grep -A2 'schedule:' .github/workflows/test-crud-*.yml | grep -c '0 10 \* \* 5'` — acceptance:
      returns `15` (every `test-crud-*` on the weekly cron), proving the alignment is needed.
- [ ] [AI] **GREEN**: in each of the 15 `.github/workflows/test-crud-*.yml`, replace the single weekly
      `cron: "0 10 * * 5"` with the two WIB-aligned crons `"0 23 * * *"` (06:00 WIB) and
      `"0 11 * * *"` (18:00 WIB) — command:
      `grep -c 'cron:' .github/workflows/test-crud-*.yml` — acceptance: each file shows `2` cron
      lines, and `grep -c '0 10 \* \* 5' .github/workflows/test-crud-*.yml` returns 0 across the set.
  - _Suggested executor: ci-fixer_
- [ ] [AI] **REFACTOR**: run `actionlint .github/workflows/test-crud-*.yml` — acceptance: exits 0.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] Every `test-crud-*` workflow declares exactly two WIB-aligned crons — verify:
      `for f in .github/workflows/test-crud-*.yml; do grep -c 'cron:' "$f"; done` shows `2` for each.
- [ ] [AI] No `0 10 * * 5` weekly cron remains — verify:
      `grep -rc '0 10 \* \* 5' .github/workflows/test-crud-*.yml` totals 0.
- [ ] [AI] `actionlint .github/workflows/test-crud-*.yml` exits 0.
- [ ] [AI] Commit created: `ci(test-crud): align schedules to twice-daily WIB cadence`.

> **Pause Safety**: all 15 scheduled workflows now match the documented cadence and lint clean. Safe
> to stop. To resume: `for f in .github/workflows/test-crud-*.yml; do grep -c 'cron:' "$f"; done`.

---

## Phase 4: Governance — ci-conventions Alignment + CI Parity Checklist + ci-checker

Non-code carve-out: governance-doc and agent-definition edits use DIRECT-ACTION + acceptance (no
RED/GREEN/REFACTOR), per `repo-governance/development/workflow/test-driven-development.md`.

- [ ] [AI] Align `repo-governance/development/infra/ci-conventions.md` to the Converged CI Target:
      reconcile any wording the Phase 1–3 changes affect (concurrency now on every workflow,
      specs-gate now present, cadence now twice-daily WIB) so the doc and the workflows agree —
      acceptance: no statement in the doc contradicts the actual workflow files.
  - _Suggested executor: repo-rules-maker_
- [ ] [AI] Add a `## CI Parity Checklist` section to
      `repo-governance/development/infra/ci-conventions.md` enumerating the Converged CI Target
      invariants (checkout@v6; non-TS `nx affected`; gherkin target wired; reusable-workflow pattern;
      canonical concurrency on every workflow; tool-named lint jobs; `naming` + `specs-gate`; 2× WIB
      cadence) and recording primer's deviations (full polyglot matrix). Reference primer's own lint
      governance generically — do **not** cross-link the public-only `cross-language-lint-strictness.md`
      (it does not exist in primer) — command:
      `grep -c 'CI Parity Checklist' repo-governance/development/infra/ci-conventions.md` —
      acceptance: returns ≥ 1 and the section enumerates the parity criteria.
  - _Suggested executor: repo-rules-maker_
- [ ] [AI] Evaluate `.claude/agents/ci-checker.md` for parity-check additions (e.g. asserting the CI
      Parity Checklist criteria during CI audits); apply edits only if they add value — acceptance:
      either `ci-checker.md` gains explicit parity-check guidance, or the implementation notes record
      an explicit "no change needed" decision with rationale.
  - _Suggested executor: repo-rules-maker_
- [ ] [AI] **If and only if `.claude/agents/ci-checker.md` changed**, re-sync platform bindings:
      `npm run generate:bindings` — command: `git status --porcelain .opencode/agents/ci-checker.md`
      — acceptance: the generated OpenCode (and any other) binding reflects the updated definition (or
      the command is a recorded no-op if `ci-checker.md` was unchanged).
- [ ] [AI] Run the doc gates this plan touches on the governance doc:
      `npx nx run rhino-cli:validate:links && npx nx run rhino-cli:validate:heading-hierarchy && npx nx run rhino-cli:validate:mermaid`
      — acceptance: all exit 0.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `ci-conventions.md` contains a `## CI Parity Checklist` section — verify:
      `grep -c 'CI Parity Checklist' repo-governance/development/infra/ci-conventions.md` ≥ 1.
- [ ] [AI] `ci-conventions.md` no longer contradicts the workflows (concurrency, specs-gate, cadence)
      — verify by re-reading the changed sections.
- [ ] [AI] If `ci-checker.md` changed, its generated binding is re-synced — verify:
      `git status --porcelain` shows no un-synced binding drift.
- [ ] [AI] `validate:links`, `validate:heading-hierarchy`, `validate:mermaid` exit 0.
- [ ] [AI] Commit(s) created: `docs(ci): align ci-conventions + add CI Parity Checklist`
      (and `chore(agents): re-sync ci-checker bindings` if applicable).

> **Pause Safety**: governance now matches the converged CI surface and bindings are in sync. Safe to
> stop. To resume: `grep -c 'CI Parity Checklist' repo-governance/development/infra/ci-conventions.md`.

---

## Phase 5: Final Quality Gate, Push, CI Verify, Archival

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck` — acceptance: exits 0.
- [ ] [AI] Run affected lint: `npx nx affected -t lint` — acceptance: exits 0.
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick` — acceptance: exits 0.
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage` — acceptance: exits 0.
- [ ] [AI] Lint all changed workflows: `actionlint .github/workflows/*.yml` — acceptance: exits 0.
- [ ] [AI] Run the plan's own doc gates:
      `npx nx run rhino-cli:validate:links && npx nx run rhino-cli:validate:mermaid && npx nx run rhino-cli:validate:heading-hierarchy && npx nx run rhino-cli:validate:gherkin-keyword-cardinality`
      — acceptance: all exit 0 (relative links from this 3-deep plan dir use `../../../` to repo
      root; mermaid node labels ≤ 30 chars with the color-blind palette).
- [ ] [AI] Confirm no secrets in any committed file: `git diff --cached | grep -iE 'secret|token|password|key' || true`
      — acceptance: any matches are `${{ secrets.NAME }}` references or doc prose, never real values.
- [ ] [AI] Fix ALL failures found — including preexisting issues not caused by this plan — and re-run
      until zero failures.

### Commit Guidelines

- [ ] [AI] Commit thematically (one phase = one+ cohesive commit), Conventional Commits format
      `<type>(<scope>): <description>`, preexisting fixes in separate commits.

### Post-Push CI Verification

- [ ] [AI] Merge the worktree branch to `main` and push: `git push origin main` — acceptance: push
      succeeds (no PR; Trunk Based Development).
- [ ] [AI] Monitor triggered GitHub Actions: poll **every 3 minutes** via
      `gh run list --branch main --limit 10` then `gh run view <id> --json status,conclusion` —
      acceptance: all triggered workflows reach `conclusion: success`. Do **NOT** use `gh run watch`.
- [ ] [AI] If any CI check fails, fix the root cause, push a follow-up commit, and repeat until all
      GitHub Actions are green.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked and ALL phase gates are green.
- [ ] [AI] Verify ALL quality gates pass (local + CI).
- [ ] [AI] Move the plan to done using **today's completion date** (date-agnostic — do not hardcode):
      `git mv plans/in-progress/standardize-ci-parity plans/done/$(date +%F)__standardize-ci-parity`
      — acceptance: the folder now lives under `plans/done/<completion-date>__standardize-ci-parity/`.
- [ ] [AI] Update `plans/in-progress/README.md` — remove the `standardize-ci-parity` entry.
- [ ] [AI] Update `plans/done/README.md` — add the entry with the completion date.
- [ ] [AI] Update any other READMEs referencing this plan (e.g. `plans/README.md`).
- [ ] [AI] Commit the archival: `chore(plans): move standardize-ci-parity to done`.
- [ ] [AI] Push the archival commit and verify CI green.
- [ ] [AI] Remove the worktree once merged and archived: `git worktree remove worktrees/standardize-ci-parity`.

### Phase 5 Gate

> Final gate — the plan is complete only when every check below is green.

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` exits 0.
- [ ] [AI] `actionlint .github/workflows/*.yml` exits 0.
- [ ] [AI] All four plan doc gates exit 0 on the plan files.
- [ ] [AI] All triggered GitHub Actions on `main` report `conclusion: success`.
- [ ] [AI] The plan folder lives under `plans/done/<completion-date>__standardize-ci-parity/` and the
      in-progress/done READMEs are updated.

> **Pause Safety**: the convergence is complete, pushed, CI-green, and archived. The repo is at a
> fully coherent terminal state. To resume (if re-verifying): `gh run list --branch main --limit 5`
> and confirm the latest runs are green.

---

## Validation Checklist

- [ ] [AI] All config (YAML/JSON) phases followed the RED→GREEN→REFACTOR shape (grep/actionlint RED →
      edit GREEN → cleanup REFACTOR).
- [ ] [AI] All governance-doc / agent-definition edits used DIRECT-ACTION + acceptance (non-code carve-out).
- [ ] [AI] Every phase ended with a green `### Phase N Gate` and a Pause Safety note.
- [ ] [AI] The already-at-target dimensions (checkout@v6, nx affected, gherkin target, reusable
      workflows, tool-named lint jobs, naming job, ubuntu runner) were confirmed, not re-edited.
- [ ] [AI] The accepted deviation (full polyglot matrix) remains intact — not converged.
- [ ] [AI] Acceptance criteria in [prd.md](./prd.md) all verified.
      </content>
