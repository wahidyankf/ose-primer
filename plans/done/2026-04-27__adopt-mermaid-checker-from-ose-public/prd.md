---
title: PRD — Adopt ose-public Mermaid Checker Enhancements
---

# Product Requirements

## Product overview

This plan delivers an upgraded `validate-mermaid` CLI surface inside
`rhino-cli`. Concretely it adds a Rule 4 subgraph-density check (new
`--max-subgraph-nodes` flag), corrects the direction-mapped reporting
fields on `complex_diagram` warnings, extends the Nx target input list
so pre-push validation reaches `docs/` and `plans/`, and remediates
every flagged diagram in the repo. The result is a Go CLI feature set
that closes the readability-correctness gap between ose-primer and its
downstream consumer ose-public, while keeping the change non-breaking
for existing callers.

## Personas

- **rhino-cli maintainer** — ports, tests, and ships the Go source
  changes; owns coverage and spec-coverage gates.
- **Markdown author / repo contributor** — pushes commits with
  Mermaid diagrams in `docs/` or `plans/`; experiences the pre-push
  hook firing the upgraded validator and receives correct
  direction-mapped warning output.
- **CI consumer** — any workflow or bot reading the `validate:mermaid`
  Nx target output; depends on deterministic exit-code semantics
  being unchanged (exit 0 for warnings, non-zero for violations).
- **plan-executor agent** — runs the delivery checklist step by step;
  needs clear per-phase acceptance commands to confirm progress.
- **Template cloner** — any downstream repo bootstrapped from
  ose-primer; inherits the upgraded checker baseline via the normal
  clone path.

## User stories

- As a template maintainer, I want the mermaid checker to warn on
  subgraphs with more than 6 direct children, so that dense cluster
  diagrams are flagged before they land on `origin/main`.
- As a markdown author, I want the `complex_diagram` warning to report
  direction-mapped `ActualWidth` and `ActualDepth` values, so that the
  numbers match what I see in the rendered horizontal and vertical axes.
- As a contributor pushing to main, I want the pre-push hook to
  validate Mermaid blocks in `docs/` and `plans/`, so that readability
  regressions in those trees are caught before merge.
- As a template cloner, I want the checker shipped in ose-primer to
  have feature parity with ose-public, so that cloning the template
  gives me the correct baseline without a follow-up sync plan.
- As a plan-executor agent, I want each delivery phase to end with a
  runnable `nx run rhino-cli:validate:mermaid` assertion, so that
  progress is verifiable without manual inspection.

## Functional requirements

- **FR-1 — Subgraph-density rule (Rule 4).** The checker must parse
  `subgraph ... end` blocks (including nested), attribute every newly
  introduced node ID on each line to the innermost open subgraph as a
  direct child, and emit a `subgraph_density` warning when the count
  of direct children for any subgraph exceeds `MaxSubgraphNodes`.
  Warning is non-blocking (exit 0). Threshold default 6, exposed via
  `--max-subgraph-nodes N`. Setting `N <= 0` disables the rule.
- **FR-2 — Direction-mapped warning fields.** When emitting a
  `complex_diagram` warning, the checker reports `ActualWidth` as the
  direction-mapped horizontal value and `ActualDepth` as the
  direction-mapped vertical value. For `graph LR` / `RL`,
  `horizontal = depth` and `vertical = span`. For `graph TD` / `TB` /
  `BT`, `horizontal = span` and `vertical = depth`. Output formats
  (text, JSON, markdown) all reflect the mapped values.
- **FR-3 — Subgraph-aware parser.** The parser exposes
  `ParsedDiagram.Subgraphs []Subgraph` with `{ID, Label, NodeIDs,
StartLine}` per block. The parser tolerates:
  unlabelled subgraphs, ID-only subgraphs, ID + bracketed label,
  quoted-label-only, and ID with bracketed quoted label. Unclosed
  subgraphs are popped at parse end so the warning still fires.
- **FR-4 — Pre-push coverage extension.** The
  `rhino-cli:validate:mermaid` Nx target scans every `*.md` file
  under the four canonical roots (`docs/`, `governance/`, `.claude/`,
  `plans/`) plus repo-root `*.md` files. The Husky pre-push hook
  continues to fire the target on any `*.md` change in the push
  range.
- **FR-5 — Repository remediation.** Every Mermaid diagram in
  ose-primer that the upgraded checker surfaces with a `Violation`
  or `Warning` is fixed in the same plan run. Acceptable
  remediations: shrink labels, restructure subgraphs into multiple
  smaller subgraphs, or set per-invocation flag overrides via the
  Nx target only when redesigning would lose pedagogical value
  (must justify in commit body). Existing pass-through diagrams
  remain pass-through.
- **FR-6 — Test parity.** Test files
  (`{extractor,parser,graph,validator,reporter}_test.go` plus
  `cmd/docs_validate_mermaid*_test.go`) are ported alongside the
  source so coverage stays at or above 90%.
- **FR-7 — Backward-compat for existing flags.** The three existing
  flags (`--max-label-len`, `--max-width`, `--max-depth`),
  the four input-selection flags (`--staged-only`, `--changed-only`,
  positional args, default scan), and exit-code semantics
  (violations → non-zero, warnings → zero) are unchanged.

## Non-functional requirements

- **NFR-1 — Coverage.** Go coverage stays ≥ 90% for the rhino-cli
  package and the `internal/mermaid` package in particular.
