# ose-primer Template Cleanup

**Status**: In Progress
**Created**: 2026-04-18
**Plan Folder**: `plans/in-progress/2026-04-18__ose-primer-template-cleanup/`
**Working Directory**: `/Users/wkf/ose-projects/ose-primer/`
**Git Remote**: `origin` → `git@github.com:wahidyankf/ose-primer.git`
**Git Workflow**: Trunk Based Development on `main` (no worktrees, no PRs; direct commits + push to `main`)

## Context

`ose-primer` is a fresh repo forked from `ose-public` (`wahidyankf/ose-public.git` → `wahidyankf/ose-primer.git`). Its purpose is to become a **repository template**: a clean source that contributors can clone — or cherry-pick files from — when bootstrapping new OSE-style repos. The repo currently still contains all product-specific content carried over from `ose-public`: three web apps (`ayokoding-web`, `oseplatform-web`, `organiclever-fe`), their supporting CLIs and E2E projects, the `organiclever-be` F#/Giraffe backend, every matching Gherkin spec tree, the deprecated `hugo-commons` lib, a long list of product-specific agents and skills under `.claude/` and `.opencode/`, and scattered CI workflows, `package.json` scripts, governance enumerations, and Diátaxis docs that reference them.

This plan scopes the cleanup that strips product content while preserving everything generic — the `a-demo-*` backends and frontends, `rhino-cli` repo tooling, shared libs, governance, generic agents/skills, and planning infrastructure.

`ose-primer` is a single-repo Nx monorepo. The parent-repo / subrepo worktree conventions documented under `ose-projects` do NOT apply here. Execution runs in the main checkout on branch `main` (trunk-based development — no PRs).

## Scope

### In Scope

**File deletions** (product-specific content):

- **Apps** (12): `ayokoding-web`, `ayokoding-web-be-e2e`, `ayokoding-web-fe-e2e`, `ayokoding-cli`, `oseplatform-web`, `oseplatform-web-be-e2e`, `oseplatform-web-fe-e2e`, `oseplatform-cli`, `organiclever-fe`, `organiclever-fe-e2e`, `organiclever-be`, `organiclever-be-e2e`
- **Specs** (3): `specs/apps/ayokoding/`, `specs/apps/organiclever/`, `specs/apps/oseplatform/`
- **Libs** (1): `hugo-commons` (deprecated; `swe-hugo-dev` agent marked DEPRECATED)
- **Archived** (3): `archived/ayokoding-web-hugo/`, `archived/organiclever-web/`, `archived/oseplatform-web-hugo/`
- **Infra configs** (6): `infra/dev/ayokoding-web/`, `infra/dev/oseplatform-web/`, `infra/dev/organiclever/`, `infra/dev/ayokoding-cli/`, `infra/dev/oseplatform-cli/`, `infra/k8s/organiclever/`
- **Agents** (22): all `apps-ayokoding-web-*` (12), all `apps-oseplatform-web-*` (4), `apps-organiclever-fe-deployer`, `swe-hugo-dev` — in both `.claude/agents/` and `.opencode/agent/`
- **Skills** (3): `apps-ayokoding-web-developing-content`, `apps-organiclever-fe-developing-content`, `apps-oseplatform-web-developing-content` — in both `.claude/skills/` and `.opencode/skill/`
- **Plan** (1): `plans/in-progress/2026-04-16__organiclever-fe-local-first/`
- **CI workflows** (3 product + 1 orphan reusable = 4): `.github/workflows/test-and-deploy-ayokoding-web.yml`, `test-and-deploy-oseplatform-web.yml`, `test-and-deploy-organiclever.yml`, `_reusable-test-and-deploy.yml` (no remaining callers after the three product workflows are deleted)

**File rewrites** (audit + prune enumerations):

