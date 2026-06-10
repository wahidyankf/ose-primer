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
golden corpus is therefore **unchanged**, and the golden-master CLI suite is compared against the
**EXISTING Phase 0 baseline throughout the whole plan**. There is **no re-baselining anywhere**.

O1/O2/O3 below still occur, but **ONLY as byte-neutral port extractions** — they relocate IO
writes behind named ports without changing a single emitted byte. The candidate table is
retained purely for traceability; **none of these candidates introduces a visible change, and
none requires re-capturing the golden corpus.**

| #   | Command / surface                       | Current output (observed)                                                                        | Byte-neutral seam extraction (NO visible change)                                             | Why it aids layering                                                  |
| --- | --------------------------------------- | ------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| O1  | `--say` with `--verbose` (root)         | `[<ts>] INFO: Executing say command` then `[<ts>] INFO: Message: <msg>`                          | Route the two INFO lines through a domain-role `DiagnosticLogger` port — **identical bytes** | Forces the verbose-logging seam behind a port; **byte-neutral**       |
| O2  | `git pre-commit` step warnings          | `⚠️  Step %q timed out…` / `⚠️  Total pre-commit timeout reached…` [Repo-grounded — `runner.rs`] | Route emoji-warning writes through a `StepReporter` port — **identical bytes**               | Removes direct stdout writes from orchestration; pure seam extraction |
| O3  | Error prefix on failure (root dispatch) | `Error: %v` to stderr                                                                            | Centralize through an inbound-adapter error presenter — **identical bytes**                  | Makes the inbound adapter the single error-formatting point           |

> **Frozen-output rule (binding on every phase)**: O1/O2/O3 are byte-neutral seam extractions
> only. No phase introduces a visible text/format change, and no phase re-captures the golden
> corpus. Every phase compares the golden-master CLI suite against the existing Phase 0
> baseline and expects GREEN with a byte-for-byte unchanged corpus.

### D2. Phase grouping for the 13 features — RESOLVED: one phase per feature

**RESOLVED.** The user chose **Option B: one phase per feature** (smallest blast radius per
gate), rejecting the previously-drafted grouped Option A. The delivery checklist below
implements one feature per phase in dependency-respecting order (Phases 2–14), with the `git`
pilot at Phase 1 and the convention-doc update at Phase 15. Each feature phase carries the same
per-phase machinery the pilot uses: golden-master suite GREEN before + after, RED→GREEN→REFACTOR
where ports are extracted, a coverage-allowlist lockstep item for any relocated file, Local
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
      — acceptance: exits 0 with no unresolved drift (Rust toolchain present).
- [ ] [AI] Build the binary: `npx nx run rhino-cli:build`
      — acceptance: exits 0; `apps/rhino-cli/dist/rhino-cli` exists.
- [ ] [AI] Capture the golden-master baseline with the golden-master CLI suite
      — acceptance: exits 0 (GREEN) with zero divergence; record the run in this checklist.
- [ ] [AI] Establish test + coverage baseline on the app:
      `npx nx run-many -t test:unit test:integration test:quick --projects=rhino-cli`
      — acceptance: record pass/fail counts and coverage %; `test:quick` reports ≥90%.
- [ ] [AI] Establish lint + typecheck baseline:
      `npx nx run-many -t lint typecheck --projects=rhino-cli`
      — acceptance: all exit 0; document any preexisting failures.
- [ ] [AI] Resolve ALL preexisting failures before proceeding (root-cause orientation)
      — acceptance: no unresolved preexisting failures remain.
- [ ] [AI] Confirm the empty hex placeholder dirs exist:
      `ls apps/rhino-cli/src/domain apps/rhino-cli/src/application apps/rhino-cli/src/infrastructure`
      — acceptance: all listed (placeholders present); note any missing dir as a Phase-1 prerequisite.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
