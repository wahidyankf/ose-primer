---
title: "Secrets and Environment Variable Standards"
description: >
  Hub convention for all secrets and environment-variable discipline in this
  repository: naming, annotation, fail-fast validation, backup/restore tooling,
  drift guard, and the secret-surface census.
category: explanation
subcategory: conventions
tags:
  - security
  - secrets
  - env-vars
  - conventions
---

# Secrets and Environment Variable Standards

One authoritative hub for every secrets and environment-variable rule in this
repository. Three older documents redirect here as stubs:

- [No Secrets in Committed Files Convention](./no-secrets-in-committed-files.md)
- [Environment File Access Convention](./env-file-access.md)
- [Reproducible Environments](../../development/workflow/reproducible-environments.md)

---

## 1. The Iron Rules

**Rule 1 — no secrets in committed files.** Never put system secrets into any
file committed to git. "System secrets" includes SSH keys, passwords, sensitive
usernames, API keys, tokens, OAuth secrets, database connection strings with
real credentials, and cloud provider credentials. The rule applies to **every
committed file type**: plans, docs, source code, tests, fixtures, CI workflows,
shell scripts, commit messages, and configuration. Git history is permanent; a
pushed secret is a leaked secret.

**Rule 2 — env-file access guard.** AI agents operating in this repository MUST
NOT read, write, edit, or commit real `.env*` files (`.env`, `.env.local`,
`.env.production`, etc.). Only `.env.example` is permitted. See the
[Environment File Access Convention](./env-file-access.md) stub for the
six-layer enforcement detail.

**Where secrets belong.** Real secrets go in gitignored locations: `.env`,
`.env.local`, `.env.production`, `.secrets/`, `secrets.json`, `*.pem`/`*.key`/
`*.crt`/`*.pfx`, or an external vault. The committed surface carries only
variable **names and placeholders** — never values.

**Remediation.** If a secret has already been committed: remove it, replace with
a variable-name reference, commit the corrected version, and **rotate the secret
immediately** — git history is permanent so the value is compromised from the
moment it was pushed.

---

## 2. Naming Standard

### 12-factor framing

The Twelve-Factor App mandates config-in-environment but is **silent on naming
structure**. It **authorizes** a per-app prefix without prescribing one; the
prefix is a practitioner-consensus convention for shared environments where
multiple services' vars coexist (primer's compose stacks load several apps'
vars at once), not a 12-factor requirement.

### Variable classes

| Variable class             | Rule                                        | Example                                   |
| -------------------------- | ------------------------------------------- | ----------------------------------------- |
| App-defined value          | `SCREAMING_SNAKE`, per-app prefix           | `CRUD_BE_RUST_AXUM_JWT_SECRET`            |
| Framework-reserved value   | Keep the framework's required name          | `NEXT_PUBLIC_*`, `PORT` (framework-owned) |
| Shared service connection  | Unprefixed, conventional name               | `DATABASE_URL`, `POSTGRES_USER`           |
| Environment tier in a name | **Forbidden** (keys identical across tiers) | ~~`PROD_DATABASE_URL`~~                   |

The per-app prefix is the app's Nx project name uppercased with `_` separators
(`crud-be-rust-axum` → `CRUD_BE_RUST_AXUM_`).

### Framework-exempt names

| Exempt name     | Why                                                                                      |
| --------------- | ---------------------------------------------------------------------------------------- |
| `NEXT_PUBLIC_*` | Framework-required browser-exposure prefix (Next.js)                                     |
| `PORT`          | Platform convention (host/PaaS injects it); exempt where the **framework** owns the bind |
| `NODE_ENV`      | Node reserved                                                                            |
| `DATABASE_URL`  | Cross-ecosystem convention, intentionally unprefixed                                     |

Where **our own code** reads the port (e.g. `crud-be-rust-axum` `APP_PORT`), the
value **is** app-defined and takes the prefix. This asymmetry is documented
explicitly to match the implementation.

### Per-app prefix map

| App (Nx project)            | Prefix                                  |
| --------------------------- | --------------------------------------- |
| `crud-be-clojure-pedestal`  | `CRUD_BE_CLOJURE_PEDESTAL_`             |
| `crud-be-csharp-aspnetcore` | `CRUD_BE_CSHARP_ASPNETCORE_`            |
| `crud-be-elixir-phoenix`    | `CRUD_BE_ELIXIR_PHOENIX_`               |
| `crud-be-fsharp-giraffe`    | `CRUD_BE_FSHARP_GIRAFFE_`               |
| `crud-be-golang-gin`        | `CRUD_BE_GOLANG_GIN_`                   |
| `crud-be-java-springboot`   | `CRUD_BE_JAVA_SPRINGBOOT_`              |
| `crud-be-java-vertx`        | `CRUD_BE_JAVA_VERTX_`                   |
| `crud-be-kotlin-ktor`       | `CRUD_BE_KOTLIN_KTOR_`                  |
| `crud-be-python-fastapi`    | `CRUD_BE_PYTHON_FASTAPI_`               |
| `crud-be-rust-axum`         | `CRUD_BE_RUST_AXUM_`                    |
| `crud-be-ts-effect`         | `CRUD_BE_TS_EFFECT_`                    |
| `crud-fs-ts-nextjs`         | `CRUD_FS_TS_NEXTJS_` (server-side only) |

