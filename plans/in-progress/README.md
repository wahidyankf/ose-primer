# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

- [Add `investment-oracle` desktop demo](./add-investment-oracle-app/README.md)
  — second demo family alongside `crud-*`: a four-project desktop suite that ingests
  financial reports (10-K filings, annual reports), generates LLM-driven analysis, and
  exports research dossiers.
- [Mermaid state diagram validation](./mermaid-state-diagram-validation/README.md)
  — extend rhino-cli's Mermaid render-discipline rules to cover `stateDiagram-v2` and
  `stateDiagram`, re-shape the validator onto a fresh kind-agnostic module design, and
  clean up every violating state diagram repo-wide.
- [Standardize CI parity](./standardize-ci-parity/README.md)
  — third sibling of the three-repo `standardize-ci-parity` set (with `ose-public` and
  `ose-infra`): converge ose-primer CI to the shared, static Converged CI Target. Primer
  is already the most converged sibling; the gaps it closes are concurrency blocks on all
  workflows, a `specs-gate` job, and scheduled-cadence alignment. Parallel-safe — no
  cross-repo plan ordering.

## Folder Naming

Folders in `in-progress/` use the bare project identifier only — **no date prefix**:

```
[project-identifier]/
```

Example: `add-investment-oracle-app/` (not `2026-04-27__add-investment-oracle-app/`)

When a plan is moved from `backlog/` to `in-progress/`, the `YYYY-MM-DD__` prefix is stripped. See the [Plans Organization Convention](../../repo-governance/conventions/structure/plans.md#plan-folder-naming) for full naming rules.

## Instructions

**Quick Idea Capture**: For 1-3 liner ideas not ready for formal planning, use `../ideas.md`.

When starting work on a plan:

1. Move the plan folder from `backlog/` to `in-progress/` via `git mv`
2. Strip the date prefix: rename `YYYY-MM-DD__[identifier]/` to `[identifier]/`
3. Update the plan's README.md status to "In Progress"
4. Add the plan to this list

When completing a plan:

1. Determine the completion date (date of the last file modification in the folder)
2. Add the completion-date prefix: rename `[identifier]/` to `YYYY-MM-DD__[identifier]/`
3. Move the renamed folder from `in-progress/` to `done/` via `git mv`
4. Remove the plan from this list
