# Delivery — Primer Polyglot Demo-App CI Restoration

> All code steps are expressed as fresh-checkout reproductions so "done" means a clean tree regenerates
> and the gate passes.
>
> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.

## Worktree

Worktree path: `worktrees/primer-polyglot-codegen-ci-restoration/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree primer-polyglot-codegen-ci-restoration
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before deleting the
worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0: Environment + baseline

- [ ] [AI] From the repo root, run `npm install && npm run doctor -- --fix` to install dependencies and
      converge all polyglot toolchains. Acceptance: `npm run doctor -- --scope minimal` exits 0,
      confirming Dart/Flutter, Elixir, Go (with golangci-lint), Rust, and .NET toolchains are present.

- [ ] [AI] Establish the failing baseline for each affected app by running the following four commands
      (each must fail — these are the RED states the fixes must turn GREEN):
  - `rm -rf apps/crud-fe-dart-flutterweb/generated-contracts && npx nx run crud-fe-dart-flutterweb:lint --skip-nx-cache` — expected failure: "No pubspec.yaml found for package crud_contracts"
  - `rm -rf apps/crud-be-rust-axum/generated-contracts && npx nx run crud-be-rust-axum:lint --skip-nx-cache` — expected failure: Cargo.toml missing
  - `rm -rf apps/crud-be-golang-gin/generated-contracts && npx nx run crud-be-golang-gin:lint --skip-nx-cache` — expected failure: types.gen.go missing
  - `cd apps/crud-be-elixir-phoenix && mix deps.clean --all && mix deps.get && MIX_ENV=test mix compile --warnings-as-errors` — record whether it passes (transient) or fails (real dep issue)

  Acceptance: exact failure messages recorded. Repo is in a known-failing state for Dart/Rust/Go (expected).

> **Important**: Fix ALL failures found during quality gates — including preexisting issues not caused by
> your changes. This follows the root cause orientation principle — proactively fix preexisting errors
> encountered during work. Do not defer or mention-and-skip existing issues.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm run doctor -- --scope minimal` exits 0 — all required language toolchains present.
- [ ] [AI] Each of the four baseline commands above was run and its failure (or pass, for Elixir) was
      recorded — baseline RED state confirmed for Dart, Rust, Go.

> **Pause Safety**: All toolchains installed; baseline failures documented. Repo is in a known-failing
> state for the target apps (expected). Safe to stop. To resume: re-read this Phase 0 Gate and verify each
> baseline still fails as recorded, then proceed to Phase 1.

## Phase 1: .NET CVE — DONE (verify only)

- [ ] [AI] Confirm `c82c66c6f` is present: `git log --oneline | grep SQLitePCLRaw`. Acceptance: commit
      hash appears in output.
- [ ] [AI] Verify `dotnet build apps/crud-be-fsharp-giraffe/tests/DemoBeFsgi.Tests/DemoBeFsgi.Tests.fsproj -c Release`
      and `dotnet build apps/crud-be-csharp-aspnetcore/tests/DemoBeCsas.Tests/DemoBeCsas.Tests.csproj -c Release`
      each complete with 0 errors and no `NU1903`. Acceptance: both commands exit 0 with no `NU1903` in output.

### Phase 1 Gate

> All checks below must pass before starting Phase 1b.

- [ ] [AI] `git log --oneline | grep SQLitePCLRaw` returns the `c82c66c6f` entry.
- [ ] [AI] Both dotnet build commands exit 0 with no `NU1903`.

> **Pause Safety**: .NET CVE remediation confirmed green. Safe to stop. To resume: re-run the two dotnet
> build commands above and confirm 0 errors and no `NU1903`.

## Phase 1b: Class B (CI-only) — .NET codegen ordering + Elixir deps

- [ ] [AI] Inspect `.github/workflows/pr-quality-gate.yml`: confirm the C# build step has a `dependsOn`
      or `needs` relationship that ensures the `codegen` target completes before the C# compile step runs
      under the cold-cache matrix. Acceptance: `grep -n "dependsOn\|codegen\|needs" .github/workflows/pr-quality-gate.yml`
      returns the relevant dependency declaration for the dotnet quality-gate job.

