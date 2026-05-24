@agents-bindings
Feature: Harness Binding Emit and Validate

  As a repository maintainer
  I want to generate vendor binding files from the canonical AGENTS.md and assert they never drift
  So that harnesses that do not read AGENTS.md natively (e.g. Amazon Q) stay bridged without manual upkeep

  # emit-bindings generates the Tier-2 bridge files (a rules pointer + a default agent JSON)
  # that reference AGENTS.md rather than copying its body.

  Scenario: Emitting bindings writes the expected files
    Given a repository with a canonical AGENTS.md at the root
    When the developer runs agents emit-bindings
    Then the command exits successfully
    And the Amazon Q rules pointer and default agent JSON are written
    And each generated file references AGENTS.md without duplicating its body

  Scenario: A dry-run previews the binding files without writing them
    Given a repository with a canonical AGENTS.md at the root
    When the developer runs agents emit-bindings with the --dry-run flag
    Then the command exits successfully
    And the output lists the files that would be written
    And no binding files are created on disk

  # validate-bindings is the deterministic, agent-free pre-push guard. It re-derives each
  # generated binding file from AGENTS.md in memory and asserts byte-equality, and asserts
  # every binding directory on disk has a row in the platform-bindings catalog. No network, no agent.

  Scenario: Validation passes when every generated binding matches and the catalog covers each binding directory
    Given a repository whose committed binding files match a fresh regenerate
    And the platform-bindings catalog documents every binding directory present on disk
    When the developer runs agents validate-bindings
    Then the command exits successfully
    And the output reports zero binding drift and zero catalog gaps

  Scenario: Validation fails when a committed binding file drifts from its regenerate
    Given a repository where a committed binding file no longer matches a regenerate from AGENTS.md
    When the developer runs agents validate-bindings
    Then the command exits with a failure code
    And the output identifies the drifted binding file

  Scenario: Validation fails when a binding directory has no catalog row
    Given a repository with a binding directory that the platform-bindings catalog does not document
    When the developer runs agents validate-bindings
    Then the command exits with a failure code
    And the output identifies the binding directory missing from the catalog
