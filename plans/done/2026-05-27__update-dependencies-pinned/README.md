# Update and Pin All npm Dependencies

## Status

In Progress

## Context

The `ose-primer` repository currently declares most npm dependencies using range prefixes
(`^`, `~`). This allows `npm install` to silently resolve to newer minor or patch versions
than those tested, creating non-reproducible builds. Several packages are also behind their
latest safe version.

This plan audits every `package.json` in the repository (root + all apps + all libs) and
`.tool-versions`, upgrades packages where a safe newer version exists, and pins all declarations
to exact version strings — eliminating range ambiguity for reproducible, CVE-free installs.

**Cutoff date for eligibility**: 2026-03-27 (two months before plan creation date 2026-05-27).
Versions released after this date are excluded from target selection.

## Scope

**In-scope**:

- Root `package.json` — `devDependencies` and `dependencies`
- App-level `package.json` files:
  - `apps/crud-fe-ts-nextjs/package.json`
  - `apps/crud-fe-ts-tanstack-start/package.json`
  - `apps/crud-fs-ts-nextjs/package.json`
  - `apps/crud-be-ts-effect/package.json`
  - `apps/crud-fe-e2e/package.json`
  - `apps/crud-be-e2e/package.json`
- Lib-level `package.json` files:
  - `libs/ts-ui/package.json`
  - `libs/ts-ui-tokens/package.json`
- `.tool-versions` (erlang, elixir version pins)
- Post-update `npm install` to regenerate the lockfile

**Out-of-scope**:

- Docker base images in `infra/dev/` Dockerfiles
- Non-npm manifests (`go.mod`, `Cargo.toml`, `.csproj`, etc.)
- Node.js and npm versions in `volta` config (already exact-pinned)

## Plan Documents

- [Business Requirements (brd.md)](./brd.md) — why this work is needed
- [Product Requirements (prd.md)](./prd.md) — what gets built, acceptance criteria
- [Technical Documentation (tech-docs.md)](./tech-docs.md) — how the update is performed
- [Delivery Checklist (delivery.md)](./delivery.md) — executable phased steps

## Quick Reference: Safe Target Versions (Root package.json)

| Package                               | Current Declared | Safe Target | Action        |
| ------------------------------------- | ---------------- | ----------- | ------------- |
| `@commitlint/cli`                     | `^20.1.0`        | `20.5.0`    | Pin           |
| `@commitlint/config-conventional`     | `^20.0.0`        | `20.5.0`    | Pin           |
| `@hey-api/client-fetch`               | `^0.13.1`        | `0.13.1`    | Pin           |
| `@hey-api/openapi-ts`                 | `^0.94.2`        | `0.94.5`    | Upgrade + Pin |
| `@openapitools/openapi-generator-cli` | `^2.30.2`        | `2.31.0`    | Upgrade + Pin |
| `@redocly/cli`                        | `^2.22.1`        | `2.25.1`    | Upgrade + Pin |
| `@stoplight/spectral-cli`             | `^6.15.0`        | `6.15.0`    | Pin           |
| `eslint-plugin-jsx-a11y`              | `^6.10.2`        | `6.10.2`    | Pin           |
| `husky`                               | `^9.1.7`         | `9.1.7`     | Pin           |
| `lint-staged`                         | `^16.2.6`        | `16.4.0`    | Upgrade + Pin |
| `markdownlint-cli2`                   | `^0.21.0`        | `0.22.0`    | Upgrade + Pin |
| `nx`                                  | `22.5.2`         | `22.6.2`    | Upgrade       |
| `prettier`                            | `^3.6.2`         | `3.8.1`     | Pin           |
| `prettier-plugin-tailwindcss`         | `^0.7.2`         | `0.7.2`     | Pin           |
| `tailwindcss` (dep)                   | `^4.2.1`         | `4.2.2`     | Pin           |
| `tsx`                                 | `^4.20.6`        | `4.21.0`    | Pin           |

App-level packages: executor determines exact pins during Phase 2 using `npm outdated` + cutoff date check.

## Approach Summary

1. Update and pin root `package.json` using the pre-researched safe target table.
2. Update and pin each app/lib `package.json` — run `npm outdated` per workspace, apply
   cutoff-date filter, upgrade where eligible, pin all to exact versions.
3. Update `.tool-versions` after verifying erlang and elixir release dates and security.
4. Run `npm install` from repo root to regenerate `package-lock.json`.
5. Run `npm audit` to confirm zero vulnerabilities.
6. Run affected quality gates (typecheck, lint, test:quick) and fix any regressions.
7. Push to `main`.
