Feature: Structural design token exports
  As a frontend developer
  I want ts-ui-tokens to export the structural design tokens
  So that every app can consume a consistent color, spacing, radius, and typography scale

  @unit
  Scenario: The package exports every structural token module
    Given the ts-ui-tokens package
    When I import from "@open-sharia-enterprise/ts-ui-tokens"
    Then "colorTokens" should be exported
    And "radius" should be exported
    And "spacing" should be exported
    And "typography" should be exported

  @unit
  Scenario: colorTokens maps every semantic color name to its CSS custom property
    Given the ts-ui-tokens package
    When I read the "colorTokens" export
    Then "colorTokens.background" should equal "--color-background"
    And "colorTokens.primary" should equal "--color-primary"
    And "colorTokens.destructive" should equal "--color-destructive"
