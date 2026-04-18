# Delivery Checklist — ose-primer Template Cleanup

**Working directory for every command**: `/Users/wkf/ose-projects/ose-primer/`
**Branch**: `main` (trunk-based; no branches, no PRs)
**Remote**: `origin` → `git@github.com:wahidyankf/ose-primer.git`
**Command prefix**: all `git`, `nx`, and shell commands use `rtk` prefix.

> **Fix-all-issues rule**: fix ALL failures found during quality gates, not just those caused by your changes. Root Cause Orientation principle — proactively fix preexisting errors encountered during work. Preexisting fixes go in their own thematic commits, separate from cleanup commits. Never bypass gates with `--no-verify`.

## Environment Setup

- [x] Install dependencies in the root worktree: `rtk npm install`
  - Date: 2026-04-18
  - Status: done
  - Files Changed: none (installs only)
  - Notes: 19/19 tools OK; up to date, 1909 packages audited. 39 vulns reported by npm audit (13 moderate, 24 high, 2 critical) — preexisting, not in scope for cleanup plan.
- [x] Converge the full polyglot toolchain: `rtk npm run doctor -- --fix` (required — `postinstall` runs `doctor || true` and tolerates drift silently; see [Worktree Toolchain Initialization](../../../governance/development/workflow/worktree-setup.md))
  - Date: 2026-04-18
  - Status: done
  - Files Changed: none
  - Notes: 19/19 tools OK. Nothing to fix.
- [x] Verify `rhino-cli` builds cleanly: `rtk nx build rhino-cli`
  - Date: 2026-04-18
  - Status: done
  - Files Changed: none (cache hit)
  - Notes: Build succeeded, cache hit. Binary at `apps/rhino-cli/dist/rhino-cli`.
- [x] Run existing full typecheck + lint to establish baseline: `rtk nx run-many -t typecheck lint`
  - Date: 2026-04-18
  - Status: done
  - Files Changed: none
  - Notes: 40 projects + 21 dep tasks succeeded. 85/96 tasks cache-hit. 5 ESLint `no-empty-pattern` warnings in `oseplatform-web-be-e2e/src/steps/common.steps.ts` (preexisting; app removed in Phase 1 so fix is deletion). Spurious `SqliteFailure` at end is Nx daemon telemetry, not a task failure.
- [x] Note any preexisting failures in `local-temp/preexisting-failures.txt` for fixing during execution
  - Date: 2026-04-18
  - Status: done
  - Files Changed: local-temp/preexisting-failures.txt
  - Notes: Only preexisting issues are 5 lint warnings in removed-in-Phase-1 oseplatform-web-be-e2e; no errors.
- [x] Run existing `test:quick` for a kept a-demo app to confirm baseline: `rtk nx run a-demo-be-golang-gin:test:quick`
  - Date: 2026-04-18
  - Status: done
  - Files Changed: none (cache hit)
  - Notes: PASS 90.99% coverage >= 90% threshold. Cache hit.

## Phase 0 — Preflight

- [x] Run `rtk git status` and confirm working tree is clean
  - Date: 2026-04-18
  - Status: done
  - Files Changed: none
  - Notes: Working tree clean at execution start (only delivery.md has the in-flight atomic-sync ticks from this session).
- [x] Run `rtk git remote -v` and confirm `origin` = `git@github.com:wahidyankf/ose-primer.git`
  - Date: 2026-04-18
  - Status: done
  - Notes: origin matches.
- [x] Run `rtk git branch --show-current` and confirm branch is `main`
  - Date: 2026-04-18
  - Status: done
  - Notes: On `main`.
- [x] Record current project list: `rtk npx nx show projects > local-temp/pre-cleanup-nx-projects.txt`
  - Date: 2026-04-18
  - Status: done
  - Notes: 41 projects.
- [x] Record current file count: `rtk git ls-files | wc -l > local-temp/pre-cleanup-filecount.txt`
  - Date: 2026-04-18
  - Status: done
  - Notes: 4749 tracked files.
- [x] Record current agent count: `rtk ls .claude/agents/*.md | wc -l > local-temp/pre-cleanup-agent-count.txt`
  - Date: 2026-04-18
  - Status: done
  - Notes: 70 agents.
- [x] Record current skill count: `rtk ls -d .claude/skills/*/ | wc -l > local-temp/pre-cleanup-skill-count.txt`
  - Date: 2026-04-18
  - Status: done
  - Notes: 38 skills.

## Phase 1 — Remove product apps

- [x] `rtk git rm -r apps/ayokoding-web`
  - Date: 2026-04-18
  - Status: done
- [x] `rtk git rm -r apps/ayokoding-web-be-e2e`
  - Date: 2026-04-18
  - Status: done
- [x] `rtk git rm -r apps/ayokoding-web-fe-e2e`
  - Date: 2026-04-18
  - Status: done
- [x] `rtk git rm -r apps/ayokoding-cli`
  - Date: 2026-04-18
  - Status: done
- [x] `rtk git rm -r apps/oseplatform-web`
  - Date: 2026-04-18
  - Status: done
- [x] `rtk git rm -r apps/oseplatform-web-be-e2e`
  - Date: 2026-04-18
  - Status: done
- [x] `rtk git rm -r apps/oseplatform-web-fe-e2e`
  - Date: 2026-04-18
  - Status: done
