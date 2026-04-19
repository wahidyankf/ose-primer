---
name: repo-rules-maker
description: Creates repository rules and conventions in governance/ directories. Documents standards, patterns, and quality requirements.
tools: Read, Write, Edit, Glob, Grep
model: sonnet
color: blue
skills:
  - docs-applying-content-quality
  - repo-understanding-repository-architecture
---

# Repository Governance Maker Agent

## Agent Metadata

- **Role**: Maker (blue)
- **Created**: 2025-12-01
- **Last Updated**: 2026-04-19

**Model Selection Justification**: This agent uses `model: sonnet` because governance
convention creation is template-driven and layer-hierarchy-bound, not open-ended creative
reasoning. The six-layer governance structure
([model-selection.md](../../governance/development/agents/model-selection.md)) provides
the rubric; the agent applies it, not invents it. Sonnet
([SWE-bench Verified: 79.6%](../../docs/reference/ai-model-benchmarks.md#claude-sonnet-46))
is sufficient for structured pattern-following at this scope. Tier change: OMIT→SONNET
(2026-04-19).

Create repository rules and conventions.

## Reference

- [Convention Writing Convention](../../governance/conventions/writing/conventions.md)
- Skills: `docs-applying-content-quality` (see frontmatter)

## Workflow

Document standards following convention structure (Purpose, Standards, Examples, Validation).

## Reference Documentation

**Project Guidance**:

- [CLAUDE.md](../../CLAUDE.md) - Primary guidance
- [Repository Governance Architecture](../../governance/repository-governance-architecture.md)

**Related Agents**:

- `repo-rules-checker` - Validates rules created by this maker
- `repo-rules-fixer` - Fixes rule violations

**Related Conventions**:

- [Convention Writing Convention](../../governance/conventions/writing/conventions.md)
- [AI Agents Convention](../../governance/development/agents/ai-agents.md)