- [ ] [AI] The golden-master CLI suite exits 0 (GREEN baseline recorded).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli`
      all green; `test:quick` ≥90%; baseline counts recorded; zero preexisting failures unresolved.

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no feature
> work exists yet. Safe to stop indefinitely. To resume: re-run the golden-master CLI suite and the
> run-many baseline command and confirm both are still GREEN.

---

## Phase 1: PILOT — Migrate the `git` feature [PROOF GATE]

> Migrates the most-DI-mature feature first; proves the migration recipe end-to-end. `git`
> already injects IO via `Deps`, so this phase formalizes that into named consumer-owned ports.

### 1a. Rust `git` slice

- [ ] [AI] Confirm the golden-master CLI suite GREEN before any move — acceptance: exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the git pre-commit use case in
      `apps/rhino-cli/src/application/git/run.rs` (`#[cfg(test)]` module; sibling pattern:
      existing tests in `apps/rhino-cli/src/internal/git/runner.rs`) asserting the use case
      calls injected `Box<dyn StagedFileProvider>`/`Box<dyn ToolProber>` — command:
      `npx nx run rhino-cli:test:unit` — acceptance: fails to compile (traits undefined).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: define domain-role port traits in
      `apps/rhino-cli/src/application/git/ports.rs` (one trait per `Deps<'a>` field
      [Repo-grounded — `runner.rs` lines 56–76]) and the use case in
      `src/application/git/run.rs`; move pure logic into `apps/rhino-cli/src/domain/git/`
      — command: `npx nx run rhino-cli:test:unit` — acceptance: new test passes.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: implement `Box<dyn Trait>` adapters in
      `apps/rhino-cli/src/infrastructure/git/` backing each trait with the existing
      production closures; wire at the dispatch point in `apps/rhino-cli/src/commands/git.rs`
      [Repo-grounded — file exists] — command: `npx nx run rhino-cli:test:integration`
      — acceptance: cucumber tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce `apps/rhino-cli/src/internal/git/runner.rs` to adapter shims
      (or remove `Deps<'a>`); confirm `src/domain/git/` imports no IO module — command:
      `npx nx run rhino-cli:lint` and
      `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli/src/domain/git/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the entries
      `internal/git/runner\.rs` and `internal/git/root\.rs` to their new relocated paths (or
      remove them if the relocated domain/application files are now directly unit-tested)
      — command: `npx nx run rhino-cli:test:quick` — acceptance: coverage ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck` — exits 0.
- [ ] [AI] `npx nx affected -t lint` — exits 0.
- [ ] [AI] `npx nx affected -t test:quick` — exits 0; ≥90%.
- [ ] [AI] `npx nx affected -t spec-coverage` — exits 0.
- [ ] [AI] Fix ALL failures found — including preexisting issues not caused by these changes.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes (root-cause orientation). Commit preexisting fixes separately with their own
> Conventional Commit messages.

### Commit Guidelines (applies to every phase)

- [ ] [AI] Commit thematically; Conventional Commits `refactor(rhino-cli): …` /
      `test(rhino-cli): …`; preexisting fixes get their own commits.

### Post-Push Verification (push mode per D3)