- [x] `rtk git rm -r apps/oseplatform-cli`
  - Date: 2026-04-18
  - Status: done
- [x] `rtk git rm -r apps/organiclever-fe`
  - Date: 2026-04-18
  - Status: done
- [x] `rtk git rm -r apps/organiclever-fe-e2e`
  - Date: 2026-04-18
  - Status: done
- [x] `rtk git rm -r apps/organiclever-be`
  - Date: 2026-04-18
  - Status: done
- [x] `rtk git rm -r apps/organiclever-be-e2e`
  - Date: 2026-04-18
  - Status: done
- [x] Run `rtk npx nx show projects` and verify no `ayokoding-*`, `oseplatform-*`, `organiclever-*` projects remain
  - Date: 2026-04-18
  - Status: done
  - Notes: all apps removed. `organiclever-contracts` persists (tied to `specs/apps/organiclever/`) — will cascade-delete in Phase 2 as planned.
- [x] Run `rtk npx nx affected -t typecheck lint --base=HEAD`
  - Date: 2026-04-18
  - Status: done
  - Notes: Ran as `rtk nx affected -t typecheck lint --base=HEAD`. Exit 0.
- [x] Fix any failure surfaced (may be preexisting — address root cause)
  - Date: 2026-04-18
  - Status: done
  - Notes: No failures surfaced.
- [x] Commit: `rtk git commit -m "chore(cleanup): remove product apps (ayokoding, oseplatform, organiclever)"`
  - Date: 2026-04-18
  - Status: done (commit d251ed7e)
  - Files Changed: 1463 (12 apps + go.work cleanup — go.work stripped of deleted module refs to unblock pre-commit hook)
  - Notes: Pre-commit blocked on stale `go.work` (still listed ayokoding-cli, oseplatform-cli, hugo-commons). Fixed in same commit.

## Phase 2 — Remove product specs

- [x] `rtk git rm -r specs/apps/ayokoding`
  - Date: 2026-04-18 — done
- [x] `rtk git rm -r specs/apps/organiclever`
  - Date: 2026-04-18 — done
- [x] `rtk git rm -r specs/apps/oseplatform`
  - Date: 2026-04-18 — done
- [x] Run `rtk npx nx affected -t lint spec-coverage --base=HEAD`
  - Date: 2026-04-18 — done; exit 0 after go.work/hugo-commons re-add (removal deferred to Phase 3)
- [x] Fix any failure surfaced
  - Date: 2026-04-18 — done; hugo-commons lint failed after Phase 1's go.work prune dropped it; re-added `./libs/hugo-commons` to `go.work` until Phase 3 deletes the lib itself.
- [x] Run `rtk npx nx show projects` and confirm `organiclever-contracts` is no longer listed (spec-tree deletion cascades into contract project removal)
  - Date: 2026-04-18 — done; no ayokoding/oseplatform/organiclever projects remain.
- [x] Commit: `rtk git commit -m "chore(cleanup): remove product-app Gherkin specs"`
  - Date: 2026-04-18 — done (commit a8eaf74a)

## Phase 3 — Remove deprecated libs

- [x] `rtk grep -r "hugo-commons" apps/ libs/` and confirm no hits (demo apps must not import it)
  - Date: 2026-04-18 — done; only self-refs in libs/hugo-commons/, none in apps/ or other libs/.
- [x] `rtk git rm -r libs/hugo-commons`
  - Date: 2026-04-18 — done
- [x] `rtk git rm -r specs/libs/hugo-commons`
  - Date: 2026-04-18 — done
- [x] Run `rtk npx nx affected -t typecheck lint --base=HEAD`
  - Date: 2026-04-18 — done; exit 0 after go.work pruned.
- [x] Fix any failure surfaced
  - Date: 2026-04-18 — done; stripped `./libs/hugo-commons` from go.work.
- [x] Commit: `rtk git commit -m "chore(cleanup): remove deprecated hugo-commons lib"`
  - Date: 2026-04-18 — pending commit below

## Phase 4 — Remove product agents (.claude side)

- [ ] `rtk git rm .claude/agents/apps-ayokoding-web-by-example-maker.md`
- [ ] `rtk git rm .claude/agents/apps-ayokoding-web-by-example-checker.md`
- [ ] `rtk git rm .claude/agents/apps-ayokoding-web-by-example-fixer.md`
- [ ] `rtk git rm .claude/agents/apps-ayokoding-web-in-the-field-maker.md`
- [ ] `rtk git rm .claude/agents/apps-ayokoding-web-in-the-field-checker.md`
- [ ] `rtk git rm .claude/agents/apps-ayokoding-web-in-the-field-fixer.md`
- [ ] `rtk git rm .claude/agents/apps-ayokoding-web-general-maker.md`
- [ ] `rtk git rm .claude/agents/apps-ayokoding-web-general-checker.md`
- [ ] `rtk git rm .claude/agents/apps-ayokoding-web-general-fixer.md`
- [ ] `rtk git rm .claude/agents/apps-ayokoding-web-facts-checker.md`
- [ ] `rtk git rm .claude/agents/apps-ayokoding-web-facts-fixer.md`
- [ ] `rtk git rm .claude/agents/apps-ayokoding-web-link-checker.md`
- [ ] `rtk git rm .claude/agents/apps-ayokoding-web-link-fixer.md`
- [ ] `rtk git rm .claude/agents/apps-ayokoding-web-deployer.md`
- [ ] `rtk git rm .claude/agents/apps-oseplatform-web-content-maker.md`
- [ ] `rtk git rm .claude/agents/apps-oseplatform-web-content-checker.md`
- [ ] `rtk git rm .claude/agents/apps-oseplatform-web-content-fixer.md`
- [ ] `rtk git rm .claude/agents/apps-oseplatform-web-deployer.md`
- [ ] `rtk git rm .claude/agents/apps-organiclever-fe-deployer.md`
- [ ] `rtk git rm .claude/agents/swe-hugo-dev.md`
- [ ] Run `rtk npm run validate:claude` and verify .claude format remains valid
- [ ] Commit: `rtk git commit -m "chore(cleanup): remove product-specific agents from .claude"`

