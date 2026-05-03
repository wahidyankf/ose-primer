---
title: PRD — Adopt ose-public Vendor-Neutrality, OpenCode Go, and Companion Tooling
---

# Product Requirements

## Personas

- **Template consumer (clone-and-customize)** — the primary downstream
  user. Clones `ose-primer`, expects the OpenCode session to start
  cleanly without first having to debug singular-vs-plural directory
  paths or swap out a vendor model they have no subscription for.
- **Template consumer (cherry-pick-and-merge)** — picks one workstream
  at a time. Needs each workstream to be atomically described and
  independently runnable.
- **ose-primer maintainer** — runs the delivery checklist. Needs each
  phase to leave the tree in a known-good state (tests green, scanner
  clean, sync no-op).
- **Cross-vendor contributor** — works the repo from Cursor / Codex CLI /
  Gemini CLI / Aider / Copilot / Continue / Sourcegraph Cody. Needs
  governance prose that reads as vendor-neutral.
- **Plan agents** (`plan-maker`, `plan-checker`, `plan-execution-checker`) —
  read `governance/conventions/structure/plans.md` to validate plan
  structure. Need the new five-doc DEFAULT and the four-criteria
  single-file exception clearly stated.

## User stories

### W1 — Sync correctness

- _As a_ template consumer launching their first OpenCode session, _I want_
  agents and skills to live at the canonical plural directory paths
  (`.opencode/agents/`, `.opencode/skills/`), _so that_ OpenCode actually
  loads them per the published spec.
- _As a_ ose-primer maintainer running `npm run sync:claude-to-opencode`,
  _I want_ the sync to write to one canonical destination directory,
  _so that_ I never have to debug "why does my OpenCode session not see
  my synced agent" against undocumented behavior.

### W2 — OpenCode Go provider

- _As a_ template consumer with no Z.ai subscription, _I want_ the template
  to ship with OpenCode Go as the default provider, _so that_ I can pick
  any vendor's API key without having to swap defaults before my first
  session.
- _As a_ ose-primer maintainer regenerating `.opencode/agents/`, _I want_
  `rhino-cli ConvertModel()` to emit `opencode-go/*` IDs, _so that_ the
  sync is consistent with the project-level model fields and consumers
  inherit a coherent provider stack.

### W3 — Vendor-audit scanner

- _As a_ governance steward, _I want_ `rhino-cli governance vendor-audit
governance/` to flag every convention violation, _so that_ I do not
  need to manually grep for each forbidden term.
- _As a_ future contributor authoring governance prose, _I want_ the
  pre-push hook to fail when I introduce a forbidden vendor term,
  _so that_ I learn about the violation before my commit lands on
  `main`.

### W4 — Vendor-neutral governance

- _As a_ cross-vendor contributor reading `governance/`, _I want_ load-bearing
  prose to be vendor-neutral with vendor-specific examples in
  `binding-example` fences, _so that_ the rules apply to my AI coding
  agent of choice without translation.
- _As a_ template consumer, _I want_ `AGENTS.md` to be the canonical root
  instruction file (per the AGENTS.md / Linux Foundation Agentic AI
  Foundation standard), and `CLAUDE.md` to be a thin Claude Code
  binding shim that imports it via `@AGENTS.md`, _so that_ I can use
  any AGENTS.md-aware coding agent without losing instructions.

### W5 — Cross-vendor parity gate

- _As a_ ose-primer maintainer, _I want_ `nx run rhino-cli:validate:cross-vendor-parity`
  to verify the five behavioral-parity invariants in one command, _so that_
  parity drift is caught before push.
- _As a_ template consumer, _I want_ the parity gate to be a Nx target wired
  into the pre-push hook, _so that_ regressions cannot land silently.

### W6 — Plans convention refresh

- _As a_ plan author, _I want_ `governance/conventions/structure/plans.md`
  to clearly state that five-doc multi-file is the DEFAULT and to
  enumerate exactly four criteria that must ALL hold for single-file
  to be allowed, _so that_ I do not waste time arguing structure with
  `plan-checker`.
