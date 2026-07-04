Feature: Feature file compilation to ExUnit tests
  As an Elixir test author
  I want Cabbage.Feature to compile a .feature file into ExUnit tests at compile time
  So that I can write BDD-style scenarios without hand-writing test boilerplate

  @unit
  Scenario: A scenario with all steps matched compiles into a passing ExUnit test
    Given a .feature file with a scenario whose every step matches a defgiven, defwhen, or defthen clause
    When the consuming module compiles with "use Cabbage.Feature, file: ..."
    Then one ExUnit test is generated for the scenario
    And running "mix test" passes that generated test

  @unit
  Scenario: A step with no matching macro clause fails at compile time
    Given a .feature file with a step whose text matches no defgiven, defwhen, or defthen clause
    When the consuming module compiles with "use Cabbage.Feature, file: ..."
    Then compilation raises a Cabbage.Feature.MissingStepError
