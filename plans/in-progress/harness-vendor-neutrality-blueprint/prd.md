---
title: "PRD: Harness/Vendor Neutrality Blueprint — Phase 1 (ose-primer)"
---

# Product Requirements Document: Harness/Vendor Neutrality Blueprint — Phase 1

## Product Overview

Establish a harness/vendor neutrality blueprint for ose-primer and deliver its first concrete
implementation: replace the vendor-locked `sync:claude-to-opencode` npm script with
`generate:bindings`, which regenerates all secondary binding artifacts (OpenCode + Amazon Q).
Update all documentation, agent definitions, workflow files, parity scripts, and governance to use
the new name. Remove the old script completely (hard delete — no alias, no passthrough).

## Personas

- **AI agent** — reads agent definition files containing instructions to run the sync script; must
  receive the correct, unified command name covering all harnesses
- **Human contributor** — follows governance documentation; expects a single command to keep all
  harnesses current after editing `.claude/` source files
- **Cross-vendor parity checker** — runs Invariant 3; must verify ALL secondary binding
  directories are clean after a fresh `generate:bindings` run
- **Blueprint reader** — future team member or harness adopter; must find a clear definition of
  vendor-neutral zones, rules, and enforcement mechanisms in the plan

## User Stories

**US-1**: As an AI agent editing `.claude/agents/*.md` files, I want a single command that
regenerates all secondary bindings (OpenCode + Amazon Q), so that no harness is silently left
stale after my edits.

**US-2**: As a human contributor, I want the npm script name to be vendor-neutral, so that the
script's purpose is obvious without reading its implementation and I know it applies to all
harnesses, not just one.

**US-3**: As the cross-vendor parity checker, I want Invariant 3 to verify that ALL secondary
bindings (including Amazon Q) are up to date after a `generate:bindings` run, so that a passing
parity check actually guarantees full harness coverage.

**US-4**: As a future harness adopter, I want a written blueprint that defines vendor-neutral
zones, vendor-specific zones, and enforcement mechanisms, so that I can onboard without introducing
vendor-locked naming.

**US-5**: As a governance contributor, I want `repo-governance/` prose to be verifiably
vendor-neutral (passing `vendor-audit`), so that AI agents reading governance docs receive
harness-neutral instructions regardless of which harness they use.

## Acceptance Criteria (Gherkin)