- _As a_ `plan-checker` agent, _I want_ the four single-file criteria to
  be machine-checkable bullet items, _so that_ my structural validation
  is deterministic.

## Acceptance criteria (Gherkin)

### W1 — Sync correctness

```gherkin
Feature: rhino-cli writes synced agents to canonical OpenCode plural path
  Scenario: Sync writes agents to plural directory
    Given a clean ose-primer working tree
    And ".claude/agents/agent-maker.md" exists with valid frontmatter
    When I run "npm run sync:claude-to-opencode"
    Then ".opencode/agents/agent-maker.md" exists
    And ".opencode/agent/" directory does not exist
    And ".opencode/agent/agent-maker.md" does not exist

Feature: rhino-cli writes synced skills to canonical OpenCode plural path
  Scenario: Sync writes skills to plural directory or relies on .claude/ native read
    Given a clean ose-primer working tree
    And ".claude/skills/<skill-name>/SKILL.md" exists
    When I run "npm run sync:claude-to-opencode"
    Then ".opencode/skill/" directory does not exist
    And either ".opencode/skills/<skill-name>/SKILL.md" exists OR the sync explicitly skips skill copy with documented rationale

Feature: agents_validate_sync detects drift against the canonical directory
  Scenario: Drift detection at canonical plural path
    Given ".claude/agents/foo.md" exists
    And ".opencode/agents/foo.md" is stale relative to ".claude/agents/foo.md"
    When I run "rhino-cli agents validate-sync"
    Then exit code is non-zero
    And the report cites ".opencode/agents/foo.md" (plural) as the drifted file
```

### W2 — OpenCode Go provider

```gherkin
Feature: ConvertModel emits opencode-go IDs
  Scenario Outline: Capability-tier mapping
    Given a Claude agent frontmatter "model: <claude-tier>"
    When ConvertModel is called
    Then it returns "<opencode-id>"

    Examples:
      | claude-tier | opencode-id              |
      | opus        | opencode-go/minimax-m2.7 |
      | sonnet      | opencode-go/minimax-m2.7 |
      | haiku       | opencode-go/glm-5        |
      | (omitted)   | opencode-go/minimax-m2.7 |

Feature: opencode.json declares the opencode-go provider block
  Scenario: Provider block resolves API key from env
    Given ".opencode/opencode.json" exists
    When I parse it as JSON
    Then ".model" equals "opencode-go/minimax-m2.7"
    And ".small_model" equals "opencode-go/glm-5"
    And ".provider['opencode-go'].options.apiKey" equals "{env:OPENCODE_GO_API_KEY}"
    And ".mcp" does not contain any Z.ai-bundled MCP server
```

### W3 — Vendor-audit scanner

````gherkin
Feature: rhino-cli governance vendor-audit flags forbidden terms
  Scenario: Forbidden vendor term in governance prose fails the audit
    Given "governance/example.md" contains the load-bearing line "Use Claude Code to run the workflow"
    When I run "rhino-cli governance vendor-audit governance/"
    Then exit code is non-zero
    And the report includes "governance/example.md" with term "Claude Code"

  Scenario: Same term inside a binding-example fence is allowed
    Given "governance/example.md" contains a fenced "```binding-example" block that names "Claude Code"
    When I run "rhino-cli governance vendor-audit governance/"
    Then exit code is zero

  Scenario: Capitalized "Skills" is forbidden in governance prose
    Given "governance/example.md" contains "Skills auto-load from .claude/skills/"
    When I run "rhino-cli governance vendor-audit governance/"
    Then exit code is non-zero
    And the report includes term "\bSkills\b" with replacement suggestion "agent skills"

  Scenario: Convention definition file is allowlisted
    When I run "rhino-cli governance vendor-audit governance/"
    Then "governance/conventions/structure/governance-vendor-independence.md" is not flagged
    even though it contains every forbidden term in its definition table
````

### W4 — Vendor-neutral governance

