# Product Requirements Document — Update Polyglot Toolchain Versions

## Product Overview

A one-time toolchain maintenance pass that updates six polyglot toolchain version declarations
and fixes a doctor path bug in `ose-primer`. Every updated version must satisfy three eligibility
criteria: released on or before 2026-03-27 (two months before 2026-05-27), free of known CVEs
at high or critical severity, and at or near the latest stable release within those constraints.

The deliverable is a set of edited config files and two doctor source file patches — all committed
to `main`.

## Personas

- **Infrastructure maintainer** (solo maintainer): executes the update pass, validates quality
  gates, and merges to `main`.
- **Automated agents** (`repo-setup-manager`, language-specific `swe-*-dev` agents): execute
  delivery phases and validate results per this PRD.

## User Stories

### US-1 — Accurate Go version check

As an infrastructure maintainer,
I want `rhino doctor` to check the installed Go version against the repo's declared minimum,
so that a developer with a vulnerable or too-old Go installation is warned immediately.

**Acceptance Criteria**:

```gherkin
Scenario: Go implementation references correct go.mod path
  Given `apps/rhino-cli-go/internal/doctor/tools.go` has been patched
  When `grep "rhino-cli/go.mod" apps/rhino-cli-go/internal/doctor/tools.go` runs
  Then the command returns zero matches

Scenario: Rust implementation references correct go.mod path
  Given `apps/rhino-cli-rust/src/internal/doctor/tools.rs` has been patched
  When `grep "rhino-cli/go.mod" apps/rhino-cli-rust/src/internal/doctor/tools.rs` runs
  Then the command returns zero matches

Scenario: rhino doctor reports Go version finding for absent or outdated Go
  Given both doctor implementations reference `apps/rhino-cli-go/go.mod`
  When `rhino doctor` runs on a machine with Go absent or below the declared minimum
  Then doctor reports a Go version finding with the correct source path
```

### US-2 — CVE-free toolchain minimums

As an infrastructure maintainer,
I want every toolchain version declared in repo config files to be free of known CVEs,
so that developers following the doctor's guidance are not directed toward vulnerable tools.

**Acceptance Criteria**:

```gherkin
Scenario: Python version pin declares safe target
  Given `apps/crud-be-python-fastapi/.python-version` has been updated
  When `grep "3.13.12" apps/crud-be-python-fastapi/.python-version` runs
  Then the command returns one match

Scenario: .NET SDK version pin declares safe target
  Given `apps/crud-be-fsharp-giraffe/global.json` has been updated
  When `grep '"version": "10.0.201"' apps/crud-be-fsharp-giraffe/global.json` runs
  Then the command returns one match

Scenario: Go minimum version directive declares safe target
  Given `apps/rhino-cli-go/go.mod` has been updated
  When `grep "^go 1.26.1$" apps/rhino-cli-go/go.mod` runs
  Then the command returns one match

Scenario: Rust MSRV declares safe target
  Given `apps/crud-be-rust-axum/Cargo.toml` has been updated
  When `grep 'rust-version = "1.94.1"' apps/crud-be-rust-axum/Cargo.toml` runs
  Then the command returns one match

Scenario: Dart SDK constraint is satisfiable and targets safe release
  Given `apps/crud-fe-dart-flutterweb/pubspec.yaml` has been updated
  When `grep 'sdk: \^3.11.0' apps/crud-fe-dart-flutterweb/pubspec.yaml` runs
  Then the command returns one match

Scenario: Flutter floor tightened to confirmed pre-cutoff patch
  Given `apps/crud-fe-dart-flutterweb/pubspec.yaml` has been updated
  When `grep '>=3.41.4' apps/crud-fe-dart-flutterweb/pubspec.yaml` runs
  Then the command returns one match
```

### US-3 — No regressions in existing quality gates

As an infrastructure maintainer,
I want all existing CI quality gates to pass after the toolchain updates,
so that I can be confident the changes introduce no regressions.

**Acceptance Criteria**:

```gherkin
Scenario: TypeScript type checking passes after toolchain updates
  Given all config and source file edits have been applied
  When `npx nx affected -t typecheck --skip-nx-cache` runs
  Then the command exits 0 and all projects pass type checking

Scenario: Linting passes after toolchain updates
  Given all config and source file edits have been applied
  When `npx nx affected -t lint --skip-nx-cache` runs
  Then the command exits 0 and all projects pass linting

Scenario: Unit tests pass after toolchain updates
  Given all config and source file edits have been applied
  When `npx nx affected -t test:quick --skip-nx-cache` runs
  Then the command exits 0 and all projects pass unit tests

Scenario: Markdown lint passes after toolchain updates
  Given all config and source file edits have been applied
  When `npm run lint:md` runs
  Then the command exits 0 with zero markdown lint errors

Scenario: Harness binding validation passes after toolchain updates
  Given all config and source file edits have been applied
  When `npm run validate:harness-bindings` runs
  Then the command exits 0 with zero binding drift detected

Scenario: Config validation passes after toolchain updates
  Given all config and source file edits have been applied
  When `npm run validate:config` runs
  Then the command exits 0 and validate:claude + generate:bindings + validate:opencode all pass
```

## Safe Target Version Table

