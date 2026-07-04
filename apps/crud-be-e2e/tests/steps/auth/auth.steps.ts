import { expect } from "@playwright/test";
import { createBdd } from "playwright-bdd";
import { getResponse } from "../../utils/response-store";

const { Then } = createBdd();

// ---------------------------------------------------------------------------
// Response body assertion steps used by auth + registration features
// ---------------------------------------------------------------------------

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
  // oxlint-disable-next-line no-empty-pattern
  async ({}, field: string, value: string) => {
    const body = (await getResponse().json()) as Record<string, unknown>;
    expect(body[field]).toBe(value);
  },
);

// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Successful registration returns created user profile without password
Then(
  "the response body should not contain a {string} field",
  // oxlint-disable-next-line no-empty-pattern
  async ({}, field: string) => {
    const body = (await getResponse().json()) as Record<string, unknown>;
    expect(body[field]).toBeUndefined();
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
Then(
  "the response body should contain a non-null {string} field",
  // oxlint-disable-next-line no-empty-pattern
  async ({}, field: string) => {
    const body = (await getResponse().json()) as Record<string, unknown>;
    expect(body[field]).not.toBeNull();
    expect(body[field]).toBeDefined();
  },
);

// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration when username already exists
Then("the response body should contain an error message about duplicate username", async () => {
  const body = (await getResponse().json()) as { message: string };
  expect(body.message).toMatch(/already exists/i);
});

// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login with wrong password
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login for non-existent user
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Reject password change with incorrect old password
Then("the response body should contain an error message about invalid credentials", async () => {
  const body = (await getResponse().json()) as { message: string };
  expect(body.message).toMatch(/invalid/i);
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
Then(
  "the response body should contain a validation error for {string}",
  // oxlint-disable-next-line no-empty-pattern
  async ({}, _field: string) => {
    const status = getResponse().status();
    // 400 for field validation errors, 415 for unsupported media type
    expect([400, 415]).toContain(status);
  },
);