```gherkin
Feature: governance/ is vendor-neutral after remediation
  Scenario: Full audit returns zero violations
    Given the W3 scanner is installed
    When I run "rhino-cli governance vendor-audit governance/"
    Then exit code is zero
    And the report contains 0 violations

Feature: AGENTS.md is canonical, CLAUDE.md is a thin shim
  Scenario: AGENTS.md vendor-audit
    When I run "rhino-cli governance vendor-audit AGENTS.md"
    Then exit code is zero
    And only binding-example-fenced regions reference vendors

  Scenario: CLAUDE.md imports AGENTS.md
    Given "CLAUDE.md" exists
    When I read "CLAUDE.md"
    Then it contains the line "@AGENTS.md"
    And its prose body cites no forbidden vendor term outside of binding-example fences
```

### W5 — Cross-vendor parity gate

```gherkin
Feature: validate:cross-vendor-parity Nx target verifies five invariants
  Scenario: All five invariants pass on a clean tree
    Given a clean ose-primer working tree post-W1/W2/W3/W4
    When I run "nx run rhino-cli:validate:cross-vendor-parity"
    Then exit code is zero
    And the report verifies:
      | invariant                                                |
      | npm run sync:claude-to-opencode is a no-op               |
      | .claude/agents count matches .opencode/agents count      |
      | governance vendor-audit governance/ returns 0            |
      | color-translation map covers every named color in agents |
      | capability-tier map covers every model tier in agents    |

  Scenario: Drift in one invariant fails the gate with clear citation
    Given the agent count parity is broken (an .opencode/agents/*.md file removed by hand)
    When I run "nx run rhino-cli:validate:cross-vendor-parity"
    Then exit code is non-zero
    And the report cites the count mismatch with file paths

Feature: pre-push hook runs the parity gate for affected projects
  Scenario: Push from a clean tree
    Given a clean ose-primer working tree
    When I run "git push"
    Then the pre-push hook invokes "nx affected -t validate:cross-vendor-parity"
    And the push succeeds
```

### W6 — Plans convention refresh

```gherkin
Feature: plans.md states five-doc multi-file as the DEFAULT
  Scenario: Convention prose
    Given "governance/conventions/structure/plans.md"
    When I read its plan-folder-naming section
    Then it explicitly identifies five-doc multi-file as "DEFAULT"
    And it requires ALL FOUR criteria to hold before single-file is allowed
    And the four criteria match those in the ose-public source plan

  Scenario: Single-file collapse rule
    Given "governance/conventions/structure/plans.md"
    When I grep for the single-file decision rule
    Then I find a single-paragraph statement requiring all four named criteria to be met
    And it states "If any criterion is unmet, use the five-document layout"
```

### W7 — Worktree standard

User stories:

- _As a_ template consumer creating a parallel worktree, _I want_ a single
  authoritative convention saying where the worktree must land on disk
  (`.claude/worktrees/<name>/` for primer; `worktrees/<name>/` for the
  ose-public override), _so that_ I never invent a path that breaks tooling
  expectations.
- _As a_ ose-primer maintainer entering an existing worktree session, _I want_
  the worktree-setup workflow to mandate `npm install` then `npm run doctor -- --fix`
  in that order, _so that_ the polyglot toolchain converges before I run
  any Nx target.

```gherkin
Feature: worktree-path convention exists and is authoritative
  Scenario: Convention file presence and content
    Given "governance/conventions/structure/worktree-path.md" exists
    When I read it
    Then it states the canonical worktree on-disk path for ose-primer (".claude/worktrees/<name>/")
    And it explains the rationale (gitignored, parallel-safety, isolation)
    And it cross-references "governance/development/workflow/worktree-setup.md"

Feature: worktree-setup workflow matches ose-public's current version
  Scenario: Toolchain init order
    Given "governance/development/workflow/worktree-setup.md"
    When I read its initialization-order section
    Then it mandates "npm install" before "npm run doctor -- --fix"
    And it explains the postinstall trailing "|| true" rationale

  Scenario: AGENTS.md and CLAUDE.md cross-link to the worktree-path convention
    Given "AGENTS.md" and "CLAUDE.md"
    When I grep for "worktree-path.md"
    Then both files link to the new convention from their worktree subsection
```

### W8 — Plan + workflow refresh

User stories:

