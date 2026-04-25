# Product Requirements

## Product Overview

Three discrete governance adoptions applied to `ose-primer` in a single plan execution.
Each change is independent and can be verified in isolation, but all three ship in one
commit sequence.

## Personas

- **Template consumer** — a team cloning `ose-primer` to bootstrap a new polyglot Nx
  monorepo. Inherits all agents, skills, governance, and tooling.
- **AI agent** (`plan-maker`, `plan-checker`, `plan-fixer`) — operates in consumer repos
  derived from this template; must exhibit correct push and PR behavior.
- **Documentation author** — writes Mermaid diagrams in `governance/` or `docs/`; relies
  on automated structural feedback.

## User Stories

### Change A — git-push-default

- As an AI agent running `plan-maker`, I want explicit guidance that delivery checklists
  must not include unsolicited `- [ ] Create PR` steps, so I do not generate unnecessary
  PR workflow on TBD repos.
- As an AI agent running `plan-checker`, I want a HIGH finding rule for unsolicited PR
  steps in delivery checklists, so I can catch and flag violations before execution.
- As a developer reading the convention, I want a single canonical document that states
  the default push behavior and when PRs are opt-in, so I do not have to infer it from
  the TBD convention.

### Change B — no-date-metadata

- As a template consumer, I want agent files to not carry `- **Last Updated**: DATE`
  rows, so my repo does not inherit stale metadata on first clone.
- As a documentation author, I want governance and docs files to not carry `created:` /
  `updated:` frontmatter, so I can use `git log` as the single source of truth for dates.
- As a template consumer using `docs-tutorial-maker`, I want its frontmatter template
  to not include `created:` / `updated:` fields, so tutorials I generate do not carry
  the anti-pattern.

### Change C — rhino-cli docs validate-mermaid

- As a documentation author, I want `rhino-cli docs validate-mermaid` to flag node
  labels exceeding 30 characters, so diagrams do not clip at Mermaid's wrapping width.
- As a documentation author, I want the tool to flag more than 3 parallel nodes at one
  rank, so diagrams do not become overcrowded (with a warning-not-error exception when
  both width and depth thresholds are exceeded).
- As a documentation author, I want the tool to flag mermaid blocks containing multiple
  flowchart declarations, so each block renders exactly one diagram.
- As a developer, I want `nx run rhino-cli:validate:mermaid` to run in CI (cacheable)
  and the pre-push hook to check only changed `.md` files, so the feedback is fast.

## Acceptance Criteria (Gherkin)

### Change A — git-push-default

```gherkin
Feature: git-push-default convention

  Scenario: Convention file exists
    Given the ose-primer repository
    When I look at governance/development/workflow/
    Then git-push-default.md exists
    And it documents that default push is direct to main with no PR
    And it documents a responsibility table for plan-maker, plan-checker, plan-fixer

  Scenario: plan-maker does not insert unsolicited PR steps
    Given a plan-maker invocation with no explicit PR instruction in the prompt
    When plan-maker authors a delivery checklist
    Then the checklist contains no "Create PR" or "Open PR" step

  Scenario: plan-checker flags unsolicited PR step
    Given a delivery.md with a "- [ ] Create PR" step
    And no explicit PR instruction exists in the plan or prompt
    When plan-checker validates the plan
    Then it reports a HIGH finding for the unsolicited PR step

  Scenario: plan-fixer removes unsolicited PR step
    Given a plan-checker audit report with a HIGH unsolicited-PR finding
    When plan-fixer applies fixes
    Then the unsolicited "- [ ] Create PR" step is removed from delivery.md
```

### Change B — no-date-metadata

