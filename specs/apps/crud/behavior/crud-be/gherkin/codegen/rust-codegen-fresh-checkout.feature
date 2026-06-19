@codegen
Feature: Rust OpenAPI codegen on fresh checkout

  As a developer working on a clean checkout
  I want the Rust codegen target to produce a buildable crate
  So that the CI gate passes without pre-generated contracts committed to git

  Scenario: Fresh Rust codegen yields Cargo.toml and module wiring
    Given apps/crud-be-rust-axum/generated-contracts does not exist
    When nx run crud-be-rust-axum:codegen runs with --skip-nx-cache
    Then Cargo.toml, src/lib.rs, and src/models/mod.rs exist under generated-contracts/
    And nx run crud-be-rust-axum:lint and :test:quick exit 0
