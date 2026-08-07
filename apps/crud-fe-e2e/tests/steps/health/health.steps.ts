import { createBdd } from "playwright-bdd";
import { expect } from "@playwright/test";

const { When, Then } = createBdd();

When("the user opens the app", async ({ page }) => {
  await page.goto("/");
});

When("an unauthenticated user opens the app", async ({ page }) => {
  await page.goto("/");
});

// @covers specs/apps/crud/behavior/crud-web/gherkin/health/health-status.feature:Health indicator does not expose component details to regular users
Then("no detailed component health information should be visible", async ({ page }) => {
  await expect(page.getByTestId("health-components")).not.toBeVisible();
  await expect(page.getByText("diskSpace", { exact: true })).not.toBeVisible();
  await expect(page.getByText("db", { exact: true })).not.toBeVisible();
});
