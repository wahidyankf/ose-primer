# Delivery Checklist — lint-safety-parity (ose-primer)

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.
>
> **Delivery mode**: `main-to-main` — commits push directly to `origin main` (Trunk Based
> Development). For ose-primer this deviates from the ose-primer Sync Convention's PR-only default
> (**M1**); the deviation is explicitly approved by the invoker and recorded in
> [tech-docs.md §M1](./tech-docs.md#m1--ose-primer-sync-convention-deviation). **Do NOT open a PR.**

## Worktree

Worktree path: `worktrees/lint-safety-parity/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree lint-safety-parity
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [x] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized.
  - _Date_: 2026-06-12. _Status_: DONE (repo-setup-manager). _Files Changed_: none. _Notes_: `npm install` exited 0, node_modules synchronized in worktree.
- [x] [AI] Converge the toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift.
  - _Date_: 2026-06-12. _Status_: DONE (repo-setup-manager). _Files Changed_: none. _Notes_: `npm run doctor -- --fix` exited 0; all 19 polyglot tools present, no drift.
- [x] [AI] Install the new linter binaries available to CI/local: `hadolint`, `shellcheck`,
      `actionlint` (record install method per tool) — acceptance: `hadolint --version`,
      `shellcheck --version`, `actionlint --version` each exit 0.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: none (binaries already installed via Homebrew).
    _Notes_: All three present — `hadolint` 2.14.0, `shellcheck` 0.11.0, `actionlint` 1.7.12; each `--version` exits 0. Install method: Homebrew (pre-existing on PATH).\_
- [x] [AI] Record the affected-project baseline:
      `npx nx affected -t typecheck lint test:quick spec-coverage`
      — acceptance: baseline pass/fail recorded; every preexisting failure documented.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: none (measurement). _Notes_: Ran `npx nx run-many -t typecheck lint test:quick spec-coverage --all` — "Successfully ran for 26 projects". One real preexisting failure found: `clojure-openapi-codegen:build` (AOT compile aborted, missing `classes/` dir) — resolved separately (see next item). The setup agent's initially-reported "5 Elixir + flaky Rust/TS" failures were **phantom** (re-running each project's `test:quick` directly returned exit 0; the agent's failure list did not reproduce). Nx's post-run "flaky tasks" notice is historical detection, not current failures.\_
- [x] [AI] Re-derive exact file lists for D6/D7/D8 (Dockerfiles, shell scripts, workflows) excluding
      `node_modules/`, `.venv/`, `target/`, `deps/`, `archived/` — acceptance: three lists written
      into the plan working notes.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: none (derivation only).
    _Notes_: **D6 Dockerfiles (30)** — 18 under `apps/*` (`Dockerfile`, `Dockerfile.integration`) + 12 under `infra/dev/*` (`Dockerfile.be.dev`/`.ci`). **D7 shell (14)** — `.claude/hooks/*.sh` (4), `scripts/*.sh` (6: check-no-env-staged, format-clojure, format-csharp, format-dart, format-elixir, git-identity-check), `apps/crud-fe-dart-flutterweb/nginx/entrypoint.sh`, `apps/rhino-cli/scripts/validate-cross-vendor-parity.sh`, `.husky/{pre-commit,pre-push,commit-msg}`. **D8 workflows (24)** — all `.github/workflows/*.yml` (7 `_reusable-*`, `pr-quality-gate`, `validate-markdown`, 15 `test-crud-*`).\_
- [x] [AI] Confirm `apps/crud-be-rust-axum/src` contains no handwritten `unsafe`:
      `grep -rn "unsafe" apps/crud-be-rust-axum/src` — acceptance: no matches.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: none (verification only).
    _Notes_: `grep -rn "unsafe" apps/crud-be-rust-axum/src` returned no matches (exit 1). Confirms `forbid(unsafe_code)` needs no D1b test refactor here.\_
- [x] [AI] Resolve all preexisting failures before proceeding — acceptance: none remain unresolved.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `libs/clojure-openapi-codegen/project.json`, `libs/clojure-openapi-codegen/.gitignore` (new). _Notes_: Fixed `clojure-openapi-codegen:build` — `(compile ...)` requires its `*compile-path*` (`classes/`) to exist; prepended `mkdir -p classes &&` to the build command, added `"outputs": ["{projectRoot}/classes"]`, and gitignored `classes/`. Verified `nx run clojure-openapi-codegen:build --skip-nx-cache` exits 0. This is in scope because Phase 5 edits `apps/rhino-cli/project.json` and ~20 projects (incl. this lib) carry `rhino-cli` as an implicit dependency, so the final `nx affected` fans out to the whole workspace. Committed separately as a preexisting fix. Full workspace baseline now green.\_

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [x] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: Both exited 0; 19 tools converged, no drift.
- [x] [AI] `hadolint`, `shellcheck`, `actionlint` all resolve on PATH.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: hadolint 2.14.0, shellcheck 0.11.0, actionlint 1.7.12 — all on PATH (Homebrew).
- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` baseline recorded; zero
      unresolved preexisting failures.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: Full-workspace `run-many` succeeded for all 26 projects after the clojure-openapi-codegen build fix. Zero unresolved failures.\_

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no feature
> work exists yet. Safe to stop indefinitely. To resume: re-run the baseline command and confirm it
> is still clean.

---

## Phase 1: D1 Rust — `forbid(unsafe_code)` + full `[lints]` standard

> _Executor: swe-rust-dev_

- [ ] [AI] **RED**: Add the verbatim public `[lints.rust]` + `[lints.clippy]` standard (see
      [tech-docs.md §D1](./tech-docs.md#d1--rust--target--publics-existing-crates-verbatim)) to
      `apps/crud-be-rust-axum/Cargo.toml`, then run `npx nx run crud-be-rust-axum:lint`
      — acceptance: lint **fails** on latent pedantic/nursery/`unwrap_used` violations (the failing
      gate is the RED test). If it already passes, record that and skip to REFACTOR.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: Clean all reported clippy violations in `apps/crud-be-rust-axum/src` (and tests)
      until `npx nx run crud-be-rust-axum:lint` exits 0 — acceptance: lint passes, no other tests
      broken (`npx nx run crud-be-rust-axum:test:quick` still passes).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: Update the `lint` target in `apps/crud-be-rust-axum/project.json` from
      `cargo clippy -- -D warnings` to `cargo clippy --all-targets -- -D warnings`
      — acceptance: `npx nx run crud-be-rust-axum:lint` exits 0 with the escalated command.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED (safety proof)**: Temporarily add `unsafe { /* noop */ }` to a function in
      `apps/crud-be-rust-axum/src` and run `npx nx run crud-be-rust-axum:typecheck`
      — acceptance: build **fails** with the forbidden-unsafe error.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN (safety proof)**: Remove the temporary `unsafe` block; re-run
      `npx nx run crud-be-rust-axum:lint` — acceptance: green build restored.
  - _Suggested executor: `swe-rust-dev`_

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `apps/crud-be-rust-axum/Cargo.toml` contains `unsafe_code = "forbid"` under `[lints.rust]`.
- [ ] [AI] `npx nx run crud-be-rust-axum:lint` exits 0 (with `--all-targets`).
- [ ] [AI] `npx nx run crud-be-rust-axum:test:quick` exits 0.

> **Pause Safety**: Rust crate is clean and forbids unsafe; no other dimension touched. Safe to stop.
> To resume: `npx nx run crud-be-rust-axum:lint`.

---

## Phase 2: D3 C# — `AnalysisLevel=latest-All` + Sonar enforced

> _Executor: swe-csharp-dev_

- [ ] [AI] **RED**: Add `<AnalysisLevel>latest-All</AnalysisLevel>` to
      `apps/crud-be-csharp-aspnetcore/Directory.Build.props` and raise `SonarAnalyzer.CSharp` rule
      severities to error in `apps/crud-be-csharp-aspnetcore/.editorconfig`, then run
      `npx nx run crud-be-csharp-aspnetcore:lint`
      — acceptance: lint **fails** on the `latest-All`/Sonar backlog (RED).
  - _Suggested executor: `swe-csharp-dev`_
- [ ] [AI] **GREEN**: Clean all reported analyzer violations across
      `apps/crud-be-csharp-aspnetcore/src` and `tests` until
      `npx nx run crud-be-csharp-aspnetcore:lint` exits 0 — acceptance: lint passes;
      `npx nx run crud-be-csharp-aspnetcore:test:quick` still passes.
  - _Suggested executor: `swe-csharp-dev`_
- [ ] [AI] **REFACTOR**: Verify the `lint`/`typecheck` targets keep
      `/p:TreatWarningsAsErrors=true` so the gate stays enforced — acceptance:
      `npx nx run crud-be-csharp-aspnetcore:typecheck` exits 0.
  - _Suggested executor: `swe-csharp-dev`_
- [ ] [AI] **RED (proof)**: Temporarily introduce a Sonar-flagged construct (e.g. an unused private
      field) in `src` and run `npx nx run crud-be-csharp-aspnetcore:lint`
      — acceptance: build **fails** on the Sonar diagnostic.
  - _Suggested executor: `swe-csharp-dev`_
- [ ] [AI] **GREEN (proof)**: Remove the construct; re-run lint — acceptance: green build restored.
  - _Suggested executor: `swe-csharp-dev`_

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `Directory.Build.props` contains `<AnalysisLevel>latest-All</AnalysisLevel>`.
- [ ] [AI] `npx nx run crud-be-csharp-aspnetcore:lint` and `:test:quick` exit 0.

> **Pause Safety**: C# strict gate enabled and clean. Safe to stop. To resume:
> `npx nx run crud-be-csharp-aspnetcore:lint`.

---

## Phase 3: D4 Python — basedpyright strict + expanded ruff

> _Executor: default per Agent Selection (no Python-specialist agent)_

- [ ] [AI] **RED (types)**: In `apps/crud-be-python-fastapi/pyproject.toml`, replace the
      `pyright==1.1.408` dev dependency with `basedpyright`, rename `[tool.pyright]` →
      `[tool.basedpyright]`, and set `typeCheckingMode = "strict"`; then run
      `npx nx run crud-be-python-fastapi:typecheck`
      — acceptance: typecheck **fails** on latent strict-mode type errors (RED).
- [ ] [AI] **GREEN (types)**: Add type annotations / fixes across
      `apps/crud-be-python-fastapi/src` and `tests` until
      `npx nx run crud-be-python-fastapi:typecheck` exits 0 — acceptance: typecheck passes.
- [ ] [AI] **RED (lint)**: Expand the ruff `select` in `[tool.ruff.lint]` to
      `E,W,F,B,UP,SIM,I,N,S,RUF,C4,T20,ANN`, EXCLUDE `ANN101`/`ANN102`, and add
      `[tool.ruff.lint.per-file-ignores]` exempting tests from `S101` and `ANN`; run
      `npx nx run crud-be-python-fastapi:lint`
      — acceptance: lint **fails** on the expanded-select backlog (RED).
- [ ] [AI] **GREEN (lint)**: Clean all reported ruff violations until
      `npx nx run crud-be-python-fastapi:lint` exits 0 — acceptance: lint passes;
      `npx nx run crud-be-python-fastapi:test:quick` still passes.
- [ ] [AI] **REFACTOR**: Update any project.json/CI references from `pyright` to `basedpyright` so the
      gate invokes the new type-checker — acceptance: `npx nx run crud-be-python-fastapi:typecheck`
      runs basedpyright and exits 0.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `pyproject.toml` uses `basedpyright` with `typeCheckingMode = "strict"` and the expanded
      ruff select set (no `ANN101`/`ANN102`).
- [ ] [AI] `npx nx run crud-be-python-fastapi:lint`, `:typecheck`, and `:test:quick` exit 0.

> **Pause Safety**: Python strict gate enabled and clean. Safe to stop. To resume:
> `npx nx run crud-be-python-fastapi:typecheck`.

---

## Phase 4: D6/D7/D8 — Dockerfile, shell, and Actions lint (configs + cleanup)

> _Executor: default per Agent Selection (config + shell work)_

- [ ] [AI] **RED (Docker)**: Create `.hadolint.yaml` (`failure-threshold: warning`,
      `trustedRegistries: [docker.io, ghcr.io]`, justified `ignore` per
      [tech-docs.md §D6](./tech-docs.md#d6--dockerfile-all-3)); run
      `hadolint --failure-threshold warning` over every Dockerfile from the Phase 0 list
      — acceptance: hadolint **fails** on the existing backlog (RED).
- [ ] [AI] **GREEN (Docker)**: Fix all hadolint warning-and-above findings (or add justified
      per-rule `ignore`) until the hadolint run exits 0 — acceptance: clean hadolint run.
- [ ] [AI] **RED (shell)**: Create `.shellcheckrc` (`shell=bash`, `external-sources=true`, justified
      disables per [tech-docs.md §D7](./tech-docs.md#d7--shell-all-3)); run
      `shellcheck --severity=warning` over the Phase 0 shell-script list
      — acceptance: shellcheck **fails** on the existing backlog (RED).
- [ ] [AI] **GREEN (shell)**: Fix all shellcheck findings (or add justified disables) until the run
      exits 0 — acceptance: clean shellcheck run.
- [ ] [AI] **RED (Actions)**: Add optional `.github/actionlint.yaml`; run `actionlint` over
      `.github/workflows/` — acceptance: actionlint **fails** on any existing findings (RED). If
      already clean, record that.
- [ ] [AI] **GREEN (Actions)**: Fix all actionlint findings until the run exits 0 — acceptance: clean
      actionlint run.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `.hadolint.yaml`, `.shellcheckrc` exist; `.github/actionlint.yaml` exists if needed.
- [ ] [AI] `hadolint --failure-threshold warning <all Dockerfiles>` exits 0.
- [ ] [AI] `shellcheck --severity=warning <all scripts>` exits 0.
- [ ] [AI] `actionlint` over `.github/workflows/` exits 0.

> **Pause Safety**: infra-file configs exist and the repo is clean against them, but the gates are
> not yet wired into Nx/CI/hooks. Safe to stop. To resume: re-run the three lint commands above.

---

## Phase 5: Flip gates ON — Nx targets, CI jobs, local hooks (clean-then-gate)

> _Executor: default per Agent Selection_

- [ ] [AI] **REFACTOR (Nx)**: Add `lint:dockerfiles`, `lint:shell`, `lint:actions` targets to
      `apps/rhino-cli/project.json` (model on existing `validate:*` run-commands targets), each
      invoking its tool at warning-threshold — acceptance: each `npx nx run rhino-cli:lint:<x>`
      exits 0.
- [ ] [AI] **RED (CI)**: Add `hadolint`, `shellcheck`, `actionlint` jobs to
      `.github/workflows/pr-quality-gate.yml` and add them to the `quality-gate` job's `needs` list;
      inject a deliberately-bad fixture (e.g. a `RUN apt-get install vim` line in any Dockerfile),
      then push a throwaway branch to trigger CI:
      `bash
