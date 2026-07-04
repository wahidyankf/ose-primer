import { Given, Then } from "@cucumber/cucumber";
import { expect } from "@playwright/test";
import type { CustomWorld } from "../world.js";
import { TEST_PORT } from "../hooks.js";

Given("the API is running", async function (this: CustomWorld) {
  // Server is started in BeforeAll hook
  this.baseUrl = `http://localhost:${TEST_PORT}`;
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
Then("the response status code should be {int}", function (this: CustomWorld, statusCode: number) {
  expect(this.response).not.toBeNull();
  expect(this.response?.status).toBe(statusCode);
});