- [ ] [AI] Push per the D3 decision (default: `git push origin HEAD:main`).
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push; verify all green; fix and
      re-push until green. Do NOT start Phase 2 until CI is fully green.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] The golden-master CLI suite exits 0 (GREEN, unchanged corpus).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick --projects=rhino-cli`
      — all green; `test:quick` ≥90%.
- [ ] [AI] `npx nx run-many -t lint typecheck --projects=rhino-cli` — all exit 0.
- [ ] [AI] `git` `Deps` is replaced by named consumer-owned ports; `domain/git/` contains zero IO
      imports (grep confirmed).

> **Pause Safety**: the `git` feature is fully migrated and behavior-identical; all other
> features remain on the old shape (compiling, green). Safe to stop indefinitely. To resume:
> re-run the golden-master suite + run-many gate commands and confirm GREEN, then begin Phase 2.

---

## Phase 2: shared kernel `cliout` [DEPENDENCY ROOT]

> Shared kernel; migrate before its consumers (`doctor`, `envbackup`, `mermaid`).

- [ ] [AI] Confirm the golden-master CLI suite GREEN before moves — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test asserting the relocated `cliout` kernel
      API in `apps/rhino-cli/src/domain/shared/cliout/` (new module; `#[cfg(test)]`; sibling
      pattern: existing `cliout` tests in `apps/rhino-cli/src/internal/cliout/`) — command:
      `npx nx run rhino-cli:test:unit` — acceptance: fails to compile (module not yet relocated).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: move the pure `cliout` logic into `apps/rhino-cli/src/domain/shared/cliout/`
      (3 consumers: `doctor`, `envbackup`, `mermaid`); update the three consumers'
      `use` paths to the relocated kernel — command: `npx nx run rhino-cli:test:unit`
      — acceptance: new test passes; consumers import the relocated kernel; no other Rust test broken.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce the old `apps/rhino-cli/src/internal/cliout/` to a shim or
      remove it; confirm `src/domain/shared/cliout/` imports no IO module — command:
      `npx nx run rhino-cli:lint` and
      `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli/src/domain/shared/cliout/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update any
      `internal/cliout/…` entry to its relocated `domain/shared/cliout/…` path (or remove it if the
      relocated file is now directly unit-tested) — command: `npx nx run rhino-cli:test:quick`
      — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; ≥90%.
- [ ] [AI] Fix ALL failures (root-cause orientation), including preexisting.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green; do not proceed until CI green.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] The golden-master CLI suite exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli` — all green; ≥90%.
- [ ] [AI] `cliout` lives in `src/domain/shared/cliout/`, IO-free (grep); its 3 consumers import the relocated kernel.

> **Pause Safety**: `cliout` kernel relocated; all other features unchanged and green. Safe
> to stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 3.

---

## Phase 3: `mermaid` shared kernel [DEPENDENCY ROOT]

> Shared kernel (dependency constraint: `mermaid` is imported by `docs` and `git`). Migrate
> before `docs`.

- [ ] [AI] Confirm the golden-master CLI suite GREEN before moves — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the relocated `mermaid` kernel API —
      `#[cfg(test)]` in `apps/rhino-cli/src/domain/shared/mermaid/` (sibling pattern: existing
      `internal/mermaid` tests) — command: `npx nx run rhino-cli:test:unit` — acceptance:
      fails (kernel not yet relocated).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: move `mermaid` into the shared kernel at
      `apps/rhino-cli/src/domain/shared/mermaid/`; update `docs` and `git` consumers to
      import the relocated kernel — command: `npx nx run rhino-cli:test:unit` — acceptance:
      green; `docs` and `git` import the relocated kernel.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove the old `internal/mermaid` module; confirm
      `domain/shared/mermaid/` imports no IO — command: `npx nx run rhino-cli:lint` and
      `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli/src/domain/shared/mermaid/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update any
      `internal/mermaid/…` entry to its relocated `domain/shared/mermaid/…` path (or remove if now
      directly unit-tested) — command: `npx nx run rhino-cli:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] The golden-master CLI suite exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli` — green; ≥90%.
- [ ] [AI] `mermaid` lives in `domain/shared/mermaid/`, IO-free (grep); `docs` and `git` import the relocated kernel.

> **Pause Safety**: `mermaid` kernel relocated; remaining features green. Safe
> to stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 4.

---

## Phase 4: `naming` (feature-local) [DEPENDENCY ROOT]

> Feature-local (single consumer — NOT a kernel). Migrate before `agents`, which consumes
> `naming`.

- [ ] [AI] Confirm the golden-master CLI suite GREEN — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the relocated `naming` use case —
      `#[cfg(test)]` in `apps/rhino-cli/src/application/naming/` (sibling pattern: existing
      `internal/naming` tests) — command: `npx nx run rhino-cli:test:unit`
      — acceptance: fails (slice not yet present).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `naming` (feature-local) into `domain/naming/` +
      `application/naming/`; port any IO seam; inbound stays the existing command entry —
      command: `npx nx run rhino-cli:test:unit` and the Rust naming-agents validate target if present —
      acceptance: green; `agents` imports the relocated `naming`.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove the old `internal/naming`; confirm `domain/naming/`
      imports no IO — command: `npx nx run rhino-cli:lint`
      and `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli/src/domain/naming/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update any
      `internal/naming/…` entry to its relocated `domain/naming/…` or `application/naming/…` path
      (or remove if now directly unit-tested) — command: `npx nx run rhino-cli:test:quick`
      — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] The golden-master CLI suite exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli` — green; ≥90%.
