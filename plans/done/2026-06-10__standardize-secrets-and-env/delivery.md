# Delivery — Standardize Secrets and Environment-Variable Storage (ose-primer)

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase is
> not complete until its gate is green; do not start phase N+1 while any gate check fails.

All checkboxes are `[AI]` unless tagged otherwise. Commit + push at each phase gate (Conventional
Commits, `origin main`).

> **PR-override (R5)**: this plan pushes **directly to `ose-primer` `main`** (`worktree-to-main`),
> bypassing the normal sibling-sync PR-only rule. Explicit, one-off, invoker-owned. No `--force` is
> ever used. See [tech-docs.md §1](./tech-docs.md#1-resolved-deviation-matrix-all-16-decisions).

<!-- separates adjacent blockquotes (markdownlint MD028) -->

> **Dual-implementation rule**: every `rhino-cli` change is authored **spec-first**, implemented in
> `apps/rhino-cli-rust/` (canonical) **then** `apps/rhino-cli-go/` (twin), and verified byte-identical
> by the shadow-diff parity harness before its phase gate closes.

<!-- separates adjacent blockquotes (markdownlint MD028) -->

> **Safety rule for the whole plan**: no `.env`, `.env.local`, or other real secret file is ever
> deleted. Primer performs **no layout migration** (R12), so no relocation occurs; any hypothetical
> real-file relocation would be a `[HUMAN]` step (env-file-access guard).

## Worktree

Worktree path: `worktrees/standardize-secrets-and-env/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree standardize-secrets-and-env
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before deleting
the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0 — Environment Setup + Baseline

> _Executor: repo-setup-manager_

<!-- separates adjacent blockquotes (markdownlint MD028) -->

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.
> This follows the root cause orientation principle — proactively fix preexisting errors encountered
> during work.

- [x] [AI] From the worktree root, run `npm install` — exits 0 and `node_modules/` is present.
<!-- Date: 2026-06-10 | Status: done | Files Changed: none | Notes: npm install ran in worktrees/standardize-secrets-and-env/, node_modules/.bin/nx present, npm audit warnings only (no failures) -->
- [x] [AI] Converge the polyglot toolchain: run `npm run doctor -- --fix` — exits 0 with no
    unresolved drift (Rust, Go, Node, JVM, Python, Elixir, Dart, etc. report present).
<!-- Date: 2026-06-10 | Status: done | Files Changed: none | Notes: 19/19 tools OK (volta, node, npm, java, maven, golang, python, rust, cargo-llvm-cov, elixir, erlang, dotnet, clojure, dart, flutter, docker, jq, playwright) -->
- [x] [AI] Capture the rhino-cli baseline (both implementations): run
    `./node_modules/.bin/nx run rhino-cli-rust:test:quick` and
    `./node_modules/.bin/nx run rhino-cli-go:test:quick` — both exit 0 (record coverage %).
<!-- Date: 2026-06-10 | Status: done | Files Changed: none | Notes: Rust 617 tests pass; Go line coverage 90.14% (≥90% threshold). Both exit 0. -->
- [x] [AI] Capture the parity baseline: run `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits
    0 (byte-identical) — record that parity is green before any change.
<!-- Date: 2026-06-10 | Status: done | Files Changed: apps/rhino-cli-rust/src/internal/mermaid/parser.rs, apps/rhino-cli-go/internal/mermaid/parser.go (worktree) | Notes: Preexisting mermaid parity bug fixed (HTML entity & in node label caused non-deterministic fallback ordering in Go). Both Rust and Go fallback now sort remaining node IDs alphabetically. Shadow-diff PASS: 267/267 cases byte-identical. -->
- [x] [AI] Record the naming baseline: run
    `grep -rn "APP_JWT_SECRET\|APP_PORT\|JWT_SECRET" apps/crud-be-* apps/crud-fs-ts-nextjs infra/dev`
    and save the hit list per app — Phases 1–2 eliminate exactly these app-defined reads.
<!-- Date: 2026-06-10 | Status: done | Files Changed: none (read-only baseline) | Notes: APP_JWT_SECRET in code: clojure-pedestal, csharp-aspnetcore, elixir-phoenix, fsharp-giraffe, golang-gin, java-springboot, java-vertx, python-fastapi, rust-axum, ts-effect, crud-fs-ts-nextjs. JWT_SECRET (bare) in code: kotlin-ktor, golang-gin-test. APP_PORT in code: java-vertx, rust-axum. PORT (bare) in code: clojure, csharp, elixir, golang-gin, kotlin-ktor, ts-effect. -->
- [x] [AI] Confirm the secret-backup gap: run `./node_modules/.bin/nx run rhino-cli-rust:build` then
    run the built binary `env backup` to a throwaway dir (place a throwaway `secrets.json`, a
    `throwaway.pem`, and a `.secrets/throwaway.md` first) and confirm **all three** are **absent**
    from the archive (proving the `.env`-prefix filter at `discover.rs:71` and the hidden-dir skip
    at `discover.rs:50`). Delete the throwaway files and dir after. Phase 3 makes them appear.
<!-- Date: 2026-06-10 | Status: done | Files Changed: none (throwaway files created and deleted) | Notes: env backup collected only 16 .env.example files. secrets.json, throwaway.pem, .secrets/throwaway.md all absent from backup output — gap confirmed. Phase 3 will widen discover.rs to include these secret kinds. -->

### Phase 0 Gate

> All checks below must pass before starting Phase 1; if any fails, fix it in Phase 0 first.

- [x] [AI] `rhino-cli-rust:test:quick` and `rhino-cli-go:test:quick` both exit 0 (clean baseline).
<!-- Date: 2026-06-10 | Status: done | Notes: Rust 617 tests pass (cache hit); Go 90.14% (cache hit). Preexisting mermaid parity fix committed and pushed. -->
- [x] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` exits 0 (parity green at baseline).
<!-- Date: 2026-06-10 | Status: done | Notes: Shadow-diff PASS 270/270 cases byte-identical. -->
- [x] [AI] Run `git status` — working tree clean (no changes yet).
<!-- Date: 2026-06-10 | Status: done | Notes: Worktree clean after committing mermaid parity fix and delivery.md P0 baseline checkboxes. Both commits pushed to origin main. -->

> **Pause Safety**: Phase 0 made no code changes; the repo is at a clean, green, parity-aligned
> baseline. Resume by re-running both `test:quick` targets and the shadow-diff harness.

---

## Phase 1 — Naming Standard + Per-App Rename (backends)

> One sub-task per backend family. For each app, rename **only** app-defined vars (`APP_JWT_SECRET` /
> `JWT_SECRET` / `APP_PORT` / `PORT` where our own code binds it) to the per-app prefix from
> [tech-docs.md §2](./tech-docs.md#2-naming-standard). Do **not** rename `DATABASE_URL`, `POSTGRES_*`,
> `NEXT_PUBLIC_*`, framework-owned `PORT`, or `ENABLE_TEST_API`.

For **each** of the 11 backends (`crud-be-rust-axum`, `crud-be-golang-gin`, `crud-be-clojure-pedestal`,
`crud-be-csharp-aspnetcore`, `crud-be-elixir-phoenix`, `crud-be-fsharp-giraffe`,
`crud-be-java-springboot`, `crud-be-java-vertx`, `crud-be-kotlin-ktor`, `crud-be-python-fastapi`,
`crud-be-ts-effect`):

- [x] [AI] Re-grep the app for its app-defined env reads:
      `grep -rniE "APP_JWT_SECRET|APP_PORT|JWT_SECRET|getenv|env::var|environ|System\.getenv|System/getenv|GetEnvironmentVariable|System.get_env|Config\.string" apps/<app>`
      — record every read site (config file + any auxiliary reader).
  - _Suggested executor: `swe-rust-dev` (Rust), `swe-golang-dev` (Go),
    `swe-typescript-dev` (TS), `swe-java-dev` (Java), `swe-kotlin-dev` (Kotlin),
    `swe-elixir-dev` (Elixir), `swe-python-dev` (Python), `swe-fsharp-dev` (F#),
    `swe-csharp-dev` (C#), `swe-clojure-dev` (Clojure)._
- [x] [AI] **RED**: add/adjust a unit test in the app's config test asserting the **prefixed** key
      resolves (e.g. setting `CRUD_BE_RUST_AXUM_PORT` resolves the port). Run that app's
      `./node_modules/.bin/nx run <app>:test:unit` — acceptance: it fails because the code still reads
      the old key.
- [x] [AI] **GREEN**: rename the read key(s) in the app's config code to the per-app-prefixed name
      (leave shared/framework vars untouched). Run `./node_modules/.bin/nx run <app>:test:unit` —
      acceptance: the prefixed-key test passes.
- [x] [AI] **REFACTOR**: verify the renamed config code is idiomatic for this language family (no dead
      default paths, clean struct/record layout). Run `./node_modules/.bin/nx run <app>:test:quick` —
      acceptance: all tests still pass, coverage ≥ baseline.
- [x] [AI] Edit `infra/dev/<app>/.env.example`: rename the same app-defined keys to the prefixed
      names; keep placeholders obviously-dev.
- [x] [AI] Edit `infra/dev/<app>/docker-compose.yml` and `docker-compose.ci.yml` (where present): in
      each `environment:` block rename the same keys.
- [x] [AI] Verify zero residue for this app:
      `grep -rn "APP_JWT_SECRET\|APP_PORT" apps/<app> infra/dev/<app>` returns zero app-defined hits
      (framework `PORT`, if any, remains only where the framework owns it).
- [x] [AI] Run `./node_modules/.bin/nx run <app>:test:quick` — exits 0, coverage ≥ baseline.

### Phase 1 Gate

> All checks below must pass before starting Phase 2; if any fails, fix it in Phase 1 first.

- [x] [AI] `grep -rn "APP_JWT_SECRET\|APP_PORT\|JWT_SECRET" apps/crud-be-* infra/dev` returns zero
      **app-defined** hits (only framework-owned `PORT` may remain, where the framework binds it).
      Note: `JWT_SECRET` hits are the Kotlin `crud-be-kotlin-ktor` bare key that Phase 1 renames; any
      remaining `JWT_SECRET` match outside test scaffolding is an outstanding rename target.
- [x] [AI] Every backend's `./node_modules/.bin/nx run <app>:test:quick` exits 0 with coverage ≥
      baseline.
- [x] [AI] `npm run lint:md` exits 0.
- [x] [AI] Commit thematically (one commit per backend family, or grouped sensibly,
      `refactor(<app>): rename env vars to per-app prefix`) and push to `origin main`; `git status`
      clean.

> **Pause Safety**: Phase 1 left every backend's config code, `.env.example`, and compose env blocks
> naming the same per-app-prefixed keys; tests green. Resume by re-running the affected backends'
> `test:quick`.

---

## Phase 2 — Naming Standard + Per-App Rename (frontends)

- [x] [AI] **RED**: in `apps/crud-fs-ts-nextjs/src/lib/jwt.ts`, add/adjust a unit test asserting
      `process.env.CRUD_FS_TS_NEXTJS_JWT_SECRET` resolves the JWT secret. Run
      `./node_modules/.bin/nx run crud-fs-ts-nextjs:test:unit` — acceptance: the test fails because the
      code still reads `process.env.APP_JWT_SECRET`. - _Suggested executor: `swe-typescript-dev`._
- [x] [AI] **GREEN**: rename the server-side read in `apps/crud-fs-ts-nextjs/src/lib/jwt.ts` from
      `process.env.APP_JWT_SECRET` to `process.env.CRUD_FS_TS_NEXTJS_JWT_SECRET`. Run
      `./node_modules/.bin/nx run crud-fs-ts-nextjs:test:unit` — acceptance: the prefixed-key test
      passes. - _Suggested executor: `swe-typescript-dev`._
- [x] [AI] Update `infra/dev/crud-fs-ts-nextjs/.env.example` (rename `APP_JWT_SECRET` →
      `CRUD_FS_TS_NEXTJS_JWT_SECRET`) and the matching key in its compose env block.
- [x] [AI] **REFACTOR**: tidy the jwt.ts module if needed (no dead branches, clean types). Run
      `./node_modules/.bin/nx run crud-fs-ts-nextjs:test:unit` — acceptance: all tests still pass.
- [x] [AI] `crud-fe-ts-nextjs`, `crud-fe-ts-tanstack-start`, `crud-fe-dart-flutterweb`: re-grep each
      for any **app-defined** env read (`grep -rniE "process\.env\.|import\.meta\.env\.|String\.fromEnvironment" apps/<app>/src`),
      excluding framework-reserved `NEXT_PUBLIC_*` and shared `DATABASE_URL`. Rename only app-defined
      reads to the per-app prefix; update the matching `infra/dev/<app>/.env.example`. If an app has
      no app-defined reads to rename, record that and skip.
      For `crud-fe-ts-nextjs` specifically: no server-side app-defined env reads (beyond
      `NEXT_PUBLIC_*` and shared vars) are expected; run the grep to confirm and record the observation
      in a checklist note. The Phase 2 gate grep confirms zero `APP_JWT_SECRET` residue across all
      frontends; if the per-app grep surfaces any additional old-form app-defined read, rename it too.
- [x] [AI] Run each touched frontend's `./node_modules/.bin/nx run <app>:typecheck` and
      `./node_modules/.bin/nx run <app>:test:quick` — both exit 0.

### Phase 2 Gate

> All checks below must pass before starting Phase 3; if any fails, fix it in Phase 2 first.

- [x] [AI] `grep -rn "APP_JWT_SECRET" apps/crud-fs-ts-nextjs apps/crud-fe-* infra/dev/crud-fe-* infra/dev/crud-fs-*`
      returns zero hits.
- [x] [AI] Every touched frontend's `typecheck` and `test:quick` exit 0.
- [x] [AI] `npm run lint:md` exits 0.
- [x] [AI] Commit (`refactor(frontends): rename app-defined env vars to per-app prefix`) and push;
      `git status` clean.

> **Pause Safety**: Phase 2 left all frontends naming app-defined env vars by per-app prefix; types
> and tests green. Resume by re-running the touched frontends' `typecheck`.

---

## Phase 3 — `env backup`/`restore`: every secret kind + `--dry-run` (spec → Rust → Go)

> Spec-first dual-implementation. Author scenarios, implement Rust-canonical, then Go-twin, then prove
> byte-identical with the shadow-diff harness.

- [x] [AI] **Spec**: extend `specs/apps/rhino/behavior/cli/gherkin/env/env-backup.feature` and
      `env-restore.feature` with scenarios for: (a) `secrets.json` is backed up; (b) a `*.pem` is
      backed up; (c) a `.secrets/` file is backed up; (d) `.git/` is still skipped; (e) `--dry-run`
      writes nothing and lists the would-back-up/would-restore set. Each scenario uses exactly one
      primary `Given`/`When`/`Then` (extras chained with `And`). Run
      `./node_modules/.bin/nx run rhino-cli-rust:validate:gherkin-keyword-cardinality` — exits 0. - _Suggested executor: `specs-maker`._
- [x] [AI] **RED (Rust canonical)**: add failing unit tests in
      `apps/rhino-cli-rust/src/internal/envbackup/discover.rs` (temp-dir fixtures) asserting:
      `.secrets/notes.md`, `secrets.json`, and `cert.pem` appear in the discovered set; `.git/` is
      still skipped; a `dry_run=true` backup creates no files. Run
      `./node_modules/.bin/nx run rhino-cli-rust:test:unit` — acceptance: all new tests fail. - _Suggested executor: `swe-rust-dev`._
- [x] [AI] **GREEN (Rust) — carve `.secrets/` out of the hidden-dir skip**: in
      `apps/rhino-cli-rust/src/internal/envbackup/discover.rs`, at the hidden-dir prune (`:50`,
      `base.starts_with('.')`), add an exception so a top-level `.secrets/` is descended into; all
      other dot-dirs still pruned. Run `./node_modules/.bin/nx run rhino-cli-rust:test:unit` —
      acceptance: the `.secrets/` test passes.
- [x] [AI] **GREEN (Rust) — widen the secret allowlist**: replace the `discover.rs:71` basename filter
      (`!base.starts_with(".env")`) with an allowlist matching `.env`/`.env.*`, `secrets.json`,
      `*.pem`/`*.key`/`*.crt`/`*.pfx`, and any file reached under `.secrets/`. Apply the same widened
      filter to `restore`'s non-config branch (`ops.rs:294` / `base_starts_with_env`). Ship the
      `*.tfvars`/inventory patterns **commented** with an "activate when IaC is added" note (R3/R11).
      Keep all existing skip-dir, max-size, and inside-repo-refusal checks intact. Run
      `./node_modules/.bin/nx run rhino-cli-rust:test:unit` — acceptance: the `secrets.json`/`*.pem`
      tests pass.
- [x] [AI] **GREEN (Rust) — add `--dry-run`**: add a `dry_run: bool` to the shared `Options`; add a
      `--dry-run` clap arg to the backup and restore commands in `apps/rhino-cli-rust/src/commands/env.rs`;
      when set, run discovery but perform **no** writes and report the would-act file list. Run
      `./node_modules/.bin/nx run rhino-cli-rust:test:unit` — acceptance: the dry-run no-write test
      passes; then `./node_modules/.bin/nx run rhino-cli-rust:test:quick` exits 0, coverage ≥ gate.
- [x] [AI] **RED (Rust) — backup default dir (R11b)**: add a failing unit test in
      `apps/rhino-cli-rust/src/internal/envbackup/types.rs` (or a test module adjacent to
      `commands/env.rs`) asserting that when `--dir` is empty the derived default equals
      `~/<repo-root-basename>-env-backup` for a fixture repo root (e.g. a temp dir named `ose-primer`
      produces `~/ose-primer-env-backup`, not `~/ose-open-env-backup`). Run
      `./node_modules/.bin/nx run rhino-cli-rust:test:unit` — acceptance: the new test fails because the
      code still returns the hardcoded `ose-open-env-backup` path.
- [x] [AI] **GREEN (Rust) — backup default dir (R11b)**: change `DEFAULT_BACKUP_DIR` handling in
      `apps/rhino-cli-rust/src/internal/envbackup/types.rs` + `commands/env.rs` so the `--dir`-empty
      fallback derives `~/<repo-root-basename>-env-backup` from the already-computed repo-root path
      (replacing the hardcoded `ose-open-env-backup`); update the help/example strings accordingly. Run
      `./node_modules/.bin/nx run rhino-cli-rust:test:unit` — acceptance: the derived-default test passes.
- [x] [AI] **GREEN (Go twin)**: mirror every change byte-identically in
      `apps/rhino-cli-go/internal/envbackup/discover.go` (the `:41` hidden-dir skip and `:52`
      allowlist), `restore.go` (the `:97` filter), the `--dry-run` flag in
      `apps/rhino-cli-go/cmd/env_backup.go` + `env_restore.go`, **and the per-repo-derived backup
      default dir + help strings (R11b)**. Port the unit tests. Run
      `./node_modules/.bin/nx run rhino-cli-go:test:unit` then `:test:quick` — both exit 0. - _Suggested executor: `swe-golang-dev`._
- [x] [AI] **REFACTOR**: review the widened discover/ops code in both implementations for duplication
      or dead code. Run `./node_modules/.bin/nx run rhino-cli-rust:test:unit` and
      `./node_modules/.bin/nx run rhino-cli-go:test:unit` — acceptance: all tests still pass.
- [x] [AI] **Shadow-diff**: run `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0
      (byte-identical stdout/stderr/exit codes across all output formats).
- [x] [AI] Smoke-check: place a throwaway `secrets.json`, `throwaway.pem`, and `.secrets/throwaway.md`,
      run the built `env backup --dry-run` — the would-back-up list now includes all three (Phase 0
      gaps closed) and creates nothing. Remove the throwaway files.

### Phase 3 Gate

> All checks below must pass before starting Phase 4; if any fails, fix it in Phase 3 first.

- [x] [AI] `rhino-cli-rust:test:quick`, `rhino-cli-go:test:quick`, and both `spec-coverage` exit 0.
- [x] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` exits 0 (parity preserved).
- [x] [AI] `env backup --dry-run` lists `secrets.json`, a `*.pem`, and `.secrets/` files and writes
      nothing; a backup→restore round-trip over a fixture reproduces all secret kinds byte-for-byte.
- [x] [AI] **Backup default dir (R11b) lands in BOTH twins**: `env backup`/`restore` with no `--dir`
      resolves to `~/ose-primer-env-backup` (per-repo-derived, not `~/ose-open-env-backup`) in both the
      Rust and Go binaries, and `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` confirms the new
      default path + help text are **byte-identical** across the twins (hard acceptance criterion).
- [x] [AI] `npm run lint:md` exits 0.
- [x] [AI] Commit (`feat(rhino-cli): back up and restore all secret kinds; add --dry-run; per-repo backup dir`)
      and push; `git status` clean.

> **Pause Safety**: Phase 3 left backup/restore covering every active secret kind, able to preview
> with no side effects, byte-identical across implementations. Resume by running the shadow-diff
> harness then `env backup --dry-run`.

---

## Phase 4 — Fail-Fast Startup Validation (per-language sub-phases)

> One sub-phase per language family. For each app: clear its validator dependency (HARD), then RED →
> GREEN the fail-fast behavior per [tech-docs.md §4](./tech-docs.md#4-startup-validation-per-language).
> Where the idiomatic path is framework-native with no new dependency, skip the clearance step for
> that family and do only the code RED→GREEN.

- [x] [AI] **Dependency clearance (HARD, all families)**: per
      [tech-docs.md §7](./tech-docs.md#7-dependency-additions--security-clearance-dependency-bump-policy),
      compute the cutoff (`today − 60 days`) in writing; for each named validator dep (`dotenvy`,
      `envy`, `caarlos0/env` v11, `@t3-oss/env-nextjs`, `zod`, Spring Validation, and the long-tail
      libs) select the most recent eligible (Path B) version, confirm not yanked / no open
      release-blocker, and CVE-clear each; record results in the `tech-docs.md §7` table. **Python
      family skip**: `pydantic-settings==2.13.1` is already pinned in
      `apps/crud-be-python-fastapi/pyproject.toml` — no dep-add step needed; proceed directly to the
      RED/GREEN sub-phase for the Python family. Select the exact long-tail lib (or framework-native
      no-dep approach) per remaining family here.

For **each** app in [tech-docs.md §4](./tech-docs.md#4-startup-validation-per-language):

- [x] [AI] Add the family's validator dependency as an **exact pin** to that app's manifest (skip if
      framework-native, no dep); build the app and run its language-appropriate audit — clean.
- [x] [AI] **RED**: write a failing test asserting the app's config loader returns/raises an error
      **naming the variable** when a required var is unset (for build-time-validated frontends, assert
      the build/typecheck fails naming the var). Run `./node_modules/.bin/nx run <app>:test:unit`
      (or `:test:quick` where unit is not split) — acceptance: the test fails (validator not yet
      wired). - _Suggested executor: the language-matching dev agent._
- [x] [AI] **GREEN**: rewrite the app's config loader to the fail-fast shape from
      [tech-docs.md §4](./tech-docs.md#4-startup-validation-per-language) (required vars have no
      default; missing → named error). For `crud-be-ts-effect`, **remove** the
      `Effect.catchAll(() => Effect.succeed(defaults))` wrapper in
      `apps/crud-be-ts-effect/src/config.ts`. Run `./node_modules/.bin/nx run <app>:test:quick` —
      acceptance: the RED test passes and the loader resolves correctly when all required vars are set.
- [x] [AI] **REFACTOR**: tidy the loader (extract a single config struct/record, remove dead default
      paths). Run `./node_modules/.bin/nx run <app>:test:quick` — acceptance: all tests still pass,
      coverage ≥ gate.

After all apps:

- [x] [AI] Prove the Effect swallow is gone:
      `grep -n "catchAll" apps/crud-be-ts-effect/src/config.ts` shows no default-swallowing handler.
- [x] [AI] Prove one Next.js build-time validation: in `crud-fe-ts-nextjs`, temporarily unset its
      required `NEXT_PUBLIC_*` var and run `./node_modules/.bin/nx run crud-fe-ts-nextjs:build` —
      acceptance: build fails naming the var; restore and re-run — acceptance: build exits 0.
- [x] [AI] **Manual API verification (curl)** for one representative backend (e.g. `crud-be-rust-axum`):
      start it with all required prefixed vars set
      (`CRUD_BE_RUST_AXUM_PORT=8299 CRUD_BE_RUST_AXUM_JWT_SECRET=dev-secret DATABASE_URL=<local-dev-url>`)
      and run `curl -sf http://localhost:8299/health` — acceptance: HTTP 200 with a JSON body; then
      unset `CRUD_BE_RUST_AXUM_JWT_SECRET` and confirm startup aborts naming the variable.

### Phase 4 Gate

> All checks below must pass before starting Phase 5; if any fails, fix it in Phase 4 first.

- [x] [AI] `tech-docs.md §7` clearance table filled (exact versions, Path B, CVE status); no
      caret/tilde/range in any touched manifest; all language audits clean.
- [x] [AI] Every app's `./node_modules/.bin/nx run <app>:test:quick` exits 0 with coverage ≥ gate; the
      missing-required-var test asserts a named error for each.
- [x] [AI] `grep -n "catchAll" apps/crud-be-ts-effect/src/config.ts` shows no default swallow; one
      Next.js build fails-then-passes as expected.
- [x] [AI] `npm run lint:md` exits 0.
- [x] [AI] Commit thematically per app/family (`feat(<app>): fail-fast env validation`) and push;
      `git status` clean.

> **Pause Safety**: Phase 4 left every app validating env at startup/build with all gates green and
> deps cleared. Resume by re-running the affected apps' `test:quick`.

---

## Phase 5 — `.env.example` Annotation Format

- [x] [AI] For each `infra/dev/<app>/.env.example` (15 files): above each variable add a comment block
      stating required-or-optional, type, and format per the hub doc's annotation standard. Example for
      a JWT secret: `# Required. String, min 32 chars. Generate with: openssl rand -hex 32`. Mark
      shared `DATABASE_URL` / `POSTGRES_*` and framework `NEXT_PUBLIC_*` explicitly.
- [x] [AI] Annotate the root `.env.example` (`OPENCODE_GO_API_KEY`) to the same standard (it already
      carries prose comments — fold them into the standard's shape).
- [x] [AI] Verify placeholders are obviously-dev (no real-looking secret): run
      `grep -rnE "secret|token|key|pass" infra/dev/*/.env.example .env.example` and confirm every value
      is a placeholder, not a credential.
- [x] [AI] Run `npm run lint:md` and `npm run format:md:check` — exit 0.

### Phase 5 Gate

> All checks below must pass before starting Phase 6; if any fails, fix it in Phase 5 first.

- [x] [AI] Every variable in every annotated `.env.example` has a required/optional + type + format
      comment.
- [x] [AI] `npm run lint:md` exits 0.
- [x] [AI] Commit (`docs(env): annotate env example files with type and required status`) and push;
      `git status` clean.

> **Pause Safety**: Phase 5 left the env templates self-documenting; no code touched. Resume by
> re-reading the annotated files (no command needed).

---

## Phase 6 — `env validate` Drift Guard (spec → Rust → Go) + pre-push + CI

> Spec-first dual-implementation again. App surface is the only **active** surface in primer;
> Terraform/Ansible validators ship documented-but-gated (R3).

- [x] [AI] **Spec**: create `specs/apps/rhino/behavior/cli/gherkin/env/env-validate.feature` with
      scenarios: (a) a seeded declared-but-unread key in a fixture app → non-zero exit naming the key;
      (b) a read-but-undeclared key → non-zero exit naming the key; (c) a matching app → exit 0; (d)
      allowlisted keys (`ENABLE_TEST_API`, framework `PORT`) are ignored. Each scenario uses one
      primary `Given`/`When`/`Then`. Run
      `./node_modules/.bin/nx run rhino-cli-rust:validate:gherkin-keyword-cardinality` — exits 0. - _Suggested executor: `specs-maker`._
- [x] [AI] Inspect rhino-cli's existing subcommand + config layout (`apps/rhino-cli-rust/src/`) to
      match the established clap-subcommand pattern. Decide the contract surface (`env-contract.yaml`
      via an already-present parser, or an existing config block — **no new crate**) and record the
      choice as a `// ENV-VALIDATE CONFIG: <choice>` comment at the top of the new
      `apps/rhino-cli-rust/src/commands/env_validate.rs`. The contract lists **surfaces**, each with a
      root, kind (`app` only in primer), globs, and an allowlist.
- [x] [AI] **RED (Rust canonical)**: write failing unit tests in `apps/rhino-cli-rust/src/` (in-memory
      fixtures) for the app validator: a fixture app with a seeded declared-but-unread key → non-zero
      naming the key; a read-but-undeclared key → non-zero naming the key. Run
      `./node_modules/.bin/nx run rhino-cli-rust:test:unit` — acceptance: the new tests fail. - _Suggested executor: `swe-rust-dev`._
- [x] [AI] **GREEN (Rust) — app validator**: parse `infra/dev/<app>/.env.example` declared keys; scan
      each language's literal env-read form (see
      [tech-docs.md §6.1](./tech-docs.md#61-app-validator-the-only-active-surface-in-primer)) for read
      keys; compute declared-but-unread and read-but-undeclared; honor the allowlist; exit non-zero
      naming keys on any non-empty set. Ship Terraform/Ansible validators **stubbed-but-gated** (no
      surface configured for primer). Run `./node_modules/.bin/nx run rhino-cli-rust:test:unit` —
      acceptance: the RED tests pass.
- [x] [AI] Write Rust integration tests (`cargo test --tests`) with temp-dir fixtures: a seeded
      mismatch (non-zero + key named) and a matching app (exit 0). Run
      `./node_modules/.bin/nx run rhino-cli-rust:test:quick` — exits 0, coverage ≥ gate.
- [x] [AI] **GREEN (Go twin)**: implement the identical `env validate` app validator in
      `apps/rhino-cli-go/cmd/env_validate.go` + internal package, byte-identical behavior; port the
      unit + integration tests. Run `./node_modules/.bin/nx run rhino-cli-go:test:quick` — exits 0. - _Suggested executor: `swe-golang-dev`._
- [x] [AI] **REFACTOR**: review the `env validate` code in both implementations (extract helpers,
      reduce duplication). Run `./node_modules/.bin/nx run rhino-cli-rust:test:unit` and
      `./node_modules/.bin/nx run rhino-cli-go:test:unit` — acceptance: all tests pass.
- [x] [AI] **Shadow-diff**: run `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0
      (byte-identical).
- [x] [AI] Run the built `rhino-cli env validate` against the real repo — exits 0 on the app surface
      (Phases 1–2 + 5 aligned every app's reads and declarations; allowlist framework-injected `PORT`
      and `ENABLE_TEST_API`).
- [x] [AI] Add `rhino-cli env validate` to `.husky/pre-push` (after the existing
      `npx nx affected … && npm run lint:md` sequence). Verify by running the pre-push script body
      locally — it invokes the command and passes.
- [x] [AI] Add a CI invocation: add a step running `rhino-cli env validate` to the appropriate
      existing `.github/workflows/` workflow (matched to the repo's layout). Validate the YAML per the
      repo's workflow conventions.
- [x] [AI] Prove the guard bites: temporarily rename a key in one app's
      `infra/dev/<app>/.env.example` → `rhino-cli env validate` exits non-zero naming the key; revert.

### Phase 6 Gate

> All checks below must pass before starting Phase 7; if any fails, fix it in Phase 6 first.

- [x] [AI] `rhino-cli-rust:test:quick`, `rhino-cli-go:test:quick`, and both `spec-coverage` exit 0.
- [x] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` exits 0 (parity preserved).
- [x] [AI] `rhino-cli env validate` exits 0 on the clean repo and non-zero on a seeded app mismatch.
- [x] [AI] `.husky/pre-push` and a `.github/workflows/` workflow both invoke the command.
- [x] [AI] `npm run lint:md` exits 0.
- [x] [AI] Commit (`feat(rhino-cli): add env validate drift guard (app surface; IaC gated)`) and push;
      `git status` clean.

> **Pause Safety**: Phase 6 left a working app-surface drift guard enforced by pre-push and CI,
> byte-identical across implementations, with IaC validators gated. Resume by running the shadow-diff
> harness then `rhino-cli env validate`.

---

## Phase 7 — Hub Convention Doc + Stub Redirects + Rationale Doc + Link Repointing

- [x] [AI] **Doc canonicalization (R10b)**: `git mv` the two security docs from
      `repo-governance/development/quality/` → `repo-governance/conventions/security/`
      (`no-secrets-in-committed-files.md`, `env-file-access.md`), creating the `conventions/security/`
      directory if absent. This aligns primer's governance paths with the `ose-infra` canonical layout.
      Record that primer's `repository-ecosystem` convention update (it currently pins these under
      `development/quality/`) is an **authorized downstream follow-up**, not part of this plan.
- [x] [AI] Create `repo-governance/conventions/security/secrets-and-env-standards.md` — the hub
      convention: principles, naming standard (per-app prefix + framework-exemption table including the
      12-factor "authorizes-not-prescribes" framing and the exempt class `NEXT_PUBLIC_*` / `PORT` /
      `NODE_ENV` / `DATABASE_URL`), the
      `.env.example` annotation format, the per-language fail-fast validation expectations (the
      validator-per-language table), the `rhino-cli env` family (backup/restore/init/validate including
      the repo-wide secret-backup scope and the `--dry-run` preview), the **secret-surface census**
      table, the **gated IaC scaffold** (commented `*.tfvars`/inventory patterns + the inert
      Terraform/Ansible validators with their "activate when IaC is added" trigger), and the
      storage-tier ladder + Tier-1 trigger. Fold the substantive content of the three existing docs in. - _Suggested executor: `repo-rules-maker`._
- [x] [AI] In the hub doc, add the canonical **secret-surface census** table: one row per active
      secret kind (`.env*`, `secrets.json`, `*.pem`/`*.key`/`*.crt`/`*.pfx`, `.secrets/`), each with
      its path/pattern, whether it is **backed up** (all) and **validated** (app surface only); plus a
      clearly-marked **gated** section for `*.tfvars` / inventories. Bless `.secrets/` as the catch-all
      home for homeless secrets (always backed up, never validated). Cross-link `.gitignore` lines 104
      and 105/108–111.
- [x] [AI] Reduce `repo-governance/conventions/security/no-secrets-in-committed-files.md` (at its new
      moved path) to a stub: keep its title + a one-paragraph summary (so the rule stays greppable) and
      link to the hub doc.
- [x] [AI] Reduce `repo-governance/conventions/security/env-file-access.md` (at its new moved path) to
      a stub redirecting to the hub doc (preserve the agent-permission rule summary).
- [x] [AI] Reduce `repo-governance/development/workflow/reproducible-environments.md` to a stub
      redirecting to the hub doc (preserve the `.env.example` pattern summary).
- [x] [AI] Create `docs/explanation/standardize-secrets-and-env-parity-decisions.md` — a plain-language
      explanation of **every** decision in the deviation matrix, matching the existing
      `*-parity-decisions.md` precedents. Explain especially: the **PR-override** (R5/R2 — why primer
      pushes directly to `main` this once and who owns it, and that the sync-governance change is a
      separate downstream follow-up), the **IaC = N/A** decision (R3 — primer holds no infra and never
      receives infra artifacts, so the Terraform/Ansible drift-guard is dropped, unlike ose-infra's
      real validators and ose-public's commented forward-scaffold), the **doc canonicalization** (R10b
      — moving the two security docs to `conventions/security/`, with the ecosystem-convention update
      deferred downstream), the **backup default dir** (R11b — per-repo-derived `~/<repo-basename>-env-backup`
      in both twins), the **full polyglot adoption** (R7/R8), the **no-migration layout** (R12), and the
      **canonical/twin correction** (R6 — Rust canonical, Go twin). - _Suggested executor: `docs-maker`._
- [x] [AI] Repoint **active** inbound links to the hub doc (a **link-check gate** — `npm run lint:md`
      must report zero broken links after the move + fold): update the root governance indexes
      (`repo-governance/development/quality/README.md`, `repo-governance/conventions/security/README.md`
      if created, `repo-governance/development/workflow/README.md`, `repo-governance/conventions/README.md`
      if it references these), `CLAUDE.md`/`AGENTS.md`, `docs/` references, and any
      `.claude/`/`.opencode/` agent/skill references found by the inbound-link sweep — rewriting both
      the changed **paths** (`development/quality/` → `conventions/security/`) and the changed targets.
      Leave `plans/done/**` links pointing at the stubs (historical, must not be rewritten).
- [x] [AI] If any `.claude/` agent/skill text changed, run the repo's binding-sync (e.g.
      `npm run generate:bindings` or `rhino-cli agents sync`) to resync `.opencode/`.
- [x] [AI] Run `npm run lint:md` — exits 0 (no broken links from the fold).
- [x] [AI] Inbound-link verification:
      `grep -rl "no-secrets-in-committed-files\|env-file-access\|reproducible-environments" --include="*.md" . | grep -v node_modules | grep -v plans/done`
      — every remaining active hit is either a stub file itself or now also links the hub doc.

### Phase 7 Gate

> All checks below must pass before starting Phase 8; if any fails, fix it in Phase 7 first.

- [x] [AI] `repo-governance/conventions/security/secrets-and-env-standards.md` exists; the two
      security docs have moved to `conventions/security/` (R10b) and the three prior docs are stubs
      linking to it.
- [x] [AI] `docs/explanation/standardize-secrets-and-env-parity-decisions.md` exists and covers all 16
      decisions, including R5/R2 (PR-override), R3 (IaC = N/A), R10b (doc canonicalization), R11b
      (per-repo backup dir), and R6 (canonical/twin correction).
- [x] [AI] `npm run lint:md` exits 0 (link check passes; no `done/` link broken).
- [x] [AI] If `.claude/` changed, `.opencode/` is in sync (`git status` shows matching regenerated
      files).
- [x] [AI] Commit (`docs(governance): consolidate secrets/env rules into one hub convention`) and push;
      `git status` clean.

> **Pause Safety**: Phase 7 left one authoritative hub doc with the three prior docs redirecting, the
> rationale doc published, and all links intact. Resume by re-running `npm run lint:md`.

---

## Phase 8 — Final Quality Gate + Commit + Push

- [x] [AI] Run the full affected gate:
      `npx nx affected -t typecheck lint test:quick spec-coverage` across `main` — all exit 0.
- [x] [AI] Run `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` — exits 0 (parity green).
- [x] [AI] Run the built `rhino-cli env validate` — exits 0 on the app surface.
- [x] [AI] Run `npm run lint:md` and `npm run format:md:check` — exit 0.
- [x] [AI] Re-verify every BRD success criterion: per-app naming applied (grep-to-zero per app);
      fail-fast validation active in every app; Effect swallow removed; backup covers every active
      secret kind with working `--dry-run`; backup default dir per-repo-derived in both twins
      (both resolve `~/ose-primer-env-backup`, shadow-diff green); dual-impl parity preserved;
      guard wired into pre-push + CI;
      hub doc exists with the three stubs; IaC scaffold present but gated; PR-override recorded in
      `tech-docs.md §1` + the rationale doc; deps exact-pinned + cleared.
- [x] [AI] Confirm all per-phase commits landed on `origin main`:
      `git log --oneline origin/main -20` shows the Phase 1–7 commits; `git status` clean, nothing
      unpushed.
- [x] [AI] Confirm the Phase 4 manual curl assertion is carried forward as the terminal record: the
      Phase 4 step started `crud-be-rust-axum` with all required prefixed vars and confirmed
      `curl -sf http://localhost:8299/health` returns HTTP 200, and confirmed non-zero exit when
      `CRUD_BE_RUST_AXUM_JWT_SECRET` is unset. No app startup code changes after Phase 4, so no
      re-run is required; tick this item once you have verified that Phase 4's curl step is ticked.

### Phase 8 Gate

> All checks below must pass before archiving this plan; if any fails, fix it in Phase 8 first.

- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` exits 0.
- [x] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh` exits 0; `rhino-cli env validate` exits 0;
      `npm run lint:md` exits 0.
- [x] [AI] Every BRD success criterion verified true.
- [x] [AI] Working tree clean; all phase commits pushed to `origin main`.

> **Pause Safety**: Phase 8 is terminal — the standard is live and self-enforcing across the polyglot
> template, byte-identical across both rhino-cli implementations. The plan is ready for archival.

---

## Plan Archival

- [x] [AI] Verify ALL delivery checklist items are ticked.
- [x] [AI] Verify ALL quality gates pass (local affected gate + shadow-diff + `env validate` + CI).
- [x] [AI] Verify ALL manual assertions pass (curl health check; one Next.js build fail-then-pass).
- [x] [AI] Rename and move:
      `git mv plans/in-progress/standardize-secrets-and-env/ plans/done/2026-MM-DD__standardize-secrets-and-env/`
      using today's date as the **completion** date (not the creation date).
- [x] [AI] Update `plans/in-progress/README.md` — remove the plan entry.
- [x] [AI] Update `plans/done/README.md` — add the plan entry with the completion date.
- [x] [AI] Update any other READMEs that reference this plan (e.g. `plans/README.md`).
- [x] [AI] Commit the archival: `chore(plans): move standardize-secrets-and-env to done` and push.
