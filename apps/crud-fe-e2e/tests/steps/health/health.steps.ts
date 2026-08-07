import { createBdd } from "playwright-bdd";
import { expect } from "@playwright/test";

const { Given, When, Then } = createBdd();

Given("the demo application is running", async () => {});

When("a reader opens the demo application", async ({ page }) => {
  await page.goto("/");
});

// @covers specs/apps/crud/behavior/crud-web/gherkin/reference/reference-role.feature:Landing page explains its reference role
Then("the landing page should identify itself as a reusable reference application", async ({ page }) => {
  await expect(page.getByText("A reusable reference application", { exact: true })).toBeVisible();
});

Then("it should explain that the example can be adapted for a team's product", async ({ page }) => {
  await expect(
    page.getByText("Explore this working example, then adapt it to fit your team's product.", { exact: true }),
  ).toBeVisible();
});

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
