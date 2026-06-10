# PRD — Standardize Secrets and Environment-Variable Storage (ose-primer)

## Product Overview

A repository-wide standard for how secrets and environment variables are **named, documented,
validated, and kept in sync** across the `ose-primer` polyglot template — delivered as one hub
convention plus the concrete code, config, spec, and tooling changes that make the standard real and
self-enforcing across every app and both rhino-cli implementations.

The product has four user-facing surfaces:

1. **The hub convention** — the single document a contributor (or fork) reads to learn the rules.
2. **The naming standard** — applied across all ~15 apps so the template demonstrates its own rules.
3. **Fail-fast startup validators** — every app aborts on misconfiguration instead of silently
   defaulting.
4. **The `rhino-cli env` family** — widened `backup`/`restore` (every secret kind, `--dry-run`) and a
   new `env validate` drift guard, authored spec-first and kept byte-identical across Rust (canonical)
   and Go (twin).

## Personas

Hats the maintainer wears and agents that consume the outputs (not external stakeholders):

- **Contributor (human or agent)** adding or changing an env var — needs one place to learn the rule
  and immediate feedback when they get it wrong.
- **Fork adopter** copying `ose-primer` — needs the standard, the validators, and the hub doc to come
  along intact so the fork inherits a drift-proof config posture.
- **Reviewer** (the maintainer at push time, plus `ci-checker`) — needs the guard to catch drift and
  the parity harness to catch implementation divergence automatically.
- **Operator** running an app locally — needs a clear, named error when a required value is missing,
  not a silent fallback to a wrong default.

## User Stories

1. **As a contributor**, I want one authoritative document for secrets/env rules, so that I do not
   have to reassemble the policy from three separate files.
2. **As a contributor**, I want a per-app naming standard with documented framework exemptions, so
   that I know whether a new variable should be prefixed.
3. **As an operator**, I want every app to abort at startup (or build) with a named-variable error
   when a required value is missing, so that I never debug a silent wrong-default.
4. **As a reviewer**, I want code↔config drift to fail the pre-push hook and CI, so that an
   `APP_PORT`-class mismatch can never merge.
5. **As an operator**, I want `rhino-cli env backup` and `env restore` to support `--dry-run`, so that
   I can preview exactly which files would be touched before committing to the operation.
6. **As an operator**, I want one `rhino-cli env backup` to capture every secret kind in the repo —
   `.env*`, `secrets.json`, `*.pem`/`*.key`/`*.crt`/`*.pfx`, and the `.secrets/` directory — so that
   recovering the machine does not silently leave any secret behind.
7. **As a contributor**, I want each `.env.example` variable annotated with its required/optional
   status, type, and format, so that I can populate `.env` correctly without reading code.
8. **As a maintainer**, I want every rhino-cli change to land in both the Rust and Go implementations
   and pass the shadow-diff parity gate, so that the template's dual-implementation promise holds.
9. **As a fork adopter**, I want the hub doc to ship the IaC backup/validate patterns as a gated
   scaffold, so that when my fork adds Terraform/Ansible the discipline is one config-flip away rather
   than a redesign.

## Acceptance Criteria (Gherkin)

Every scenario uses exactly one primary `Given`/`When`/`Then`, with extras chained via `And`.

### AC-01 — Single hub convention exists at the canonical path and the prior docs redirect

```gherkin
Scenario: Contributor finds one authoritative secrets/env document
  Given the repository governance under repo-governance/conventions/security/
  When a contributor opens secrets-and-env-standards.md
  Then it documents naming, annotation, per-language validation, the secret-surface census, and the storage-tier ladder
  And no-secrets-in-committed-files.md and env-file-access.md have moved here from development/quality/ and each contain a stub pointing to it
  And reproducible-environments.md under development/workflow/ contains a stub pointing to it
  And the repository markdown link check reports zero broken links
```

### AC-02 — Per-app naming standard with framework exemptions

