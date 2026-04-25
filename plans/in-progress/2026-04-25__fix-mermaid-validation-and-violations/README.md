# Fix Mermaid Validation and Violations

**Status**: In Progress
**Scope**: `ose-primer` — `apps/rhino-cli/` (validator fix) + `docs/` (violation fixes)
**Branch strategy**: Trunk-based development (direct commits to `main`)

## Problem

Two related problems must be fixed in order:

**P1 — Validator is direction-blind (rhino-cli bug)**: The `width_exceeded` rule always
checks `span` (max nodes per rank) regardless of graph direction. For `graph LR`/`RL`
diagrams, the horizontal dimension that causes overflow is **depth** (number of rank
columns), not span (row height). The validator flags LR diagrams on the wrong axis,
producing false positives and missing true horizontal overflows in deeply chained LR
graphs.

**P2 — 107 docs files failing (audit, 2026-04-25)**: Even with the direction bug, 247
violations exist across `docs/`. The pre-push hook targets only `governance/` and
`.claude/`, so these don't block pushes — but diagrams render poorly on GitHub and in
VS Code preview.

**Dependency**: P1 must be fixed first. The direction-aware validator will produce a
different error list — the Phase 1 file inventory is provisional until Phase 0 re-audit.

## Violation Summary (pre-fix baseline, 2026-04-25)

| Error type         | Count   | Severity  |
| ------------------ | ------- | --------- |
| `width_exceeded`   | 179     | ✗ Error   |
| `label_too_long`   | 56      | ✗ Error   |
| `complex_diagram`  | 12      | ⚠ Warning |
| **Files affected** | **107** | —         |

_Update this table from the Phase 0 re-audit output._

## Approach Summary

**Phase 0 — Fix the validator**: Update `apps/rhino-cli/internal/mermaid/validator.go`
to use `diagram.Direction` when selecting the horizontal dimension. For LR/RL: check
`depth > MaxWidth`. For TD/TB/BT: check `span > MaxWidth` (current behaviour). Update
tests. Re-audit to get the accurate Phase 1 file list.

**Phase 1 — Fix the docs**: 10 batches by doc area. Fix strategies:

- **Sequential chaining** — chain nodes that are genuinely sequential (Strategy 1)
- **Diagram splitting** — split overloaded diagrams into 2–3 focused diagrams (Strategy 2)
- **Label shortening** — replace HTML entities with literals; abbreviate long text (Strategy 4)

The direction-aware validator gates each batch. Zero `✗` lines = batch complete.

## Documents

- [brd.md](./brd.md) — Business rationale and stakeholder impact
- [prd.md](./prd.md) — Product requirements and Gherkin acceptance criteria
- [tech-docs.md](./tech-docs.md) — Fix strategies per error type, batch breakdown
- [delivery.md](./delivery.md) — Step-by-step delivery checklist

## Work Structure

**Out of scope**: `complex_diagram` warnings — deferred to a future pass.

| Phase | Batch | Area                                    | Files (provisional) |
| ----- | ----- | --------------------------------------- | ------------------- |
| 0     | —     | `apps/rhino-cli/` — direction-aware fix | 2 Go files + tests  |
| 1     | 1     | `programming-languages/typescript/`     | ~18                 |
| 1     | 2     | `programming-languages/python/`         | ~15                 |
| 1     | 3     | `programming-languages/golang/`         | ~11                 |
| 1     | 4     | `platform-web/tools/jvm-spring-boot/`   | ~10                 |
| 1     | 5     | `platform-web/tools/elixir-phoenix/`    | ~8                  |
| 1     | 6     | `platform-web/tools/fe-react/`          | ~8                  |
| 1     | 7     | `platform-web/tools/fe-nextjs/`         | ~6                  |
| 1     | 8     | `programming-languages/elixir/`         | ~6                  |
| 1     | 9     | `architecture/c4-architecture-model/`   | ~5                  |
| 1     | 10    | Remaining files                         | ~14                 |

Phase 1 file counts are provisional — update from Phase 0 re-audit before executing.

## Definition of Done

- `nx run rhino-cli:test:quick` passes with direction-aware tests (Phase 0).
- `go run ./apps/rhino-cli/main.go docs validate-mermaid` exits 0 with zero `✗` lines
  after all Phase 1 batches. `⚠` warnings tolerated.
