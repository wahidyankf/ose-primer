# Product Requirements Document — Plan Domain Parity (ose-primer)

## Product Overview

Bring ose-primer's planning system (workflows, agents, skills, convention) to the merged
3-way canon, add the two governance files primer lacks, modernize both rhino CLI binding
emitters to current vendor conventions, audit the full binding surface, supersede the
overlapping `planning-system-overhaul` plan, and document every decision in a rationale
doc. All decisions trace to the 26-row matrix embedded in
[tech-docs.md](./tech-docs.md#deviation-matrix-verbatim).

## Personas

Solo-maintainer repo — personas are hats and consuming agents:

- **Plan author** (maintainer hat): authors plans in primer using the merged workflows.
- **Template consumer** (downstream user hat): scaffolds a repo from ose-primer and
  inherits the planning system.
- **Planning agents**: `plan-maker`, `plan-checker`, `plan-fixer`,
  `plan-execution-checker`, `repo-setup-manager`.
- **Binding tooling**: `rhino-cli-rust` (canonical), `rhino-cli-go` (parity port),
  consumed via `package.json` scripts and Nx validate targets.

## User Stories

1. As a **plan author**, I want primer's `plan-establishment-execution.md` to accept a
   `target-stage` input and to author plans in a designated worktree by default, so that
   plan establishment behaves identically in all three repos.
2. As a **plan author**, I want the `plan-multi-repo-parity-planning` workflow available
   in primer, so that I can anchor a parity pass from any repo.
3. As a **planning agent**, I want the merged agent and skill definitions, so that plans
   authored in primer are held to the same quality bar as upstream.
4. As a **plan author**, I want the `grilling-with-options.md` convention in primer, so
   that the multi-options grilling rule cited by primer's workflow README resolves to a
   real file.
5. As **binding tooling**, I want OpenCode mirrors emitted with the `permission` object
   and Codex config under official `config.toml` sub-tables, so that primer's bindings
   conform to current vendor conventions.
6. As **binding tooling**, I want `generate:bindings` to invoke the Rust CLI directly via
   `cargo run`, so that the invocation pattern is uniform across all three repos.
7. As a **template consumer**, I want the Go CLI's bindings emission behavior to stay at
   parity with the Rust CLI, validated by the parity guard, so that the dual-CLI guarantee
   holds.
8. As the **maintainer**, I want `planning-system-overhaul` superseded and archived with
   a pointer, so that exactly one plan owns the planning-system concern.
9. As the **maintainer**, I want a rationale doc explaining every matrix decision —
   especially the direct-push-to-main deviation — so that the audit trail survives the
   matrix file (which lives in gitignored `local-temp/`).

## Acceptance Criteria (Gherkin)

### Feature: Merged canon adoption (rows 3–16)

```gherkin
Scenario: target-stage input restored in plan-establishment-execution
  Given the ose-public plan-domain-parity plan has landed the merged canon on its main
  When the merged plan-establishment-execution.md is adopted into primer
  Then "repo-governance/workflows/plan/plan-establishment-execution.md" contains a "target-stage" input
  And the input documents plans/in-progress/ as default and plans/backlog/ as the alternative

Scenario: Worktree-default establishment behavior present
  Given the merged plan-establishment-execution.md is adopted into primer
  When the workflow's authoring mechanics section is read
  Then it specifies authoring the plan in "worktrees/<identifier>/"
  And it specifies provisioning via "git worktree add -b <identifier> worktrees/<identifier> main" plus "npm install" plus "npm run doctor -- --fix" when the worktree is absent
  And it specifies committing in the worktree and pushing HEAD to the confirmed push-target (default "origin main")
  And it specifies removing the worktree after delivery

Scenario: Plan-family agents merged with primer divergences preserved
  Given the merged canon for plan-maker, plan-checker, plan-fixer, and plan-execution-checker
  When the agent files under ".claude/agents/" are updated
  Then each file matches the merged canon except documented repo-specific references
  And ".claude/agents/repo-setup-manager.md" keeps its primer-specific lines only where they are repo-specific (rhino-cli-rust naming)

Scenario: Plan-family skills merged including infra grilling gates
  Given the merged canon for the three plan-family skills
  When ".claude/skills/plan-creating-project-plans/SKILL.md" is updated
  Then it contains the mandatory pre-write and post-write grilling gates adopted from ose-infra
  And ".claude/skills/plan-writing-gherkin-criteria/SKILL.md" and ".claude/skills/grill-me/SKILL.md" match the merged canon

Scenario: Plans Organization Convention merged
  Given the merged canon for conventions/structure/plans.md
  When "repo-governance/conventions/structure/plans.md" is updated
  Then it matches the merged canon modulo primer-specific examples
```

### Feature: New governance files (rows 1, 2, 15)

```gherkin
Scenario: Parity workflow exists in primer with the amended step structure
  When "repo-governance/workflows/plan/plan-multi-repo-parity-planning.md" is created
  Then its step sequence is Survey, Matrix, First Grill (hard gate), web-researcher (conditional), Second Grill (post-research), Author, Gate, Deliver
  And "repo-governance/workflows/plan/README.md" indexes 4 workflows including the parity workflow
  And "repo-governance/workflows/README.md" lists the parity workflow

Scenario: Grilling convention exists in primer
  When "repo-governance/development/workflow/grilling-with-options.md" is created
  Then it merges ose-public's grilling-with-options.md with ose-infra's broader-scope grilling.md content
  And primer documents that previously cited the rule by skill reference resolve to the convention file
```

### Feature: rhino-cli emitter modernization (rows 18–21)

```gherkin
Scenario: OpenCode mirrors use the permission object
  Given the Rust converter previously emitted boolean "tools" flags
  When "npm run generate:bindings" runs after the emitter change
  Then every regenerated ".opencode/agents/*.md" frontmatter contains a "permission" object instead of a boolean "tools" map
  And "validate:sync" (rhino-cli-rust agents validate-sync) exits 0

Scenario: Codex config migrated to config.toml sub-tables
  When the Codex migration step completes
  Then ".codex/config.toml" carries the ci-monitor-subagent configuration under an "agents.ci-monitor-subagent" sub-table per the official Codex config reference
  And the ".codex/agents/" directory no longer exists
  And no rhino-cli code or test references ".codex/agents/"
  And "docs/reference/platform-bindings.md" and "repo-governance/conventions/structure/multi-harness-binding.md" reflect the new layout

Scenario: generate:bindings invokes cargo directly
  When "package.json" is updated
  Then the "generate:bindings" script equals the direct-cargo form using "--manifest-path apps/rhino-cli-rust/Cargo.toml"
  And the sibling scripts (sync:agents, sync:skills, sync:dry-run, validate:sync, validate:claude, validate:harness-bindings) use the same direct-cargo pattern
  And "npm run generate:bindings" exits 0
  And a second consecutive run leaves "git status" clean

Scenario: Go CLI bindings emission stays at capability parity
  Given rhino-cli-go already ships "agents sync" and "agents emit-bindings" commands
  When the row-18 and row-19 emitter changes are ported to "apps/rhino-cli-go"
  Then "nx run rhino-cli-go:test:unit" passes with new permission-object tests
  And "nx run rhino-cli-go:validate:cross-vendor-parity" and "nx run rhino-cli-rust:validate:cross-vendor-parity" both exit 0
  And the generate:bindings script still invokes only the Rust CLI
```

### Feature: Repo-wide binding audit (row 17)

```gherkin
Scenario: Full binding surface audited
  Given primer has 50 agent definitions under ".claude/agents/" (excluding README) and 50 OpenCode mirrors (50:50, no gap as of plan authoring 2026-06-06)
  When the binding audit phase completes
  Then every ".claude/agents/*.md" agent has a corresponding ".opencode/agents/*.md" mirror, or its exclusion is documented in the audit record
  And ".amazonq/rules/00-agents-md.md" and ".amazonq/cli-agents/ose-default.json" are byte-identical to the emitter's expected content
  And ".codex/config.toml" conforms to the post-migration layout
  And "validate:harness-bindings", "validate:sync", and "validate:config" npm scripts all exit 0
```

### Feature: Supersession and rationale (rows 22–24)

```gherkin
Scenario: planning-system-overhaul superseded and archived
  Given only archival items remain unchecked in "plans/in-progress/planning-system-overhaul/delivery.md"
  When the supersession step completes
  Then the old plan's README carries a supersession pointer to "plans/in-progress/plan-domain-parity/"
  And the folder is moved to "plans/done/" with a completion-date prefix via git mv
  And "plans/in-progress/README.md" and "plans/done/README.md" are updated
  And no orphaned references to "in-progress/planning-system-overhaul" remain

Scenario: Rationale doc explains every matrix decision
  When "docs/explanation/plan-domain-parity-decisions.md" is created
  Then it explains all 26 matrix rows with their justifications
  And it documents that this plan reached primer via direct push to origin main from a worktree as an invoker-approved deviation from the PR-only default (Safety Invariant 6)
  And "docs/explanation/README.md" indexes the new doc
```

## Product Scope

### In-Scope Features

- Semantic 3-way merge adoption of 13 governance/agent/skill files (rows 3–16)
- 2 new governance files (rows 1–2, 15)
- Emitter changes in `apps/rhino-cli-rust` and `apps/rhino-cli-go` (rows 18, 19, 21)
- `package.json` script alignment (row 20)
- Binding regeneration + full audit (row 17)
- Plan supersession/archival (row 23)
- Rationale doc + index updates (rows 22, 24)
- Governance doc updates touched by rows 18–20 (`AGENTS.md`, `CLAUDE.md`,
  `docs/reference/platform-bindings.md`, `multi-harness-binding.md`, workflow indexes)

### Out-of-Scope Features

- Sibling repo file changes (own plans)
- Automated drift guard (row 26)
- Go CLI in `generate:bindings` (row 21)
- Any UI or HTTP API surface — no Playwright/curl manual-assertion sections apply
  (this plan touches only governance markdown, CLI code, and config)

## Product-Level Risks

| Risk                                                                | Severity | Note                                                                                        |
| ------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------- |
| Exact `permission` object shape underspecified until upstream lands | MEDIUM   | Phase 1 gate forces upstream canon to land first; primer mirrors the upstream emitter shape |
| 50-vs-47 mirror gap may hide intentional exclusions                 | LOW      | Audit step reconciles each gap explicitly instead of blind regeneration                     |
| Markdown gates (mermaid/links/heading) on large merged docs         | LOW      | All gates run locally pre-commit; fix-all-issues instruction applies                        |