- **NFR-2 — Caching.** Nx target `validate:mermaid` remains cacheable
  with explicit `inputs` listing every scanned tree.
- **NFR-3 — Performance.** Full-repo scan completes in under 5
  seconds locally on a 153-file corpus (current footprint).
- **NFR-4 — Determinism.** Reporter output ordering is deterministic
  so audit reports diff cleanly across runs.
- **NFR-5 — License compatibility.** The ported code originates in
  ose-public (MIT) into ose-primer (MIT) — license-compatible by
  construction; no `LICENSING-NOTICE.md` change required.

## Acceptance criteria — Gherkin

```gherkin
Feature: Subgraph-density rule

  Scenario: Diagram with one over-dense subgraph emits a warning
    Given a markdown file containing a flowchart with a subgraph
      that has 7 direct child nodes
    And the default MaxSubgraphNodes threshold of 6
    When validate-mermaid scans the file
    Then the result has 1 warning of kind "subgraph_density"
    And the warning's SubgraphNodeCount is 7
    And the warning's MaxSubgraphNodes is 6
    And the validator exit code is 0

  Scenario: Diagram with subgraph at threshold emits no warning
    Given a markdown file containing a flowchart with a subgraph
      that has exactly 6 direct child nodes
    When validate-mermaid scans the file
    Then the result has 0 warnings of kind "subgraph_density"

  Scenario: Subgraph rule disabled via flag
    Given a markdown file containing a flowchart with a subgraph
      that has 20 direct child nodes
    When validate-mermaid scans the file with --max-subgraph-nodes 0
    Then the result has 0 warnings of kind "subgraph_density"

  Scenario: Nested subgraphs attribute children to the innermost
    Given a markdown file with a flowchart containing
      a subgraph "Outer" with subgraph "Inner" inside
    And node A is declared inside Inner
    When validate-mermaid scans the file
    Then Inner's NodeIDs contains A
    And Outer's NodeIDs does not contain A

Feature: Direction-mapped complex_diagram warning fields

  Scenario: LR diagram exceeding both axes reports depth as width
    Given a flowchart with direction LR, span 2, depth 8
    And MaxWidth 4 and MaxDepth 4 are configured
    When validate-mermaid scans the file
    Then the result has 1 warning of kind "complex_diagram"
    And the warning's ActualWidth is 8
    And the warning's ActualDepth is 2

  Scenario: TB diagram exceeding both axes reports span as width
    Given a flowchart with direction TB, span 8, depth 2
    And MaxWidth 4 and MaxDepth 1 are configured
    When validate-mermaid scans the file
    Then the result has 1 warning of kind "complex_diagram"
    And the warning's ActualWidth is 8
    And the warning's ActualDepth is 2

Feature: Pre-push coverage extension

  Scenario: docs/ markdown change triggers mermaid validation
    Given the pre-push hook is installed
    And docs/explanation/.../README.md contains a mermaid block
    When the user pushes a commit modifying that file
    Then rhino-cli:validate:mermaid runs against the changed file
    And exits 0 when no violations exist

  Scenario: plans/ markdown change triggers mermaid validation
    Given the pre-push hook is installed
    And plans/in-progress/.../README.md contains a mermaid block
    When the user pushes a commit modifying that file
    Then rhino-cli:validate:mermaid runs against the changed file
    And exits 0 when no violations exist

Feature: Repository remediation

  Scenario: Full-repo scan exits clean after remediation
    Given the upgraded checker is installed
    And every flagged diagram has been fixed or scoped via flag
    When validate-mermaid is run with no arguments
    Then the exit code is 0
    And the violation count is 0
```

## Product risks

- **False-positive density warnings on intentionally dense diagrams.**
  Authors writing architectural overview diagrams with many nodes in a
  single `subgraph` may find the Rule 4 warning noisy. Mitigation:
  `--max-subgraph-nodes 0` disables the rule entirely; this opt-out is
  exposed in the Nx target configuration, not the Husky hook.
- **Cache thrash from broadened Nx target inputs.** Adding `docs/` and
  `plans/` to the `validate:mermaid` Nx input list invalidates the
  target cache on every markdown edit in those trees, not just mermaid
  edits. Mitigation: the Go binary scan is fast (under 5 seconds on
  the 153-file corpus); the tradeoff is acceptable.
- **Flag confusion between `--max-subgraph-nodes` and existing flags.**
  Operators unfamiliar with the new flag may confuse it with
  `--max-width` / `--max-depth`. The `Long` description in
  `cmd/docs_validate_mermaid.go` will be updated to distinguish the
  three axes (label length, graph dimensions, subgraph density).

## Out-of-scope clarifications

- **No new rules** beyond Rule 4. No node-count limit, no edge-count
  limit, no density metric, no diameter check.
- **No threshold changes** to existing Rules 1–3.
- **No reporter format changes** beyond the new
  `subgraph_density` rendering and the direction-mapped values for
  `complex_diagram`.
- **No removal** of the existing `ActualWidth/ActualDepth` field
  names — only the meaning shifts (raw → direction-mapped).
- **No pre-commit wiring** — the rule lives at pre-push only.

## Manual verification

This plan touches no UI and no HTTP endpoint, so neither Playwright
MCP nor curl-based assertion applies. The CLI itself is the manual
verification surface: run `nx run rhino-cli:validate:mermaid` after
each phase boundary and assert the violation/warning counts shift as
expected.
