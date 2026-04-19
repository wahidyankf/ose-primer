# Product Requirements Document

## Product Overview

A governance documentation update and 1 targeted agent tier correction. All phases are
pending for `ose-primer` (the equivalent changes were applied to `ose-public` 2026-04-19):
document budget-adaptive inheritance, extend model-selection.md to dual-platform coverage,
add inline plan format to CLAUDE.md, create a cited benchmark reference document, correct
`repo-rules-maker` tier (OMIT→SONNET), add benchmark citations to policy docs, validate,
sync, archive.

## Personas

- **Maintainer as agent author** — creates or reviews agent files; needs a single policy
  doc that answers "which model do I use, on which platform, and why"
- **Maintainer as plan author** — reads CLAUDE.md to learn plan structure; needs the
  5-doc format description visible inline without navigating to the convention file
- **`agent-maker` agent** — consulted by the maintainer when scaffolding a new agent;
  reads `model-selection.md` to determine frontmatter and justification text; benefits
  from dual-platform model coverage
- **`plan-maker` agent** — reads CLAUDE.md to determine how many files to create and how
  to name them; currently must follow an external link to learn the correct format
- **`repo-rules-checker` agent** — validates model selection compliance; policy
  completeness directly determines check quality

## User Stories

**US-1**: As a maintainer or agent author reading the model selection policy, I want to
understand why opus-tier agents omit the `model` field, so that I do not accidentally add
`model: opus` and break the budget-adaptive inheritance behavior.

**US-2**: As an agent author adding a new agent, I want a single policy document that
covers both Claude Code model aliases and OpenCode GLM model IDs, so that I can correctly
fill in the `model:` field for both platforms without cross-referencing multiple files.

**US-3**: As a maintainer reading CLAUDE.md onboarding material, I want the Plans
Organization section to describe the 5-doc format inline, so that when I (or an AI agent)
creates a new plan it has the correct number and names of files on the first attempt.

**US-4**: As `repo-rules-checker`, I want the model selection policy to enumerate all
valid model values and their OpenCode equivalents, so that I can flag non-compliant agents
with confidence.

**US-5**: As a maintainer auditing agent tier assignments, I want a single benchmark
reference document with cited scores for all five models (Claude Opus 4.7, Sonnet 4.6,
Haiku 4.5, GLM-5.1, GLM-5-turbo), so that I can verify WHY each tier was assigned
without having to independently research vendor documentation.

**US-6**: As a maintainer reading any policy document that makes a tier claim, I want
each claim to link to the benchmark reference document, so that I can follow the citation
chain from claim to primary source in one click.

## Acceptance Criteria

