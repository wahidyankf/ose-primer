# Standardize Secrets and Env — Parity Decisions

Plain-language explanation of every decision in the cross-repo deviation matrix
for the "Standardize Secrets and Environment-Variable Storage" parity effort.
Follows the `*-parity-decisions.md` precedents in this directory.

`ose-infra` is the **canonical reference** for this parity set. Every divergence
below is intentional and recorded; zero silent deviations.

---

## R1 — Parity set

`ose-primer` is authored alongside `ose-public`; `ose-infra` is the reference
because its plan already gated and passed before primer's plan started. This
avoids any risk of primer regressing an already-green reference.

---

## R2 — Delivery mode

`ose-primer`'s plan was delivered `worktree-to-main`: a git worktree was created,
all work committed inside it, and then pushed directly to `origin main` via
`git push origin HEAD:main`. This is ordinary Trunk-Based Development for a
maintainer-driven operation.

---

## R3 — IaC surfaces (N/A for ose-primer)

`ose-primer` holds no Terraform or Ansible and, per its repository-ecosystem
convention, **never receives infra artifacts**. The Terraform/Ansible drift-guard
validators and `*.tfvars`/inventory backup patterns are therefore **N/A** here —
unlike `ose-infra` (where they are real) and `ose-public` (where they ship as a
commented forward-scaffold).

Primer ships only inert commented scaffold patterns in the backup allowlist and
the `env validate` contract so a downstream fork can activate the discipline with
a config flip if it ever adds IaC. No standing-up of inert tooling.

---

## R4 — Research

Research ran before implementation. Findings covered: Go/Effect long-tail
validator patterns, Dart build-time validation idioms, the 12-factor precise
framing (authorizes but does not prescribe per-app prefixes), and framework-exempt
variable names. Findings are recorded in
`plans/in-progress/standardize-secrets-and-env/tech-docs.md §9`.

---

## R5 — PR override

`ose-primer`'s governance sync invariant normally requires changes received from
upstream (`ose-public`) to arrive via a **draft PR**, not a direct push to `main`.
This parity run **explicitly overrode** that invariant, with the invoker's
authorization, to deliver all three sibling plans (`ose-infra`, `ose-primer`,
`ose-public`) as a synchronized cross-repo parity landing treated as one
maintainer-driven operation. No `--force` was used; the push was a normal fast-
forward. The corresponding update to `ose-primer`'s `repository-ecosystem`
convention (to reflect this authorized exception) is a **separate downstream
follow-up**, not part of this plan.

---

## R6 — rhino-cli tooling

`ose-primer`'s rhino-cli has a single implementation:
**`apps/rhino-cli/` is the sole CLI** (all `package.json` scripts and Husky
hooks invoke it). The spec-first flow implemented the CLI in Rust and validated it
against the behavior contract.

---

## R7 — Startup validation (full adoption)

All 11 backend families and all 4 frontend apps received a language-idiomatic
fail-fast validator. This is **full adoption** — every app in the monorepo, not
just the two referenced in `ose-infra`. Chosen by the invoker for maximal
convergence.

---

## R8 — Polyglot reach

A validator-per-language table covering all 14 apps (11 backends + 4 frontends)
was delivered, with one delivery sub-phase per language family. This matches the
"full adoption" decision in R7 and honors it literally.

---

## R9 — Naming prefix (full rename)

All app-defined env var names were renamed to per-app prefix form across all
existing surfaces. This is maximal convergence — chosen by the invoker to make
every app's variable ownership unambiguous in shared environments.

---

## R10 — Hub doc

A new hub convention document was created at
`repo-governance/conventions/security/secrets-and-env-standards.md`. The three
existing documents (`no-secrets-in-committed-files.md`, `env-file-access.md`,
`reproducible-environments.md`) were reduced to stub redirects pointing at the
hub. This gives one authoritative source for all secrets and env-var rules and
matches the `ose-infra` canonical doc layout.

---

## R10b — Doc canonicalization

The two security docs were moved from `repo-governance/development/quality/` to
`repo-governance/conventions/security/` to align `ose-primer`'s governance paths
with the `ose-infra` canonical layout. All inbound links were rewritten to the
new paths.

