# BRD: Rename `demo-*` → `crud-*`

## Problem

All current demo apps are CRUD expense-tracker apps. They are named `demo-*`, which
means "demonstration" — ambiguous once a second demo family (e.g. AI chat) is added.
A template user cloning ose-primer would see `demo-be-golang-gin` sitting next to
`ai-chat-be-golang-gin` with no naming signal distinguishing them beyond the first
segment.

## Root cause

The prefix `demo` was chosen before a multi-family demo strategy existed. It conflates
two orthogonal concerns: "this is a demo app" (true of all families) and "this app
demonstrates CRUD patterns" (true only of the current family).

## Proposed solution

Rename the existing family to `crud-*`. The prefix now encodes **what the app
demonstrates**, not just that it is a demo. Future families follow the same pattern:
`ai-chat-*`, `realtime-*`, etc.

## Business value

| Value                       | Detail                                                                              |
| --------------------------- | ----------------------------------------------------------------------------------- |
| Naming clarity              | Template users instantly understand the purpose of each app family                  |
| Unblocks roadmap            | AI chat apps can be introduced without naming collisions or confusion               |
| Reduces onboarding friction | New contributors don't need context to understand family boundaries                 |
| Convention enforceability   | `rhino-cli agents validate-naming` and `spec-coverage` can enforce per-family rules |

## Out of scope

- Creating AI chat apps (separate plan)
- Migrating any existing deployment data (template repo, no production instances)
- Changing the underlying app functionality — behavior is unchanged, names differ

## Success at business level

A developer cloning ose-primer after this change can read the `apps/` directory listing
and immediately understand: "crud-\* apps show CRUD patterns; future families will be
named for their own patterns."

## Affected roles

- Maintainer (template author): performs all implementation steps, validates all quality gates
- AI agents (plan-executor, swe-\* dev agents): read this plan to orient implementation work; rely on consistent Nx project names in targets and commands
- Template consumers (cloners): benefit from the clearer naming after the rename lands; `apps/` listing immediately communicates family purpose

## Business risks

| Risk                                                                                       | Likelihood | Mitigation                                                                                    |
| ------------------------------------------------------------------------------------------ | ---------- | --------------------------------------------------------------------------------------------- |
| Partial rename leaves stale `demo-` strings in non-obvious file categories                 | Medium     | Phase 19 stale-reference audit grep; CI red on misnamed project                               |
| Build breakage due to missed Nx project reference                                          | Low        | Phase 21 `nx graph` validation catches broken deps                                            |
| Flutter codegen produces stale `demo_contracts` package name                               | Medium     | Phase 5 extended sweep + Phase 21 codegen run regenerates; verify pubspec.yaml post-codegen   |
| CI workflow files still named `test-demo-*.yml` after rename, causing navigation confusion | Medium     | Dedicated `.github/workflows/` phase (`git mv` all 15 files) + Phase 19 stale-reference audit |
| Agent or governance workflow references `demo-*` project name in a generated command       | Low        | Phase 18 `.claude/` audit + Phase 19 final stale-reference grep across all file types         |
