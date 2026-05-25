# Delivery Checklist — rhino-cli-rust Strictness Alignment

## Worktree

Worktree path: `worktrees/rhino-cli-rust-strictness/`

Execution runs directly on `main` per user directive (no worktree isolation required for
single-app changes). Working directory: `/Users/wkf/ose-projects/ose-primer`.

If worktree isolation is later needed:

```bash
claude --worktree rhino-cli-rust-strictness
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0: Environment Setup

_Suggested executor: calling context (orchestrator)_

- [ ] Run `npm install && npm run doctor -- --fix` from repo root to ensure toolchain is current.
      Acceptance: `npm run doctor` exits 0.

- [ ] Verify `cargo deny --version` — if missing, run `cargo install cargo-deny --locked`.
      Acceptance: `cargo deny --version` exits 0.

- [ ] Verify `cargo hack --version` — if missing, run `cargo install cargo-hack --locked`.
      Acceptance: `cargo hack --version` exits 0.

- [ ] Run baseline unit tests: `cargo test --manifest-path apps/rhino-cli-rust/Cargo.toml --lib`.
      Acceptance: exits 0; note any preexisting failures before proceeding.

- [ ] Run baseline clippy: `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings`.
      Acceptance: violations counted and noted; note preexisting failures before proceeding.

## Phase 1: project.json Structural Alignment

_Suggested executor: swe-rust-dev_

- [ ] Add `fmt` target to `apps/rhino-cli-rust/project.json` before the `lint` target.
      Command: `"cargo fmt --manifest-path apps/rhino-cli-rust/Cargo.toml"`.
      Acceptance: `npx nx run rhino-cli-rust:fmt` exits 0.

- [ ] Add `fmt:check` target to `apps/rhino-cli-rust/project.json` after `fmt`.
      Command: `"cargo fmt --manifest-path apps/rhino-cli-rust/Cargo.toml -- --check"`, `cache: true`,
      inputs: `["{projectRoot}/src/**/*.rs", "{projectRoot}/.rustfmt.toml", "{workspaceRoot}/.rustfmt.toml"]`.
      Acceptance: `npx nx run rhino-cli-rust:fmt:check` exits 0 on a clean tree.

- [ ] Add `deny:check` target to `apps/rhino-cli-rust/project.json` after `fmt:check`.
      Command: `"cargo deny --manifest-path apps/rhino-cli-rust/Cargo.toml check"`, `cache: true`,
      inputs: `["{projectRoot}/Cargo.toml", "{projectRoot}/Cargo.lock", "{projectRoot}/deny.toml"]`.
      Acceptance: `npx nx run rhino-cli-rust:deny:check` exits 0.

- [ ] Add `check:msrv` target to `apps/rhino-cli-rust/project.json` after `deny:check`.
      Command: `"cargo hack --manifest-path apps/rhino-cli-rust/Cargo.toml check --rust-version"`,
      `cache: true`, inputs: `["{projectRoot}/Cargo.toml", "{projectRoot}/src/**/*.rs"]`.
      Acceptance: `npx nx run rhino-cli-rust:check:msrv` exits 0.

- [ ] Fix `build` target in `apps/rhino-cli-rust/project.json`: replace `cwd: apps/rhino-cli-rust`
      with full `--manifest-path apps/rhino-cli-rust/Cargo.toml` command; set `outputs` to
      `["{projectRoot}/dist", "{projectRoot}/target"]`.
      Acceptance: `npx nx run rhino-cli-rust:build` exits 0 and `apps/rhino-cli-rust/dist/rhino-cli` exists.

- [ ] Fix `install` target: remove `cwd`, set command to
      `"cargo fetch --manifest-path apps/rhino-cli-rust/Cargo.toml"`.
      Acceptance: `npx nx run rhino-cli-rust:install` exits 0.

- [ ] Fix `run` target: remove `cwd`, set command to
      `"cargo run --manifest-path apps/rhino-cli-rust/Cargo.toml --"`.
      Acceptance: command in project.json matches ose-public pattern.

- [ ] Fix `typecheck` target: remove `cwd`, set command to
      `"cargo check --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets"`.
      Acceptance: `npx nx run rhino-cli-rust:typecheck` exits 0.

- [ ] Fix `lint` target: convert from single `command` + `cwd` to `commands` array with
      `parallel: false`; commands are `"cargo fmt --manifest-path apps/rhino-cli-rust/Cargo.toml -- --check"`
      and `"cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings"`;
      remove `inputs` from the target (ose-public lint has no inputs).
      Acceptance: `npx nx run rhino-cli-rust:lint` exits 0.

- [ ] Fix `test:unit` target: remove `cwd`, set command to
      `"cargo test --manifest-path apps/rhino-cli-rust/Cargo.toml --lib"`, add `cache: true`,
      set inputs to `["{projectRoot}/Cargo.toml", "{projectRoot}/src/**/*.rs"]`.
      Acceptance: `npx nx run rhino-cli-rust:test:unit` exits 0.

- [ ] Fix `test:quick` target: remove `cwd`, change command prefix to
      `cargo llvm-cov --manifest-path apps/rhino-cli-rust/Cargo.toml --lib ...`;
      change `--output-path cover.out` to `--output-path apps/rhino-cli-rust/lcov.info`;
      update `outputs` to `["{projectRoot}/lcov.info"]`.
      Acceptance: `npx nx run rhino-cli-rust:test:quick` exits 0 and `apps/rhino-cli-rust/lcov.info` is created.

- [ ] Fix `test:integration` target: remove `cwd`, set command to
      `"cargo test --manifest-path apps/rhino-cli-rust/Cargo.toml --tests"`, add `cache: true`,
      update inputs to use `{projectRoot}/Cargo.toml` prefix.
      Acceptance: command in project.json matches ose-public pattern.

- [ ] Commit Phase 1 changes with message `feat(rhino-cli-rust): align project.json targets to ose-public structure`.
      Acceptance: `git log --oneline -1` shows the commit.

## Phase 2: Cargo.toml Lint Alignment

_Suggested executor: swe-rust-dev_

- [ ] Remove the Go-parity allows block from `apps/rhino-cli-rust/Cargo.toml`:
      delete the comment `# --- Structural allows for Go-parity ports: ...` and all seven allows
      (`too_many_lines`, `manual_let_else`, `assigning_clones`, `format_push_string`,
      `cast_sign_loss`, `unnecessary_debug_formatting`, `collapsible_if`).
      Acceptance: `grep -E "too_many_lines|manual_let_else|assigning_clones|format_push_string|cast_sign_loss|unnecessary_debug_formatting|collapsible_if" apps/rhino-cli-rust/Cargo.toml` returns empty.

