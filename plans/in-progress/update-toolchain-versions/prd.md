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

- AC-1: `apps/rhino-cli-go/internal/doctor/tools.go` references `apps/rhino-cli-go/go.mod`
  (not `apps/rhino-cli/go.mod`)
- AC-2: `apps/rhino-cli-rust/src/internal/doctor/tools.rs` references `apps/rhino-cli-go/go.mod`
  (not `apps/rhino-cli/go.mod`)
- AC-3: `rhino doctor` (both implementations) reports a Go version finding when Go is absent
  or below the declared minimum

### US-2 — CVE-free toolchain minimums

As an infrastructure maintainer,
I want every toolchain version declared in repo config files to be free of known CVEs,
so that developers following the doctor's guidance are not directed toward vulnerable tools.

**Acceptance Criteria**:

- AC-4: `.python-version` declares `3.13.12` (released 2026-02-03, no high/critical CVEs)
- AC-5: `global.json` declares SDK `10.0.201` (released 2026-03-12, fixes CVE-2026-26127,
  CVE-2026-26130, CVE-2026-26131)
- AC-6: `go.mod` `go` directive is `1.26.1` (released 2026-03-05, fixes five Go CVEs including
  CVE-2026-27137, CVE-2026-27138, CVE-2026-27142, CVE-2026-25679, CVE-2026-27139)
- AC-7: `Cargo.toml` `rust-version` is `1.94.1` (released 2026-03-26, fixes CVE-2026-33056)
- AC-8: `pubspec.yaml` Dart SDK constraint is `^3.11.0` (Dart 3.11.0 released 2026-02-11;
  3.11.1 was never released — this fixes the unsatisfiable constraint)
- AC-9: `pubspec.yaml` Flutter constraint is `>=3.41.4` (3.41.4 released ~early March 2026,
  confirmed pre-cutoff; tightens the floor from the vulnerable 3.41.0)

### US-3 — No regressions in existing quality gates

As an infrastructure maintainer,
I want all existing CI quality gates to pass after the toolchain updates,
so that I can be confident the changes introduce no regressions.

**Acceptance Criteria**:

- AC-10: `npx nx affected -t typecheck` exits 0 (all projects pass type checking)
- AC-11: `npx nx affected -t lint` exits 0 (all projects pass linting)
- AC-12: `npx nx affected -t test:quick` exits 0 (all projects pass unit tests)
- AC-13: `npm run validate:mermaid` exits 0 (no diagram violations)
- AC-14: `npm run validate:harness-bindings` exits 0 (no binding drift)
- AC-15: `npm run vendor-audit` exits 0 (no vendor audit failures)

## Safe Target Version Table

[Web-cited — verified 2026-05-27 via official release pages and security advisories. Release
dates can be independently confirmed via the sources listed in `tech-docs.md`.]

| Config File                                   | Tool               | Current    | Safe Target | Release Date      | CVE Notes                                                                                 |
| --------------------------------------------- | ------------------ | ---------- | ----------- | ----------------- | ----------------------------------------------------------------------------------------- |
| `apps/crud-be-python-fastapi/.python-version` | Python             | `3.13`     | `3.13.12`   | 2026-02-03        | 3.14.3 carries CVE-2026-4519 (unfixed at cutoff); stay on 3.13.x                          |
| `apps/crud-be-fsharp-giraffe/global.json`     | .NET SDK           | `10.0.103` | `10.0.201`  | 2026-03-12        | Fixes three CVEs; 10.0.200 had macOS debugger regression; 10.0.201 is the correct release |
| `apps/rhino-cli-go/go.mod`                    | Go (min directive) | `go 1.26`  | `go 1.26.1` | 2026-03-05        | Go 1.26.0 has five unpatched CVEs; 1.26.1 is required                                     |
| `apps/crud-be-rust-axum/Cargo.toml`           | Rust MSRV          | `1.80`     | `1.94.1`    | 2026-03-26        | MSRV update; 1.94.1 fixes CVE-2026-33056 in Cargo's tar handling                          |
| `apps/crud-fe-dart-flutterweb/pubspec.yaml`   | Dart SDK           | `^3.11.1`  | `^3.11.0`   | 2026-02-11        | 3.11.1 was never released; constraint is unsatisfiable as written                         |
| `apps/crud-fe-dart-flutterweb/pubspec.yaml`   | Flutter            | `>=3.41.0` | `>=3.41.4`  | ~early March 2026 | Tightens floor to confirmed pre-cutoff hotfix patch                                       |

## Out-of-Scope

- Go module dependencies in `go.mod` / `go.sum` (separate dependency audit plan)
- Dart `pubspec.yaml` package dependencies (`dio`, `web`, `flutter_lints`, etc.)
- npm packages and `.tool-versions` (completed in `2026-05-27__update-dependencies-pinned`)
- Rebuilding and releasing rhino-cli binaries (separate release workflow)
- Kotlin, Java, Clojure, C#, Elixir — versions managed by `compareGTE` or `noReq` in doctor;
  no config-file pins exist for these in the repo

## Non-Functional Requirements

- Changes must be pure config/source edits — no logic changes to doctor behavior
- All commits must follow Conventional Commits format
- All changes committed directly to `main` (Trunk Based Development)