- [ ] [AI] `naming` lives in `domain/naming/` + `application/naming/`, IO-free (grep).

> **Pause Safety**: `naming` migrated; remaining features green. Safe to stop.
> To resume: re-run the gate commands and confirm GREEN, then begin Phase 5.

---

## Phase 5: `docs` (depends on `mermaid`)

- [ ] [AI] Confirm the golden-master CLI suite GREEN — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `docs` use case —
      `#[cfg(test)]` in `apps/rhino-cli/src/application/docs/` (sibling pattern: existing
      `internal/docs` tests) — command: `npx nx run rhino-cli:test:unit`
      — acceptance: fails (ports/slice not yet present).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `docs` into `domain/docs/`, `application/docs/`, adapters
      (`src/infrastructure/docs/`); `docs` imports the relocated `mermaid` kernel; inbound stays
      `src/commands/docs.rs` [Repo-grounded — file exists] — commands: `test:unit`,
      `test:integration` — acceptance: green.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/docs`; confirm `domain/docs/` is
      IO-free — command: `npx nx run rhino-cli:lint` and
      `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli/src/domain/docs/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the `commands/docs\.rs`
      entry (and any relocated `internal/docs` file) to its new path — command:
      `npx nx run rhino-cli:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] The golden-master CLI suite exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli` — green; ≥90%.
- [ ] [AI] `docs` slice complete, domain IO-free (grep); imports the relocated `mermaid` kernel.

> **Pause Safety**: `docs` migrated; remaining features green. Safe to stop.
> To resume: re-run the gate commands and confirm GREEN, then begin Phase 6.

---

## Phase 6: `doctor` (depends on `cliout`)

- [ ] [AI] Confirm the golden-master CLI suite GREEN — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `doctor` use case (assert injected
      `ToolProber`-style ports) — `#[cfg(test)]` in `apps/rhino-cli/src/application/doctor/`
      (sibling pattern: existing `internal/doctor` tests) — command:
      `npx nx run rhino-cli:test:unit` — acceptance: fails (ports not yet defined).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `doctor` to slices; extract a `ToolProber`-style port per
      probed tool seam (doctor shells out heavily); `doctor` imports the relocated `cliout`
      kernel; inbound stays `src/commands/doctor.rs` [Repo-grounded — file exists]
      — commands: `test:unit`, `test:integration` — acceptance: green.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/doctor`; confirm `domain/doctor/` is
      IO-free — command: `npx nx run rhino-cli:lint` and
      `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli/src/domain/doctor/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the `commands/doctor\.rs`
      entry (and any relocated `internal/doctor` file) to its new path — command:
      `npx nx run rhino-cli:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] The golden-master CLI suite exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli` — green; ≥90%.
- [ ] [AI] `doctor` slice complete, domain IO-free (grep); `doctor` imports the relocated `cliout` kernel.

> **Pause Safety**: `doctor` migrated; remaining features green. Safe to stop.
> To resume: re-run the gate commands and confirm GREEN, then begin Phase 7.

---

## Phase 7: `envbackup` / `env` (depends on `cliout`)

