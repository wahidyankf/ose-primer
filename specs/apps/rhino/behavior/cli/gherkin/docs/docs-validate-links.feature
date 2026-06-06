@docs-validate-links
Feature: Markdown Internal Link Validation

  As a documentation author
  I want to detect broken internal links in markdown files
  So that readers always reach the intended documents

  Scenario: A document set with all valid internal links passes validation
    Given markdown files where all internal links point to existing files
    When the developer runs docs validate-links
    Then the command exits successfully
    And the output reports no broken links found

  Scenario: A broken internal link is detected and reported
    Given a markdown file with a link pointing to a non-existent file
    When the developer runs docs validate-links
    Then the command exits with a failure code
    And the output identifies the file containing the broken link

  Scenario: External URLs are not validated
    Given a markdown file containing only external HTTPS links
    When the developer runs docs validate-links
    Then the command exits successfully
    And the output reports no broken links found

  Scenario: With --staged-only only staged files are checked
    Given a markdown file with a broken link that has not been staged in git
    When the developer runs docs validate-links with the --staged-only flag
    Then the command exits successfully

  Scenario: With --exclude an excluded tree is not validated
    Given a markdown file with a broken link inside a directory tree
    When the developer runs docs validate-links with the --exclude flag for that tree
    Then the command exits successfully
    And the output reports no broken links found

  Scenario: A broken link outside the historic scan scope is detected
    Given a markdown file under libs with a link pointing to a non-existent file
    When the developer runs docs validate-links
    Then the command exits with a failure code
    And the output identifies the file containing the broken link

  Scenario: A link to a missing anchor in an existing file is reported as a broken anchor
    Given a markdown file with a link to an existing file whose anchor matches no heading
    When the developer runs docs validate-links
    Then the command exits with a failure code
    And the output reports a broken-anchor finding for the link

  Scenario: A link to a valid anchor in an existing file passes validation
    Given a markdown file with a link to an existing file whose anchor matches a heading
    When the developer runs docs validate-links
    Then the command exits successfully
    And the output reports no broken links found

  Scenario: A same-file anchor link with no matching heading is reported as a broken anchor
    Given a markdown file with a pure-anchor link that matches no heading in the same file
    When the developer runs docs validate-links
    Then the command exits with a failure code
    And the output reports a broken-anchor finding for the link