## Phase 5 — Remove product skills (.claude side)

- [ ] `rtk git rm -r .claude/skills/apps-ayokoding-web-developing-content`
- [ ] `rtk git rm -r .claude/skills/apps-organiclever-fe-developing-content`
- [ ] `rtk git rm -r .claude/skills/apps-oseplatform-web-developing-content`
- [ ] Run `rtk npm run validate:claude`
- [ ] Commit: `rtk git commit -m "chore(cleanup): remove product-specific skills from .claude"`

## Phase 6 — Remove all other plans + clean ideas.md + generated-socials

- [ ] `rtk git rm -r plans/in-progress/2026-04-16__organiclever-fe-local-first`
- [ ] Edit `plans/in-progress/README.md` — remove the `2026-04-16__organiclever-fe-local-first` bullet from the Active Plans list
- [ ] Run `rtk ls plans/done/` and confirm count of YYYY-MM-DD\_\_\* entries matches the expected 53 snapshot taken at plan creation
- [ ] `rtk git rm -r plans/done/2026-01-02__rules-consolidation`
- [ ] `rtk git rm -r plans/done/2026-01-02__skills-layer-implementation`
- [ ] `rtk git rm -r plans/done/2026-01-03__agent-skills-simplification`
- [ ] `rtk git rm -r plans/done/2026-01-03__opencode-adoption`
- [ ] `rtk git rm -r plans/done/2026-01-06__move-rules-to-root`
- [ ] `rtk git rm -r plans/done/2026-01-12__claude-code-full-migration`
- [ ] `rtk git rm -r plans/done/2026-01-17__dolphin-be-init`
- [ ] `rtk git rm -r plans/done/2026-01-17__markdown-linting`
- [ ] `rtk git rm -r plans/done/2026-01-17__repository-link-remediation`
- [ ] `rtk git rm -r plans/done/2026-01-22__stack-lang-golang`
- [ ] `rtk git rm -r plans/done/2026-02-03__go-docs-alignment`
- [ ] `rtk git rm -r plans/done/2026-02-14__orca-grid-be-removal`
- [ ] `rtk git rm -r plans/done/2026-02-23__local-ci-standardization`
- [ ] `rtk git rm -r plans/done/2026-02-24__dependency-update`
- [ ] `rtk git rm -r plans/done/2026-03-09__auth-register-login`
- [ ] `rtk git rm -r plans/done/2026-03-09__organiclever-be-exph`
- [ ] `rtk git rm -r plans/done/2026-03-11__demo-be-fsharp-giraffe`
- [ ] `rtk git rm -r plans/done/2026-03-11__demo-be-golang-gin`
- [ ] `rtk git rm -r plans/done/2026-03-11__demo-be-java-vertx`
- [ ] `rtk git rm -r plans/done/2026-03-11__demo-be-kotlin-ktor`
- [ ] `rtk git rm -r plans/done/2026-03-11__demo-be-python-fastapi`
- [ ] `rtk git rm -r plans/done/2026-03-11__demo-be-rust-axum`
- [ ] `rtk git rm -r plans/done/2026-03-12__demo-be-clojure-pedestal`
- [ ] `rtk git rm -r plans/done/2026-03-12__demo-be-csharp-aspnetcore`
- [ ] `rtk git rm -r plans/done/2026-03-12__demo-be-ts-effect`
- [ ] `rtk git rm -r plans/done/2026-03-13__demo-specs-consolidation`
- [ ] `rtk git rm -r plans/done/2026-03-13__testing-standardization`
- [ ] `rtk git rm -r plans/done/2026-03-17__demo-api-contract-enforcement`
- [ ] `rtk git rm -r plans/done/2026-03-17__demo-fe-ts-tanstack-start`
- [ ] `rtk git rm -r plans/done/2026-03-18__api-contract-adoption`
- [ ] `rtk git rm -r plans/done/2026-03-19__demo-ci-test-standardization`
- [ ] `rtk git rm -r plans/done/2026-03-20__rhino-cli-coverage-improvements`
- [ ] `rtk git rm -r plans/done/2026-03-22__demo-fs-ts-nextjs`
- [ ] `rtk git rm -r plans/done/2026-03-23__ayokoding-web-v2`
- [ ] `rtk git rm -r plans/done/2026-03-24__ayokoding-web-v1-to-v2-migration`
- [ ] `rtk git rm -r plans/done/2026-03-25__ayokoding-web-ci-quality-standardization`
- [ ] `rtk git rm -r plans/done/2026-03-26__database-migration-tooling`
- [ ] `rtk git rm -r plans/done/2026-03-27__demo-repository-pattern`
- [ ] `rtk git rm -r plans/done/2026-03-27__ui-development-improvement`
- [ ] `rtk git rm -r plans/done/2026-03-28__organiclever-fullstack-evolution`
- [ ] `rtk git rm -r plans/done/2026-03-28__oseplatform-web-nextjs-rewrite`
- [ ] `rtk git rm -r plans/done/2026-03-30__cli-testing-alignment`
- [ ] `rtk git rm -r plans/done/2026-03-30__env-backup-restore`
- [ ] `rtk git rm -r plans/done/2026-03-30__oseplatform-web-e2e-apps`
- [ ] `rtk git rm -r plans/done/2026-03-31__ci-standardization`
- [ ] `rtk git rm -r plans/done/2026-03-31__env-enhanced-backup-restore`
- [ ] `rtk git rm -r plans/done/2026-04-02__spec-coverage-full-enforcement`
- [ ] `rtk git rm -r plans/done/2026-04-02__specs-structure-consistency`
- [ ] `rtk git rm -r plans/done/2026-04-04__fsl-license-migration`
- [ ] `rtk git rm -r plans/done/2026-04-04__native-dev-setup-improvements`
- [ ] `rtk git rm -r plans/done/2026-04-11__remove-obsidian-compat`
- [ ] `rtk git rm -r plans/done/2026-04-17__agent-and-workflow-naming-consistency`
- [ ] `rtk git rm -r plans/done/2026-04-18__plan-convention-brd-prd-split`
- [ ] Verify `plans/done/` now contains only `README.md`: `rtk ls plans/done/` should show `README.md` and no `YYYY-MM-DD__*` entries
- [ ] Edit `plans/done/README.md` — rewrite to reflect empty archive: retain H1 "# Completed Plans" and "Completed Projects" section heading; replace the bulleted list body with a short line such as "_No completed plans yet in this template._"
- [ ] Read `plans/backlog/README.md` — confirm it describes an empty backlog; no edit expected
- [ ] Rewrite `plans/ideas.md` — replace body with template-generic placeholder: keep H1 "# Ideas" and the one-line description, then replace the section bullets with a single placeholder line such as "_No ideas yet. Capture 1-3 liner ideas here and promote mature ones to backlog/ plans._"
- [ ] Verify `rtk grep -n "ayokoding\|oseplatform\|organiclever\|FSL" plans/ideas.md` returns empty
- [ ] Run `rtk ls generated-socials 2>/dev/null` — if present, `rtk git rm -r generated-socials`; if absent, skip
- [ ] Run `rtk npm run lint:md:fix`
- [ ] Commit: `rtk git commit -m "chore(cleanup): remove all other plans, reset ideas, drop generated-socials"`

