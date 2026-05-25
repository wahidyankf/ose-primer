# Technical Approach — rhino-cli-rust Strictness Alignment

## Architecture Overview

This plan makes three categories of changes to `apps/rhino-cli-rust/`:

1. **`project.json` restructuring** — target-level changes only; no source code affected
2. **`Cargo.toml` lint changes** — removing seven `[lints.clippy]` allows triggers Clippy violations
   in source files that must be fixed
3. **New `.gitignore`** — trivial file addition; no source changes

## Reference: ose-public vs ose-primer Diff

All differences confirmed by reading both files directly. [Repo-grounded]

### project.json: Missing Targets

ose-public has these targets; ose-primer does NOT: [Repo-grounded: ose-public project.json]

| Target       | Command                                                                          |
| ------------ | -------------------------------------------------------------------------------- |
| `fmt`        | `cargo fmt --manifest-path apps/rhino-cli-rust/Cargo.toml`                       |
| `fmt:check`  | `cargo fmt --manifest-path apps/rhino-cli-rust/Cargo.toml -- --check`            |
| `deny:check` | `cargo deny --manifest-path apps/rhino-cli-rust/Cargo.toml check`                |
| `check:msrv` | `cargo hack --manifest-path apps/rhino-cli-rust/Cargo.toml check --rust-version` |

### project.json: Target Structure Changes

| Target       | ose-primer (current)                                 | ose-public (target)                                    |
| ------------ | ---------------------------------------------------- | ------------------------------------------------------ |
| `lint`       | single `command` string with `&&`, `cwd`             | `commands` array, `parallel: false`, `--manifest-path` |
| `build`      | `cwd: apps/rhino-cli-rust`, outputs `target/release` | `--manifest-path`, outputs `target`                    |
| `test:quick` | `cwd`, output `cover.out`                            | `--manifest-path`, output `lcov.info`                  |
| All others   | `cwd: apps/rhino-cli-rust`                           | `--manifest-path apps/rhino-cli-rust/Cargo.toml`       |

### Cargo.toml: Lint Allows to Remove

Seven allows in ose-primer that are absent from ose-public [Repo-grounded: both Cargo.toml files]:

```toml
too_many_lines = "allow"
manual_let_else = "allow"
assigning_clones = "allow"
format_push_string = "allow"
cast_sign_loss = "allow"
unnecessary_debug_formatting = "allow"
collapsible_if = "allow"
```

These are currently under the comment `# --- Structural allows for Go-parity ports ---`.
After removal, all violations in src/ must be fixed.

## Implementation Plan

### Phase 1: project.json Structural Alignment

Edit `apps/rhino-cli-rust/project.json`:

1. **Add `fmt` target** (before `lint`):

   ```json
   "fmt": {
     "executor": "nx:run-commands",
     "options": {
       "command": "cargo fmt --manifest-path apps/rhino-cli-rust/Cargo.toml"
     }
   }
   ```

2. **Add `fmt:check` target** (after `fmt`):

   ```json
   "fmt:check": {
     "executor": "nx:run-commands",
     "options": {
       "command": "cargo fmt --manifest-path apps/rhino-cli-rust/Cargo.toml -- --check"
     },
     "cache": true,
     "inputs": [
       "{projectRoot}/src/**/*.rs",
       "{projectRoot}/.rustfmt.toml",
       "{workspaceRoot}/.rustfmt.toml"
     ]
   }
   ```

3. **Add `deny:check` target** (after `fmt:check`):

   ```json
   "deny:check": {
     "executor": "nx:run-commands",
     "options": {
       "command": "cargo deny --manifest-path apps/rhino-cli-rust/Cargo.toml check"
     },
     "cache": true,
     "inputs": [
       "{projectRoot}/Cargo.toml",
       "{projectRoot}/Cargo.lock",
       "{projectRoot}/deny.toml"
     ]
   }
   ```

4. **Add `check:msrv` target** (after `deny:check`):

   ```json
   "check:msrv": {
     "executor": "nx:run-commands",
     "options": {
       "command": "cargo hack --manifest-path apps/rhino-cli-rust/Cargo.toml check --rust-version"
     },
     "cache": true,
     "inputs": [
       "{projectRoot}/Cargo.toml",
       "{projectRoot}/src/**/*.rs"
     ]
   }
   ```