PROBE="ci-probe-$(date +%s)"
git push origin HEAD:refs/heads/$PROBE
`
      Monitor with `gh run list --branch $PROBE --limit 5` (poll every 3 min; do NOT use
      `gh run watch`) — acceptance: the new CI job **fails** on the fixture (RED proof).
      Cleanup: `git push origin --delete refs/heads/$PROBE` (run this whether the RED proof
      succeeded or failed — the branch must not persist).
- [ ] [AI] **GREEN (CI)**: Remove the fixture — acceptance: the new CI jobs pass on clean `main`.
- [ ] [AI] **REFACTOR (hooks)**: Wire the three infra-lint gates into `.husky/pre-commit` and/or
      `.husky/pre-push` (scoped to changed files where the tool supports it), matching the existing
      `npm run lint:md` pattern — acceptance: a committed violation is rejected by the hook; a clean
      commit passes.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `npx nx run rhino-cli:lint:dockerfiles`, `:lint:shell`, `:lint:actions` each exit 0.
- [ ] [AI] `pr-quality-gate.yml` lists the three new jobs in `quality-gate` `needs`.
- [ ] [AI] A deliberately-bad Docker/shell/workflow change is rejected by the local hook (verified
      once, then reverted).

> **Pause Safety**: all six dimensions' gates are enabled and green in Nx, CI, and hooks. Safe to
> stop. To resume: `npx nx affected -t lint` + the three `rhino-cli:lint:*` targets.

---

## Phase 6: Documentation — rationale doc + governance/convention updates

> _Executor: docs-maker / repo-rules-maker_

- [ ] [AI] Write `docs/explanation/lint-safety-parity-decisions.md` covering: plain-language
      rationale for **every** deviation-matrix row (D1, D3, D4, D6, D7, D8 and why D1b/D2/D5/D9/D10
      are skipped), the **D5 deferral + exemption philosophy** (DDD enforcement targets
      business-domain backends only; demo/content/frontend apps exempt), and the **M1 main-to-main
      sync deviation + justification** — acceptance: file exists; `npm run lint:md` passes; all
      matrix rows and M1 are present. [Repo-grounded: `docs/explanation/` already exists.]
  - _Suggested executor: `docs-maker`_
- [ ] [AI] Register the new doc in `docs/explanation/README.md` — acceptance: the index links the new
      file; `npx nx run rhino-cli:validate:links` passes.
- [ ] [AI] Update `repo-governance/development/infra/nx-targets.md` to document the new
      `lint:dockerfiles`/`lint:shell`/`lint:actions` targets and the warning-threshold gating policy
      — acceptance: the targets appear in the canonical target list; `npm run lint:md` passes.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Update the shared cross-language strictness standard / Quality-Gates governance surface to
      reference the enforced gates (the convention doc documenting the shared strictness standard)
      — acceptance: the strictness standard names hadolint/shellcheck/actionlint + Rust forbid +
      C#/Python strict gates; `npm run lint:md` passes.
  - _Suggested executor: `repo-rules-maker`_

### Phase 6 Gate

> All checks below must pass before archiving the plan.

- [ ] [AI] `docs/explanation/lint-safety-parity-decisions.md` exists, covers all matrix rows, the D5
      deferral, and the M1 deviation.
- [ ] [AI] `npx nx run rhino-cli:validate:links` and `npm run lint:md` exit 0.

> **Pause Safety**: documentation and governance reflect the enforced gates. Safe to stop. To
> resume: `npm run lint:md`.

---

## Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck`
- [ ] [AI] Run affected linting: `npx nx affected -t lint`
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick`
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage`
- [ ] [AI] Run the three infra-lint targets: `npx nx run rhino-cli:lint:dockerfiles`,
      `:lint:shell`, `:lint:actions`