## Phase 7 — Rewrite CLAUDE.md

- [ ] Edit `CLAUDE.md` — rewrite "Project Overview" paragraph: drop "Phase 1 (OrganicLever - Productivity Tracker)"; reframe as "Repository template for OSE-style polyglot monorepos"
- [ ] Edit `CLAUDE.md` — "Current Apps" list: delete every bullet for `ayokoding-*`, `oseplatform-*`, `organiclever-*` (keep only `a-demo-*`, `rhino-cli`, `a-demo-contracts`)
- [ ] Edit `CLAUDE.md` — "Project Structure" ASCII tree: strip the same apps
- [ ] Edit `CLAUDE.md` — "Coverage thresholds" table: remove rows for `ayokoding-web`, `oseplatform-web`, `organiclever-fe`, `organiclever-be`
- [ ] Edit `CLAUDE.md` — "Git Workflow" section: delete the three `prod-*` env-branch bullets
- [ ] Edit `CLAUDE.md` — "AI Agents" catalog: drop every removed agent from each role grouping (Content Creation, Validation, Fixing, Operations, SWE)
- [ ] Edit `CLAUDE.md` — delete the entire "Web Sites" section (oseplatform-web, ayokoding-web, organiclever-fe, organiclever-be sub-sections)
- [ ] Verify: `rtk grep -n "ayokoding\|oseplatform\|organiclever\|hugo-commons" CLAUDE.md` returns empty
- [ ] Run `rtk npm run lint:md:fix`
- [ ] Run `rtk npm run lint:md` and confirm pass
- [ ] Commit: `rtk git commit -m "docs(claude): reframe CLAUDE.md as repository template guidance"`

## Phase 8 — Rewrite top-level README.md

