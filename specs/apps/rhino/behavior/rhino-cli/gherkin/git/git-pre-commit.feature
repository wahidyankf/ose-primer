@git-pre-commit
Feature: Pre-commit hook orchestration

  As a developer
  I want rhino-cli git pre-commit to orchestrate all pre-commit checks
  So that code quality is enforced consistently before every commit

  Scenario: Running pre-commit outside a git repository fails
    Given the developer is outside a git repository
    When the developer runs rhino-cli git pre-commit
    Then the command exits with a failure code
    And the output mentions that a git repository was not found

  Scenario: staged-mermaid-blocks - a staged markdown file with a malformed flowchart fails pre-commit
    Given a staged markdown file containing a flowchart with a malformed mermaid block
    When the developer runs rhino-cli git pre-commit
    Then the command exits with a failure code
    And the output reports a mermaid violation for the staged file

  Scenario: staged-prose-heading-blocks - a staged docs markdown file with a duplicate H1 fails pre-commit
    Given a staged markdown file under docs/ containing a duplicate H1 heading
    When the developer runs rhino-cli git pre-commit
    Then the command exits with a failure code
    And the output reports a heading hierarchy violation for the staged file

  Scenario: staged-skill-file-exempt - a staged SKILL.md under .claude/skills with many H1s passes the heading step
    Given a staged SKILL.md file under .claude/skills/ containing multiple H1 headings
    When the developer runs rhino-cli git pre-commit
    Then the command exits successfully
    And no heading violation is reported for the skill file

  Scenario: link-step-honors-exclusions - the link step skips files under plans/done
    Given a staged markdown file under plans/done/ containing a broken internal link
    When the developer runs rhino-cli git pre-commit
    Then the command exits successfully
    And no broken-link violation is reported for the plans/done file
