# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

- [2026-04-27 — Add `investment-oracle` desktop demo](./2026-04-27__add-investment-oracle-app/README.md)
  — second demo family alongside `crud-*`: a four-project desktop suite that ingests
  financial reports (10-K filings, annual reports), generates LLM-driven analysis, and
  exports research dossiers.
- [2026-05-03 — Adopt ose-public vendor-neutrality, OpenCode Go, and companion tooling](./2026-05-03__adopt-ose-public-vendor-neutrality-and-opencode-go/README.md)
  — nine-workstream batch from the upstream consumer:
  W1 sync correctness (`.opencode/agent` → `.opencode/agents`), W2 OpenCode Go provider,
  W3 rhino-cli vendor-audit scanner, W4 vendor-neutral governance (AGENTS.md canonical,
  CLAUDE.md shim, `governance/` remediation), W5 cross-vendor parity gate (two new agents
  - workflow + Nx target), W6 stricter five-doc-DEFAULT plans convention,
    W7 worktree-path convention + worktree-setup refresh, W8 plan + workflow refresh
    (plan-execution / plan-quality-gate / ci-monitoring / ci-post-push-verification),
    W9 test-driven-development convention.

## Instructions

**Quick Idea Capture**: For 1-3 liner ideas not ready for formal planning, use `../ideas.md`.

When starting work on a plan:

1. Move the plan folder from `backlog/` to `in-progress/`
2. Update the plan's README.md status to "In Progress"
3. Add the plan to this list

When completing a plan:

1. Move the plan folder from `in-progress/` to `done/`
2. Update this list
