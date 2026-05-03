---
title: Adopt ose-public Vendor-Neutrality, OpenCode Go, and Companion Tooling
status: in-progress
owner: Maintainer
scope:
  - ose-primer
---

# Adopt ose-public Vendor-Neutrality, OpenCode Go, and Companion Tooling

## Overview

`ose-public` shipped a coherent batch of governance, agent, workflow, and
rhino-cli changes between 2026-04-30 and 2026-05-03 that turn `governance/`
into vendor-neutral prose, swap the OpenCode model provider from Z.ai to
OpenCode Go, fix the Claude Code ↔ OpenCode sync to write canonical
plural paths, and operationalize cross-vendor behavioral parity through
two new agents, one new workflow, and one new rhino-cli scanner. As the
upstream MIT template, ose-primer must not lag the downstream consumer:
this plan adopts the full batch into ose-primer in one coordinated effort.

The work decomposes into nine logically independent workstreams that
share execution order because each later workstream consumes invariants
the previous one establishes.

```mermaid
flowchart TD
    A["W1 — Sync Correctness Fix\n.opencode/agent → agents\n.opencode/skill → skills"] --> B["W2 — OpenCode Go Provider\nrhino-cli ConvertModel()\nopencode.json provider block"]
    A --> C["W3 — Vendor-Audit Scanner\ninternal/governance/\ncmd/governance.go\nrhino-cli governance vendor-audit"]
    C --> D["W4 — Vendor-Neutral Governance\nconvention port\nAGENTS.md / CLAUDE.md\ngovernance/ remediation"]
    D --> E["W5 — Cross-Vendor Parity Gate\nrepo-parity-checker / fixer\nworkflow + Nx target\npre-push wire-up"]
    F["W6 — Plans Convention Refresh\n5-doc DEFAULT, 4-criteria gate"] -.parallel.-> D
    G["W7 — Worktree Standard\nworktree-path.md convention\nworktree-setup.md refresh"] -.parallel.-> D
    H["W8 — Plan + Workflow Refresh\nplan-execution.md\nplan-quality-gate.md\nci-monitoring + post-push workflows"] -.parallel.-> E
    I["W9 — TDD Convention\ntest-driven-development.md port\nRed-Green-Refactor enforced in delivery"] -.parallel.-> E
    style A fill:#0173B2,stroke:#000000,color:#FFFFFF
    style B fill:#029E73,stroke:#000000,color:#FFFFFF
    style C fill:#DE8F05,stroke:#000000,color:#000000
    style D fill:#CC78BC,stroke:#000000,color:#000000
    style E fill:#0173B2,stroke:#000000,color:#FFFFFF
    style F fill:#56B4E9,stroke:#000000,color:#000000
    style G fill:#029E73,stroke:#000000,color:#FFFFFF
    style H fill:#DE8F05,stroke:#000000,color:#000000
    style I fill:#CC78BC,stroke:#000000,color:#000000
```

## Scope

**In scope** (ose-primer-only adoption from `ose-public`):

- **W1 — Sync correctness**: rhino-cli writes to `.opencode/agents/` and `.opencode/skills/`
  (plural) instead of singular paths; remove dual-population leftovers.
- **W2 — OpenCode Go provider**: `ConvertModel()` outputs `opencode-go/*` IDs;
  `.opencode/opencode.json` switches `model`/`small_model` and adds provider block;
  Z.ai MCPs removed; `.opencode/agents/*.md` regenerated.
- **W3 — rhino-cli vendor-audit scanner**: port `internal/governance/governance_vendor_audit{,_test}.go`,
  `cmd/governance.go`, `cmd/governance_vendor_audit{,_test}.go`; new Nx target
  `validate:vendor-audit`; godog Gherkin scenarios under `specs/apps/rhino/cli/gherkin/`.
- **W4 — Vendor-neutral governance**: port the convention `governance/conventions/structure/governance-vendor-independence.md`
  (scoped for primer); neutralize `AGENTS.md` (canonical, with binding-example fences) and
  `CLAUDE.md` (Claude Code shim importing `@AGENTS.md`); remediate vendor terms across
  `governance/` until `rhino-cli governance vendor-audit governance/` returns zero violations.
- **W5 — Cross-vendor parity gate**: port `.claude/agents/repo-parity-{checker,fixer}.md`;
  port `governance/workflows/repo/repo-cross-vendor-parity-quality-gate.md`; add Nx target
  `validate:cross-vendor-parity`; wire into pre-push.