`DATABASE_URL`, `POSTGRES_USER`, `POSTGRES_PASSWORD`, `ENABLE_TEST_API`, and
all framework-reserved names are **not** renamed.

---

## 3. `.env.example` Annotation Format

Every variable in a committed `.env.example` file carries a one-line comment
directly above it stating three things in order: **required-or-optional**,
**type**, **format/constraint**. Examples:

```bash
# Required. String, min 32 chars. HS256 JWT signing key. Generate: openssl rand -hex 32
CRUD_BE_RUST_AXUM_JWT_SECRET=dev-jwt-secret-that-is-32-chars-long!

# Optional. Integer. TCP port the server binds to. Default: 8080.
CRUD_BE_RUST_AXUM_PORT=8080

# Optional. String. Docker Compose credential for the local PostgreSQL container.
POSTGRES_USER=crud_be_rust_axum
```

Rules:

- Every non-blank, non-comment line must have an annotation comment above it.
- All placeholder values must be obviously-dev (not real secrets).
- Commented-out vars (`# KEY=value`) are documentation; they do not require an
  annotation but must not carry real values.

---

## 4. Fail-Fast Startup Validation

Every app replaces soft-default config reads with a **language-idiomatic
fail-fast validator** that errors (naming the missing variable) at startup or
build time. Full adoption across all 11 backend families and 4 frontend apps.