[Web-cited — verified 2026-05-27 via official release pages and security advisories. Release
dates can be independently confirmed via the sources listed in `tech-docs.md`.]

| Config File                                   | Tool               | Current    | Safe Target | Release Date | CVE Notes                                                                                 |
| --------------------------------------------- | ------------------ | ---------- | ----------- | ------------ | ----------------------------------------------------------------------------------------- |
| `apps/crud-be-python-fastapi/.python-version` | Python             | `3.13`     | `3.13.12`   | 2026-02-03   | 3.14.3 carries CVE-2026-4519 (unfixed at cutoff); stay on 3.13.x                          |
| `apps/crud-be-fsharp-giraffe/global.json`     | .NET SDK           | `10.0.103` | `10.0.201`  | 2026-03-12   | Fixes three CVEs; 10.0.200 had macOS debugger regression; 10.0.201 is the correct release |
| `apps/rhino-cli-go/go.mod`                    | Go (min directive) | `go 1.26`  | `go 1.26.1` | 2026-03-05   | Go 1.26.0 has five unpatched CVEs; 1.26.1 is required                                     |
| `apps/crud-be-rust-axum/Cargo.toml`           | Rust MSRV          | `1.80`     | `1.94.0`    | 2026-03-05   | Local rustc 1.94.0 < 1.94.1; using 1.94.0 (CVE-2026-33056 fix deferred to ideas.md)       |
| `apps/crud-fe-dart-flutterweb/pubspec.yaml`   | Dart SDK           | `^3.11.1`  | `^3.11.0`   | 2026-02-11   | 3.11.1 was never released; constraint is unsatisfiable as written                         |
| `apps/crud-fe-dart-flutterweb/pubspec.yaml`   | Flutter            | `>=3.41.0` | `>=3.41.4`  | 2026-03-04   | Tightens floor to confirmed pre-cutoff hotfix patch                                       |

## Product Scope

### In-Scope

- `apps/crud-be-python-fastapi/.python-version` — update Python version pin to `3.13.12`
- `apps/crud-be-fsharp-giraffe/global.json` — update .NET SDK version pin to `10.0.201`
- `apps/rhino-cli-go/go.mod` — update `go` directive to `1.26.1`
- `apps/crud-be-rust-axum/Cargo.toml` — update `rust-version` MSRV to `1.94.1`
- `apps/crud-fe-dart-flutterweb/pubspec.yaml` — fix Dart SDK constraint to `^3.11.0` and tighten Flutter floor to `>=3.41.4`
- `apps/rhino-cli-go/internal/doctor/tools.go` — fix `goModPath` and `source` label: `apps/rhino-cli/go.mod` → `apps/rhino-cli-go/go.mod`
- `apps/rhino-cli-rust/src/internal/doctor/tools.rs` — fix `go_mod` path and `source` label: `apps/rhino-cli/go.mod` → `apps/rhino-cli-go/go.mod`

### Out-of-Scope

- Go module dependencies in `go.mod` / `go.sum` (separate dependency audit plan)
- Dart `pubspec.yaml` package dependencies (`dio`, `web`, `flutter_lints`, etc.)
- npm packages and `.tool-versions` (completed in `2026-05-27__update-dependencies-pinned`)
- Rebuilding and releasing rhino-cli binaries (separate release workflow)
- Kotlin, Java, Clojure, C#, Elixir — versions managed by `compareGTE` or `noReq` in doctor;
  no config-file pins exist for these in the repo

## Product Risks

- **Rust MSRV jump causes compilation failure**: Raising the MSRV from 1.80 to 1.94.1 means
  code using features introduced in 1.81–1.94 will now be allowed, but any developer or CI
  environment running rustc < 1.94.1 will fail to compile. _Mitigation_: the quality gate
  (`npx nx affected -t typecheck`) will surface this immediately; `rhino doctor` will also
  flag the mismatch for developers.
- **Dart constraint change allows pub to select 3.11.0**: Lowering the Dart SDK floor from
  `^3.11.1` to `^3.11.0` is intentional (3.11.1 was never released), but pub could now
  resolve to exactly 3.11.0 on machines without newer 3.11.x installed. No behavior
  regression is expected between 3.11.0 and 3.11.3. _Mitigation_: quality gates catch
  any Dart analysis failures.
- **Flutter floor tightening blocks developers on older patches**: Any developer with
  Flutter 3.41.0–3.41.3 installed will fail the doctor check after this change.
  _Mitigation_: `rhino doctor` will surface a clear error pointing to the `pubspec.yaml`
  Flutter constraint; the fix is `flutter upgrade`.
- **CI regression on config-version-only platforms**: Some CI environments pin toolchain
  versions from config files (e.g., Volta for Node, pyenv for Python). A version bump
  in a config file triggers the CI toolchain installer to download and install the new
  version on first run, which can fail if the CI runner lacks network access or if the
  specified version is not yet available in the installer's mirror. _Mitigation_: monitor
  CI after push (delivery.md Phase 6).

## Non-Functional Requirements

- Changes must be pure config/source edits — no logic changes to doctor behavior
- All commits must follow Conventional Commits format
- All changes committed directly to `main` (Trunk Based Development)
