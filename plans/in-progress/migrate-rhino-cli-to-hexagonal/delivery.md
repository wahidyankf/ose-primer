# Delivery Checklist — rhino-cli Hexagonal Migration

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

---

## Decisions Requiring User Approval

> The orchestrator MUST resolve these with the user (grilling, 2–4 options each) BEFORE
> executing this plan. They are micro/scoping choices the subagent could not interactively
> settle. None block plan acceptance; all block execution.

### D1. CLI-output improvement set — RESOLVED: output fully frozen (zero visible change)

**RESOLVED.** The user chose **strict zero visible-output-change**. The frozen output-change
list is **EMPTY** — no visible text or format change is in scope anywhere in this plan. The
golden corpus is therefore **unchanged**, and `shadow-diff.sh` is compared against the
**EXISTING Phase 0 baseline throughout the whole plan**. There is **no re-baselining anywhere**.

O1/O2/O3 below still occur, but **ONLY as byte-neutral port extractions** — they relocate IO
writes behind named ports without changing a single emitted byte. The candidate table is
retained purely for traceability; **none of these candidates introduces a visible change, and
none requires re-capturing the shadow-diff baseline.**

| #   | Command / surface                        | Current output (observed)                                                                                           | Byte-neutral seam extraction (NO visible change)                                             | Why it aids layering                                                             |
| --- | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| O1  | `--say` with `--verbose` (root)          | `[<ts>] INFO: Executing say command` then `[<ts>] INFO: Message: <msg>` [Repo-grounded — `cmd/root.go` lines 31–36] | Route the two INFO lines through a domain-role `DiagnosticLogger` port — **identical bytes** | Forces the verbose-logging seam behind a port; **byte-neutral**                  |
| O2  | `git pre-commit` step warnings           | `⚠️  Step %q timed out…` / `⚠️  Total pre-commit timeout reached…` [Repo-grounded — `runner.go` lines 64,79]        | Route emoji-warning writes through a `StepReporter` port — **identical bytes**               | Removes direct `Fprintf(deps.Stdout,…)` from orchestration; pure seam extraction |
| O3  | Error prefix on failure (root `Execute`) | `Error: %v` to stderr [Repo-grounded — `cmd/root.go` line 46]                                                       | Centralize through an inbound-adapter error presenter — **identical bytes**                  | Makes the inbound adapter the single error-formatting point                      |

> **Frozen-output rule (binding on every phase)**: O1/O2/O3 are byte-neutral seam extractions
> only. No phase introduces a visible text/format change, and no phase re-captures the
> shadow-diff baseline. Every phase compares `shadow-diff.sh` against the existing Phase 0
> baseline and expects GREEN with a byte-identical, unchanged corpus.

### D2. Phase grouping for the 13 features — RESOLVED: one phase per feature

**RESOLVED.** The user chose **Option B: one phase per feature** (smallest blast radius per
gate), rejecting the previously-drafted grouped Option A. The delivery checklist below
implements one feature per phase in dependency-respecting order (Phases 2–14), with the `git`
pilot at Phase 1 and the convention-doc update at Phase 15. Each feature phase carries the same
per-phase machinery the pilot uses: shadow-diff GREEN before + after, RED→GREEN→REFACTOR where
ports are extracted, a coverage-allowlist lockstep item for any relocated Rust file, Local
Quality Gates, Post-Push Verification per D3, and a `### Phase N Gate` + Pause Safety note.

### D3. Execution push mode for the eventual CODE refactor (flag for execution phase)

The PLAN DOCUMENT is pushed direct to `origin main` (Trunk-Based Development). The code-refactor
execution push mode is deliberately NOT decided here:

- **Option A (Recommended)**: direct push to `origin main` per phase (TBD default; each phase is a
  green natural pause). Rationale: matches repo default; phase gates already guarantee safety.
- **Option B**: single draft PR for the whole migration, flipped to ready at the end. Rationale:
  one reviewable surface; trade-off: long-lived branch drifts from `main`.
- **Option C**: one draft PR per phase. Rationale: reviewable per feature; trade-off: PR churn.

> This item is intentionally left for the execution phase. Do NOT add PR-creation steps to this
> plan unless the user selects Option B or C.

---

## Worktree

Worktree path: `worktrees/migrate-rhino-cli-to-hexagonal/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree migrate-rhino-cli-to-hexagonal
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the
latest `origin/main` when missing, syncs with `origin/main` before implementing, and prompts
before deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized.
- [ ] [AI] Converge the full polyglot toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift (Go + Rust toolchains present).
- [ ] [AI] Build both binaries: `npx nx run rhino-cli-go:build` and
      `npx nx run rhino-cli-rust:build` — acceptance: both exit 0; `apps/rhino-cli-go/dist/rhino-cli`
      and `apps/rhino-cli-rust/dist/rhino-cli` exist.
- [ ] [AI] Capture the golden-master baseline: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh`
      — acceptance: exits 0 (GREEN) with zero divergence; record the run in this checklist.
- [ ] [AI] Establish test + coverage baseline on both apps:
      `npx nx run-many -t test:unit test:integration test:quick --projects=rhino-cli-go,rhino-cli-rust`
      — acceptance: record pass/fail counts and coverage % for each; both `test:quick` report ≥90%.