5. **Fix `build` target**: remove `cwd`, use `--manifest-path`, fix outputs
6. **Fix `install` target**: remove `cwd`, use `--manifest-path`
7. **Fix `run` target**: remove `cwd`, use `--manifest-path`
8. **Fix `typecheck` target**: remove `cwd`, use `--manifest-path`
9. **Fix `lint` target**: convert to `commands` array with `parallel: false`, use `--manifest-path`
10. **Fix `test:unit` target**: remove `cwd`, use `--manifest-path`
11. **Fix `test:quick` target**: remove `cwd`, use `--manifest-path`, change output to `lcov.info`
12. **Fix `test:integration` target**: remove `cwd`, use `--manifest-path`

Note: `validate:*` targets already use `--manifest-path`; verify they don't use `cwd`.

### Phase 2: Cargo.toml Lint Alignment

Edit `apps/rhino-cli-rust/Cargo.toml`:

- Remove the entire "Go-parity ports" comment block and its seven allows
- Keep all other existing allows (they match ose-public)

### Phase 3: Clippy Violation Fixes

Run:

```bash
cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings 2>&1
```

Fix each violation by applying idiomatic Rust:

| Lint removed                   | Typical fix                                                                                     |
| ------------------------------ | ----------------------------------------------------------------------------------------------- |
| `collapsible_if`               | Merge nested `if` into `if condition1 && condition2`                                            |
| `too_many_lines`               | Extract helper functions                                                                        |
| `manual_let_else`              | Convert `if let Some(x) = y { ... } else { return/continue }` to `let Some(x) = y else { ... }` |
| `assigning_clones`             | Use `.clone_from()` instead of `= other.clone()`                                                |
| `format_push_string`           | Use `push_str(&format!(...))` → `write!(&mut s, ...)` or restructure                            |
| `cast_sign_loss`               | Add explicit cast or use unsigned arithmetic                                                    |
| `unnecessary_debug_formatting` | Change `{:?}` to `{}` for Display-implementing types                                            |

### Phase 4: Add .gitignore

Create `apps/rhino-cli-rust/.gitignore`:

```
target/
dist/
lcov.info
lcov_spec.info
*.profraw
```

### Phase 5: Quality Gates

```bash
# Format check
npx nx run rhino-cli-rust:fmt:check

# Lint (fmt + clippy)
npx nx run rhino-cli-rust:lint

# Type check
npx nx run rhino-cli-rust:typecheck

# Unit tests + coverage gate
npx nx run rhino-cli-rust:test:quick

# Deny check
npx nx run rhino-cli-rust:deny:check

# Affected suite
npx nx affected -t typecheck lint test:quick spec-coverage
```

## External Tool Requirements

- `cargo-deny`: must be installed. Check: `cargo deny --version`. Install: `cargo install cargo-deny --locked`
- `cargo-hack`: must be installed. Check: `cargo hack --version`. Install: `cargo install cargo-hack --locked`

Both tools are referenced in ose-public's project.json and assumed available in the toolchain.
[Judgment call — not verified in ose-primer doctor config]

## Design Decisions

**Keep `cwd` removal scope limited**: Only change target invocation style (manifest-path vs cwd).
Do not change the behavior or semantics of any target. The manifest-path approach is equivalent
to `cwd` for single-crate workspaces.

**No dependency changes**: `serde_norway`, `thiserror`, and `tokio` are kept as-is. They are used
in the code and removing them would require a separate refactor plan.

**Lint fixes follow minimal-change principle**: Fix only what Clippy reports. Do not refactor
beyond the violation sites.

## File Impact

| File                                          | Change               | Notes                                                                                                                                          |
| --------------------------------------------- | -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `apps/rhino-cli-rust/project.json`            | Modified             | Add 4 new targets; restructure all existing targets to use `--manifest-path`; fix `lint` to use `commands` array; fix `test:quick` output path |
| `apps/rhino-cli-rust/Cargo.toml`              | Modified             | Remove 7 Go-parity `[lints.clippy]` allows                                                                                                     |
| `apps/rhino-cli-rust/src/**` (multiple files) | Modified             | Fix Clippy violations surfaced by the allow removals; specific files determined by Phase 3 baseline run                                        |
| `apps/rhino-cli-rust/.gitignore`              | Created (_New file_) | Add project-level gitignore matching ose-public (`target/`, `dist/`, `lcov.info`, `lcov_spec.info`, `*.profraw`)                               |

## Rollback

If any phase introduces a regression that cannot be resolved:

1. Identify the failing commit with `git log --oneline`.
2. Revert the offending commit: `git revert <sha>` (creates a new revert commit — do not
   force-push).
3. Push the revert commit: `git push origin main`.
4. Verify CI recovers: `gh run list --limit=3`.

All plan changes are additive or restructural (no binary format changes, no API changes);
reverting individual commits is safe. The `.gitignore` addition (Phase 4) is independently
revertable.