- [ ] Commit Cargo.toml change: `feat(rhino-cli-rust): remove Go-parity clippy allows to match ose-public`.
      Acceptance: `git log --oneline -1` shows the commit.

## Phase 3: Clippy Violation Remediation

_Suggested executor: swe-rust-dev_

- [ ] Run clippy baseline to identify all violations:
      `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | tee /tmp/clippy-baseline.txt`.
      Acceptance: output captured in `/tmp/clippy-baseline.txt`; violations counted.

- [ ] RED: Confirm `collapsible_if` violation exists before fixing.
      `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | grep "collapsible_if"`.
      Acceptance: at least one line of output (violation present).
- [ ] GREEN: Fix all `collapsible_if` violations in `apps/rhino-cli-rust/src/`.
      Merge nested `if` conditions with `&&`.
      Verification command: `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | grep "collapsible_if"`.
      Acceptance: command output is empty (zero `collapsible_if` warnings).

- [ ] RED: Confirm `too_many_lines` violation exists before fixing.
      `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | grep "too_many_lines"`.
      Acceptance: at least one line of output (violation present).
- [ ] GREEN: Fix all `too_many_lines` violations in `apps/rhino-cli-rust/src/`.
      Extract helper functions to reduce function body length.
      Verification command: `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | grep "too_many_lines"`.
      Acceptance: command output is empty (zero `too_many_lines` warnings).

- [ ] RED: Confirm `manual_let_else` violation exists before fixing.
      `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | grep "manual_let_else"`.
      Acceptance: at least one line of output (violation present).
- [ ] GREEN: Fix all `manual_let_else` violations in `apps/rhino-cli-rust/src/`.
      Convert `if let Some(x) = y { ... } else { return/continue }` to `let Some(x) = y else { ... }`.
      Verification command: `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | grep "manual_let_else"`.
      Acceptance: command output is empty (zero `manual_let_else` warnings).

- [ ] RED: Confirm `assigning_clones` violation exists before fixing.
      `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | grep "assigning_clones"`.
      Acceptance: at least one line of output (violation present).
- [ ] GREEN: Fix all `assigning_clones` violations in `apps/rhino-cli-rust/src/`.
      Replace `x = y.clone()` with `x.clone_from(&y)` where applicable.
      Verification command: `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | grep "assigning_clones"`.
      Acceptance: command output is empty (zero `assigning_clones` warnings).

- [ ] RED: Confirm `format_push_string` violation exists before fixing.
      `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | grep "format_push_string"`.
      Acceptance: at least one line of output (violation present).
- [ ] GREEN: Fix all `format_push_string` violations in `apps/rhino-cli-rust/src/`.
      Replace `s.push_str(&format!(...))` with `use std::fmt::Write; write!(&mut s, ...).unwrap_or(())`.
      Verification command: `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | grep "format_push_string"`.
      Acceptance: command output is empty (zero `format_push_string` warnings).

- [ ] RED: Confirm `cast_sign_loss` violation exists before fixing.
      `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | grep "cast_sign_loss"`.
      Acceptance: at least one line of output (violation present).
- [ ] GREEN: Fix all `cast_sign_loss` violations in `apps/rhino-cli-rust/src/`.
      Add explicit `as usize` or restructure arithmetic to avoid signed→unsigned casts.
      Verification command: `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | grep "cast_sign_loss"`.
      Acceptance: command output is empty (zero `cast_sign_loss` warnings).

