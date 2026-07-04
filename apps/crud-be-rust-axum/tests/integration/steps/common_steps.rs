use cucumber::{given, then, when};

use crate::world::AppWorld;

#[given("the API is running")]
async fn api_is_running(_world: &mut AppWorld) {
    // AppState is already initialized in AppWorld::new()
}

#[when("an operations engineer sends GET /health")]
async fn get_health(world: &mut AppWorld) {
    world.svc_health().await;
}

#[when("an unauthenticated engineer sends GET /health")]
async fn get_health_unauth(world: &mut AppWorld) {
    world.svc_health().await;
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
#[then(expr = "the response status code should be {int}")]
async fn check_status(world: &mut AppWorld, code: u16) {
    assert_eq!(
        world.last_status, code,
        "Expected status {}, got {}, body: {}",
        code, world.last_status, world.last_body
    );
}

// @covers specs/apps/crud/behavior/crud-be/gherkin/health/health-check.feature:Health endpoint reports the service as UP
#[then(expr = "the health status should be {string}")]
async fn check_health_status(world: &mut AppWorld, expected: String) {
    let status = world
        .last_body
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert_eq!(status, expected.as_str(), "Health status mismatch");
}

// @covers specs/apps/crud/behavior/crud-be/gherkin/health/health-check.feature:Anonymous health check does not expose component details
#[then("the response should not include detailed component health information")]
async fn no_component_details(world: &mut AppWorld) {
    assert!(
        world.last_body.get("components").is_none(),
        "Response should not include 'components'"
    );
    assert!(
        world.last_body.get("details").is_none(),
        "Response should not include 'details'"
    );
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
#[then(expr = "the response body should contain {string} equal to {string}")]
async fn body_field_equals(world: &mut AppWorld, field: String, value: String) {
    let actual = match world.last_body.get(&field) {
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Number(n)) => n.to_string(),
        Some(v) => v.to_string().trim_matches('"').to_string(),
        None => String::new(),
    };
    assert_eq!(
        actual, value,
        "Field '{field}' expected '{value}', got '{actual}', body: {}",
        world.last_body
    );
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
#[then(expr = "the response body should contain a non-null {string} field")]
async fn body_field_non_null(world: &mut AppWorld, field: String) {
    let val = world.last_body.get(&field);
    assert!(
        val.is_some() && !val.unwrap().is_null(),
        "Field '{field}' should be non-null in body: {}",
        world.last_body
    );
}

// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Successful registration returns created user profile without password
#[then(expr = "the response body should not contain a {string} field")]
async fn body_field_not_present(world: &mut AppWorld, field: String) {
    assert!(
        world.last_body.get(&field).is_none(),
        "Field '{field}' should not be present in body: {}",
        world.last_body
    );
}

// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login with wrong password
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login for non-existent user
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Reject password change with incorrect old password
#[then("the response body should contain an error message about invalid credentials")]
async fn error_invalid_credentials(world: &mut AppWorld) {
    let msg = world
        .last_body
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(
        !msg.is_empty(),
        "Expected error message about invalid credentials, got: {}",
        world.last_body
    );
}

// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/password-login.feature:Reject login for deactivated account
// @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Refresh fails for a deactivated user
// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Self-deactivated user cannot log in with previous credentials
#[then("the response body should contain an error message about account deactivation")]
async fn error_account_deactivated(world: &mut AppWorld) {
    let msg = world
        .last_body
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(
        !msg.is_empty(),
        "Expected error message about deactivation, got: {}",
        world.last_body
    );
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
#[then(expr = "the response body should contain a validation error for {string}")]
async fn validation_error_for_field(world: &mut AppWorld, field: String) {
    let msg = world
        .last_body
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(
        msg.contains(field.as_str()),
        "Expected validation error for '{field}', got message: '{msg}', body: {}",
        world.last_body
    );
}

// @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration when username already exists
#[then("the response body should contain an error message about duplicate username")]
async fn error_duplicate_username(world: &mut AppWorld) {
    let msg = world
        .last_body
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(
        !msg.is_empty(),
        "Expected error message, got: {}",
        world.last_body
    );
}

#[given(expr = "a user {string} is registered with password {string}")]
async fn register_user_with_password(world: &mut AppWorld, username: String, password: String) {
    let email = format!("{username}@example.com");
    world.svc_register(&username, &email, &password).await;
    if world.last_status == 201 && username == "alice" {
        world.alice_id = world
            .last_body
            .get("id")
            .and_then(|v| v.as_str())
            .and_then(|s| uuid::Uuid::parse_str(s).ok());
    }
}

#[given(expr = "{string} has logged in and stored the access token and refresh token")]
async fn login_store_both_tokens(world: &mut AppWorld, username: String) {
    world.svc_login(&username, "Str0ng#Pass1").await;
    if world.last_status == 200 {
        world.auth_token = world
            .last_body
            .get("accessToken")
            .and_then(|v| v.as_str())
            .map(String::from);
        world.refresh_token = world
            .last_body
            .get("refreshToken")
            .and_then(|v| v.as_str())
            .map(String::from);
        if username == "alice" {
            let token = world.auth_token.clone().unwrap_or_default();
            world.svc_get_profile(&format!("Bearer {token}")).await;
            world.user_id = world
                .last_body
                .get("id")
                .and_then(|v| v.as_str())
                .and_then(|s| uuid::Uuid::parse_str(s).ok());
            world.alice_id = world.user_id;
        }
    }
}

#[given(expr = "{string} has logged in and stored the access token")]
async fn login_store_access_token(world: &mut AppWorld, username: String) {
    let passwords = ["Str0ng#Pass1", "Str0ng#Pass2"];
    for password in passwords {
        world.svc_login(&username, password).await;
        if world.last_status == 200 {
            world.auth_token = world
                .last_body
                .get("accessToken")
                .and_then(|v| v.as_str())
                .map(String::from);
            world.refresh_token = world
                .last_body
                .get("refreshToken")
                .and_then(|v| v.as_str())
                .map(String::from);
            if username == "alice" {
                let token = world.auth_token.clone().unwrap_or_default();
                world.svc_get_profile(&format!("Bearer {token}")).await;
                world.user_id = world
                    .last_body
                    .get("id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| uuid::Uuid::parse_str(s).ok());
                world.alice_id = world.user_id;
            }
            break;
        }
    }
}
