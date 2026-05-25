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
- Plan validation (that's plan/)

## Workflows

- [Repository Rules Validation](./repo-rules-quality-gate.md) - Validate repository consistency across all layers (principles, conventions, development, agents) and apply fixes iteratively until ZERO findings. Supports four strictness modes (lax, normal, strict, ocd)
- [Harness Compatibility Quality Gate](./repo-harness-compatibility-quality-gate.md) - The single harness-compat gate. **Phase 0** runs 5 deterministic cross-vendor parity invariants (governance vendor-neutrality, AGENTS+CLAUDE, binding sync no-op over `.opencode/` + `.amazonq/`, agent count parity, color + tier maps); **Phase 1** detects external drift between each supported harness's current upstream conventions and the platform-bindings catalog plus committed binding files (agent-backed, web-research-backed). Complements the deterministic `rhino-cli agents validate-bindings` / `validate:cross-vendor-parity` pre-push guard. Iterative checker→fixer until two consecutive zero-finding runs.

## Related Documentation

- [Workflows Index](../README.md) - All orchestrated workflows
- [Repository Architecture](../../repository-governance-architecture.md) - Six-layer governance model these workflows enforce
- [Maker-Checker-Fixer Pattern](../../development/pattern/maker-checker-fixer.md) - Core workflow pattern
- [Core Principles](../../principles/README.md) - Layer 1 governance

---
