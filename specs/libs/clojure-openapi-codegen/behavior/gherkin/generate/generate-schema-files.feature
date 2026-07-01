@wip
Feature: Malli schema file generation from an OpenAPI spec
  As a Clojure backend developer
  I want openapi-codegen.core/generate to produce Malli schema files from components.schemas
  So that my application can validate and coerce data against a shared, contract-generated schema

  Scenario: Generating a schema with required and optional properties
    Given a bundled OpenAPI spec whose "LoginRequest" schema requires "username" and "password"
    When I call generate with the spec path and an output directory
    Then a file is written at "output-dir/openapi_codegen/schemas/login_request.clj"
    And the file's schema def is a Malli "[:map [:username :string] [:password :string]]" form

  Scenario Outline: OpenAPI types map to their corresponding Malli type
    Given an OpenAPI property of type "<openapi-type>"
    When I call openapi-type->malli on the property
    Then the result is the Malli type "<malli-type>"

    Examples:
      | openapi-type | malli-type |
      | string       | :string    |
      | integer      | :int       |
      | boolean      | :boolean   |
      | object       | :map       |
