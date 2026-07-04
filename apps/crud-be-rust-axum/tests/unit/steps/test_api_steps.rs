//! Step definitions for `specs/apps/crud/behavior/crud-be/gherkin/test-support/test-api.feature`.
//!
//! This unit tier calls service functions directly (see `world.rs`) rather than
//! routing through the real Axum app in `src/app.rs`, so `ENABLE_TEST_API` (which
//! gates that router's `/api/v1/test/*` routes) has no effect here — the
//! Background step below is a documented no-op for that reason.

use std::collections::HashMap;

use cucumber::gherkin::Step;
use cucumber::{given, then, when};

use crate::world::AppWorld;

// ---------------------------------------------------------------------------
// Background
// ---------------------------------------------------------------------------

#[given("the test API is enabled via ENABLE_TEST_API environment variable")]
async fn test_api_enabled(_world: &mut AppWorld) {
    // No-op: this unit tier bypasses the real Axum router (see module docs).
}

// ---------------------------------------------------------------------------
// Given
// ---------------------------------------------------------------------------

#[given("users and expenses exist in the database")]
async fn users_and_expenses_exist(world: &mut AppWorld) {
    world
        .svc_register("resetuser", "resetuser@example.com", "Str0ng#Pass1")
        .await;
    assert_eq!(
        world.last_status, 201,
        "failed to register resetuser: {}",
        world.last_body
    );

    world.svc_login("resetuser", "Str0ng#Pass1").await;
    assert_eq!(
        world.last_status, 200,
        "failed to login resetuser: {}",
        world.last_body
    );
    let bearer = world
        .last_body
        .get("accessToken")
        .and_then(|v| v.as_str())
        .map(|t| format!("Bearer {t}"))
        .unwrap_or_default();

    world
        .svc_create_expense(
            &bearer,
            "10.00",
            "USD",
            "food",
            "Test expense",
            "2025-01-01",
            "expense",
            None,
            None,
        )
        .await;
    assert_eq!(
        world.last_status, 201,
        "failed to create expense: {}",
        world.last_body
    );
}

#[given(expr = "a user {string} exists")]
async fn a_user_exists(world: &mut AppWorld, username: String) {
    let email = format!("{username}@example.com");
    world.svc_register(&username, &email, "Str0ng#Pass1").await;
    assert_eq!(
        world.last_status, 201,
        "failed to register {username}: {}",
        world.last_body
    );
}

// ---------------------------------------------------------------------------
// When
// ---------------------------------------------------------------------------

#[when(expr = "a POST request is sent to {string}")]
async fn post_request(world: &mut AppWorld, path: String) {
    if path.ends_with("/test/reset-db") {
        world.svc_test_reset_db().await;
    } else {
        panic!("unhandled test-support POST path: {path}");
    }
}

#[when(expr = "a POST request is sent to {string} with body:")]
async fn post_request_with_body(world: &mut AppWorld, path: String, step: &Step) {
    let table = step
        .table
        .as_ref()
        .expect("expected a data table on this step");
    let params: HashMap<&str, &str> = table
        .rows
        .iter()
        .filter_map(|row| match row.as_slice() {
            [key, value] => Some((key.as_str(), value.as_str())),
            _ => None,
        })
        .collect();

    if path.ends_with("/test/promote-admin") {
        let username = params
            .get("username")
            .expect("missing 'username' in request body table");
        world.svc_test_promote_admin(username).await;
    } else {
        panic!("unhandled test-support POST-with-body path: {path}");
    }
}

// ---------------------------------------------------------------------------
// Then
// ---------------------------------------------------------------------------

#[then(expr = "the response status should be {int}")]
async fn response_status_should_be(world: &mut AppWorld, code: u16) {
    assert_eq!(
        world.last_status, code,
        "Expected status {code}, got {}, body: {}",
        world.last_status, world.last_body
    );
}

#[then("all user accounts should be deleted")]
async fn all_users_deleted(world: &mut AppWorld) {
    assert_eq!(
        world.test_user_count().await,
        0,
        "Expected all user accounts to be deleted"
    );
}

#[then("all expenses should be deleted")]
async fn all_expenses_deleted(world: &mut AppWorld) {
    assert_eq!(
        world.test_expense_count().await,
        0,
        "Expected all expenses to be deleted"
    );
}

#[then("all attachments should be deleted")]
async fn all_attachments_deleted(world: &mut AppWorld) {
    assert_eq!(
        world.test_attachment_count().await,
        0,
        "Expected all attachments to be deleted"
    );
}

#[then(expr = "user {string} should have the {string} role")]
async fn user_should_have_role(world: &mut AppWorld, username: String, expected_role: String) {
    let user = world
        .state
        .user_repo
        .find_by_username(&username)
        .await
        .expect("repo lookup should not fail")
        .unwrap_or_else(|| panic!("user '{username}' not found"));
    assert_eq!(
        user.role, expected_role,
        "Expected user '{username}' to have role '{expected_role}', got '{}'",
        user.role
    );
}