- [ ] [AI] Establish lint + typecheck + parity baseline:
      `npx nx run-many -t lint typecheck --projects=rhino-cli-go,rhino-cli-rust` and
      `npx nx run rhino-cli-rust:validate:cross-vendor-parity`
      — acceptance: all exit 0; document any preexisting failures.
- [ ] [AI] Resolve ALL preexisting failures before proceeding (root-cause orientation)
      — acceptance: no unresolved preexisting failures remain.
- [ ] [AI] Confirm the empty hex placeholder dirs exist:
      `ls apps/rhino-cli-go/internal/domain apps/rhino-cli-go/internal/application apps/rhino-cli-go/internal/adapter`
      and `ls apps/rhino-cli-rust/src/domain apps/rhino-cli-rust/src/application apps/rhino-cli-rust/src/infrastructure`
      — acceptance: all listed (placeholders present); note any missing dir as a Phase-1 prerequisite.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` exits 0 (GREEN baseline recorded).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust`
      all green; both `test:quick` ≥90%; baseline counts recorded; zero preexisting failures unresolved.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` exits 0.

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no feature
> work exists yet. Safe to stop indefinitely. To resume: re-run the shadow-diff and the
> run-many baseline command and confirm both are still GREEN.

---

## Phase 1: PILOT — Migrate the `git` feature (both languages) [PROOF GATE]

> Migrates the most-DI-mature feature first; proves the migration recipe end-to-end. `git`
> already injects IO via `Deps` in both languages, so this phase formalizes that into named
> consumer-owned ports.

### 1a. Go `git` slice

- [ ] [AI] Confirm shadow-diff GREEN before any move:
      `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — acceptance: exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the git pre-commit use case in
      `apps/rhino-cli-go/internal/application/git/run_test.go` (new file; sibling pattern:
      existing `apps/rhino-cli-go/internal/git/runner_test.go`) asserting the use case calls
      injected `StagedFileProvider`/`ToolProber` ports — command:
      `npx nx run rhino-cli-go:test:unit` — acceptance: fails (port types undefined).
  - _Suggested executor: `swe-golang-dev`_
- [ ] [AI] **GREEN**: define domain-role ports (`StagedFileProvider`, `ToolProber`, and one
      named port per remaining `Deps` field) in `apps/rhino-cli-go/internal/application/git/ports.go`
      (new file) and the use case in `internal/application/git/run.go`; move pure logic into
      `apps/rhino-cli-go/internal/domain/git/` — command: `npx nx run rhino-cli-go:test:unit`
      — acceptance: new test passes; no other Go test broken.
  - _Suggested executor: `swe-golang-dev`_
- [ ] [AI] **GREEN**: implement outbound adapters under `apps/rhino-cli-go/internal/adapter/`
      (e.g. `internal/adapter/git/staged_files.go`) backing each port with the existing
      `DefaultDeps` implementations; wire them in `apps/rhino-cli-go/cmd/git_pre_commit.go`
      [Repo-grounded — file exists] — command: `npx nx run rhino-cli-go:test:integration`
      — acceptance: integration tests pass.
  - _Suggested executor: `swe-golang-dev`_
- [ ] [AI] **REFACTOR**: remove the now-redundant `Deps` struct from
      `apps/rhino-cli-go/internal/git/runner.go` (or reduce the package to adapter shims) and
      confirm `internal/domain/git/` imports no IO package — command:
      `npx nx run rhino-cli-go:typecheck` and `grep -rEn "os/exec|\"os\"|\"io\"|net|path/filepath" apps/rhino-cli-go/internal/domain/git/`
      — acceptance: `go vet` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-golang-dev`_

### 1b. Rust `git` slice

- [ ] [AI] **RED**: add a failing fake-backed unit test for the git pre-commit use case in
      `apps/rhino-cli-rust/src/application/git/run.rs` (`#[cfg(test)]` module; sibling pattern:
      existing tests in `apps/rhino-cli-rust/src/internal/git/runner.rs`) asserting the use case
      calls injected `Box<dyn StagedFileProvider>`/`Box<dyn ToolProber>` — command:
      `npx nx run rhino-cli-rust:test:unit` — acceptance: fails to compile (traits undefined).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: define domain-role port traits in
      `apps/rhino-cli-rust/src/application/git/ports.rs` (one trait per `Deps<'a>` field
      [Repo-grounded — `runner.rs` lines 56–76]) and the use case in
      `src/application/git/run.rs`; move pure logic into `apps/rhino-cli-rust/src/domain/git/`
      — command: `npx nx run rhino-cli-rust:test:unit` — acceptance: new test passes.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: implement `Box<dyn Trait>` adapters in
      `apps/rhino-cli-rust/src/infrastructure/git/` backing each trait with the existing
      production closures; wire at the dispatch point in `apps/rhino-cli-rust/src/commands/git.rs`
      [Repo-grounded — file exists] — command: `npx nx run rhino-cli-rust:test:integration`
      — acceptance: cucumber tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce `apps/rhino-cli-rust/src/internal/git/runner.rs` to adapter shims
      (or remove `Deps<'a>`); confirm `src/domain/git/` imports no IO module — command:
      `npx nx run rhino-cli-rust:lint` and
      `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli-rust/src/domain/git/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli-rust/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the entries
      `internal/git/runner\.rs` and `internal/git/root\.rs` to their new relocated paths (or
      remove them if the relocated domain/application files are now directly unit-tested)
      — command: `npx nx run rhino-cli-rust:test:quick` — acceptance: coverage ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck` — exits 0.
