# Tech Docs — Standardize Secrets and Environment-Variable Storage (ose-primer)

This document holds the resolved deviation matrix, the design decisions and their rationale, the
file-impact analysis, and the mechanics. The step-by-step checklist lives in
[delivery.md](./delivery.md).

## 1. Resolved Deviation Matrix (all 16 decisions)

This is the full cross-repo deviation matrix from the parity effort, reproduced verbatim with
`ose-primer`'s column and justification. The matrix is the source of truth; nothing in this plan
silently deviates from it. **Deviation count: 16 recorded decisions, 0 silent deviations.**

| #    | Dimension                 | Decision (primer)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Justification                                                           |
| ---- | ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| R1   | Parity set                | primer is authored alongside public; infra is the reference (already gated/passing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | infra plan already gated; avoid regression                              |
| R2   | Delivery mode             | **`worktree-to-main`** (push directly to `ose-primer` `main`)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | invoker choice                                                          |
| R3   | IaC surfaces              | **Forward-looking scaffold** — IaC validators + `*.tfvars`/inventory backup patterns shipped **commented/gated** "when IaC is added"; primer has no IaC today                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | keep design ready without standing up inert tooling                     |
| R4   | Research                  | Ran — Go/Effect/long-tail validator findings in [§9](./tech-docs.md#9-research-findings-cited)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | tooling needed verification                                             |
| R5   | **PR override**           | **DEVIATION ACCEPTED**: primer is delivered `worktree-to-main` despite the sibling-sync rule that `ose-primer` receives governance changes via a **draft PR**. Explicit, one-off, **invoker-owned**. Recorded here and in the Phase 7 rationale doc. No `--force` ever.                                                                                                                                                                                                                                                                                                                                                                                                                 | invoker explicitly accepted; one-off, owned by invoker                  |
| R6   | rhino-cli tooling         | **Spec-first dual-impl**: update `specs/apps/rhino/behavior/cli/gherkin/env/` → implement **Rust (canonical)** → implement **Go (twin)** → shadow-diff byte-identical. `env validate` authored Rust-canonical.                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | preserve primer's mandated dual-CLI parity model (see correction below) |
| R7   | Startup validation        | **Full adoption in every app** (11 backends + 4 frontends)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | maximal convergence (invoker)                                           |
| R8   | primer polyglot reach     | **All 11 backends** — validator-per-language table ([§4](./tech-docs.md#4-startup-validation-per-language)) + one delivery sub-phase per family                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | honors full-adoption literally                                          |
| R9   | Naming prefix             | **Full per-app prefix rename across all existing app-defined vars**                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | maximal convergence (invoker)                                           |
| R10  | Hub doc                   | New `repo-governance/conventions/security/secrets-and-env-standards.md`; fold the three existing docs to stub redirects at their **new** `conventions/security/` location                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | cross-repo structural symmetry; ose-infra canonical                     |
| R10b | **Doc canonicalization**  | **THIS repo acts**: MOVE `no-secrets-in-committed-files.md` and `env-file-access.md` from `repo-governance/development/quality/` → `repo-governance/conventions/security/` to match the `ose-infra` canonical layout; fold both into the hub `conventions/security/secrets-and-env-standards.md` with stubs at the **new** location; REWRITE all inbound links. (`reproducible-environments.md` stays under `development/workflow/`; only its secrets/env content folds out.) Authorized canonicalization; primer's `repository-ecosystem` convention pins these under `development/quality/`, so the ecosystem-convention update is a **downstream follow-up**, not part of this plan. | ose-infra is the canonical reference; align primer's paths to it        |
| R11  | Backup allowlist          | **Real floor + IaC gated scaffold**: `.env*` + `.secrets/` + `secrets.json` + `*.pem`/`*.key`/`*.crt`/`*.pfx`; `*.tfvars`/inventory patterns shipped **commented** forward-scaffold. Hybrid floor ∪ optional registry `backup_globs`.                                                                                                                                                                                                                                                                                                                                                                                                                                                   | per-repo honesty + ready for IaC                                        |
| R11b | **Backup default dir**    | **Adopt canonical per-repo-derived `~/<repo-root-basename>-env-backup`** (replacing today's hardcoded `ose-open-env-backup`), implemented in **BOTH** `rhino-cli-rust` **and** `rhino-cli-go`, with the **shadow-diff parity gate as a hard acceptance criterion**. All-three-align; ose-infra canonical; primer applies it in both twins.                                                                                                                                                                                                                                                                                                                                              | ose-infra canonical; primer applies in both twins                       |
| R12  | Layout                    | **primer keeps its `infra/dev/<app>/` layout and root `.env.example`** (no migration to `apps/<app>/`); `env init` keeps walking `infra/dev/`. Real gitignored-file relocations (none expected) would be **[HUMAN]** (env-file-access guard).                                                                                                                                                                                                                                                                                                                                                                                                                                           | primer is a template repo; preserve its layout + Nx scaffold path       |
| R13  | Rationale doc             | `docs/explanation/standardize-secrets-and-env-parity-decisions.md`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | aligns with existing `*-parity-decisions.md` precedents                 |
| R14  | Drift `APP_PORT` live-fix | **DROP** — no such live drift exists in primer (the `APP_*` reads are renamed for naming, not to fix a live mismatch)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | no such drift exists here                                               |

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

### Cross-repo deviation matrix (parity set)

The three sibling plans deliver the same end-state — named, startup-validated, drift-guarded,
fully-backed-up secrets/env consolidated into one hub doc — but are **not identical**: each diverges
where its repo's reality forces it. **`ose-infra` is the CANONICAL reference member**; its governance
paths are already canonical (`repo-governance/conventions/security/`), so the siblings canonicalize
**to** `ose-infra`. Every divergence below is intentional and recorded (zero silent deviations).

| #    | Dimension             | ose-infra (canonical)                          | ose-primer (this plan)                                                        | ose-public                                                    |
| ---- | --------------------- | ---------------------------------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------- |
| R2   | Delivery mode         | `main-to-main` (Trunk-Based, normal)           | `worktree → push origin main` (PR-override, maintainer-authorized)            | `worktree → push origin main`                                 |
| R3   | Terraform/Ansible     | full IaC validators + backup (**real**)        | **none → N/A** (no infra; never receives infra artifacts; gated scaffold)     | none → forward-scaffold (commented)                           |
| R6   | rhino-cli tooling     | Rust, single impl                              | **spec-first DUAL-impl** (Rust canonical + Go twin, shadow-diff)              | Rust, single impl                                             |
| R7   | Startup validation    | `coralpolyp-be` + `coralpolyp-fe`              | full adoption, all 11 polyglot backends + 4 frontends                         | full adoption, both Rust backends + all Next.js webs          |
| R9   | Naming prefix         | `CORALPOLYP_BE_*` rename                       | full per-app prefix rename                                                    | full per-app prefix rename                                    |
| R10b | Doc canonicalization  | already canonical (`conventions/security/`)    | **MOVE** docs `development/quality/` → `conventions/security/`; rewrite links | rename `no-secrets-in-git.md` → canonical name; rewrite links |
| R11  | Backup floor          | `.env*`, `*.tfvars`, inventories, `.secrets/`  | `.env*`, `.secrets/`, `secrets.json`, `*.pem/key/crt/pfx` + IaC scaffold      | `.env*`, `.secrets/`, `secrets.json` + IaC scaffold           |
| R11b | Backup default dir    | `~/<repo-basename>-env-backup` (**canonical**) | adopt canonical derivation, in **BOTH** rust + go twins (shadow-diff parity)  | adopt canonical derivation                                    |
| R12  | Layout                | `infra/dev/<app>` → `apps/<app>`               | keep root template (no migration)                                             | consolidate `apps/<app>` (remove `infra/dev` dup)             |
| R13  | Parity-decisions doc  | `docs/explanation/…-parity-decisions.md`       | same path under primer `docs/explanation/`                                    | same path under public `docs/explanation/`                    |
| R14  | Live `APP_PORT` drift | fix (real bug)                                 | drop (no drift)                                                               | drop (no drift)                                               |

**On R2 (delivery mode) — the `ose-primer` PR-override.** This parity run delivers all three plans the
same operational way: a git worktree, then a direct push to `origin main` (no PR). For `ose-infra` this
is ordinary Trunk-Based Development. For `ose-primer` it is an **explicit, maintainer-authorized
deviation** from that repo's PR-only sync invariant — justified as a synchronized cross-repo parity
landing treated as one maintainer-driven operation rather than three independent contributions. The
corresponding change to the `ose-primer` sync governance is a **separate downstream follow-up**, NOT
part of this plan.

**On R3 (IaC = N/A, primer-specific).** `ose-primer` holds no Terraform or Ansible and, per its
repository-ecosystem convention, **never receives infra artifacts**. So the Terraform/Ansible
drift-guard surfaces are **dropped as N/A** — unlike `ose-infra` (where they are real) and `ose-public`
(where they ship commented as a forward-scaffold). Primer ships only the inert contract-schema stub so
a downstream fork can activate the discipline if it ever adds IaC.

**On R10b / R11b (canonicalization toward ose-infra).** Primer fully canonicalizes its governance doc
paths to match `ose-infra`: it **moves** `no-secrets-in-committed-files.md` and `env-file-access.md`
from `development/quality/` → `conventions/security/`, folds them into the hub, and rewrites inbound
links. Primer's `repository-ecosystem` convention currently pins these docs under `development/quality/`,
so the matching ecosystem-convention update is an **authorized downstream follow-up**, not part of this
plan. The per-repo-derived backup default dir (`~/<repo-root-basename>-env-backup`), previously an
`ose-infra`-only behavior, is now the **canonical standard adopted by all three** — primer applies it in
**both** its Rust and Go rhino-cli twins so the shadow-diff stays in parity.

## 2. Naming Standard

**On 12-factor authority (precise framing).** The Twelve-Factor App is **silent on naming
structure**: it mandates config-in-environment and per-deploy values, but prescribes nothing about
prefixes or casing. So 12-factor **authorizes** a per-app prefix without **prescribing** it; the
prefix is a **practitioner-consensus** convention for shared environments where many services' vars
coexist (primer's compose stacks load several apps' vars at once), not a 12-factor requirement. This
is also what resolves primer's current inconsistency — some backends read `APP_PORT`, others read a
bare `PORT` for the same app-owned bind — by giving every app-defined value one per-app-prefixed
form. Distinct from app-defined names is a **framework-reserved / exempt class** that this standard
never prefixes:

| Reserved/exempt name | Why exempt                                           |
| -------------------- | ---------------------------------------------------- |
| `NEXT_PUBLIC_*`      | Framework-required (Next.js browser-exposure prefix) |
| `PORT`               | Platform convention (host/PaaS injects it)           |
| `NODE_ENV`           | Node reserved                                        |
| `DATABASE_URL`       | Cross-ecosystem convention, intentionally unprefixed |

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
> environment tier in the name. 12-factor is silent on prefix/casing structure; it authorizes but does
> not prescribe per-app prefixes (those are practitioner consensus, not a 12-factor mandate).

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

> **Env-loading mechanics (why colocation matters, recorded for forks).** `.env.local` is loaded from
> each frontend app's **root**, not from `src/`; `.env.example` is **never** auto-loaded by Next.js or
> Nx (it is a committed documentation/template file only). Nx loads env from **both** the workspace
> root **and** the project root, with the **project root taking priority** — so colocating env files
> with the app is precisely what enables auto-loading. Primer's current layout keeps these templates
> under `infra/dev/<app>/` (R12), so primer's apps rely on the documented dev workflow rather than
> tool-mandated auto-load paths; the colocation rationale is recorded here so a downstream fork that
> chooses to migrate to `apps/<app>/` understands what auto-loading it would gain. (Cross-tool
> consensus: Nx and Turborepo both colocate env files with the package.)

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

- New crate deps: `dotenvy`, `envy` (serde is already present). `dotenv` (the unmaintained
  predecessor, RUSTSEC-2021-0141) is **not** used. Exact pins chosen at execution per §7.
- **`envy` staleness caveat.** `envy`'s last crates.io release is `0.4.2` (Jan 2021, ~5 years stale).
  It carries **no** RustSec/CVE advisory and is functionally complete for its narrow scope (deserialize
  env vars into a serde struct), so it stays a Path-B candidate — but the staleness is recorded
  explicitly in §7 and a `Cargo.toml` comment notes a re-evaluation trigger: if a RustSec advisory
  analogous to RUSTSEC-2021-0141 (the `dotenv` unmaintained flag) is ever filed against `envy`,
  re-evaluate the dependency.
- **`dotenvy` note.** `dotenvy` is the accepted successor to the unmaintained `dotenv`
  (RUSTSEC-2021-0141). Its last release is `0.15.7` (Mar 2023, ~3-year gap; the `0.16` branch is
  unpublished); it carries no CVE — stable-but-not-recently-released rather than abandoned. Pin `"0.15.7"`.

> Source: [envy — docs.rs](https://docs.rs/envy) (accessed 2026-06-09): field `foo_bar` ↔ `FOO_BAR`;
> non-`Option` fields fail fast when absent. Last release `0.4.2` (Jan 2021); no RustSec advisory.

<!-- separates adjacent blockquotes (markdownlint MD028) -->

> Source: [dotenvy — crates.io](https://crates.io/crates/dotenvy) (accessed 2026-06-09): maintained
> successor to the unmaintained `dotenv` crate (RUSTSEC-2021-0141). Last release `0.15.7` (Mar 2023);
> no CVE.

### Go (`crud-be-golang-gin`) — `caarlos0/env` v11

Replace the `os.Getenv` reads in `apps/crud-be-golang-gin/internal/config/config.go` [Repo-grounded]
with a struct whose fields carry `env:"CRUD_BE_GOLANG_GIN_JWT_SECRET,required"` tags; `env.Parse`
returns a hard error naming a missing required var.

### Next.js frontends — plain `zod` (as-executed) / `@t3-oss/env-nextjs` (plan)

**As-executed divergence:** plan specified `@t3-oss/env-nextjs` + `zod`; execution used **plain `zod`
only** in `src/env.ts` (t3-env installed in `package.json` but not used in env module). Root cause:
`@t3-oss/env-nextjs` uses a Proxy for server-var access that throws in the vitest jsdom environment
(`TypeError: Cannot access server-side environment variable ... on the client`). Plain `z.object({
... }).parse({ ... })` avoids the Proxy path entirely, satisfies the same validation contract, and
keeps vitest clean. The `@t3-oss/env-nextjs` + `@t3-oss/env-core` packages remain in `package.json`
(locked during Phase 4) and their §7 clearance row stands — they are installed but not load-path
active in the env module.

Add `src/env.ts` exporting a validated `env` object; import it in `next.config.ts` so validation runs
at build time. The `createEnv({ server, client, runtimeEnv })` API (t3-env) is not used but was
planned; the plain-zod shape achieves the same fail-fast behavior at build and runtime.

- **`zod` v4 API form (HARD).** The default `zod` export is v4 (v4 has been the default export since
  Jul 2025; v3 now lives at `zod/v3`). In v4 the string-format helpers moved to top-level functions:
  `z.string().email()` / `.uuid()` / `.ip()` became `z.email()` / `z.uuid()` / `z.ipv4()`. The env
  schemas in this plan MUST use the new top-level form (e.g. `z.url()`, not `z.string().url()`). The
  hub doc's annotation/validation section records this so future env schemas do not regress to the v3
  form. Pin on the **4.x** line, exact pin (no caret/tilde — keep the exact-pin policy).
- **`zod` is an OPTIONAL peer of `t3-env`, not a hard requirement.** `@t3-oss/env-nextjs` accepts any
  Standard-Schema-v1 validator (Valibot, ArkType, …); `zod` is needed only because we author
  zod-based schemas. The dependency on `zod` is ours, not transitively forced by t3-env — relevant
  when reasoning about the dependency surface in §7.
- **Next.js standalone caveat.** A standalone Next.js build must list `@t3-oss/env-nextjs` and
  `@t3-oss/env-core` in `transpilePackages` (`next.config.ts`) so the validator is bundled. t3-env
  requires Next.js ≥ 13.4.4 (both primer Next.js frontends satisfy this).

> Source: [T3 Env — Next.js](https://env.t3.gg/docs/nextjs) (accessed 2026-06-09): import `env.ts`
> into `next.config.ts` for build-time validation; client vars must carry `NEXT_PUBLIC_`; zod is an
> optional Standard-Schema-v1 peer (any Standard-Schema-v1 validator works); standalone builds need
> `@t3-oss/env-nextjs` + `@t3-oss/env-core` in `transpilePackages`; Next.js ≥ 13.4.4 required.

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

### 5.2b Backup default directory — canonical per-repo derivation (R11b)

Today both twins hardcode `DEFAULT_BACKUP_DIR = "ose-open-env-backup"` (`types.rs:99` and the Go
mirror) [Repo-grounded], so `env backup`/`restore` default to `~/ose-open-env-backup`. This plan
adopts the **canonical per-repo-derived** default `~/<repo-root-basename>-env-backup`: the basename
comes from the already-computed repo-root path (the same `git`-root result the discovery walker uses),
so `ose-primer` defaults to `~/ose-primer-env-backup`. This matches the `ose-infra` canonical behavior
(now adopted by all three repos) and stops sibling repos from colliding on one shared backup dir.

Because the change touches a shared constant and its `--dir`-empty fallback, it **MUST land in BOTH**
`rhino-cli-rust` (canonical) **and** `rhino-cli-go` (twin), and the **shadow-diff parity gate passing
is a hard acceptance criterion** for the change — the help text, the default path, and every output
format must stay byte-identical across the twins. Update the example/help strings (`commands/env.rs`
and `cmd/env_*.go`) in lockstep so no twin advertises the old `ose-open-env-backup` path.

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

#### App-surface regex extractor failure modes (known, deliberate)

Because the env-read extractor is line-oriented regex with **no language-parser dependency**, it has
known false-positive/negative modes on the **app** surface (the only surface primer runs). These are
documented openly — the per-surface allowlist + the required-comment rule **surface** (not silence)
any case the regex cannot handle:

- **Dynamic / computed reads** — a key built at runtime (`os.Getenv(prefix + name)`,
  `process.env[keyVar]`, `System.getenv(buildKey())`) cannot be resolved statically by a
  literal-argument matcher; it must be allowlisted with a comment so the contract records it.
- **Multi-line constructs** — a read split across lines, a struct-tag block, or an `import.meta.env`
  destructure spanning multiple lines can confuse the line-oriented matcher.
- **String/comment false hits** — the literal text of a key appearing inside a comment or unrelated
  string can register as a spurious read.

Mature linters use full parsers; the regex here is a **deliberate lightweight first-approximation**
sized to primer's actual app files, not a general source analyzer. Any construct the regex cannot
resolve statically MUST be allowlisted with a comment, so the unsupported case is visible in the
contract rather than silently mis-scanned. The extractors are unit-tested against the real app files
to keep the approximation honest. (Primer holds **no** Terraform or Ansible surface, so the
HCL/heredoc and Ansible `lookup(...)`/Jinja2 regex modes the `ose-infra` reference documents are
**N/A here** — see §6.2.)

### 6.2 Terraform/Ansible validators — N/A for ose-primer (no infra); gated scaffold only (R3)

`ose-primer` holds **no Terraform or Ansible**, and per its repository-ecosystem convention it
**never receives infra artifacts** — so the Terraform/Ansible drift-guard surfaces are **N/A here**.
This is the primer-specific contrast in the parity set: `ose-infra` has these validators **real** and
`ose-public` ships them **commented as a forward-scaffold**, whereas primer drops them as N/A. To keep
the contract schema cross-repo-symmetric (so a downstream fork that adds IaC can activate the
discipline with a config flip rather than a redesign), the Terraform (`tfvars.example` vs
`variables.tf`) and Ansible (`.env.example` vs playbook `lookup` keys) validators ship **documented
and code-stubbed but inert**: no surface of kind `terraform` or `ansible` is configured for primer, so
`env validate` runs **only** the app surface. The hub doc and the contract schema document how a fork
activates them when it adds IaC. This is decision R3.

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

**Execution-time cutoff (recorded 2026-06-10):** `2026-06-10 − 60 days = 2026-04-11`. Eligible
versions are those released on or before 2026-04-11 that are not yanked and carry no open
release-blocker. GitHub Advisories checked via `gh api advisories?ecosystem=...&package=...` on
2026-06-10. Long-tail families (Vert.x, Ktor, F#, C#, Clojure, Dart) use framework-native +
explicit-guard — no new dependency; no clearance row needed per §7 obligation 6.

| Dependency                                      | Manifest                                                                                                          | Path | Version line                 | Clearance                                                                                                     |
| ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- | ---- | ---------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `dotenvy`                                       | `apps/crud-be-rust-axum/Cargo.toml`                                                                               | B    | `0.15.7` (2023-03-22)        | **CLEAR** — no CVE/RustSec advisory; checked 2026-06-10                                                       |
| `envy`                                          | `apps/crud-be-rust-axum/Cargo.toml`                                                                               | B    | `0.4.2` (**stale**, 2021-01) | **CLEAR (staleness caveat)** — no CVE/RustSec; 5 yr stale; re-eval trigger: if RustSec advisory filed         |
| `github.com/caarlos0/env/v11`                   | `apps/crud-be-golang-gin/go.mod`                                                                                  | B    | `v11.4.0` (2026-02-22)       | **CLEAR** — no v11.x-range advisory in GitHub Advisories; checked 2026-06-10                                  |
| `@t3-oss/env-nextjs`                            | `apps/crud-fe-ts-nextjs/package.json`, `crud-fs-ts-nextjs/package.json`                                           | B    | `0.13.11` (2026-03-22)       | **CLEAR** — no known advisory; checked 2026-06-10                                                             |
| `zod`                                           | `apps/crud-fe-ts-nextjs/package.json`, `crud-fs-ts-nextjs/package.json`, `crud-fe-ts-tanstack-start/package.json` | B    | `4.3.6` (2026-01-22)         | **CLEAR** — listed advisories affect v0.x/v1.x ranges only; v4.3.6 unaffected; checked 2026-06-10             |
| `pydantic-settings`                             | `apps/crud-be-python-fastapi/pyproject.toml`                                                                      | B    | (already pinned)             | **ALREADY INSTALLED** (`pydantic-settings==2.13.1` — no new dep needed) [Repo-grounded — `pyproject.toml:18`] |
| Spring Validation starter (Jakarta)             | `apps/crud-be-java-springboot/pom.xml`                                                                            | B    | BOM-managed (parent `4.0.6`) | **BOM-MANAGED** — spring-boot-starter-parent 4.0.6 governs version; no explicit pin needed                    |
| Long-tail (Vert.x, Ktor, F#, C#, Clojure, Dart) | n/a                                                                                                               | B    | n/a                          | **FRAMEWORK-NATIVE** — no new dep; explicit required-env guard added in code only                             |

**Version-line notes (per-dep):**

- **`zod` — 4.x line, exact pin.** Since Jul 2025 the default `zod` export is v4 (v3 now lives at
  `zod/v3`). New code targets v4, so this plan pins on the **4.x** line. The Dependency Bump Policy
  requires an **exact** pin (no caret/tilde): `"zod": "X.Y.Z"` resolved at execution to the current
  eligible 4.x version. A bare `"^4"` is **not** acceptable (it is a caret range, not exact); `"4"` is
  likewise not an exact pin. The 4.x line is the constraint; the exact patch is filled in at execution.
  v4 also relocates string-format helpers to top-level (`z.email()` / `z.uuid()` / `z.ipv4()` replace
  `z.string().email()` / `.uuid()` / `.ip()`) — env schemas use the new form (§2/§4).
- **`@t3-oss/env-nextjs` — 0.13.x.** `createEnv({ server, client, runtimeEnv })` API is current. `zod`
  is an **optional** Standard-Schema-v1 peer (t3-env accepts Valibot/ArkType/etc.); our `zod`
  dependency is a choice, not a transitive requirement of t3-env. Standalone builds need
  `@t3-oss/env-nextjs` + `@t3-oss/env-core` in `transpilePackages`; Next.js ≥ 13.4.4 required.
- **`dotenvy` — 0.15.7.** Pin `"0.15.7"` (Mar 2023; `0.16` unpublished). No CVE; the accepted
  successor to the unmaintained `dotenv` (RUSTSEC-2021-0141). Stable-but-not-recently-released.
- **`envy` — 0.4.2 (STALENESS CAVEAT).** Last release `0.4.2` (Jan 2021, ~5 years stale); **no**
  CVE/RustSec advisory; functionally complete for its narrow deserialize-env-into-struct scope. It
  stays Path B, but `apps/crud-be-rust-axum/Cargo.toml` carries a comment recording the staleness and
  the **re-evaluation trigger**: revisit if a RustSec advisory analogous to RUSTSEC-2021-0141 is ever
  filed against `envy`. Example pin + comment:

  ```toml
  # envy 0.4.2 is the latest release (Jan 2021); stale but advisory-clean and
  # functionally complete. Re-evaluate if a RustSec advisory (cf. RUSTSEC-2021-0141)
  # is ever filed against envy.
  envy = "0.4.2"
  ```

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

### Post-install audit results (recorded 2026-06-10)

`cargo audit` (Rust workspace, after adding `dotenvy`/`envy`): **1 advisory, 1 warning — both
pre-existing, not in our deps**.

- RUSTSEC-2023-0071 (`rsa 0.9.10`, medium CVSS 5.9): Marvin Attack timing side-channel in
  `sqlx-mysql 0.8.6` transitive dep. No fix available. Pre-existing before Phase 4.
- RUSTSEC-2026-0097 (`rand 0.8.5`, warning/unsound): unsound with custom logger, from `sqlx`
  transitive dep. Pre-existing before Phase 4.

`dotenvy 0.15.7` and `envy 0.4.2`: **no advisory in cargo audit output** — CLEAR.

`npm audit --audit-level=moderate` (`crud-fe-ts-nextjs`, `crud-fs-ts-nextjs` after adding
`@t3-oss/env-nextjs`/`zod`): **2 moderate — pre-existing in `next` itself, not in our deps**.

- GHSA-qx2v-qp2m-jg93 (`postcss < 8.5.10`, moderate): XSS in CSS stringify output; found in
  `node_modules/next/node_modules/postcss`. Pre-existing before Phase 4; fix would require `next@9.3.3`
  (breaking downgrade — not applicable).

`@t3-oss/env-nextjs 0.13.11`, `@t3-oss/env-core 0.13.11`, `zod 4.3.6`: **no advisory output from
npm audit** — CLEAR.

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
| `apps/rhino-cli-rust/src/internal/envbackup/types.rs` + `commands/env.rs` (help)      | Backup default dir → per-repo-derived `~/<repo-basename>-env-backup` (R11b); update help strings (canonical)              | 3     |
| `apps/rhino-cli-go` backup-dir constant + `cmd/env_*.go` (help)                       | Same per-repo-derived default dir + help strings, byte-identical (twin)                                                   | 3     |
| `apps/rhino-cli-rust/src/commands/env_validate.rs` (new) + tests                      | App validator (canonical); Terraform/Ansible validators stubbed-but-gated                                                 | 6     |
| `apps/rhino-cli-go/cmd/env_validate.go` (new) + tests                                 | App validator (twin), byte-identical                                                                                      | 6     |
| `apps/rhino-cli-rust/scripts/shadow-diff.sh` (run, not edited)                        | Parity gate run after every rhino-cli change                                                                              | 3, 6  |
| `.husky/pre-push`                                                                     | Invoke `rhino-cli env validate`                                                                                           | 6     |
| `.github/workflows/` (existing)                                                       | Invoke `rhino-cli env validate`                                                                                           | 6     |
| `repo-governance/conventions/security/secrets-and-env-standards.md` (new)             | Hub convention (new `conventions/security/` location — R10b)                                                              | 7     |
| `repo-governance/conventions/security/no-secrets-in-committed-files.md` (moved stub)  | MOVED from `development/quality/`; reduced to stub redirect at the new path (R10b)                                        | 7     |
| `repo-governance/conventions/security/env-file-access.md` (moved stub)                | MOVED from `development/quality/`; reduced to stub redirect at the new path (R10b)                                        | 7     |
| `repo-governance/development/workflow/reproducible-environments.md`                   | Reduce to stub redirect (stays under `development/workflow/`; only its secrets/env content folds out)                     | 7     |
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
