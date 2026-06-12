# PRD — Mermaid State Diagram Validation (ose-primer)

> Product Requirements Document. WHAT gets built.

## Product Overview

Extend `rhino-cli docs validate-mermaid` so that Mermaid **state diagrams** (`stateDiagram-v2` and
legacy `stateDiagram`) are parsed and subjected to the same width and label rules already enforced
on flowcharts. Deliver this on a fresh, kind-agnostic module design in which both the flowchart and
state parsers emit a single shared `ParsedDiagram` consumed by a shared validation core. The
behavior must be byte-identical to the ose-public reference, locked by a shared golden test corpus.

## Personas

Solo-maintainer repo; the maintainer wears multiple hats, and agents consume the plan:

- **Template maintainer (human)** — owns the validator and the governance surface; wants state
  diagrams in the template held to the same readability bar as flowcharts.
- **`swe-rust-dev` (agent)** — implements the module re-shape and state parser via TDD.
- **`repo-rules-maker` (agent)** — propagates the rule into governance and re-syncs bindings.
- **`plan-execution-checker` (agent)** — verifies the finished plan to zero findings.

## User Stories

- As a template maintainer, I want state diagrams width-checked, so that adopters who copy the
  scaffolding inherit a validator that catches over-wide state diagrams the same way it catches
  over-wide flowcharts.
- As a Rust toolsmith, I want one kind-agnostic validation core, so that adding state support (and
  later other diagram kinds) is a front-end addition rather than a fork of the rule logic.
- As a governance maintainer, I want the documented Mermaid rules to match the validator, so that
  `repo-governance/conventions/formatting/diagrams.md` does not promise behavior the code lacks.
- As a parity-run operator, I want a shared golden corpus, so that the same fixtures produce
  identical violations in ose-public, ose-primer, and ose-infra.

## Acceptance Criteria (Gherkin)

> Step-keyword cardinality: each scenario uses exactly one primary `Given`, one `When`, one
> `Then`; extras chain with `And`/`But`.

```gherkin
Feature: State diagram width validation

  Background:
    Given the rhino-cli validate-mermaid command with default options (max_width 4, max_label_len 30)

  Scenario: Over-wide LR state chain is flagged
    Given a stateDiagram-v2 with direction LR and 11 sequential states on one rank
    When validate-mermaid parses the block
    Then a width_exceeded violation is reported
    And the violation actual_width is greater than 4

  Scenario: Start and end pseudostates count as nodes
    Given a stateDiagram-v2 whose single rank holds four named states plus a [*] pseudostate
    When validate-mermaid computes rank width
    Then the [*] pseudostate is counted toward the rank
    And a width_exceeded violation is reported because the rank width is 5

  Scenario: Stereotype states count as nodes
    Given a stateDiagram-v2 rank of four named states plus a state marked <<choice>>
    When validate-mermaid computes rank width
    Then the <<choice>> state is counted toward the rank
    And a width_exceeded violation is reported because the rank width is 5
```

```gherkin
Feature: State diagram label validation

  Background:
    Given the rhino-cli validate-mermaid command with default options (max_width 4, max_label_len 30)

  Scenario: Over-long state display label is flagged
    Given a stateDiagram-v2 with a state whose display label exceeds 30 characters
    When validate-mermaid checks labels
    Then a label_too_long violation is reported for that state
    And the violation label_len is greater than 30

  Scenario: Over-long transition-edge label is flagged
    Given a stateDiagram-v2 with a transition "A --> B : <label longer than 30 characters>"
    When validate-mermaid checks labels
    Then a label_too_long violation is reported for the transition label
    But the source and target states are not themselves flagged
```

```gherkin
Feature: State diagram structural mapping

  Background:
    Given the rhino-cli validate-mermaid command with default options (max_width 4, max_label_len 30)

  Scenario: Composite state is treated as a subgraph
    Given a stateDiagram-v2 containing a composite "state Outer { ... }" with seven inner states
    When validate-mermaid recurses into the composite
    Then the composite is treated like a flowchart subgraph
    And a subgraph_density warning is reported for the inner region

  Scenario: Notes, comments, and concurrency separators are not misparsed
    Given a stateDiagram-v2 containing a "note right of X ... end note", a "%%" comment, and a "--" concurrency separator
    When validate-mermaid parses the block
    Then the note text is exempt from the label rule
    And neither the comment nor the "--" separator is counted as a node or transition
```

```gherkin
Feature: Flowchart behavior is preserved

  Scenario: Existing flowchart validation is unchanged
    Given the flowchart test fixtures that passed before this plan
    When validate-mermaid runs after the module re-shape and state support land
    Then every flowchart violation and warning is byte-identical to the pre-plan output
    And no flowchart test is removed or weakened
```

```gherkin
Feature: Cross-repo parity

  Scenario: Shared golden corpus produces identical violations
    Given the shared golden corpus committed identically to ose-public, ose-primer, and ose-infra
    When validate-mermaid runs over the corpus in ose-primer
    Then the violation JSON matches the committed expected.json fixtures
    And the output is identical to the ose-public reference output
```

## Product Scope

In-scope features:

- Parse `stateDiagram-v2` and `stateDiagram` (v1) headers; dispatch to a state front-end parser.
- Map state-diagram grammar to `ParsedDiagram`: `[*]` and stereotype (`<<choice>>` / `<<fork>>` /
  `<<join>>`) states become `Node`s; composite `state X { }` becomes a recursed `Subgraph`;
  `A --> B : lbl` becomes an `Edge` whose label feeds the transition-label check.
- Apply width rule (`≤4 nodes/rank`), label rule (`≤30` for state display labels AND transition
  labels), and the subgraph-density warning inside composites.
- Honor `direction TB|BT|LR|RL` (no `TD`); `LR`/`RL` swap width/depth axes.
- Skip notes (`note left of X: …` and `note … end note`), comments (`%%…`, `#…`), and `--`
  concurrency separators.
- Shared golden test corpus committed identically across the three repos.

Out-of-scope features:

- Other diagram block types (`sequenceDiagram`, `classDiagram`, `erDiagram`, `gitGraph`).
- Threshold changes, gate-wiring changes, exclusion-list changes.

## Product Risks

- **Grammar edge cases.** The state grammar has several state-declaration forms (bare id,
  `id : desc`, `state "desc" as id`, composite, stereotype). Mitigation: the golden corpus exercises
  each form; the parser is built TDD-first against those fixtures.
- **Transition-label vs concurrency-separator ambiguity.** `-->` must be matched before the `--`
  concurrency separator. Mitigation: explicit ordering in the parser, covered by a corpus fixture
  combining both.
- **Parity divergence.** Mitigation: the shared corpus' `expected.json` is the hard lock; the
  Phase B gate fails on any mismatch with the reference fixtures.
