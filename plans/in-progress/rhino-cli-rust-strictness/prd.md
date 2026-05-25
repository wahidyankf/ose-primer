# Product Requirements — rhino-cli-rust Strictness Alignment

## Product Overview

This product iteration aligns `apps/rhino-cli-rust/` with the canonical `ose-public/apps/rhino-cli/`
reference implementation. It delivers three concrete changes: (1) restructure `project.json` targets
to use `--manifest-path` from the workspace root and add four missing Nx targets (`fmt`, `fmt:check`,
`deny:check`, `check:msrv`); (2) remove seven Go-parity Clippy allows from `Cargo.toml` and fix all
surfaced violations; (3) add a project-level `.gitignore`. The result is a Rust project that matches
ose-public's strictness and target surface, making ose-primer a higher-fidelity starting point for
new OSE-style repositories.

## Personas

- **Maintainer (Rust-quality hat)** — the solo developer running `nx run rhino-cli-rust:lint` and
  `cargo clippy` locally; primary consumer of the stricter Clippy ruleset and new Nx targets.
- **`swe-rust-dev` agent** — the specialized executor that applies Phase 1–4 changes; needs
  execution-grade clarity on every delivery checkbox.
- **CI pipeline** — runs `deny:check`, `check:msrv`, `lint`, and `test:quick` on every push;
  gains two new security/compatibility gates from this plan.

## Product Scope

**In scope** (this iteration):

- `project.json` target restructuring (manifest-path, add `fmt`, `fmt:check`, `deny:check`, `check:msrv`)
- `Cargo.toml` removal of seven Go-parity Clippy allows and remediation of all surfaced violations
- `apps/rhino-cli-rust/.gitignore` addition

**Out of scope** (explicitly deferred — see also brd.md `## Non-Goals`):

- Replacing `serde_norway` with `serde_yml`
- Adding `tree-sitter` dependency
- Removing `thiserror`
- Implementing `validate:specs-*` Nx targets
- Modifying `[[test]]` harness entries

## Product Risks

- **Unknown violation count**: The number of Clippy violations surfaced by removing the seven
  allows is unknown until the Phase 3 baseline run. A large violation count (e.g., >50 sites)
  may require scope trimming or a follow-on plan.
- **`cargo-deny` advisory state unknown**: `cargo deny check` may surface existing dependency
  advisories unrelated to this plan's changes. Failing advisories block Phase 5.
- **`cargo-hack` / `cargo-deny` not in toolchain**: These tools may not be installed in fork
  environments; environment setup (Phase 0) mitigates this.

## User Stories

**As a developer forking ose-primer,**
I want `apps/rhino-cli-rust/` to expose the same Nx targets as `ose-public/apps/rhino-cli/`,
so that I can run `nx run rhino-cli-rust:fmt`, `nx run rhino-cli-rust:deny:check`, and
`nx run rhino-cli-rust:check:msrv` without manual cargo invocations.

**As a code reviewer,**
I want the Clippy configuration to match ose-public (no Go-parity exemptions),
so that the same strictness applies to all Rust code in the repository.

**As a CI pipeline author,**
I want `test:quick` output to land at `lcov.info` (matching ose-public),
so that coverage tooling reads from a predictable path across both repos.

## Acceptance Criteria

```gherkin
Feature: rhino-cli-rust project.json structural alignment

  Scenario: fmt target formats code
    Given the rhino-cli-rust project.json is updated
    When I run "npx nx run rhino-cli-rust:fmt"
    Then cargo fmt runs against the app with --manifest-path
    And the command exits 0

  Scenario: fmt:check target verifies formatting
    Given the rhino-cli-rust project.json is updated
    When I run "npx nx run rhino-cli-rust:fmt:check"
    Then cargo fmt --check runs and exits 0 on a correctly formatted codebase
    And the target has cache:true with inputs for .rs files and rustfmt config

  Scenario: deny:check target scans dependencies
    Given the rhino-cli-rust project.json is updated
    When I run "npx nx run rhino-cli-rust:deny:check"
    Then cargo deny check runs and exits 0 with no advisory violations
    And the target has cache:true with Cargo.toml, Cargo.lock, deny.toml as inputs

  Scenario: check:msrv target verifies minimum supported Rust version
    Given the rhino-cli-rust project.json is updated
    When I run "npx nx run rhino-cli-rust:check:msrv"
    Then cargo hack check --rust-version runs and exits 0
    And the target has cache:true

  Scenario: lint uses sequential two-command array
    Given the rhino-cli-rust project.json is updated
    When I run "npx nx run rhino-cli-rust:lint"
    Then cargo fmt --check runs first
    And cargo clippy --all-targets -- -D warnings runs second
    And the commands run with parallel:false

  Scenario: test:quick outputs lcov.info
    Given the rhino-cli-rust project.json is updated
    When I run "npx nx run rhino-cli-rust:test:quick"
    Then coverage output lands at apps/rhino-cli-rust/lcov.info
    And the outputs array declares {projectRoot}/lcov.info

Feature: rhino-cli-rust Clippy lint alignment

  Scenario: Go-parity allows removed from Cargo.toml
    Given the Go-parity allows are removed from [lints.clippy]
    When I run "cargo clippy --manifest-path apps/rhino-cli-rust/Cargo.toml --all-targets -- -D warnings"
    Then clippy exits 0 with zero warnings or errors
    And the [lints.clippy] section matches ose-public exactly (same keys, same values)

Feature: rhino-cli-rust .gitignore

  Scenario: .gitignore filters build artifacts
    Given apps/rhino-cli-rust/.gitignore exists
    Then it ignores target/, dist/, lcov.info, lcov_spec.info, and *.profraw
    And git status shows no untracked build artifacts after a build
```

## Non-Functional Requirements

- All existing tests continue to pass after changes (`cargo test --manifest-path ... --lib` exits 0)
- `nx affected -t typecheck lint test:quick spec-coverage` passes before every push
- Changes committed thematically: project.json, Cargo.toml, code fixes, and .gitignore each
  in separate commits
