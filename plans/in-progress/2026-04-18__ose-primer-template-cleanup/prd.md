# Product Requirements Document — ose-primer Template Cleanup

## Product Overview

The cleaned `ose-primer` repository is a **cloneable / cherry-pickable repository template** whose contents are exclusively:

- Polyglot `a-demo-*` scaffolding — 11 backend language variants, 3 frontend variants, 1 fullstack variant, contracts spec, E2E harness.
- Repository tooling — `rhino-cli` (doctor, agents sync, spec coverage, test coverage, naming validators).
- Shared libs — `golang-commons`, `elixir-cabbage`, `elixir-gherkin`, `elixir-openapi-codegen`, `clojure-openapi-codegen`, `ts-ui`, `ts-ui-tokens`.
- Governance, conventions, development standards, principles, workflows.
- Generic agents (plan, docs, swe-\*, repo-rules, readme, ci, social) and generic skills (docs, plan, repo, swe-programming-\*).
- Planning infrastructure (`plans/` folders, ideas, this plan's own entry — moving to `done/` at the end).

Nothing in the cleaned repo names AyoKoding, OSE Platform, OrganicLever, or Hugo outside of historical `plans/done/` entries.

## Personas

Solo-maintainer repo — each persona is a hat the maintainer wears, or an agent that consumes the cleaned artefacts.

- **Template maintainer** — wears the hat while executing this plan and every subsequent change to the template.
- **Future cloner** — bootstraps a new repo from `ose-primer` via `git clone` or cherry-pick.
- **plan-executor agent** — reads `delivery.md` to drive execution.
- **plan-checker / plan-execution-checker agent** — validates plan conformance against `governance/conventions/structure/plans.md`.
- **repo-rules-checker agent** — audits cleaned repo for governance consistency at the end.

## User Stories

### US-1 — Demo scaffolding intact

As a **future cloner**, I want every `a-demo-*` app retained so that I can delete only the language variants I don't need and start from a working polyglot scaffold.

### US-2 — No product-brand residue

As a **future cloner**, I want zero references to `ayokoding`, `oseplatform`, `organiclever`, or `hugo-commons` in the template so that brand-specific assumptions don't leak into my downstream repo.

### US-3 — Fresh install works

As a **template maintainer**, I want `npm install` and `npm run doctor` to succeed on a fresh clone of the cleaned `main` so that the repo is immediately usable.

### US-4 — Affected quality gates pass

As a **template maintainer**, I want `nx affected -t typecheck lint test:quick spec-coverage` to exit 0 for all kept projects so that the pre-push gate is satisfied and the cleanup can be pushed to `main`.

### US-5 — Agent and skill catalogs only list generic automation

As a **template maintainer**, I want `.claude/agents/` and `.claude/skills/` (and their `.opencode/` mirrors) to contain only repo-agnostic agents/skills so that cloned repos don't inherit deprecated product automation.

### US-6 — Governance audit clean

As a **template maintainer**, I want `repo-rules-checker` to report zero CRITICAL and zero HIGH findings after cleanup so that the template passes its own governance bar.

### US-7 — Root README explains the template to a first-time cloner

As a **future cloner**, I want the root `README.md` to explain what this repo ships, how to clone it as a template, and what the main entry points are, so that I can get oriented without reading `CLAUDE.md` first.

### US-8 — Empty plans history for template

As a **future cloner**, I want `plans/` to ship empty (no backlog folders, no archived plans, ideas file reset to template-generic placeholders) so that my new repo starts from a clean planning slate without inheriting the template's own development history.

### US-9 — Every surviving markdown file is scrubbed

As a **template maintainer**, I want every remaining `.md` file (app READMEs, lib READMEs, kept agent/skill bodies, spec READMEs, governance, docs, infra/dev READMEs) scrubbed of product-brand references, so that no reader encounters leftover product context no matter which file they open first.

## Acceptance Criteria (Gherkin)

### AC-1 — Demo scaffolding intact

```gherkin
Scenario: All a-demo-* apps remain after cleanup
  Given the cleanup execution has completed
  When I run "ls apps/"
  Then the output includes "a-demo-be-clojure-pedestal"
  And the output includes "a-demo-be-csharp-aspnetcore"
  And the output includes "a-demo-be-e2e"
  And the output includes "a-demo-be-elixir-phoenix"
  And the output includes "a-demo-be-fsharp-giraffe"
  And the output includes "a-demo-be-golang-gin"
  And the output includes "a-demo-be-java-springboot"
  And the output includes "a-demo-be-java-vertx"
  And the output includes "a-demo-be-kotlin-ktor"
  And the output includes "a-demo-be-python-fastapi"
  And the output includes "a-demo-be-rust-axum"
  And the output includes "a-demo-be-ts-effect"
  And the output includes "a-demo-fe-dart-flutterweb"
  And the output includes "a-demo-fe-e2e"
  And the output includes "a-demo-fe-ts-nextjs"
  And the output includes "a-demo-fe-ts-tanstack-start"
  And the output includes "a-demo-fs-ts-nextjs"
  And the output includes "rhino-cli"
```

### AC-2 — No product-brand residue

```gherkin
Scenario: Product-brand grep sweep returns empty
  Given the cleanup execution has completed
  When I run "rtk grep -R -in 'ayokoding|oseplatform|organiclever|hugo-commons' apps libs specs scripts infra archived .github .claude .opencode governance docs README.md CLAUDE.md AGENTS.md LICENSING-NOTICE.md package.json nx.json tsconfig.base.json | rg -v 'plans/done/'"
  Then the command exits with status 0
  And the output is empty

Scenario: Product apps are absent
  Given the cleanup execution has completed
  When I run "ls apps/"
  Then the output does not include "ayokoding-web"
  And the output does not include "ayokoding-cli"
  And the output does not include "oseplatform-web"
  And the output does not include "oseplatform-cli"
  And the output does not include "organiclever-fe"
  And the output does not include "organiclever-be"
```

### AC-3 — Fresh install works

```gherkin
Scenario: Fresh install succeeds
  Given a fresh checkout of "main" on the "wahidyankf/ose-primer" remote after cleanup
  When I run "npm install"
  Then the command exits with status 0
  When I run "npm run doctor"
  Then the command exits with status 0
```

### AC-4 — Affected quality gates pass

```gherkin
Scenario: Affected gates pass after every phase commit
  Given a phase commit is about to land on "main"
  When I run "npx nx affected -t typecheck lint --base=HEAD~1"
  Then the command exits with status 0

Scenario: Full gates pass at cleanup completion
  Given every cleanup phase has been committed
  When I run "npx nx run-many -t typecheck lint test:quick spec-coverage"
  Then the command exits with status 0
```

### AC-5 — Agent and skill catalogs only list generic automation

```gherkin
Scenario: Product-specific agents are absent from .claude and .opencode
  Given the cleanup execution has completed
  When I run "ls .claude/agents/"
  Then the output does not include any file starting with "apps-ayokoding-web-"
  And the output does not include any file starting with "apps-oseplatform-web-"
  And the output does not include "apps-organiclever-fe-deployer.md"
  And the output does not include "swe-hugo-dev.md"
  When I run "ls .opencode/agent/"
  Then the output does not include any file starting with "apps-ayokoding-web-"
  And the output does not include any file starting with "apps-oseplatform-web-"
  And the output does not include "apps-organiclever-fe-deployer.md"
  And the output does not include "swe-hugo-dev.md"

Scenario: Product-specific skills are absent
  Given the cleanup execution has completed
  When I run "ls .claude/skills/"
  Then the output does not include "apps-ayokoding-web-developing-content"
  And the output does not include "apps-organiclever-fe-developing-content"
  And the output does not include "apps-oseplatform-web-developing-content"
  When I run "ls .opencode/skill/"
  Then the output does not include "apps-ayokoding-web-developing-content"
  And the output does not include "apps-organiclever-fe-developing-content"
  And the output does not include "apps-oseplatform-web-developing-content"

Scenario: Agent catalog README only lists kept agents
  Given the cleanup execution has completed
  When I read ".claude/agents/README.md"
  Then it does not contain the strings "apps-ayokoding-web-", "apps-oseplatform-web-", "apps-organiclever-fe-deployer", or "swe-hugo-dev"
```

### AC-6 — Governance audit clean

```gherkin
Scenario: repo-rules-checker reports clean
  Given the cleanup execution has completed
  When I invoke the "repo-rules-checker" agent in normal strictness mode
  Then the final report lists zero CRITICAL findings
  And the final report lists zero HIGH findings

Scenario: Nx project graph matches expected kept set
  Given the cleanup execution has completed
  When I run "npx nx show projects"
  Then every listed project is either an "a-demo-*" project or one of "rhino-cli", "a-demo-contracts", "golang-commons", "elixir-cabbage", "elixir-gherkin", "elixir-openapi-codegen", "clojure-openapi-codegen", "ts-ui", "ts-ui-tokens"
  And no listed project matches "ayokoding-*", "oseplatform-*", "organiclever-*", or "hugo-commons"
```

### AC-7 — Uniform MIT license across the repo

```gherkin
Scenario: LICENSING-NOTICE.md declares uniform MIT
  Given the cleanup execution has completed
  When I read "LICENSING-NOTICE.md"
  Then it states that the entire repository is licensed under MIT
  And it does not mention "FSL-1.1-MIT" outside of historical context
  And it does not declare any per-directory license split (apps vs libs vs specs)

Scenario: LICENSE file contains MIT license text
  Given the cleanup execution has completed
  When I read "LICENSE"
  Then the text is the standard MIT license template
  And the copyright line names the repository owner

Scenario: package.json license field is MIT
  Given the cleanup execution has completed
  When I parse "package.json"
  Then the top-level "license" field equals "MIT"
```

### AC-8 — Markdown lint clean

```gherkin
Scenario: Markdown lint passes
  Given the cleanup execution has completed
  When I run "npm run lint:md"
  Then the command exits with status 0
```

### AC-9 — Plan archival

```gherkin
Scenario: Plan is archived to done at cleanup completion
  Given every cleanup phase has been committed and pushed
  And Phase 17 archival has run
  When I run "ls plans/in-progress/"
  Then the output does not include "2026-04-18__ose-primer-template-cleanup"
  When I run "ls plans/done/"
  Then the output contains "2026-04-18__ose-primer-template-cleanup"
  And the output does not contain any other YYYY-MM-DD__* entry (this cleanup plan is the only archived entry)
  When I read "plans/in-progress/README.md"
  Then the entry for "2026-04-18__ose-primer-template-cleanup" is absent
  When I read "plans/done/README.md"
  Then an entry for "2026-04-18__ose-primer-template-cleanup" exists with completion date "2026-04-18"
  And the "_No completed plans yet in this template._" placeholder is no longer present
```

### AC-10 — Root README contains required template sections and no product-brand strings

```gherkin
Scenario: Rewritten README contains required template sections
  Given the cleanup execution has completed
  When I read "README.md"
  Then it contains a section titled "What this is"
  And it contains a section titled "How to use this template"
  And it contains a section titled "License"
  And it contains a section titled "What it ships"
  And it contains a section titled "Prerequisites"
  And it contains a section titled "Common commands"

Scenario: Rewritten README contains no product-brand strings
  Given the cleanup execution has completed
  When I run "rtk grep -in 'ayokoding\|oseplatform\|organiclever\|hugo-commons' README.md"
  Then the command exits with status 0
  And the output is empty
```

### AC-11 — Empty plans history for template

```gherkin
Scenario: plans/in-progress contains only the current cleanup plan pre-archival
  Given the cleanup execution has completed but Phase 17 archival has not yet run
  When I run "ls plans/in-progress/"
  Then the output contains "2026-04-18__ose-primer-template-cleanup"
  And the output does not contain "2026-04-16__organiclever-fe-local-first"
  And the output does not contain any other YYYY-MM-DD__* entries

Scenario: plans/done is empty pre-archival
  Given the cleanup execution has completed but Phase 17 archival has not yet run
  When I run "ls plans/done/"
  Then the output contains "README.md"
  And the output does not contain any YYYY-MM-DD__* entries

Scenario: plans/backlog is empty
  Given the cleanup execution has completed
  When I run "ls plans/backlog/"
  Then the output contains "README.md"
  And the output does not contain any YYYY-MM-DD__* entries

Scenario: plans/ideas.md has no product-specific entries
  Given the cleanup execution has completed
  When I read "plans/ideas.md"
  Then it does not contain "ayokoding"
  And it does not contain "oseplatform"
  And it does not contain "organiclever"
  And it does not contain "FSL"
```

### AC-12 — Every surviving markdown file is scrubbed

```gherkin
Scenario: All remaining markdown files are product-clean
  Given the cleanup execution has completed
  When I run "rtk find apps libs specs infra apps-labs archived .claude .opencode governance docs plans -name '*.md' -type f | xargs rtk grep -l 'ayokoding\|oseplatform\|organiclever\|hugo-commons\|FSL-1.1-MIT' 2>/dev/null | rg -v 'plans/done/'"
  Then the output is empty

Scenario: Each kept app has a product-clean README
  Given the cleanup execution has completed
  When I list every "apps/*/README.md" file
  Then none contains "ayokoding", "oseplatform", "organiclever", "hugo-commons", or "FSL-1.1-MIT"

Scenario: Each kept lib has a product-clean README
  Given the cleanup execution has completed
  When I list every "libs/*/README.md" file
  Then none contains "ayokoding", "oseplatform", "organiclever", "hugo-commons", or "FSL-1.1-MIT"

Scenario: Every kept agent body is product-clean
  Given the cleanup execution has completed
  When I list every ".claude/agents/*.md" file
  Then none contains "ayokoding", "oseplatform", "organiclever", "hugo-commons", "FSL-1.1-MIT", or "swe-hugo-dev"

Scenario: Every kept skill body is product-clean
  Given the cleanup execution has completed
  When I list every ".claude/skills/**/*.md" file
  Then none contains "ayokoding", "oseplatform", "organiclever", "hugo-commons", or "FSL-1.1-MIT"
```

## Product Scope

### In Scope

- Deletion of 12 product apps, 3 spec trees, 1 deprecated lib, 3 archived product apps, 6 infra configs (5 `infra/dev/` + 1 `infra/k8s/organiclever/`), 20 product agents (in `.claude/` + `.opencode/`), 3 product skills (in `.claude/` + `.opencode/`), 54 plans (53 archived + 1 product in-progress), ideas.md reset, 4 CI workflow files (3 product `test-and-deploy-*.yml` + 1 orphan reusable `_reusable-test-and-deploy.yml`).
- Rewrite of `CLAUDE.md`, `AGENTS.md`, top-level `README.md`, `.claude/agents/README.md`, `.claude/settings.json`, `LICENSING-NOTICE.md`.
- Audit and prune of governance enumerations (`governance/**`) and Diátaxis docs (`docs/**`) that reference removed products.
- Removal of product-specific scripts from `package.json`.
- Audit of `nx.json` and `tsconfig.base.json` for dangling references.
- Sync of `.opencode/` from `.claude/` via `npm run sync:claude-to-opencode`.
- Plan archival from `plans/in-progress/` to `plans/done/`.

### Out of Scope

- Adding new demo languages or new variants to the `a-demo-*` suite.
- Restructuring `governance/` or `docs/` topology.
- Creating new agents or skills.
- Deploy-time env-branch management on the new `wahidyankf/ose-primer` remote (no env branches exist).
- Any modification to `ose-public` or `ose-infra`.
- Rewriting the README of every kept app/lib as part of this plan (those are owned by their normal maintenance cycles).

## Product Risks

### PR-1 — Broken internal links after doc pruning

- **Risk**: deleting a product-scoped Diátaxis tutorial or governance doc orphans links elsewhere.
- **Mitigation**: run `docs-link-checker` agent on `governance/` after Phase 12 and on `docs/` after Phase 13; fix broken links in the same phase commit.

### PR-2 — Markdown lint failures in rewritten files

- **Risk**: heading-nesting or line-length violations sneak in during `CLAUDE.md` / `README.md` / `AGENTS.md` rewrites.
- **Mitigation**: run `npm run lint:md:fix` at the end of each rewrite phase and confirm `npm run lint:md` exits 0 before the phase commit.

### PR-3 — A kept `a-demo-*` app imports something from a removed lib

- **Risk**: unexpected compile-time breakage after `hugo-commons` removal.
- **Mitigation**: Phase 3 grep (`rtk grep -r hugo-commons apps/ libs/`) must return empty before committing; `nx affected -t typecheck lint` must pass.

### PR-4 — A removed agent is invoked by a workflow still present

- **Risk**: `governance/workflows/**` still names a deleted agent in its orchestration, producing a broken workflow definition.
- **Mitigation**: Phase 12 governance grep covers `governance/workflows/**`; any named-agent reference to a deleted agent is rewritten or the workflow file is deleted.
