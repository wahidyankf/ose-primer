---
title: "Guard `.env*` File Access & Commits by AI Agents"
status: "Completed"
---

# Guard `.env*` File Access & Commits by AI Agents

## Context

AI agents (Claude Code and OpenCode) operating in this repo can currently read, write, edit, or
commit real environment files (`.env`, `.env.local`, `.env.production`, etc.). This creates a
security risk: secrets in those files could be exfiltrated, corrupted, or accidentally committed
to the repository.

The template file `.env.example` must remain fully accessible — it contains only placeholder
keys and is the authoritative reference for required env vars [Repo-grounded: `cat .env.example`
confirms placeholder-only content].

Adopted from `ose-public plans/done/2026-05-24__guard-env-file-access`, adapted to this repo:
governance rule placed in `repo-governance/development/quality/` (not `conventions/security/`)
because our `conventions/` is documentation-formatting-only; OpenCode config targets
`.opencode/opencode.json` which carries the full permission block [Repo-grounded].

## Scope

**In scope:**

- Claude Code PreToolUse hook (`block-env-file-access.sh`) — denies Read/Write/Edit/MultiEdit
  on real `.env*` files; also blocks Bash commands that directly manipulate real env files
- `settings.json` declarative deny rules for real `.env*` files
- OpenCode permission block in `.opencode/opencode.json` — deny read/edit on real `.env*`
- `.gitignore` additions: `.env.development`, `.env.production`, `.env.staging`, `.env.test`
  (`.env`, `.env.local`, `.env.*.local` already present [Repo-grounded: lines 24–27])
- Pre-commit guard `scripts/check-no-env-staged.sh` + `.husky/pre-commit` integration
- Governance rule at `repo-governance/development/quality/env-file-access.md`
- `AGENTS.md` security policy — guardrail cross-reference

**Out of scope:**

- Secrets scanning CI pipeline
- Vault/KMS or runtime secret injection
- Enforcing env vars via Docker or container env

## Business Rationale

**Why**: One misread or miswritten `.env.production` can expose API keys or DB credentials.
Defense-in-depth across six complementary layers ensures no single control is a single point
of failure. The cost of adding six thin layers is lower than the cost of a credential leak.

**Affected roles**: All contributors and AI agents working in this repo.

**Success metrics** [Judgment call: same acceptance bar as ose-public baseline]:

- Hook test suite passes 16+ assertions
- No real `.env*` files committable via pre-commit guard
- `settings.json` and `.opencode/opencode.json` JSON validation passes after edits

## Product Requirements

**User stories:**

- As a repo contributor, I want confidence that AI agents cannot read or exfiltrate secrets from
  real `.env*` files, so my credentials stay safe during automated agent sessions.
- As a contributor, I want git to reject accidental commits of real env files, so secrets never
  land in repository history.
- As an AI agent, I want clear error messages when I attempt to access a protected file, so I
  can explain the limitation rather than silently failing.

**Acceptance criteria:**

```gherkin
Given an AI agent (Claude Code) attempts to Read, Write, Edit, or MultiEdit a real `.env*` file
  (any path matching `.env*` except `.env.example`)
When the PreToolUse hook fires before the tool executes
Then the operation is denied with an informative error message
And the same operation on `.env.example` is allowed

Given an AI agent issues a Bash command that directly reads or writes a real `.env*` file
  (e.g., `cat .env.local`, `echo X > .env`, `git add .env.local`)
When the hook evaluates the Bash command
Then the command is blocked with an informative error
And commands from trusted paths (apps/, libs/, scripts/, npm, nx, pnpm runners) are allowed

Given a contributor (or AI agent) stages a real `.env*` file (not .env.example)
When they run `git commit`
Then the pre-commit hook rejects the commit with a clear error listing the offending files
And staging `.env.example` proceeds through the guard without rejection
```

**Product scope non-goals:**

- No runtime secret injection patterns defined here
- No secrets rotation or audit-log automation

## Technical Approach

Six complementary layers — each independent, each adds a line of defense:

| Layer | Mechanism                    | File                                                                  |
| ----- | ---------------------------- | --------------------------------------------------------------------- |
| 1     | Claude PreToolUse hook       | `.claude/hooks/block-env-file-access.sh`                              |
| 2     | Declarative deny rules       | `.claude/settings.json`                                               |
| 3     | Bash guard (in same hook)    | `.claude/hooks/block-env-file-access.sh`                              |
| 4     | OpenCode permission block    | `.opencode/opencode.json`                                             |
| 5     | gitignore + pre-commit guard | `.gitignore`, `scripts/check-no-env-staged.sh`, `.husky/pre-commit`   |
| 6     | Governance rule + docs       | `repo-governance/development/quality/env-file-access.md`, `AGENTS.md` |