- [ ] [AI] If the ordering dependency is missing or insufficient, fix the `.github/workflows/pr-quality-gate.yml`
      to add an explicit `needs: [codegen]` or equivalent `dependsOn` for the C# build step. Also check
      whether the first-run `openapi-generator` JAR download may race with the compile step, and add a
      pre-flight JAR warm-up step if needed. Acceptance: `grep -n "dependsOn\|needs" .github/workflows/pr-quality-gate.yml`
      confirms the dependency; `.NET quality gate` job on GitHub Actions (https://github.com/wahidyankf/ose-primer/actions)
      concludes `success` with no `CS2001` error on the next all-affected commit.

- [ ] [AI] Cross-check the parallel-restore race family (the `.NET` `nuget.g.targets` "already exists" race
      seen on first run). If the race recurs after the ordering fix, add `--no-parallel` to the NuGet restore
      step in `.github/workflows/pr-quality-gate.yml`. Acceptance: no `nuget.g.targets already exists` error
      appears in subsequent CI runs.

- [ ] [AI] For Elixir CI deps: if Phase 5 (below) confirms the failure is transient, add a note to this
      step: `[transient — reproduced clean locally on YYYY-MM-DD, CI retry recommended]`. If Phase 5 reveals
      a real dependency issue, fix it in Phase 5 and update this step. Acceptance: this step records the
      resolution outcome.

### Phase 1b Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `.github/workflows/pr-quality-gate.yml` codegen ordering dependency confirmed present.
- [ ] [AI] `.NET quality gate` job on the next all-affected CI run concludes `success` with no `CS2001`.

> **Pause Safety**: CI-only ordering issues investigated and fixed (or documented). Safe to stop. To resume:
> re-read this Phase 1b Gate and verify the `CS2001` fix is in place by re-checking the workflow file.

## Phase 2: Dart — produce a resolvable package fresh (AC-1)

- [ ] [AI] Determine remediation option: review `tech-docs.md` §Dart options A and B.
  - **Option A (preferred)**: make the dart `codegen` target in `apps/crud-fe-dart-flutterweb/project.json`
    emit a complete package (drop `--global-property=models` to get a full package, or append a `printf`/template
    `pubspec.yaml` like the Rust target does for `Cargo.toml`).
  - **Option B**: activate `rhino-cli specs scaffold dart` in `apps/rhino-cli/src/commands/specs_scaffold_dart.rs`
    runtime-conditionally; update `.claude/agents/repo-harness-compatibility-checker.md` to reflect the
    deliberate divergence; run `npm run generate:bindings`.

  Record the chosen option and rationale as a comment at the top of `apps/crud-fe-dart-flutterweb/project.json`
  (inline in the file). Acceptance: the choice and one-sentence rationale is traceable in the git commit diff.

- [ ] [AI] **RED**: `rm -rf apps/crud-fe-dart-flutterweb/generated-contracts && npx nx run crud-fe-dart-flutterweb:codegen --skip-nx-cache` — expected to fail with "No pubspec.yaml found for package crud_contracts".
      **Gherkin (binds) →** "Fresh Dart codegen yields a usable crud_contracts package"

  ```gherkin
  Scenario: Fresh Dart codegen yields a usable crud_contracts package
    Given apps/crud-fe-dart-flutterweb/generated-contracts does not exist
    When nx run crud-fe-dart-flutterweb:codegen runs with --skip-nx-cache
    Then a pubspec.yaml exists at apps/crud-fe-dart-flutterweb/generated-contracts/
    And flutter pub get for crud-fe-dart-flutterweb resolves crud_contracts without error
    And nx run crud-fe-dart-flutterweb:lint exits 0
  ```

  Acceptance: command fails — confirms the RED state is still present before the fix.

- [ ] [AI] **GREEN**: Implement the chosen fix in `apps/crud-fe-dart-flutterweb/project.json` (Option A:
      update the `codegen` executor command to emit a complete package including `pubspec.yaml`; or Option B:
      wire `dart_scaffold.rs`, run `npm run generate:bindings`). Re-run:
      `rm -rf apps/crud-fe-dart-flutterweb/generated-contracts && npx nx run crud-fe-dart-flutterweb:codegen --skip-nx-cache`.
      Acceptance: `pubspec.yaml` exists at `apps/crud-fe-dart-flutterweb/generated-contracts/`; `flutter pub get`
      resolves `crud_contracts` without error; `npx nx run crud-fe-dart-flutterweb:lint --skip-nx-cache` exits 0.

- [ ] [AI] **REFACTOR**: Review `apps/crud-fe-dart-flutterweb/project.json` (and, if Option B, `apps/rhino-cli/src/commands/specs_scaffold_dart.rs`) for cleanup opportunities — remove redundant flags, clarify command comments, remove dead code. Re-run `npx nx run crud-fe-dart-flutterweb:lint --skip-nx-cache`. Acceptance: exits 0; no regressions introduced.

- [ ] [AI] If Option B: run `npm run generate:bindings`; verify `rhino-cli` source stays byte-identical
      across repos (`md5` compare vs `ose-public`/`ose-infra`) OR the divergence is documented as deliberate
      with `.claude/agents/repo-harness-compatibility-checker.md` updated; run
      `npx nx run rhino-cli:cross-vendor:parity-validation`. Acceptance: parity-validation exits 0.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `rm -rf apps/crud-fe-dart-flutterweb/generated-contracts && npx nx run crud-fe-dart-flutterweb:codegen --skip-nx-cache` — `pubspec.yaml` exists at `apps/crud-fe-dart-flutterweb/generated-contracts/`.
- [ ] [AI] `npx nx run crud-fe-dart-flutterweb:lint --skip-nx-cache` exits 0.
- [ ] [AI] (If Option B) `npx nx run rhino-cli:cross-vendor:parity-validation` exits 0.

> **Pause Safety**: Dart codegen produces a resolvable package on a clean checkout. Safe to stop. To resume:
> re-run the Phase 2 Gate commands and confirm all pass.

## Phase 3: Rust — deterministic manifest under nx (AC-2)

- [ ] [AI] Diagnose: run `npx nx run crud-be-rust-axum:codegen --skip-nx-cache --verbose 2>&1 | tee /tmp/rust-codegen-diag.txt`;
      inspect the output for cwd value and per-`&&`-step exit codes. Acceptance: root cause confirmed —
      record in a comment in `apps/crud-be-rust-axum/project.json`: either "cwd mismatch" (if `$(pwd)`
      resolves to the wrong directory), "&&-chain short-circuit" (if an earlier step silently fails before
      the `printf` steps), or "other: `<description>`".

- [ ] [AI] **RED**: `rm -rf apps/crud-be-rust-axum/generated-contracts && npx nx run crud-be-rust-axum:lint --skip-nx-cache` — expected to fail (Cargo.toml missing).
      **Gherkin (binds) →** "Fresh Rust codegen yields Cargo.toml and module wiring"

  ```gherkin
  Scenario: Fresh Rust codegen yields Cargo.toml and module wiring
    Given apps/crud-be-rust-axum/generated-contracts does not exist
    When nx run crud-be-rust-axum:codegen runs with --skip-nx-cache
    Then Cargo.toml, src/lib.rs, and src/models/mod.rs exist under generated-contracts/
    And nx run crud-be-rust-axum:lint and :test:quick exit 0
  ```

  Acceptance: command fails — confirms the RED state is still present before the fix.

- [ ] [AI] **GREEN**: Fix `apps/crud-be-rust-axum/project.json` codegen command to make manifest generation
      robust under nx (replace `$(pwd)` with `{workspaceRoot}/apps/crud-be-rust-axum` as the absolute path
      prefix, split the `&&` chain into ordered steps that do not silently short-circuit, or move the
      manifest scaffolding into a small script the target invokes). Re-run:
      `rm -rf apps/crud-be-rust-axum/generated-contracts && npx nx run crud-be-rust-axum:codegen --skip-nx-cache`.
      Acceptance: `Cargo.toml`, `src/lib.rs`, `src/models/mod.rs` exist under
      `apps/crud-be-rust-axum/generated-contracts/`; `npx nx run crud-be-rust-axum:lint --skip-nx-cache`
      and `npx nx run crud-be-rust-axum:test:quick --skip-nx-cache` both exit 0 fresh.

- [ ] [AI] **REFACTOR**: Review `apps/crud-be-rust-axum/project.json` codegen command and any new script
      files for cleanup — remove redundant flags, improve inline comments, ensure the cwd fix is minimal and
      clear. Re-run `npx nx run crud-be-rust-axum:lint --skip-nx-cache` and
      `npx nx run crud-be-rust-axum:test:quick --skip-nx-cache`. Acceptance: all tests still pass; code is cleaner.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `rm -rf apps/crud-be-rust-axum/generated-contracts && npx nx run crud-be-rust-axum:codegen --skip-nx-cache` — `Cargo.toml`, `src/lib.rs`, and `src/models/mod.rs` exist under `apps/crud-be-rust-axum/generated-contracts/`.
- [ ] [AI] `npx nx run crud-be-rust-axum:lint --skip-nx-cache` exits 0.
- [ ] [AI] `npx nx run crud-be-rust-axum:test:quick --skip-nx-cache` exits 0.

> **Pause Safety**: Rust codegen deterministically produces a buildable crate on a clean checkout. Safe to
> stop. To resume: re-run the Phase 3 Gate commands and confirm all pass.

## Phase 4: Go — generate types from OpenAPI 3.1 (AC-3)

- [ ] [AI] Determine remediation option: review `tech-docs.md` §Go options A and B.
  - **Option A**: swap `oapi-codegen` for an OpenAPI-3.1-capable generator (e.g., `openapi-generator -g go`
    models, matching the Rust/Dart pattern).
  - **Option B**: add a 3.0 downconversion step (e.g., `openapi-cli bundle --openapi-version 3.0`) feeding
    only the Go types step.

  Record the chosen option and rationale as a comment in `apps/crud-be-golang-gin/project.json`.
  Before finalizing, review the generated-type diff (use `git diff` on `apps/crud-be-golang-gin/`) to
  confirm no previously-used type names are absent or renamed. Acceptance: choice documented in file;
  `go build ./...` with the chosen generator exits 0; no previously-used type names missing in `git diff`.

- [ ] [AI] **RED**: `rm -rf apps/crud-be-golang-gin/generated-contracts && npx nx run crud-be-golang-gin:lint --skip-nx-cache` — expected to fail (types.gen.go missing).
      **Gherkin (binds) →** "Fresh Go codegen yields types.gen.go from a 3.1 spec"

  ```gherkin
  Scenario: Fresh Go codegen yields types.gen.go from a 3.1 spec
    Given apps/crud-be-golang-gin/generated-contracts does not exist
    When nx run crud-be-golang-gin:codegen runs with --skip-nx-cache
    Then types.gen.go exists with the contract types
    And nx run crud-be-golang-gin:lint and :test:quick exit 0
  ```

  Acceptance: command fails — confirms the RED state is still present before the fix.

- [ ] [AI] **GREEN**: Implement the chosen fix in `apps/crud-be-golang-gin/project.json` (update the
      `codegen` executor command to use the chosen generator). Re-run:
      `rm -rf apps/crud-be-golang-gin/generated-contracts && npx nx run crud-be-golang-gin:codegen --skip-nx-cache`.
      Acceptance: `types.gen.go` exists at `apps/crud-be-golang-gin/generated-contracts/` with the contract
      types; `npx nx run crud-be-golang-gin:lint --skip-nx-cache` and
      `npx nx run crud-be-golang-gin:test:quick --skip-nx-cache` both exit 0 fresh.

- [ ] [AI] **REFACTOR**: Review `apps/crud-be-golang-gin/project.json` codegen command for cleanup —
      remove redundant flags, clarify generator selection comments. Re-run
      `npx nx run crud-be-golang-gin:lint --skip-nx-cache` and
      `npx nx run crud-be-golang-gin:test:quick --skip-nx-cache`. Acceptance: all tests still pass; code
      is cleaner.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `rm -rf apps/crud-be-golang-gin/generated-contracts && npx nx run crud-be-golang-gin:codegen --skip-nx-cache` — `types.gen.go` exists with contract types.
- [ ] [AI] `npx nx run crud-be-golang-gin:lint --skip-nx-cache` exits 0.
- [ ] [AI] `npx nx run crud-be-golang-gin:test:quick --skip-nx-cache` exits 0.

> **Pause Safety**: Go codegen generates types from the OpenAPI 3.1 spec on a clean checkout. Safe to
> stop. To resume: re-run the Phase 4 Gate commands and confirm all pass.

## Phase 5: Elixir — reproduce or confirm transient (AC-4)

- [ ] [AI] Reproduce clean: from the repo root, run
      `cd apps/crud-be-elixir-phoenix && mix deps.clean --all && mix deps.get && MIX_ENV=test mix compile --warnings-as-errors`.
      Acceptance:
  - If it **fails**: root-cause the offending dependency (identify the package name from the error output),
    fix it (pin or upgrade in `apps/crud-be-elixir-phoenix/mix.exs`), re-run until `mix compile` exits 0.
  - If it **passes**: document the CI failure as transient by appending this note to this step:
    `[transient — reproduced clean locally on YYYY-MM-DD, CI retry recommended]`.

- [ ] [AI] Optional hygiene: move `:preferred_cli_env` from `def project` to `def cli` in
      `apps/crud-be-elixir-phoenix/mix.exs` to clear the deprecation warning. Acceptance:
      `mix compile --warnings-as-errors` exits 0 with no `:preferred_cli_env in def project is deprecated` warning.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `cd apps/crud-be-elixir-phoenix && mix deps.clean --all && mix deps.get && MIX_ENV=test mix compile --warnings-as-errors` exits 0.
- [ ] [AI] Resolution outcome documented: either real fix applied (if deps issue found) or transient note added to the step above.

> **Pause Safety**: Elixir gate either fixed or documented as transient. Safe to stop. To resume: re-run
> the mix compile command above and confirm it exits 0.

## Phase 6: Specs/Gherkin delivery

- [ ] [AI] Write `specs/apps/crud/behavior/crud-web/gherkin/codegen/dart-codegen-fresh-checkout.feature`
      (create any missing parent directories; `crud-fe-dart-flutterweb` is a web frontend → surface
      `crud-web`) with a scenario verbatim-equal to prd.md AC-1:
      `Scenario: Fresh Dart codegen yields a usable crud_contracts package`. Acceptance: file exists and
      contains the scenario; `grep -l "Fresh Dart codegen" specs/apps/crud/` returns the file path.

- [ ] [AI] Write `specs/apps/crud/behavior/crud-be/gherkin/codegen/rust-codegen-fresh-checkout.feature`
      (create any missing parent directories; `crud-be-rust-axum` is a backend HTTP service → surface
      `crud-be`) with a scenario matching prd.md AC-2:
      `Scenario: Fresh Rust codegen yields Cargo.toml and module wiring`. Acceptance: file exists with
      matching scenario.

- [ ] [AI] Write `specs/apps/crud/behavior/crud-be/gherkin/codegen/go-codegen-fresh-checkout.feature`
      (create any missing parent directories; `crud-be-golang-gin` is a backend HTTP service → surface
      `crud-be`) with a scenario matching prd.md AC-3:
      `Scenario: Fresh Go codegen yields types.gen.go from a 3.1 spec`. Acceptance: file exists with
      matching scenario.

- [ ] [AI] Run `npx nx affected -t specs:coverage`. Acceptance: exits 0 — specs coverage gate passes.

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] All three `.feature` files exist with their respective scenarios — confirmed by `ls specs/apps/crud/behavior/crud-web/gherkin/codegen/dart-codegen-fresh-checkout.feature specs/apps/crud/behavior/crud-be/gherkin/codegen/rust-codegen-fresh-checkout.feature specs/apps/crud/behavior/crud-be/gherkin/codegen/go-codegen-fresh-checkout.feature`.
- [ ] [AI] `npx nx affected -t specs:coverage` exits 0.

