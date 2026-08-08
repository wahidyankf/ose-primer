import { defineConfig, devices } from "@playwright/test";
import { defineBddConfig } from "playwright-bdd";

const testDir = defineBddConfig({
  featuresRoot: "../../specs/apps/crud/behavior/crud-web/gherkin",
  features: "../../specs/apps/crud/behavior/crud-web/gherkin/health/health-status.feature",
  outputDir: ".features-gen/frontend-only",
  steps: ["./tests/frontend-only-steps/**/*.steps.ts"],
  tags: "@frontend-only",
});

export default defineConfig({
  testDir,
  timeout: 60000,
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  workers: 1,
  reporter: process.env.CI ? [["list"], ["html", { outputFolder: "playwright-report-frontend-only" }]] : "list",
  use: {
    baseURL: "http://localhost:3401",
    trace: "on-first-retry",
    screenshot: "only-on-failure",
  },
  webServer: {
    command: "npx nx run crud-fs-ts-nextjs:dev",
    cwd: "../..",
    env: {
      ...process.env,
      CRUD_FS_TS_NEXTJS_JWT_SECRET: "",
      NEXT_PUBLIC_BACKEND_ENABLED: "false",
    },
    url: "http://localhost:3401",
    timeout: 120000,
    reuseExistingServer: false,
    gracefulShutdown: { signal: "SIGTERM", timeout: 5000 },
  },
  projects: [
    {
      name: "chromium-frontend-only",
      use: {
        ...devices["Desktop Chrome"],
        ...(process.env.PLAYWRIGHT_FRONTEND_ONLY_CHANNEL
          ? { channel: process.env.PLAYWRIGHT_FRONTEND_ONLY_CHANNEL }
          : {}),
      },
    },
  ],
});