- [ ] Read existing `README.md` once to understand its current framing
- [ ] Draft new section order: "What this is", "What it ships", "How to use this template", "Prerequisites", "Common commands", "Governance & conventions", "Repository layout", "License"
- [ ] Write "What this is" — 2-3 sentences framing ose-primer as a cloneable/cherry-pickable template for OSE-style polyglot monorepos
- [ ] Write "What it ships" — bullet list: polyglot `a-demo-*` scaffolding (11 backends, 3 frontends, 1 fullstack, contracts, E2E), `rhino-cli` repo tooling, shared libs, governance, generic agents/skills, planning infrastructure
- [ ] Write "How to use this template" — step-by-step: `git clone`, choose `a-demo-*` variants to keep, delete unwanted variants, rename via search-and-replace or `rhino-cli` helpers, point `origin` at the new remote, push to `main`
- [ ] Write "Prerequisites" — Volta + Node pinned version; single-command setup `npm install && npm run doctor -- --fix`
- [ ] Write "Common commands" — `nx build`, `nx affected -t …`, `npm run lint:md`, `npm run doctor`, `npm run sync:claude-to-opencode`
- [ ] Write "Governance & conventions" — link to `governance/README.md` and list top-level principle categories
- [ ] Write "Repository layout" — brief ASCII or bullet tree showing `apps/`, `libs/`, `specs/`, `governance/`, `docs/`, `plans/`, `.claude/`, `.opencode/`
- [ ] Write "License" — short statement: "MIT. See `LICENSE` and `LICENSING-NOTICE.md`."
- [ ] Remove any product-site links (`oseplatform.com`, `ayokoding.com`, `www.organiclever.com`)
- [ ] Remove any "Phase 1 (OrganicLever)" or product-app framing left over from `ose-public`
- [ ] Verify: `rtk grep -n "ayokoding\|oseplatform\|organiclever\|hugo-commons" README.md` returns empty
- [ ] Run `rtk npm run lint:md:fix`
- [ ] Run `rtk npm run lint:md` and confirm pass
- [ ] Commit: `rtk git commit -m "docs(readme): rewrite root README as ose-primer template entry point"`

## Phase 9 — Update AGENTS.md (OpenCode mirror)

- [ ] Apply Phase 7 edits to `AGENTS.md` — same overview reframe, same app-list pruning, same agent-catalog pruning, same Web Sites deletion
- [ ] Verify: `rtk grep -n "ayokoding\|oseplatform\|organiclever\|hugo-commons" AGENTS.md` returns empty
- [ ] Run `rtk npm run lint:md:fix`
- [ ] Run `rtk npm run lint:md` and confirm pass
- [ ] Commit: `rtk git commit -m "docs(agents): sync AGENTS.md with CLAUDE.md template framing"`

## Phase 10 — Update .claude/agents/README.md + .claude/settings.json

- [ ] Edit `.claude/agents/README.md` — drop every removed agent from catalog tables and role groupings
- [ ] Verify: `rtk grep -n "apps-ayokoding-web\|apps-oseplatform-web\|apps-organiclever-fe-deployer\|swe-hugo-dev" .claude/agents/README.md` returns empty
- [ ] Open `.claude/settings.json` and scan for permission entries scoped to removed paths (`apps/ayokoding-web/**`, `apps/oseplatform-web/**`, `apps/organiclever-*/**`, `libs/hugo-commons/**`); strip any matches — expected no-op, verified 2026-04-18 that current file contains only generic `.claude/**`, `.opencode/**`, `/tmp/**` entries
- [ ] Run `rtk npm run validate:claude`
- [ ] Run `rtk npm run lint:md:fix`
- [ ] Commit: `rtk git commit -m "chore(claude): prune agent catalog and settings of removed product paths"`

## Phase 11 — Audit and prune governance docs

- [ ] Enumerate hits: `rtk grep -rn "ayokoding\|oseplatform\|organiclever\|hugo" governance/ > local-temp/governance-hits.txt`
- [ ] For each listed file, decide: rewrite (generalise the example) or delete (product-sole subject)
- [ ] Apply edits to conventions files (`governance/conventions/`)
- [ ] Apply edits to development files (`governance/development/`)
- [ ] Apply edits to workflow files (`governance/workflows/`)
- [ ] Apply edits to principles + vision files if touched (`governance/principles/`, `governance/vision/`)
- [ ] Run `docs-link-checker` agent on `governance/` subtree and address any broken-link findings
- [ ] Verify: `rtk grep -rn "ayokoding\|oseplatform\|organiclever\|hugo" governance/` returns empty
- [ ] Run `rtk npm run lint:md:fix`
- [ ] Run `rtk npm run lint:md` and confirm pass
- [ ] Commit conventions subgrouping: `rtk git commit -m "docs(governance): drop product-specific references from conventions"`
- [ ] Commit development subgrouping: `rtk git commit -m "docs(governance): drop product-specific references from development"`
- [ ] Commit workflows subgrouping: `rtk git commit -m "docs(governance): drop product-specific references from workflows"`
- [ ] Commit principles + vision if touched: `rtk git commit -m "docs(governance): drop product-specific references from principles and vision"`

## Phase 12 — Audit and prune Diátaxis docs

- [ ] Enumerate hits: `rtk grep -rn "ayokoding\|oseplatform\|organiclever\|hugo" docs/ > local-temp/docs-hits.txt`
- [ ] For each listed file, decide: rewrite (generalise) or delete (product-sole subject)
- [ ] Apply edits to tutorials + how-to files (`docs/tutorials/`, `docs/how-to/`)
- [ ] Apply edits to reference + explanation files (`docs/reference/`, `docs/explanation/`)
- [ ] Run `docs-link-checker` agent on `docs/` subtree and address any broken-link findings
- [ ] Verify: `rtk grep -rn "ayokoding\|oseplatform\|organiclever\|hugo" docs/` returns empty
- [ ] Run `rtk npm run lint:md:fix`
- [ ] Run `rtk npm run lint:md` and confirm pass
- [ ] Commit: `rtk git commit -m "docs(diataxis): remove product-scoped tutorials, how-tos, and references"`

