# PRD: Rename `demo-*` → `crud-*`

## Product overview

A pure rename: all `demo-*` apps, specs, infra, CI workflows, and documentation become
`crud-*`. No behavioral or API change occurs. The product surface visible to template
consumers changes only in naming — `apps/` listings, Nx project names, CI workflow file
names, and all documentation references.

## Product scope

This plan covers the complete rename of all `demo-*` artifacts to `crud-*` across the
ose-primer repository. It does not create new apps, change app functionality, or modify
git history.

## Personas

- **Template consumer (maintainer hat)**: clones ose-primer to bootstrap a new repo;
  needs immediately readable family names in `apps/` to understand the purpose of each
  demo family.
- **AI development agent (plan-executor / swe-\* dev)**: reads project names to orient
  codegen, test, and lint targets; needs consistent project names in the Nx workspace so
  that generated `nx run` commands are always correct.
- **CI system**: runs per-app test workflows keyed on workflow filename and internal
  `backend-name:` / `frontend-name:` fields; needs workflow files to match project names.

## User stories

- As a template consumer, I want `apps/` to list `crud-*` directories So that I can
  distinguish CRUD demos from future AI chat or realtime families at a glance.
- As an AI development agent, I want all Nx project names to say `crud-*` So that `nx run`
  commands I generate are always correct.
- As a CI system, I want `.github/workflows/` to have `test-crud-*.yml` filenames So that
  workflow names match the project names they test.

## Functional requirements

> See [tech-docs.md](./tech-docs.md) for the complete file change map and affected-file-categories table.

1. Every `apps/demo-*` directory is renamed to `apps/crud-*`.
2. Every `infra/dev/demo-*` directory is renamed to `infra/dev/crud-*`.
3. `specs/apps/demo/` tree is moved to `specs/apps/crud/`.
4. All 18 Nx project names (`"name"` in `project.json`) change from `demo-*` to `crud-*`.
5. All cross-project Nx references (`implicitDependencies`, `dependsOn`) updated to `crud-*`.
6. All target command strings referencing `apps/demo-*/`, `infra/dev/demo-*/`,
   `specs/apps/demo/` updated to the new paths.
7. Docker Compose DB credential env vars (`demo_be_*`) renamed to `crud_be_*` in all
   `docker-compose*.yml` files.
8. Root `package.json` npm scripts renamed: `dev:demo-*` → `dev:crud-*`,
   `demo-be:*` → `crud-be:*`.
9. OpenAPI contract internal metadata ("demo API", "demo application") updated to
   reference "crud".
10. All Gherkin `.feature` files audited; any `demo-` references updated.
11. All C4 diagram files in `specs/apps/crud/c4/` audited; `demo-` references updated.
12. All `README.md` files inside the moved spec tree updated.
13. Governance workflows (`governance/workflows/`) audited and updated.
14. Governance convention docs (`governance/conventions/`) audited and updated.
15. `docs/` tree (tutorials, how-to, reference, explanation) audited and updated.
16. `CLAUDE.md`, root `README.md`, and `AGENTS.md` updated.
17. Active plans (`plans/ideas.md`, `plans/backlog/`) audited and updated; `plans/done/`
    left untouched (historical records).
18. All `.github/workflows/test-demo-*.yml` files renamed to `test-crud-*.yml` via
    `git mv`; all internal `backend-name:` / `frontend-name:` with-block fields updated
    to `crud-*`; hardcoded `demo-be-golang-gin`, `demo-contracts:bundle`, and similar
    strings in reusable workflow files updated.

## Product risks

| Risk                                                                                 | Likelihood | Mitigation                                                                  |
| ------------------------------------------------------------------------------------ | ---------- | --------------------------------------------------------------------------- |
| A renamed Gherkin feature file breaks spec-coverage path resolution                  | Low        | Phase 9 Gherkin audit + Phase 22 spec-coverage validation                   |
| A renamed workflow file breaks CI on main after push                                 | Medium     | Dedicated `.github/workflows/` rename phase + post-push CI monitoring step  |
| Flutter codegen pipeline references stale `demo_contracts` package name post-rename  | Medium     | Phase 5 extended sweep covers `pubspec.yaml`; Phase 21 codegen re-runs      |
| Missed reference in `.md` documentation not caught by Phase 19 stale-reference audit | Low        | Phase 19 grep extended to include `--include="*.md"` for all demo- patterns |

## Non-requirements

- No change to app functionality or API behaviour.
- No rename of abbreviation suffixes in DB credentials (`cjpd`, `csas`, `exph`,
  `fsgi`, `ggn`, `jasp`, `javx`, `ktkt`, `pyfp`, `rsax`, `tsex`) — only the `demo_`
  prefix changes.
- No rewrite of git history.
- No modification of `plans/done/` or `generated-reports/`.

## Acceptance criteria

```gherkin
Feature: Rename demo-* to crud-* across the entire ose-primer repository

  Background:
    Given all rename tasks in delivery.md are completed
    And the changes are pushed to origin main

  Scenario: Nx workspace recognises all crud-* projects
    Given the workspace root contains the updated nx.json and all project.json files
    When I run "npx nx graph --file=output.json"
    Then all 18 projects appear with "crud-*" names
    And no project appears with a "demo-*" name

  Scenario: Contract codegen pipeline succeeds end-to-end
    Given "crud-contracts" project exists at "specs/apps/crud/contracts"
    When I run "npx nx run-many -t codegen --projects=crud-be-*,crud-fe-*,crud-fs-*"
    Then generated type files exist under each "apps/crud-*/generated-contracts/"
    And the exit code is 0

  Scenario: All quick tests pass after rename
    Given all app source paths and import references are updated
    When I run "npx nx affected -t test:quick"
    Then every affected project exits 0
    And coverage thresholds are met per CLAUDE.md table

  Scenario: No demo- strings remain in workspace configuration files
    Given all renaming tasks are complete
    When I search project.json, package.json, docker-compose*.yml, and nx.json
    Then no "demo-" string is found
    And no "specs/apps/demo/" path is found
    And no "apps/demo-" path is found

  Scenario: Spec coverage validates for be-e2e project
    Given all rename tasks in delivery.md are completed
    When I run "npx nx run crud-be-e2e:spec-coverage"
    Then exit code is 0

  Scenario: Spec coverage validates for fe-e2e project
    Given all rename tasks in delivery.md are completed
    When I run "npx nx run crud-fe-e2e:spec-coverage"
    Then exit code is 0

  Scenario: Markdown quality gate passes
    Given all rename edits to markdown files are complete
    When I run "npm run lint:md" locally
    Then exit code is 0
    And no markdownlint violation is reported

  Scenario: Governance workflows contain no stale demo- references
    Given all governance workflow files in "governance/workflows/" are updated
    When I search "governance/workflows/" for "demo-"
    Then no matches are found

  Scenario: Documentation files contain no stale demo- references
    Given all doc files in "docs/" are updated
    When I search "docs/" for "demo-be-" or "demo-fe-" or "demo-fs-"
    Then no matches are found

  Scenario: GitHub Actions workflow files are renamed
    Given all renaming is complete
    When I list files in ".github/workflows/"
    Then no file is named "test-demo-*.yml"
    And each "test-crud-*.yml" file references "crud-*" project names internally
```
