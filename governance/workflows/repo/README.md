# Repository Workflows

Orchestrated workflows for validating repository-level operations across principles, conventions, development practices, and agents.

## Purpose

These workflows define **WHEN and HOW to validate repository rules**, orchestrating repo-rules-checker and repo-rules-fixer agents to ensure consistency across all governance layers (principles, conventions, development practices, agents).

## Scope

**✅ Workflows Here:**

- Repository-wide consistency validation
- Cross-layer governance checking
- Agent standards enforcement
- Iterative check-fix-verify cycles

**❌ Not Included:**

- Content quality validation (that's docs/)
- Hugo content validation (that's crud-fs-ts-nextjs/)
- Plan validation (that's plan/)

## Workflows

- [Repository Rules Validation](./repo-rules-quality-gate.md) - Validate repository consistency across all layers (principles, conventions, development, agents) and apply fixes iteratively until ZERO findings. Supports four strictness modes (lax, normal, strict, ocd)
- [Cross-Vendor Parity Gate](./repo-cross-vendor-parity-quality-gate.md) - Validate that primary and secondary binding directories stay byte-for-byte equivalent (5 invariants: governance vendor-neutrality, AGENTS+CLAUDE, sync no-op, agent count parity, color + tier maps). Iterative checker→fixer until two consecutive zero-finding runs.

## Related Documentation

- [Workflows Index](../README.md) - All orchestrated workflows
- [Repository Architecture](../../repository-governance-architecture.md) - Six-layer governance model these workflows enforce
- [Maker-Checker-Fixer Pattern](../../development/pattern/maker-checker-fixer.md) - Core workflow pattern
- [Core Principles](../../principles/README.md) - Layer 1 governance

---
