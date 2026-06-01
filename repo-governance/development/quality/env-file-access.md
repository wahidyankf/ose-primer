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

Prevent AI agents from reading, writing, editing, or committing real environment files
(`.env`, `.env.local`, `.env.production`, etc.). Secrets in those files must never be
exfiltrated, corrupted, or accidentally committed to repository history.

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

| Layer | Mechanism                                    | File                                                                | Scope             |
| ----- | -------------------------------------------- | ------------------------------------------------------------------- | ----------------- |
| 1     | Coding-agent PreToolUse hook (file tools)    | primary-binding hook script                                         | Primary binding   |
| 2     | Declarative deny rules                       | primary-binding permission config                                   | Primary binding   |
| 3     | Bash command guard (in same hook)            | primary-binding hook script                                         | Primary binding   |
| 4     | Secondary-binding permission block           | secondary-binding permission config                                 | Secondary binding |
| 5     | gitignore + pre-commit guard                 | `.gitignore`, `scripts/check-no-env-staged.sh`, `.husky/pre-commit` | All git clients   |
| 6     | This governance rule + `AGENTS.md` reference | This file                                                           | All agents        |

Layers 1–3 protect primary-binding file and Bash tool operations.
Layer 4 protects secondary-binding file operations.
Layer 5 is platform-agnostic: protects every git client (human or agent).
Layer 6 is documentation: establishes the policy, carve-out scope, and known gaps.

See [Platform Binding Examples](#platform-binding-examples) for the concrete file paths for
each active platform binding.

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

| Gap                                                            | Impact                                                       | Compensating Control                                                |
| -------------------------------------------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------------- |
| Secondary-binding Bash cannot express command-level env denies | Secondary-binding agents can run Bash commands touching env  | Primary-binding hook (Layers 1–3) + pre-commit guard (Layer 5)      |
| Primary-binding Bash guard is regex-based (best-effort)        | Sufficiently obfuscated commands may bypass detection        | Declarative deny rules (Layer 2) + pre-commit guard (Layer 5)       |
| Script carve-out is bypassable                                 | Agent could write then execute a script that reads env files | Reviewer vigilance; all scripts are committed to git history        |
| Robust sandbox-level enforcement not delivered                 | No filesystem-level denyRead/denyWrite rules                 | All six layers together; future hardening via agent sandbox support |

## Cross-Platform Scope

This policy applies to all AI agent bindings configured in this repository:

- **Primary binding** (Layers 1–3): PreToolUse hook intercepting file-tool and Bash-tool
  operations; declarative deny rules in the binding's permission config
- **Secondary binding** (Layer 4): File-read and file-edit deny rules in the binding's
  permission config; bash operations permitted to support package runners and project scripts

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

- **[Governance Vendor-Independence Convention](../../conventions/structure/governance-vendor-independence.md)**:
  Body prose uses vendor-neutral terms; all vendor-specific binding details are confined to the
  Platform Binding Examples section below.

## Related Documentation

- [No Secrets in Committed Files Convention](./no-secrets-in-committed-files.md) — the broad
  iron rule this convention enforces for `.env*` files specifically: no system secret may
  enter any committed file (plans, docs, code, config).
- [No Machine-Specific Information in Commits](./no-machine-specific-commits.md) — companion
  rule covering paths, usernames, and local IPs in committed files.

## Platform Binding Examples

All vendor-specific implementation details for this convention are listed here.
The vendor-audit scanner skips this section.

### Primary Binding (Claude Code)

- **Hook script** (Layers 1 & 3): `.claude/hooks/block-env-file-access.sh`
  — PreToolUse hook; reads stdin JSON; denies `Read|Write|Edit|MultiEdit` on real `.env*`;
  denies Bash commands that directly manipulate real `.env*`; exits 2 to block.
- **Permission config** (Layer 2): `.claude/settings.json` — `permissions.deny` array with
  explicit `Read(...)`, `Write(...)`, `Edit(...)` entries for each real env file variant;
  `Read(.env.example)` in `permissions.allow`.
- **Hook registration**: `hooks.PreToolUse` matcher `Read|Write|Edit|MultiEdit|Bash`.

### Secondary Binding (OpenCode)

- **Permission config** (Layer 4): `.opencode/opencode.json` — `permission.read` and
  `permission.edit` glob-map deny rules for each real env variant; `.env.example` allow entry
  placed last (last-match-wins semantics).
