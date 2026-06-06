# Plans

<!--
  MAINTENANCE NOTE: Brief landing page
  For comprehensive documentation, see:
  repo-governance/conventions/structure/plans.md
-->

This folder contains temporary, ephemeral project planning documents, distinct from permanent documentation in `docs/`.

## 🧭 Quick Reference

- **ideas.md** - Quick 1-3 liner ideas not yet formalized into plans
- ⏳ **backlog/** - Planned projects for future implementation
- 🚧 **in-progress/** - Active plans currently being worked on
- ✅ **done/** - Completed and archived plans

## Complete Documentation

For detailed information on plans organization, structure, naming conventions, and workflow, see:

**[Plans Organization Convention](../repo-governance/conventions/structure/plans.md)**

## Plan Folder Naming

Naming rules differ by stage:

| Stage          | Pattern                             | Date            |
| -------------- | ----------------------------------- | --------------- |
| `backlog/`     | `YYYY-MM-DD__[project-identifier]/` | Creation date   |
| `in-progress/` | `[project-identifier]/`             | No date prefix  |
| `done/`        | `YYYY-MM-DD__[project-identifier]/` | Completion date |

Examples:

- `backlog/2025-12-01__auth-system/` (creation date)
- `in-progress/auth-system/` (no date)
- `done/2025-12-15__auth-system/` (completion date)

For the full rules and rationale, see the [Plans Organization Convention](../repo-governance/conventions/structure/plans.md#plan-folder-naming).

## Related Documentation

- [How to Organize Your Work](../docs/how-to/organize-work.md) - Decision guide for plans/ and docs/
