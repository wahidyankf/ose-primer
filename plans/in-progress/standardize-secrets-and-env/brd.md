# BRD — Standardize Secrets and Environment-Variable Storage (ose-primer)

## Business Goal

Make environment configuration **predictable, self-validating, and drift-proof** across every app in
the `ose-primer` polyglot template, so that adding or changing a configuration value is a mechanical,
mistake-resistant operation rather than a source of silent runtime bugs. Consolidate scattered
governance into one authoritative reference so a contributor (human or agent) — and every downstream
fork that adopts this template — has exactly one place to learn the rules.

## Why Now

- **`ose-primer` is the template every downstream fork copies.** A naming + validation standard set
  here propagates outward by design (this is the sync hub). Standardizing the template means every
  fork inherits a drift-proof, fail-fast config posture for free, rather than re-deriving it.
- **Soft-default config is a live, repo-wide latent bug.** Every backend falls back to a hardcoded
  default on a missing value instead of failing fast — `crud-be-ts-effect` even swallows the entire
  config read in `Effect.catchAll(() => Effect.succeed({ … }))` [Repo-grounded]. This is the exact
  class of silent-default error the sibling `ose-infra` plan exists to kill; primer has it in eleven
  languages at once.
- **The three governance docs have no hub.** Rules live in three separate documents
  ([no-secrets-in-committed-files](../../../repo-governance/development/quality/no-secrets-in-committed-files.md),
  [env-file-access](../../../repo-governance/development/quality/env-file-access.md),
  [reproducible-environments](../../../repo-governance/development/workflow/reproducible-environments.md))
  [Repo-grounded] forcing readers (and forks) to assemble the full picture themselves.
- **Cross-repo convergence is in flight now.** The sibling `ose-infra` and `ose-public` plans are
  being authored in lockstep; doing primer at the same time keeps the three repos converging on one
  end-state instead of drifting apart.

## Business Impact

| Pain point today                                                                                       | After this plan                                                                             |
| ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------- |
| Bare `APP_*` / `JWT_SECRET` names give no hint which app owns a variable                               | One per-app-prefix standard (`CRUD_BE_RUST_AXUM_*`, …) with documented framework exemptions |
| Missing/mistyped env value defaults silently at runtime in every backend                               | A language-idiomatic fail-fast validator aborts startup with a named-variable error         |
| `crud-be-ts-effect` swallows config errors via `Effect.catchAll(succeed(defaults))`                    | The Effect `Config` read fails fast; no `catchAll`-to-defaults swallow                      |
| `env backup` skips non-`.env` secrets (`secrets.json`, `*.pem/key/crt/pfx`, the whole `.secrets/` dir) | One `env backup` captures every secret kind via an explicit allowlist + `.secrets/` descent |
| No preview before a backup or restore writes to disk                                                   | `--dry-run` previews the exact file set on both subcommands with zero writes                |
| No code↔config drift guard; an `APP_PORT`-class mismatch could ship in any app                         | `rhino-cli env validate` fails pre-push/CI on any code↔config key mismatch                  |
| `.env.example` files give no type/required hints                                                       | Every variable annotated (required/optional, type, format)                                  |
| Three docs, no hub; rules must be reassembled by the reader and by every fork                          | One hub convention; the three become stub redirects; forks inherit it intact                |

## Affected Roles

This repository has one maintainer collaborating with AI agents; the "roles" below are hats the
maintainer wears and agents that consume the outputs — not sign-off gates.

- **Template maintainer** — defines and applies the standard; benefits from the drift guard catching
  config mistakes before they ship, and from forks inheriting the standard automatically.
- **Downstream fork adopter** — copies `ose-primer`; gains a drift-proof, fail-fast config posture and
  one hub doc out of the box.
- **`swe-rust-dev` / `swe-golang-dev` / `swe-typescript-dev` and the polyglot dev agents** — consume
  the naming standard and per-language validation patterns when authoring app config code.
- **`repo-rules-checker` / `ci-checker` agents** — gain a single authoritative doc to validate
  against instead of three.

## Success Criteria

All criteria are **observable facts** verifiable on demand — no fabricated metrics.

1. **Per-app naming applied everywhere**: for every backend, `grep -rn "APP_JWT_SECRET\|APP_PORT" apps/<app>/`
   returns zero hits and the per-app-prefixed names are present in code, the `infra/dev/<app>/.env.example`,
   and the `docker-compose*.yml` env blocks. Verify: run the grep per app; confirm the prefixed names
   resolve at startup.
2. **Fail-fast validation active in every app**: starting each app with a required var unset aborts
   with a non-zero exit and a named-variable error (or, for build-time-validated frontends, the build
   fails naming the variable). Verify: per app, unset one required var and confirm the documented
   failure mode.
3. **`crud-be-ts-effect` no longer swallows config errors**: `apps/crud-be-ts-effect/src/config.ts`
   no longer wraps the read in `Effect.catchAll(() => Effect.succeed(defaults))`; a missing required
   var surfaces as a `ConfigError`. Verify: `grep -n "catchAll" apps/crud-be-ts-effect/src/config.ts`
   returns no default-swallowing handler.