> **Pause Safety**: Gherkin feature files written and specs:coverage gate passing. Safe to stop. To resume:
> re-run `npx nx affected -t specs:coverage` and confirm exit 0.

## Phase 7: Local full-matrix verification (before push)

- [ ] [AI] For each language, clean and re-run the per-language gate exactly as the CI matrix does
      (with `--skip-nx-cache`):
  - **Dart**: `rm -rf apps/crud-fe-dart-flutterweb/generated-contracts && npx nx run-many -t typecheck lint test:quick specs:coverage -p crud-fe-dart-flutterweb --skip-nx-cache`
  - **Rust**: `rm -rf apps/crud-be-rust-axum/generated-contracts && npx nx run-many -t typecheck lint test:quick specs:coverage -p crud-be-rust-axum --skip-nx-cache`
  - **Go**: `rm -rf apps/crud-be-golang-gin/generated-contracts && npx nx run-many -t typecheck lint test:quick specs:coverage -p crud-be-golang-gin --skip-nx-cache`
  - **Elixir**: `rm -rf apps/crud-be-elixir-phoenix/generated-contracts 2>/dev/null; npx nx run-many -t typecheck lint test:quick specs:coverage -p crud-be-elixir-phoenix --skip-nx-cache`
  - **.NET**: `npx nx run-many -t typecheck lint test:quick specs:coverage -p crud-be-csharp-aspnetcore,crud-be-fsharp-giraffe --skip-nx-cache`

  Acceptance: all five run-many commands exit 0.