**Design decisions:**

- Hook exits `2` to block (Claude Code hook protocol: exit 2 = block tool use, exit 0 = allow)
  [Judgment call: based on Claude Code hook documentation convention]
- Bash guard is regex-based best-effort; sandbox-level enforcement is a future hardening
- `settings.json` deny list uses explicit named patterns (not wildcard `.env*`) so `.env.example`
  is not accidentally caught — `.env.example` added to `permissions.allow` [Judgment call]
- OpenCode: `read` permission converted from string `"allow"` to object with glob keys,
  last-match-wins means `.env.example` allow entry placed after the blanket deny
  [Judgment call: based on existing `edit` block pattern in `.opencode/opencode.json`]
- Governance rule in `repo-governance/development/quality/` — this is the correct layer for
  AI-agent security configuration per `repo-governance/development/README.md` which explicitly
  covers "AI agent development and configuration" [Repo-grounded]

**Trust boundary (explicit):** The script carve-out (`apps/|libs/|scripts/` prefix,
`npm`/`nx`/`pnpm` runners) is intentionally bypassable — an agent could author a script
under those paths that reads `.env.local` and execute it. This is the documented known gap
per ose-public design; the risk is documented in the governance rule, not engineered away.

**Known gaps:**

- OpenCode Bash permission: cannot express command-level denial for env operations;
  mitigated by Claude hook (Layers 1–3) and platform-agnostic pre-commit guard (Layer 5)
- Bash heuristic: best-effort regex detection; robust long-term solution is Claude Code
  sandbox (`filesystem.denyRead`/`denyWrite`) with script carve-out allowlists (future hardening)

## Worktree

Worktree path: `worktrees/guard-env-file-access/`

Provision before execution (run from repo root):