## Phase 12.5 — Audit every remaining markdown file under kept paths

- [ ] Enumerate every surviving `.md` file under audit scope: `rtk find apps libs specs infra apps-labs archived .claude .opencode governance docs plans -name "*.md" -type f > local-temp/remaining-md-files.txt`
- [ ] Enumerate hits: `xargs rtk grep -l "ayokoding\|oseplatform\|organiclever\|hugo-commons\|FSL-1.1-MIT" < local-temp/remaining-md-files.txt | rg -v "plans/done/" > local-temp/remaining-md-hits.txt || true`
- [ ] For each file in `local-temp/remaining-md-hits.txt`, open and decide: rewrite (generalise) or delete (product-sole subject)
- [ ] Scrub `apps/*/README.md` for every kept app (17 a-demo-\* + rhino-cli) — rewrite any leftover product mentions
- [ ] Scrub `libs/*/README.md` for every kept lib (golang-commons, elixir-cabbage, elixir-gherkin, elixir-openapi-codegen, clojure-openapi-codegen, ts-ui, ts-ui-tokens) — rewrite any leftover product mentions
- [ ] Scrub `.claude/agents/*.md` bodies (kept agents only; removed agents were deleted in Phase 4) — rewrite any leftover product mentions in agent descriptions, examples, or tool-use hints
- [ ] Scrub `.claude/skills/*/SKILL.md` files — rewrite any leftover product mentions
- [ ] Scrub `.claude/skills/*/reference/*.md` files (when present) — rewrite any leftover product mentions
- [ ] Scrub `.opencode/agent/*.md` and `.opencode/skill/**/*.md` — note these are synced via Phase 15; verify post-sync that zero product refs exist, and fix the source in `.claude/` if any do
- [ ] Scrub `specs/apps/a-demo/**/*.md` and `specs/apps/rhino/**/*.md` and `specs/libs/**/*.md` — rewrite any leftover product mentions
- [ ] Scrub `infra/dev/*/README.md` (only the kept a-demo-\* and rhino-cli subdirs remain after Phase 14) — rewrite any leftover product mentions
- [ ] Scrub `archived/README.md` — rewrite any references to removed snapshots (`ayokoding-web-hugo`, `organiclever-web`, `oseplatform-web-hugo` are deleted in Phase 14)
- [ ] Scrub `apps-labs/README.md` — rewrite any leftover product mentions
- [ ] Scrub `plans/backlog/README.md`, `plans/in-progress/README.md`, `plans/done/README.md` — rewrite any leftover product mentions (note Phase 6 already rewrote done/README.md to the empty-state placeholder)
- [ ] Re-run enumeration: `xargs rtk grep -l "ayokoding\|oseplatform\|organiclever\|hugo-commons\|FSL-1.1-MIT" < local-temp/remaining-md-files.txt | rg -v "plans/done/"` — output MUST be empty
- [ ] Run `rtk npm run lint:md:fix`
- [ ] Run `rtk npm run lint:md` and confirm pass
- [ ] Commit: `rtk git commit -m "docs(cleanup): scrub all remaining markdown files of product references"`

## Phase 13 — Switch to MIT license

- [ ] Read existing `LICENSE` to capture copyright holder name
- [ ] Replace `LICENSE` body with standard MIT license text (year 2026, same copyright holder)
- [ ] Read existing `LICENSING-NOTICE.md`
- [ ] Rewrite `LICENSING-NOTICE.md` as short MIT-only notice (no FSL split; pointer to `LICENSE`; one-line "policy shift from ose-public" note)
- [ ] Edit root `package.json`: change `"license": "FSL-1.1-MIT"` to `"license": "MIT"`
- [ ] Enumerate embedded license metadata: `rtk grep -rn "FSL-1.1-MIT" apps libs specs`
- [ ] For each hit, change the license identifier to `MIT`
- [ ] Verify zero hits remain: `rtk grep -rn "FSL-1.1-MIT" .` (exclude `plans/done/` and `.git/`); output must be empty
- [ ] Verify root `LICENSE` contains standard MIT text: `rtk head -3 LICENSE | rtk grep -i "MIT License"` returns a match
- [ ] Run `rtk npm run lint:md:fix`
- [ ] Commit: `rtk git commit -m "docs(license): switch ose-primer to MIT license (template repo policy)"`

## Phase 14 — Update tooling files (package.json, nx.json, tsconfig.base.json, .github/workflows, infra/dev, archived)