> **Important**: Fix ALL failures found — including preexisting issues not caused by your changes. Root
> cause orientation — proactively fix preexisting errors encountered during work. Do not defer or
> mention-and-skip existing issues.

- [ ] [AI] `npm run lint:md` exits 0. Acceptance: no markdown linting errors.
- [ ] [AI] (If Phase 2 Option B touched `rhino-cli`) `npx nx run rhino-cli:cross-vendor:parity-validation`
      exits 0. Acceptance: parity-validation exits 0.

### Phase 7 Gate

> All checks below must pass before starting Phase 8.

- [ ] [AI] All five per-language run-many commands exit 0.
- [ ] [AI] `npm run lint:md` exits 0.

> **Pause Safety**: All local per-language quality gates pass on a clean checkout. Safe to stop. To resume:
> re-run the Phase 7 run-many commands per language and confirm all exit 0.

## Phase 8: Commit, push, CI verification

### Commit Guidelines

- [ ] [AI] Commit changes thematically using Conventional Commits format (`<type>(<scope>): <description>`):
      one commit per language fix — e.g., `fix(dart): emit pubspec.yaml from codegen target`,
      `fix(rust): replace $(pwd) with {workspaceRoot} in codegen command`, `fix(go): switch to 3.1-capable generator`,
      `fix(elixir): pin offending dep or document transient flake`. Commit CI/workflow changes separately:
      `fix(ci): ensure dotnet codegen dependsOn ordering`. Do NOT bundle fixes for different languages or
      CI/app into one commit. Do NOT include unrelated changes.