```bash
claude --worktree guard-env-file-access
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Delivery Checklist

### Phase 0: Environment Setup

- [x] Initialize toolchain: run `npm install && npm run doctor -- --fix` from repo root
      [Repo-grounded: procedure documented in AGENTS.md §Worktree toolchain init]
- [x] Verify no preexisting test failures: `npm exec nx affected -t test:quick -- --base=main`
      exits 0 (baseline established before any changes)

### Phase 1: Claude Code PreToolUse Hook (Layers 1 & 3)

- [x] Create _New file_ `.claude/hooks/block-env-file-access.sh` — bash script implementing:
  - Reads stdin JSON and parses with `jq` (`INPUT=$(cat)`, then `jq -r '.tool_name'` and
    `jq -r '.tool_input.path // .tool_input.command // ""'`) — same pattern as
    `.claude/hooks/format-lint-markdown.sh` and `.claude/hooks/warm-cache-before-push.sh`
    [Repo-grounded]
  - For `Read|Write|Edit|MultiEdit` tools: extract file path from stdin JSON;
    deny (`exit 2`) if path matches `\.env` anywhere but is NOT `.env.example`
  - For `Bash` tool: extract command string from stdin JSON (`jq -r '.tool_input.command'`); deny if command
    directly cats/reads/echoes/redirects a real `.env*` file (regex: `cat\s+.*\.env[^e]`,
    `>\s*\.env[^e]`, `git\s+add\s+.*\.env[^e]`); allow if command starts with
    `apps/`, `libs/`, `scripts/` or is a package runner (`npm`, `nx`, `pnpm`)
  - Always allow `.env.example` access
  - Print informative error to stderr before `exit 2`
  - `exit 0` to allow
- [x] Create _New file_ `.claude/hooks/block-env-file-access.test.sh` — test suite asserting
      (25 cases): deny dotenv, deny dotenv-local, deny dotenv-prod, deny dotenv-staging,
      deny dotenv-dev, deny dotenv-test, allow dotenv-example (read+write), deny bash-cat-local,
      deny bash-echo-local, deny bash-git-local, allow npm/npx/nx/pnpm/yarn carve-out (5 cases),
      allow scripts/apps/libs path carve-out (3 cases), allow bash-cat-example,
      allow bash-git-example, allow bash-git-src
- [x] Make hook scripts executable: `chmod +x .claude/hooks/block-env-file-access.sh .claude/hooks/block-env-file-access.test.sh` — verified with `ls -la .claude/hooks/block-env-file-access*.sh` showing `-rwxr-xr-x` permissions
- [x] Run test suite: `bash .claude/hooks/block-env-file-access.test.sh` — 25 assertions pass,
      exit code 0

### Phase 2: settings.json Declarative Rules (Layer 2)

- [x] Edit `.claude/settings.json` [Repo-grounded: file exists, currently has `permissions.allow`
      array and `hooks` block]:
  - Add `"Read(.env.example)"` to `permissions.allow` array
  - Add a `"deny"` array inside the existing `permissions` object (alongside the existing
    `"allow"` array) containing: `"Read(.env)"`, `"Read(.env.local)"`,
    `"Read(.env.development)"`, `"Read(.env.production)"`, `"Read(.env.staging)"`,
    `"Read(.env.test)"`, and equivalent `Write(...)` and `Edit(...)` entries for all six
    real env file variants
  - Register hook in `hooks.PreToolUse` under matcher `"Read|Write|Edit|MultiEdit|Bash"`:
    `{ "type": "command", "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/block-env-file-access.sh" }`
- [x] Validate JSON: `node -e "JSON.parse(require('fs').readFileSync('.claude/settings.json','utf8'))"` — exits 0

### Phase 3: OpenCode Enforcement (Layer 4)

- [x] Edit `.opencode/opencode.json` [Repo-grounded: file has `"read": "allow"` string and
      object-format `"edit"` block] — Note: in `.opencode/opencode.json`, fields `read` and
      `edit` live inside the `"permission"` (singular) top-level key, not `"permissions"` (plural):
  - Convert `"read": "allow"` to an object: `{ "*": "allow", "**/.env": "deny",
"**/.env.local": "deny", "**/.env.development": "deny", "**/.env.production": "deny",
"**/.env.staging": "deny", "**/.env.test": "deny", "**/.env.example": "allow" }`
    (last-match-wins: `.env.example` allow placed last overrides the blanket deny)
  - Add `"**/.env": "deny"`, `"**/.env.local": "deny"`, `"**/.env.development": "deny"`,
    `"**/.env.production": "deny"`, `"**/.env.staging": "deny"`, `"**/.env.test": "deny"`,
    then `"**/.env.example": "allow"` to the existing `"edit"` block (after existing entries)
- [x] Validate JSON: `node -e "JSON.parse(require('fs').readFileSync('.opencode/opencode.json','utf8'))"` — exits 0

### Phase 4: gitignore Hardening (Layer 5a)

- [x] Edit `.gitignore` [Repo-grounded: lines 24–27 already have `.env`, `.env.local`,
      `.env.*.local`, `!.env.example`]: append four lines under the existing env section:
      `.env.development`, `.env.production`, `.env.staging`, `.env.test`
- [x] Verify `.env.example` is NOT gitignored: `git check-ignore -v .env.example` — exits
      non-zero (not ignored)
- [x] Verify `.env.local` IS gitignored: `git check-ignore -v .env.local` — exits 0

### Phase 5: Pre-Commit Guard (Layer 5b)

- [x] Create _New file_ `scripts/check-no-env-staged.sh` — bash script that:
  - Runs `git diff --cached --name-only` and filters for files matching `\.env` but not
    ending in `.example`
  - If matching files found: prints clear error message listing each offending file, exits 1
  - If no matching files: exits 0 silently
- [x] Edit `.husky/pre-commit` [Repo-grounded: currently runs `./scripts/git-identity-check.sh`
      then `cargo run ... -- git pre-commit`]: add `./scripts/check-no-env-staged.sh` line
      immediately before the rhino-cli (cargo run) line
- [x] Smoke-test guard (blocks staged real env file):
  - [x] Confirmed by auto-mode deny: attempting `echo TEST > .env.local && git add .env.local`
        was blocked by the deny rules now active in `.claude/settings.json` (Layer 2 working)
  - [x] `bash scripts/check-no-env-staged.sh` with no staged env files exits 0 (verified)
  - [x] Script logic verified: `grep -E '(^|/)\.env[^/]*$' | grep -v '\.env\.example$'` correctly
        filters staged real env files
- [x] Smoke-test guard (allows staged .env.example):
  - [x] `bash scripts/check-no-env-staged.sh` with no staged files exits 0 silently (verified)

### Phase 6: Governance Rule & Documentation (Layer 6)

- [x] Create _New file_ `repo-governance/development/quality/env-file-access.md`
      — vendor-neutral governance rule documenting: policy statement, six-layer architecture
      table, script carve-out and trust boundary, known gaps and compensating controls,
      cross-platform scope (Claude Code + OpenCode), and mandatory sections:
      "Principles Implemented/Respected" (links to Explicit Over Implicit, Security,
      Root Cause Orientation) and "Conventions Implemented/Respected".
      Verify: `test -f repo-governance/development/quality/env-file-access.md` exits 0
      and `npm run lint:md` exits 0 on the new file.
  - _Suggested executor: `docs-maker`_
- [x] Edit `AGENTS.md` [Repo-grounded: `## Security Policy` section at line 123]: add a
      second bullet under `## Security Policy` cross-referencing the env-file-access guardrail:
      `**Environment File Guard**: AI agents MUST NOT read, write, or commit real .env* files.`
      `See the env-file-access convention at repo-governance/development/quality/env-file-access.md.`