| App / family                                       | Validator                                            | Mode                    |
| -------------------------------------------------- | ---------------------------------------------------- | ----------------------- |
| `crud-be-rust-axum` (Rust)                         | `dotenvy` + `envy` (serde-derived struct)            | runtime fail-fast       |
| `crud-be-golang-gin` (Go)                          | `caarlos0/env` v11, `env:"KEY,required"`             | runtime fail-fast       |
| `crud-fe-ts-nextjs`, `crud-fs-ts-nextjs` (Next.js) | `@t3-oss/env-nextjs` + `zod`                         | **build-time**          |
| `crud-be-ts-effect` (TS/Effect)                    | Effect `Config` (no `catchAll`-to-defaults)          | runtime fail-fast       |
| `crud-fe-ts-tanstack-start` (TS)                   | `zod` schema parse of `import.meta.env`              | runtime/build fail-fast |
| `crud-be-java-springboot` (Java/Spring)            | `@ConfigurationProperties` + `@Validated`            | startup fail-fast       |
| `crud-be-java-vertx` (Java/Vert.x)                 | explicit required-env check on absence               | startup fail-fast       |
| `crud-be-kotlin-ktor` (Kotlin)                     | explicit `requireNotNull`/Konform-style check        | startup fail-fast       |
| `crud-be-elixir-phoenix` (Elixir)                  | `runtime.exs` `System.fetch_env!/1`                  | startup fail-fast       |
| `crud-be-python-fastapi` (Python)                  | `pydantic-settings` `BaseSettings` (required field)  | startup fail-fast       |
| `crud-be-fsharp-giraffe` (F#)                      | explicit required-env read raising on absence        | startup fail-fast       |
| `crud-be-csharp-aspnetcore` (C#)                   | `IOptions` + `ValidateOnStart()`                     | startup fail-fast       |
| `crud-be-clojure-pedestal` (Clojure)               | explicit required-env read throwing on absence       | startup fail-fast       |
| `crud-fe-dart-flutterweb` (Dart/Flutter)           | `String.fromEnvironment` compile-time const + assert | build-time fail-fast    |

---

## 5. `rhino-cli env` Tooling

All commands are provided by `rhino-cli`.

### `env backup` / `env restore`

Back up and restore all gitignored secret files. Default directory:
`~/ose-primer-env-backup` (per-repo-derived from the repo root basename).

```bash
# Back up all secret files to the default directory
npx nx run rhino-cli:run -- env backup

# Preview without writing (dry run)
npx nx run rhino-cli:run -- env backup --dry-run

# Custom directory
npx nx run rhino-cli:run -- env backup --dir /tmp/my-backup

# Restore
npx nx run rhino-cli:run -- env restore
npx nx run rhino-cli:run -- env restore --dry-run
```

`--dry-run` prints the file set and intended actions but performs no filesystem
writes. Supported in all three output formats (`--output text|json|markdown`).

**Safety constraints**: rejects backup directories inside the repository
(prevents accidental commits of secrets); skips symlinks and files larger than
1 MB.

### `env init`

Scaffold `.env` files from `.env.example` templates under `infra/dev/<app>/`.

```bash
npx nx run rhino-cli:run -- env init
```

### `env validate`

Diff each app's **declared** keys (`infra/dev/<app>/.env.example`) against the
keys its source code **actually reads**. Exits non-zero on any drift (subject to
the per-surface allowlist). Wired into `.husky/pre-push` and the CI workflow.

```bash
npx nx run rhino-cli:run -- env validate
```

Reports two violation classes per surface:

- **declared-but-unread**: key in `.env.example` not found in source
- **read-but-undeclared**: key read in source but missing from `.env.example`

The extractor uses line-oriented regex (no parser dependency). Dynamic/computed
reads and multi-line constructs that the regex cannot resolve statically **must**
be added to the per-surface allowlist with a comment.

---

## 6. Secret-Surface Census

All secret kinds backed up by `rhino-cli env backup`. Cross-reference:
`.gitignore` lines 104–105, 108–111.

| Pattern                            | Example paths                                     | Backed up                           | Validated by `env validate` |
| ---------------------------------- | ------------------------------------------------- | ----------------------------------- | --------------------------- |
| `.env`, `.env.*`                   | `apps/<app>/.env.local`, root `.env`              | Yes                                 | App surface only            |
| `secrets.json`                     | `secrets.json` at any non-skipped path            | Yes                                 | No                          |
| `*.pem`, `*.key`, `*.crt`, `*.pfx` | `cert.pem`, `server.key`                          | Yes                                 | No                          |
| files under `.secrets/`            | `.secrets/notes.md` (freeform local secret notes) | Yes                                 | No                          |
| ~~`*.tfvars`, inventories~~        | ~~`infra/.../terraform.tfvars`~~                  | **Gated** (activate when IaC added) | **Gated**                   |

`.secrets/` is the **catch-all home for homeless secrets**: a blessed exception
to the hidden-dir skip. Every file inside it is gitignored and always backed up.
No content filter is applied inside `.secrets/` — whatever you put there gets
backed up.

### Gated IaC scaffold (R3)

`ose-primer` holds no Terraform or Ansible and never receives infra artifacts
(per its repository-ecosystem convention). The `*.tfvars`/inventory backup
patterns and the Terraform/Ansible `env validate` validators **ship commented**
with an "activate when IaC is added" note. A downstream fork that adds IaC
activates them with a config flip; no redesign required.

---

## 7. Storage-Tier Ladder

| Tier   | Storage                                          | When to use                                                             |
| ------ | ------------------------------------------------ | ----------------------------------------------------------------------- |
| Tier 0 | `.env*` (gitignored)                             | All local dev secrets — default choice                                  |
| Tier 1 | `.secrets/` (gitignored)                         | Secrets that are not env vars: PEM keys, JSON blobs, ad-hoc local notes |
| Tier 2 | `secrets.json` (gitignored)                      | Structured non-env secret store                                         |
| Tier 3 | External vault (AWS SM, GCP SM, HashiCorp Vault) | Production secrets, team-shared credentials, rotation-managed values    |

**Tier-1 trigger**: use `.secrets/` when you have a local secret that is not an
environment variable — e.g. a PEM key, a JSON credential blob, or a personal
note that must not leave the machine. Everything in `.secrets/` is gitignored
and backed up by `env backup`.

---

## 8. Principles Implemented/Respected

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**:
  Rules are stated as unambiguous prohibitions with enumerated secret types,
  explicit file scopes, and a six-layer technical enforcement model.

- **[Root Cause Orientation](../../principles/general/root-cause-orientation.md)**:
  Keeping secrets out of committed files at authoring time addresses the root
  cause; backup tooling addresses the recovery root cause; drift guard addresses
  the drift root cause.

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**:
  One rule (secrets in uncommitted files), one per-app prefix pattern, one hub
  doc.

---

## 9. Related Documentation

- [No Secrets in Committed Files Convention](./no-secrets-in-committed-files.md) — stub redirect
- [Environment File Access Convention](./env-file-access.md) — stub redirect (six-layer agent enforcement)
- [Reproducible Environments](../../development/workflow/reproducible-environments.md) — stub redirect (`.env.example` pattern, Volta, lockfiles)
- [No Machine-Specific Information in Commits](../../development/quality/no-machine-specific-commits.md) — companion rule: paths, usernames, local IPs
- [Standardize Secrets and Env Parity Decisions](../../../docs/explanation/standardize-secrets-and-env-parity-decisions.md) — plain-language rationale for all 16 decisions