4. **Backup captures every secret kind**: `rhino-cli env backup --dry-run` lists `.env*`,
   `secrets.json`, `*.pem`/`*.key`/`*.crt`/`*.pfx`, and every file under `.secrets/`. Verify: place a
   throwaway `secrets.json`, a `throwaway.pem`, and a `.secrets/throwaway.md`, and confirm all three
   appear in the `--dry-run` list; confirm the dry-run writes nothing.
5. **Dual-implementation parity preserved**: every rhino-cli change lands in both `rhino-cli-rust`
   (canonical) and `rhino-cli-go` (twin) and the shadow-diff parity harness exits 0. Verify: run
   `apps/rhino-cli-rust/scripts/shadow-diff.sh` (or the `parity` job) — byte-identical.
6. **Drift guard wired into gates**: the pre-push hook and a CI workflow both invoke
   `rhino-cli env validate`; a forced key mismatch in any app fails both. Verify: seed a mismatch,
   confirm non-zero exit naming the key, then revert.
7. **Single hub doc exists**: `repo-governance/development/quality/secrets-and-env-standards.md`
   exists and the three prior docs are stubs pointing to it; no inbound link is broken (the repo's
   markdown link check passes).
8. **IaC scaffold present but gated**: the hub doc and the backup floor carry the `*.tfvars` /
   inventory patterns **commented** with an "activate when IaC is added" note, and no IaC validator
   runs against primer. Verify: `grep -n "tfvars" repo-governance/development/quality/secrets-and-env-standards.md`
   shows them inside a commented/"future" block.
9. **PR-override deviation recorded**: `tech-docs.md §1` row R5 and the Phase 7 rationale doc both
   state, in plain language, that this plan pushes directly to `ose-primer` `main` despite the normal
   PR-only sibling-sync rule, why, and who owns it.
10. **Dependency policy satisfied**: every new dependency (the per-language validators, `caarlos0/env`,
    `envy`, `dotenvy`, `@t3-oss/env-nextjs`, `zod`, and the long-tail libs) is pinned exactly,
    classified Path B, and CVE-cleared per the
    [Dependency Bump Policy](../../../repo-governance/development/workflow/dependency-bump-policy.md);
    `tech-docs.md §7` carries the Security Clearance table.

## Non-Goals (Business Scope)

- Introducing any encrypted-at-rest secret store (SOPS/age, Vault, Infisical, Doppler) in this plan.
- Migrating the `infra/dev/<app>/` env-template layout (primer keeps it deliberately — it is a
  template repo).
- Standing up real IaC (Terraform/Ansible) — the IaC scaffold ships gated, not active.
- Re-architecting any app's runtime beyond the env-validation boundary.

## Business Risks and Mitigations

| Risk                                                                             | Likelihood | Mitigation                                                                                                                                                            |
| -------------------------------------------------------------------------------- | ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Env-var rename breaks a developer's local `.env` (gitignored, not migrated)      | Medium     | Rename-only of the committed `.env.example` + compose blocks; the hub doc + Phase 1/2 notes instruct re-copying from the renamed `.env.example`; grep-to-zero per app |
| PR-override push to `ose-primer` `main` bypasses the normal sibling-sync PR gate | Medium     | Explicit invoker-owned, one-off deviation (R5); recorded in `tech-docs.md §1` + the rationale doc; no `--force`; per-phase pushes are reviewable in history           |
| A rhino-cli change lands in one implementation but not the other (parity break)  | Medium     | Spec-first flow + the permanent shadow-diff `parity` CI job; both-land-together is a hard convention rule; each phase gate runs the parity harness                    |
| A long-tail language's idiomatic fail-fast validator is misidentified            | Medium     | The exact lib per long-tail family is selected at execution under the Dependency Bump Policy; the matrix names the idiomatic candidate but defers the pin             |
| `t3-env` conflicts with an app's existing config style in a Next.js frontend     | Low        | Scope `t3-env` to the env boundary only (`src/env.ts`); downstream code consumes the validated object                                                                 |
| New `rhino-cli env validate` lowers crate/Go coverage below the gate             | Low        | The subcommand ships with unit + integration tests in both implementations; `test:quick` + `spec-coverage` gates enforce coverage                                     |
| Folding three docs loses content or breaks inbound links                         | Medium     | Stub-redirect approach keeps old paths resolvable; `done/` plan links untouched; the markdown link check gates each commit                                            |
| A new dependency ships a regression or unpatched CVE                             | Low        | Dependency Bump Policy Path B (60-day soak + CVE-clean), exact pins, Security Clearance table verified at execution                                                   |
| IaC scaffold accidentally activates against a repo with no IaC                   | Low        | The Terraform/Ansible validators ship gated/commented with an explicit "activate when IaC is added" trigger; no surface is configured for them in primer              |
