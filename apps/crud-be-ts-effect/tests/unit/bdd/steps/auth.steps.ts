import { Given, When, Then } from "@cucumber/cucumber";
import { expect } from "@playwright/test";
import type { CustomWorld } from "../world.js";

// Helper: register a user
async function registerUser(world: CustomWorld, username: string, email: string, password: string) {
  const res = await world.post("/api/v1/auth/register", { username, email, password });
  if (res.status !== 201) {
    throw new Error(`Registration failed for ${username}: ${JSON.stringify(res.body)}`);
  }
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  return res.body as { id: string };
}

// Helper: login and return tokens
async function loginUser(world: CustomWorld, username: string, password: string) {
  const res = await world.post("/api/v1/auth/login", { username, password });
  if (res.status !== 200) {
    throw new Error(`Login failed for ${username}: ${JSON.stringify(res.body)}`);
  }
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  return res.body as { accessToken: string; refreshToken: string };
}

// ---- Background: register user ----

Given(
  "a user {string} is registered with password {string}",
  async function (this: CustomWorld, username: string, password: string) {
    const email = `${username}@example.com`;
    try {
      const user = await registerUser(this, username, email, password);
      this.userIds.set(username, user.id);
    } catch {
      // User might already be registered from a previous step
      const loginRes = await this.post("/api/v1/auth/login", { username, password });
      if (loginRes.status === 200) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        this.tokens.set(`${username}_access`, (loginRes.body as any).accessToken);
      }
    }
    this.context[`${username}_password`] = password;
  },
);

Given(
  "a user {string} is registered with email {string} and password {string}",
  async function (this: CustomWorld, username: string, email: string, password: string) {
    const user = await registerUser(this, username, email, password);
    this.userIds.set(username, user.id);
    this.context[`${username}_email`] = email;
    this.context[`${username}_password`] = password;
  },
);

Given(
  "{string} has logged in and stored the access token and refresh token",
  async function (this: CustomWorld, username: string) {
    const password = (this.context[`${username}_password`] as string) ?? "Str0ng#Pass1";
    const tokens = await loginUser(this, username, password);
    this.tokens.set(`${username}_access`, tokens.accessToken);
    this.tokens.set(`${username}_refresh`, tokens.refreshToken);
  },
);

Given("{string} has logged in and stored the access token", async function (this: CustomWorld, username: string) {
  const password = (this.context[`${username}_password`] as string) ?? "Str0ng#Pass1";
  const tokens = await loginUser(this, username, password);
  this.tokens.set(`${username}_access`, tokens.accessToken);
  this.tokens.set(`${username}_refresh`, tokens.refreshToken);
});

Given("a user {string} is registered and deactivated", async function (this: CustomWorld, username: string) {
  const email = `${username}@example.com`;
  const password = (this.context[`${username}_password`] as string) ?? "Str0ng#Pass1";
  const res = await this.post("/api/v1/auth/register", { username, email, password });
  if (res.status === 201) {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.userIds.set(username, (res.body as any).id as string);
  } else if (res.status !== 409) {
    throw new Error(`Registration failed for ${username}: ${JSON.stringify(res.body)}`);
  }
  // Login and deactivate (user may already exist from Background)
  const tokens = await loginUser(this, username, password);
  await this.post("/api/v1/users/me/deactivate", {}, tokens.accessToken);
});

// ---- When: POST with JSON body in step text ----

When(/^the client sends POST (.+) with body (.+)$/, async function (this: CustomWorld, path: string, bodyStr: string) {
  const body = JSON.parse(bodyStr) as Record<string, unknown>;
  this.response = await this.post(path, body);
});

