# Rename `repo-governance/` → `repo-governance/`

## Status

In Progress

## Context

The `repo-governance/` directory name is ambiguous: in enterprise contexts, "governance" is a core GRC
(Governance, Risk & Compliance) discipline with its own tooling, frameworks, and teams. Contributors
or tooling searching for GRC artifacts may mistake this directory for that. Renaming to
`repo-governance/` makes the scope unambiguous — this directory governs the **repository**, not a
business compliance program.

This plan adopts the same rename applied to `ose-public` (mirrored from
`ose-public/plans/in-progress/rename-governance-to-repo-governance`), adapted for the `ose-primer`
template repo.

## Scope

| Area             | Detail                                                                                     |
| ---------------- | ------------------------------------------------------------------------------------------ |
| **In scope**     | Rename `repo-governance/` → `repo-governance/` in `ose-primer`; update all path references |
| **Out of scope** | Any content changes inside governance files                                                |
| **Out of scope** | Parent repo `CLAUDE.md` (no `ose-primer/repo-governance/` references exist there)          |
| **Out of scope** | `ose-public`, `ose-infra` repos (separate governance trees; `ose-public` has its own plan) |

## Approach Summary

1. `git mv governance repo-governance` in `ose-primer`
2. Pass A — mass `sed` replace `repo-governance/` → `repo-governance/` across all text files (excluding
   `.git/`, `node_modules/`, `.nx/workspace-data/`, `worktrees/`, `*.out`, `.opencode/agents/`)
3. Pass A also applied to `.husky/pre-push` explicitly (no extension, not caught by `find`)
4. Pass B — bare `"governance"` string in `*.go` files
5. Pass C — CLI verb `repo-governance vendor-audit` (space-separated)
6. Pass D — hyphenated form `repo-governance-vendor-audit`
7. Pass E — Go package rename (`internal/governance` → `internal/repo-governance`)
8. Pass F — Cobra command rename (`governance` → `repo-governance`)
9. Regenerate `.opencode/agents/` via `npm run sync:claude-to-opencode`
10. Run quality gates, push to `main`

## Documents

- [brd.md](./brd.md) — Business rationale
- [prd.md](./prd.md) — Requirements and acceptance criteria
- [tech-docs.md](./tech-docs.md) — Technical design and file impact
- [delivery.md](./delivery.md) — Step-by-step execution checklist
