package com.demobektkt.integration.steps

import io.cucumber.java.en.Given
import io.cucumber.java.en.Then
import io.cucumber.java.en.When
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue

class CommonSteps {

  @Given("the API is running")
  fun theApiIsRunning() {
    TestDatabase.init()
  }

  @When("an operations engineer sends GET \\/health")
  fun operationsEngineerSendsGetHealth() {
    val (status, body) = ServiceDispatcher.health()
    TestWorld.lastResponseStatus = status
    TestWorld.lastResponseBody = body
  }

  @When("an unauthenticated engineer sends GET \\/health")
  fun unauthenticatedEngineerSendsGetHealth() {
    val (status, body) = ServiceDispatcher.health()
    TestWorld.lastResponseStatus = status
    TestWorld.lastResponseBody = body
  }

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
  @Then("the response status code should be {int}")
  fun theResponseStatusCodeShouldBe(code: Int) {
    assertEquals(
      code,
      TestWorld.lastResponseStatus,
      "Expected status $code but got ${TestWorld.lastResponseStatus}. Body: ${TestWorld.lastResponseBody}",
    )
  }

  // @covers specs/apps/crud/behavior/crud-be/gherkin/health/health-check.feature:Health endpoint reports the service as UP
  @Then("the health status should be {string}")
  fun theHealthStatusShouldBe(status: String) {
    val actual = JsonHelper.getString(TestWorld.lastResponseBody, "status")
    assertEquals(
      status,
      actual,
      "Expected health status '$status' in: ${TestWorld.lastResponseBody}",
    )
  }

  // @covers specs/apps/crud/behavior/crud-be/gherkin/health/health-check.feature:Anonymous health check does not expose component details
  @Then("the response should not include detailed component health information")
  fun theResponseShouldNotIncludeDetailedComponentHealthInformation() {
    val body = TestWorld.lastResponseBody
    assertTrue(
      !body.contains("components") && !body.contains("db"),
      "Should not include component details: $body",
    )
  }

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
  @Then("the response body should contain a non-null {string} field")
  fun theResponseBodyShouldContainNonNullField(field: String) {
    JsonHelper.assertNonNull(TestWorld.lastResponseBody, field)
  }

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
  @Then("the response body should contain {string} equal to {string}")
  fun theResponseBodyShouldContainFieldEqualTo(field: String, expected: String) {
    JsonHelper.assertStringField(TestWorld.lastResponseBody, field, expected)
  }

  // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Successful registration returns created user profile without password
  @Then("the response body should not contain a {string} field")
  fun theResponseBodyShouldNotContainField(field: String) {
    JsonHelper.assertNotPresent(TestWorld.lastResponseBody, field)
  }

  // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login with wrong password
  // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login for non-existent user
  // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Reject password change with incorrect old password
  @Then("the response body should contain an error message about invalid credentials")
  fun theResponseBodyShouldContainInvalidCredentialsError() {
    val body = TestWorld.lastResponseBody
    assertTrue(
      body.contains("Invalid credentials") ||
        body.contains("invalid") ||
        body.contains("Unauthorized"),
      "Expected invalid credentials message in: $body",
    )
  }

  // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login for deactivated account
  // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Refresh fails for a deactivated user
  // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Self-deactivated user cannot log in with previous credentials
  @Then("the response body should contain an error message about account deactivation")
  fun theResponseBodyShouldContainAccountDeactivationError() {
    val body = TestWorld.lastResponseBody
    assertTrue(
      body.contains("deactivated") || body.contains("INACTIVE") || body.contains("inactive"),
      "Expected account deactivation message in: $body",
    )
  }

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
  @Then("the response body should contain a validation error for {string}")
  fun theResponseBodyShouldContainValidationErrorFor(field: String) {
    val body = TestWorld.lastResponseBody
    assertTrue(body.contains(field), "Expected validation error for '$field' in: $body")
  }

  // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration when username already exists
  @Then("the response body should contain an error message about duplicate username")
  fun theResponseBodyShouldContainDuplicateUsernameError() {
    val body = TestWorld.lastResponseBody
    assertTrue(
      body.contains("already exists") || body.contains("duplicate") || body.contains("Conflict"),
      "Expected duplicate username message in: $body",
    )
  }

  // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Reject refresh with an expired refresh token
  @Then("the response body should contain an error message about token expiration")
  fun theResponseBodyShouldContainTokenExpirationError() {
    val body = TestWorld.lastResponseBody
    assertTrue(
      body.contains("expired") || body.contains("Unauthorized") || body.contains("token"),
      "Expected token expiration message in: $body",
    )
  }

  // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Original refresh token is rejected after rotation (single-use)
  @Then("the response body should contain an error message about invalid token")
  fun theResponseBodyShouldContainInvalidTokenError() {
    val body = TestWorld.lastResponseBody
    assertTrue(
      body.contains("invalid") || body.contains("Invalid") || body.contains("Unauthorized"),
      "Expected invalid token message in: $body",
    )
  }

  // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload file exceeding the size limit returns 413
  @Then("the response body should contain an error message about file size")
  fun theResponseBodyShouldContainFileSizeError() {
    val body = TestWorld.lastResponseBody
    assertTrue(
      body.contains("size") || body.contains("limit") || body.contains("large"),
      "Expected file size message in: $body",
    )
  }
}