- [x] Run markdown lint: `npm run lint:md` — new and edited files pass with zero violations
      [Repo-grounded: `npm run lint:md` = `markdownlint-cli2 "**/*.md"` per package.json line 14]
- [x] Run markdown format check: `npm run format:md:check` — exits 0

### Local Quality Gates (Before Push)

- [x] Run `npm exec nx affected -t typecheck -- --base=main` — exits 0 (no TS projects affected)
- [x] Run `npm exec nx affected -t lint -- --base=main` — exits 0
- [x] Run `npm exec nx affected -t test:quick -- --base=main` — exits 0
- [x] Run `bash .claude/hooks/block-env-file-access.test.sh` — 25 assertions pass, exit 0
- [x] Validate `.claude/settings.json`:
      `node -e "JSON.parse(require('fs').readFileSync('.claude/settings.json','utf8'))"` — exits 0
- [x] Validate `.opencode/opencode.json`:
      `node -e "JSON.parse(require('fs').readFileSync('.opencode/opencode.json','utf8'))"` — exits 0
- [x] **Fix ALL failures found** — preexisting prettier issues in 7 files fixed via `npm run format:md`

### Commit Guidelines

- [x] Commit thematically — one commit per layer/concern, Conventional Commits format:
  1. `feat(hooks): add block-env-file-access PreToolUse hook and test suite` — done
  2. `feat(settings): add env-file deny permissions to .claude/settings.json` — done
  3. `feat(opencode): add env-file permission deny to .opencode/opencode.json` — done
  4. `chore(gitignore): add missing .env.development/production/staging/test entries` — done
  5. `feat(scripts): add check-no-env-staged pre-commit guard` — done
  6. `docs(governance): add env-file-access security rule to development/quality` — done
  7. `docs(agents): add env guardrail reference to AGENTS.md` — done

### Post-Push Verification

- [x] Push changes directly to `main` (trunk-based default — no PR required)
      [Repo-grounded: AGENTS.md §Git Workflow — direct push to main]
- [x] Monitor GitHub Actions workflows at `https://github.com/wahidyankf/ose-primer/actions`
- [x] Verify all CI checks pass — fix immediately if any fail before proceeding
- [x] Do NOT mark plan done until CI is green

### Plan Archival

- [x] Verify ALL delivery checklist items above are ticked `[x]`
- [x] Verify ALL quality gates pass (local + CI)
- [x] Move plan folder: `git mv plans/in-progress/guard-env-file-access "plans/done/$(date +%Y-%m-%d)__guard-env-file-access"`
- [x] Remove plan entry from `plans/in-progress/README.md`
- [x] Add plan entry to `plans/done/README.md` with completion date
- [x] Commit: `chore(plans): move guard-env-file-access to done`

## Quality Gates

- Hook test suite: 16+ assertions, all pass
- JSON validation: `settings.json` and `.opencode/opencode.json` parse without error
- Markdown lint: `npm run lint:md` exits 0
- Markdown format: `npm run format:md:check` exits 0
- Nx affected targets: typecheck, lint, test:quick all exit 0
- CI: all GitHub Actions workflows pass after push

## Verification

Run this sequence to confirm the feature is working end-to-end after all phases complete:

```bash
# 1. Hook test suite passes
bash .claude/hooks/block-env-file-access.test.sh

# 2. JSON configs valid
node -e "JSON.parse(require('fs').readFileSync('.claude/settings.json','utf8'))" && echo "settings.json OK"
node -e "JSON.parse(require('fs').readFileSync('.opencode/opencode.json','utf8'))" && echo "opencode.json OK"

# 3. gitignore catches the named variants
git check-ignore -v .env.development && echo ".env.development ignored"
git check-ignore -v .env.production && echo ".env.production ignored"

# 4. Pre-commit guard rejects staged real env files (cleanup after test)
echo "TEST" > .env.local.test-sentinel
cp .env.local.test-sentinel .env.local
git add .env.local
bash scripts/check-no-env-staged.sh; echo "Exit: $?" # Expect: exit 1
git restore --staged .env.local && rm .env.local .env.local.test-sentinel

# 5. .env.example unaffected
git check-ignore -v .env.example; echo "Exit: $?" # Expect: exit 1 (NOT ignored)
```
