import path from "path";
import { loadFeature, describeFeature } from "@amiceli/vitest-cucumber";
import { expect } from "vitest";
import * as tsUiTokens from "./index";

const feature = await loadFeature(
  path.resolve(__dirname, "../../../specs/libs/ts-ui-tokens/behavior/gherkin/tokens/tokens-export.feature"),
);

describeFeature(feature, ({ Scenario }) => {
  Scenario("The package exports every structural token module", ({ Given, When, Then, And }) => {
    Given("the ts-ui-tokens package", () => {});

    When('I import from "@open-sharia-enterprise/ts-ui-tokens"', () => {});

    // @covers specs/libs/ts-ui-tokens/behavior/gherkin/tokens/tokens-export.feature:The package exports every structural token module
    Then('"colorTokens" should be exported', () => {
      expect(tsUiTokens.colorTokens).toBeDefined();
    });

    And('"radius" should be exported', () => {
      expect(tsUiTokens.radius).toBeDefined();
    });

    And('"spacing" should be exported', () => {
      expect(tsUiTokens.spacing).toBeDefined();
    });

    And('"typography" should be exported', () => {
      expect(tsUiTokens.typography).toBeDefined();
    });
  });

  Scenario("colorTokens maps every semantic color name to its CSS custom property", ({ Given, When, Then, And }) => {
    Given("the ts-ui-tokens package", () => {});

    When('I read the "colorTokens" export', () => {});

    // @covers specs/libs/ts-ui-tokens/behavior/gherkin/tokens/tokens-export.feature:colorTokens maps every semantic color name to its CSS custom property
    Then('"colorTokens.background" should equal "--color-background"', () => {
      expect(tsUiTokens.colorTokens.background).toBe("--color-background");
    });

    And('"colorTokens.primary" should equal "--color-primary"', () => {
      expect(tsUiTokens.colorTokens.primary).toBe("--color-primary");
    });

    And('"colorTokens.destructive" should equal "--color-destructive"', () => {
      expect(tsUiTokens.colorTokens.destructive).toBe("--color-destructive");
    });
  });
});