- `CLAUDE.md` — drop product app list, product website sections, env-branch table, removed agents; reframe as repository template
- `AGENTS.md` — mirror CLAUDE.md changes
- Top-level `README.md` — reframe as ose-primer template entry point — full rewrite, not a stub; first-read onboarding doc for new cloners
- `.claude/agents/README.md` — prune removed agents from catalog tables
- `.claude/settings.json` — audit permission entries for removed paths
- `governance/**` — generalize product examples; delete product-sole-subject files
- `docs/**` — generalize product examples; delete product-sole-subject Diátaxis files
- `LICENSING-NOTICE.md` — rewrite as MIT-only policy (no FSL split)
- `LICENSE` — replace FSL-1.1-MIT license text with MIT license text
- `package.json` — change top-level `"license": "FSL-1.1-MIT"` to `"license": "MIT"`
- All kept app/lib `package.json`, `pyproject.toml`, `Cargo.toml`, `.csproj`, `pom.xml`, `mix.exs`, `deps.edn`, `pubspec.yaml`, `go.mod` (if a license field is declared) — change license metadata to MIT
- `package.json` — remove product-specific scripts (~15 `dev:*` and `*:dev` entries)
- `nx.json`, `tsconfig.base.json` — audit for removed-project references
- `.opencode/` — regenerate from cleaned `.claude/` via `npm run sync:claude-to-opencode`

### Out of Scope

- Adding new template scaffolding beyond what already exists
- Restructuring `governance/` or `docs/` architecture
- Broad license-term migration beyond the realignment noted in `LICENSING-NOTICE.md`
- Remote env-branch deletion (new remote `wahidyankf/ose-primer` has only `main` — nothing to delete remotely)
- Any work on `ose-public` or `ose-infra` (this plan is `ose-primer` only)

### Affected Apps / Projects

- **Kept**: all `a-demo-*` apps (17), `rhino-cli`, `a-demo-contracts` (if project), `golang-commons`, `elixir-cabbage`, `elixir-gherkin`, `elixir-openapi-codegen`, `clojure-openapi-codegen`, `ts-ui`, `ts-ui-tokens`
- **Removed**: all apps listed under Scope → In Scope above

## Approach Summary

Phased execution in 17 phases, ordered so dependencies flow correctly:

1. **Preflight** — snapshot state, confirm clean tree and correct remote
2. **Remove product apps** (12 Nx projects)
3. **Remove product specs** (3 spec trees)
4. **Remove deprecated libs** (`hugo-commons`)
5. **Remove product agents** (22 agents in `.claude/agents/`)
6. **Remove product skills** (3 skills in `.claude/skills/`)
7. **Remove product plans + generated-socials** (plans/in-progress entry; `generated-socials/` absent)
8. **Rewrite `CLAUDE.md`** as template guidance
9. **Rewrite top-level `README.md`** as template usage guide
10. **Update `AGENTS.md`** (OpenCode mirror) to match `CLAUDE.md`
11. **Update agent catalog + `.claude/settings.json`**
12. **Audit and prune `governance/` enumerations**
13. **Audit and prune `docs/` Diátaxis content**
14. **Update `LICENSING-NOTICE.md`**
15. **Update tooling files** (`package.json`, `nx.json`, `tsconfig.base.json`, `.github/workflows/`, `infra/dev/`, `archived/`)
16. **Sync `.opencode/` from `.claude/`**
17. **Final validation + residual grep sweep + push**

Each phase commits independently using Conventional Commits. No force-push, no `--no-verify`, no branch creation — this is direct trunk-based work on `main`.

## Document Navigation

- [brd.md](./brd.md) — Business Requirements Document (WHY this cleanup)
- [prd.md](./prd.md) — Product Requirements Document (WHAT gets removed/rewritten, with Gherkin acceptance criteria)
- [tech-docs.md](./tech-docs.md) — Technical approach, file-impact analysis, mechanics, rollback
- [delivery.md](./delivery.md) — Phased delivery checklist (one checkbox = one action)
