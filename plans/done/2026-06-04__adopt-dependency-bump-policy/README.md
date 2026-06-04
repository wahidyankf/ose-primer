# Adopt Dependency Bump Policy & Planning Workflow

**Status**: In Progress
**Created**: 2026-06-04
**Identifier**: `adopt-dependency-bump-policy`

## Context

`ose-primer` inherits governance, conventions, and AI agent patterns from the upstream
[`ose-public`](https://github.com/wahidyankf/ose-public) platform monorepo. Two governance
artifacts that already exist in `ose-public` are **not yet present** in this repository:

1. **[Dependency Bump Stability & Safety Policy](https://github.com/wahidyankf/ose-public/blob/main/repo-governance/development/workflow/dependency-bump-policy.md)**
   — the three-path decision tree (LTS / 60-day soak / security waiver), exact-pin hard rule,
   CVE clearance process, CISA KEV fast-track, EPSS escalation, and the Rule 5a/5b selection rules
   that govern every dependency bump across the polyglot monorepo.
2. **[Repository Dependency Bump Planning Workflow](https://github.com/wahidyankf/ose-public/blob/main/repo-governance/workflows/repo/repo-dependency-bump-planning.md)**
   — the survey-and-classify workflow that turns the policy into a validated **backlog plan**
   (never editing manifests directly).

This plan adopts both documents into `ose-primer`, adapts their repo-specific references to this
repository's actual structure, and brings in the small set of **related rules and registers** the
two documents depend on but that this repo lacks.

## Scope

### In scope

- Create `repo-governance/development/workflow/dependency-bump-policy.md` (adapted).
- Create `repo-governance/workflows/repo/repo-dependency-bump-planning.md` (adapted to
  `ose-primer`'s real ecosystems: npm, Cargo, .NET, Go, Docker, GitHub Actions).
- Add the `planning` workflow **type token** to the
  [Workflow Naming Convention](../../../repo-governance/conventions/structure/workflow-naming.md)
  and to **both** `rhino-cli` validators (`rhino-cli-rust`, `rhino-cli-go`) so the new workflow
  filename passes `rhino-cli workflows validate-naming` (Husky pre-push + CI).
- Create `repo-governance/development/agents/subagent-orchestration.md` (the concurrency-cap rule
  the planning workflow references).
- Create the `docs/reference/security-waivers.md` register stub (waiver destination referenced by
  the policy and workflow).
- Wire all new documents into their index READMEs and cross-references.

### Out of scope

- **Running any dependency bump.** This plan adopts governance documents only. No `package.json`,
  `Cargo.toml`, `rust-toolchain.toml`, `go.mod`, `*.csproj`, `*.fsproj`, `global.json`,
  `Dockerfile`, `docker-compose*.yml`, `.github/` action, or lockfile is bumped by this plan.
- Authoring an actual `dependency-bump` backlog plan (that is what the newly-adopted workflow
  produces later, on demand).

## Approach Summary

Pure governance/documentation adoption plus a small supporting code change (the `planning` type
token in two `rhino-cli` validators). Work proceeds phase-by-phase: policy doc → naming-type
support → subagent-orchestration convention → planning workflow doc → security-waivers register →
cross-reference wiring and validation. Each phase ends with a green gate.

## Navigation

- [Business Requirements](./brd.md) — why this adoption matters.
- [Product Requirements](./prd.md) — what is delivered, with Gherkin acceptance criteria.
- [Technical Documentation](./tech-docs.md) — adaptation decisions and file-impact map.
- [Delivery Checklist](./delivery.md) — phased, executable steps.

## Related Documentation

- [Repository Ecosystem Convention](../../../repo-governance/conventions/structure/repository-ecosystem.md)
- [Workflow Naming Convention](../../../repo-governance/conventions/structure/workflow-naming.md)
- [Reproducible Environments Convention](../../../repo-governance/development/workflow/reproducible-environments.md)
