# Product Requirements Document — Update and Pin All npm Dependencies

## Product Overview

A one-time dependency maintenance pass that updates and pins all npm `package.json` declarations
in `ose-primer` to exact version strings. Every pinned version must satisfy three eligibility
criteria: released more than two months before 2026-05-27 (cutoff 2026-03-27), free of known
CVEs at high or critical severity, and at or near the latest stable release within those
constraints.

The deliverable is a set of edited `package.json` files, a regenerated `package-lock.json`,
and a clean `npm audit` result — all committed to `main`.

## Personas

- **Infrastructure maintainer** (solo maintainer): executes the update pass, validates quality
  gates, and merges to `main`.
- **Automated agents** (`repo-setup-manager`, `swe-typescript-dev`): execute delivery phases
  and validate results per this PRD.

## User Stories

### US-1 — Reproducible installs

As an infrastructure maintainer,
I want every `package.json` dependency to declare an exact version (no `^` or `~`),
so that `npm install` produces the same resolution on every machine and every CI run.

### US-2 — Current and safe versions

As an infrastructure maintainer,
I want each pinned version to be released more than two months ago and free of CVEs,
so that I am not pinning to a version that is bleeding-edge or carries known security issues.

### US-3 — Passing quality gates after update

As an infrastructure maintainer,
I want all typecheck, lint, and test targets to pass after the dependency changes,
so that the update does not introduce regressions.

### US-4 — Clean audit result

As an infrastructure maintainer,
I want `npm audit --audit-level=high` to exit 0 after all pins are applied,
so that there are no unresolved high or critical vulnerabilities in the dependency tree.

### US-5 — Template best-practice signal

As a downstream fork author,
I want the template to demonstrate exact-pinned dependencies,
so that my fork starts from a reproducible, secure baseline.

## Acceptance Criteria

### AC-1: No range prefixes remain in any package.json

```gherkin
Scenario: All package.json files use exact version pins
  Given all package.json files in the repository have been updated
  When I run: grep -rn '"[\^~]' apps/ libs/ package.json
  Then the command produces no output (exit 0 with empty stdout)
```

### AC-2: Root package.json pins match the safe target table

```gherkin
Scenario: Root devDependency nx is pinned to 22.6.2
  Given the root package.json has been updated
  When I inspect the "nx" entry under devDependencies
  Then its value is exactly "22.6.2" (no prefix)

Scenario: Root devDependency lint-staged is pinned to 16.4.0
  Given the root package.json has been updated
  When I inspect the "lint-staged" entry under devDependencies
  Then its value is exactly "16.4.0" (no prefix)

Scenario: Root devDependency markdownlint-cli2 is pinned to 0.22.0
  Given the root package.json has been updated
  When I inspect the "markdownlint-cli2" entry under devDependencies
  Then its value is exactly "0.22.0" (no prefix)

Scenario: Root dependency tailwindcss is pinned to 4.2.2
  Given the root package.json has been updated
  When I inspect the "tailwindcss" entry under dependencies
  Then its value is exactly "4.2.2" (no prefix)
```

### AC-3: npm install succeeds with zero errors

```gherkin
Scenario: npm install completes cleanly after all package.json edits
  Given all package.json files have been updated and saved
  When I run: npm install (from the repository root)
  Then the command exits 0
  And package-lock.json is updated
  And no "ERESOLVE" or peer-dependency conflict errors appear in stdout or stderr
```

### AC-4: npm audit finds no high or critical vulnerabilities

```gherkin
Scenario: Dependency tree is free of high and critical CVEs
  Given npm install has completed successfully
  When I run: npm audit --audit-level=high
  Then the command exits 0
  And the output contains "found 0 vulnerabilities" or equivalent zero-findings message
```

### AC-5: All affected typecheck targets pass

```gherkin
Scenario: TypeScript compilation succeeds after dependency update
  Given all package.json files have been pinned and npm install has run
  When I run: npx nx affected -t typecheck
  Then all affected projects exit 0 with no type errors
```

### AC-6: All affected lint targets pass

```gherkin
Scenario: Linting passes after dependency update
  Given all package.json files have been pinned and npm install has run
  When I run: npx nx affected -t lint
  Then all affected projects exit 0 with no lint errors
```

### AC-7: All affected quick test targets pass

```gherkin
Scenario: Unit and quick tests pass after dependency update
  Given all package.json files have been pinned and npm install has run
  When I run: npx nx affected -t test:quick
  Then all affected projects exit 0 with no test failures
```

### AC-8: CI passes on main after push

```gherkin
Scenario: All GitHub Actions workflows pass after push to main
  Given all local quality gates have passed
  When the changes are pushed to origin main
  Then all GitHub Actions workflow runs triggered by the push complete with status "success"
  And no workflow run has status "failure" or "cancelled" due to the dependency changes
```

### AC-9: .tool-versions pins are verified

```gherkin
Scenario: .tool-versions entries are verified against release date and security
  Given .tool-versions contains erlang and elixir version declarations
  When the executor checks release dates and security advisories for each version
  Then each declared version has a release date on or before 2026-03-27
  And each declared version has no known high or critical CVEs
  And the file is updated if a more recent eligible version exists
```

## Product Scope

### In-Scope Features

- Exact-pinning of all `^`/`~` prefixed declarations in root `package.json`
- Upgrade + pin of root packages where a newer eligible version exists (see
  `README.md` Quick Reference table)
- Exact-pinning of all app-level and lib-level `package.json` files (8 files)
- Verification and update of `.tool-versions` (erlang, elixir)
- Lockfile regeneration via `npm install`
- `npm audit` clean result
- Full local quality gate pass before push
- CI green on `main` after push

### Out-of-Scope Features

- Docker base image pinning
- Non-npm manifest updates (`go.mod`, `Cargo.toml`, `.csproj`, etc.)
- Automated dependency refresh tooling (Renovate, Dependabot)
- Downgrading any package
- Upgrading Node.js or npm versions (already exact-pinned via Volta)

## Product-Level Risks

| Risk                                                                    | Impact                                | Mitigation                                                                                       |
| ----------------------------------------------------------------------- | ------------------------------------- | ------------------------------------------------------------------------------------------------ |
| A minor version bump changes a public API surface used in the codebase  | Medium — typecheck or test failures   | AC-5 and AC-6 catch this; fix before push                                                        |
| `npm install` produces ERESOLVE peer-dependency conflicts after pinning | Medium — blocks lockfile regeneration | Resolve by adjusting peer dependency ranges or accepting `--legacy-peer-deps` with documentation |
| An app-level package has no release within the cutoff window            | Low — pin to current resolved version | Document in delivery notes; pin the resolved version as-is                                       |
