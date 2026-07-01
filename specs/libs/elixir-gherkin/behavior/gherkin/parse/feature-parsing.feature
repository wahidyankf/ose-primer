@wip
Feature: Gherkin feature-file parsing
  As an Elixir developer consuming elixir-gherkin
  I want Gherkin.parse to translate .feature text into structured Elixir terms
  So that Cabbage.Feature (and any other caller) can compile or inspect scenarios programmatically

  Scenario: Parsing a simple feature returns its name and scenarios
    Given the text of a .feature file with one Feature and one Scenario with two steps
    When I call Gherkin.parse on the text
    Then the result is a Gherkin.Elements.Feature struct
    And the feature's scenarios list contains 1 scenario
    And that scenario's steps list contains 2 steps

  Scenario: Flattening a Scenario Outline expands one scenario per example row
    Given a parsed feature containing a Scenario Outline with 3 Examples rows
    When I call Gherkin.flatten on the feature
    Then the flattened feature's scenarios list contains 3 scenarios
    And each scenario's step text has its "<placeholder>" tokens replaced by the row's values