- [ ] [AI] Run markdown lint: `npm run lint:md`
- [ ] [AI] Fix ALL failures found — including preexisting issues not caused by these changes
- [ ] [AI] Verify zero failures before pushing

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.
> This follows the root cause orientation principle — proactively fix preexisting errors encountered
> during work. Commit preexisting fixes separately with appropriate conventional commit messages.

## Post-Push CI Verification

- [ ] [AI] Push changes to `main` (main-to-main; **no PR** — see M1)
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every 3 min; do NOT use
      `gh run watch`)
- [ ] [AI] Verify ALL CI checks pass — no exceptions
- [ ] [AI] If any CI check fails, fix immediately and push a follow-up commit
- [ ] [AI] Repeat until ALL GitHub Actions pass with zero failures
- [ ] [AI] Do NOT proceed to archival until CI is fully green

## Commit Guidelines

- [ ] [AI] Commit changes thematically — group related changes into logically cohesive commits
- [ ] [AI] Follow Conventional Commits format: `<type>(<scope>): <description>`
- [ ] [AI] Split different domains/concerns into separate commits (one per dimension where practical)
- [ ] [AI] Preexisting fixes get their own commits, separate from plan work
- [ ] [AI] Do NOT bundle unrelated changes into a single commit

## Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked
- [ ] [AI] Verify ALL quality gates pass (local + CI)
- [ ] [AI] Rename and move:
      `git mv plans/in-progress/lint-safety-parity/ plans/done/YYYY-MM-DD__lint-safety-parity/`
      using today's date as the completion date (NOT the creation date)
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date
- [ ] [AI] Update any other READMEs that reference this plan (e.g. `plans/README.md`)
- [ ] [AI] Commit the archival: `chore(plans): move lint-safety-parity to done`