- [ ] [AI] `npx nx affected -t lint` — exits 0.
- [ ] [AI] `npx nx affected -t test:quick` — exits 0; both apps ≥90%.
- [ ] [AI] `npx nx affected -t spec-coverage` — exits 0.
- [ ] [AI] Fix ALL failures found — including preexisting issues not caused by these changes.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes (root-cause orientation). Commit preexisting fixes separately with their own
> Conventional Commit messages.

### Commit Guidelines (applies to every phase)

- [ ] [AI] Commit thematically; Conventional Commits `refactor(rhino-cli): …` /
      `test(rhino-cli): …`; split Go and Rust into separate commits; preexisting fixes get
      their own commits.

### Post-Push Verification (push mode per D3)

- [ ] [AI] Push per the D3 decision (default: `git push origin HEAD:main`).
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push; verify all green; fix and
      re-push until green. Do NOT start Phase 2 until CI is fully green.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (GREEN, unchanged corpus).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick --projects=rhino-cli-go,rhino-cli-rust`
      — all green; both `test:quick` ≥90%.
- [ ] [AI] `npx nx run-many -t lint typecheck --projects=rhino-cli-go,rhino-cli-rust` — all exit 0.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] `git` `Deps` is replaced by named consumer-owned ports in BOTH languages; both
      `domain/git/` dirs contain zero IO imports (grep confirmed).

> **Pause Safety**: the `git` feature is fully migrated and behavior-identical; all other
> features remain on the old shape (compiling, green). Safe to stop indefinitely. To resume:
> re-run the shadow-diff + run-many gate commands and confirm GREEN, then begin Phase 2.

---

## Phase 2: Rust shared kernel `cliout` (Rust-only) [DEPENDENCY ROOT]

> Rust-only shared kernel; migrate before its consumers (`doctor`, `envbackup`, `mermaid`). Go
> has **no** `cliout` kernel — **Go is a no-op for this phase**.

- [ ] [AI] Confirm shadow-diff GREEN before moves: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test asserting the relocated `cliout` kernel
      API in `apps/rhino-cli-rust/src/domain/shared/cliout/` (new module; `#[cfg(test)]`; sibling
      pattern: existing `cliout` tests in `apps/rhino-cli-rust/src/internal/cliout/`) — command:
      `npx nx run rhino-cli-rust:test:unit` — acceptance: fails to compile (module not yet relocated).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: move the pure `cliout` logic into `apps/rhino-cli-rust/src/domain/shared/cliout/`
      (Rust-only kernel; 3 consumers: `doctor`, `envbackup`, `mermaid`); update the three consumers'
      `use` paths to the relocated kernel — command: `npx nx run rhino-cli-rust:test:unit`
      — acceptance: new test passes; consumers import the relocated kernel; no other Rust test broken.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce the old `apps/rhino-cli-rust/src/internal/cliout/` to a shim or
      remove it; confirm `src/domain/shared/cliout/` imports no IO module — command:
      `npx nx run rhino-cli-rust:lint` and
      `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli-rust/src/domain/shared/cliout/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli-rust/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update any
      `internal/cliout/…` entry to its relocated `domain/shared/cliout/…` path (or remove it if the
      relocated file is now directly unit-tested) — command: `npx nx run rhino-cli-rust:test:quick`
      — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Go no-op confirmation: Go has no `cliout` kernel — record "Go: no change this phase" in
      this checklist — acceptance: note present; no Go files touched.

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; both apps ≥90%.
- [ ] [AI] Fix ALL failures (root-cause orientation), including preexisting.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green; do not proceed until CI green.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust` — all green; both ≥90%.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] Rust `cliout` lives in `src/domain/shared/cliout/`, IO-free (grep); its 3 consumers import the relocated kernel.

> **Pause Safety**: Rust `cliout` kernel relocated; all other features unchanged and green. Safe
> to stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 3.

---

## Phase 3: `mermaid` shared kernel (both languages) [DEPENDENCY ROOT]

> Shared kernel in BOTH languages (dependency constraint: `mermaid` is imported by `docs` and
> `git`). Migrate before `docs`.

- [ ] [AI] Confirm shadow-diff GREEN before moves: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the relocated `mermaid` kernel API —
      Go in `apps/rhino-cli-go/internal/domain/shared/mermaid/mermaid_test.go` (new file) and Rust
      `#[cfg(test)]` in `apps/rhino-cli-rust/src/domain/shared/mermaid/` (sibling pattern: existing
      `internal/mermaid` tests) — commands: `npx nx run rhino-cli-go:test:unit` /
      `npx nx run rhino-cli-rust:test:unit` — acceptance: both fail (kernel not yet relocated).
  - _Suggested executor: `swe-golang-dev` (Go) / `swe-rust-dev` (Rust)_
- [ ] [AI] **GREEN**: move `mermaid` into the shared kernel: Go → `apps/rhino-cli-go/internal/domain/shared/mermaid/`;
      Rust → `apps/rhino-cli-rust/src/domain/shared/mermaid/`; update `docs` and `git` consumers to
      import the relocated kernel — commands: `npx nx run rhino-cli-go:test:unit` /
      `npx nx run rhino-cli-rust:test:unit` — acceptance: both green; `docs` and `git` import the relocated kernel.
  - _Suggested executor: `swe-golang-dev` (Go) / `swe-rust-dev` (Rust)_