```gherkin
Feature: no-date-metadata convention

  Scenario: Convention file exists
    Given the ose-primer repository
    When I look at governance/conventions/writing/
    Then no-date-metadata.md exists
    And it states that created: and updated: frontmatter are forbidden
    And it states that manual Last Updated annotations are forbidden
    And it states that git log is the authoritative source for file dates

  Scenario: No agent files carry Last Updated rows
    Given all files under .claude/agents/
    When I search for "- **Last Updated**:" patterns
    Then zero matches are found

  Scenario: No governance files carry date frontmatter
    Given all files under governance/
    When I search for "^created:" or "^updated:" in frontmatter
    Then zero matches are found

  Scenario: No docs files carry date frontmatter
    Given all files under docs/
    When I search for "^created:" or "^updated:" in frontmatter
    Then zero matches are found

  Scenario: Template examples do not include date fields
    Given docs-tutorial-maker.md and agent-developing-agents SKILL.md
    When I inspect their frontmatter template examples
    Then no created: or updated: fields appear in those templates
```

### Change C — rhino-cli docs validate-mermaid

```gherkin
Feature: docs validate-mermaid command

  Scenario: Command exists and exits 0 on clean repo
    Given the ose-primer repository with no Mermaid violations
    When I run rhino-cli docs validate-mermaid
    Then the command exits successfully
    And the output reports no violations

  Scenario: Node label over 30 chars is flagged
    Given a markdown file with a flowchart node label of 35 characters
    When I run rhino-cli docs validate-mermaid
    Then the command exits with a failure code
    And the output identifies the file, block, and node with the oversized label

  Scenario: More than 3 parallel nodes at one rank is flagged
    Given a markdown file with a TB flowchart where one rank has 4 nodes
    When I run rhino-cli docs validate-mermaid
    Then the command exits with a failure code
    And the output identifies the file and block with excessive width

  Scenario: Multiple flowchart declarations in one block are flagged
    Given a markdown file with a mermaid block containing two flowchart declarations
    When I run rhino-cli docs validate-mermaid
    Then the command exits with a failure code
    And the output identifies the file and block with multiple diagrams

  Scenario: Nx validate:mermaid target exists and is cacheable
    Given apps/rhino-cli/project.json
    When I inspect the targets
    Then validate:mermaid target exists
    And cache is true
    And inputs include projectRoot Go files and workspace governance/ and .claude/ md files

  Scenario: Pre-push hook runs validate:mermaid only when md files changed
    Given the pre-push hook
    When the push range includes .md files
    Then the hook runs nx run rhino-cli:validate:mermaid with --changed-only

  Scenario: test:quick passes with >=90% coverage
    Given the ose-primer rhino-cli project after porting
    When I run nx run rhino-cli:test:quick
    Then it exits successfully
    And coverage is at or above 90%

  Scenario: Gherkin spec exists for docs validate-mermaid
    Given specs/apps/rhino/cli/gherkin/
    When I look for a docs-validate-mermaid.feature file
    Then it exists and contains scenarios for label length, rank width, and multi-diagram violations
```

## Product Scope

**In scope:**

- Convention files A and B with full standards, examples, and related-docs sections.
- Agent body updates for A (plan-maker, plan-checker, plan-fixer).
- plan-execution workflow update for A (linear history, opt-in PR).
- Mechanical date-metadata removal across all affected files for B.
- Template example updates in docs-tutorial-maker, agent-developing-agents,
  repo-defining-workflows skill for B.
- Full port of `internal/mermaid` package (11 Go files) for C.
- Full port of `cmd/docs_validate_mermaid.go` and test files for C.
- Nx target `validate:mermaid` and pre-push hook for C.
- Diagrams convention automated enforcement note for C.
- `npm run sync:claude-to-opencode` after all `.claude/` changes.

**Out of scope:**

- Changes to any ose-public or ose-projects files.
- Adding new Gherkin scenarios for existing rhino-cli commands.
- Updating the OpenAPI spec or demo app contracts.
- Changing `go.mod` module path.

## Product Risks

- **Sed false positive on skill examples**: `agent-developing-agents` SKILL.md contains
  `- **Last Updated**: YYYY-MM-DD` as template copy. Stripping it is correct per the
  convention; verifying by grep after the run confirms no residual.
- **Mermaid validator rejects existing diagrams**: `validate:mermaid` may flag diagrams
  in `governance/` or `.claude/` that violate rules. These are pre-existing violations
  and must be fixed as part of this plan (not deferred).
