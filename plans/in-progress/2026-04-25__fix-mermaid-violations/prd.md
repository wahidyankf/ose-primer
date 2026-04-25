# PRD — Fix Mermaid Violations

## Product Overview

This plan delivers a zero-violation state for all mermaid diagrams in `docs/` of the
`ose-primer` repository. The deliverable is a set of edited markdown files — no code,
no configuration, no tooling changes — where every mermaid block passes the
`rhino-cli docs validate-mermaid` checker. Errors targeted: `width_exceeded` (BFS
span > 3) and `label_too_long` (any label line > 30 raw characters).

## Personas

| Persona            | Description                                                                        |
| ------------------ | ---------------------------------------------------------------------------------- |
| As a contributor   | A developer pushing commits to `ose-primer` who wants clean validator output       |
| As a docs reader   | A learner reading `docs/` in GitHub preview or VS Code who needs readable diagrams |
| As a plan executor | The agent or human running the delivery checklist batch-by-batch                   |

## User Stories

**As a contributor**, I want the mermaid validator to report zero errors on `docs/` files
so that any future expansion of the hook scope does not surface a backlog of pre-existing
violations.

**As a docs reader**, I want diagrams that fit within their containers without horizontal
scrollbars so that I can read them without zooming or scrolling horizontally.

**As a plan executor**, I want each batch to be independently verifiable before committing
so that I can confirm my fixes are correct without running the full repo validator every
time.

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

## Product Scope

**In scope**:

- Fixing `width_exceeded` violations in all 107 affected `docs/` files
- Fixing `label_too_long` violations in all 107 affected `docs/` files
- Preserving diagram semantics (node relationships, information content)

**Out of scope**:

- `complex_diagram` warnings — these are warnings, not errors; deferred to a future pass
- Changes to `governance/` or `.claude/` files — audited clean, no violations
- Changes to rhino-cli source code or the pre-push hook configuration
- Adding new diagrams or expanding documentation content beyond what is needed for fixes

## Product Risks

| Risk                                                                 | Impact | Note                                                                                                   |
| -------------------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------------------ |
| Diagram restructuring loses semantic meaning                         | High   | Re-read surrounding prose alongside each fix; preserve all node relationships                          |
| Batch validation grep misses a fixed file (silent false pass)        | Medium | Batch 10 grep pattern must include `software-engineering/development` — see delivery                   |
| Executor applies wrong fix strategy (visual change, not topological) | Medium | Strategy selection guide in tech-docs.md must be followed; changing direction alone does not fix width |

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
