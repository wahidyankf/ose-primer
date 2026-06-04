# Product Requirements — Adopt Dependency Bump Policy & Planning Workflow

## Product Overview

Deliver, into `ose-primer`, the governance documents and supporting rules that let the repository
classify, clear, and plan dependency bumps according to the upstream `ose-public` standard. The
deliverable is **documentation plus a minimal validator change** — not any dependency edit.

## Personas

- **Maintainer (human)** — needs an authoritative, in-repo policy to apply during dependency review
  and a repeatable planning sweep to schedule bumps.
- **AI agent (planning/execution)** — needs machine-checkable rules (exact-pin grep, CVE sources,
  KEV/EPSS lookups, clearance statuses) and a workflow file that passes naming validation.

## User Stories

1. As a maintainer, I want a written three-path policy so that every bump decision is classified
   and auditable.
2. As an AI agent, I want a `repo` planning workflow file that conforms to the naming convention so
   that `rhino-cli workflows validate-naming` passes in pre-push and CI.
3. As a maintainer, I want a waiver register so that Path C / FUNCTIONAL-HOLD / KEV-listed
   decisions accumulate in one long-lived place.
4. As an AI agent, I want a documented subagent concurrency cap so that the planning workflow's
   research fan-out has a governing rule to cite.

## Acceptance Criteria (Gherkin)

```gherkin
Feature: Adopt dependency bump policy and planning workflow into ose-primer

Scenario: Policy document is present and indexed
  Given the adoption plan has been executed
  When I list repo-governance/development/workflow/
  Then dependency-bump-policy.md exists
  And the development workflow README links to it
  And the document describes the three-path decision tree, exact-pin rule, CVE clearance,
      CISA KEV fast-track, EPSS escalation, and Rule 5a/5b

Scenario: Planning workflow conforms to the naming convention
  Given the planning workflow file repo-dependency-bump-planning.md exists under repo-governance/workflows/repo/
  When I run the workflow-naming enforcement command
    """
    find repo-governance/workflows -name '*.md' -not -name 'README.md' -not -path '*/meta/*' \
      | sed 's|.*/||; s|\.md$||' \
      | grep -vE -- '-(quality-gate|execution|setup|planning)$'
    """
  Then the command prints no lines

Scenario: Both rhino-cli validators accept the planning type
  Given the planning type token has been added
  When I run the rhino-cli-rust workflow naming tests and the rhino-cli-go workflow naming tests
  Then both test suites pass
  And the validators accept a filename ending in "-planning"

Scenario: Supporting rules and registers exist
  Given the adoption plan has been executed
  Then repo-governance/development/agents/subagent-orchestration.md exists and is indexed
  And docs/reference/security-waivers.md exists and is indexed
  And no internal documentation link in the new files is broken

Scenario: No dependency manifest is modified
  Given the adoption plan has been executed
  When I inspect git diff for the plan's commits
  Then no package.json, Cargo.toml, rust-toolchain.toml, go.mod, *.csproj, *.fsproj,
      global.json, Dockerfile, docker-compose*.yml, .github/ action or workflow, or lockfile
      version pin was changed
```

## Product Scope

### In scope

- Adapted policy document and planning workflow document.
- `planning` workflow type token in the convention and both validators (with tests).
- Subagent-orchestration convention and security-waivers register.
- Index/cross-reference wiring for every new file.

### Out of scope

- Any actual dependency version change.
- A scheduled/automated invocation of the planning workflow.
- Editing the upstream `ose-public` copies.

## Product Risks

- **Broken cross-links** if a referenced doc path differs in `ose-primer`. Mitigation: every
  reference is repo-grounded in `tech-docs.md` before authoring; the link checker runs in the
  validation phase.
- **Validator test gaps** if the `planning` token is added to the constant but not to the tests'
  expected messages. Mitigation: update help text and test expectations alongside the constant.
