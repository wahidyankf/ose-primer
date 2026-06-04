# Product Requirements Document — Dependency Bump June 2026

## Product overview

A repository-wide dependency-bump deliverable that, when executed, pins every in-scope manifest to
its clearance-approved exact version, regenerates lockfiles, re-audits clean, and records every
security trade-off. The "product" is the resulting clean, deterministic, exactly-pinned template
state plus an updated waiver register.

## Personas

Solo-maintainer repository; personas are the hats the maintainer wears and the agents that consume
this plan:

- **Security reviewer** — decides and records waivers/holds; reads `tech-docs.md` clearance tables.
- **Polyglot maintainer** — applies per-ecosystem edits; consumed by the `swe-*-dev` agent family.
- **Release engineer** — owns deterministic Docker/CI pins.
- **`repo-setup-manager`** — executes Phase 0 baseline.
- **Human operator** — performs the two `[HUMAN]` items (Flutter image replacement, `flutter upgrade`).

## User stories

- **As a security reviewer**, I want every in-scope CVE resolved or explicitly waived, so that the
  template does not propagate known vulnerabilities to downstream forks.
- **As a polyglot maintainer**, I want every dependency pinned exact (no caret/tilde/floating), so
  that builds are reproducible across CI runs and forks.
- **As a release engineer**, I want Docker base images and GitHub Actions pinned to exact digests
  or versions, so that the build pipeline is deterministic.
- **As a security reviewer**, I want every WAIVER and FUNCTIONAL-HOLD recorded in the long-lived
  register with KEV + EPSS columns, so the trade-off is auditable after plan archival.
- **As a polyglot maintainer**, I want breaking upgrades (Exposed 1.0, kotlinx-datetime 0.8) driven
  by tests, so that regressions are caught before they ship.
- **As a human operator**, I want the discontinued Flutter image surfaced as an explicit decision
  with a clear resume signal, so I know exactly what to choose and when the agent can continue.

## Acceptance criteria (Gherkin)

### Exact-pinning

```gherkin
Scenario: No caret or tilde specifiers remain in in-scope manifests
  Given the dependency bump has been applied to all in-scope ecosystems
  When I run a caret/tilde scan over each edited manifest
  Then the scan returns zero matches for floating specifiers in pinned in-scope dependencies
  And every approved target version matches the clearance report exactly
```

```gherkin
Scenario: No floating Docker base-image tags remain
  Given all Dockerfiles and docker-compose files have been edited
  When I grep for floating image tags (e.g. ":1.25-alpine", ":17-alpine", ":24-alpine", ":alpine")
  Then no floating tag remains for an in-scope base image
  And each base image is pinned to its exact approved tag (e.g. "golang:1.25.11-alpine3.22")
```

```gherkin
Scenario: No floating GitHub Actions majors remain
  Given all workflow and composite-action files have been edited
  When I grep each "uses:" line for the action majors in scope
  Then each action references its approved exact major (checkout@v6, cache@v5, …)
  And composite-action default version pins match the approved values
```

### Security re-audit

```gherkin
Scenario: npm audit is clean after the bump
  Given the npm ecosystem bumps are applied and lockfiles regenerated
  When I run "npm audit --audit-level=moderate"
  Then the audit reports zero vulnerabilities at moderate severity or above
```

```gherkin
Scenario: govulncheck is clean after the Go bump
  Given the Go ecosystem bumps are applied and "go mod tidy" has run
  When I run "govulncheck ./..." in each Go module
  Then govulncheck reports no known vulnerabilities in reachable code
```

```gherkin
Scenario: Per-ecosystem audits are clean after the bump
  Given each ecosystem's bumps are applied and its lockfile is regenerated
  When I run that ecosystem's available audit (pip-audit, mix deps.audit, cargo audit, etc.)
  Then the audit reports no unresolved vulnerabilities outside the documented waivers
```

```gherkin
Scenario: CISA KEV cross-reference is clean after the bump
  Given all bumps are applied
  When I cross-reference every resolved CVE against the CISA KEV catalog
  Then no in-scope pinned dependency carries an unpatched KEV-listed CVE
```

### Waiver register

```gherkin
Scenario: Every waiver and hold is recorded in the register
  Given the bump introduces 12 Path C waivers and 1 FUNCTIONAL-HOLD
  When I open docs/reference/security-waivers.md after execution
  Then the register contains one row per Path C waiver and one row for the FUNCTIONAL-HOLD
  And each row populates Date, Package, Pinned Version, Status, CVE(s)+URL, Severity, Release Date
  And the EPSS column is populated for every driving CVE with CVSS >= 7.0
  And the KEV columns are populated for any (KEV-listed) entry (otherwise "—")
  And the Sign-off column names the AI agent that applied the waiver
```

### Quality gates

```gherkin
Scenario: Affected quality gates are green after each ecosystem phase
  Given an ecosystem phase has applied its bumps
  When I run "npx nx affected -t typecheck lint test:quick spec-coverage"
  Then all four targets exit 0 for the affected projects
  And any preexisting failures encountered are fixed, not deferred
```

```gherkin
Scenario: CI is fully green after each push
  Given a phase's changes are pushed to origin main
  When the triggered GitHub Actions workflows run
  Then every CI check passes with zero failures
  And no subsequent phase begins until CI is green
```

### Breaking-upgrade scenarios

```gherkin
Scenario: Exposed 1.0 migration is test-driven
  Given crud-be-kotlin-ktor pins Exposed at 0.59.0
  When I bump Exposed to 1.0.0 following a RED -> GREEN -> REFACTOR cycle
  Then a failing test first reproduces the breaking API change
  And the migration code makes that test pass
  And "nx run crud-be-kotlin-ktor:test:quick" exits 0 after the migration
```

### Human decision

```gherkin
Scenario: Flutter base-image replacement is a human decision with a resume signal
  Given the Flutter build image "ghcr.io/cirruslabs/flutter:stable" is discontinued
  When execution reaches the [HUMAN] Flutter-image decision step
  Then execution pauses and surfaces the trade-off to the operator
  And the operator selects a maintained replacement image and pins it exact
  And the agent resumes only after the Flutter Dockerfile build succeeds with the new image
```

## Product scope

### In scope

- All 11-ecosystem manifest edits enumerated in [`tech-docs.md`](./tech-docs.md).
- Docker base-image exact pins and GitHub Actions major bumps.
- Lockfile regeneration, re-audit, KEV cross-reference, waiver-register propagation.
- Manual behavioral assertions (Playwright MCP for the four web frontends; curl for each backend).

### Out of scope

- Deferred post-cutoff upgrades: Dart 3.12.1, tailwindcss 4.3.0, FluentAssertions 8.x.
- Lockfile hand-edits; workspace-internal refs; type-only zero-surface dev deps.
- `crud-be-ts-effect`, `crud-be-e2e`, `crud-fe-e2e` (no in-scope bumps this cycle).

## Product-level risks

- **Breaking-upgrade regressions** — mitigated by TDD-shaped delivery items + phase gates.
- **Re-audit surfaces a newly-disclosed CVE** — mitigated by the snapshot caveat / eligibility re-run.
- **Manual assertion flakiness** — mitigated by explicit Playwright MCP / curl steps per app.

## References

- Business rationale → [`brd.md`](./brd.md)
- Clearance tables and waiver detail → [`tech-docs.md`](./tech-docs.md)
- Executable checklist → [`delivery.md`](./delivery.md)
- [June 2026 clearance report](../../../generated-reports/repo-dependency-bump-planning__be6560__2026-06-04--13-24__report.md)