- [ ] [AI] Confirm the golden-master CLI suite GREEN — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `envbackup` use case (assert
      injected file-read/write + exec ports) — `#[cfg(test)]` in
      `apps/rhino-cli/src/application/env/` (sibling pattern: existing `internal/envbackup`
      tests) — command: `npx nx run rhino-cli:test:unit` — acceptance: fails (ports not yet defined).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `envbackup` to slices; extract one named port per IO seam
      (file read/write, exec); `env` imports the relocated `cliout` kernel; inbound stays
      `src/commands/env.rs` [Repo-grounded — file exists] — commands: `test:unit`,
      `test:integration` — acceptance: green.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/envbackup`; confirm the domain dir is IO-free
      — command: `npx nx run rhino-cli:lint` and
      `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli/src/domain/env/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the `commands/env\.rs`
      entry (and any relocated `internal/envbackup` file) to its new path — command:
      `npx nx run rhino-cli:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 7 Gate

> All checks below must pass before starting Phase 8.

- [ ] [AI] The golden-master CLI suite exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli` — green; ≥90%.
- [ ] [AI] `envbackup`/`env` slice complete, domain IO-free (grep); `env` imports the relocated `cliout` kernel.

> **Pause Safety**: `envbackup`/`env` migrated; remaining features green. Safe
> to stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 8.

---

## Phase 8: `testcoverage` (file + exec; git-diff/merge paths)

- [ ] [AI] Confirm the golden-master CLI suite GREEN — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `testcoverage` use case (assert
      injected `CoverageReader`-style ports for read/diff/merge) — `#[cfg(test)]` in
      `apps/rhino-cli/src/application/testcoverage/` (sibling pattern: existing
      `internal/testcoverage` tests) — command: `npx nx run rhino-cli:test:unit`
      — acceptance: fails (ports not yet defined).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `testcoverage` to slices; extract `CoverageReader`-style
      ports for the file-read/diff/merge seams; inbound stays
      `src/commands/testcoverage.rs` [Repo-grounded — file exists] — commands: `test:unit`,
      `test:integration` — acceptance: green.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/testcoverage`; confirm the domain dir is
      IO-free — command: `npx nx run rhino-cli:lint` and
      `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli/src/domain/testcoverage/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the entries
      `commands/testcoverage\.rs`, `internal/testcoverage/diff\.rs`, and
      `internal/testcoverage/merge\.rs` to their relocated paths (or remove any whose relocated file
      is now directly unit-tested) — command: `npx nx run rhino-cli:test:quick`
      — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 8 Gate

> All checks below must pass before starting Phase 9.

- [ ] [AI] The golden-master CLI suite exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli` — green; ≥90%.
- [ ] [AI] `testcoverage` slice complete, domain IO-free (grep); all three allowlist entries updated.

> **Pause Safety**: `testcoverage` migrated; remaining features green. Safe to
> stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 9.

---

## Phase 9: `agents` (depends on `naming`)

- [ ] [AI] Confirm the golden-master CLI suite GREEN — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `agents` use case (assert injected
      file-scan + exec ports) — `#[cfg(test)]` in `apps/rhino-cli/src/application/agents/`
      (sibling pattern: existing `internal/agents` tests) — command:
      `npx nx run rhino-cli:test:unit` — acceptance: fails (ports not yet defined).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `agents` to slices; ports for file-scan + exec seams;
      `agents` imports the relocated `naming`; inbound stays `src/commands/agents.rs`
      [Repo-grounded — file exists] — commands: `test:unit`, `test:integration` — acceptance: green.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/agents`; confirm `domain/agents/` is
      IO-free — command: `npx nx run rhino-cli:lint` and
      `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli/src/domain/agents/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the `commands/agents\.rs`
      entry (and any relocated `internal/agents` file) to its new path — command:
      `npx nx run rhino-cli:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 9 Gate

> All checks below must pass before starting Phase 10.

- [ ] [AI] The golden-master CLI suite exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli` — green; ≥90%.
- [ ] [AI] `agents` slice complete, domain IO-free (grep); `agents` imports the relocated `naming`.