- [ ] [AI] Push all commits to `origin main` directly (Trunk Based Development — no PR unless explicitly
      requested). Acceptance: `git push origin HEAD:main` exits 0.

- [ ] [AI] Trigger an all-affected condition (these fixes touch app config; if not all-affected, a
      follow-up `rhino-cli`-touching commit or a manual `workflow_dispatch` exercises the full matrix).
      Monitor `PR - Quality Gate` at https://github.com/wahidyankf/ose-primer/actions until every
      per-language job concludes `success` (poll per CI-monitoring convention; never use `gh run watch`).
      Fix-forward any residual failure; do not bypass.

### Phase 8 Gate

> All checks below must pass before starting Phase 9.

- [ ] [AI] `git log --oneline origin/main | head -1` returns the most recent fix commit.
- [ ] [AI] `PR - Quality Gate` on GitHub Actions shows every per-language job as `success` on an
      all-affected commit.

> **Pause Safety**: All commits pushed and CI is green on a full all-affected run. Safe to stop. To resume:
> check the GitHub Actions run at https://github.com/wahidyankf/ose-primer/actions and confirm green.

## Phase 9: Archival

- [ ] [AI] Verify ALL delivery checklist items above are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI).
- [ ] [AI] Move plan folder: `git mv plans/in-progress/primer-polyglot-codegen-ci-restoration plans/done/$(date +%Y-%m-%d)__primer-polyglot-codegen-ci-restoration`.
      Acceptance: `git status` shows the rename.
