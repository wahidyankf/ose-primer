import { expect } from "@playwright/test";
import { createBdd } from "playwright-bdd";

const { Given, When, Then } = createBdd();

Given("the app is running", async () => {});

let frontendOnlyHealthRequests = 0;

When("the user opens the frontend-only reference app", async ({ page }) => {
  frontendOnlyHealthRequests = 0;
  page.on("request", (request) => {
    if (new URL(request.url()).pathname === "/health") {
      frontendOnlyHealthRequests += 1;
    }
  });
  await page.goto("/");
});

Then("the app should explain that a backend can be connected later", async ({ page }) => {
  await expect(page.getByText(/connect one when you are ready/i)).toBeVisible();
});

Then("the frontend-only reference app should not request backend health", async () => {
  expect(frontendOnlyHealthRequests).toBe(0);
});
