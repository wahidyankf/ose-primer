---
title: "Plan Workflows"
description: Orchestrated workflows for establishing, validating, and executing project plans
category: explanation
subcategory: workflows
tags:
  - plan
  - workflows
  - orchestration
created: 2026-05-12
---

# Plan Workflows

Orchestrated workflows for project planning quality validation and systematic execution.

## Purpose

These workflows define **WHEN and HOW to establish, validate, and execute plans**. The
plan-establishment workflow orchestrates the full prompt-to-pushed-plan lifecycle (repo
exploration → grill → research → plan-maker → quality gate → push). The plan-quality-gate
workflow orchestrates `plan-checker` and `plan-fixer` for authoring-time validation. The
plan-execution workflow is orchestrated directly by the calling context (which delegates
per-item work to specialized agents) and invokes `plan-execution-checker` for independent
validation at the end.

## Grilling Format (All Plan-Creation Workflows)

Every plan-creation workflow that invokes grilling (pre-write and post-write) MUST follow
the multi-options grilling format defined in the
[`grill-me` skill](../../../.claude/skills/grill-me/SKILL.md):

- Ask **one question at a time**
- Present **2–4 concrete options** with trade-off descriptions for each question (no
  open-ended questions)
- Mark the recommended option with **(Recommended)**

This format applies to both the First Grill (before writing) and the Second Grill
(post-research, in plan-establishment-execution) steps in the plan-establishment workflow.

## Scope

**✅ Workflows Here:**

- Plan quality validation
- Plan execution tracking
- Iterative plan improvement
- Multi-agent orchestration for plans/
- Check-fix-verify and execution cycles

**❌ Not Included:**

- Content quality validation (that's docs/)
- App-specific content validation (use per-app agents directly)
- Single-agent operations (use agents directly)

## Workflows

- [Plan Establishment](./plan-establishment-execution.md) - Orchestrate the full prompt-to-pushed-plan
  lifecycle: repo exploration → grill → web research → grill → plan-maker → plan-quality-gate →
  push. Use when turning a behavioral prompt into a production-ready plan.
- [Plan Execution](./plan-execution.md) - Execute plan tasks systematically with validation and completion tracking; orchestrated directly by the calling context, validated by `plan-execution-checker`
- [Plan Quality Gate](./plan-quality-gate.md) - Validate plan completeness and accuracy, apply fixes iteratively until ZERO findings using plan-checker and plan-fixer

## Related Documentation

- [Workflows Index](../README.md) - All orchestrated workflows
- [Plans Organization Convention](../../conventions/structure/plans.md) - Plan structure standards, including:
  - [§Execution Markers: `[AI]` vs `[HUMAN]`](../../conventions/structure/plans.md#execution-markers-ai-vs-human) — executor tagging, legend requirement, handoff/resume signal rule
  - [§Phase Gates and Natural Pauses](../../conventions/structure/plans.md#phase-gates-and-natural-pauses-hard-rule) — per-phase gate + Pause Safety requirement, barrier rule
- [Maker-Checker-Fixer Pattern](../../development/pattern/maker-checker-fixer.md) - Core workflow pattern
- [Repository Architecture](../../repository-governance-architecture.md) - Six-layer governance model