- [ ] [AI] Update `plans/in-progress/README.md` — remove the `primer-polyglot-codegen-ci-restoration` entry.
      Acceptance: entry absent from file.
- [ ] [AI] Update `plans/done/README.md` — add the entry with the completion date.
      Acceptance: entry present in file.
- [ ] [AI] Commit: `chore(plans): move primer-polyglot-codegen-ci-restoration to done`. Push to `origin main`.

### Phase 9 Gate

> All checks below must pass to declare this plan complete.

- [ ] [AI] Plan folder exists under `plans/done/YYYY-MM-DD__primer-polyglot-codegen-ci-restoration/`.
- [ ] [AI] `plans/in-progress/README.md` does NOT contain `primer-polyglot-codegen-ci-restoration`.
- [ ] [AI] `plans/done/README.md` DOES contain `primer-polyglot-codegen-ci-restoration`.

> **Pause Safety**: Plan archived. All work complete. Safe to stop.

## Verification (how to confirm done)

- AC-1..AC-5: each verified by the fresh-checkout reproduction exiting 0 for the relevant app.
- AC-6: `ose-primer` `PR - Quality Gate` concludes `success` with every per-language job green on an
  all-affected commit.
- AC-7: `rhino-cli` byte-identical mirror holds (or deliberately updated with checker) and bindings stay in
  sync.
