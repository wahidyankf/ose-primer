# Fix Mermaid Violations

**Status**: In Progress
**Scope**: `ose-primer` — `docs/` and `governance/` only
**Branch strategy**: Trunk-based development (direct commits to `main`)

## Problem

`rhino-cli docs validate-mermaid` reports **107 files** failing validation with
**247 total violations** across three error types. All violations were found on
2026-04-25 audit. The pre-push hook enforces this check — violations block every
push from any contributor.

## Violation Summary

| Error type         | Count   | Severity  |
| ------------------ | ------- | --------- |
| `width_exceeded`   | 179     | ✗ Error   |
| `label_too_long`   | 56      | ✗ Error   |
| `complex_diagram`  | 12      | ⚠ Warning |
| **Files affected** | **107** | —         |

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
| 3     | `programming-languages/golang/`       | 10    |
| 4     | `platform-web/tools/jvm-spring-boot/` | 9     |
| 5     | `platform-web/tools/elixir-phoenix/`  | 8     |
| 6     | `platform-web/tools/fe-react/`        | 7     |
| 7     | `platform-web/tools/fe-nextjs/`       | 6     |
| 8     | `programming-languages/elixir/`       | 5     |
| 9     | `architecture/c4-architecture-model/` | 5     |
| 10    | Remaining files                       | 14    |

## Definition of Done

`go run ./apps/rhino-cli/main.go docs validate-mermaid` exits 0 with zero `✗` error
lines. `⚠` warning lines (complex_diagram) are tolerated in this plan.
