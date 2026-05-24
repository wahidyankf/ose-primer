---
title: "Environment File Access Convention"
description: >
  Vendor-neutral policy governing AI agent access to real .env* files.
  Defines six defence layers, script carve-out, trust boundary, and known gaps.
category: explanation
subcategory: development
tags:
  - security
  - ai-agents
  - env-files
  - conventions
created: 2026-05-24
---

# Environment File Access Convention

## Purpose

Prevent AI agents (Claude Code and OpenCode) from reading, writing, editing, or committing real
environment files (`.env`, `.env.local`, `.env.production`, etc.). Secrets in those files must
never be exfiltrated, corrupted, or accidentally committed to repository history.

The template file `.env.example` is explicitly permitted at every layer — it contains only
placeholder keys and is the authoritative reference for required env vars.

## Policy

**Rule**: AI agents operating in this repository MUST NOT:

1. Read, write, or edit any file matching `.env*` except `.env.example`
2. Execute Bash commands that directly read, write, or stage real `.env*` files
3. Commit any file matching `.env*` except `.env.example` to git history

**Permitted**: Operations on `.env.example`; invocations of project scripts under `apps/`,
`libs/`, or `scripts/` paths; package runner commands (`npm`, `npx`, `nx`, `pnpm`, `yarn`).

## Six Defence Layers

| Layer | Mechanism                                    | File                                                                | Scope           |
| ----- | -------------------------------------------- | ------------------------------------------------------------------- | --------------- |
| 1     | Claude PreToolUse hook (file tools)          | `.claude/hooks/block-env-file-access.sh`                            | Claude Code     |
| 2     | Declarative deny rules                       | `.claude/settings.json` `permissions.deny`                          | Claude Code     |
| 3     | Bash command guard (in same hook)            | `.claude/hooks/block-env-file-access.sh`                            | Claude Code     |
| 4     | OpenCode permission block                    | `.opencode/opencode.json` `permission`                              | OpenCode        |
| 5     | gitignore + pre-commit guard                 | `.gitignore`, `scripts/check-no-env-staged.sh`, `.husky/pre-commit` | All git clients |
| 6     | This governance rule + `AGENTS.md` reference | This file                                                           | All agents      |

Layers 1–3 protect Claude Code file and Bash tool operations.
Layer 4 protects OpenCode file operations.
Layer 5 is platform-agnostic: protects every git client (human or agent).
Layer 6 is documentation: establishes the policy, carve-out scope, and known gaps.

## Script Carve-Out

The following are **always allowed**, even when they touch env file paths at runtime:

- Commands prefixed with `apps/`, `libs/`, or `scripts/` (trusted project scripts)
- Package runner invocations: `npm`, `npx`, `nx`, `pnpm`, `yarn`

**Rationale**: Legitimate project automation (e.g., `scripts/setup-env.sh`) must be able to
manage env files as part of the developer workflow. The carve-out allows this while blocking
direct agent manipulation.

## Trust Boundary

The script carve-out is **intentionally bypassable** — an agent could author a script under
`apps/`, `libs/`, or `scripts/` that reads `.env.local` and then execute it. This is a
deliberate design tradeoff: the risk is documented here (not engineered away) because:

1. Authoring new scripts requires Write tool access, which is gated by the same permission model
2. Reviewers and the pre-commit guard catch any `.env*` file staged for commit
3. Perfect enforcement (sandbox-level) is a future hardening captured as a known gap

## Known Gaps & Compensating Controls

| Gap                                                   | Impact                                                       | Compensating Control                                              |
| ----------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------------------------- |
| OpenCode Bash cannot express command-level env denies | OpenCode agents can run Bash commands touching env files     | Claude hook (Layers 1–3) + pre-commit guard (Layer 5)             |
| Claude Bash guard is regex-based (best-effort)        | Sufficiently obfuscated commands may bypass detection        | Declarative deny rules (Layer 2) + pre-commit guard (Layer 5)     |
| Script carve-out is bypassable                        | Agent could write then execute a script that reads env files | Reviewer vigilance; all scripts are committed to git history      |
| Robust sandbox-level enforcement not delivered        | No filesystem-level denyRead/denyWrite rules                 | All six layers together; future hardening via Claude Code sandbox |

## Cross-Platform Scope

This policy applies to both AI agent platforms configured in this repository:

- **Claude Code** (primary binding): Layers 1–3 via PreToolUse hook; Layer 2 via
  `permissions.deny` in `.claude/settings.json`
- **OpenCode** (secondary binding): Layer 4 via `permission.read` and `permission.edit`
  glob deny rules in `.opencode/opencode.json`; bash operations permitted to support
  package runners and project scripts

Layer 5 (git) and Layer 6 (governance) are platform-agnostic.

## Principles Implemented/Respected

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**:
  Policy is enumerated across six named layers with explicit file paths, carve-outs, and
  known gaps — no hidden assumptions.

- **[Root Cause Orientation](../../principles/general/root-cause-orientation.md)**:
  The policy addresses the root cause (unrestricted agent file access) rather than patching
  symptoms (e.g., post-commit secrets scanning alone).

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**:
  Each layer is a thin, independent control. The carve-out is a deliberate scope boundary,
  not an attempt to solve every possible attack vector.

## Conventions Implemented/Respected

- **[File Naming Convention](../../conventions/structure/file-naming.md)**: This file uses
  kebab-case, matches the convention identifier, and lives under the correct governance layer.

- **[Content Quality Principles](../../conventions/writing/quality.md)**: Active voice, single
  H1, no time estimates, concrete examples over abstract descriptions.

- **[Linking Convention](../../conventions/formatting/linking.md)**: All cross-references use
  relative paths with `.md` extensions.
