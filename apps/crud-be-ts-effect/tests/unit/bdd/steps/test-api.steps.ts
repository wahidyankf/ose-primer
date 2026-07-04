import { Given, When, Then, DataTable } from "@cucumber/cucumber";
import { expect } from "@playwright/test";
import type { CustomWorld } from "../world.js";
import { countExpenses, countAttachments } from "../hooks.js";

const DEFAULT_PASSWORD = "Str0ng#Pass1";

// ---- Background ----

Given("the test API is enabled via ENABLE_TEST_API environment variable", function (this: CustomWorld) {
  // Unit BDD tests call service functions directly (see service-layer.ts) rather
  // than going through the HTTP router, so ENABLE_TEST_API — which gates the
  // real router in src/app.ts — has no effect here. Kept as a no-op step so the
  // shared Background reads naturally in the report.
});

// ---- Given ----

Given("users and expenses exist in the database", async function (this: CustomWorld) {
  const registerRes = await this.post("/api/v1/auth/register", {
    username: "alice",
    email: "alice@example.com",
    password: DEFAULT_PASSWORD,
  });
  if (registerRes.status !== 201) {
    throw new Error(`Failed to register alice: ${JSON.stringify(registerRes.body)}`);
  }
  this.userIds.set("alice", (registerRes.body as { id: string }).id);
  this.context["alice_password"] = DEFAULT_PASSWORD;

  const loginRes = await this.post("/api/v1/auth/login", { username: "alice", password: DEFAULT_PASSWORD });
  if (loginRes.status !== 200) {
    throw new Error(`Failed to login alice: ${JSON.stringify(loginRes.body)}`);
  }
  const accessToken = (loginRes.body as { accessToken: string }).accessToken;
  this.tokens.set("alice_access", accessToken);

  const expenseRes = await this.post(
    "/api/v1/expenses",
    {
      type: "expense",
      amount: "10.00",
      currency: "USD",
      category: "food",
      description: "Test expense",
      date: "2025-01-01",
    },
    accessToken,
  );
  if (expenseRes.status !== 201) {
    throw new Error(`Failed to create expense: ${JSON.stringify(expenseRes.body)}`);
  }
});

Given("a user {string} exists", async function (this: CustomWorld, username: string) {
  const email = `${username}@example.com`;
  const res = await this.post("/api/v1/auth/register", { username, email, password: DEFAULT_PASSWORD });
  if (res.status !== 201) {
    throw new Error(`Failed to register ${username}: ${JSON.stringify(res.body)}`);
  }
  this.userIds.set(username, (res.body as { id: string }).id);
  this.context[`${username}_password`] = DEFAULT_PASSWORD;
});

// ---- When ----

When("a POST request is sent to {string}", async function (this: CustomWorld, path: string) {
  this.response = await this.post(path, {});
});

When(
  "a POST request is sent to {string} with body:",
  async function (this: CustomWorld, path: string, dataTable: DataTable) {
    this.response = await this.post(path, dataTable.rowsHash());
  },
);

// ---- Then ----

Then("the response status should be {int}", function (this: CustomWorld, status: number) {
  expect(this.response).not.toBeNull();
  expect(this.response?.status).toBe(status);
});

Then("all user accounts should be deleted", async function (this: CustomWorld) {
  const res = await this.post("/api/v1/auth/login", { username: "alice", password: DEFAULT_PASSWORD });
  expect(res.status).toBe(401);
});

Then("all expenses should be deleted", async function () {
  expect(await countExpenses()).toBe(0);
});

// @covers specs/apps/crud/behavior/crud-be/gherkin/test-support/test-api.feature:Reset database clears all user-created data
Then("all attachments should be deleted", async function () {
  expect(await countAttachments()).toBe(0);
});

// @covers specs/apps/crud/behavior/crud-be/gherkin/test-support/test-api.feature:Promote user to admin role
Then("user {string} should have the {string} role", async function (this: CustomWorld, username: string, role: string) {
  const password = (this.context[`${username}_password`] as string) ?? DEFAULT_PASSWORD;
  const loginRes = await this.post("/api/v1/auth/login", { username, password });
  expect(loginRes.status).toBe(200);
  const accessToken = (loginRes.body as { accessToken: string }).accessToken;

  const meRes = await this.get("/api/v1/users/me", accessToken);
  expect(meRes.status).toBe(200);
  expect((meRes.body as { role: string }).role).toBe(role);
});