> **Pause Safety**: `agents` migrated; remaining features green. Safe to stop.
> To resume: re-run the gate commands and confirm GREEN, then begin Phase 10.

---

## Phase 10: `contracts`

- [ ] [AI] Confirm the golden-master CLI suite GREEN — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `contracts` use case (assert
      injected scaffold file-write + exec ports) — `#[cfg(test)]` in
      `apps/rhino-cli/src/application/contracts/` (sibling pattern: existing `internal/contracts`
      tests) — command: `npx nx run rhino-cli:test:unit` — acceptance: fails (ports not yet defined).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `contracts` to slices; ports for scaffold file-write +
      exec seams; inbound stays `src/commands/contracts.rs` [Repo-grounded —
      file exists] — commands: `test:unit`, `test:integration` — acceptance: green.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/contracts`; confirm `domain/contracts/`
      is IO-free — command: `npx nx run rhino-cli:lint`
      and `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli/src/domain/contracts/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the `commands/contracts\.rs`
      entry (and any relocated `internal/contracts` file) to its new path — command:
      `npx nx run rhino-cli:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 10 Gate

> All checks below must pass before starting Phase 11.

- [ ] [AI] The golden-master CLI suite exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli` — green; ≥90%.
- [ ] [AI] `contracts` slice complete, domain IO-free (grep).

> **Pause Safety**: `contracts` migrated; remaining features green. Safe to
> stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 11.

---

## Phase 11: `java`

- [ ] [AI] Confirm the golden-master CLI suite GREEN — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `java` use case (assert injected
      file-scan port) — `#[cfg(test)]` in `apps/rhino-cli/src/application/java/` (sibling
      pattern: existing `internal/java` tests) — command: `npx nx run rhino-cli:test:unit`
      — acceptance: fails (port not yet defined).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `java` to slices; port the file-scan seam; inbound stays
      `src/commands/java.rs` [Repo-grounded] — command: `test:unit` — acceptance: green.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/java`; confirm `domain/java/` is
      IO-free — command: `npx nx run rhino-cli:lint` and
      `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli/src/domain/java/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the `commands/java\.rs`
      entry (and any relocated `internal/java` file) to its new path — command:
      `npx nx run rhino-cli:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 11 Gate

> All checks below must pass before starting Phase 12.

- [ ] [AI] The golden-master CLI suite exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli` — green; ≥90%.
- [ ] [AI] `java` slice complete, domain IO-free (grep).

> **Pause Safety**: `java` migrated; remaining features green. Safe to stop.
> To resume: re-run the gate commands and confirm GREEN, then begin Phase 12.

---

## Phase 12: `repo-governance`

> Logic currently lives in `commands/` only [Repo-grounded — `repo_governance` logic in
>
> > `commands/`]; extract the use case into `application/`, pure rules into `domain/`, keep a thin
> > inbound shim. Validate with the governance gates.

- [ ] [AI] Confirm the golden-master CLI suite GREEN — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `repo-governance` use case (assert
      injected file-scan port; pure rule logic separated) — `#[cfg(test)]` in
      `apps/rhino-cli/src/application/repo_governance/` (sibling pattern: existing
      `commands/repo_governance.rs` tests) — command: `npx nx run rhino-cli:test:unit`
      — acceptance: fails (use case not yet extracted).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: extract the vendor-audit + gherkin-cardinality use cases into `application/`,
      pure rule logic into `domain/`; thin inbound shim stays
      `src/commands/repo_governance.rs` [Repo-grounded] — commands: `test:unit`,
      `npx nx run rhino-cli:validate:repo-governance-vendor-audit`,
      `npx nx run rhino-cli:validate:gherkin-keyword-cardinality`
      — acceptance: green.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce the inbound to a thin shim; confirm the `domain/` rule dir is
      IO-free — command: `npx nx run rhino-cli:lint` and
      `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli/src/domain/repo_governance/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update any
      `commands/repo_governance\.rs` entry to its relocated path (or remove if now directly
      unit-tested) — command: `npx nx run rhino-cli:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 12 Gate

