@codegen
Feature: Go OpenAPI codegen on fresh checkout

  As a developer working on a clean checkout
  I want the Go codegen target to produce contract types from the OpenAPI 3.1 spec
  So that the CI gate passes without pre-generated contracts committed to git

  Scenario: Fresh Go codegen yields types.gen.go from a 3.1 spec
    Given apps/crud-be-golang-gin/generated-contracts does not exist
    When nx run crud-be-golang-gin:codegen runs with --skip-nx-cache
    Then types.gen.go exists with the contract types
    And nx run crud-be-golang-gin:lint and :test:quick exit 0
