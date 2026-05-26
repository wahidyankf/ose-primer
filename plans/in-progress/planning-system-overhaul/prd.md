# Product Requirements Document — Planning System Overhaul

## Product Overview

This plan makes three targeted text edits to governance files in ose-primer:

1. Adds an explicit HARD RULE paragraph to `test-driven-development.md` prohibiting the
   collapse of RED, GREEN, and REFACTOR phases into a single checkbox.
2. Updates `AGENTS.md` item 3 (Project Planning) to include `repo-setup-manager`, reference
   the `plan-establishment-execution.md` workflow, and document the `plan-maker` grill
   mandate.
3. Excludes `plans/done/` and `archived/` from markdown linting in both
   `.markdownlintignore` and `.markdownlint-cli2.jsonc`, and adds a policy section to
   `repo-governance/development/quality/markdown.md`.

No new files, agents, or code are created.

## Personas

| Persona            | Description                                                                   |
| ------------------ | ----------------------------------------------------------------------------- |
| Plan author        | Solo maintainer writing delivery checklists who must follow the TDD HARD RULE |
| Plan executor      | Agent (or human) executing delivery checklists step by step                   |
| Repo contributor   | Anyone reading `AGENTS.md` to discover available agents and workflows         |
| repo-rules-checker | Agent validating governance docs for convention compliance                    |

## User Stories

- As a plan executor, I want each TDD phase (RED, GREEN, REFACTOR) to be a separate
  checkbox so that I can track progress independently and the plan-execution workflow can
  verify each phase was completed.
- As a plan author, I want `test-driven-development.md` to contain an explicit prohibition
  so that I know combining phases is forbidden, not just discouraged.
- As a repo contributor, I want `AGENTS.md` to list `repo-setup-manager` and reference
  `plan-establishment-execution.md` so that I can discover all agents and workflows from a
  single source.
- As a repo maintainer, I want `plans/done/` and `archived/` excluded from markdown lint
  so that historical link rot does not block the quality gate on actively maintained files.

## Product Scope

**In scope** (cross-reference: README.md §In-Scope):

- Insert HARD RULE paragraph into `test-driven-development.md`
- Insert grouping-label note into `test-driven-development.md`
- Update item 3 in `AGENTS.md` Agent Organization list
- Append archive exclusion entries to `.markdownlintignore` and `.markdownlint-cli2.jsonc`
- Append Archive Exclusion section to `repo-governance/development/quality/markdown.md`

**Out of scope** (cross-reference: README.md §Out-of-Scope):

- Enforcing the TDD HARD RULE in `plan-checker` (future plan)
- Any changes to `plan-execution.md`, `plan-maker.md`, `plan-establishment-execution.md`,
  or `repo-setup-manager.md`
- Changing agent or skill behavior

## Product Risks

| Risk                                                                               | Mitigation                                                                    |
| ---------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| HARD RULE paragraph inserted at the wrong location in `test-driven-development.md` | Delivery Step 1.2 uses grep to confirm exactly one match in the right section |
| `AGENTS.md` change introduces markdown lint violations                             | Step 2.2 runs lint immediately after the edit                                 |
| Archive exclusions in `.markdownlint-cli2.jsonc` introduce JSONC syntax errors     | Step 3.4 runs full `npm run lint:md` which fails fast on JSONC parse errors   |

## Requirements

### REQ-1: TDD HARD RULE in `test-driven-development.md`

**Description**: After the three-substep template code block in the "TDD Shape for Delivery
Checklists" section, insert a HARD RULE paragraph making it explicit that RED, GREEN, and
REFACTOR must each be their own `- [ ]` item.

Also add a "grouping label" note after the mini-TDD nested example in the "Plan Creation
(plan-maker)" subsection, clarifying that the parent label (`- [ ] TDD cycle:`) is grouping
only and must not substitute for the three phase items.

**Acceptance criteria**: See Gherkin below.

### REQ-2: `AGENTS.md` catalog completeness

**Description**: Update the "Project Planning" agent category (item 3 in the list) to:

- Add `repo-setup-manager` to the agent list
- Add a reference to the `plan-establishment-execution.md` workflow
- Add a note that `plan-maker` mandates grilling before and after plan creation and that
  delivery checklists must begin with Phase 0

**Acceptance criteria**: See Gherkin below.

### REQ-3: Markdown archive exclusions

**Description**: Exclude `plans/done/` and `archived/` from markdown linting in:

- `.markdownlintignore` — append two entries with a comment
- `.markdownlint-cli2.jsonc` — add two entries to the `ignores` array
- `repo-governance/development/quality/markdown.md` — add an "Archive Exclusion" section

**Acceptance criteria**: See Gherkin below.

## Gherkin Acceptance Criteria

### Feature: TDD HARD RULE

```gherkin
Feature: TDD HARD RULE in test-driven-development.md

  Scenario: HARD RULE paragraph is present after the three-substep template
    Given the file "repo-governance/development/workflow/test-driven-development.md"
    When I search for "HARD RULE: Never combine RED, GREEN, and REFACTOR"
    Then exactly one match is found in the "TDD Shape for Delivery Checklists" section

  Scenario: Grouping-label note is present after the mini-TDD nested example
    Given the file "repo-governance/development/workflow/test-driven-development.md"
    When I search for "grouping label"
    Then exactly one match is found in the "Plan Creation (plan-maker)" subsection

  Scenario: Markdown lint passes after the change
    Given the modified "test-driven-development.md"
    When I run "npm run lint:md"
    Then the command exits 0 with no violations on the file
```

### Feature: AGENTS.md catalog completeness

```gherkin
Feature: AGENTS.md reflects repo-setup-manager and plan-establishment

  Scenario: repo-setup-manager appears in the agent catalog
    Given the file "AGENTS.md"
    When I search for "repo-setup-manager"
    Then at least one match is found in the Project Planning agent category

  Scenario: plan-establishment workflow is referenced
    Given the file "AGENTS.md"
    When I search for "plan-establishment"
    Then at least one match is found referencing "plan-establishment-execution.md"

  Scenario: plan-maker grill mandate is documented
    Given the file "AGENTS.md"
    When I search for "grill" near "plan-maker"
    Then at least one match describing the mandatory grill is found

  Scenario: Markdown lint passes after the change
    Given the modified "AGENTS.md"
    When I run "npm run lint:md"
    Then the command exits 0 with no violations on the file
```

### Feature: Markdown archive exclusions

```gherkin
Feature: plans/done/ and archived/ excluded from markdown lint

  Scenario: .markdownlintignore contains archive entries
    Given the file ".markdownlintignore"
    When I search for "plans/done/"
    Then exactly one match is found
    When I search for "archived/"
    Then exactly one match is found

  Scenario: .markdownlint-cli2.jsonc contains archive entries
    Given the file ".markdownlint-cli2.jsonc"
    When I search for "plans/done/**"
    Then exactly one match is found in the ignores array
    When I search for "archived/**"
    Then exactly one match is found in the ignores array

  Scenario: markdown.md documents archive exclusion
    Given the file "repo-governance/development/quality/markdown.md"
    When I search for "Archive Exclusion" or "archive exclusion"
    Then at least one match is found
    When I search for "plans/done"
    Then at least one match is found

  Scenario: Markdown lint passes with archive exclusions
    Given all three files are updated
    When I run "npm run lint:md"
    Then the command exits 0 with zero violations
```
