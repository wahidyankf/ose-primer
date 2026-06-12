# Product Requirements — lint-safety-parity (ose-primer)

## Product overview

Deliver an equal, enforced cross-language lint/safety strictness gate to `ose-primer` for the
dimensions it owns (D1 Rust, D3 C#, D4 Python, D6 Dockerfile, D7 shell, D8 GitHub Actions YAML),
using the locked **clean-then-gate** rollout. The "product" is the combination of: updated build
configs (`Cargo.toml`, C# props, `pyproject.toml`), new lint configs (`.hadolint.yaml`,
`.shellcheckrc`, optional `.github/actionlint.yaml`), new/updated Nx lint targets, CI quality-gate
jobs, local hook wiring, a rationale doc, and governance/convention doc updates.

## Personas

Solo-maintainer repo — personas are hats and consuming agents.

- **Maintainer** — flips the gates and owns a green build.
- **Downstream forker** — clones ose-primer and inherits the strict floor.
- **`repo-setup-manager`** — establishes the Phase 0 baseline.
- **`swe-rust-dev` / `swe-csharp-dev`** — execute language-specific gate work.
- **CI (GitHub Actions)** — enforces the gates on every PR/push.
- **`plan-execution-checker`** — validates the finished work.

## User stories

- **US-1 (Rust safety)**: As a maintainer, I want `crud-be-rust-axum` to forbid unsafe code and
  carry the full public `[lints]` standard, so that no contributor can silently introduce `unsafe`.
- **US-2 (C# strictness)**: As a maintainer, I want the C# projects to enforce `AnalysisLevel=latest-All`
  and SonarAnalyzer at error severity, so that the C# demo matches the cross-repo strictness bar.
- **US-3 (Python strictness)**: As a maintainer, I want `basedpyright` strict and an expanded ruff
  select set, so that the Python demo enforces strict typing and broad lint coverage.
- **US-4 (Dockerfile lint)**: As a maintainer, I want hadolint to fail on warning-and-above for all
  Dockerfiles, so that container-image defects are caught before runtime.
- **US-5 (Shell lint)**: As a maintainer, I want shellcheck to fail on warning-and-above for repo
  scripts, so that shell defects are caught at commit/PR time.
- **US-6 (CI YAML lint)**: As a maintainer, I want actionlint to validate workflow YAML, so that
  malformed GitHub Actions are caught before they break CI.
- **US-7 (Rationale)**: As a downstream forker, I want a plain-language rationale doc, so that I
  understand each strictness decision and the D5 deferral.
- **US-8 (Sync deviation honesty)**: As a maintainer, I want the M1 main-to-main deviation recorded,
  so that bypassing the ose-primer Sync Convention is explicit and justified.

## Acceptance criteria (Gherkin)

> Step-keyword cardinality: each scenario uses exactly one primary `Given` / `When` / `Then`;
> extras chain with `And`.

```gherkin
Feature: Rust forbids unsafe and carries the full lint standard (D1)

  Scenario: Cargo.toml gains the public [lints] standard
    Given apps/crud-be-rust-axum/Cargo.toml currently has no [lints] table
    When the public verbatim [lints.rust] and [lints.clippy] standard is added
    Then "unsafe_code = \"forbid\"" is present under [lints.rust]
    And "npx nx run crud-be-rust-axum:lint" exits 0

  Scenario: Introducing unsafe code fails the build
    Given the [lints] standard is in place with unsafe_code = "forbid"
    When an unsafe block is added to apps/crud-be-rust-axum/src
    Then "cargo build" fails with the forbidden-unsafe error
    And removing the unsafe block restores a green build
```

```gherkin
Feature: C# strict gate (D3)

  Scenario: AnalysisLevel latest-All and Sonar enforced
    Given apps/crud-be-csharp-aspnetcore/Directory.Build.props lacks AnalysisLevel
    When AnalysisLevel=latest-All is added and SonarAnalyzer runs at error severity
    Then "npx nx run crud-be-csharp-aspnetcore:lint" exits 0 after cleanup
    And a deliberately Sonar-flagged construct fails the build
```

```gherkin
Feature: Python strict gate (D4)

  Scenario: basedpyright strict replaces pyright basic
    Given apps/crud-be-python-fastapi/pyproject.toml sets typeCheckingMode "basic" via pyright
    When pyright is swapped for basedpyright with typeCheckingMode "strict"
    Then "npx nx run crud-be-python-fastapi:typecheck" exits 0 after cleanup
    And a deliberately untyped function fails strict type-checking

  Scenario: ruff select set is expanded
    Given the ruff select set is E,F,I,N,UP,B,A,C4,PT
    When the select set is expanded to E,W,F,B,UP,SIM,I,N,S,RUF,C4,T20,ANN excluding ANN101 and ANN102
    Then "npx nx run crud-be-python-fastapi:lint" exits 0 after cleanup
    And per-file-ignores exempt tests from S101 and ANN
```

```gherkin
Feature: Dockerfile lint (D6)

  Scenario: hadolint fails on warning-and-above
    Given a .hadolint.yaml with failure-threshold warning exists
    When hadolint runs against all repository Dockerfiles
    Then the hadolint gate exits 0 after cleanup
    And a deliberately bad Dockerfile instruction fails the gate
```

```gherkin
Feature: Shell lint (D7)

  Scenario: shellcheck fails on warning-and-above
    Given a .shellcheckrc with shell=bash and external-sources=true exists
    When shellcheck runs with --severity=warning against repo scripts
    Then the shellcheck gate exits 0 after cleanup
    And a deliberately bad shell construct fails the gate
```

```gherkin
Feature: GitHub Actions YAML lint (D8)

  Scenario: actionlint validates workflow YAML
    Given actionlint is wired into CI and the local hook
    When actionlint runs against .github/workflows
    Then the actionlint gate exits 0 after cleanup
    And a deliberately malformed workflow expression fails the gate
```

```gherkin
Feature: Gates enforced in CI and local hooks (gating policy)

  Scenario: A new linter blocks both CI and local hooks
    Given hadolint, shellcheck, and actionlint gates are enabled
    When a violating change is committed and pushed
    Then the local pre-commit or pre-push hook rejects it
    And the pr-quality-gate.yml CI job also reports failure
```

```gherkin
Feature: Documentation and sync-deviation honesty (M1, deliverables c/d)

  Scenario: Rationale doc and governance updates land
    Given the strict gates are enabled
    When the rationale doc and governance/convention docs are written
    Then docs/explanation/lint-safety-parity-decisions.md covers every deviation-matrix row plus the D5 deferral
    And the M1 main-to-main sync deviation and its justification are recorded in tech-docs.md and the rationale doc
```

## Product scope

### In-scope features

- D1 Rust `[lints]` standard + `forbid(unsafe_code)` on `crud-be-rust-axum`.
- D3 C# `AnalysisLevel=latest-All` + Sonar-at-error on the two C# projects.
- D4 Python basedpyright strict + expanded ruff select.
- D6/D7/D8 hadolint / shellcheck / actionlint configs, Nx lint targets, CI jobs, hook wiring.
- Rationale doc + governance/convention doc updates.
- M1 sync-deviation record.

### Out-of-scope features

- D2 (F#), D5 (TS DDD), D9 (IaC), D10 (golangci removal — primer keeps it).
- Sibling repos' execution.
- New linters beyond the locked dimension set.

## Product-level risks

| Risk                                                        | Mitigation                                                                 |
| ----------------------------------------------------------- | -------------------------------------------------------------------------- |
| Gate enabled before cleanup → first run red                 | Clean-then-gate ordering enforced in delivery phases.                      |
| basedpyright/`latest-All` surface large latent backlogs     | Dedicated cleanup steps budgeted before each gate flip.                    |
| hadolint/shellcheck false positives on idiomatic constructs | Justified per-rule `ignore`/disable entries in the config files.           |
| Hook wiring slows pre-commit/pre-push noticeably            | Scope shell/Docker/YAML gates to changed files where the tool supports it. |