- [ ] Edit `package.json` — remove scripts: `organiclever:dev`, `organiclever:dev:restart`, `dev:ayokoding-web`, `dev:oseplatform-web`, `dev:organiclever`, `dev:ayokoding-cli`, `dev:oseplatform-cli`
- [ ] Audit `package.json` for any other scripts referencing removed apps (grep `ayokoding\|oseplatform\|organiclever`)
- [ ] Read `nx.json` and confirm `targetDefaults`, `namedInputs`, `defaultBase` are project-agnostic (no edits expected)
- [ ] Read `tsconfig.base.json` and confirm `compilerOptions.paths` has no alias for `hugo-commons` or any removed lib; strip any that exist
- [ ] `rtk git rm .github/workflows/test-and-deploy-ayokoding-web.yml`
- [ ] `rtk git rm .github/workflows/test-and-deploy-oseplatform-web.yml`
- [ ] `rtk git rm .github/workflows/test-and-deploy-organiclever.yml`
- [ ] Confirm `_reusable-test-and-deploy.yml` has zero remaining callers via `rtk grep -l "_reusable-test-and-deploy" .github/workflows`; if empty, `rtk git rm .github/workflows/_reusable-test-and-deploy.yml`
- [ ] Audit `.github/workflows/pr-quality-gate.yml` for references (`needs:`, `uses:`, job filters) to removed workflows or removed projects; strip matches
- [ ] Audit `.github/workflows/_reusable-*.yml` for the same; strip matches
- [ ] Audit `.github/workflows/codecov-upload.yml` for references to removed projects; strip product-specific matrix entries, job filters, or path triggers
- [ ] Inspect `.github/workflows/test-a-demo-be-java-springboot.yml` for product-brand mentions (confirmed on 2026-04-18 via grep); scrub any stale product references (comments, matrix entries, job names) without deleting the workflow — the `a-demo-be-java-springboot` app is a KEPT app
- [ ] `rtk git rm -r infra/dev/ayokoding-web`
- [ ] `rtk git rm -r infra/dev/oseplatform-web`
- [ ] `rtk git rm -r infra/dev/organiclever`
- [ ] `rtk git rm -r infra/dev/ayokoding-cli`
- [ ] `rtk git rm -r infra/dev/oseplatform-cli`
- [ ] `rtk git rm -r infra/k8s/organiclever`
- [ ] `rtk git rm -r archived/ayokoding-web-hugo`
- [ ] `rtk git rm -r archived/organiclever-web`
- [ ] `rtk git rm -r archived/oseplatform-web-hugo`
- [ ] Run `rtk npx nx affected -t typecheck lint test:quick --base=HEAD`
- [ ] Run `rtk npm run lint:md:fix`
- [ ] Commit: `rtk git commit -m "chore(tooling): prune Nx/package/workflow/infra references to removed apps"`

## Phase 15 — Sync .opencode from .claude

- [ ] Run `rtk npm run sync:claude-to-opencode`
- [ ] Verify `.opencode/agent/` no longer contains any `apps-ayokoding-web-*`, `apps-oseplatform-web-*`, `apps-organiclever-fe-deployer.md`, or `swe-hugo-dev.md`
- [ ] Verify `.opencode/skill/` no longer contains `apps-ayokoding-web-developing-content/`, `apps-organiclever-fe-developing-content/`, `apps-oseplatform-web-developing-content/`
- [ ] Run `rtk npm run validate:opencode`
- [ ] Run `rtk git status` and `rtk git diff --stat .opencode/` to review sync delta
- [ ] Commit: `rtk git commit -m "chore(opencode): sync after .claude cleanup"`

## Phase 16 — Final validation

### Local Quality Gates

- [ ] `rtk npm run doctor`
- [ ] `rtk npm run lint:md`
- [ ] `rtk npx nx run-many -t typecheck`
- [ ] `rtk npx nx run-many -t lint`
- [ ] `rtk npx nx run-many -t test:quick`
- [ ] `rtk npx nx run-many -t spec-coverage`
- [ ] Fix ALL failures surfaced — including preexisting issues; commit each preexisting fix separately with an accurate Conventional Commits message
- [ ] Re-run all gates until zero failures

### Residual Brand Sweep

- [ ] Run the multi-term sweep:
      `rtk grep -R -in "ayokoding\|oseplatform\|organiclever\|hugo-commons" apps libs specs scripts infra archived .github .claude .opencode governance docs README.md CLAUDE.md AGENTS.md LICENSING-NOTICE.md package.json nx.json tsconfig.base.json | rg -v "plans/done/"`
- [ ] Output MUST be empty. If not empty, fix the residual hit, re-run, then proceed. Note: the `plans/done/` exclusion is retained as a safety net; post-cleanup `plans/done/` is empty until Phase 17 archives the current plan, then contains exactly one entry. The exclusion ensures future archived plans referencing product names historically will not trigger the sweep.
- [ ] Run the per-file markdown sweep:
      `xargs rtk grep -l "ayokoding\|oseplatform\|organiclever\|hugo-commons\|FSL-1.1-MIT" < local-temp/remaining-md-files.txt 2>/dev/null | rg -v "plans/done/"` — output MUST be empty
- [ ] Run `rtk grep -R -in "FSL-1.1-MIT" . --exclude-dir=plans/done --exclude-dir=.git`. Output MUST be empty. If not empty, fix and re-run.
- [ ] If residual fix produced changes: `rtk git commit -m "chore(cleanup): final sweep of product references"`

### Nx Graph Sanity

- [ ] `rtk npx nx show projects > local-temp/post-cleanup-nx-projects.txt`
- [ ] `rtk diff local-temp/pre-cleanup-nx-projects.txt local-temp/post-cleanup-nx-projects.txt` and confirm removed projects are gone and no unintended kept-project changes

### Governance Audit