- [ ] [AI] **REFACTOR**: reduce/remove the old `internal/mermaid` packages; confirm both
      `domain/shared/mermaid/` dirs import no IO — commands: `npx nx run rhino-cli-go:typecheck` /
      `npx nx run rhino-cli-rust:lint` and
      `grep -rEn "os/exec|\"os\"|\"io\"|net|path/filepath" apps/rhino-cli-go/internal/domain/shared/mermaid/`
      plus `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli-rust/src/domain/shared/mermaid/`
      — acceptance: `go vet` exits 0; clippy `-D warnings` exits 0; both greps return no IO imports.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli-rust/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update any
      `internal/mermaid/…` entry to its relocated `domain/shared/mermaid/…` path (or remove if now
      directly unit-tested) — command: `npx nx run rhino-cli-rust:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; both ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust` — green; both ≥90%.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] `mermaid` lives in `domain/shared/mermaid/` in BOTH languages, IO-free (grep); `docs` and `git` import the relocated kernel.

> **Pause Safety**: `mermaid` kernel relocated in both languages; remaining features green. Safe
> to stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 4.

---

## Phase 4: `naming` (both languages, feature-local) [DEPENDENCY ROOT]

> Feature-local (single consumer — NOT a kernel). Migrate before `agents`, which consumes
> `naming` in Rust.

- [ ] [AI] Confirm shadow-diff GREEN: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the relocated `naming` use case — Go in
      `apps/rhino-cli-go/internal/application/naming/` (new test file) and Rust `#[cfg(test)]` in
      `apps/rhino-cli-rust/src/application/naming/` (sibling pattern: existing `internal/naming`
      tests) — commands: `npx nx run rhino-cli-go:test:unit` / `npx nx run rhino-cli-rust:test:unit`
      — acceptance: both fail (slice not yet present).
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `naming` (both langs, feature-local) into `domain/naming/` +
      `application/naming/`; port any IO seam; inbound stays the existing command entry —
      commands: `npx nx run rhino-cli-go:test:unit` / `npx nx run rhino-cli-rust:test:unit` and
      `npx nx run rhino-cli-go:validate:naming-agents` (and Rust equivalent if present) —
      acceptance: green; `agents` (Rust) imports the relocated `naming`.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove the old `internal/naming`; confirm both `domain/naming/`
      dirs import no IO — commands: `npx nx run rhino-cli-go:typecheck` / `npx nx run rhino-cli-rust:lint`
      and `grep -rEn "os/exec|\"os\"|\"io\"|net|path/filepath" apps/rhino-cli-go/internal/domain/naming/`
      plus `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli-rust/src/domain/naming/`
      — acceptance: `go vet` exits 0; clippy `-D warnings` exits 0; both greps return no IO imports.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli-rust/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update any
      `internal/naming/…` entry to its relocated `domain/naming/…` or `application/naming/…` path
      (or remove if now directly unit-tested) — command: `npx nx run rhino-cli-rust:test:quick`
      — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; both ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust` — green; both ≥90%.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] `naming` lives in `domain/naming/` + `application/naming/` in BOTH languages, IO-free (grep).

> **Pause Safety**: `naming` migrated in both languages; remaining features green. Safe to stop.
> To resume: re-run the gate commands and confirm GREEN, then begin Phase 5.

---

## Phase 5: `docs` (both languages; depends on `mermaid`)

- [ ] [AI] Confirm shadow-diff GREEN: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `docs` use case — Go in
      `apps/rhino-cli-go/internal/application/docs/` and Rust `#[cfg(test)]` in
      `apps/rhino-cli-rust/src/application/docs/` (sibling pattern: existing `internal/docs` tests)
      — commands: `npx nx run rhino-cli-go:test:unit` / `npx nx run rhino-cli-rust:test:unit`
      — acceptance: both fail (ports/slice not yet present).
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `docs` (both langs) into `domain/docs/`, `application/docs/`, adapters
      (Go `internal/adapter/docs/` / Rust `src/infrastructure/docs/`); `docs` imports the relocated
      `mermaid` kernel; inbound stays `cmd/docs_*.go` / `src/commands/docs.rs` [Repo-grounded —
      files exist] — commands: `test:unit`, `test:integration` both apps — acceptance: green.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/docs`; confirm both `domain/docs/` dirs are
      IO-free — commands: `npx nx run rhino-cli-go:typecheck` / `npx nx run rhino-cli-rust:lint` and
      `grep -rEn "os/exec|\"os\"|\"io\"|net|path/filepath" apps/rhino-cli-go/internal/domain/docs/`
      plus `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli-rust/src/domain/docs/`
      — acceptance: `go vet` exits 0; clippy `-D warnings` exits 0; both greps return no IO imports.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli-rust/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the `commands/docs\.rs`
      entry (and any relocated `internal/docs` file) to its new path — command:
      `npx nx run rhino-cli-rust:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; both ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust` — green; both ≥90%.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] `docs` slice complete in BOTH languages, domain IO-free (grep); imports the relocated `mermaid` kernel.

> **Pause Safety**: `docs` migrated in both languages; remaining features green. Safe to stop.
> To resume: re-run the gate commands and confirm GREEN, then begin Phase 6.

---

## Phase 6: `doctor` (both languages; depends on Rust `cliout`)

- [ ] [AI] Confirm shadow-diff GREEN: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `doctor` use case (assert injected
      `ToolProber`-style ports) — Go in `apps/rhino-cli-go/internal/application/doctor/` and Rust
      `#[cfg(test)]` in `apps/rhino-cli-rust/src/application/doctor/` (sibling pattern: existing
      `internal/doctor` tests) — commands: `npx nx run rhino-cli-go:test:unit` /
      `npx nx run rhino-cli-rust:test:unit` — acceptance: both fail (ports not yet defined).
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `doctor` (both langs) to slices; extract a `ToolProber`-style port per
      probed tool seam (doctor shells out heavily); Rust `doctor` imports the relocated `cliout`
      kernel; inbound stays `cmd/doctor.go` / `src/commands/doctor.rs` [Repo-grounded — files exist]
      — commands: `test:unit`, `test:integration` both apps — acceptance: green.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/doctor`; confirm both `domain/doctor/` dirs are
      IO-free — commands: `npx nx run rhino-cli-go:typecheck` / `npx nx run rhino-cli-rust:lint` and
      `grep -rEn "os/exec|\"os\"|\"io\"|net|path/filepath" apps/rhino-cli-go/internal/domain/doctor/`
      plus `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli-rust/src/domain/doctor/`
      — acceptance: `go vet` exits 0; clippy `-D warnings` exits 0; both greps return no IO imports.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli-rust/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the `commands/doctor\.rs`
      entry (and any relocated `internal/doctor` file) to its new path — command:
      `npx nx run rhino-cli-rust:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; both ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust` — green; both ≥90%.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] `doctor` slice complete in BOTH languages, domain IO-free (grep); Rust `doctor` imports the relocated `cliout` kernel.

> **Pause Safety**: `doctor` migrated in both languages; remaining features green. Safe to stop.
> To resume: re-run the gate commands and confirm GREEN, then begin Phase 7.

---

## Phase 7: `envbackup` / `env` (both languages; depends on Rust `cliout`)

- [ ] [AI] Confirm shadow-diff GREEN: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `envbackup` use case (assert
      injected file-read/write + exec ports) — Go in `apps/rhino-cli-go/internal/application/envbackup/`
      and Rust `#[cfg(test)]` in `apps/rhino-cli-rust/src/application/env/` (sibling pattern: existing
      `internal/envbackup` tests) — commands: `npx nx run rhino-cli-go:test:unit` /
      `npx nx run rhino-cli-rust:test:unit` — acceptance: both fail (ports not yet defined).
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `envbackup` (both langs) to slices; extract one named port per IO seam
      (file read/write, exec); Rust `env` imports the relocated `cliout` kernel; inbound stays
      `cmd/env_*.go` / `src/commands/env.rs` [Repo-grounded — files exist] — commands: `test:unit`,
      `test:integration` both apps — acceptance: green.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/envbackup`; confirm both domain dirs are IO-free
      — commands: `npx nx run rhino-cli-go:typecheck` / `npx nx run rhino-cli-rust:lint` and
      `grep -rEn "os/exec|\"os\"|\"io\"|net|path/filepath" apps/rhino-cli-go/internal/domain/envbackup/`
      plus `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli-rust/src/domain/env/`
      — acceptance: `go vet` exits 0; clippy `-D warnings` exits 0; both greps return no IO imports.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli-rust/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the `commands/env\.rs`
      entry (and any relocated `internal/envbackup` file) to its new path — command:
      `npx nx run rhino-cli-rust:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; both ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 7 Gate

