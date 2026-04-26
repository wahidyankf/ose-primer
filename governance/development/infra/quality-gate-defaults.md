---
title: "Quality Gate Workflow Defaults Convention"
description: Canonical default values (mode: strict, max-iterations: 7) that all quality gate workflows in this repository must use, so new workflows are consistent and existing workflows are auditable
category: explanation
subcategory: development
tags:
  - quality-gates
  - workflows
  - ci
  - defaults
  - strict-mode
---

# Quality Gate Workflow Defaults Convention

Every quality gate workflow in this repository defaults to `strict` mode and `max-iterations: 7`. This document makes that default explicit, auditable, and enforceable for anyone authoring a new quality gate workflow.

## Principles Implemented/Respected

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Defaults are stated, not assumed. A reader of any quality gate workflow must be able to verify the default without guessing.

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: One canonical default pair reduces configuration drift. Authors of new quality gate workflows start from a known baseline rather than inventing their own defaults.

- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**: Enforcing defaults automatically prevents silent quality degradation. A quality gate that silently uses lower-than-intended mode or fewer iterations is a defect in the automation layer.

## Conventions Implemented/Respected

- **[Workflow Naming Convention](../../conventions/structure/workflow-naming.md)**: Workflow files follow naming rules; this convention supplements them with mandatory content requirements for quality gate workflows specifically.

- **[File Naming Convention](../../conventions/structure/file-naming.md)**: This document and all quality gate workflow files use lowercase kebab-case naming.

## Scope

### What This Convention Covers

- All iterative quality gate workflows using the checker → fixer → re-check loop pattern.
- The `mode` and `max-iterations` frontmatter fields in workflow YAML.
- Escalation behavior when a workflow approaches the iteration limit.

### What This Convention Does NOT Cover

- Non-iterative workflows (single-pass checks, one-shot scripts).
- Per-invocation overrides — users may always override `mode` and `max-iterations` at invocation time.

## Standards

### Standard 1: Default Mode Is strict

All quality gate workflows must declare `mode` in their YAML frontmatter with `default: strict`. The `strict` mode level fixes CRITICAL, HIGH, and MEDIUM findings while leaving LOW findings reported but unfixed.

The four mode levels:

| Mode   | Levels Fixed           | Levels Reported Only |
| ------ | ---------------------- | -------------------- |
| lax    | CRITICAL               | HIGH, MEDIUM, LOW    |
| normal | CRITICAL, HIGH         | MEDIUM, LOW          |
| strict | CRITICAL, HIGH, MEDIUM | LOW                  |
| ocd    | All levels             | None                 |

Choosing `strict` as the default balances thoroughness with practical convergence speed. The `ocd` level is available for use cases that require eliminating all findings, but it is not the default because LOW findings often involve subjective style choices that may never fully converge.

### Standard 2: Default max-iterations Is 7

All quality gate workflows must declare `max-iterations` in their YAML frontmatter with `default: 7`. Seven iterations is sufficient for convergence in practice. A quality gate that has not converged after seven cycles indicates a systemic problem that requires human investigation — not more automated iterations.

### Standard 3: Escalation Warning at Iteration 5

If a quality gate workflow reaches iteration 5 without achieving zero threshold-level findings, it must log an escalation warning before starting iteration 6. This early signal prevents silent runaway loops and gives the operator an opportunity to intervene before the workflow exhausts its iteration budget.

The escalation warning must state:

- The current iteration number.
- The number of remaining iterations.
- The count of threshold-level findings still open.

### Standard 4: Consecutive Zero Requirement

Success requires zero threshold-level findings on two consecutive independent validations. A single zero-finding check is not sufficient — it must be confirmed by an independent re-run. This eliminates false positives caused by transient state, file-system caching, or non-deterministic checkers.

### Standard 5: New Quality Gate Workflows Must Conform

When authoring a new quality gate workflow, the `mode` and `max-iterations` frontmatter fields must use the canonical defaults from this convention. Deviation from these defaults requires explicit written justification in the workflow's purpose section explaining why the canonical defaults do not apply.

## Conforming Examples

Both existing quality gate workflows conform to these defaults. The frontmatter pattern they share:

```yaml
---
name: <workflow-name>
# ...other fields...
inputs:
  - name: mode
    type: enum
    values: [lax, normal, strict, ocd]
    description: "Quality threshold (lax: CRITICAL only, normal: CRITICAL/HIGH, strict: +MEDIUM, ocd: all levels)"
    required: false
    default: strict
  - name: max-iterations
    type: number
    description: Maximum check-fix cycles to prevent infinite loops
    required: false
    default: 7
```

The `repo-rules-quality-gate` workflow and the `plan-quality-gate` workflow both declare this pattern verbatim.

## Related Documentation

- [Repo Rules Quality Gate Workflow](../../workflows/repo/repo-rules-quality-gate.md) — Repository consistency validation workflow using `strict` and `max-iterations: 7`.
- [Plan Quality Gate Workflow](../../workflows/plan/plan-quality-gate.md) — Plan completeness validation workflow using the same canonical defaults.
- [Criticality Levels Convention](../quality/criticality-levels.md) — The classification system that produces CRITICAL/HIGH/MEDIUM/LOW findings consumed by quality gate workflows.
- [Fixer Confidence Levels Convention](../quality/fixer-confidence-levels.md) — The confidence system that governs which findings fixer agents apply automatically.