> All checks below must pass before starting Phase 13.

- [ ] [AI] The golden-master CLI suite exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli` — green; ≥90%.
- [ ] [AI] `npx nx run rhino-cli:validate:repo-governance-vendor-audit` and `:validate:gherkin-keyword-cardinality` — exit 0.
- [ ] [AI] `repo-governance` use case in `application/`, rules in `domain/` (IO-free, grep), thin inbound shim.

> **Pause Safety**: `repo-governance` migrated; remaining features green. Safe
> to stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 13.

---

## Phase 13: `speccoverage`

- [ ] [AI] Confirm the golden-master CLI suite GREEN — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `speccoverage` use case (assert
      injected gherkin/impl on-disk scan port) — `#[cfg(test)]` in
      `apps/rhino-cli/src/application/speccoverage/` (sibling pattern:
      existing `internal/speccoverage` tests) — command: `npx nx run rhino-cli:test:unit`
      — acceptance: fails (port not yet defined).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `speccoverage` to slices; port the on-disk gherkin/impl
      scan seam; inbound stays `src/commands/speccoverage.rs` [Repo-grounded]
      — command: `test:unit` — acceptance: green.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce/remove old `internal/speccoverage`; confirm the domain dir is
      IO-free — command: `npx nx run rhino-cli:lint` and
      `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli/src/domain/speccoverage/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update the `commands/speccoverage\.rs`
      entry (and any relocated `internal/speccoverage` file) to its new path — command:
      `npx nx run rhino-cli:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 13 Gate

> All checks below must pass before starting Phase 14.

- [ ] [AI] The golden-master CLI suite exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli` — green; ≥90%.
- [ ] [AI] `speccoverage` slice complete, domain IO-free (grep).

> **Pause Safety**: `speccoverage` migrated; remaining features green. Safe to
> stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 14.

---

## Phase 14: `workflows`

> Move the use case into `application/`/`domain/`, keep a thin inbound shim. Validate with
> `validate:naming-workflows`.

- [ ] [AI] Confirm the golden-master CLI suite GREEN — exits 0.
- [ ] [AI] **RED**: add a failing fake-backed unit test for the `workflows` use case —
      `#[cfg(test)]` in `apps/rhino-cli/src/application/workflows/` (sibling pattern: existing
      `workflows` tests) — command: `npx nx run rhino-cli:test:unit`
      — acceptance: fails (use case not yet extracted).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: migrate `workflows`; move the use case into
      `application/`/`domain/`, keep a thin inbound shim — commands: `test:unit`,
      `npx nx run rhino-cli:validate:naming-workflows` (if present)
      — acceptance: green.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: reduce the inbound to a thin shim; confirm `domain/workflows/`
      is IO-free — command: `npx nx run rhino-cli:lint`
      and `grep -rEn "std::fs|std::process|std::net" apps/rhino-cli/src/domain/workflows/`
      — acceptance: clippy `-D warnings` exits 0; grep returns no IO imports.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **CRITICAL — coverage allowlist lockstep**: in `apps/rhino-cli/project.json`
      `test:quick` `--ignore-filename-regex` [Repo-grounded — line 83], update any relocated
      `workflows` entry to its new path (or remove if now directly unit-tested) — command:
      `npx nx run rhino-cli:test:quick` — acceptance: ≥90%, no false break.
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; ≥90%. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 14 Gate

> All checks below must pass before starting Phase 15.

