package com.demobejavx.integration.steps;

import com.demobejavx.support.AppFactory;
import com.demobejavx.support.ScenarioState;
import com.demobejavx.support.ServiceResponse;
import io.cucumber.java.After;
import io.cucumber.java.AfterAll;
import io.cucumber.java.Before;
import io.cucumber.java.BeforeAll;
import io.cucumber.java.en.Given;
import io.cucumber.java.en.Then;
import io.vertx.core.json.JsonObject;
import org.junit.jupiter.api.Assertions;

public class CommonSteps {

    private final ScenarioState state;

    public CommonSteps(ScenarioState state) {
        this.state = state;
    }

    @BeforeAll
    public static void deployApp() throws Exception {
        AppFactory.deploy();
    }

    @AfterAll
    public static void closeApp() {
        AppFactory.close();
    }

    @Before
    public void resetState() throws Exception {
        AppFactory.reset();
        state.reset();
    }

    @After
    public void afterScenario() {
        // no-op — cleanup done in @Before
    }

    @Given("the API is running")
    public void theApiIsRunning() {
        // Repositories are initialised in @BeforeAll via AppFactory.deploy()
    }

    // @covers specs/apps/crud/behavior/crud-be/gherkin/admin/admin.feature:Disabled user's access token is rejected with 401
    // @covers specs/apps/crud/behavior/crud-be/gherkin/security/security.feature:Admin unlocks a locked account
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Delete attachment returns 204
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload attachment to another user's entry returns 403
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:List attachments on another user's entry returns 403
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Delete attachment on another user's entry returns 403
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Delete non-existent attachment returns 404
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Delete an entry returns 204
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Unauthenticated request to create an entry returns 401
    // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Logout is idempotent — repeating logout on the same token returns 200
    // @covers specs/apps/crud/behavior/crud-be/gherkin/token-management/tokens.feature:Blacklisted access token is rejected with 401 on protected endpoints
    // @covers specs/apps/crud/behavior/crud-be/gherkin/token-management/tokens.feature:Deactivating a user revokes all their active tokens
    // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Successful password change returns 200
    // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Authenticated user self-deactivates their account
    @Then("the response status code should be {int}")
    public void theResponseStatusCodeShouldBe(int expected) {
        ServiceResponse response = state.getLastResponse();
        Assertions.assertNotNull(response, "No response stored in state");
        Assertions.assertEquals(expected, response.statusCode(),
                "Expected status " + expected + " but got " + response.statusCode()
                        + ". Body: " + response.body());
    }

    // @covers specs/apps/crud/behavior/crud-be/gherkin/admin/admin.feature:List all users returns a paginated response
    // @covers specs/apps/crud/behavior/crud-be/gherkin/admin/admin.feature:Admin generates a password-reset token for a user
    // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Successful login returns access token and refresh token
    // @covers specs/apps/crud/behavior/crud-be/gherkin/security/security.feature:Unlocked account can log in with correct password
    // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Successful registration response includes non-null user ID
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload JPEG image returns 201 with attachment metadata
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload PDF document returns 201 with attachment metadata
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Create expense entry with amount and currency returns 201 with entry ID
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Create income entry with amount and currency returns 201 with entry ID
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:List own entries returns a paginated response
    // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Successful refresh returns a new access token and refresh token
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/unit-handling.feature:Expense without quantity and unit fields is accepted
    // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Get own profile returns username, email, and display name
    @Then("the response body should contain a non-null {string} field")
    public void theResponseBodyShouldContainNonNullField(String field) {
        ServiceResponse response = state.getLastResponse();
        Assertions.assertNotNull(response);
        JsonObject body = response.body();
        Assertions.assertNotNull(body, "Response body is null");
        String mapped = mapFieldName(field);
        Assertions.assertNotNull(body.getValue(mapped),
                "Expected non-null field '" + mapped + "' in body: " + body.encode());
    }

    // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Successful login response includes token type "Bearer"
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:USD expense amount preserves two decimal places
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:IDR expense amount is stored and returned as a whole number
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Get own entry by ID returns amount, currency, category, description, date, and type
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Update an entry amount and description returns 200
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/unit-handling.feature:Create expense with metric unit "liter" stores quantity and unit correctly
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/unit-handling.feature:Create expense with imperial unit "gallon" stores quantity and unit correctly
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary returns income total, expense total, and net for a period
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:Income entries are excluded from expense total
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:Expense entries are excluded from income total
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary filters by currency without cross-currency mixing
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary for a period with no entries returns zero totals
    // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Update display name succeeds
    @Then("the response body should contain {string} equal to {string}")
    public void theResponseBodyShouldContainStringEqualTo(String field, String value) {
        ServiceResponse response = state.getLastResponse();
        Assertions.assertNotNull(response);
        JsonObject body = response.body();
        Assertions.assertNotNull(body);
        String mapped = mapFieldName(field);
        Assertions.assertEquals(value, body.getString(mapped),
                "Field '" + mapped + "' expected '" + value + "' but was '"
                        + body.getString(mapped) + "'");
    }

    // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Successful registration returns created user profile without password
    @Then("the response body should not contain a {string} field")
    public void theResponseBodyShouldNotContainField(String field) {
        ServiceResponse response = state.getLastResponse();
        Assertions.assertNotNull(response);
        JsonObject body = response.body();
        Assertions.assertNotNull(body);
        String mapped = mapFieldName(field);
        Assertions.assertNull(body.getValue(mapped),
                "Expected no field '" + mapped + "' but found: " + body.getValue(mapped));
    }

    // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login with wrong password
    // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login for non-existent user
    // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Reject password change with incorrect old password
    @Then("the response body should contain an error message about invalid credentials")
    public void responseContainsInvalidCredentials() {
        checkErrorResponse("Invalid credentials", "deactivated", "disabled", "locked");
    }

    // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login for deactivated account
    // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Refresh fails for a deactivated user
    // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Self-deactivated user cannot log in with previous credentials
    @Then("the response body should contain an error message about account deactivation")
    public void responseContainsAccountDeactivation() {
        checkErrorResponse("deactivated", "Account deactivated", "disabled", "Disabled");
    }

    // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Reject refresh with an expired refresh token
    @Then("the response body should contain an error message about token expiration")
    public void responseContainsTokenExpiration() {
        checkErrorResponse("expired", "Token expired");
    }

    // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Original refresh token is rejected after rotation (single-use)
    @Then("the response body should contain an error message about invalid token")
    public void responseContainsInvalidToken() {
        checkErrorResponse("invalid", "Token invalid", "Invalid");
    }

    // @covers specs/apps/crud/behavior/crud-be/gherkin/security/security.feature:Reject password shorter than 12 characters
    // @covers specs/apps/crud/behavior/crud-be/gherkin/security/security.feature:Reject password with no special character
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:Unsupported currency code returns 400
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:Malformed currency code returns 400
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/currency-handling.feature:Negative amount is rejected with 400
    // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration with invalid email format
    // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration with empty password
    // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration with weak password — no uppercase letter
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload unsupported file type returns 415
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/unit-handling.feature:Create expense with an unsupported unit returns 400
    @Then("the response body should contain a validation error for {string}")
    public void theResponseBodyShouldContainValidationError(String field) {
        ServiceResponse response = state.getLastResponse();
        Assertions.assertNotNull(response);
        JsonObject body = response.body();
        Assertions.assertNotNull(body, "Expected JSON body for validation error");
        String msg = body.getString("message", "");
        String fieldProp = body.getString("field", "");
        boolean containsField = msg.toLowerCase().contains(field.toLowerCase())
                || field.equalsIgnoreCase(fieldProp);
        Assertions.assertTrue(containsField,
                "Expected validation error for '" + field + "' but got: " + body.encode());
    }

    // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration when username already exists
    @Then("the response body should contain an error message about duplicate username")
    public void responseContainsDuplicateUsername() {
        checkErrorResponse("already", "Username");
    }

    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload file exceeding the size limit returns 413
    @Then("the response body should contain an error message about file size")
    public void responseContainsFileSizeError() {
        checkErrorResponse("size", "maximum", "10MB");
    }

    private void checkErrorResponse(String... fragments) {
        ServiceResponse response = state.getLastResponse();
        Assertions.assertNotNull(response);
        JsonObject body = response.body();
        if (body == null) {
            return;
        }
        String message = body.getString("message", "").toLowerCase();
        boolean found = false;
        for (String fragment : fragments) {
            if (message.contains(fragment.toLowerCase())) {
                found = true;
                break;
            }
        }
        Assertions.assertTrue(found,
                "Expected error message containing one of " + java.util.Arrays.toString(fragments)
                        + " but got: " + message);
    }

    private String mapFieldName(String field) {
        return switch (field) {
            case "accessToken" -> "accessToken";
            case "refreshToken" -> "refreshToken";
            default -> field;
        };
    }
}
