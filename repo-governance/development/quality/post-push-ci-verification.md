---
title: "Post-Push CI Verification Convention"
description: Requirement to manually trigger and verify related GitHub Actions CI workflows pass after pushing changes to origin main
category: explanation
subcategory: development
tags:
  - ci
  - github-actions
  - verification
  - post-push
  - quality-gates
---

# Post-Push CI Verification Convention

After pushing changes to `origin main`, the contributor — human or AI agent — must trigger the related GitHub Actions CI workflows and verify they pass before declaring the task complete.

## Principles Implemented/Respected

- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**: CI is the automated quality boundary. Bypassing or ignoring CI results defeats its purpose and produces a false signal of completeness.

- **[Root Cause Orientation](../../principles/general/root-cause-orientation.md)**: A CI failure after push is the contributor's responsibility to investigate and fix, not defer. Deferring a known CI failure accumulates debt and can block downstream contributors.

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: The post-push verification step is an explicit required action, not an optional courtesy. Making it explicit prevents the assumption that "push succeeded" equals "quality confirmed".

## Conventions Implemented/Respected

- **[CI Blocker Resolution Convention](./ci-blocker-resolution.md)**: When CI fails, fix the root cause. This convention extends that rule to the post-push phase: the contributor who pushed is responsible for triggering and monitoring CI and owns any failures that result.

- **[Trunk Based Development Convention](../workflow/trunk-based-development.md)**: `main` must always be in a releasable state. Post-push CI verification enforces this invariant by catching regressions before they propagate.

## Scope

### What This Convention Covers

- All pushes to `origin main` (direct pushes, not PR merges handled automatically by GitHub).
- All GitHub Actions workflows triggered by pushes to `main` or that can be dispatched manually via `workflow_dispatch`.
- Human contributors and AI agents alike.

### What This Convention Does NOT Cover

- CI infrastructure failures unrelated to the pushed code (GitHub Actions outage, runner disk full, network timeout) — these are operational issues, not contributor responsibility.
- Scheduled CRON workflows not triggered by or related to the pushed changes.

## Standards

### Standard 1: Trigger Related CI Workflows After Push

After every push to `origin main`, identify which GitHub Actions workflows are relevant to the changed files and trigger them manually via `gh workflow run` or the GitHub Actions UI.

```bash
# Trigger a specific workflow manually
gh workflow run test-crud-be-golang-gin.yml

# Trigger via workflow_dispatch with explicit branch reference
gh workflow run pr-quality-gate.yml --ref main
```

When a workflow is already configured to trigger automatically on `push` to `main`, verify that it started — do not assume it triggered correctly.

### Standard 2: Verify CI Passes Before Declaring Complete

Do not declare a task or plan complete until the triggered CI workflows show a green (passing) status.

```bash
# List recent runs for a specific workflow
gh run list --workflow=test-crud-be-golang-gin.yml --limit=5

# Watch a specific run until it finishes
gh run watch <run-id>
```

"Declaring complete" includes: closing a plan delivery checklist, reporting success to the user, moving a plan to `plans/done/`, or marking a task as done in any context.

### Standard 3: Fix CI Failures Before Moving On

If a triggered workflow fails, investigate the root cause and fix it before moving on. See [CI Blocker Resolution Convention](./ci-blocker-resolution.md) for the mandatory investigation process. Never bypass a failing CI check by re-pushing with modified conditions that skip the check.

### Standard 4: Scope CI Verification to Changed Files

Scope the required CI verification to the files actually changed in the push. Not every push requires triggering every workflow.

| Changed Path Pattern                            | Required CI Workflows                           |
| ----------------------------------------------- | ----------------------------------------------- |
| `governance/**`, `docs/**`, `.claude/agents/**` | `pr-quality-gate.yml` (naming validators, lint) |
| `apps/rhino-cli/**`                             | `test-rhino-cli.yml`                            |
| `apps/crud-be-golang-gin/**`                    | `test-crud-be-golang-gin.yml`                   |
| `apps/crud-fe-ts-nextjs/**`                     | `test-crud-fe-ts-nextjs.yml`                    |
| Any app                                         | The per-app test workflow for that app          |

When a push touches multiple path patterns, trigger all relevant workflows.

### Standard 5: AI Agents Must Perform Post-Push Verification

AI agents that push to `origin main` are subject to the same verification requirement as human contributors. After pushing, an AI agent must:

1. Identify the related CI workflows based on the changed paths.
2. Trigger them via `gh workflow run`.
3. Monitor until completion using `gh run watch` or `gh run list`.
4. Report pass or fail status to the user.
5. Fix any failures before declaring the task complete.

An AI agent that pushes and immediately reports success without verifying CI violates this standard.

## Related Documentation

- [CI Blocker Resolution Convention](./ci-blocker-resolution.md) — Mandatory root-cause investigation process when CI fails.
- [CI/CD Conventions](../infra/ci-conventions.md) — Central reference for CI/CD conventions, GitHub Actions structure, and naming rules.
- [Git Push Default Convention](../workflow/git-push-default.md) — Default push behavior (direct to main, no PR unless requested).
- [Trunk Based Development Convention](../workflow/trunk-based-development.md) — Git workflow establishing `main` as the always-releasable trunk.