> All checks below must pass before starting Phase 8.

- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust` — green; both ≥90%.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] `envbackup`/`env` slice complete in BOTH languages, domain IO-free (grep); Rust `env` imports the relocated `cliout` kernel.

> **Pause Safety**: `envbackup`/`env` migrated in both languages; remaining features green. Safe
> to stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 8.

---

## Phase 8: `testcoverage` (both languages; file + exec; git-diff/merge paths)

- [ ] [AI] Confirm shadow-diff GREEN: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `testcoverage` use case (assert
      injected `CoverageReader`-style ports for read/diff/merge) — Go in
      `apps/rhino-cli-go/internal/application/testcoverage/` and Rust `#[cfg(test)]` in
      `apps/rhino-cli-rust/src/application/testcoverage/` (sibling pattern: existing
      `internal/testcoverage` tests) — commands: `npx nx run rhino-cli-go:test:unit` /
      `npx nx run rhino-cli-rust:test:unit` — acceptance: both fail (ports not yet defined).
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `testcoverage` (both langs) to slices; extract `CoverageReader`-style
      ports for the file-read/diff/merge seams; inbound stays `cmd/test_coverage.go`,
      `cmd/test_coverage_diff.go`, `cmd/test_coverage_merge.go` /
      `src/commands/testcoverage.rs` [Repo-grounded — files exist] — commands: `test:unit`,
      `test:integration` both apps — acceptance: green.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/testcoverage`; confirm both domain dirs are
      IO-free — commands: `npx nx run rhino-cli-go:typecheck` / `npx nx run rhino-cli-rust:lint` and
      `grep -rEn "os/exec|\"os\"|\"io\"|net|path/filepath" apps/rhino-cli-go/internal/domain/testcoverage/`
      plus `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli-rust/src/domain/testcoverage/`
      — acceptance: `go vet` exits 0; clippy `-D warnings` exits 0; both greps return no IO imports.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli-rust/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the entries
      `commands/testcoverage\.rs`, `internal/testcoverage/diff\.rs`, and
      `internal/testcoverage/merge\.rs` to their relocated paths (or remove any whose relocated file
      is now directly unit-tested) — command: `npx nx run rhino-cli-rust:test:quick`
      — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; both ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 8 Gate

> All checks below must pass before starting Phase 9.

- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust` — green; both ≥90%.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] `testcoverage` slice complete in BOTH languages, domain IO-free (grep); all three Rust allowlist entries updated.