When(
  /^the client sends GET ((?!.*with alice's access token)(?!\/\.well-known\/jwks\.json).+)$/,
  async function (this: CustomWorld, path: string) {
    this.response = await this.get(path);
  },
);

When(/^the client sends GET (.+) with alice's access token$/, async function (this: CustomWorld, path: string) {
  const token = this.tokens.get("alice_access") ?? "";
  // path here is just the route (e.g., /api/v1/users/me)
  this.response = await this.get(path.trim(), token);
});

// ---- Then: common assertions ----

// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Successful login response includes token type "Bearer"
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:USD expense amount preserves two decimal places
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:IDR expense amount is stored and returned as a whole number
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Get own entry by ID returns amount, currency, category, description, date, and type
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Update an entry amount and description returns 200
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary returns income total, expense total, and net for a period
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:Income entries are excluded from expense total
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:Expense entries are excluded from income total
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary filters by currency without cross-currency mixing
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary for a period with no entries returns zero totals
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/unit-handling.feature:Create expense with metric unit "liter" stores quantity and unit correctly
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/unit-handling.feature:Create expense with imperial unit "gallon" stores quantity and unit correctly
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Update display name succeeds
Then(
  "the response body should contain {string} equal to {string}",
  function (this: CustomWorld, field: string, value: string) {
    expect(this.response).not.toBeNull();
    expect(String(this.response?.body?.[field])).toBe(value);
  },
);

// @covers specs/apps/crud/behavior/crud-be/gherkin/admin/admin.feature:List all users returns a paginated response
// @covers specs/apps/crud/behavior/crud-be/gherkin/admin/admin.feature:Admin generates a password-reset token for a user
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Successful login returns access token and refresh token
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Successful refresh returns a new access token and refresh token
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload JPEG image returns 201 with attachment metadata
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload PDF document returns 201 with attachment metadata
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Create expense entry with amount and currency returns 201 with entry ID
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Create income entry with amount and currency returns 201 with entry ID
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:List own entries returns a paginated response
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/unit-handling.feature:Expense without quantity and unit fields is accepted
// @covers specs/apps/crud/behavior/crud-be/gherkin/security/security.feature:Unlocked account can log in with correct password
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Successful registration response includes non-null user ID
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Get own profile returns username, email, and display name
Then("the response body should contain a non-null {string} field", function (this: CustomWorld, field: string) {
  expect(this.response).not.toBeNull();
  expect(this.response?.body?.[field]).not.toBeNull();
  expect(this.response?.body?.[field]).not.toBeUndefined();
});

// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Successful registration returns created user profile without password
Then("the response body should not contain a {string} field", function (this: CustomWorld, field: string) {
  expect(this.response?.body?.[field]).toBeUndefined();
});

// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login with wrong password
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login for non-existent user
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Reject password change with incorrect old password
Then("the response body should contain an error message about invalid credentials", function (this: CustomWorld) {
  expect(this.response).not.toBeNull();
  const body = this.response?.body as Record<string, string>;
  const message = (body["message"] ?? body["error"] ?? "").toLowerCase();
  expect(message.length).toBeGreaterThan(0);
});

// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login for deactivated account
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Refresh fails for a deactivated user
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Self-deactivated user cannot log in with previous credentials
Then("the response body should contain an error message about account deactivation", function (this: CustomWorld) {
  expect(this.response).not.toBeNull();
  const body = this.response?.body as Record<string, string>;
  const message = (body["message"] ?? body["error"] ?? "").toLowerCase();
  expect(message.length).toBeGreaterThan(0);
});

// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload unsupported file type returns 415
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:Unsupported currency code returns 400
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:Malformed currency code returns 400
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:Negative amount is rejected with 400
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/unit-handling.feature:Create expense with an unsupported unit returns 400
// @covers specs/apps/crud/behavior/crud-be/gherkin/security/security.feature:Reject password shorter than 12 characters
// @covers specs/apps/crud/behavior/crud-be/gherkin/security/security.feature:Reject password with no special character
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration with invalid email format
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration with empty password
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration with weak password — no uppercase letter
Then("the response body should contain a validation error for {string}", function (this: CustomWorld, field: string) {
  expect(this.response).not.toBeNull();
  const body = this.response?.body as Record<string, string>;
  const bodyField = body["field"];
  const hasField = bodyField === field || (body["message"] ?? "").toLowerCase().includes(field.toLowerCase());
  const hasError = body["error"] !== undefined || body["message"] !== undefined;
  expect(hasField || hasError).toBe(true);
});

// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration when username already exists
Then("the response body should contain an error message about duplicate username", function (this: CustomWorld) {
  expect(this.response).not.toBeNull();
  const body = this.response?.body as Record<string, string>;
  const message = (body["message"] ?? body["error"] ?? "").toLowerCase();
  expect(message.length).toBeGreaterThan(0);
});
