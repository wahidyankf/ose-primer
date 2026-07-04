import { Given, Then } from "@cucumber/cucumber";
import assert from "node:assert/strict";
import type { CustomWorld } from "../world.js";

Given("the API is running", function (this: CustomWorld) {
  // Integration tests call service functions directly — no HTTP server is needed.
  // This step is a no-op: the service runtime is initialised in the BeforeAll hook.
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
  assert.ok(this.response !== null);
  assert.strictEqual(this.response?.status, statusCode);
});