> **Pause Safety**: `testcoverage` migrated in both languages; remaining features green. Safe to
> stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 9.

---

## Phase 9: `agents` (both languages; depends on `naming`)

- [ ] [AI] Confirm shadow-diff GREEN: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `agents` use case (assert injected
      file-scan + exec ports) — Go in `apps/rhino-cli-go/internal/application/agents/` and Rust
      `#[cfg(test)]` in `apps/rhino-cli-rust/src/application/agents/` (sibling pattern: existing
      `internal/agents` tests) — commands: `npx nx run rhino-cli-go:test:unit` /
      `npx nx run rhino-cli-rust:test:unit` — acceptance: both fail (ports not yet defined).
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `agents` (both langs) to slices; ports for file-scan + exec seams;
      Rust `agents` imports the relocated `naming`; inbound stays `cmd/agents_*.go` /
      `src/commands/agents.rs` [Repo-grounded — files exist] — commands: `test:unit`,
      `test:integration` both apps — acceptance: green.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/agents`; confirm both `domain/agents/` dirs are
      IO-free — commands: `npx nx run rhino-cli-go:typecheck` / `npx nx run rhino-cli-rust:lint` and
      `grep -rEn "os/exec|\"os\"|\"io\"|net|path/filepath" apps/rhino-cli-go/internal/domain/agents/`
      plus `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli-rust/src/domain/agents/`
      — acceptance: `go vet` exits 0; clippy `-D warnings` exits 0; both greps return no IO imports.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli-rust/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the `commands/agents\.rs`
      entry (and any relocated `internal/agents` file) to its new path — command:
      `npx nx run rhino-cli-rust:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; both ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 9 Gate

> All checks below must pass before starting Phase 10.

- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust` — green; both ≥90%.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] `agents` slice complete in BOTH languages, domain IO-free (grep); Rust `agents` imports the relocated `naming`.

> **Pause Safety**: `agents` migrated in both languages; remaining features green. Safe to stop.
> To resume: re-run the gate commands and confirm GREEN, then begin Phase 10.

---

## Phase 10: `contracts` (both languages)

- [ ] [AI] Confirm shadow-diff GREEN: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `contracts` use case (assert
      injected scaffold file-write + exec ports) — Go in `apps/rhino-cli-go/internal/application/contracts/`
      and Rust `#[cfg(test)]` in `apps/rhino-cli-rust/src/application/contracts/` (sibling pattern:
      existing `internal/contracts` tests) — commands: `npx nx run rhino-cli-go:test:unit` /
      `npx nx run rhino-cli-rust:test:unit` — acceptance: both fail (ports not yet defined).
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `contracts` (both langs) to slices; ports for scaffold file-write +
      exec seams; inbound stays `cmd/contracts_*.go` / `src/commands/contracts.rs` [Repo-grounded —
      files exist] — commands: `test:unit`, `test:integration` both apps — acceptance: green.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/contracts`; confirm both `domain/contracts/`
      dirs are IO-free — commands: `npx nx run rhino-cli-go:typecheck` / `npx nx run rhino-cli-rust:lint`
      and `grep -rEn "os/exec|\"os\"|\"io\"|net|path/filepath" apps/rhino-cli-go/internal/domain/contracts/`
      plus `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli-rust/src/domain/contracts/`
      — acceptance: `go vet` exits 0; clippy `-D warnings` exits 0; both greps return no IO imports.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli-rust/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the `commands/contracts\.rs`
      entry (and any relocated `internal/contracts` file) to its new path — command:
      `npx nx run rhino-cli-rust:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; both ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 10 Gate

> All checks below must pass before starting Phase 11.

- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust` — green; both ≥90%.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] `contracts` slice complete in BOTH languages, domain IO-free (grep).

> **Pause Safety**: `contracts` migrated in both languages; remaining features green. Safe to
> stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 11.

---

## Phase 11: `java` (both languages)

- [ ] [AI] Confirm shadow-diff GREEN: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `java` use case (assert injected
      file-scan port) — Go in `apps/rhino-cli-go/internal/application/java/` and Rust `#[cfg(test)]`
      in `apps/rhino-cli-rust/src/application/java/` (sibling pattern: existing `internal/java`
      tests) — commands: `npx nx run rhino-cli-go:test:unit` / `npx nx run rhino-cli-rust:test:unit`
      — acceptance: both fail (port not yet defined).
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `java` (both langs) to slices; port the file-scan seam; inbound stays
      `cmd/java_*.go` / `src/commands/java.rs` [Repo-grounded] — commands: `test:unit` both apps
      — acceptance: green.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/java`; confirm both `domain/java/` dirs are
      IO-free — commands: `npx nx run rhino-cli-go:typecheck` / `npx nx run rhino-cli-rust:lint` and
      `grep -rEn "os/exec|\"os\"|\"io\"|net|path/filepath" apps/rhino-cli-go/internal/domain/java/`
      plus `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli-rust/src/domain/java/`
      — acceptance: `go vet` exits 0; clippy `-D warnings` exits 0; both greps return no IO imports.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli-rust/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the `commands/java\.rs`
      entry (and any relocated `internal/java` file) to its new path — command:
      `npx nx run rhino-cli-rust:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; both ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 11 Gate

