# Delivery — Primer Polyglot Demo-App CI Restoration

> Backlog plan. Promote to `plans/in-progress/` via `plan-maker` (which adds grilling + finalizes the
> TDD-shaped checklist) before execution. All code steps are expressed as fresh-checkout reproductions so
> "done" means a clean tree regenerates and the gate passes.

## Phase 0: Environment + baseline

- [ ] In the repo root run `npm install && npm run doctor -- --fix`; confirm all language toolchains
      present (`npm run doctor -- --scope minimal` exits 0). Dart/Flutter, Elixir, Go (golangci-lint),
      Rust, .NET all required to reproduce locally.
- [ ] Establish the failing baseline per app: `rm -rf apps/<app>/generated-contracts` then
      `npx nx run <app>:lint --skip-nx-cache`; record the exact failure for dart, rust, go (and the
      elixir clean-deps reproduction). These are the RED states the fixes must turn GREEN.

## Phase 1: .NET CVE — DONE (verify only)

- [ ] Confirm `c82c66c6f` is present: `git log --oneline | grep SQLitePCLRaw`. Verify
      `dotnet build apps/crud-be-fsharp-giraffe/tests/DemoBeFsgi.Tests/DemoBeFsgi.Tests.fsproj -c Release`
      and the C# test project build with 0 errors and no `NU1903`. (CI on `c82c66c6f` confirms `NU1903` is
      gone; the `.NET quality gate` now fails on a separate Class B codegen-ordering issue — Phase 5b.)

## Phase 5b: Class B (CI-only) — .NET codegen ordering + Elixir deps

- [ ] These pass fresh locally but fail in CI. Investigate CI-side, not app code:
- [ ] **.NET `CS2001`**: confirm the C# `codegen` `dependsOn` actually runs and completes before the C#
      build under the cold-cache matrix; check first-run `openapi-generator` JAR download. Fix the ordering
      / ensure contracts exist before compile. Verify on CI (not just locally, where it already passes).
- [ ] **Elixir deps**: see Phase 5; if it recurs in CI after a clean local repro passes, treat as a
      first-run Hex dependency-compile race and add a retry/soak or pin the offending dep.
- [ ] Cross-check the parallel-restore race family (the .NET `nuget.g.targets` "already exists" race seen on
      the first run; the ose-public `.NET` flake) — consider disabling intra-job restore parallelism if it
      recurs.

## Phase 2: Dart — produce a resolvable package fresh (AC-1)

- [ ] Decide remediation: **A (app-level, preferred)** make dart `codegen` emit `pubspec.yaml` (drop
      `--global-property=models` to get a full package, or append a `printf`/template `pubspec.yaml` like
      the Rust target's `Cargo.toml`); **B** activate `rhino-cli specs scaffold dart` runtime-conditionally
      (preserve byte-identical mirror + update harness-compat checker). Record the choice + rationale.
- [ ] RED: `rm -rf apps/crud-fe-dart-flutterweb/generated-contracts && npx nx run
crud-fe-dart-flutterweb:codegen --skip-nx-cache` fails (no pubspec).
- [ ] GREEN: implement the chosen fix; re-run the same command → `pubspec.yaml` exists and `flutter pub
get` resolves `crud_contracts`; `npx nx run crud-fe-dart-flutterweb:lint --skip-nx-cache` exits 0.
- [ ] If option B: run `npm run generate:bindings`; `rhino-cli` source stays byte-identical across repos
      (md5 compare vs ose-public/ose-infra) OR the divergence is deliberate with the harness-compat checker
      updated; `npx nx run rhino-cli:cross-vendor:parity-validation` exits 0.

## Phase 3: Rust — deterministic manifest under nx (AC-2)

- [ ] Diagnose: run the exact `codegen` command via `npx nx run crud-be-rust-axum:codegen --skip-nx-cache
--verbose`; capture cwd and per-`&&`-step exit codes to confirm why `Cargo.toml` is not written.
- [ ] RED: fresh `nx run crud-be-rust-axum:lint --skip-nx-cache` fails (Cargo.toml missing).
- [ ] GREEN: make manifest generation robust (replace `$(pwd)` with `{workspaceRoot}`, de-`&&`-chain into
      ordered steps, or move scaffolding to a small script). Re-run → `Cargo.toml`, `src/lib.rs`,
      `src/models/mod.rs` exist; `:lint` and `:test:quick` exit 0 fresh.

## Phase 4: Go — generate types from OpenAPI 3.1 (AC-3)

- [ ] Decide: swap `oapi-codegen` for a 3.1-capable generator (e.g., `openapi-generator -g go` models) OR
      add a 3.0 downconversion feeding only the Go types step. Record choice; keep generated type names
      stable for the app code.
- [ ] RED: fresh `nx run crud-be-golang-gin:lint --skip-nx-cache` fails (types.gen.go missing).
- [ ] GREEN: implement; re-run → `types.gen.go` present with contract types; `:lint` + `:test:quick` exit 0
      fresh. Review the generated-type diff for unintended renames.

## Phase 5: Elixir — reproduce or confirm transient (AC-4)

- [ ] Reproduce clean: `cd apps/crud-be-elixir-phoenix && mix deps.clean --all && mix deps.get && MIX_ENV=test
mix compile --warnings-as-errors`. If it fails, root-cause the offending dependency and fix; if it
      passes, document the CI failure as transient.
- [ ] Optional hygiene: move `:preferred_cli_env` from `def project` to `def cli` in
      `apps/crud-be-elixir-phoenix/mix.exs` to clear the deprecation warning.

## Phase 6: Local full-matrix verification (before push)

- [ ] For each language: `rm -rf` its `generated-contracts` then run the per-language `run-many` exactly as
      the workflow does (`nx show projects --affected … | nx run-many -t typecheck lint test:quick
specs:coverage -p …`) with `--skip-nx-cache`; all exit 0.
- [ ] `npm run lint:md` exits 0; `npx nx run rhino-cli:cross-vendor:parity-validation` exits 0 (if
      Phase 2B touched `rhino-cli`).

## Phase 7: Commit, push, CI verification

- [ ] Thematic commits per language (`fix(dart): …`, `fix(rust): …`, `fix(go): …`, `fix(elixir): …`). Push
      to `main`.
- [ ] Trigger an all-affected condition (these fixes touch app config; if not all-affected, a follow-up
      `rhino-cli`-touching commit or a manual `workflow_dispatch` exercises the full matrix). Monitor
      `PR - Quality Gate` until every per-language job concludes `success` (poll per CI-monitoring
      convention; never `gh run watch`). Fix-forward any residual failure; do not bypass.

## Phase 8: Archival

- [ ] Move the plan to `plans/done/YYYY-MM-DD__primer-polyglot-codegen-ci-restoration/`; update
      `plans/backlog/README.md` and `plans/done/README.md`.

## Verification (how to confirm done)

- AC-1..AC-5 each verified by the fresh-checkout reproduction exiting 0 for the relevant app.
- AC-6: `ose-primer` `PR - Quality Gate` concludes `success` with every per-language job green on an
  all-affected commit.
- AC-7: `rhino-cli` byte-identical mirror holds (or deliberately updated with checker) and bindings stay in
  sync.