- _As a_ plan-execution agent invoked on this template, _I want_ the
  `plan-execution.md` workflow to match ose-public's current iteration loop,
  _so that_ termination rules and Iron Rules behave the same way the
  upstream consumer's plans agents do.
- _As a_ template consumer onboarding a CI workflow, _I want_ the
  `ci-monitoring.md`, `ci-post-push-verification.md`, and
  `test-driven-development.md` workflows to ship in the template, _so that_
  I do not need to copy them from `ose-public` by hand.

```gherkin
Feature: plan-execution workflow matches ose-public's current shape
  Scenario: Termination rules
    Given "governance/workflows/plan/plan-execution.md"
    When I diff the file against the ose-public version
    Then differences are limited to primer-specific phrasing (single-repo, no parent gitlinks)
    And the iteration loop, Iron Rules, and termination conditions match

Feature: plan-quality-gate workflow matches ose-public's current shape
  Scenario: Termination rule
    Given "governance/workflows/plan/plan-quality-gate.md"
    When I diff the file against the ose-public version
    Then it terminates on "two consecutive zero-finding validations"
    And the max-iterations default is 7 with escalation warning at 5

Feature: companion CI workflows are present
  Scenario: Files present
    Given "governance/development/workflow/" tree
    When I list it
    Then "ci-monitoring.md" and "ci-post-push-verification.md" both exist
    And each file's frontmatter title and purpose statement match ose-public's
```

### W9 — TDD convention

User stories:

- _As a_ ose-primer maintainer driving any code change, _I want_
  `governance/development/workflow/test-driven-development.md` to be the
  authoritative convention spelling out Red→Green→Refactor, _so that_ I
  can cite a single source instead of relying on `implementation.md`'s
  passing reference.
- _As a_ `plan-checker` agent reviewing a plan, _I want_ the TDD
  convention to be reachable from `plan-execution.md` and from
  `implementation.md`, _so that_ I can mechanically check that a
  plan's delivery checklist follows Red→Green→Refactor.
- _As a_ template consumer, _I want_ the test-driven-development
  convention to ship in `governance/development/workflow/`, _so that_ my
  fork inherits the same testing discipline ose-public uses today.

```gherkin
Feature: test-driven-development convention is present and authoritative
  Scenario: File present
    Given "governance/development/workflow/test-driven-development.md" exists
    When I read its first heading
    Then it states "Test-Driven Development Convention"
    And the first paragraph mandates "Write the failing test first, then make it pass, then refactor"

  Scenario: Cross-references in place
    Given "governance/development/workflow/implementation.md"
    When I grep for "test-driven-development.md"
    Then I find at least one link
    And "governance/workflows/plan/plan-execution.md" also links to "test-driven-development.md"

  Scenario: Three-level testing standard cross-link
    Given "governance/development/workflow/test-driven-development.md"
    When I grep for "three-level-testing-standard.md"
    Then I find at least one link in the "Conventions Implemented/Respected" section

  Scenario: Plan-checker can cite TDD when validating delivery checklists
    Given a plan's "delivery.md" with a code-touching item that lacks a preceding "write failing test" item
    When "plan-checker" runs
    Then the report cites "governance/development/workflow/test-driven-development.md" as the violated convention
```

## Definition of Done

- All nine Gherkin Feature groups above pass against `ose-primer`'s tip-of-`main`
  after this plan executes.
- `nx affected -t typecheck lint test:quick spec-coverage` is green.
- `nx run rhino-cli:validate:vendor-audit` is green.
- `nx run rhino-cli:validate:cross-vendor-parity` is green for two consecutive runs.
- `npm run sync:claude-to-opencode` is a no-op.
- `governance/conventions/structure/worktree-path.md` exists and is referenced by `AGENTS.md` / `CLAUDE.md`.
- `governance/workflows/plan/{plan-execution,plan-quality-gate,README}.md` match ose-public's current versions modulo primer-specific phrasing.
- `governance/development/workflow/{ci-monitoring,ci-post-push-verification,test-driven-development}.md` are present.
- Plan archived to `plans/done/2026-05-03__adopt-ose-public-vendor-neutrality-and-opencode-go/`
  with delivery checklist 100% ticked.
