# Update Polyglot Toolchain Versions (rhino doctor config files)

## Status

In Progress

## Context

The `ose-primer` repository uses `rhino doctor` (implemented in both `apps/rhino-cli-go` and
`apps/rhino-cli-rust`) to validate that the local developer toolchain matches required versions.
The doctor reads required versions from config files scattered across the repo — it has no
hardcoded version strings of its own.

The previous dependency plan (`2026-05-27__update-dependencies-pinned`) updated npm packages,
`.tool-versions` (erlang/elixir), and the Volta pins for Node.js and npm. It did **not** update
the six other toolchain config files read by the doctor:

| Config File                                   | Tool               | Current    | Safe Target |
| --------------------------------------------- | ------------------ | ---------- | ----------- |
| `apps/crud-be-python-fastapi/.python-version` | Python             | `3.13`     | `3.13.12`   |
| `apps/crud-be-fsharp-giraffe/global.json`     | .NET SDK           | `10.0.103` | `10.0.201`  |
| `apps/rhino-cli-go/go.mod`                    | Go (min directive) | `go 1.26`  | `go 1.26.1` |
| `apps/crud-be-rust-axum/Cargo.toml`           | Rust MSRV          | `1.80`     | `1.94.1`    |
| `apps/crud-fe-dart-flutterweb/pubspec.yaml`   | Dart SDK           | `^3.11.1`  | `^3.11.0`   |
| `apps/crud-fe-dart-flutterweb/pubspec.yaml`   | Flutter            | `>=3.41.0` | `>=3.41.4`  |

A separate bug also exists: both doctor implementations reference `apps/rhino-cli/go.mod`
(a path from the `ose-public` parent repo) but the actual file in `ose-primer` is at
`apps/rhino-cli-go/go.mod`. This silently disables the Go version check in `rhino doctor`.

This plan fixes the path bug and updates all six config files to the latest versions that are
both CVE-free and released on or before the cutoff date (2026-03-27).

**Cutoff date for eligibility**: 2026-03-27 (two months before plan creation date 2026-05-27).
Versions released after this date are excluded from target selection.

## Scope

**In-scope**:

- `apps/crud-be-python-fastapi/.python-version` — update Python version pin
- `apps/crud-be-fsharp-giraffe/global.json` — update .NET SDK version pin
- `apps/rhino-cli-go/go.mod` — update `go` directive to require `go 1.26.1`
- `apps/crud-be-rust-axum/Cargo.toml` — update `rust-version` MSRV field
- `apps/crud-fe-dart-flutterweb/pubspec.yaml` — fix Dart SDK constraint and tighten Flutter floor
- `apps/rhino-cli-go/internal/doctor/tools.go` — fix `apps/rhino-cli/go.mod` path → `apps/rhino-cli-go/go.mod`
- `apps/rhino-cli-rust/src/internal/doctor/tools.rs` — same path fix

**Out-of-scope**:

- Go module dependencies in `go.mod` / `go.sum` (indirect dep upgrades — separate plan)
- Dart `pubspec.yaml` package dependencies (`dio`, `web`, etc.) — separate plan
- npm packages (covered by `2026-05-27__update-dependencies-pinned`)
- `.tool-versions` erlang/elixir (covered by `2026-05-27__update-dependencies-pinned`)
- Volta Node.js/npm pins (covered by `2026-05-27__update-dependencies-pinned`)
- Rebuilding and releasing rhino-cli binaries (separate release plan)

## Success Criteria

- All six toolchain config files declare the safe target version
- Both doctor implementations reference `apps/rhino-cli-go/go.mod` (correct path)
- All quality gates pass: typecheck, lint, test:quick, validate:mermaid,
  validate:harness-bindings, vendor-audit
- `git status` clean, all commits pushed to `origin/main`

## Documents

- [Business Requirements](./brd.md)
- [Product Requirements](./prd.md)
- [Technical Documentation](./tech-docs.md)
- [Delivery Checklist](./delivery.md)