- [ ] Invoke `repo-rules-checker` agent in normal strictness mode; report writes to `generated-reports/`
- [ ] Read the produced report
- [ ] If report lists any CRITICAL or HIGH findings, invoke `repo-rules-fixer` and re-run `repo-rules-checker`; loop until double-zero pass
- [ ] Commit any repo-rules-fixer changes per subgrouping with accurate Conventional Commits messages

### Push to main

- [ ] Run `rtk git log --oneline origin/main..HEAD` to review the full commit set about to land
- [ ] `rtk git push origin main` (Husky pre-push gate runs; must exit 0)
- [ ] Monitor GitHub Actions on the push via `rtk gh run list --branch main --limit 5`
- [ ] Stream the latest run: `rtk gh run watch`
- [ ] If any CI check fails, fix root cause, commit with accurate Conventional Commits message, and push follow-up
- [ ] Repeat until ALL CI checks pass green

## Phase 17 — Plan Archival

- [ ] Verify every checkbox above is ticked
- [ ] Verify `repo-rules-checker` is double-zero
- [ ] Verify post-push CI is green
- [ ] `rtk git mv plans/in-progress/2026-04-18__ose-primer-template-cleanup plans/done/2026-04-18__ose-primer-template-cleanup`
- [ ] Edit `plans/in-progress/README.md` — confirm no entry for this plan exists (should already be absent since Phase 6 never added one for this plan itself; the plan folder was there from creation but this plan's own entry line, if present, must be removed)
- [ ] Edit `plans/done/README.md` — replace the "_No completed plans yet in this template._" placeholder with the first archival entry (paste as a single bullet line; the relative link resolves from `plans/done/README.md`, not from this file):

```text
- [2026-04-18: ose-primer Template Cleanup](./2026-04-18__ose-primer-template-cleanup/README.md) — Strip all product-specific content (ayokoding, oseplatform, organiclever, hugo-commons) from the ose-primer repo so it functions as a clean repository template. Removed 12 apps, 3 spec trees, 1 deprecated lib, 3 archived product snapshots, 6 infra directories (5 infra/dev + 1 infra/k8s), 20 product agents (both .claude/ and .opencode/ mirrors), 3 product skills, 1 product plan, 4 CI workflow files (3 test-and-deploy + 1 orphan reusable). Rewrote CLAUDE.md, AGENTS.md, README.md, .claude/agents/README.md, LICENSING-NOTICE.md, and pruned governance + docs enumerations. Trunk-based direct push to main. Post-cleanup: zero product-brand grep hits outside plans/done/, nx affected + full run-many green, repo-rules-checker double-zero (Completed: 2026-04-18)
```

- [ ] Run `rtk npm run lint:md:fix`
- [ ] Commit: `rtk git commit -m "chore(plans): archive 2026-04-18__ose-primer-template-cleanup to done"`
- [ ] `rtk git push origin main`
- [ ] Confirm CI green on archival push via `rtk gh run watch`

## Quality Gates (Summary)

Gates that MUST pass at each commit boundary and at final push:

1. `rtk npm run doctor` — polyglot toolchain convergence
2. `rtk npx nx affected -t typecheck lint test:quick spec-coverage --base=HEAD~1` — between-phase affected gate
3. `rtk npx nx run-many -t typecheck lint test:quick spec-coverage` — full workspace gate (Phase 16)
4. `rtk npm run lint:md` — markdown lint
5. `docs-link-checker` — governance + docs link integrity (Phases 11 + 12)
6. `repo-rules-checker` in normal strictness mode — double-zero CRITICAL/HIGH (Phase 16)
7. GitHub Actions CI on `main` push — all checks green

**No bypass**: never pass `--no-verify`, `--force`, or any other flag that skips gates. If a gate fires, fix root cause in its own commit, then proceed.

### Commit Guidelines

- [ ] Commit changes thematically — group related changes into logically cohesive commits
- [ ] Follow Conventional Commits format: `<type>(<scope>): <description>`
- [ ] Split different domains/concerns into separate commits
- [ ] Do NOT bundle unrelated fixes into a single commit

## Verification (how to confirm this plan's outcome)

Three independent confirmations:

1. **File-system state**: `rtk npx nx show projects` lists only `a-demo-*` projects + `rhino-cli` + `a-demo-contracts` (if tracked as project) + kept libs. `ls apps/` does not list any `ayokoding-*`, `oseplatform-*`, or `organiclever-*`. `ls .claude/agents/ .claude/skills/` does not list any product agent / skill.
2. **Grep cleanliness**: `rtk grep -R -in "ayokoding\|oseplatform\|organiclever\|hugo-commons" apps libs specs scripts infra archived .github .claude .opencode governance docs README.md CLAUDE.md AGENTS.md LICENSING-NOTICE.md package.json nx.json tsconfig.base.json | rg -v "plans/done/"` returns empty. `rtk grep -R -in "FSL-1.1-MIT" . --exclude-dir=plans/done --exclude-dir=.git` returns empty.
3. **Build cleanliness**: a fresh `git clone git@github.com:wahidyankf/ose-primer.git && cd ose-primer && rtk npm install && rtk npm run doctor && rtk npx nx run-many -t typecheck lint test:quick spec-coverage` exits 0 end-to-end.

All three must hold simultaneously to consider the plan complete.
