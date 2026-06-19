# Product Requirements — Primer Polyglot Demo-App CI Restoration

## Product Overview

Restore fresh-checkout codegen and green per-language CI for the `ose-primer` `crud-*` demo apps. Each
language's quality gate (`typecheck`, `lint`, `test:quick`, `specs:coverage`) must pass from a clean tree
where `generated-contracts/` does not yet exist and is produced solely by the `codegen` target chain.

## Personas

Solo-maintainer repo — hats and consumers:

- **Maintainer (template-owner hat)** — runs the gate; needs each language green on a clean checkout.
- **Template adopter** — clones the repo; expects `nx run-many -t …` for the demo apps to succeed.
- **`pr-quality-gate` workflow** — orchestrates the per-language matrix; needs the demo apps fixable.

## User Stories

- **US-1**: As the maintainer, I want the Dart demo app's `codegen` to produce a valid `pubspec.yaml` so
  `flutter pub get` resolves the `crud_contracts` path package on a clean checkout.
- **US-2**: As the maintainer, I want the Rust demo app's `codegen` to deterministically produce
  `Cargo.toml` + `src/lib.rs` + `src/models/mod.rs` under nx, so `cargo` lint/test pass fresh.
- **US-3**: As the maintainer, I want the Go demo app's contract types generated from the OpenAPI 3.1 spec
  so `go build`/`golangci-lint` find the `contracts` package fresh.
- **US-4**: As the maintainer, I want the Elixir gate's CI dependency failure either reproduced + fixed or
  shown to be transient, so the gate is reliably green.
- **US-5**: As the maintainer, I want the .NET apps free of the `NU1903` SQLite advisory (already done) and
  building green alongside the others.
- **US-6**: As the maintainer, I want any change to shared `rhino-cli` to preserve the cross-repo
  byte-identical mirror invariant (or consciously, reviewably diverge with the checker updated).

## Acceptance Criteria (Gherkin)

### AC-1: Dart codegen produces a resolvable package on a clean checkout

```gherkin
Scenario: Fresh Dart codegen yields a usable crud_contracts package
  Given apps/crud-fe-dart-flutterweb/generated-contracts does not exist
  When nx run crud-fe-dart-flutterweb:codegen runs with --skip-nx-cache
  Then a pubspec.yaml exists at apps/crud-fe-dart-flutterweb/generated-contracts/
  And flutter pub get for crud-fe-dart-flutterweb resolves crud_contracts without error
  And nx run crud-fe-dart-flutterweb:lint exits 0
```

### AC-2: Rust codegen produces a buildable crate on a clean checkout

```gherkin
Scenario: Fresh Rust codegen yields Cargo.toml and module wiring
  Given apps/crud-be-rust-axum/generated-contracts does not exist
  When nx run crud-be-rust-axum:codegen runs with --skip-nx-cache
  Then Cargo.toml, src/lib.rs, and src/models/mod.rs exist under generated-contracts/
  And nx run crud-be-rust-axum:lint and :test:quick exit 0
```

### AC-3: Go contract types generate from the OpenAPI 3.1 spec

```gherkin
Scenario: Fresh Go codegen yields types.gen.go from a 3.1 spec
  Given apps/crud-be-golang-gin/generated-contracts does not exist
  When nx run crud-be-golang-gin:codegen runs with --skip-nx-cache
  Then types.gen.go exists with the contract types
  And nx run crud-be-golang-gin:lint and :test:quick exit 0
```

### AC-4: Elixir gate is reliably green

```gherkin
Scenario: Elixir demo app builds from clean deps
  Given a clean _build and deps for crud-be-elixir-phoenix
  When mix deps.clean --all and mix deps.get and MIX_ENV=test mix compile --warnings-as-errors run
  Then compilation succeeds with exit 0 and no dependency errors
  And the Elixir quality gate job in the ose-primer PR - Quality Gate workflow concludes success on the commit that fixes or documents the dependency issue
```

### AC-5: .NET apps are CVE-clean and build (already satisfied)

```gherkin
Scenario: .NET apps clear NU1903
  Given the SQLitePCLRaw.bundle_e_sqlite3 3.0.3 pin is present
  When dotnet build runs for DemoBeCsas.Tests and DemoBeFsgi.Tests
  Then the build succeeds with 0 errors and no NU1903 advisory
```

### AC-6: Whole matrix green on an all-affected commit

```gherkin
Scenario: PR quality gate passes when all demo apps are affected
  Given a commit that makes every demo app affected (e.g., a rhino-cli change)
  When the ose-primer PR - Quality Gate workflow runs
  Then every per-language quality-gate job concludes success
```

### AC-7: Cross-repo parity preserved

```gherkin
Scenario: rhino-cli changes keep the mirror invariant
  Given a change to rhino-cli source or its dormant scaffold commands
  When npm run generate:bindings and the harness parity checks run
  Then .opencode / .amazonq / .codex bindings stay in sync
  And the cross-repo byte-identical rhino-cli invariant holds or is consciously updated with the harness-compatibility checker
```

## Product Scope

### In Scope

- Fresh-checkout codegen fixes for Dart, Rust, Go demo apps
- Elixir CI dependency failure reproduction + fix-or-document
- Keeping the .NET CVE fix landed and green
- Preserving (or consciously updating) the cross-repo `rhino-cli` parity invariant

### Out of Scope

- Codegen architecture redesign; new languages/apps
- The completed rename + role additions
- ose-public / ose-infra changes (no demo apps)

## Product-Level Risks

- **Generator swap churn (Go)**: changing the Go generator alters generated types. Mitigation: scope to the
  types target; keep public names stable; review the diff.
- **Shared-tooling divergence (Dart)**: fixing via `rhino-cli` risks the mirror invariant. Mitigation:
  prefer an app-level codegen fix; if tooling changes, keep it runtime-conditional + update the checker.
- **Local-green-CI-red drift**: reproduce every fix on a cleaned tree with `--skip-nx-cache` before push.
