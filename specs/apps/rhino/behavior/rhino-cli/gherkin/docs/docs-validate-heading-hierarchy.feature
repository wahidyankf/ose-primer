@docs-validate-heading-hierarchy
Feature: Markdown Heading Hierarchy Validation

  As a documentation author
  I want to detect heading hierarchy violations in markdown files
  So that documents follow consistent heading structure

  Scenario: A docs file with two H1s is flagged duplicate-h1
    Given a markdown file under docs with two H1 headings
    When the developer runs docs validate-heading-hierarchy
    Then the command exits with a failure code
    And the output reports a duplicate-h1 finding for that file

  Scenario: A docs file with zero H1s is flagged missing-h1
    Given a markdown file under docs with no H1 heading
    When the developer runs docs validate-heading-hierarchy
    Then the command exits with a failure code
    And the output reports a missing-h1 finding for that file

  Scenario: A docs file jumping from H1 to H3 is flagged skipped-level
    Given a markdown file under docs that jumps from H1 directly to H3
    When the developer runs docs validate-heading-hierarchy
    Then the command exits with a failure code
    And the output reports a skipped-level finding for that file

  Scenario: A .claude/agents file with heading violations is exempt
    Given a markdown file under .claude/agents with zero H1 headings
    When the developer runs docs validate-heading-hierarchy
    Then the command exits successfully
    And no missing-h1 finding is reported for that file

  Scenario: A SKILL.md under .claude/skills with many H1s is exempt
    Given a SKILL.md file under .claude/skills with multiple H1 headings
    When the developer runs docs validate-heading-hierarchy
    Then the command exits successfully
    And no duplicate-h1 finding is reported for that file

  Scenario: A file under plans/done with violations is excluded
    Given a markdown file under plans/done with a skipped heading level
    When the developer runs docs validate-heading-hierarchy
    Then the command exits successfully
    And no skipped-level finding is reported for that file

  Scenario: An app README with a violation is flagged
    Given an apps/example/README.md file with a skipped heading level
    When the developer runs docs validate-heading-hierarchy
    Then the command exits with a failure code
    And the output reports a skipped-level finding for that file

  Scenario: A deep app internal path with violations is excluded
    Given a markdown file at apps/example/src/notes.md with zero H1 headings
    When the developer runs docs validate-heading-hierarchy
    Then the command exits successfully
    And no missing-h1 finding is reported for that file

  Scenario: With --exclude docs the docs tree findings are suppressed
    Given a markdown file under docs with a duplicate H1
    And a markdown file under repo-governance with a duplicate H1
    When the developer runs docs validate-heading-hierarchy with --exclude docs
    Then no finding is reported for the docs file
    And the output reports a duplicate-h1 finding for the repo-governance file
