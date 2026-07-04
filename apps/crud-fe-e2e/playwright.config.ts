import { defineConfig, devices } from "@playwright/test";
import { defineBddConfig } from "playwright-bdd";

const testDir = defineBddConfig({
  featuresRoot: "../../specs/apps/crud/behavior/crud-web/gherkin",
  features: "../../specs/apps/crud/behavior/crud-web/gherkin/**/*.feature",
  steps: ["./tests/steps/**/*.steps.ts", "./tests/hooks/**/*.ts"],
  // Excludes codegen/dart-codegen-fresh-checkout.feature: it exercises
  // crud-fe-dart-flutterweb's own `codegen` Nx target on a fresh checkout and
  // has no browser-driven step definitions in this project (crud-fe-e2e only
  // drives real browsers against a running app). Tag-expression syntax per
  // playwright-bdd's `tags` config option.
  tags: "not @codegen",
});

export default defineConfig({
  testDir,
  timeout: 60000,
  // Each scenario resets the shared database before running, so tests must
  // run sequentially within a single machine to avoid DB state conflicts.
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  workers: 1,
  reporter: process.env.CI ? [["list"], ["html"]] : "list",
  use: {
    baseURL: process.env.BASE_URL || "http://localhost:3301",
    trace: "on-first-retry",
    screenshot: "only-on-failure",
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],
});