```gherkin
Feature: Budget-adaptive inheritance documented in model-selection.md

  Scenario: Opus tier section documents why model field is omitted
    Given governance/development/agents/model-selection.md
    When I read the Opus tier section
    Then I find a "Budget-Adaptive Inheritance" block
    And it explains that omitting model inherits the session's active model
    And it includes the account-tier table (Max/Team → Opus 4.7, Pro/Standard → Sonnet 4.6)
    And it contains a warning not to add model: opus

  Scenario: Common Mistakes table includes budget-adaptive entry
    Given the Common Mistakes table in model-selection.md
    When I read all rows
    Then one row describes "Adding model: opus to opus-tier agents" as a mistake
    And the correction says "Omit the field — inherit session model to match user's tier"

Feature: Dual-platform model selection policy

  Scenario: OpenCode section exists in model-selection.md
    Given governance/development/agents/model-selection.md
    When I search for "OpenCode"
    Then I find a section titled "OpenCode / GLM Equivalents"
    And the section contains a mapping table
    And the table includes rows for omit, sonnet, and haiku

  Scenario: 3-to-2 collapse is documented
    Given the OpenCode / GLM Equivalents section
    When I read the mapping table
    Then I see that both omit and sonnet map to zai-coding-plan/glm-5.1
    And I see an explanation of why the collapse happens

  Scenario: Current model versions table exists
    Given governance/development/agents/model-selection.md
    When I search for "Current Model Versions"
    Then I find a table with rows for opus, sonnet, and haiku
    And the haiku row notes Haiku 3 retirement on 2026-04-19

  Scenario: Haiku 3 retirement is noted
    Given governance/development/agents/model-selection.md
    When I search for "retired"
    Then I find a note that Haiku 3 (claude-3-haiku) was retired 2026-04-19

  Scenario: Opus tier frontmatter example omits model field
    Given the Opus tier frontmatter example in model-selection.md
    When I read the YAML block
    Then it does not contain a model field
    And the surrounding text explains this is intentional budget-adaptive design

Feature: CLAUDE.md inline plan format

  Scenario: CLAUDE.md Plans Organization section describes five-document format
    Given the Plans Organization section of CLAUDE.md
    When I read the plan layout description
    Then it describes "five documents"
    And it lists: README.md, brd.md, prd.md, tech-docs.md, delivery.md
    And it notes the single-README.md collapse rule for trivially small plans

Feature: Budget-adaptive note propagated to related docs

  Scenario: ai-agents.md documents budget-adaptive inheritance
    Given governance/development/agents/ai-agents.md
    When I search for "budget-adaptive" or "inherit"
    Then I find a note that opus-tier agents omit model field by design
    And it warns against adding model: opus

  Scenario: best-practices.md documents budget-adaptive inheritance
    Given governance/development/agents/best-practices.md
    When I search for "budget-adaptive" or "inherit"
    Then I find guidance that opus-tier agents omit the model field intentionally

  Scenario: .claude/agents/README.md documents omit-by-design
    Given the .claude/agents/README.md file is readable
    When I search for "budget-adaptive" or "omit"
    Then I find a note that opus-tier agents omit model by design

Feature: Benchmark reference document

  Scenario: Benchmark reference document exists
    Given the docs/reference/ directory
    When I look for ai-model-benchmarks.md
    Then the file exists
    And it contains sections for Claude Opus 4.7, Sonnet 4.6, Haiku 4.5, GLM-5.1, and GLM-5-turbo

  Scenario: Every benchmark score is cited
    Given docs/reference/ai-model-benchmarks.md
    When I read every benchmark score row
    Then each row has a source URL
    And each row has a publication date
    And each row has a confidence level (Verified, Self-reported, or Needs Verification)

  Scenario: GLM-5-turbo has no-standard-benchmarks flag
    Given docs/reference/ai-model-benchmarks.md GLM-5-turbo section
    When I read the benchmark table
    Then it explicitly states no standard benchmarks are published for this model
    And it explains ZClawBench is proprietary and unverified

  Scenario: Policy docs link to benchmark reference
    Given governance/development/agents/model-selection.md
    When I read the tier sections containing benchmark numbers
    Then each benchmark number links to docs/reference/ai-model-benchmarks.md

Feature: Agent tier right-sizing

  Scenario: repo-rules-maker changes tier
    Given .claude/agents/repo-rules-maker.md before and after Phase 5
    When I compare model fields
    Then repo-rules-maker changes from omit to sonnet

  Scenario: Opus-inherit count drops from 15 to 14
    Given all .md files in .claude/agents/ except README.md
    When I count files with no model field or empty model field
    Then the count is 14

  Scenario: Justification block updated for changed agent
    Given .claude/agents/repo-rules-maker.md after Phase 5
    When I read its Model Selection Justification block
    Then the justification references sonnet tier
    And it does not contradict the model field value

Feature: Validations pass after changes

  Scenario: Claude agent validation passes
    Given all .claude/agents/ files including the updated repo-rules-maker.md
    When I run npm run validate:claude
    Then the command exits with code 0
    And no validation errors are printed

  Scenario: Sync validation passes
    Given all .claude/agents/ files
    And the re-synced .opencode/agent/ files
    When I run npm run validate:sync
    Then the command exits with code 0
    And no sync drift errors are printed

  Scenario: OpenCode agents have explicit model
    Given all .md files in .opencode/agent/ after sync
    When I inspect each file's model field
    Then every file has a non-empty model value
    And the value is one of: zai-coding-plan/glm-5.1, zai-coding-plan/glm-5-turbo

  Scenario: rhino-cli tests pass
    Given the existing agent files and test fixtures
    When I run nx run rhino-cli:test:quick
    Then the command exits with code 0
```

## Product Scope

### In Scope

- Update `governance/development/agents/model-selection.md`:
  - "Budget-Adaptive Inheritance" block, OpenCode section, version table, Common Mistakes row
- Update `CLAUDE.md`: add inline 5-doc plan format description + `opus` alias in Format
  Differences models row
- Propagate budget-adaptive note to related governance docs
- Create `docs/reference/ai-model-benchmarks.md`: cited benchmark scores for all 5 models
- Add benchmark citations (links to reference doc) to `model-selection.md` and
  `.claude/agents/README.md`
- Correct 1 agent tier assignment: `repo-rules-maker` OMIT→SONNET
- Re-run `npm run sync:claude-to-opencode` to reflect tier change in `.opencode/agent/`

### Out of Scope

- Changing any agent's cognitive task category (only tier correction, not re-classification)
- Modifying rhino-cli Go source (already supports all tiers)
- Manually editing `.opencode/agent/` files (sync generates these; do not hand-edit)
- Adding or removing any agent
- Acquiring independent GLM-5.1 benchmark verification (documented as a limitation)
- Resolving GLM-5-turbo's lack of standard benchmarks (documented as platform constraint)

## Product-Level Risks

| Risk                                                                    | Impact | Note                                                                            |
| ----------------------------------------------------------------------- | ------ | ------------------------------------------------------------------------------- |
| plan-maker omits plan format if CLAUDE.md is not updated first          | Medium | CLAUDE.md update is Phase 2 of delivery; fix it early                           |
| Reviewer confuses `model: opus` alias with a full model ID              | Low    | Policy doc makes alias vs ID distinction explicit                               |
| Future model upgrade (e.g., Opus 4.8) requires another round of updates | Low    | Aliases (`opus`) auto-track new versions; only the version table needs updating |
