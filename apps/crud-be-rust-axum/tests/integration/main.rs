// Test-harness crate: relax the pedantic/nursery groups and the
// `unwrap`/`expect`/`panic` denies that are idiomatic in test and BDD
// step-definition code. Production code keeps the strict crate-wide lints.
#![allow(clippy::pedantic, clippy::nursery)]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod steps;
mod world;

use cucumber::World as _;
use world::AppWorld;

#[tokio::main]
async fn main() {
    AppWorld::cucumber()
        .max_concurrent_scenarios(Some(1))
        .run("/specs/apps/crud/behavior/crud-be/gherkin")
        .await;
}
