@repo-governance-gherkin-keyword-cardinality
Feature: Gherkin Keyword Cardinality Audit

  As a repository maintainer
  I want to flag scenarios that repeat a primary Given/When/Then keyword
  So that Gherkin specs keep the one-each chained step shape

  Scenario: A scenario with two primary When keywords fails the audit
    Given a feature file containing a scenario with two primary "When" keywords
    When the developer runs repo-governance gherkin-keyword-cardinality on the file
    Then the command exits with a failure code
    And the output names the offending file and scenario

  Scenario: A conforming feature file passes the audit
    Given a feature file whose scenarios each use one primary keyword chained with "And"
    When the developer runs repo-governance gherkin-keyword-cardinality on the file
    Then the command exits successfully
    And the output reports zero cardinality findings

  Scenario: A Background block with repeated keywords passes the audit
    Given a feature file whose Background block repeats the "Given" keyword
    When the developer runs repo-governance gherkin-keyword-cardinality on the file
    Then the command exits successfully
    And the output reports zero cardinality findings

  Scenario: A Scenario Outline Examples table passes the audit
    Given a feature file with a Scenario Outline whose Examples table has many rows
    When the developer runs repo-governance gherkin-keyword-cardinality on the file
    Then the command exits successfully
    And the output reports zero cardinality findings

  Scenario: Keyword words inside doc-strings and comments pass the audit
    Given a feature file whose doc-strings and comments contain primary keyword words
    When the developer runs repo-governance gherkin-keyword-cardinality on the file
    Then the command exits successfully
    And the output reports zero cardinality findings

  Scenario: A directory of conforming feature files passes the audit
    Given a directory of feature files that all obey the one-each keyword rule
    When the developer runs repo-governance gherkin-keyword-cardinality on the directory
    Then the command exits successfully
    And the output reports zero cardinality findings