`ose-primer`'s `repository-ecosystem` convention currently records these docs
under `development/quality/`. The update to that convention is an **authorized
downstream follow-up** — not part of this plan. This deferred item was explicitly
recorded to prevent it from becoming a silent divergence.

---

## R11 — Backup allowlist (real floor + IaC gated scaffold)

The `rhino-cli env backup`/`restore` widened secret discovery from `.env*`-only
to a full set: `.env*` (existing), `secrets.json`, `*.pem`/`*.key`/`*.crt`/`*.pfx`,
and any file under `.secrets/`. The IaC patterns (`*.tfvars`, inventories) ship
commented/gated per R3.

`.secrets/` is the blessed catch-all for secrets that are not env vars: a
single exception to the hidden-dir skip, so every file inside it is always
backed up.

---

## R11b — Backup default directory

The CLI previously hardcoded `ose-open-env-backup` as the backup directory
name, so all sibling repos shared one backup folder (`~/ose-open-env-backup`).
This plan adopted the **canonical per-repo-derived** default from `ose-infra`:
`~/<repo-root-basename>-env-backup`. For `ose-primer`, this resolves to
`~/ose-primer-env-backup`.

The change was applied in `rhino-cli`, the sole CLI implementation.

---

## R12 — Layout (no migration)

`ose-primer` is a **template repository**. It deliberately keeps its existing
layout: a root `.env.example` template plus per-app `infra/dev/<app>/.env.example`
files. `rhino-cli env init` continues to walk `infra/dev/`. No migration to
`apps/<app>/` was performed.

This is the principal layout divergence from `ose-infra` (which migrated) and
`ose-public` (which consolidated). Primer preserves its template-repo layout and
Nx scaffold path. The env-loading rationale (why colocation matters for
auto-loading) is documented in `tech-docs.md §3` for downstream forks that might
choose to migrate.

---

## R13 — Rationale doc

This document. Created at
`docs/explanation/standardize-secrets-and-env-parity-decisions.md`, matching the
`*-parity-decisions.md` naming convention used by prior plans in this directory.

---

## R14 — Live `APP_PORT` drift fix (dropped)

The `ose-infra` plan fixed a live `APP_PORT` drift bug. No equivalent drift exists
in `ose-primer` — the `APP_*` renaming was purely a naming-standard enforcement,
not a fix for a live environment mismatch. This decision was dropped as N/A.

---

## Summary

| #    | Decision              | Outcome                                                                                             |
| ---- | --------------------- | --------------------------------------------------------------------------------------------------- |
| R1   | Parity set            | `ose-infra` as canonical reference                                                                  |
| R2   | Delivery mode         | `worktree-to-main` (ordinary TBD for maintainer op)                                                 |
| R3   | IaC surfaces          | N/A — gated scaffold only, no real validators                                                       |
| R4   | Research              | Ran — findings in tech-docs.md §9                                                                   |
| R5   | PR override           | Accepted, invoker-owned, one-off; ecosystem-convention update deferred                              |
| R6   | rhino-cli direction   | Single Rust implementation (`apps/rhino-cli`)                                                       |
| R7   | Startup validation    | Full adoption — all 11 backends + 4 frontends                                                       |
| R8   | Polyglot reach        | All 14 apps, one sub-phase per family                                                               |
| R9   | Naming prefix         | Full per-app prefix rename across all app-defined vars                                              |
| R10  | Hub doc               | `conventions/security/secrets-and-env-standards.md`; three stubs                                    |
| R10b | Doc canonicalization  | Moved 2 docs `development/quality/` → `conventions/security/`; ecosystem-convention update deferred |
| R11  | Backup allowlist      | `.env*` + `secrets.json` + `*.pem/key/crt/pfx` + `.secrets/` + IaC gated                            |
| R11b | Backup default dir    | Per-repo-derived `~/ose-primer-env-backup` in both twins                                            |
| R12  | Layout                | No migration; `infra/dev/<app>/` kept (template-repo layout)                                        |
| R13  | Rationale doc         | This file                                                                                           |
| R14  | Live `APP_PORT` drift | Dropped (no such drift in primer)                                                                   |
