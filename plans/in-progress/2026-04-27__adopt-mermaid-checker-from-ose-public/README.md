---
title: Adopt ose-public Mermaid Checker Enhancements
status: in-progress
owner: rhino-cli
---

# Adopt ose-public Mermaid Checker Enhancements

## Overview

Port the subgraph-density rule, direction-mapped warning fields, and supporting
parser changes from `ose-public/apps/rhino-cli/internal/mermaid/` into the
ose-primer template, extend the pre-push Nx target so the new rule actually
guards `docs/` and `plans/`, and remediate every Mermaid block in the repo
that the upgraded checker flags. ose-primer is the upstream template and
should not lag the downstream consumer.

## Scope

- **In scope**
  - `apps/rhino-cli/internal/mermaid/{types,parser,validator,reporter,*_test}.go`
  - `apps/rhino-cli/cmd/docs_validate_mermaid*.go`
  - `apps/rhino-cli/project.json` (Nx target inputs and command paths)
  - `.husky/pre-push` (no logic change expected; verify still fires)
  - All `**/*.md` under `docs/`, `governance/`, `.claude/`, `plans/`,
    repository-root that contain ` ```mermaid ` blocks (153 files
    surveyed at plan time)
- **Out of scope**
  - Reciprocal sync from ose-public into ose-infra (separate plan if ever
    needed)
  - New rules beyond what ose-public already ships (no node-count or
    edge-count gating)
  - Changing the existing three rules' default thresholds
    (`--max-label-len 30`, `--max-width 4`)

## Reading order

1. [brd.md](./brd.md) — why this work matters and the cost of the drift
2. [prd.md](./prd.md) — functional requirements, Gherkin acceptance criteria
3. [tech-docs.md](./tech-docs.md) — file-level porting map and remediation
   strategy
4. [delivery.md](./delivery.md) — phase-by-phase execution checklist

## Required reading before execution

- Mermaid checker source — ose-public (canonical reference for the
  port). Inside an ose-primer-rooted Claude session, the parent
  gitlink renders `../../ose-public/...` as an empty directory by
  the bare-gitlink contract. Read via the GitHub UI:
  `https://github.com/wahidyankf/ose-public/tree/main/apps/rhino-cli/internal/mermaid`
  and
  `https://github.com/wahidyankf/ose-public/tree/main/apps/rhino-cli/cmd`.
  Alternatively open a parent-rooted Claude session for filesystem
  side-by-side view.
- [governance/development/infra/nx-targets.md](../../../governance/development/infra/nx-targets.md)
  for caching rules on the upgraded `validate:mermaid` target.
- [governance/development/quality/code.md](../../../governance/development/quality/code.md)
  for the pre-push contract.
- [governance/development/workflow/git-push-default.md](../../../governance/development/workflow/git-push-default.md)
  for the direct-to-main publish path used by every commit in this
  plan.
- [plans/in-progress/README.md](../README.md) for status
  conventions.

## Quality gates

- `nx affected -t typecheck lint test:quick spec-coverage` clean before
  pushing.
- `nx run rhino-cli:test:unit` ≥ 90% coverage maintained (Go threshold).
- `nx run rhino-cli:validate:mermaid` exits 0 on the full repo after
  remediation.
- Husky pre-push hook fires `validate:mermaid` on a `*.md` change and
  returns exit 0.
- All preexisting non-mermaid CI failures encountered must be fixed in
  the same plan run per root-cause orientation.

## Execution readiness

- This plan executes inside the ose-primer-rooted Claude session
  (or a worktree-isolated session created via
  `claude --worktree adopt-mermaid-checker`).
- Publish path: **direct push to `origin main`** per
  [Git Push Default Convention](../../../governance/development/workflow/git-push-default.md)
  Standards 1, 2, 6. Draft PR is opt-in only when the user
  explicitly requests one for review-warranting changes — no such
  request has been made for this plan.
- Worktree is optional. If used: branch name `worktree-adopt-mermaid-checker`,
  push via `git push origin HEAD:main` per Standard 6.
- Linear history maintained via `git pull --rebase origin main`
  before each push per Standard 4.