```gherkin
Feature: Harness/vendor neutrality blueprint — Phase 1 (ose-primer)

  Background:
    Given the repository root contains package.json with npm scripts
    And the primary binding source is .claude/agents/*.md
    And rhino-cli is built from apps/rhino-cli-rust/

  Scenario: generate:bindings runs both OpenCode and Amazon Q generation
    Given I have edited a .claude/agents/*.md file
    When I run: npm run generate:bindings
    Then rhino-cli agents sync is invoked (generates .opencode/agents/*.md)
    And rhino-cli agents emit-bindings is invoked (generates .amazonq/ bridge files)
    And git diff --quiet .opencode/ .amazonq/ exits 0 after the run

  Scenario: sync:claude-to-opencode is fully removed (hard delete)
    Given the plan has been executed
    When node -e "const p=require('./package.json'); console.log(p.scripts['sync:claude-to-opencode'])" is run
    Then the output is undefined
    And no alias, passthrough, or redirect exists for the old name

  Scenario: validate:config uses generate:bindings
    When I run: npm run validate:config
    Then generate:bindings is invoked as part of the validation pipeline
    And the string sync:claude-to-opencode does not appear in validate:config's invocation chain

  Scenario: Invariant 3 covers both OpenCode and Amazon Q
    Given the cross-vendor parity checker runs Invariant 3
    When Invariant 3 is evaluated
    Then the checker tool runs: npm run generate:bindings
    And verifies both .opencode/ and .amazonq/ are clean with git diff --quiet

  Scenario: No reference to old script name remains anywhere in the repo
    Given all files in the repo are scanned (excluding generated-reports/, dist/, and plans/)
    When grep -r "sync:claude-to-opencode" is run with --include="*.md" --include="*.json" --include="*.sh"
    Then zero matches are returned
    And the old name is completely absent from the codebase

  Scenario: Agent maker instructions use generate:bindings
    Given the agent-maker.md agent definition file
    When its description and body are read
    Then the instruction to regenerate bindings refers to generate:bindings not sync:claude-to-opencode

  Scenario: Both dual-CLI parity scripts use generate:bindings
    Given apps/rhino-cli-rust/scripts/validate-cross-vendor-parity.sh
    And apps/rhino-cli-go/scripts/validate-cross-vendor-parity.sh
    When each script is read
    Then every invocation and error message refers to generate:bindings not sync:claude-to-opencode

  Scenario: Governance vendor-audit passes
    Given all plan phases have been executed
    When vendor-audit is run against repo-governance/
    Then it exits 0
    And no governance prose outside exempt sections contains vendor product names

  Scenario: Governance propagation — repo-rules-maker confirms convention coverage
    Given the Phase 2 governance sweep is complete
    When repo-rules-maker is invoked to check for convention gaps
    Then it either confirms multi-harness-binding.md already covers the generate:bindings naming pattern
    Or it creates or updates a convention entry documenting the harness-neutral npm script requirement
    And the new entry does NOT reuse the AD8 number (already taken by Dual-Implementation Byte-Parity)
    And no new convention duplicates content already in multi-harness-binding.md

  Scenario: repo-rules-quality-gate passes in strict mode after all changes
    Given all phases (1–3) have been applied
    When the repo-rules-quality-gate workflow is run in strict mode
    Then it reaches double-zero CRITICAL/HIGH/MEDIUM findings
    And the vendor-audit command exits 0 against repo-governance/
```

## Product Scope

### In-Scope Features

- `generate:bindings` npm script that builds rhino-cli once, then runs `agents sync` and
  `agents emit-bindings` sequentially
- `sync:claude-to-opencode` fully removed (hard delete — no alias, no passthrough)
- `validate:config` updated to use `generate:bindings`
- All governance `.md` files updated (grep-verified)
- All `.claude/agents/*.md` and `.claude/skills/` references updated
- Invariant 3 tooling string updated in the cross-vendor parity gate workflow and the
  `repo-parity-checker` / `repo-parity-fixer` agents (now covers `.amazonq/`)
- Both dual-CLI parity scripts updated (Rust + Go)
- `repo-rules-maker` invoked to confirm convention coverage or create new convention entry
- `repo-rules-quality-gate` run in strict mode until double-zero findings
- `vendor-audit` confirmed passing against `repo-governance/`

### Out-of-Scope Features

- `generate:bindings:dry-run` or `generate:bindings:agents-only` variants
- Removing `sync:agents`, `sync:skills`, `sync:dry-run` targeted scripts
- Changing rhino-cli CLI subcommand names (`agents sync`, `agents emit-bindings`)
- Changing Rust or Go rhino-cli source logic
- Any new harness support
- Merging `repo-parity-*` into `repo-harness-compatibility-*`

## Product-Level Risks

| Risk                                                    | Impact                                   | Mitigation                                                                       |
| ------------------------------------------------------- | ---------------------------------------- | -------------------------------------------------------------------------------- |
| Missed reference in a `.md`/`.sh` file                  | Agent uses old name; Amazon Q left stale | grep-verify step in delivery after bulk rename before any commit                 |
| `validate:config` invoked in CI before the rename lands | CI uses old name briefly                 | Phases 1–3 land in coordinated push; no window where old name is absent          |
| PostToolUse hook reformats files after Edit             | `old_string` mismatch on next Edit       | Read file before each targeted Edit                                              |
| `.opencode/` mirror not auto-synced for some file       | Stale old name persists in a mirror      | Phase 3 reruns `generate:bindings` and greps `.opencode/`; manual fix if needed  |
| vendor-audit fails on governance prose                  | Governance docs contain vendor names     | vendor-audit run as Phase 5 gate; prose fixed to neutral terms before proceeding |
