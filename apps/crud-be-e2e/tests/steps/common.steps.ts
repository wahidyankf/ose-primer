import { expect } from "@playwright/test";
import { createBdd } from "playwright-bdd";
import { getResponse } from "../utils/response-store";
import { validateResponseAgainstContract } from "../utils/contract-validator";

const { Given, Then } = createBdd();

Given("the API is running", async () => {
  // No-op: the test suite assumes the API is running at baseURL.
});

// @covers specs/apps/crud/behavior/crud-be/gherkin/admin/admin.feature:Disabled user's access token is rejected with 401
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Logout is idempotent — repeating logout on the same token returns 200
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Delete attachment returns 204
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload attachment to another user's entry returns 403
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:List attachments on another user's entry returns 403
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Delete attachment on another user's entry returns 403
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Delete non-existent attachment returns 404
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Delete an entry returns 204
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Unauthenticated request to create an entry returns 401
// @covers specs/apps/crud/behavior/crud-be/gherkin/security/security.feature:Admin unlocks a locked account
// @covers specs/apps/crud/behavior/crud-be/gherkin/token-management/tokens.feature:Blacklisted access token is rejected with 401 on protected endpoints
// @covers specs/apps/crud/behavior/crud-be/gherkin/token-management/tokens.feature:Deactivating a user revokes all their active tokens
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Successful password change returns 200
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Authenticated user self-deactivates their account
// oxlint-disable-next-line no-empty-pattern
Then("the response status code should be {int}", async ({}, code: number) => {
  const res = getResponse();
  expect(res.status()).toBe(code);

  // Validate 2xx response bodies against the OpenAPI contract schema
  if (code >= 200 && code < 300) {
    try {
      const body = await res.json();
      const url = new URL(res.url());
      // Normalize path: strip IDs from path segments (UUIDs become {id})
      const normalizedPath = url.pathname
        .replace(/\/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/gi, "/{id}")
        .replace(/\/\d+/g, "/{id}");
      const contractError = validateResponseAgainstContract(
        normalizedPath,
        "get", // Will be overridden by specific step validators
        code,
        body,
      );
      if (contractError) {
        console.warn(`[contract] ${contractError}`);
      }
    } catch {
      // Response may not have JSON body (e.g., 204 No Content)
    }
  }
});
