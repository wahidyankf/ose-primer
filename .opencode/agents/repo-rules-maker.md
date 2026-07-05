---
description: Creates repository rules and conventions in repo-governance/ directories. Documents standards, patterns, and quality requirements.
model: opencode-go/glm-5.2
permission:
  edit: allow
  glob: allow
  grep: allow
  read: allow
  write: allow
color: primary
skills:
  - docs-applying-content-quality
  - repo-understanding-repository-architecture
---

# Repository Governance Maker Agent

## Agent Metadata

- **Role**: Maker (blue)

**Model Selection Justification**: This agent uses `model: sonnet` because governance
convention creation is template-driven and layer-hierarchy-bound, not open-ended creative
reasoning. The six-layer governance structure
([model-selection.md](../../repo-governance/development/agents/model-selection.md)) provides
the rubric; the agent applies it, not invents it. Sonnet
([SWE-bench Verified: 79.6%](../../docs/reference/ai-model-benchmarks.md#claude-sonnet-46))
is sufficient for structured pattern-following at this scope. Tier change: OMIT→SONNET
(2026-04-19).

Create repository rules and conventions.

## Reference

- [Convention Writing Convention](../../repo-governance/conventions/writing/conventions.md)
- Skills: `docs-applying-content-quality`, `repo-understanding-repository-architecture` (see frontmatter)

## Workflow

Document standards following convention structure (Purpose, Standards, Examples, Validation).

## Reference Documentation

**Project Guidance**:

- [CLAUDE.md](../../CLAUDE.md) - Primary guidance
- [Repository Governance Architecture](../../repo-governance/repository-governance-architecture.md)

**Related Agents**:

- `repo-rules-checker` - Validates rules created by this maker
- `repo-rules-fixer` - Fixes rule violations

**Related Conventions**:

- [Convention Writing Convention](../../repo-governance/conventions/writing/conventions.md)
- [AI Agents Convention](../../repo-governance/development/agents/ai-agents.md)
