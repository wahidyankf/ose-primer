// Test-harness crate: relax the pedantic/nursery groups and the
// `unwrap`/`expect`/`panic` denies that are idiomatic in test and BDD
// step-definition code. Production code keeps the strict crate-wide lints.
#![allow(clippy::pedantic, clippy::nursery)]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod in_memory_repos;
mod steps;
mod world;

use cucumber::World as _;
use world::AppWorld;

#[tokio::main]
async fn main() {
    // `@codegen`-tagged scenarios (see specs/apps/crud/behavior/crud-be/gherkin/codegen/)
    // assert fresh-checkout code-generation output for OTHER language variants (or, for the
    // Rust one, this project's own `nx run crud-be-rust-axum:codegen` build/CI step) — not
    // per-scenario runtime behavior a language's BDD *unit* binary can implement. No
    // `crud-be-*` project implements these steps; they're excluded from every language's unit
    // tier the same way (see e.g. `--exclude-dir codegen` in this project's project.json).
    AppWorld::cucumber()
        .fail_on_skipped()
        .filter_run_and_exit(
            "../../specs/apps/crud/behavior/crud-be/gherkin",
            |feature, _rule, scenario| {
                // `@codegen` is declared at the Feature level in both codegen/*.feature
                // files, not inherited into `scenario.tags` by this gherkin parser
                // version — check both to catch it regardless of where it's tagged.
                let is_codegen = |tags: &[String]| tags.iter().any(|t| t == "codegen");
                !is_codegen(&feature.tags) && !is_codegen(&scenario.tags)
            },
        )
        .await;
}
