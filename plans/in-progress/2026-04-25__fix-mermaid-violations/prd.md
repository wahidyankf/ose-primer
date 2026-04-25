# PRD — Fix Mermaid Violations

## Requirements

### R1 — Zero `width_exceeded` errors

All mermaid diagrams must have a BFS-level span ≤ 3. The span is the maximum
number of nodes at any single BFS depth level across all connected components.

### R2 — Zero `label_too_long` errors

All mermaid node labels must be ≤ 30 raw characters per line (measured after
splitting on `<br/>`, before HTML-entity decoding). The constraint applies to each
individual line of a multi-line label separately.

### R3 — Semantic preservation

Every fixed diagram must convey the same information as the original. Content
removed from a label must appear in surrounding prose. Relationships between
nodes must not change.

### R4 — No regressions

Files not listed in the audit must continue to pass. Each batch commit must leave
the validator result for that batch's files at zero errors before moving to the
next batch.

## Acceptance Criteria

```gherkin
Feature: Mermaid diagram compliance in ose-primer

  Background:
    Given I am at the root of the ose-primer repository
    And rhino-cli is available via `go run ./apps/rhino-cli/main.go`

  Scenario: No error files remain after all batches
    When I run `go run ./apps/rhino-cli/main.go docs validate-mermaid`
    Then no output line starts with "✗"

  Scenario: No width_exceeded errors remain
    When I run `go run ./apps/rhino-cli/main.go docs validate-mermaid`
    Then no output line contains "[width_exceeded]"

  Scenario: No label_too_long errors remain
    When I run `go run ./apps/rhino-cli/main.go docs validate-mermaid`
    Then no output line contains "[label_too_long]"

  Scenario: Diagram relationships are preserved
    Given a diagram that was refactored to fix a width_exceeded error
    When I read the surrounding prose and the fixed diagram together
    Then all node relationships present in the original diagram are still represented

  Scenario: Batch validation passes after each batch
    Given I have completed fixing files in one batch
    When I run `go run ./apps/rhino-cli/main.go docs validate-mermaid`
    Then none of the files in the completed batch appear in the error output
```
