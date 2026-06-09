# Tech Docs — Standardize Secrets and Environment-Variable Storage (ose-primer)

This document holds the resolved deviation matrix, the design decisions and their rationale, the
file-impact analysis, and the mechanics. The step-by-step checklist lives in
[delivery.md](./delivery.md).

## 1. Resolved Deviation Matrix (all 14 decisions)

This is the full cross-repo deviation matrix from the parity effort, reproduced verbatim with
`ose-primer`'s column and justification. The matrix is the source of truth; nothing in this plan
silently deviates from it. **Deviation count: 14 recorded decisions, 0 silent deviations.**

| #   | Dimension                 | Decision (primer)                                                                                                                                                                                                                                                       | Justification                                                                                                          |
| --- | ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| R1  | Parity set                | primer is authored alongside public; infra is the reference (already gated/passing)                                                                                                                                                                                     | infra plan already gated; avoid regression                                                                             |
| R2  | Delivery mode             | **`worktree-to-main`** (push directly to `ose-primer` `main`)                                                                                                                                                                                                           | invoker choice                                                                                                         |
| R3  | IaC surfaces              | **Forward-looking scaffold** — IaC validators + `*.tfvars`/inventory backup patterns shipped **commented/gated** "when IaC is added"; primer has no IaC today                                                                                                           | keep design ready without standing up inert tooling                                                                    |
| R4  | Research                  | Ran — Go/Effect/long-tail validator findings in [§9](./tech-docs.md#9-research-findings-cited)                                                                                                                                                                          | tooling needed verification                                                                                            |
| R5  | **PR override**           | **DEVIATION ACCEPTED**: primer is delivered `worktree-to-main` despite the sibling-sync rule that `ose-primer` receives governance changes via a **draft PR**. Explicit, one-off, **invoker-owned**. Recorded here and in the Phase 7 rationale doc. No `--force` ever. | invoker explicitly accepted; one-off, owned by invoker                                                                 |
| R6  | rhino-cli tooling         | **Spec-first dual-impl**: update `specs/apps/rhino/behavior/cli/gherkin/env/` → implement **Rust (canonical)** → implement **Go (twin)** → shadow-diff byte-identical. `env validate` authored Rust-canonical.                                                          | preserve primer's mandated dual-CLI parity model (see correction below)                                                |
| R7  | Startup validation        | **Full adoption in every app** (11 backends + 4 frontends)                                                                                                                                                                                                              | maximal convergence (invoker)                                                                                          |
| R8  | primer polyglot reach     | **All 11 backends** — validator-per-language table ([§4](./tech-docs.md#4-startup-validation-per-language)) + one delivery sub-phase per family                                                                                                                         | honors full-adoption literally                                                                                         |
| R9  | Naming prefix             | **Full per-app prefix rename across all existing app-defined vars**                                                                                                                                                                                                     | maximal convergence (invoker)                                                                                          |
| R10 | Hub doc                   | New `repo-governance/development/quality/secrets-and-env-standards.md`; fold the three existing docs to stub redirects                                                                                                                                                  | cross-repo structural symmetry (primer's security docs live under `development/quality/`, not `conventions/security/`) |
| R11 | Backup allowlist          | **Real floor + IaC gated scaffold**: `.env*` + `.secrets/` + `secrets.json` + `*.pem`/`*.key`/`*.crt`/`*.pfx`; `*.tfvars`/inventory patterns shipped **commented** forward-scaffold. Hybrid floor ∪ optional registry `backup_globs`.                                   | per-repo honesty + ready for IaC                                                                                       |
| R12 | Layout                    | **primer keeps its `infra/dev/<app>/` layout and root `.env.example`** (no migration to `apps/<app>/`); `env init` keeps walking `infra/dev/`. Real gitignored-file relocations (none expected) would be **[HUMAN]** (env-file-access guard).                           | primer is a template repo; preserve its layout + Nx scaffold path                                                      |
| R13 | Rationale doc             | `docs/explanation/standardize-secrets-and-env-parity-decisions.md`                                                                                                                                                                                                      | aligns with existing `*-parity-decisions.md` precedents                                                                |
| R14 | Drift `APP_PORT` live-fix | **DROP** — no such live drift exists in primer (the `APP_*` reads are renamed for naming, not to fix a live mismatch)                                                                                                                                                   | no such drift exists here                                                                                              |

### Correction to the matrix note (R6 canonical/twin direction)

The cross-repo matrix note recorded primer's rhino-cli as "Go canonical + Rust twin". The current
`ose-primer` governance states the **opposite**, and this plan follows the **repo's actual
convention**: per
[rhino-cli Dual-Implementation Parity Convention](../../../repo-governance/conventions/structure/rhino-cli-dual-implementation-parity.md)
[Repo-grounded], **`apps/rhino-cli-rust/` is the canonical CLI** (all `package.json` scripts and Husky
hooks invoke it) and **`apps/rhino-cli-go/` is the parity twin**. The spec-first flow therefore
implements **Rust first, then Go**, shadow-diff-gated. This is a correction to the matrix's column
label, not a new deviation — the spec-first, both-land-together, shadow-diff-gated **model** is exactly
what the matrix intended.

## 2. Naming Standard

### Decision

| Variable class             | Rule                                        | Example                                                   |
| -------------------------- | ------------------------------------------- | --------------------------------------------------------- |
| App-defined value          | `SCREAMING_SNAKE`, per-app prefix           | `CRUD_BE_RUST_AXUM_JWT_SECRET`, `CRUD_BE_GOLANG_GIN_PORT` |
| Framework-reserved value   | Keep the framework's required name          | `NEXT_PUBLIC_*`, framework `PORT`                         |
| Shared service connection  | Unprefixed, conventional name               | `DATABASE_URL`, `POSTGRES_USER`, `POSTGRES_PASSWORD`      |
| Environment tier in a name | **Forbidden** (keys identical across tiers) | not `PROD_DATABASE_URL`                                   |

The per-app prefix is the app's Nx project name upcased with `_` separators
(`crud-be-rust-axum` → `CRUD_BE_RUST_AXUM_`) [Repo-grounded — project names verified via `project.json`].
It prevents collisions when one process or compose stack loads multiple apps' vars and makes a
variable's owner obvious at a glance.

### Per-app prefix map (all renamed surfaces)

| App (Nx project)            | Prefix                       | Renamed app-defined vars (current → new)                         |
| --------------------------- | ---------------------------- | ---------------------------------------------------------------- |
| `crud-be-clojure-pedestal`  | `CRUD_BE_CLOJURE_PEDESTAL_`  | `APP_JWT_SECRET` → `…_JWT_SECRET`; `PORT` → `…_PORT`             |
| `crud-be-csharp-aspnetcore` | `CRUD_BE_CSHARP_ASPNETCORE_` | `APP_JWT_SECRET` → `…_JWT_SECRET`; `PORT` → `…_PORT`             |
| `crud-be-elixir-phoenix`    | `CRUD_BE_ELIXIR_PHOENIX_`    | `APP_JWT_SECRET` → `…_JWT_SECRET`; `PORT` → `…_PORT`             |
| `crud-be-fsharp-giraffe`    | `CRUD_BE_FSHARP_GIRAFFE_`    | `APP_JWT_SECRET` → `…_JWT_SECRET`; `PORT` → `…_PORT`             |
| `crud-be-golang-gin`        | `CRUD_BE_GOLANG_GIN_`        | `APP_JWT_SECRET` → `…_JWT_SECRET`; `PORT` → `…_PORT`             |
| `crud-be-java-springboot`   | `CRUD_BE_JAVA_SPRINGBOOT_`   | `APP_JWT_SECRET` → `…_JWT_SECRET`; `PORT` → `…_PORT`             |
| `crud-be-java-vertx`        | `CRUD_BE_JAVA_VERTX_`        | `APP_JWT_SECRET` → `…_JWT_SECRET`; `APP_PORT` → `…_PORT`         |
| `crud-be-kotlin-ktor`       | `CRUD_BE_KOTLIN_KTOR_`       | `JWT_SECRET` → `…_JWT_SECRET`; `PORT` → `…_PORT`                 |
| `crud-be-python-fastapi`    | `CRUD_BE_PYTHON_FASTAPI_`    | `APP_JWT_SECRET` (`app_jwt_secret`) → `…_JWT_SECRET`             |
| `crud-be-rust-axum`         | `CRUD_BE_RUST_AXUM_`         | `APP_JWT_SECRET` → `…_JWT_SECRET`; `APP_PORT` → `…_PORT`         |
| `crud-be-ts-effect`         | `CRUD_BE_TS_EFFECT_`         | `APP_JWT_SECRET` → `…_JWT_SECRET`; `PORT` → `…_PORT`             |
| `crud-fs-ts-nextjs`         | `CRUD_FS_TS_NEXTJS_`         | server-side `APP_JWT_SECRET` → `…_JWT_SECRET` (`src/lib/jwt.ts`) |

> Exact per-app read sites are confirmed at authoring time via `grep` (see
> [README §Context](./README.md#context)); the executor re-greps each app before renaming so no read
> site is missed. `DATABASE_URL`, `POSTGRES_USER`, `POSTGRES_PASSWORD`, and framework-reserved names
> are **not** renamed. `ENABLE_TEST_API` (a test-only toggle) stays as-is and is allowlisted in the
> drift guard. [Repo-grounded]

### Why framework-reserved names are exempt

`NEXT_PUBLIC_*` is the Next.js mechanism that decides which vars are bundled into browser JS — it is
not ours to rename. A framework that reads `PORT` natively (Next.js dev server, several backend
frameworks) keeps `PORT` where the framework, not our code, owns the bind. Where **our own code**
reads the port (e.g. `crud-be-rust-axum` reading `APP_PORT`, `crud-be-java-vertx` reading `APP_PORT`),
the value **is** app-defined and **does** take the prefix. This asymmetry is documented explicitly in
the hub doc.

### Why `DATABASE_URL` / `POSTGRES_*` stay unprefixed

`DATABASE_URL` is the de-facto conventional name understood by Postgres tooling, ORMs, and migration
runners; `POSTGRES_USER` / `POSTGRES_PASSWORD` are the names the official Postgres image reads. The
cost of renaming them (every tool that reads them by convention, plus the compose Postgres service)
exceeds the marginal collision-safety benefit. They are documented as explicitly-blessed unprefixed
shared names.

> Source: [The Twelve-Factor App — Config](https://12factor.net/config) (accessed 2026-06-09): store
> config in environment variables; keys identical across deploys, only values differ — hence no
> environment tier in the name.

## 3. Layout Decision — no migration

`ose-primer` is a **template repository**. It deliberately keeps its existing layout: a root
`.env.example` template plus per-app `infra/dev/<app>/.env.example` files, and `rhino-cli env init`
keeps walking `infra/dev/` for those templates [Repo-grounded — `apps/rhino-cli-rust/src/commands/env.rs`
joins `repo_root/infra/dev` and walks it]. **This plan does not migrate env templates to
`apps/<app>/`** and does not touch `env init`'s scaffold path. The only edits to the `infra/dev/<app>/`
files are the naming rename (§2) and the annotation pass (§5). This is decision **R12** and is the
principal layout divergence from the `ose-infra` reference (which does migrate).

Because there is no migration, there is also **no backup-first migration ritual** in this plan (the
reference's Phase 3). The widened backup tooling (§5/§6) is still delivered — it is independently
valuable — but it is not gating a layout move.

## 4. Startup Validation (per language)

Every app replaces its soft-default config read with a **language-idiomatic fail-fast validator** that
errors (naming the variable) when a required value is missing. Full adoption across all families
(decision R7/R8).

| App / family                                       | Fail-fast validator                                                          | Mode                                                         |
| -------------------------------------------------- | ---------------------------------------------------------------------------- | ------------------------------------------------------------ |
| `crud-be-rust-axum` (Rust)                         | `dotenvy` + `envy` (serde-derived struct, non-`Option`)                      | runtime fail-fast                                            |
| `crud-be-golang-gin` (Go)                          | `caarlos0/env` v11, tag `env:"KEY,required"`                                 | runtime fail-fast                                            |
| `crud-fe-ts-nextjs`, `crud-fs-ts-nextjs` (Next.js) | `@t3-oss/env-nextjs` + `zod` in `src/env.ts`                                 | **build-time** + `NEXT_PUBLIC_` enforcement                  |
| `crud-be-ts-effect` (TS/Effect, non-Next)          | Effect `Config` (remove the `catchAll`-to-defaults swallow)                  | runtime fail-fast                                            |
| `crud-fe-ts-tanstack-start` (TS, non-Next)         | `zod` schema parse of `import.meta.env` / `process.env`                      | runtime/build fail-fast (select exact boundary at execution) |
| `crud-be-java-springboot` (Java/Spring)            | Spring `@ConfigurationProperties` + `@Validated` (Jakarta Validation)        | startup fail-fast                                            |
| `crud-be-java-vertx` (Java/Vert.x)                 | explicit required-env check raising on absence (no Spring context)           | startup fail-fast                                            |
| `crud-be-kotlin-ktor` (Kotlin)                     | Ktor config + explicit `requireNotNull`/Konform-style check                  | startup fail-fast                                            |
| `crud-be-elixir-phoenix` (Elixir)                  | `runtime.exs` `System.fetch_env!/1` (already `raise`s for the secret)        | startup fail-fast                                            |
| `crud-be-python-fastapi` (Python)                  | `pydantic-settings` `BaseSettings` (required field, no default)              | startup fail-fast                                            |
| `crud-be-fsharp-giraffe` (F#)                      | explicit required-env read raising on absence                                | startup fail-fast                                            |
| `crud-be-csharp-aspnetcore` (C#)                   | `IOptions` + `ValidateOnStart()` (or explicit guard)                         | startup fail-fast                                            |
| `crud-be-clojure-pedestal` (Clojure)               | explicit required-env read throwing on absence (e.g. `aero` `:env` + assert) | startup fail-fast                                            |
| `crud-fe-dart-flutterweb` (Dart/Flutter web)       | `String.fromEnvironment` compile-time const + assert (build-define)          | build-time fail-fast                                         |

> **Long-tail libraries — exact pin selected at execution.** For the families marked above without a
> single named external library (Vert.x, Ktor, F#, C#, Clojure, TanStack-Start, Dart), the executor
> selects the exact idiomatic library/approach and pins it under the
> [Dependency Bump Policy](../../../repo-governance/development/workflow/dependency-bump-policy.md)
> Path B at execution (§7). Where the idiomatic approach is "framework-native + an explicit guard" (no
> new dependency), no dependency is added — only code changes. Rust/Go/Next.js/Effect/Python validators
> are named concretely because they are research-confirmed (§9).

### Rust (`crud-be-rust-axum`) — `dotenvy` + `envy`

Replace the `unwrap_or_else`-default loader in `apps/crud-be-rust-axum/src/config.rs` [Repo-grounded]
with a serde-derived struct deserialized by `envy` after `dotenvy::dotenv().ok()`. Required fields are
non-`Option`, no `#[serde(default)]` → a missing value is a hard error naming the field. The struct
field name `crud_be_rust_axum_jwt_secret` maps to env var `CRUD_BE_RUST_AXUM_JWT_SECRET`.

### Go (`crud-be-golang-gin`) — `caarlos0/env` v11

Replace the `os.Getenv` reads in `apps/crud-be-golang-gin/internal/config/config.go` [Repo-grounded]
with a struct whose fields carry `env:"CRUD_BE_GOLANG_GIN_JWT_SECRET,required"` tags; `env.Parse`
returns a hard error naming a missing required var.

### Next.js frontends — `@t3-oss/env-nextjs` + `zod`

Add `src/env.ts` exporting a validated `env` object; import it in `next.config.ts` so validation runs
at build time. `t3-env` enforces the `NEXT_PUBLIC_` prefix on client vars at TypeScript compile time,
encoding the naming standard into the type system.

### Effect backend (`crud-be-ts-effect`) — fail-fast Effect `Config`

`apps/crud-be-ts-effect/src/config.ts` currently wraps the whole read in
`Effect.catchAll(() => Effect.succeed({ … defaults … }))` [Repo-grounded], which swallows a missing
required value into a silent default. Remove that handler so a missing required var surfaces as a
`ConfigError`. Effect `Config` is **runtime** fail-fast and does **not** enforce `NEXT_PUBLIC_` — which
is why the Next.js apps still use `t3-env` (§9).

## 5. `rhino-cli env backup`/`restore` — every secret kind + `--dry-run` (spec-first dual-impl)

The repo ships `rhino-cli env backup`, `env restore`, and `env init` in both implementations,
behind the spec at `specs/apps/rhino/behavior/cli/gherkin/env/` [Repo-grounded]. This plan
**extends** that family via the mandated spec-first flow.

### 5.0 The two current misses (identical in both implementations)

`internal/envbackup` `Discover` misses non-`.env` secrets in **two** independent ways, in **both**
languages:

1. **Hidden-dir skip** — `discover.go:41` (`strings.HasPrefix(base, ".")` → `filepath.SkipDir`) and
   `discover.rs:50` (`base.starts_with('.')` → prune) [Repo-grounded]. The walker never descends into
   `.secrets/` (gitignored via `.gitignore:104` [Repo-grounded]).
2. **Basename-prefix filter** — `discover.go:52` (`!strings.HasPrefix(base, ".env")`) and
   `discover.rs:71` (`!base.starts_with(".env")`) [Repo-grounded]; `restore` mirrors it at
   `restore.go:97` and `ops.rs:294` (via `base_starts_with_env`) [Repo-grounded]. So `secrets.json`,
   `*.pem`/`*.key`/`*.crt`/`*.pfx` (all gitignored via `.gitignore:105,108–111` [Repo-grounded]) are
   silently skipped.

### 5.1 The widening (spec → Rust → Go → shadow-diff)

Carve `.secrets/` out of the hidden-dir skip (a single blessed-secrets-dir exception) and replace the
single-prefix filter with an explicit **secret-file allowlist**:

| Pattern                                            | Examples (real files are gitignored)              | Status                                                    |
| -------------------------------------------------- | ------------------------------------------------- | --------------------------------------------------------- |
| `.env`, `.env.*` (existing)                        | `apps/<app>/.env.local`, root `.env`              | active                                                    |
| `secrets.json` (NEW)                               | `secrets.json` at any non-skipped path            | active                                                    |
| `*.pem`, `*.key`, `*.crt`, `*.pfx` (NEW)           | `cert.pem`, `server.key`                          | active                                                    |
| any file under `.secrets/` (NEW)                   | `.secrets/notes.md` (freeform local secret notes) | active                                                    |
| `*.tfvars`, `*.tfvars.json`, generated inventories | `infra/.../terraform.tfvars`, `inventory.ini`     | **commented/gated** (R3/R11 — activate when IaC is added) |

Mechanics and safety:

- `.secrets/` is the one **blessed-secrets-dir exception** to the hidden-dir skip: descend into a
  top-level `.secrets/` (every file inside is gitignored), while still skipping all other dot-dirs
  (`.git`, `.next`, …). Inside `.secrets/`, every file qualifies (no basename filter).
- Discovery still honors the existing skip-dirs, the max-size guard, and the inside-repo backup-dir
  refusal — the allowlist only **widens which basenames qualify** plus the one `.secrets/` exception.
- `*.example` / `*.tfvars.example` templates are tracked, so they are not the backup target; the real
  gitignored files are.
- The same widened allowlist drives `restore` (its non-config filter widens in lockstep) so a backup
  round-trips exactly.
- The `*.tfvars` / inventory rows ship **commented** in both the allowlist code and the hub-doc census
  with an "activate when IaC is added" note (R3/R11) — primer has no IaC, so activating them now would
  match nothing and imply tooling that does not exist.

### 5.2 `--dry-run` for `backup` and `restore`

Add a `--dry-run` boolean to both subcommands in both implementations: thread `dry_run` into the
shared `Options`; when set, compute the exact file set and intended actions using the same
discovery/skip-dir logic but perform **no** filesystem writes; report a "would back up / would
restore" list. Honored across all three output formats (text/json/markdown), implies no overwrite
prompt.

### 5.3 Spec-first, both-land-together, shadow-diff

Per the
[rhino-cli Dual-Implementation Parity Convention](../../../repo-governance/conventions/structure/rhino-cli-dual-implementation-parity.md)
[Repo-grounded]:

1. **Spec first**: add/extend scenarios in
   `specs/apps/rhino/behavior/cli/gherkin/env/env-backup.feature` and `env-restore.feature` for the
   new secret kinds and `--dry-run` (one primary `Given`/`When`/`Then` per scenario).
2. **Rust canonical**: implement in `apps/rhino-cli-rust/`.
3. **Go twin**: implement the identical behavior in `apps/rhino-cli-go/`.
4. **Shadow-diff**: run `apps/rhino-cli-rust/scripts/shadow-diff.sh` (the permanent `parity` job) —
   stdout/stderr/exit codes byte-identical across all formats.

## 6. `env validate` Drift Guard (new subcommand, spec-first dual-impl)

A new `rhino-cli env validate` subcommand diffs each app's **declared** keys (its
`infra/dev/<app>/.env.example`) against the env vars its code **actually reads**, exiting non-zero on
any non-empty diff (subject to a per-surface allowlist). All extraction is line-oriented regex — **no
new parser dependency** added in either implementation.

### 6.1 App validator (the only active surface in primer)

1. Parse `infra/dev/<app>/.env.example` into **declared keys** (ignoring comments/blank lines).
2. Scan the app's source for **read keys**, language-aware:
   - Rust: `env::var("…")` / `std::env::var("…")` literals + `envy` struct field names.
   - Go: `os.Getenv("…")` / `os.LookupEnv("…")` literals + `env:"X"` struct tags.
   - TS: `process.env.KEY` / `process.env["KEY"]` + `createEnv({...})` keys + Effect `Config.string("X")`.
   - JVM/.NET/Elixir/Python/etc.: the family's literal env-read form (`System.getenv("X")`,
     `Environment.GetEnvironmentVariable("X")`, `System.get_env("X")`, `os.environ.get("X")`,
     `System/getenv "X"`), enumerated per family in the contract.
3. Report **declared-but-unread** and **read-but-undeclared** sets; exit non-zero if either is
   non-empty (subject to allowlist — e.g. `ENABLE_TEST_API`, framework-injected `PORT`).

### 6.2 Terraform/Ansible validators — gated scaffold only (R3)

The Terraform (`tfvars.example` vs `variables.tf`) and Ansible (`.env.example` vs playbook `lookup`
keys) validators ship **documented and code-stubbed but inert**: no surface of kind `terraform` or
`ansible` is configured for primer, so `env validate` runs **only** the app surface. The hub doc and
the contract schema document how a fork activates them when it adds IaC. This is decision R3.

### 6.3 Configuration & spec-first dual-impl

- A single `env-contract.yaml` (or an existing-config block — chosen at execution against rhino-cli's
  existing config pattern, no new crate) lists the **surfaces** to validate: root, kind
  (`app` only in primer), source globs, and an allowlist of intentionally-exempt keys.
- Authored spec-first: new `specs/apps/rhino/behavior/cli/gherkin/env/env-validate.feature` → Rust
  canonical → Go twin → shadow-diff. `env validate` must appear in both spec-coverage targets.

### 6.4 Wiring

- **Pre-push**: add `rhino-cli env validate` to `.husky/pre-push` after the existing checks. One
  invocation validates every configured (app) surface.
- **CI**: add a step invoking `rhino-cli env validate` in the appropriate `.github/workflows/`
  workflow (matched to the repo's existing layout at execution).

## 7. Dependency Additions & Security Clearance (Dependency-Bump Policy)

All new dependencies are governed by the
[Dependency Bump Policy](../../../repo-governance/development/workflow/dependency-bump-policy.md).
None has an LTS line relevant here → **all are Path B** (latest version released ≥ 60 days before the
bump date AND CVE-clean). `zod` is named in the policy as a Path-B example. The new rhino-cli surface
(widened backup/restore, `--dry-run`, `env validate`) adds **no** new external crates/modules in
either implementation — the extractors are line-oriented regex over already-walked files.

| Dependency                                                                     | Manifest                                                                                                          | Path | Clearance (verify at execution)                                                                               |
| ------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------- | ---- | ------------------------------------------------------------------------------------------------------------- |
| `dotenvy`                                                                      | `apps/crud-be-rust-axum/Cargo.toml`                                                                               | B    | TBD at execution                                                                                              |
| `envy`                                                                         | `apps/crud-be-rust-axum/Cargo.toml`                                                                               | B    | TBD at execution                                                                                              |
| `github.com/caarlos0/env/v11`                                                  | `apps/crud-be-golang-gin/go.mod`                                                                                  | B    | TBD at execution                                                                                              |
| `@t3-oss/env-nextjs`                                                           | `apps/crud-fe-ts-nextjs/package.json`, `crud-fs-ts-nextjs/package.json`                                           | B    | TBD at execution                                                                                              |
| `zod`                                                                          | `apps/crud-fe-ts-nextjs/package.json`, `crud-fs-ts-nextjs/package.json`, `crud-fe-ts-tanstack-start/package.json` | B    | TBD at execution                                                                                              |
| `pydantic-settings`                                                            | `apps/crud-be-python-fastapi/pyproject.toml`                                                                      | B    | **ALREADY INSTALLED** (`pydantic-settings==2.13.1` — no new dep needed) [Repo-grounded — `pyproject.toml:18`] |
| Spring Validation starter (Jakarta)                                            | `apps/crud-be-java-springboot/pom.xml`                                                                            | B    | TBD at execution                                                                                              |
| Long-tail validator libs (Vert.x, Ktor, F#, C#, Clojure, TanStack-Start, Dart) | respective manifests                                                                                              | B    | **lib + pin selected at execution** (or framework-native, no dep)                                             |

### Execution-time obligations (HARD)

Because a plan may span more than 60 days, the exact eligible version and CVE status are resolved **at
execution**. When the relevant phase runs, the executor MUST:

1. **Compute the cutoff in writing**: `Today − 60 days = cutoff`; eligible = released on/before cutoff
   (Path B). Record it in this section.
2. **Select the most recent eligible version** (not yanked / no open release-blocker).
3. **Pin exactly** — no caret/tilde/range. Verify per manifest (e.g.
   `grep -E '"\^|"~' apps/crud-fe-ts-nextjs/package.json` returns nothing).
4. **CVE-clear** against NVD, GitHub Advisories, Snyk, the project page, and CISA KEV; record EPSS for
   any CVE with CVSS ≥ 7.0. Fill the clearance column with `CLEAR` / `CLEAR (patch-of …)` / `WAIVER` /
   `FUNCTIONAL-HOLD`.
5. **Re-audit** post-install (`cargo audit`, `npm audit --audit-level=moderate`, `pip-audit`/`uv`
   equivalent, language-appropriate audit for the JVM/.NET families) and resolve at root cause.
6. **Record results** here; if Path C is ever required, add a Security Waivers subsection and
   propagate to the repo's security-waivers reference if one exists.

For the long-tail families, the executor MAY choose a **framework-native + explicit-guard** approach
(no new dependency) where that is the idiomatic fail-fast path; in that case no clearance row is
needed for that family — only the code change.

## 8. File-Impact Analysis

| File / area                                                                           | Change                                                                                                                    | Phase |
| ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- | ----- |
| `apps/crud-be-*/…config…` (11 backends)                                               | Per-app rename of `APP_JWT_SECRET`/`JWT_SECRET`/`APP_PORT`/`PORT` (app-defined only); later switch to fail-fast validator | 1, 4  |
| `apps/crud-fs-ts-nextjs/src/lib/jwt.ts`                                               | Rename server-side `APP_JWT_SECRET` → `CRUD_FS_TS_NEXTJS_JWT_SECRET`                                                      | 2     |
| `apps/crud-fe-ts-nextjs/`, `crud-fs-ts-nextjs/` (`src/env.ts` new, `next.config.ts`)  | Add `t3-env` + `zod` validated env; import into `next.config.ts`                                                          | 4     |
| `apps/crud-fe-ts-tanstack-start/`, `crud-fe-dart-flutterweb/`                         | Frontend env validation (zod / Dart build-define)                                                                         | 2, 4  |
| `apps/crud-be-ts-effect/src/config.ts`                                                | Remove `catchAll`-to-defaults swallow; required vars become fail-fast `ConfigError`                                       | 4     |
| `infra/dev/<app>/.env.example` (15 files)                                             | Per-app rename (§2) → annotate (§5)                                                                                       | 1,2,5 |
| `infra/dev/<app>/docker-compose*.yml`                                                 | Rename env-block keys to per-app-prefixed names                                                                           | 1, 2  |
| `apps/<app>/Cargo.toml` / `go.mod` / `package.json` / `pyproject.toml` / `pom.xml`    | Add the per-language validator dep (exact pin, Path B) where a dep is needed                                              | 4     |
| `specs/apps/rhino/behavior/cli/gherkin/env/env-backup.feature`, `env-restore.feature` | New scenarios: secret-kind allowlist + `--dry-run`                                                                        | 3     |
| `specs/apps/rhino/behavior/cli/gherkin/env/env-validate.feature` (new)                | New scenarios for the drift guard                                                                                         | 6     |
| `apps/rhino-cli-rust/src/internal/envbackup/discover.rs`, `ops.rs`, `commands/env.rs` | Carve `.secrets/` exception; widen allowlist; `--dry-run`; tests (canonical)                                              | 3     |
| `apps/rhino-cli-go/internal/envbackup/discover.go`, `restore.go`, `cmd/env_*.go`      | Same widening + `--dry-run`, byte-identical (twin)                                                                        | 3     |
| `apps/rhino-cli-rust/src/commands/env_validate.rs` (new) + tests                      | App validator (canonical); Terraform/Ansible validators stubbed-but-gated                                                 | 6     |
| `apps/rhino-cli-go/cmd/env_validate.go` (new) + tests                                 | App validator (twin), byte-identical                                                                                      | 6     |
| `apps/rhino-cli-rust/scripts/shadow-diff.sh` (run, not edited)                        | Parity gate run after every rhino-cli change                                                                              | 3, 6  |
| `.husky/pre-push`                                                                     | Invoke `rhino-cli env validate`                                                                                           | 6     |
| `.github/workflows/` (existing)                                                       | Invoke `rhino-cli env validate`                                                                                           | 6     |
| `repo-governance/development/quality/secrets-and-env-standards.md` (new)              | Hub convention                                                                                                            | 7     |
| `repo-governance/development/quality/no-secrets-in-committed-files.md`                | Reduce to stub redirect (preserve `done/` inbound links)                                                                  | 7     |
| `repo-governance/development/quality/env-file-access.md`                              | Reduce to stub redirect                                                                                                   | 7     |
| `repo-governance/development/workflow/reproducible-environments.md`                   | Reduce to stub redirect                                                                                                   | 7     |
| `docs/explanation/standardize-secrets-and-env-parity-decisions.md` (new)              | Plain-language rationale for every decision (esp. PR-override, no-IaC scaffold, full polyglot adoption)                   | 7     |
| Active inbound links (root governance docs, indexes, docs/, agents/skills)            | Repoint to hub doc; `done/` plan links left on stubs                                                                      | 7     |

## 9. Research Findings (cited)

- **Go validator** = `caarlos0/env` v11 (MIT, zero-dep), tag `env:"KEY,required"`, hard error on a
  missing required var — closest analogue to Rust `envy`. Avoid `kelseyhightower/envconfig` (stale)
  and `spf13/viper` (no required-validation).
  Source: [pkg.go.dev/github.com/caarlos0/env/v11](https://pkg.go.dev/github.com/caarlos0/env/v11)
  (accessed 2026-06-09).
- **Effect `Config`** fails fast at **runtime only** and does **not** enforce `NEXT_PUBLIC_`. So
  `@t3-oss/env-nextjs` + `zod` remains required for the Next.js **build-time** + browser-exposure
  guarantee; Effect `Config` is the right runtime validator for the non-Next Effect backend.
  Source: [effect.website/docs/configuration](https://effect.website/docs/configuration),
  [env.t3.gg/docs/nextjs](https://env.t3.gg/docs/nextjs) (accessed 2026-06-09).
- **Python** = `pydantic-settings` `BaseSettings` (required field, no default → `ValidationError` at
  load) — `pydantic` is already a dependency [Repo-grounded — `pyproject.toml`].
  Source: [docs.pydantic.dev/latest/concepts/pydantic_settings](https://docs.pydantic.dev/latest/concepts/pydantic_settings/)
  (accessed 2026-06-09).
- **No off-the-shelf cross-language drift guard** exists (dotenv-linter is file-to-file only); the
  custom per-language regex approach in §6 mirrors the `ose-infra` reference's `env validate`.
- **Long-tail validators (Vert.x, Ktor, F#, C#, Clojure, TanStack-Start, Dart)**: each family has a
  framework-idiomatic fail-fast path (e.g. C# `IOptions` + `ValidateOnStart()`, Spring
  `@ConfigurationProperties` + `@Validated`, Dart `String.fromEnvironment` + `assert`). The exact
  library or framework-native approach is `[Needs Verification]` at authoring time and is confirmed +
  pinned at execution (§7). Marked here rather than asserted as fact.

## 10. Risks & Rollback

- **Rename misses a read site** → app reads a default silently. Mitigation: the naming phases re-grep
  each app to zero before its gate; the `env validate` guard makes recurrence impossible.
- **Parity break** (one implementation changed without the other) → Mitigation: spec-first flow +
  shadow-diff `parity` gate run at each rhino-cli phase gate; both-land-together is a hard rule.
- **Long-tail validator misidentified** → Mitigation: exact lib/approach selected at execution under
  the bump policy; framework-native + guard is an acceptable no-dep path.
- **Doc fold breaks links** → Mitigation: stub redirects keep old paths live; link check gates each
  commit. Rollback: stubs revert to full docs from git history.
- **New dependency regression / CVE** → Mitigation: Path B + exact pins + CVE clearance (§7). Rollback:
  each dep add is an isolated commit.
- **PR-override push lands an undesired change on `main`** → Mitigation: per-phase thematic commits are
  individually revertable; no `--force`; the deviation is recorded (R5) so the bypass is auditable.

### Rollback Strategy

Each phase is an independent thematic commit. Reverting a single phase's commit restores the prior
state, except ordered dependencies: Phase 4 (validation) assumes Phases 1–2 (rename); Phase 6 (guard)
assumes Phases 1–2 + 3 + 5. Revert in reverse phase order when unwinding multiple phases.