> All checks below must pass before starting Phase 12.

- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust` — green; both ≥90%.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] `java` slice complete in BOTH languages, domain IO-free (grep).

> **Pause Safety**: `java` migrated in both languages; remaining features green. Safe to stop.
> To resume: re-run the gate commands and confirm GREEN, then begin Phase 12.

---

## Phase 12: `repo-governance` (both languages)

> Rust logic currently lives in `commands/` only [Repo-grounded — Rust `repo_governance` logic in
>
> > `commands/`]; extract the use case into `application/`, pure rules into `domain/`, keep a thin
> > inbound shim. Validate with the governance gates.

- [ ] [AI] Confirm shadow-diff GREEN: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `repo-governance` use case (assert
      injected file-scan port; pure rule logic separated) — Go in
      `apps/rhino-cli-go/internal/application/repogovernance/` and Rust `#[cfg(test)]` in
      `apps/rhino-cli-rust/src/application/repo_governance/` (sibling pattern: existing
      `commands/repo_governance.rs` tests) — commands: `npx nx run rhino-cli-go:test:unit` /
      `npx nx run rhino-cli-rust:test:unit` — acceptance: both fail (use case not yet extracted).
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **GREEN**: extract the vendor-audit + gherkin-cardinality use cases into `application/`,
      pure rule logic into `domain/`; thin inbound shim stays `cmd/governance_*.go` /
      `src/commands/repo_governance.rs` [Repo-grounded] — commands: `test:unit`,
      `npx nx run rhino-cli-go:validate:repo-governance-vendor-audit`,
      `npx nx run rhino-cli-go:validate:gherkin-keyword-cardinality` (and Rust equivalents) both apps
      — acceptance: green.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce the inbound to a thin shim; confirm both `domain/` rule dirs are
      IO-free — commands: `npx nx run rhino-cli-go:typecheck` / `npx nx run rhino-cli-rust:lint` and
      `grep -rEn "os/exec|\"os\"|\"io\"|net|path/filepath" apps/rhino-cli-go/internal/domain/repogovernance/`
      plus `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli-rust/src/domain/repo_governance/`
      — acceptance: `go vet` exits 0; clippy `-D warnings` exits 0; both greps return no IO imports.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli-rust/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update any
      `commands/repo_governance\.rs` entry to its relocated path (or remove if now directly
      unit-tested) — command: `npx nx run rhino-cli-rust:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; both ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 12 Gate

> All checks below must pass before starting Phase 13.

- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust` — green; both ≥90%.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` and `:validate:gherkin-keyword-cardinality` — exit 0.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] `repo-governance` use case in `application/`, rules in `domain/` (IO-free, grep), thin inbound shim — BOTH languages.

> **Pause Safety**: `repo-governance` migrated in both languages; remaining features green. Safe
> to stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 13.

---

## Phase 13: `speccoverage` (both languages)

- [ ] [AI] Confirm shadow-diff GREEN: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `speccoverage` use case (assert
      injected gherkin/impl on-disk scan port) — Go in `apps/rhino-cli-go/internal/application/speccoverage/`
      and Rust `#[cfg(test)]` in `apps/rhino-cli-rust/src/application/speccoverage/` (sibling pattern:
      existing `internal/speccoverage` tests) — commands: `npx nx run rhino-cli-go:test:unit` /
      `npx nx run rhino-cli-rust:test:unit` — acceptance: both fail (port not yet defined).
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `speccoverage` (both langs) to slices; port the on-disk gherkin/impl
      scan seam; inbound stays `cmd/spec_coverage*.go` / `src/commands/speccoverage.rs` [Repo-grounded]
      — commands: `test:unit` both apps — acceptance: green.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/speccoverage`; confirm both domain dirs are
      IO-free — commands: `npx nx run rhino-cli-go:typecheck` / `npx nx run rhino-cli-rust:lint` and
      `grep -rEn "os/exec|\"os\"|\"io\"|net|path/filepath" apps/rhino-cli-go/internal/domain/speccoverage/`
      plus `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli-rust/src/domain/speccoverage/`
      — acceptance: `go vet` exits 0; clippy `-D warnings` exits 0; both greps return no IO imports.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli-rust/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the `commands/speccoverage\.rs`
      entry (and any relocated `internal/speccoverage` file) to its new path — command:
      `npx nx run rhino-cli-rust:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; both ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 13 Gate

> All checks below must pass before starting Phase 14.

- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust` — green; both ≥90%.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] `speccoverage` slice complete in BOTH languages, domain IO-free (grep).

> **Pause Safety**: `speccoverage` migrated in both languages; remaining features green. Safe to
> stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 14.

---

## Phase 14: `workflows` (both languages)

> Go logic currently lives in `cmd/` only [Repo-grounded — `workflows` has Go logic in `cmd/`
>
> > only]; move the use case into `application/`/`domain/`, keep a thin inbound shim. Validate with
> > `validate:naming-workflows`.

- [ ] [AI] Confirm shadow-diff GREEN: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `workflows` use case — Go in
      `apps/rhino-cli-go/internal/application/workflows/` and Rust `#[cfg(test)]` in
      `apps/rhino-cli-rust/src/application/workflows/` (sibling pattern: existing `workflows` tests)
      — commands: `npx nx run rhino-cli-go:test:unit` / `npx nx run rhino-cli-rust:test:unit`
      — acceptance: both fail (use case not yet extracted).
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `workflows` (both langs); move the Go use case from `cmd/` into
      `application/`/`domain/`, keep a thin inbound shim; Rust to slices — commands: `test:unit`,
      `npx nx run rhino-cli-go:validate:naming-workflows` (and Rust equivalent if present) both apps
      — acceptance: green.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce the inbound to a thin shim; confirm both `domain/workflows/` dirs
      are IO-free — commands: `npx nx run rhino-cli-go:typecheck` / `npx nx run rhino-cli-rust:lint`
      and `grep -rEn "os/exec|\"os\"|\"io\"|net|path/filepath" apps/rhino-cli-go/internal/domain/workflows/`
      plus `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli-rust/src/domain/workflows/`
      — acceptance: `go vet` exits 0; clippy `-D warnings` exits 0; both greps return no IO imports.
  - _Suggested executor: `swe-golang-dev` / `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli-rust/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update any relocated Rust
      `workflows` entry to its new path (or remove if now directly unit-tested) — command:
      `npx nx run rhino-cli-rust:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; both ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 14 Gate

> All checks below must pass before starting Phase 15.

- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust` — green; both ≥90%.
- [ ] [AI] `npx nx run rhino-cli-go:validate:naming-workflows` — exits 0.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] `workflows` use case in `application/`/`domain/` (IO-free, grep), thin inbound shim — BOTH languages.
- [ ] [AI] ALL 13 features now live in hexagonal slices in BOTH languages; every `domain/<feature>/`
      and `domain/shared/` dir is IO-free (grep across both apps returns no IO imports under domain).

> **Pause Safety**: the full structural migration is complete and behavior-identical in both
> binaries. Safe to stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 15.

---

## Phase 15: Update the hexagonal-CLI convention doc (vendor-neutral)

> Final phase. Documents the realized architecture. Must stay vendor-neutral (no Claude/OpenCode
> /vendor terms); the vendor-audit gate enforces this. Do NOT reference an arch lint (none added).

- [ ] [AI] Update `repo-governance/development/pattern/hexagonal-architecture-cli.md`
      [Repo-grounded — file exists] to document: (a) the hybrid `domain/shared` kernel +
      per-feature vertical-slice layout; (b) the maximal-ports requirement (every IO boundary is a
      named port) with the one-line accepted trade-off vs. the lean approach; (c) the domain-role
      port-naming rule (`StagedFileProvider`, not `FileSystem`); (d) the 2+-consumer shared-kernel
      rule with the accepted Go/Rust asymmetry (`{mermaid}` vs `{mermaid, cliout}`) — acceptance:
      the four topics are present; no vendor terms; no arch-lint reference.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Render-check any added Mermaid diagram and heading hierarchy:
      `npx nx run rhino-cli-go:validate:mermaid` and `npx nx run rhino-cli-go:validate:heading-hierarchy`
      — acceptance: both exit 0.
- [ ] [AI] Vendor-neutrality + link checks:
      `npx nx run rhino-cli-go:validate:repo-governance-vendor-audit` and
      `npx nx run rhino-cli-go:validate:links` — acceptance: both exit 0.

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t lint typecheck` — exits 0. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 15 Gate

> All checks below must pass before final verification.

- [ ] [AI] `npx nx run rhino-cli-go:validate:repo-governance-vendor-audit` — exits 0 (vendor-neutral).
- [ ] [AI] `npx nx run rhino-cli-go:validate:links` and `:validate:heading-hierarchy` — exit 0.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — exits 0.
- [ ] [AI] The convention doc documents all four required topics (layout, maximal ports + trade-off,
      domain-role naming, 2+-consumer kernel rule) and references no arch lint.

> **Pause Safety**: migration + documentation complete; repo coherent and green. Safe to stop.
> To resume: proceed to final verification and archival.

---

## Final Verification

```gherkin
Scenario: The migration is complete and behavior-preserving
  Given all 13 features are migrated to hexagonal slices in both Go and Rust
  When the full gate suite runs across both apps
  Then shadow-diff, test:unit, test:integration, test:quick (>=90%), lint, typecheck, and validate:cross-vendor-parity all pass
  And the convention doc records the architecture vendor-neutrally
```

- [ ] [AI] Full sweep: `npx nx run-many -t build test:unit test:integration test:quick lint typecheck --projects=rhino-cli-go,rhino-cli-rust`
      and `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` and
      `npx nx run rhino-cli-rust:validate:cross-vendor-parity` — acceptance: all GREEN; both apps ≥90%.

## Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI) and ALL phase gates were green.
- [ ] [AI] Verify shadow-diff GREEN at completion.
- [ ] [AI] Move: `git mv plans/in-progress/migrate-rhino-cli-to-hexagonal plans/done/2026-06-09__migrate-rhino-cli-to-hexagonal`
      (use the actual completion date if later) — acceptance: folder relocated under `plans/done/`.
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date.
- [ ] [AI] Update any other READMEs that reference this plan (e.g. `plans/README.md`).
- [ ] [AI] Commit the archival: `chore(plans): move migrate-rhino-cli-to-hexagonal to done`.
