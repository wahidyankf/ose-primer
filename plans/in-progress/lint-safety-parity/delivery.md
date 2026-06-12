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

- [x] [AI] **RED**: Add the verbatim public `[lints.rust]` + `[lints.clippy]` standard (see
      [tech-docs.md §D1](./tech-docs.md#d1--rust--target--publics-existing-crates-verbatim)) to
      `apps/crud-be-rust-axum/Cargo.toml`, then run `npx nx run crud-be-rust-axum:lint`
      — acceptance: lint **fails** on latent pedantic/nursery/`unwrap_used` violations (the failing
      gate is the RED test). If it already passes, record that and skip to REFACTOR.
  - _Suggested executor: `swe-rust-dev`_
  - _Date_: 2026-06-12. _Status_: DONE (RED confirmed). _Files Changed_: `apps/crud-be-rust-axum/Cargo.toml`. _Notes_: Appended the verbatim `[lints.rust]`+`[lints.clippy]` block; lint **failed** with 121 latent violations (use_self 40, needless_raw_string_hashes 20, casts ~27, map_unwrap_or 8, missing_const_for_fn 7, doc_markdown 6, expect_used 4, …).
- [x] [AI] **GREEN**: Clean all reported clippy violations in `apps/crud-be-rust-axum/src` (and tests)
      until `npx nx run crud-be-rust-axum:lint` exits 0 — acceptance: lint passes, no other tests
      broken (`npx nx run crud-be-rust-axum:test:quick` still passes).
  - _Suggested executor: `swe-rust-dev`_
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: production `src/**` (real fixes: `main.rs` `.expect()`→`.context()?`; `auth/jwt.rs`, `auth/middleware.rs`, `handlers/{admin,attachment,token}.rs`, `db/{expense,user}_repo.rs`, `domain/expense.rs` — `try_from`/documented helpers for casts; plus `cargo clippy --fix` mechanical fixes across `auth/password.rs`, `config.rs`, `db/*`, `domain/*`, `repositories/*`, `state.rs`, `handlers/test_api.rs`). _Notes_: lint exits 0; `test:quick` still green (independently re-verified: 92.19% coverage).
- [x] [AI] **REFACTOR**: Update the `lint` target in `apps/crud-be-rust-axum/project.json` from
      `cargo clippy -- -D warnings` to `cargo clippy --all-targets -- -D warnings`
      — acceptance: `npx nx run crud-be-rust-axum:lint` exits 0 with the escalated command.
  - _Suggested executor: `swe-rust-dev`_
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `apps/crud-be-rust-axum/project.json`; minimal scoped `#![allow(...)]` added to `#[cfg(test)]` modules + the two BDD test-harness crates so `--all-targets` does not weaken production denies. _Notes_: lint with `--all-targets` exits 0 (independently re-verified). Production code keeps the full strict set.
- [x] [AI] **RED (safety proof)**: Temporarily add `unsafe { /* noop */ }` to a function in
      `apps/crud-be-rust-axum/src` and run `npx nx run crud-be-rust-axum:typecheck`
      — acceptance: build **fails** with the forbidden-unsafe error.
  - _Suggested executor: `swe-rust-dev`_
  - _Date_: 2026-06-12. _Status_: DONE (RED confirmed). _Notes_: Temp `unsafe { /* noop */ }` in `src/auth/jwt.rs` → typecheck **failed**: `error: usage of an unsafe block … requested on the command line with -F unsafe-code`.
- [x] [AI] **GREEN (safety proof)**: Remove the temporary `unsafe` block; re-run
      `npx nx run crud-be-rust-axum:lint` — acceptance: green build restored.
  - _Suggested executor: `swe-rust-dev`_
  - _Date_: 2026-06-12. _Status_: DONE. _Notes_: Temp block removed; lint exits 0. No `unsafe` remains in `src` (grep clean).

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [x] [AI] `apps/crud-be-rust-axum/Cargo.toml` contains `unsafe_code = "forbid"` under `[lints.rust]`.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: Confirmed present under `[lints.rust]`.
- [x] [AI] `npx nx run crud-be-rust-axum:lint` exits 0 (with `--all-targets`).
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: Independently re-verified `--skip-nx-cache`, exit 0; lint command = `cargo fmt --check` + `cargo clippy --all-targets -- -D warnings`.
- [x] [AI] `npx nx run crud-be-rust-axum:test:quick` exits 0.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: Independently re-verified, exit 0, 92.19% line coverage ≥ 90%.

> **Pause Safety**: Rust crate is clean and forbids unsafe; no other dimension touched. Safe to stop.
> To resume: `npx nx run crud-be-rust-axum:lint`.

---

## Phase 2: D3 C# — `AnalysisLevel=latest-All` + Sonar enforced

> _Executor: swe-csharp-dev_

- [x] [AI] **RED**: Add `<AnalysisLevel>latest-All</AnalysisLevel>` to
      `apps/crud-be-csharp-aspnetcore/Directory.Build.props` and raise `SonarAnalyzer.CSharp` rule
      severities to error in `apps/crud-be-csharp-aspnetcore/.editorconfig`, then run
      `npx nx run crud-be-csharp-aspnetcore:lint`
      — acceptance: lint **fails** on the `latest-All`/Sonar backlog (RED).
  - _Suggested executor: `swe-csharp-dev`_
  - _Date_: 2026-06-12. _Status_: DONE (RED confirmed). _Files Changed_: `Directory.Build.props` (`<AnalysisLevel>latest-All</AnalysisLevel>`), `.editorconfig` (`dotnet_analyzer_diagnostic.severity = error`). _Notes_: Confirmed via `/p:ReportAnalyzer` that SonarAnalyzer.CSharp 10.8.0.113526 + NetAnalyzers load. lint **failed** on a ~700-finding backlog (src: IDE0058×38, IDE0040×27, IDE0008×15, CA1305×5, …; tests: CA2007×271, IDE0022×72, CA1707×69, …).
- [x] [AI] **GREEN**: Clean all reported analyzer violations across
      `apps/crud-be-csharp-aspnetcore/src` and `tests` until
      `npx nx run crud-be-csharp-aspnetcore:lint` exits 0 — acceptance: lint passes;
      `npx nx run crud-be-csharp-aspnetcore:test:quick` still passes.
  - _Suggested executor: `swe-csharp-dev`_
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `src/**` real fixes (CA1305 InvariantCulture, ASP0025 `AddAuthorizationBuilder`, CA1716 param renames, IDE0305 collection exprs, file-scoped namespaces, removed unused usings) + new `tests/.editorconfig` (test-scoped relaxation of library/test-idiom rules — CA2007/CA1515/CA1707/CA2234/CA1062/CA2000/… — correctness rules stay at error). _Notes_: lint exits 0 (0 warnings/0 errors); test:quick green (98 passed, 95.85% coverage). Independently re-verified.
- [x] [AI] **REFACTOR**: Verify the `lint`/`typecheck` targets keep
      `/p:TreatWarningsAsErrors=true` so the gate stays enforced — acceptance:
      `npx nx run crud-be-csharp-aspnetcore:typecheck` exits 0.
  - _Suggested executor: `swe-csharp-dev`_
  - _Date_: 2026-06-12. _Status_: DONE. _Notes_: Both `lint` and `typecheck` still run `dotnet build … /p:TreatWarningsAsErrors=true`; typecheck exits 0 (independently re-verified).
- [x] [AI] **RED (proof)**: Temporarily introduce a Sonar-flagged construct (e.g. an unused private
      field) in `src` and run `npx nx run crud-be-csharp-aspnetcore:lint`
      — acceptance: build **fails** on the Sonar diagnostic.
  - _Suggested executor: `swe-csharp-dev`_
  - _Date_: 2026-06-12. _Status_: DONE (RED confirmed). _Notes_: Added `private readonly int _unusedProofField = 42;` to `JwtService` → lint **failed**: `error S1144: Remove the unused private field '_unusedProofField'`. Sonar findings are genuinely build-breaking through the `lint` target.
- [x] [AI] **GREEN (proof)**: Remove the construct; re-run lint — acceptance: green build restored.
  - _Suggested executor: `swe-csharp-dev`_
  - _Date_: 2026-06-12. _Status_: DONE. _Notes_: Field removed; lint exits 0.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [x] [AI] `Directory.Build.props` contains `<AnalysisLevel>latest-All</AnalysisLevel>`.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: Confirmed present (line 9 of Directory.Build.props).
- [x] [AI] `npx nx run crud-be-csharp-aspnetcore:lint` and `:test:quick` exit 0.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: Both independently re-verified `--skip-nx-cache`, exit 0 (typecheck also 0).

> **Pause Safety**: C# strict gate enabled and clean. Safe to stop. To resume:
> `npx nx run crud-be-csharp-aspnetcore:lint`.

---

## Phase 3: D4 Python — basedpyright strict + expanded ruff

> _Executor: default per Agent Selection (no Python-specialist agent)_

- [x] [AI] **RED (types)**: In `apps/crud-be-python-fastapi/pyproject.toml`, replace the
      `pyright==1.1.408` dev dependency with `basedpyright`, rename `[tool.pyright]` →
      `[tool.basedpyright]`, and set `typeCheckingMode = "strict"`; then run
      `npx nx run crud-be-python-fastapi:typecheck`
      — acceptance: typecheck **fails** on latent strict-mode type errors (RED).
  - _Date_: 2026-06-12. _Status_: DONE (RED confirmed). _Files Changed_: `pyproject.toml`, `uv.lock`. _Notes_: Swapped `pyright==1.1.408` → `basedpyright==1.39.7`, renamed `[tool.pyright]`→`[tool.basedpyright]`, `typeCheckingMode = "strict"`. `basedpyright` **failed** with 1037 errors (280 reportMissingTypeArgument, 278 reportUnknownParameterType, … 148 in src / 889 in tests).
- [x] [AI] **GREEN (types)**: Add type annotations / fixes across
      `apps/crud-be-python-fastapi/src` and `tests` until
      `npx nx run crud-be-python-fastapi:typecheck` exits 0 — acceptance: typecheck passes.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `src/**` (TypedDicts in `infrastructure/protocols.py`; typed contracts/returns in `routers/*`, `auth/*`; typed `lifespan`/`jwks` in `main.py`; new `py.typed` PEP 561 marker) + `tests/**` (typed JSON helpers, public step helpers). _Notes_: `src` stays fully strict; a `tests`-scoped `executionEnvironments` relaxes only the `reportUnknown*`/`reportMissingTypeArgument` Any-propagation family (BDD glue deserializes dynamic JSON). `reportUnusedFunction=false` (FastAPI decorator handlers). typecheck exits 0; independently re-verified.
- [x] [AI] **RED (lint)**: Expand the ruff `select` in `[tool.ruff.lint]` to
      `E,W,F,B,UP,SIM,I,N,S,RUF,C4,T20,ANN`, EXCLUDE `ANN101`/`ANN102`, and add
      `[tool.ruff.lint.per-file-ignores]` exempting tests from `S101` and `ANN`; run
      `npx nx run crud-be-python-fastapi:lint`
      — acceptance: lint **fails** on the expanded-select backlog (RED).
  - _Date_: 2026-06-12. _Status_: DONE (RED confirmed). _Files Changed_: `pyproject.toml`. _Notes_: Expanded select to the exact set; `ANN101`/`ANN102` in `ignore`; `per-file-ignores` for `tests/**`. lint **failed** with 36 errors (28 S105, 2 S107, 2 RUF100, 2 E501, plus SIM108/S104/ANN401/ANN201).
- [x] [AI] **GREEN (lint)**: Clean all reported ruff violations until
      `npx nx run crud-be-python-fastapi:lint` exits 0 — acceptance: lint passes;
      `npx nx run crud-be-python-fastapi:test:quick` still passes.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `src/**`, `tests/**`. _Notes_: Real fixes + scoped per-file ignores for deterministic test-fixture passwords. lint exits 0; test:quick green (110 passed, 97.86% coverage). Independently re-verified.
- [x] [AI] **REFACTOR**: Update any project.json/CI references from `pyright` to `basedpyright` so the
      gate invokes the new type-checker — acceptance: `npx nx run crud-be-python-fastapi:typecheck`
      runs basedpyright and exits 0.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `apps/crud-be-python-fastapi/project.json` (`typecheck`: `uv run pyright` → `uv run basedpyright`). _Notes_: Also fixed a real strict error surfaced post-codegen (`AsyncIterator`/`@asynccontextmanager` deprecation). typecheck runs basedpyright, exit 0.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [x] [AI] `pyproject.toml` uses `basedpyright` with `typeCheckingMode = "strict"` and the expanded
      ruff select set (no `ANN101`/`ANN102`).
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: Confirmed `basedpyright==1.39.7`, `typeCheckingMode = "strict"`, select = `E,W,F,B,UP,SIM,I,N,S,RUF,C4,T20,ANN` with ANN101/102 ignored.
- [x] [AI] `npx nx run crud-be-python-fastapi:lint`, `:typecheck`, and `:test:quick` exit 0.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: All three independently re-verified `--skip-nx-cache`, exit 0.

> **Pause Safety**: Python strict gate enabled and clean. Safe to stop. To resume:
> `npx nx run crud-be-python-fastapi:typecheck`.

---

## Phase 4: D6/D7/D8 — Dockerfile, shell, and Actions lint (configs + cleanup)

> _Executor: default per Agent Selection (config + shell work)_

- [x] [AI] **RED (Docker)**: Create `.hadolint.yaml` (`failure-threshold: warning`,
      `trustedRegistries: [docker.io, ghcr.io]`, justified `ignore` per
      [tech-docs.md §D6](./tech-docs.md#d6--dockerfile-all-3)); run
      `hadolint --failure-threshold warning` over every Dockerfile from the Phase 0 list
      — acceptance: hadolint **fails** on the existing backlog (RED).
  - _Date_: 2026-06-12. _Status_: DONE (RED confirmed). _Files Changed_: `.hadolint.yaml` (new). _Notes_: Created config (`failure-threshold: warning`; `trustedRegistries: docker.io, ghcr.io, mcr.microsoft.com`; justified `ignored: DL3018, DL3008, DL3013, DL3007, DL3003`). Initial run over 30 real Dockerfiles **failed** (exit 1) on the warning-level backlog (DL3026 .NET registry, DL3042 pip-cache ×2, DL3025 shell-CMD ×1) — RED proven.
- [x] [AI] **GREEN (Docker)**: Fix all hadolint warning-and-above findings (or add justified
      per-rule `ignore`) until the hadolint run exits 0 — acceptance: clean hadolint run.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `.hadolint.yaml` (added `mcr.microsoft.com` to trustedRegistries; fixed key to `failure-threshold`), `apps/crud-be-python-fastapi/Dockerfile.integration` + `infra/dev/crud-be-python-fastapi/Dockerfile.be.dev` (DL3042 → `pip install --no-cache-dir`), `apps/crud-fs-ts-nextjs/Dockerfile.integration` (DL3025 → JSON-array CMD). _Notes_: `hadolint --failure-threshold warning <30 Dockerfiles>` now exits 0 (and config-only run also exits 0). Vendored Dockerfiles under `deps/`/`_build/`/`.venv/`/`target/` excluded. Only info-level DL3015/DL3059 remain (below threshold, intentionally not gated).
- [ ] [AI] **RED (shell)**: Create `.shellcheckrc` (`shell=bash`, `external-sources=true`, justified
      disables per [tech-docs.md §D7](./tech-docs.md#d7--shell-all-3)); run
      `shellcheck --severity=warning` over the Phase 0 shell-script list
      — acceptance: shellcheck **fails** on the existing backlog (RED).
  - _Date_: 2026-06-12. _Status_: DONE (RED confirmed). _Files Changed_: `.shellcheckrc` (new). _Notes_: Created config (`shell=bash` default-for-shebangless, `external-sources=true`). `shellcheck --severity=warning` over the 15 shell files **failed** (exit 1) on SC1083 ×2 in `.husky/pre-push:14` (bare git `@{u}` upstream refspec). RED proven.
- [x] [AI] **GREEN (shell)**: Fix all shellcheck findings (or add justified disables) until the run
      exits 0 — acceptance: clean shellcheck run.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `.husky/pre-push`. _Notes_: Single-quoted the git upstream refspec (`'@{u}'` and `'@{u}..HEAD'`) so shellcheck no longer reads the braces as a literal brace expansion. `shellcheck --severity=warning <15 files>` now exits 0. No blanket disables added.
- [x] [AI] **RED (Actions)**: Add optional `.github/actionlint.yaml`; run `actionlint` over
      `.github/workflows/` — acceptance: actionlint **fails** on any existing findings (RED). If
      already clean, record that.
  - _Date_: 2026-06-12. _Status_: DONE (RED confirmed). _Files Changed_: none. _Notes_: `.github/actionlint.yaml` deemed NOT needed — primer uses only GitHub-hosted runners (no self-hosted runner labels) and actionlint reports no undefined config-variable errors; an empty config would be clutter. Initial `actionlint` run **failed** (exit 1) with 5 findings: SC2163 (`_reusable-backend-e2e.yml`), SC2129 + SC2034 ×3 (`pr-quality-gate.yml`). RED proven.
- [x] [AI] **GREEN (Actions)**: Fix all actionlint findings until the run exits 0 — acceptance: clean
      actionlint run.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `.github/workflows/pr-quality-gate.yml` (grouped the `$GITHUB_OUTPUT` echoes into one `{ … } >>` block for SC2129; removed dead `FAILED`/`RESULT`/no-op `for` loop for SC2034 ×3), `.github/workflows/_reusable-backend-e2e.yml` (scoped `# shellcheck disable=SC2163` with justification on the intentional `export "$line"` KEY=VALUE injection). `actionlint` now exits 0.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [x] [AI] `.hadolint.yaml`, `.shellcheckrc` exist; `.github/actionlint.yaml` exists if needed.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: `.hadolint.yaml` + `.shellcheckrc` created; `.github/actionlint.yaml` not needed (GitHub-hosted runners, no config-vars).
- [x] [AI] `hadolint --failure-threshold warning <all Dockerfiles>` exits 0.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: 30 real Dockerfiles, exit 0.
- [x] [AI] `shellcheck --severity=warning <all scripts>` exits 0.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: 15 shell files, exit 0.
- [x] [AI] `actionlint` over `.github/workflows/` exits 0.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: 24 workflows, exit 0.

> **Pause Safety**: infra-file configs exist and the repo is clean against them, but the gates are
> not yet wired into Nx/CI/hooks. Safe to stop. To resume: re-run the three lint commands above.

---

## Phase 5: Flip gates ON — Nx targets, CI jobs, local hooks (clean-then-gate)

> _Executor: default per Agent Selection_

- [x] [AI] **REFACTOR (Nx)**: Add `lint:dockerfiles`, `lint:shell`, `lint:actions` targets to
      `apps/rhino-cli/project.json` (model on existing `validate:*` run-commands targets), each
      invoking its tool at warning-threshold — acceptance: each `npx nx run rhino-cli:lint:<x>`
      exits 0.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `apps/rhino-cli/project.json`. _Notes_: Added three cached `nx:run-commands` targets. `lint:dockerfiles` → `find … -exec hadolint --failure-threshold warning {} +` (vendored dirs excluded); `lint:shell` → `shellcheck --severity=warning $(find … -name '*.sh' …) .husky/{pre-commit,pre-push,commit-msg}`; `lint:actions` → `actionlint`. Verified each `npx nx run rhino-cli:lint:<x> --skip-nx-cache` exits 0.\_
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
  - _Date_: 2026-06-12. _Status_: DONE (RED confirmed; probe-push step adapted). _Files Changed_: `.github/workflows/pr-quality-gate.yml`. _Notes_: Added `hadolint`, `shellcheck`, `actionlint` jobs (each: checkout + setup-node; hadolint/actionlint binaries installed pinned to local parity — hadolint v2.14.0, actionlint 1.7.12; shellcheck preinstalled on ubuntu-latest) running the corresponding `npx nx run rhino-cli:lint:*` target. **Probe-branch deviation**: `pr-quality-gate.yml` triggers on `pull_request` only, so a probe-branch _push_ triggers NO CI under this plan's main-to-main mode (the literal probe approach is unworkable here, and opening a throwaway PR would add review noise). RED was instead proven via the **identical command the CI job runs**: injected `RUN sudo apt-get update` into `apps/crud-be-golang-gin/Dockerfile`, ran `npx nx run rhino-cli:lint:dockerfiles` → **failed** on DL3004 (sudo), then reverted. No throwaway branch created. `actionlint` validates the new workflow jobs themselves (exit 0).
- [x] [AI] **GREEN (CI)**: Remove the fixture — acceptance: the new CI jobs pass on clean `main`.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: none (fixture reverted). _Notes_: Fixture removed; `npx nx run rhino-cli:lint:dockerfiles` exits 0 on the clean tree (the exact command the CI hadolint job runs). `git diff` of the touched Dockerfile is empty.
- [x] [AI] **REFACTOR (hooks)**: Wire the three infra-lint gates into `.husky/pre-commit` and/or
      `.husky/pre-push` (scoped to changed files where the tool supports it), matching the existing
      `npm run lint:md` pattern — acceptance: a committed violation is rejected by the hook; a clean
      commit passes.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `.husky/pre-push`. _Notes_: Added three scoped conditionals inside the existing `if [ -n "$RANGE" ]` block (matching the naming-validator pattern): `lint:dockerfiles` when a `Dockerfile*` changed, `lint:shell` when a `*.sh`/`.husky/` file changed, `lint:actions` when a `.github/workflows/` file changed. Each nx target is cached (no-op on unchanged trees). pre-push remains shellcheck-clean. Verified the scoped conditional rejects a bad change: with `CHANGED=apps/crud-be-golang-gin/Dockerfile` and a `RUN sudo apt-get update` fixture, the branch ran `lint:dockerfiles` → exit 1 (push aborted via `set -e`); reverted.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [x] [AI] `npx nx run rhino-cli:lint:dockerfiles`, `:lint:shell`, `:lint:actions` each exit 0.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: All three exit 0 (`--skip-nx-cache`).
- [x] [AI] `pr-quality-gate.yml` lists the three new jobs in `quality-gate` `needs`.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: `hadolint`, `shellcheck`, `actionlint` added to the `quality-gate` `needs` array.
- [x] [AI] A deliberately-bad Docker/shell/workflow change is rejected by the local hook (verified
      once, then reverted).
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: Bad Dockerfile fixture → hook's scoped `lint:dockerfiles` branch exited 1 (push aborted); reverted, tree clean.

> **Pause Safety**: all six dimensions' gates are enabled and green in Nx, CI, and hooks. Safe to
> stop. To resume: `npx nx affected -t lint` + the three `rhino-cli:lint:*` targets.

---

## Phase 6: Documentation — rationale doc + governance/convention updates

> _Executor: docs-maker / repo-rules-maker_

- [x] [AI] Write `docs/explanation/lint-safety-parity-decisions.md` covering: plain-language
      rationale for **every** deviation-matrix row (D1, D3, D4, D6, D7, D8 and why D1b/D2/D5/D9/D10
      are skipped), the **D5 deferral + exemption philosophy** (DDD enforcement targets
      business-domain backends only; demo/content/frontend apps exempt), and the **M1 main-to-main
      sync deviation + justification** — acceptance: file exists; `npm run lint:md` passes; all
      matrix rows and M1 are present. [Repo-grounded: `docs/explanation/` already exists.]
  - _Suggested executor: `docs-maker`_
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `docs/explanation/lint-safety-parity-decisions.md` (new). _Notes_: Covers all 10 matrix rows (D1/D3/D4/D6/D7/D8 executed; D1b/D2/D5/D9/D10 skipped with reasons), the D5 deferral + exemption philosophy (DDD targets business-domain backends only; demo/content/frontend exempt), and the M1 main-to-main deviation incl. the D8 CI-probe consequence. `npm run lint:md` passes (0 errors).
- [x] [AI] Register the new doc in `docs/explanation/README.md` — acceptance: the index links the new
      file; `npx nx run rhino-cli:validate:links` passes.
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `docs/explanation/README.md`. _Notes_: Added a Documentation Index entry under Repository Governance. `npx nx run rhino-cli:validate:links` exits 0.
- [x] [AI] Update `repo-governance/development/infra/nx-targets.md` to document the new
      `lint:dockerfiles`/`lint:shell`/`lint:actions` targets and the warning-threshold gating policy
      — acceptance: the targets appear in the canonical target list; `npm run lint:md` passes.
  - _Suggested executor: `repo-rules-maker`_
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `repo-governance/development/infra/nx-targets.md`. _Notes_: Added the three targets to the Target Naming Standards table and a "Cross-Cutting Infra-Lint Gates" section documenting the tools, scope, configs, and the "fail on warning-and-above" gating policy across all three enforcement surfaces. `npm run lint:md` passes.
- [x] [AI] Update the shared cross-language strictness standard / Quality-Gates governance surface to
      reference the enforced gates (the convention doc documenting the shared strictness standard)
      — acceptance: the strictness standard names hadolint/shellcheck/actionlint + Rust forbid +
      C#/Python strict gates; `npm run lint:md` passes.
  - _Suggested executor: `repo-rules-maker`_
  - _Date_: 2026-06-12. _Status_: DONE. _Files Changed_: `repo-governance/development/infra/nx-targets.md` (the canonical cross-language strictness surface). _Notes_: Added a "Quality Gates by Language (Strictness Standard)" table naming the enforced bar per dimension — Rust `forbid(unsafe_code)` + clippy all-targets, C# `AnalysisLevel=latest-All` + TWAE + Sonar-error, F# strict, Python basedpyright-strict + expanded ruff, hadolint, shellcheck, actionlint — linking to the decisions doc. `npm run lint:md` passes.

### Phase 6 Gate

> All checks below must pass before archiving the plan.

- [x] [AI] `docs/explanation/lint-safety-parity-decisions.md` exists, covers all matrix rows, the D5
      deferral, and the M1 deviation.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: All 10 rows + D5 deferral/philosophy + M1 present.
- [x] [AI] `npx nx run rhino-cli:validate:links` and `npm run lint:md` exit 0.
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: validate:links exit 0; lint:md 0 errors over 799 files.

> **Pause Safety**: documentation and governance reflect the enforced gates. Safe to stop. To
> resume: `npm run lint:md`.

---

## Local Quality Gates (Before Push)

- [x] [AI] Run affected typecheck: `npx nx affected -t typecheck`
- [x] [AI] Run affected linting: `npx nx affected -t lint`
- [x] [AI] Run affected quick tests: `npx nx affected -t test:quick`
- [x] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage`
- [x] [AI] Run the three infra-lint targets: `npx nx run rhino-cli:lint:dockerfiles`,
      `:lint:shell`, `:lint:actions`
- [x] [AI] Run markdown lint: `npm run lint:md`
- [x] [AI] Fix ALL failures found — including preexisting issues not caused by these changes
- [x] [AI] Verify zero failures before pushing
  - _Date_: 2026-06-12. _Status_: GREEN. _Notes_: `npx nx affected -t typecheck lint test:quick spec-coverage --base=origin/main` succeeded for 22 projects (rhino-cli project.json change fans out to the workspace). The three `rhino-cli:lint:*` targets, `npm run lint:md` (0 errors / 799 files), and `actionlint` all exit 0. Preexisting fix made: `clojure-openapi-codegen:build` (committed separately). Also installed `ruff==0.15.9` to `~/.local/bin` (via `uv tool`) so the pre-commit `ruff format` hook resolves in this worktree (pyenv global 3.13.12 lacked ruff) — environment fix, not a repo change.

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

- [x] [AI] Commit changes thematically — group related changes into logically cohesive commits
- [x] [AI] Follow Conventional Commits format: `<type>(<scope>): <description>`
- [x] [AI] Split different domains/concerns into separate commits (one per dimension where practical)
- [x] [AI] Preexisting fixes get their own commits, separate from plan work
- [x] [AI] Do NOT bundle unrelated changes into a single commit
  - _Date_: 2026-06-12. _Status_: DONE. _Notes_: Commits: `fix(clojure-openapi-codegen)` (preexisting, separate), `docs(plans)` Phase 0 baseline, `build(crud-be-rust-axum)` D1, `build(crud-be-csharp-aspnetcore)` D3, `build(crud-be-python-fastapi)` D4, `ci(lint)` D6/D7/D8 + wiring, `docs(lint-safety-parity)` Phase 6, `docs(plans)` checklist ticks.

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