- [ ] RED: Confirm `unnecessary_debug_formatting` violation exists before fixing.
      `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | grep "unnecessary_debug_formatting"`.
      Acceptance: at least one line of output (violation present).
- [ ] GREEN: Fix all `unnecessary_debug_formatting` violations in `apps/rhino-cli-rust/src/`.
      Change `{:?}` to `{}` for types that implement `Display`.
      Verification command: `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1 | grep "unnecessary_debug_formatting"`.
      Acceptance: command output is empty (zero `unnecessary_debug_formatting` warnings).
- [ ] REFACTOR: Verify all tests still pass after lint fixes.
      `cargo test --manifest-path apps/rhino-cli-rust/Cargo.toml --lib`.
      Acceptance: exit code 0.

- [ ] Verify `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings`
      exits 0 with zero warnings.
      Acceptance: exit code 0, no warning lines in output.

- [ ] Verify `cargo test --manifest-path apps/rhino-cli-rust/Cargo.toml --lib` still passes.
      Acceptance: exit code 0.

- [ ] Commit violation fixes: `fix(rhino-cli-rust): fix clippy violations after removing Go-parity allows`.
      Acceptance: `git log --oneline -1` shows the commit.

## Phase 4: Add .gitignore

_Suggested executor: swe-rust-dev_

- [ ] Create `apps/rhino-cli-rust/.gitignore` with content:

  ```
  target/
  dist/
  lcov.info
  lcov_spec.info
  *.profraw
  ```

  Acceptance: file exists at `apps/rhino-cli-rust/.gitignore` with those five entries.

- [ ] Commit: `chore(rhino-cli-rust): add .gitignore matching ose-public`.
      Acceptance: `git log --oneline -1` shows the commit.

## Phase 5: Quality Gates and CI

_Suggested executor: calling context (orchestrator)_

- [ ] Run `npx nx run rhino-cli-rust:typecheck` — exit 0.
      Acceptance: command exits 0.

- [ ] Run `npx nx run rhino-cli-rust:lint` — exit 0 (fmt:check + clippy pass).
      Acceptance: command exits 0.

- [ ] Run `npx nx run rhino-cli-rust:test:quick` — exit 0, coverage ≥ 90%.
      Acceptance: command exits 0, `apps/rhino-cli-rust/lcov.info` updated.

- [ ] Run `npx nx run rhino-cli-rust:deny:check` — exit 0, no advisories.
      Acceptance: command exits 0.

- [ ] Run `npx nx affected -t typecheck lint test:quick spec-coverage` — all pass.
      Acceptance: command exits 0.

- [ ] Push all commits to `origin main`.
      Acceptance: `git log --oneline origin/main..HEAD` returns empty (all commits pushed).

- [ ] Monitor GitHub Actions CI: `gh run list --limit=3` shows latest run passing.
      Acceptance: latest workflow run status is `completed` / `success`.

## Quality Gates

Local gates (run before every push):

```bash
npx nx run rhino-cli-rust:typecheck
npx nx run rhino-cli-rust:lint
npx nx run rhino-cli-rust:test:quick
npx nx run rhino-cli-rust:deny:check
npx nx affected -t typecheck lint test:quick spec-coverage
```

CI gates: All GitHub Actions workflows must pass after push.

- [ ] Fix ALL failures encountered above, including preexisting ones not caused by this plan. Commit preexisting fixes separately.

## Verification

Plan is done when:

1. `apps/rhino-cli-rust/project.json` contains `fmt`, `fmt:check`, `deny:check`, and `check:msrv` targets.
2. All targets use `--manifest-path apps/rhino-cli-rust/Cargo.toml` (no `cwd` key).
3. `lint` uses `commands` array with `parallel: false`.
4. `test:quick` outputs `{projectRoot}/lcov.info`.
5. `apps/rhino-cli-rust/Cargo.toml` `[lints.clippy]` has zero Go-parity allows.
6. `cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings` exits 0.
7. `cargo test --manifest-path apps/rhino-cli-rust/Cargo.toml --lib` exits 0.
8. `apps/rhino-cli-rust/.gitignore` exists with five entries.
9. All GitHub Actions CI checks pass.

## Plan Archival

- [ ] Move plan folder to done/: `git mv plans/in-progress/rhino-cli-rust-strictness plans/done/2026-05-25__rhino-cli-rust-strictness`.
      Acceptance: folder exists at `plans/done/2026-05-25__rhino-cli-rust-strictness`.

- [ ] Update `plans/in-progress/README.md` — remove the rhino-cli-rust-strictness entry.
      Acceptance: `grep "rhino-cli-rust-strictness" plans/in-progress/README.md` returns empty.

- [ ] Update `plans/done/README.md` — add the plan entry line:
      text `rhino-cli-rust Strictness Alignment`, link target `./2026-05-25__rhino-cli-rust-strictness/`,
      description "Aligned project.json and Clippy rules to ose-public. Completed 2026-05-25."
      Acceptance: entry appears in `plans/done/README.md`.

- [ ] Commit archival: `chore(plans): move rhino-cli-rust-strictness to done`.
      Acceptance: `git log --oneline -1` shows the commit.
