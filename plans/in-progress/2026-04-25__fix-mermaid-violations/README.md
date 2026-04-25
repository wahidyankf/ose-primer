# Fix Mermaid Violations

**Status**: In Progress
**Scope**: `ose-primer` — `docs/` only (governance/ audited clean)
**Branch strategy**: Trunk-based development (direct commits to `main`)

## Problem

`rhino-cli docs validate-mermaid` reports **107 files** failing validation with
**247 total violations** across three error types. All violations were found on
2026-04-25 audit. The pre-push hook currently targets only `governance/` and `.claude/`,
so these `docs/` violations do not block pushes today — but the diagrams render poorly
on GitHub and in VS Code preview, and fixing them now establishes a clean baseline
before any future expansion of the hook scope.

## Violation Summary

| Error type         | Count   | Severity  |
| ------------------ | ------- | --------- |
| `width_exceeded`   | 179     | ✗ Error   |
| `label_too_long`   | 56      | ✗ Error   |
| `complex_diagram`  | 12      | ⚠ Warning |
| **Files affected** | **107** | —         |

## Approach Summary

Work is divided into 10 batches by documentation area. Each batch is fixed, validated
with a targeted grep against the mermaid validator output, and committed independently.
Four fix strategies cover all violation types:

- **Subgraph grouping** — collapses wide fan-out nodes into named subgraphs (Strategy 1)
- **Sequential chaining** — replaces parallel fan-out with linear chains for sequence
  diagrams (Strategy 3)
- **Diagram splitting** — splits one overloaded diagram into 2–3 focused diagrams when
  nodes have no natural grouping (Strategy 2)
- **Label shortening** — replaces HTML entities with literals and rephrases long labels;
  moved detail goes into surrounding prose (Strategy 4)

The rhino-cli validator (`go run ./apps/rhino-cli/main.go docs validate-mermaid`) serves
as the gate after each batch. Zero `✗` error lines = batch complete.

## Documents

- [brd.md](./brd.md) — Business rationale and stakeholder impact
- [prd.md](./prd.md) — Product requirements and Gherkin acceptance criteria
- [tech-docs.md](./tech-docs.md) — Fix strategies per error type, batch breakdown
- [delivery.md](./delivery.md) — Step-by-step delivery checklist

## Batch Structure

Work is split into 10 independent batches (one per doc area) plus final validation.
Each batch is self-contained and verifiable in isolation.

**Out of scope**: `complex_diagram` warnings (6 files) — deferred to a future pass.

| Batch | Area                                  | Files |
| ----- | ------------------------------------- | ----- |
| 1     | `programming-languages/typescript/`   | 18    |
| 2     | `programming-languages/python/`       | 15    |
| 3     | `programming-languages/golang/`       | 11    |
| 4     | `platform-web/tools/jvm-spring-boot/` | 10    |
| 5     | `platform-web/tools/elixir-phoenix/`  | 8     |
| 6     | `platform-web/tools/fe-react/`        | 8     |
| 7     | `platform-web/tools/fe-nextjs/`       | 6     |
| 8     | `programming-languages/elixir/`       | 6     |
| 9     | `architecture/c4-architecture-model/` | 5     |
| 10    | Remaining files                       | 14    |

## Definition of Done

`go run ./apps/rhino-cli/main.go docs validate-mermaid` exits 0 with zero `✗` error
lines. `⚠` warning lines (complex_diagram) are tolerated in this plan.
