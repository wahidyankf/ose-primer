# Agent Model Selection Standardization

## Context

Adopted from `ose-public` plan `2026-04-19__agent-model-selection-standardization`
(phases 1-6 completed there 2026-04-19). `ose-primer` has the same structural gaps but
different agent count and zero product-specific agents.

The repo has ~45 specialized AI agents split across two runtimes:

- **Claude Code** (`.claude/agents/`) — uses `model: opus|sonnet|haiku` or omit
- **OpenCode** (`.opencode/agent/`) — auto-synced via `npm run sync:claude-to-opencode`
  (rhino-cli builds `.opencode/agent/` from `.claude/agents/`)

The sync already handles model translation:

- Claude `opus` → OpenCode `zai-coding-plan/glm-5.1`
- Claude `sonnet` → OpenCode `zai-coding-plan/glm-5.1`
- Claude `haiku` → OpenCode `zai-coding-plan/glm-5-turbo`
- Claude `""` (omit) → OpenCode `zai-coding-plan/glm-5.1` (default)

## Problems Being Fixed

1. **Budget-adaptive design undocumented** — Opus-tier agents omit the `model` field so
   they inherit the session's active model, adapting to the user's account tier and token
   budget. This is intentional (Max gets Opus 4.7, Pro gets Sonnet 4.6) but nowhere
   documented in `model-selection.md`. The gap risks someone adding `model: opus` to "fix"
   it, breaking the budget-adaptive behavior.
2. **OpenCode mapping undocumented** — `model-selection.md` covers Claude Code only; GLM
   model IDs, the 3-to-2 tier collapse, and GLM capability context are nowhere in policy.
3. **Model version refs stale** — no Claude 4.x IDs, no Haiku 3 retirement notice
   (retired 2026-04-19).
4. **CLAUDE.md lacks inline plan format guidance** — the Plans Organization section
   points to the convention doc without describing the 5-doc format inline. `plan-maker`
   must navigate to the convention to know the format; missing inline description increases
   risk of format errors.
5. **Benchmark data undocumented** — no cited benchmark reference. Tier assignments in
   `model-selection.md` are unverifiable without external research. GLM-5-turbo has no
   standard benchmarks; GLM-5.1 scores are self-reported only. 1 agent incorrectly on
   opus-inherit tier (rubric-bound governance work: `repo-rules-maker`).

## Scope

**Repo**: `ose-primer` (running directly on `main` — governance-only changes, no code).
This plan executes from within `ose-primer` directly; no parent-level orchestration or
worktree required.

**Files touched**:

- `governance/development/agents/model-selection.md` — primary policy + benchmark
  citations _(phases 1, 6)_
- `CLAUDE.md` — inline plan format description + model aliases _(phase 2)_
- `governance/development/agents/ai-agents.md` — budget-adaptive propagation _(phase 3)_
- `governance/development/agents/best-practices.md` — budget-adaptive propagation _(phase 3)_
- `.claude/agents/README.md` — opus-tier omit note + benchmark pointer _(phases 3, 6)_
- `docs/reference/ai-model-benchmarks.md` — new benchmark reference doc _(phase 4)_
- `.claude/agents/repo-rules-maker.md` — tier correction OMIT→SONNET _(phase 5)_
- `.opencode/agent/*.md` — re-synced to reflect tier change _(phase 9)_

**Files NOT touched**: `apps/rhino-cli/` source (no code changes needed), all unchanged
agents (~44 unchanged).

## Approach Summary

No rhino-cli code changes needed. Fix is documentation + 1 targeted tier correction:

1. Update `model-selection.md` — budget-adaptive inheritance, OpenCode section,
   version table, Common Mistakes entry
2. Update `CLAUDE.md` — add inline 5-doc plan format description + model aliases
3. Propagate budget-adaptive note to related governance docs
4. Create `docs/reference/ai-model-benchmarks.md` — cited benchmark reference for
   all 5 models
5. Correct 1 agent tier: `repo-rules-maker` OMIT→SONNET (rubric/layer-hierarchy driven)
6. Add benchmark citations to `model-selection.md` + `.claude/agents/README.md`
7. Run `repo-rules-checker` OCD validation; fix all findings
8. Run `validate:claude`, `validate:sync`, `nx run rhino-cli:test:quick` — all must pass
9. Re-run sync to reflect tier change in `.opencode/agent/`

## Plan Documents

| File                           | Purpose                                                          |
| ------------------------------ | ---------------------------------------------------------------- |
| [brd.md](./brd.md)             | Business rationale, pain points, affected roles, success metrics |
| [prd.md](./prd.md)             | User stories, Gherkin acceptance criteria, product scope         |
| [tech-docs.md](./tech-docs.md) | Sync architecture, model mapping, exact diffs, risk              |
| [delivery.md](./delivery.md)   | Phased execution checklist                                       |

## References

- Upstream plan: `ose-public/plans/in-progress/2026-04-19__agent-model-selection-standardization/`
- Policy: [governance/development/agents/model-selection.md](../../../governance/development/agents/model-selection.md)
- Plans convention: [governance/conventions/structure/plans.md](../../../governance/conventions/structure/plans.md)
- Sync converter: [apps/rhino-cli/internal/agents/converter.go](../../../apps/rhino-cli/internal/agents/converter.go)
- Sync types: [apps/rhino-cli/internal/agents/types.go](../../../apps/rhino-cli/internal/agents/types.go)
