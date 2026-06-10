@env-validate
Feature: Environment Variable Drift Guard

  As a developer maintaining multiple polyglot apps
  I want rhino-cli env validate to compare each app's declared env vars against its source reads
  So that renamed or missing variables are caught before they reach production

  @env-validate
  Scenario: Declared-but-unread key causes non-zero exit naming the key
    Given a fixture app whose .env.example declares FIXTURE_JWT_SECRET but whose source never reads it
    When the developer runs rhino-cli env validate
    Then the command exits with a failure code
    And the output names FIXTURE_JWT_SECRET as a declared-but-unread key

  @env-validate
  Scenario: Read-but-undeclared key causes non-zero exit naming the key
    Given a fixture app whose source reads FIXTURE_JWT_SECRET but whose .env.example does not declare it
    When the developer runs rhino-cli env validate
    Then the command exits with a failure code
    And the output names FIXTURE_JWT_SECRET as a read-but-undeclared key

  @env-validate
  Scenario: Matching declared and read keys exit successfully
    Given a fixture app whose .env.example declares FIXTURE_JWT_SECRET and whose source reads it
    When the developer runs rhino-cli env validate
    Then the command exits successfully
    And the output reports validation passed

  @env-validate
  Scenario: Allowlisted keys are ignored during validation
    Given a fixture app that reads ENABLE_TEST_API and a framework-injected PORT variant but neither is declared in .env.example
    When the developer runs rhino-cli env validate
    Then the command exits successfully
    And the output reports validation passed
