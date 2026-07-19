# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

- [Add `investment-oracle` desktop demo](./add-investment-oracle-app/README.md)
  — second demo family alongside `crud-*`: a four-project desktop suite that ingests
  financial reports (10-K filings, annual reports), generates LLM-driven analysis, and
  exports research dossiers.

## Instructions

**Quick Idea Capture**: For 1-3 liner ideas not ready for formal planning, use `../ideas.md`.

**Naming**: Plans in `in-progress/` use NO date prefix — just the slug (e.g.,
`add-investment-oracle-app/`). A date prefix is applied only when a plan is archived to `done/`,
where it records the completion date.

When starting work on a plan:

1. Move the plan folder: `git mv backlog/[identifier]/ in-progress/[identifier]/`
2. Update the plan's README.md status to "In Progress"
3. Add the plan to this list

When completing a plan:

1. Rename and move: `git mv in-progress/[identifier]/ done/YYYY-MM-DD__[identifier]/` using today's
   completion date
2. Update this list