```gherkin
Scenario: App-defined variable carries the per-app prefix
  Given the naming standard in the hub convention
  When the crud-be-rust-axum backend declares its JWT secret and port variables
  Then they are named CRUD_BE_RUST_AXUM_JWT_SECRET and CRUD_BE_RUST_AXUM_PORT
  And the framework-reserved NEXT_PUBLIC_* names keep their framework form
  And the shared DATABASE_URL and POSTGRES_* names remain unprefixed
```

### AC-03 — Per-app rename applied across code, example, and compose

```gherkin
Scenario: All sources for an app agree on the renamed key
  Given a backend whose APP_JWT_SECRET has been renamed to its per-app-prefixed name
  When a contributor greps that app for the old name across code, .env.example, and docker-compose
  Then grep -rn "APP_JWT_SECRET" apps/<app> infra/dev/<app> returns zero hits
  And the per-app-prefixed name appears in the config code, the .env.example, and every compose env block
```

### AC-04 — Fail-fast validation in a backend (named-variable error)

```gherkin
Scenario: Backend aborts when a required variable is unset
  Given a backend loading config through its language-idiomatic fail-fast validator
  When the app starts with a required variable unset and no default permitted
  Then the process exits non-zero
  And the error names the missing variable
```

### AC-05 — Effect backend no longer swallows config errors

```gherkin
Scenario: crud-be-ts-effect surfaces a missing required value
  Given apps/crud-be-ts-effect/src/config.ts loading config through Effect Config without a catchAll-to-defaults handler
  When the app starts with a required variable unset
  Then the Effect program fails with a ConfigError naming the variable
  And grep -n "catchAll" apps/crud-be-ts-effect/src/config.ts shows no default-swallowing handler
```

### AC-06 — Frontend env validated at build time

```gherkin
Scenario: Next.js frontend build fails when a required public variable is missing
  Given a Next.js frontend validating env through @t3-oss/env-nextjs and zod in src/env.ts
  When the build runs with a required NEXT_PUBLIC_* variable absent and no default
  Then the typecheck or build step fails
  And the failure names the missing variable
```

### AC-07 — Backup/restore dry-run previews without writing

```gherkin
Scenario: Dry-run reports the file set without side effects
  Given the rhino-cli env backup and env restore commands
  When the operator runs either with --dry-run
  Then the command prints exactly which files would be backed up or restored
  And no file is written, copied, or overwritten on disk
```

### AC-08 — Backup captures every secret kind, not just `.env*`

```gherkin
Scenario: One backup run includes secrets.json, certs, and the .secrets directory
  Given gitignored secret files of multiple kinds (.env.local, secrets.json, a throwaway.pem, and a .secrets/ note)
  When the operator runs rhino-cli env backup
  Then the backup archive contains the .env, the secrets.json, the .pem, and the .secrets/ note
  And no secret-bearing file kind is silently skipped
```

### AC-09 — Drift guard catches code↔config mismatch

```gherkin
Scenario: rhino-cli env validate fails on a deliberate mismatch
  Given rhino-cli env validate comparing each app's declared .env.example keys against the env vars its code reads
  When a contributor renames a key in code but not in the app's .env.example
  Then rhino-cli env validate exits non-zero and names the divergent key
  And the pre-push hook and the CI workflow both invoke the same command
```

### AC-10 — Dual-implementation parity holds for every rhino-cli change

```gherkin
Scenario: Rust and Go implementations stay byte-identical
  Given the widened backup/restore, the --dry-run flag, and the new env validate subcommand
  When the shadow-diff parity harness runs against both built binaries
  Then stdout, stderr, and exit codes match byte-for-byte across all output formats
  And both implementations pass their own spec-coverage target
```

### AC-10b — Backup default directory is per-repo-derived in both twins

