---
title: "Delivery: Adopt ose-public Specs Structure"
description: Granular execution checklist for the specs/ C4-aware migration
category: plan
---

# Delivery — Adopt ose-public Specs Structure

## Environment Setup

- [ ] `npm install` — exits 0
  - _Executor: default_
- [ ] `npm run doctor -- --fix` — exits 0 (converges 18+ polyglot toolchains)
  - _Executor: default_
- [ ] `npx nx affected -t lint typecheck` — exits 0 (baseline; distinguish pre-existing failures
      from migration-caused regressions)
  - _Executor: default_

## Commit Guidelines

Follow Conventional Commits format. Commit thematically — each phase is its own commit; do
not bundle changes from different phases or unrelated domains. Per-phase commit messages are
specified inline at the end of each phase.

## Phase 0 — Pre-flight Verification

- [ ] Confirm exact rhino flat feature-file list:
      `ls specs/apps/rhino/behavior/cli/gherkin/*.feature | sort`
      Compare against [tech-docs.md §D1](./tech-docs.md#decision--d1-domain-groupings-for-rhino-cli-gherkin)
      domain grouping table. If filenames have drifted since 2026-05-24, assign new files
      to the closest domain before Phase 3.
  - _Executor: default_

- [ ] Confirm exact crud flat-root layout:
      `find specs/apps/crud -maxdepth 1 -type d | sort`
      Must still show `be/`, `fe/`, `c4/`, `contracts/` at root. If already migrated, skip
      Phase 2 entirely.
  - _Executor: default_

- [ ] Confirm the 17 project.json files still reference old paths:
      `grep -r 'specs/apps/crud/be/gherkin\|specs/apps/crud/fe/gherkin' --include='*.json' apps/ | wc -l`
      Expected: 20+ matches. If already updated, skip the project.json sub-tasks in Phase 2.
  - _Executor: default_

- [ ] Re-read the current `repo-governance/conventions/structure/specs-directory-structure.md`
      (ose-primer version) and [tech-docs.md §Gap Inventory](./tech-docs.md#gap-inventory).
      Confirm the inventory is still accurate — if the file has been updated since 2026-05-24,
      revise the plan before executing.
  - _Executor: default_

## Phase 1 — Convention Doc Replacement (repo-rules-maker)

Update `specs-directory-structure.md` to the C4-aware five-folder convention
matching ose-public's 2026-05-24 state. This step is decoupled from the file moves
so the convention is accurate before the structural commits land.

- [ ] Invoke `repo-rules-maker` to rewrite
      `repo-governance/conventions/structure/specs-directory-structure.md`:
  - Replace "Canonical Path Pattern" section with five-folder C4-aware tree
  - Drop CLI-flat exception; add "domain subdirs for every surface" rule
  - Add "Flat-Root to C4-Aware" migration mapping table (see ose-public version)
  - Change `{layer}` terminology to `<surface>` with enum `be`, `web`, `cli`
  - Replace `fe` references with `web` throughout
  - Add §Migration Path dated note: "ose-primer adoption (2026-05-24)"
  - Update all path examples to new canonical paths
  - Update frontmatter `description` to match new scope
  - _Suggested executor: `repo-rules-maker`_

- [ ] Verify `npm run lint:md` on the updated file — exits 0.
  - _Executor: default_

- [ ] Commit: `docs(governance): replace specs-directory-structure with C4-aware convention`
  - _Executor: default_

## Phase 2 — crud Flat-Root → C4-Aware Migration (one atomic commit)

All `git mv` operations and path-reference updates in this phase land in a SINGLE commit.
Do NOT push between the moves and the updates.

### 2.1 — Create destination directories

- [ ] `mkdir -p specs/apps/crud/product` — skeleton placeholder; no README required in this migration
- [ ] `mkdir -p specs/apps/crud/behavior`
- [ ] `mkdir -p specs/apps/crud/system-context`
- [ ] `mkdir -p specs/apps/crud/containers`
- [ ] `mkdir -p specs/apps/crud/components/be`
- [ ] `mkdir -p specs/apps/crud/components/web`

### 2.2 — git mv: layer directories into behavior/

- [ ] `git mv specs/apps/crud/be specs/apps/crud/behavior/be`
      — moves `be/README.md` and `be/gherkin/` intact
- [ ] `git mv specs/apps/crud/fe specs/apps/crud/behavior/web`
      — moves `fe/` and RENAMES it to `web/` in one step

### 2.3 — git mv: c4/ → split into canonical C4 folders

- [ ] `git mv specs/apps/crud/c4/context.md specs/apps/crud/system-context/context.md`
- [ ] `git mv specs/apps/crud/c4/container.md specs/apps/crud/containers/container.md`
- [ ] `git mv specs/apps/crud/c4/component-be.md specs/apps/crud/components/be/component-be.md`
- [ ] `git mv specs/apps/crud/c4/component-fe.md specs/apps/crud/components/web/component-web.md`
      — RENAMED from `component-fe.md` to `component-web.md`
- [ ] `git rm specs/apps/crud/c4/README.md`
      — c4/ is now empty; git rm the last file; directory is automatically removed

### 2.4 — git mv: contracts/ into containers/

- [ ] `git mv specs/apps/crud/contracts specs/apps/crud/containers/contracts`
      — moves the entire contracts directory (project.json, openapi.yaml, etc.)

### 2.5 — Create new README files

- [ ] Create `specs/apps/crud/behavior/README.md`
      — cross-cutting behavior index; list `be/` and `web/` sub-sections
  - _Suggested executor: `docs-maker`_
- [ ] Create `specs/apps/crud/system-context/README.md`
      — skeleton per R3 template in tech-docs.md
  - _Suggested executor: `docs-maker`_
- [ ] Create `specs/apps/crud/containers/README.md`
      — skeleton noting `container.md` + `contracts/` subdirectory
  - _Suggested executor: `docs-maker`_
- [ ] Create `specs/apps/crud/components/README.md`
      — skeleton listing `be/` and `web/` sub-components
  - _Suggested executor: `docs-maker`_
- [ ] Create `specs/apps/crud/components/be/README.md`
      — skeleton for backend component spec
  - _Suggested executor: `docs-maker`_
- [ ] Create `specs/apps/crud/components/web/README.md`
      — skeleton for web/frontend component spec
  - _Suggested executor: `docs-maker`_

### 2.6 — Update moved README files

- [ ] Update `specs/apps/crud/README.md`:
  - Replace old `c4/`, `be/`, `fe/`, `contracts/` tree block with five-folder tree
  - Update "Spec Artifacts" links to new paths
  - Rename `fe/` references to `behavior/web/`
  - _Suggested executor: `docs-maker`_
- [ ] Update `specs/apps/crud/behavior/be/README.md` (was `be/README.md`):
  - Update any self-referential path examples
  - _Suggested executor: `docs-maker`_
- [ ] Update `specs/apps/crud/behavior/web/README.md` (was `fe/README.md`):
  - Update `fe` → `web` in heading and path examples
  - _Suggested executor: `docs-maker`_
- [ ] Update `specs/apps/crud/behavior/be/gherkin/README.md`:
  - Update relative path to parent README (now 4 levels up instead of 3)
  - _Suggested executor: `docs-maker`_
- [ ] Update `specs/apps/crud/behavior/web/gherkin/README.md` (was `fe/gherkin/README.md`):
  - Update `fe` → `web` throughout
  - Update relative path depth
  - _Suggested executor: `docs-maker`_
- [ ] Update `specs/apps/crud/containers/contracts/README.md` (was `contracts/README.md`):
  - Update relative path depth (now one level deeper)
  - _Suggested executor: `docs-maker`_

### 2.7 — Update project.json Nx inputs and spec-coverage commands (17 files)

Each file has `specs/apps/crud/be/gherkin` or `specs/apps/crud/fe/gherkin` references:

- [ ] `apps/crud-be-clojure-pedestal/project.json`: `be/gherkin` → `behavior/be/gherkin`
- [ ] `apps/crud-be-csharp-aspnetcore/project.json`: `be/gherkin` → `behavior/be/gherkin`
- [ ] `apps/crud-be-e2e/project.json`: `be/gherkin` → `behavior/be/gherkin`
- [ ] `apps/crud-be-elixir-phoenix/project.json`: `be/gherkin` → `behavior/be/gherkin`
- [ ] `apps/crud-be-fsharp-giraffe/project.json`: `be/gherkin` → `behavior/be/gherkin`
- [ ] `apps/crud-be-golang-gin/project.json`: `be/gherkin` → `behavior/be/gherkin`
- [ ] `apps/crud-be-java-springboot/project.json`: `be/gherkin` → `behavior/be/gherkin`
- [ ] `apps/crud-be-java-vertx/project.json`: `be/gherkin` → `behavior/be/gherkin`
- [ ] `apps/crud-be-kotlin-ktor/project.json`: `be/gherkin` → `behavior/be/gherkin`
- [ ] `apps/crud-be-python-fastapi/project.json`: `be/gherkin` → `behavior/be/gherkin`
- [ ] `apps/crud-be-rust-axum/project.json`: `be/gherkin` → `behavior/be/gherkin`
- [ ] `apps/crud-be-ts-effect/project.json`: `be/gherkin` → `behavior/be/gherkin`
- [ ] `apps/crud-fe-dart-flutterweb/project.json`: `fe/gherkin` → `behavior/web/gherkin`
- [ ] `apps/crud-fe-e2e/project.json`: `fe/gherkin` → `behavior/web/gherkin`
- [ ] `apps/crud-fe-ts-nextjs/project.json`: `fe/gherkin` → `behavior/web/gherkin`
- [ ] `apps/crud-fe-ts-tanstack-start/project.json`: `fe/gherkin` → `behavior/web/gherkin`
- [ ] `apps/crud-fs-ts-nextjs/project.json`: both `be/gherkin` and `fe/gherkin` paths

### 2.8 — Verify and commit atomically

- [ ] Verify no flat-root artifacts remain:
      `find specs/apps/crud -maxdepth 1 -type d | sort`
      Must NOT include `be/`, `fe/`, `c4/`, `contracts/`.
- [ ] Verify no old project.json references remain:
      `grep -r 'specs/apps/crud/be/gherkin\|specs/apps/crud/fe/gherkin' --include='*.json' apps/ | wc -l`
      Must return 0.
- [ ] Verify no stale `contracts` path references remain in any `.json` file
      (the moved `contracts/project.json` may contain self-referential paths):
      `grep -r 'specs/apps/crud/contracts' --include='*.json' . | grep -v node_modules`
      Must return empty.
- [ ] Run `npm run lint:md` on changed spec files — exits 0.
- [ ] Commit atomically:
      `refactor(specs/crud): migrate to C4-aware five-folder tree; fe surface renamed to web`

## Phase 3 — rhino Fill-out + Domain Regrouping (one atomic commit)

All `git mv` operations and README creation in this phase land in a SINGLE commit.

### 3.1 — Create missing C4 folders

- [ ] `mkdir -p specs/apps/rhino/product`
- [ ] `mkdir -p specs/apps/rhino/system-context`
- [ ] `mkdir -p specs/apps/rhino/containers`
- [ ] `mkdir -p specs/apps/rhino/components/cli`

### 3.2 — Create CLI-gherkin domain subdirs

- [ ] `mkdir -p specs/apps/rhino/behavior/cli/gherkin/agents`
- [ ] `mkdir -p specs/apps/rhino/behavior/cli/gherkin/contracts`
- [ ] `mkdir -p specs/apps/rhino/behavior/cli/gherkin/docs`
- [ ] `mkdir -p specs/apps/rhino/behavior/cli/gherkin/env`
- [ ] `mkdir -p specs/apps/rhino/behavior/cli/gherkin/git`
- [ ] `mkdir -p specs/apps/rhino/behavior/cli/gherkin/java`
- [ ] `mkdir -p specs/apps/rhino/behavior/cli/gherkin/repo-governance`
- [ ] `mkdir -p specs/apps/rhino/behavior/cli/gherkin/spec-coverage`
- [ ] `mkdir -p specs/apps/rhino/behavior/cli/gherkin/system`
- [ ] `mkdir -p specs/apps/rhino/behavior/cli/gherkin/test-coverage`
- [ ] `mkdir -p specs/apps/rhino/behavior/cli/gherkin/workflows`

### 3.3 — git mv features into domain subdirs

- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/agents-sync.feature specs/apps/rhino/behavior/cli/gherkin/agents/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/agents-validate-claude.feature specs/apps/rhino/behavior/cli/gherkin/agents/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/agents-validate-naming.feature specs/apps/rhino/behavior/cli/gherkin/agents/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/contracts-dart-scaffold.feature specs/apps/rhino/behavior/cli/gherkin/contracts/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/contracts-java-clean-imports.feature specs/apps/rhino/behavior/cli/gherkin/contracts/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/docs-validate-links.feature specs/apps/rhino/behavior/cli/gherkin/docs/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/docs-validate-mermaid.feature specs/apps/rhino/behavior/cli/gherkin/docs/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/doctor.feature specs/apps/rhino/behavior/cli/gherkin/system/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/env-backup.feature specs/apps/rhino/behavior/cli/gherkin/env/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/env-init.feature specs/apps/rhino/behavior/cli/gherkin/env/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/env-restore.feature specs/apps/rhino/behavior/cli/gherkin/env/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/git-pre-commit.feature specs/apps/rhino/behavior/cli/gherkin/git/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/java-validate-annotations.feature specs/apps/rhino/behavior/cli/gherkin/java/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/repo-governance-vendor-audit.feature specs/apps/rhino/behavior/cli/gherkin/repo-governance/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/spec-coverage-validate.feature specs/apps/rhino/behavior/cli/gherkin/spec-coverage/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/test-coverage-diff.feature specs/apps/rhino/behavior/cli/gherkin/test-coverage/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/test-coverage-merge.feature specs/apps/rhino/behavior/cli/gherkin/test-coverage/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/test-coverage-validate.feature specs/apps/rhino/behavior/cli/gherkin/test-coverage/`
- [ ] `git mv specs/apps/rhino/behavior/cli/gherkin/workflows-validate-naming.feature specs/apps/rhino/behavior/cli/gherkin/workflows/`

- [ ] Verify no flat features remain:
      `find specs/apps/rhino/behavior/cli/gherkin -maxdepth 1 -name '*.feature'`
      Must return empty.

### 3.4 — Create skeleton READMEs for new C4 folders

- [ ] Create `specs/apps/rhino/product/README.md` — skeleton per R3 template
  - _Suggested executor: `specs-maker`_
- [ ] Create `specs/apps/rhino/system-context/README.md` — skeleton per R3 template
  - _Suggested executor: `specs-maker`_
- [ ] Create `specs/apps/rhino/containers/README.md` — skeleton per R3 template
  - _Suggested executor: `specs-maker`_
- [ ] Create `specs/apps/rhino/components/README.md` — skeleton listing `cli/` sub-component
  - _Suggested executor: `specs-maker`_
- [ ] Create `specs/apps/rhino/components/cli/README.md` — skeleton per R3 template
  - _Suggested executor: `specs-maker`_

### 3.5 — Create domain subdir README files

- [ ] Create `specs/apps/rhino/behavior/cli/gherkin/agents/README.md`
      — one-para index listing 3 features + their commands
  - _Suggested executor: `specs-maker`_
- [ ] Create `specs/apps/rhino/behavior/cli/gherkin/contracts/README.md`
      — one-para index listing 2 features
  - _Suggested executor: `specs-maker`_
- [ ] Create `specs/apps/rhino/behavior/cli/gherkin/docs/README.md`
      — one-para index listing 2 features
  - _Suggested executor: `specs-maker`_
- [ ] Create `specs/apps/rhino/behavior/cli/gherkin/env/README.md`
      — one-para index listing 3 features
  - _Suggested executor: `specs-maker`_
- [ ] Create `specs/apps/rhino/behavior/cli/gherkin/git/README.md`
      — one-para index listing 1 feature
  - _Suggested executor: `specs-maker`_
- [ ] Create `specs/apps/rhino/behavior/cli/gherkin/java/README.md`
      — one-para index listing 1 feature
  - _Suggested executor: `specs-maker`_
- [ ] Create `specs/apps/rhino/behavior/cli/gherkin/repo-governance/README.md`
      — one-para index listing 1 feature
  - _Suggested executor: `specs-maker`_
- [ ] Create `specs/apps/rhino/behavior/cli/gherkin/spec-coverage/README.md`
      — one-para index listing 1 feature
  - _Suggested executor: `specs-maker`_
- [ ] Create `specs/apps/rhino/behavior/cli/gherkin/system/README.md`
      — one-para index listing 1 feature (doctor)
  - _Suggested executor: `specs-maker`_
- [ ] Create `specs/apps/rhino/behavior/cli/gherkin/test-coverage/README.md`
      — one-para index listing 3 features
  - _Suggested executor: `specs-maker`_
- [ ] Create `specs/apps/rhino/behavior/cli/gherkin/workflows/README.md`
      — one-para index listing 1 feature
  - _Suggested executor: `specs-maker`_

### 3.6 — Update existing README files

- [ ] Update `specs/apps/rhino/README.md`:
  - Add all five top-level folders to Structure block
  - Update "Adding New Specs" step to use `<domain>/` path
  - Update spec-coverage command example to use new path pattern
  - _Suggested executor: `docs-maker`_
- [ ] Update `specs/apps/rhino/behavior/README.md`:
  - Update Structure block to mention all 11 domain subdirs
  - _Suggested executor: `docs-maker`_
- [ ] Update `specs/apps/rhino/behavior/cli/gherkin/README.md`:
  - Replace flat feature table with per-domain tables matching ose-public's style
  - Update Structure block to list all 11 domain subdirs
  - _Suggested executor: `docs-maker`_

### 3.7 — Sweep per-file path references

- [ ] Run reference sweep and hand-rewrite per-feature path references:

  ```bash
  grep -rln 'specs/apps/rhino/behavior/cli/gherkin/' \
    repo-governance docs .github .husky apps \
    > /tmp/rhino-spec-refs.txt
  cat /tmp/rhino-spec-refs.txt
  ```

  Inspect each file. Update any that reference individual feature files by path
  (e.g., `bdd-spec-test-mapping.md` mentions `agents-sync.feature` by full path).
  - _Executor: default_

### 3.8 — Verify and commit atomically

- [ ] Verify `find specs/apps/rhino -maxdepth 1 -type d | sort` shows five-folder profile:
      `product/`, `system-context/`, `containers/`, `components/`, `behavior/`.
- [ ] Run `npm run lint:md` on changed files — exits 0.
- [ ] Commit atomically:
      `refactor(specs/rhino): fill out CLI-only tree and regroup features into domain subdirs`

## Phase 4 — Governance Doc Propagation (repo-rules-maker)

Propagate structural changes into governance docs, agent definitions, and the active plan.
Delegate the full sweep to `repo-rules-maker`.

- [ ] Invoke `repo-rules-maker` with the following brief:
      Phases 2–3 have landed: `specs/apps/crud/` is now on the C4-aware five-folder tree (fe
      renamed to web, c4/ split, contracts/ moved inside containers/), and
      `specs/apps/rhino/behavior/cli/gherkin/` now uses domain subdirs. Update all remaining
      governance surfaces:
  1. `repo-governance/development/infra/bdd-spec-test-mapping.md` — update path examples
     (`crud/be/gherkin/` → `crud/behavior/be/gherkin/`; flat rhino paths → domain-subdir paths
     per [tech-docs.md §D1](./tech-docs.md#decision--d1-domain-groupings-for-rhino-cli-gherkin))
  2. `repo-governance/development/infra/ci-conventions.md` — update `crud/be/gherkin/` and
     `crud/fe/gherkin/` references
  3. `repo-governance/development/infra/nx-targets.md` — update `crud/be/gherkin/` and
     `crud/fe/gherkin/` references
  4. `repo-governance/development/quality/specs-application-sync.md` — update `crud/c4/`,
     `crud/be/gherkin/`, `crud/fe/gherkin/` references
  5. `repo-governance/development/quality/feature-change-completeness.md` — update `crud/be/`
     and `crud/fe/` references
  6. `repo-governance/development/quality/three-level-testing-standard.md` — update
     `crud/be/gherkin/` and `crud/fe/gherkin/` references
  7. `repo-governance/workflows/specs/specs-quality-gate.md` — update `specs/apps/crud/be/`
     path examples
  8. `repo-governance/conventions/formatting/diagrams.md` — update `specs/apps/crud/c4/`
     reference
  9. `repo-governance/conventions/writing/dynamic-collection-references.md` — update
     `crud/be/gherkin/` examples
  10. `specs/README.md` — rewrite "Standard Folder Pattern" section to show five-folder tree;
      update "App Specs" section links for crud and rhino
  11. `README.md` (root) — update the minor crud spec path reference
  12. `plans/in-progress/add-investment-oracle-app/README.md` — update `crud/c4/` → new path,
      `crud/contracts/` → `crud/containers/contracts/`, `crud/be/gherkin/` and `crud/fe/gherkin/`
  13. `plans/in-progress/add-investment-oracle-app/tech-docs.md` — same
  14. `plans/in-progress/add-investment-oracle-app/delivery.md` — same
  15. `.claude/agents/specs-checker.md` and `.opencode/agents/specs-checker.md` — update any
      path examples that reference old `be/`/`fe/`/`c4/` or flat CLI paths
  16. `.claude/agents/specs-maker.md` and `.opencode/agents/specs-maker.md` — same
  17. `.claude/agents/specs-fixer.md` and `.opencode/agents/specs-fixer.md` — same

  **Exclusions**: Do NOT modify `specs/apps/crud/` or `specs/apps/rhino/` (already migrated);
  do NOT modify `plans/done/` (historical); do NOT modify `generated-reports/` (historical);
  do NOT introduce new conventions.
  - _Suggested executor: `repo-rules-maker`_

- [ ] Verify `repo-rules-maker` only touched files within its authorized scope
      (`repo-governance/`, `specs/README.md`, `README.md`, `plans/in-progress/`, `.claude/agents/`,
      `.opencode/agents/`). If it touched anything else, reject and re-invoke with tighter scope.
  - _Executor: default_

- [ ] Run `npm run sync:claude-to-opencode` — exits 0; diff shows only agent mirror updates.
  - _Executor: default_

- [ ] Run `npm run lint:md` — exits 0 across all updated files.
  - _Executor: default_

- [ ] Commit governance changes:
      `docs(governance): propagate specs-tree-uniform paths to conventions and plans`
  - _Executor: default_

## Phase 5 — Quality Gates

- [ ] Run `npm run lint:md` across the full repo — exits 0.
  - _Executor: default_
- [ ] Verify spec-coverage commands still resolve for a sample crud-be app:
      `npx nx run crud-be-golang-gin:spec-coverage` — exits 0.
  - _Executor: default_
- [ ] Verify spec-coverage for rhino:
      `npx nx run rhino-cli:spec-coverage` — exits 0.
      (The rhino spec-coverage command uses a directory glob `**/*.feature` so it handles
      subdirs automatically; verify the exit code anyway.)
  - _Executor: default_
- [ ] Verify no old stale paths remain in any tracked file:

  ```bash
  grep -r 'specs/apps/crud/be/gherkin\|specs/apps/crud/fe/gherkin\|specs/apps/crud/c4\|specs/apps/crud/contracts[^/]' \
    --include='*.md' --include='*.json' --include='*.yaml' . \
    | grep -v node_modules | grep -v worktrees | grep -v generated-reports | grep -v 'plans/done'
  ```

  Must return empty (or only hits inside this plan's own files, which describe old paths).
  - _Executor: default_

- [ ] Fix ALL failures found — including any pre-existing issues surfaced during quality gates.
  - _Executor: default_

## Phase 6 — Commit and Push

- [ ] Verify `git status` shows only expected unstaged files (no accidental uncommitted
      changes from prior phases, no unrelated modifications).
  - _Executor: default_
- [ ] Commit any remaining changes with message:
      `chore(specs): finalize adopt-ose-public-specs-structure migration`
  - _Executor: default_
- [ ] `git push origin main` — exits 0.
  - _Executor: default_
- [ ] Monitor GitHub Actions: open `https://github.com/wahidyankf/ose-primer/actions` and
      confirm all workflow runs triggered by the push succeed. If any fail, fix the root
      cause before advancing to Phase 7.
  - _Executor: default_

## Phase 7 — Plan Quality Gate

- [ ] Run [plan quality gate workflow](../../../repo-governance/workflows/plan/plan-quality-gate.md)
      against `plans/in-progress/adopt-ose-public-specs-structure/`.
  - _Suggested executor: `plan-execution-checker`_
- [ ] Address all CRITICAL and HIGH findings from the quality gate.
  - _Executor: default_
- [ ] Commit any fixes: `docs(plans): address plan-quality-gate findings for adopt-ose-public-specs-structure`
  - _Executor: default_
- [ ] `git push origin main` — exits 0.
  - _Executor: default_

## Plan Archival

_(Execute only when all delivery tasks above are ticked and CI is green.)_

- [ ] Verify ALL delivery checklist items above are ticked.
- [ ] `git mv plans/in-progress/adopt-ose-public-specs-structure plans/done/2026-05-24__adopt-ose-public-specs-structure`
      (use actual completion date)
- [ ] Update `plans/in-progress/README.md` — remove this plan's entry.
- [ ] Update `plans/done/README.md` — add this plan's entry with completion date and one-line summary.
- [ ] Commit: `chore(plans): move adopt-ose-public-specs-structure to done`
- [ ] `git push origin main`