- **W6 — Plans convention refresh**: adopt the stricter 5-document-DEFAULT language with
  four explicit single-file exception criteria from `ose-public/governance/conventions/structure/plans.md`.
- **W7 — Worktree standard**: port the missing `governance/conventions/structure/worktree-path.md`
  convention; refresh `governance/development/workflow/worktree-setup.md` to match ose-public's
  current version.
- **W8 — Plan + workflow refresh**: adopt the latest `governance/workflows/plan/{plan-execution,
plan-quality-gate,README}.md` from ose-public; port the missing companion development workflows
  `governance/development/workflow/{ci-monitoring,ci-post-push-verification}.md`.
- **W9 — TDD convention**: port `governance/development/workflow/test-driven-development.md`
  (316 lines, Red→Green→Refactor mandate); cross-link from `implementation.md` and
  `governance/workflows/plan/plan-execution.md`; require all future plan delivery checklists
  to follow Red→Green→Refactor for code-touching items.

**Out of scope** (intentionally excluded):

- DDD bounded-context validators (`rhino-cli bc validate`, `ul validate`) —
  registry is product-specific (organiclever-web). Per `ose-primer-sync`
  classifier these don't propagate; future-plan if a polyglot demo ever needs them.
- caveman / cavemem MCP adoption — developer environment concern, not template scaffolding.
- Z.ai cleanup at the global user-config level — personal/billing concern, not template.
- Parent `ose-projects/` adoption of any of these changes — handled in a separate parent-side plan.

## Reading order

1. [brd.md](./brd.md) — why this batch matters and the cost of the drift across six workstreams
2. [prd.md](./prd.md) — per-workstream functional requirements, Gherkin acceptance criteria
3. [tech-docs.md](./tech-docs.md) — file-level porting map, decision log, rollback per workstream
4. [delivery.md](./delivery.md) — phase-by-phase execution checklist with one action per tick

## Required reading before execution

- ose-public source plans (canonical references):
  - [`2026-05-02__validate-claude-opencode-sync-correctness`](https://github.com/wahidyankf/ose-public/tree/main/plans/done/2026-05-02__validate-claude-opencode-sync-correctness)
  - [`2026-04-30__adopt-opencode-go`](https://github.com/wahidyankf/ose-public/tree/main/plans/done/2026-04-30__adopt-opencode-go)
  - [`2026-05-02__governance-vendor-independence`](https://github.com/wahidyankf/ose-public/tree/main/plans/done/2026-05-02__governance-vendor-independence)
  - [`2026-05-03__cross-vendor-agent-parity`](https://github.com/wahidyankf/ose-public/tree/main/plans/done/2026-05-03__cross-vendor-agent-parity)
  - [`2026-05-03__rhino-cli-skills-vendor-term`](https://github.com/wahidyankf/ose-public/tree/main/plans/done/2026-05-03__rhino-cli-skills-vendor-term)
- Inside an ose-primer-rooted Claude session, `../../ose-public/...` is empty per the
  bare-gitlink contract — read via the GitHub UI above, or open a parent-rooted
  Claude session for filesystem side-by-side diffing.
- [governance/development/infra/nx-targets.md](../../../governance/development/infra/nx-targets.md)
  for caching rules on the new `validate:vendor-audit` and `validate:cross-vendor-parity` targets.
- [governance/development/quality/code.md](../../../governance/development/quality/code.md)
  for the pre-push contract.
- [governance/development/workflow/git-push-default.md](../../../governance/development/workflow/git-push-default.md)
  for the direct-to-main publish path used by every commit in this plan.

## Publish path

**Direct push to `origin main`** per [Git Push Default Convention](../../../governance/development/workflow/git-push-default.md)
Standards 1, 2, 6. No draft PR is opened — the user has not requested one for this plan.
Worktree is optional; if used, push via `git push origin HEAD:main` per Standard 6.

## Document navigation

| Document                       | Purpose                                                    |
| ------------------------------ | ---------------------------------------------------------- |
| [README.md](./README.md)       | Overview, scope, navigation (this file)                    |
| [brd.md](./brd.md)             | Business rationale, success measures, risks                |
| [prd.md](./prd.md)             | Functional requirements, Gherkin acceptance criteria       |
| [tech-docs.md](./tech-docs.md) | File-level porting map, decisions, rollback per workstream |
| [delivery.md](./delivery.md)   | Phased step-by-step execution checklist                    |
