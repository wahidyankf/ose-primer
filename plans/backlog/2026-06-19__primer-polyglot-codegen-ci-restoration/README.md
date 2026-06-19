# Primer Polyglot Demo-App CI Restoration

## Context

The `ose-primer` PR quality gate runs a **per-language matrix** (`.github/workflows/pr-quality-gate.yml`)
that builds, lints, tests, and coverage-checks the polyglot `crud-*` demo apps for each language. For a
long time these jobs never actually exercised the demo apps, for two compounding reasons:

1. The matrix command was malformed (`nx affected … --projects='tag:lang:X'`), so it either ran nothing
   useful or failed at the executor level. Fixed 2026-06-19 (commit `9ede6a70e`) — the per-language jobs
   now correctly run each language's affected projects via `nx show projects --affected … | nx run-many`.
2. A normal PR rarely makes the demo apps "affected." They only become affected when a **shared** project
   they all depend on (notably `rhino-cli`, the codegen/specs tooling) changes.

On 2026-06-19 a change to `rhino-cli` (adding the `researcher` role to `AGENT_ROLES` for the
`web-research-maker → web-researcher` rename) made **every project affected** for the first time under the
now-correct matrix. That combination caused the demo apps to build **fresh** in CI — and revealed they
are **broken on a clean checkout**. `generated-contracts/` is gitignored, so local working trees kept
stale-but-working contracts that masked the breakage; CI checks out clean and regenerates, so it fails.

This plan captures **every primer demo-app gate failure** observed, its root cause, and the remediation,
so the polyglot showcase builds green from a clean checkout.

## Scope

**In scope** — fresh-checkout codegen + green per-language CI for the `crud-*` demo apps:

- `crud-be-csharp-aspnetcore`, `crud-be-fsharp-giraffe` (.NET) — **SQLite CVE — already fixed**, see below
- `crud-fe-dart-flutterweb` (Dart) — dormant `rhino-cli specs scaffold dart`
- `crud-be-rust-axum` (Rust) — codegen orchestration produces no `Cargo.toml` under nx fresh
- `crud-be-golang-gin` (Go) — `oapi-codegen` vs OpenAPI 3.1
- `crud-be-elixir-phoenix` + elixir codegen libs — CI "errors on dependencies" (likely transient — verify)
- The cross-repo `rhino-cli` byte-identical parity invariant, where remediation touches shared tooling

**Out of scope**:

- The `web-research-maker → web-researcher` rename and the `researcher`/`tester` role additions (done +
  pushed; correct and green except for this pre-existing demo-app debt they exposed).
- ose-public / ose-infra (no polyglot demo apps; their CI is green for the same commits).
- Non-`crud-*` projects.

## Status of each gate (as of 2026-06-19)

| Gate (lang)            | Symptom on fresh CI                                       | Root cause                                                                                                                                                                 | Status                                                                  |
| ---------------------- | --------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| .NET CVE (C#/F#)       | `NU1903` SQLite CVE, build-as-error                       | `Microsoft.EntityFrameworkCore.Sqlite` 10.0.8 pulls vulnerable `SQLitePCLRaw.bundle_e_sqlite3` 2.1.11                                                                      | **DONE** — pinned 3.0.3 (commit `c82c66c6f`); CI confirms `NU1903` gone |
| Dart (Class A)         | `flutter pub get`: no `pubspec.yaml` for `crud_contracts` | `rhino-cli specs scaffold dart` is a dormant stub ("dormant in ose-public"); codegen is models-only                                                                        | TODO                                                                    |
| Rust (Class A)         | `cargo` lint/build: `Cargo.toml` missing                  | nx-orchestrated `codegen` does not leave a `Cargo.toml`; the standalone `openapi-generator` step works, the nx-run chain does not                                          | TODO (confirm mechanism)                                                |
| Go (Class A)           | `golangci-lint`/build: `types.gen.go` missing             | `oapi-codegen` warns it does not support OpenAPI 3.1.x and emits no types                                                                                                  | TODO                                                                    |
| .NET codegen (Class B) | `CS2001`: generated `*.cs` contract files not found       | C# codegen **succeeds fresh locally** (contracts produced) but CI shows the C# build running before/without the generated contracts — a CI-only ordering/environment issue | TODO (investigate CI ordering)                                          |
| Elixir (Class B)       | `** (Mix) Can't continue due to errors on dependencies`   | **Passes fresh locally** (typecheck + full `mix compile --warnings-as-errors` clean) — CI deps-compile/network flake                                                       | TODO (verify / re-run)                                                  |

**Two failure classes.** _Class A — locally reproducible_ (Dart/Rust/Go): a cleaned tree + `--skip-nx-cache`
fails identically to CI; these are genuine fresh-codegen bugs. _Class B — CI-only_ (.NET codegen, Elixir):
the same cleaned-tree reproduction **passes locally** but CI fails; these point at CI-environment causes —
codegen `dependsOn` ordering under the cold-cache matrix, first-run generator-JAR/Hex dependency download
races, or parallel-restore races (the .NET NuGet `nuget.g.targets` "already exists" race and the ose-public
`.NET` flake are the same family). Class B needs CI-side investigation, not local code fixes.

## Approach summary

Restore **fresh-checkout codegen** for each demo app so the per-language gate passes on a clean tree:

- **Dart**: either activate the dormant `specs scaffold dart` (note the cross-repo parity tradeoff) or make
  the dart codegen emit a complete package (with `pubspec.yaml`) so the scaffold is unnecessary.
- **Rust**: make the `codegen` target deterministically produce `Cargo.toml` + `src/lib.rs` +
  `src/models/mod.rs` under nx (investigate the `$(pwd)`/cwd and `&&`-chain short-circuit behavior).
- **Go**: replace/upgrade `oapi-codegen` with an OpenAPI-3.1-capable generator, or downconvert the bundled
  spec to 3.0.x for the Go types generation.
- **Elixir**: reproduce the CI deps failure (fresh `mix deps.get && mix compile`); if transient, document +
  add a retry/soak; if real, fix the offending dependency.
- **Parity**: any change to `rhino-cli` (e.g., activating `specs scaffold dart`) must keep the
  `.claude`/`.opencode`/`.amazonq` and cross-repo byte-identical invariants intact, or consciously and
  reviewably diverge with the harness-compatibility checker updated. Prefer app-level codegen fixes over
  diverging shared tooling where possible.

## Document map

- [brd.md](./brd.md) — why this matters (CI honesty, template credibility, security posture)
- [prd.md](./prd.md) — per-gate requirements + Gherkin acceptance criteria (each gate green on fresh CI)
- [tech-docs.md](./tech-docs.md) — per-gate root-cause analysis, remediation options + tradeoffs, the
  gitignored-`generated-contracts` mechanism, and the cross-repo parity invariant
- [delivery.md](./delivery.md) — phased, TDD-shaped delivery checklist

## Provenance

Root causes were diagnosed live on 2026-06-19 by reproducing each gate from a clean `generated-contracts/`
with `--skip-nx-cache`. The .NET CVE fix (commit `c82c66c6f`) was verified by `dotnet build` of both test
projects (0 errors, no `NU1903`). The remaining gates' symptoms are reproduced; some root-cause mechanisms
(Rust nx orchestration, Elixir CI deps) carry a "confirm during execution" note.
