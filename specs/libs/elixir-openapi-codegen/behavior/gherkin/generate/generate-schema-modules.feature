@wip
Feature: Elixir struct module generation from an OpenAPI spec
  As an Elixir backend developer
  I want OpenApiCodegen.generate to produce struct modules from components.schemas
  So that my application can consume contract-generated request/response types

  Scenario: Generating a schema with required and optional properties
    Given a bundled OpenAPI spec whose "User" schema requires "id", "username", and "email"
    When I call OpenApiCodegen.generate with the spec path, an output directory, and namespace "MyApp.Schemas"
    Then the result is "{:ok, paths}" with one written file path per schema
    And the generated "user.ex" file declares "@enforce_keys [:id, :username, :email]"

  Scenario: A spec with no components key fails to generate
    Given a bundled OpenAPI spec with no "components" key
    When I call OpenApiCodegen.generate with the spec path and an output directory
    Then the result is "{:error, reason}"

  Scenario: A spec with components but no schemas key fails to generate
    Given a bundled OpenAPI spec with a "components" key but no "schemas" key
    When I call OpenApiCodegen.generate with the spec path and an output directory
    Then the result is "{:error, reason}"