```gherkin
Scenario: env backup defaults to a per-repo-derived directory in both implementations
  Given rhino-cli-rust and rhino-cli-go both ship the per-repo-derived backup default dir
  When the operator runs env backup with no --dir flag in either binary
  Then the default backup directory is ~/ose-primer-env-backup (derived from the repo-root basename)
  And it is no longer the hardcoded ~/ose-open-env-backup
  And the shadow-diff parity harness reports the default path and help text byte-identical across the twins
```

### AC-11 — Annotated env example

```gherkin
Scenario: Each variable documents its contract
  Given the annotation format in the hub convention
  When a contributor reads a renamed infra/dev/<app>/.env.example
  Then each variable has a comment stating required-or-optional, type, and format
  And the placeholder values are obviously dev-only
```

### AC-12 — New dependencies follow the bump policy

```gherkin
Scenario: Each new dependency is pinned and cleared
  Given the new validator dependencies across the polyglot apps and the frontends
  When they are added to their respective manifests
  Then each version is an exact pin with no caret, tilde, or range operator
  And each is classified Path B and CVE-cleared in the tech-docs Security Clearance table
```

### AC-13 — IaC validators and backup patterns ship gated, not active

```gherkin
Scenario: Terraform and Ansible scaffolding is present but inert
  Given primer has no Terraform or Ansible today
  When a contributor reads the hub doc and the backup floor patterns
  Then the *.tfvars and inventory patterns appear inside a commented "activate when IaC is added" block
  And rhino-cli env validate runs no IaC surface against primer
```

### AC-14 — PR-override deviation is explicitly recorded

```gherkin
Scenario: The direct-to-main deviation is documented in plain language
  Given the normal sibling-sync rule that ose-primer receives governance changes via a draft PR
  When a reader opens tech-docs.md row R5 and the Phase 7 rationale doc
  Then both state that this plan pushes directly to ose-primer main, why, and that the invoker owns the one-off exception
  And no push in the plan uses a force flag
```

## Product Scope

### In Scope (Product)

- Hub convention document at `repo-governance/conventions/security/` plus the three stub redirects;
  the two security docs are **moved** from `development/quality/` → `conventions/security/` (R10b).
- Per-app naming standard applied to all backends (rename) and documented for all apps.
- Fail-fast startup validation in every backend and frontend (new deps under the Dependency Bump
  Policy).
- `rhino-cli env backup`/`restore` widened from `.env*`-only to a secret allowlist (`.env*`,
  `secrets.json`, `*.pem`/`*.key`/`*.crt`/`*.pfx`, and the `.secrets/` directory via a hidden-dir-skip
  exception), plus `--dry-run`, plus a **per-repo-derived backup default dir**
  (`~/<repo-basename>-env-backup`, R11b) — all authored spec-first, Rust-canonical → Go-twin,
  shadow-diff-gated in **both** twins.
- `rhino-cli env validate` app drift guard (spec-first dual-impl) + pre-push + CI wiring.
- Annotated `infra/dev/<app>/.env.example` files.
- Gated IaC scaffold (commented backup patterns + documented-but-inert Terraform/Ansible validators).

### Out of Scope (Product)

- Encrypted-at-rest secret storage adoption (documented as a future tier only).
- Migrating the `infra/dev/<app>/` env-template layout (primer keeps it).
- Any live IaC validation.
- Any UI; this is governance + CLI + per-app config code.

## Product-Level Risks

- **Validation friction**: overly strict startup validation could block legitimate local dev.
  Mitigation: permit documented defaults for non-secret dev values; only secrets and structural values
  are required-no-default.
- **Guard false positives**: the drift guard's code-read detection could miss a dynamic env read.
  Mitigation: the guard parses a declared allowlist per app and reports unknown/missing keys
  explicitly; dynamic reads are documented as unsupported and flagged for manual annotation.
- **Parity friction**: a change that is easy in one language may be awkward in the other. Mitigation:
  the spec-first flow forces the contract before either implementation, and the shadow-diff gate is
  the arbiter; accepted divergences require an explicit convention entry.