- [ ] [AI] The golden-master CLI suite exits 0 (GREEN against the existing Phase 0 baseline; corpus unchanged).
- [ ] [AI] `npx nx run-many -t test:unit test:integration test:quick lint typecheck --projects=rhino-cli` — green; ≥90%.
- [ ] [AI] `npx nx run rhino-cli:validate:naming-workflows` — exits 0.
- [ ] [AI] `workflows` use case in `application/`/`domain/` (IO-free, grep), thin inbound shim.
- [ ] [AI] ALL 13 features now live in hexagonal slices; every `domain/<feature>/`
      and `domain/shared/` dir is IO-free (grep across the app returns no IO imports under domain).

> **Pause Safety**: the full structural migration is complete and behavior-identical. Safe to
> stop. To resume: re-run the gate commands and confirm GREEN, then begin Phase 15.

---

## Phase 15: Update the hexagonal-CLI convention doc (vendor-neutral)

> Final phase. Documents the realized architecture. Must stay vendor-neutral (no Claude/OpenCode
> /vendor terms); the vendor-audit gate enforces this. Do NOT reference an arch lint (none added).

- [ ] [AI] Update `repo-governance/development/pattern/hexagonal-architecture-cli.md`
      [Repo-grounded — file exists] to document: (a) the hybrid `domain/shared` kernel +
      per-feature vertical-slice layout; (b) the maximal-ports requirement (every IO boundary is a
      named port) with the one-line accepted trade-off vs. the lean approach; (c) the domain-role
      port-naming rule (`StagedFileProvider`, not `FileSystem`); (d) the 2+-consumer shared-kernel
      rule (`{mermaid, cliout}`) — acceptance:
      the four topics are present; no vendor terms; no arch-lint reference.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Render-check any added Mermaid diagram and heading hierarchy:
      `npx nx run rhino-cli:validate:mermaid` and `npx nx run rhino-cli:validate:heading-hierarchy`
      — acceptance: both exit 0.
- [ ] [AI] Vendor-neutrality + link checks:
      `npx nx run rhino-cli:validate:repo-governance-vendor-audit` and
      `npx nx run rhino-cli:validate:links` — acceptance: both exit 0.

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t lint typecheck` — exits 0. Fix ALL failures.

### Post-Push Verification

- [ ] [AI] Push per D3; monitor ALL GitHub Actions; fix and re-push until green.

### Phase 15 Gate

> All checks below must pass before final verification.

- [ ] [AI] `npx nx run rhino-cli:validate:repo-governance-vendor-audit` — exits 0 (vendor-neutral).
- [ ] [AI] `npx nx run rhino-cli:validate:links` and `:validate:heading-hierarchy` — exit 0.
- [ ] [AI] The convention doc documents all four required topics (layout, maximal ports + trade-off,
      domain-role naming, 2+-consumer kernel rule) and references no arch lint.

> **Pause Safety**: migration + documentation complete; repo coherent and green. Safe to stop.
> To resume: proceed to final verification and archival.

---

## Final Verification

```gherkin
Scenario: The migration is complete and behavior-preserving
  Given all 13 features are migrated to hexagonal slices
  When the full gate suite runs across the app
  Then the golden-master CLI suite, test:unit, test:integration, test:quick (>=90%), lint, and typecheck all pass
  And the convention doc records the architecture vendor-neutrally
```

- [ ] [AI] Full sweep: `npx nx run-many -t build test:unit test:integration test:quick lint typecheck --projects=rhino-cli`
      and run the golden-master CLI suite — acceptance: all GREEN; ≥90%.

## Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI) and ALL phase gates were green.
- [ ] [AI] Verify the golden-master CLI suite GREEN at completion.
- [ ] [AI] Move: `git mv plans/in-progress/migrate-rhino-cli-to-hexagonal plans/done/2026-06-09__migrate-rhino-cli-to-hexagonal`
      (use the actual completion date if later) — acceptance: folder relocated under `plans/done/`.
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date.
- [ ] [AI] Update any other READMEs that reference this plan (e.g. `plans/README.md`).
- [ ] [AI] Commit the archival: `chore(plans): move migrate-rhino-cli-to-hexagonal to done`.
