# Delivery Checklist — Planning and Dev Practice Improvement

## Worktree

**Path**: `worktrees/planning-and-dev-practice/`

**Provision**:

```bash
claude --worktree planning-and-dev-practice
```

After the worktree is created, follow
[worktree-setup.md](../../../repo-governance/development/workflow/worktree-setup.md) to
run `npm install` and `npm run doctor -- --fix` from the **repo root** (not from inside
the new worktree directory).

See [worktree-path.md](../../../repo-governance/conventions/structure/worktree-path.md) for the
routing convention. Per [plans.md §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification),
the canonical Worktree section lives in this `delivery.md`.

## Development Environment Setup

No compilation or runtime required. All deliverables are markdown files plus one mechanical
agent-mirror regeneration.

Verify prerequisites:

```bash
node --version    # expect: v24.x.x (managed by Volta)
npm --version     # expect: v11.x.x
which rtk         # expect: path to rtk binary (token-optimized CLI proxy)
```

## Phase 1: Create Grill-Me Skill

### Step 1.1 — RED: Specify expected behavior

- [ ] Read `repo-governance/development/workflow/test-driven-development.md` — confirm
      RED-GREEN-REFACTOR mandate [Repo-grounded]
- [ ] Read `.claude/skills/plan-creating-project-plans/SKILL.md` — confirm skill frontmatter
      format [Repo-grounded]
- [ ] Read `.claude/skills/repo-applying-maker-checker-fixer/SKILL.md` — confirm body
      structure pattern [Repo-grounded]
- [ ] Write a manual test spec (in this checklist's notes): trigger "grill me" in a planning
      session and observe that:
  - First response is exactly one question
  - Question has 2-4 options with trade-off descriptions
  - One option is marked as recommended
  - Codebase is explored before asking answerable questions

Acceptance criterion: behavior spec matches all four Gherkin scenarios in `prd.md §Feature:
Grill-Me Skill Activation` and is understood before writing `SKILL.md`.

### Step 1.2 — GREEN: Create skill file

- [ ] Create directory: `mkdir -p .claude/skills/grill-me/`
- [ ] Write `.claude/skills/grill-me/SKILL.md` with the verbatim content from
      `tech-docs.md §Skill File: .claude/skills/grill-me/SKILL.md → Final content` _New file_
- [ ] Verify file exists: `ls .claude/skills/grill-me/SKILL.md`
- [ ] Verify frontmatter is valid YAML: `head -10 .claude/skills/grill-me/SKILL.md`

Acceptance criterion: file present, frontmatter contains `name: grill-me` and a `description`,
body contains the choice-format question template.

### Step 1.3 — REFACTOR: Validate skill behavior

- [ ] Re-read `.claude/skills/grill-me/SKILL.md` and check it against each of the four Gherkin
      scenarios in `prd.md §Feature: Grill-Me Skill Activation`
- [ ] Verify Rule 1 (one question at a time), Rule 2 (2-4 options), Rule 3 (recommended option
      marked `**(Recommended)**`), and Rule 4 (explore codebase first) are each present and
      unambiguous in the skill body
- [ ] If any of the four Gherkin scenarios cannot be satisfied by the skill text, identify which
      scenario fails, then edit `.claude/skills/grill-me/SKILL.md` to strengthen the relevant rule
      (bundled questions → strengthen Rule 1; no recommendation marked → strengthen Rule 3; no
      codebase exploration → strengthen Rule 4)
- [ ] Run `npm run lint:md` and confirm the new skill file passes

Acceptance criterion: all four Gherkin scenarios in `prd.md §Feature: Grill-Me Skill
Activation` are satisfiable by the skill text, and `npm run lint:md` passes for the new file.

## Phase 2: Update Related Files

### Step 2.1 — Add Step 5g (Harness-Neutrality Scan) to `.claude/agents/plan-checker.md`

_Suggested executor: agent-maker_

- [ ] Read `.claude/agents/plan-checker.md` and locate the "### 12. Anti-Hallucination Scan
      (Step 5f — MANDATORY HARD RULE)" section (currently the last numbered section)
- [ ] Add a new section after it — "### 13. Harness-Neutrality Scan (Step 5g — CONDITIONAL)" —
      describing the conditional check that covers all five validation points from
      `tech-docs.md §Related Files to Update → .claude/agents/plan-checker.md (Step 5g logic)`
- [ ] Ensure the new step is conditional: fires only when the plan touches agents, skills,
      rules, or `repo-governance/` paths; otherwise skipped with no findings
- [ ] Run `npm run generate:bindings` to regenerate the `.opencode/agents/plan-checker.md` mirror
- [ ] Verify the mirror regenerated: `git status --short .opencode/agents/plan-checker.md`
- [ ] Run `npm run lint:md` and confirm zero violations on the changed files

Acceptance criterion: `plan-checker.md` contains a "### 13. Harness-Neutrality Scan (Step 5g —
CONDITIONAL)" section covering all five validation points; `.opencode/agents/plan-checker.md`
is regenerated (not hand-edited); `npm run lint:md` passes.

### Step 2.2 — Reference Step 5g in `repo-governance/workflows/plan/plan-quality-gate.md`

_Suggested executor: repo-workflow-maker_

- [ ] Read `repo-governance/workflows/plan/plan-quality-gate.md`
- [ ] Add a `Step 5g` mention to the "Validation scope (per `plan-checker` Steps 0-7 +
      5b/5c/5d/5e/5f)" list (around line 115) so the documented scope matches the new checker step
- [ ] Add a matching bullet to the "Plan-Specific Validation" section (around line 358)
      describing the conditional harness-neutrality scan
- [ ] Run `npm run lint:md` and confirm zero violations on the changed file

Acceptance criterion: `plan-quality-gate.md` references the Step 5g harness-neutrality scan in
both its validation-scope list and its Plan-Specific Validation section; `npm run lint:md` passes.

### Step 2.3 — Reference grill-me in `repo-governance/workflows/plan/plan-execution.md`

_Suggested executor: repo-workflow-maker_

- [ ] Read `repo-governance/workflows/plan/plan-execution.md`
- [ ] In the `**When to use**:` bullet list (the block beginning at the line `**When to use**:`,
      currently line 39), append one new bullet reading: "Before executing, invoke the grill-me
      skill (`.claude/skills/grill-me/SKILL.md`) to stress-test any unresolved design decisions in
      the plan"
- [ ] Run `npm run lint:md` and confirm zero violations

Acceptance criterion: `plan-execution.md` `**When to use**:` block contains a bullet
referencing the `grill-me` skill for pre-execution design stress-testing.

### Step 2.4 — Strengthen `repo-governance/development/workflow/test-driven-development.md`

_Suggested executor: repo-rules-maker_

- [ ] Read `repo-governance/development/workflow/test-driven-development.md`
- [ ] Confirm it explicitly states that delivery checklists for code steps must use
      RED → GREEN → REFACTOR shape (it does, under "Applying TDD to Plans")
- [ ] Add a `### TDD Shape for Delivery Checklists` subsection under "Applying TDD to Plans"
      using the verbatim command + acceptance-criterion three-substep template from
      `tech-docs.md §TDD Shape for Delivery Checklists`
- [ ] Run `npm run lint:md` and confirm zero violations

Acceptance criterion: `test-driven-development.md` contains a `### TDD Shape for Delivery
Checklists` subsection with the RED/GREEN/REFACTOR command + acceptance template pattern.

### Step 2.5 — Run `repo-rules-maker` to propagate convention

_Suggested executor: repo-rules-maker_

- [ ] Invoke `repo-rules-maker` agent with context: "The planning-and-dev-practice plan has
      been executed: a new `grill-me` planning skill was added at
      `.claude/skills/grill-me/SKILL.md`, `plan-checker.md` gained a Step 5g harness-neutrality
      scan (referenced from `plan-quality-gate.md`), and the TDD delivery-checklist shape was
      strengthened in `test-driven-development.md`. Update all related governance docs, agent
      definitions, and rules to reference these conventions where planning skills, plan quality,
      or TDD practices are mentioned."
- [ ] For each file the agent creates or modifies: read the file and verify it contains no
      contradictions with `AGENTS.md`, `repo-governance/conventions/`, or other referenced
      governance docs
- [ ] If any agent definition was modified, run `npm run generate:bindings` to regenerate mirrors
- [ ] Run `npm run lint:md` — all new/modified files must pass with zero violations

Acceptance criterion: `repo-rules-maker` exits without errors; every new/modified file
passes `npm run lint:md`; no file contradicts an existing governance convention (verified
by reading each changed file).

## Phase 3: Quality Gates

### Step 3.1 — Local markdown lint

- [ ] Run `npm run lint:md` to surface markdown violations across the repo
- [ ] If violations exist, run `npm run lint:md:fix` to auto-fix
- [ ] Run `npm run lint:md` again to confirm zero violations
- [ ] Fix ALL remaining violations by hand, including preexisting issues not caused by this change

Acceptance criterion: `npm run lint:md` exits 0 with zero violations.

### Step 3.2 — Local Nx quality gates

- [ ] Run `npx nx affected -t typecheck lint test:quick spec-coverage`
- [ ] Fix ALL failures found — including preexisting issues encountered during this work
      (root cause orientation principle); do not defer any failure to a follow-up task
- [ ] Re-run the affected targets to confirm they pass

Acceptance criterion: all affected Nx targets pass.

### Step 3.3 — Repo-rules quality gate

The `repo-rules-quality-gate` workflow is agent-delegated (no CLI) per
[repo-rules-quality-gate.md](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md):

- [ ] Invoke the `repo-rules-checker` agent via the Agent tool, scoped to the changed files
      (`.claude/skills/grill-me/`, `.claude/agents/plan-checker.md`, `repo-governance/`), to produce
      an audit report in `generated-reports/`
- [ ] If the report has CRITICAL or HIGH findings, invoke the `repo-rules-fixer` agent via the
      Agent tool with the audit report path to apply validated fixes
- [ ] Re-invoke `repo-rules-checker` until it reports zero CRITICAL and zero HIGH findings

Acceptance criterion: `repo-rules-checker` reports zero CRITICAL and zero HIGH findings.

### Step 3.4 — Thematic commits

Commit each domain separately using Conventional Commits format
[Repo-grounded: `repo-governance/development/workflow/commit-messages.md`]:

```bash
# New skill file
rtk git add .claude/skills/grill-me/
rtk git commit -m "feat(skills): add grill-me planning interrogation skill"

# Agent + workflow + convention doc updates (incl. regenerated mirror)
rtk git add .claude/agents/plan-checker.md .opencode/agents/plan-checker.md repo-governance/
rtk git commit -m "docs(governance): add grill-me, tdd checklist shape, and harness-neutrality plan-checker step"

# Plan files
rtk git add plans/in-progress/planning-and-dev-practice/
rtk git commit -m "docs(plans): add planning-and-dev-practice improvement plan"
```

Split different concerns into separate commits. Do not bundle skill, governance, and plan
changes into a single commit.

### Step 3.5 — Push and verify CI

```bash
rtk git push origin HEAD:main
```

After pushing, monitor GitHub Actions CI per
[ci-monitoring.md](../../../repo-governance/development/workflow/ci-monitoring.md) — use
`ScheduleWakeup` + a single `gh run view` per wake (do NOT use `gh run watch` for long jobs):

```bash
gh run list --branch main --limit 5
gh run view [run-id] --json status,conclusion,jobs
```

Fix any CI failures immediately, including preexisting failures (root cause orientation). Do not
declare work done until CI passes.

Acceptance criterion: all GitHub Actions workflows pass on `origin main`.

## Manual Behavioral Assertions

This plan does not touch UI or API code, so Playwright/curl are not required
[Judgment call: no HTTP endpoints or browser UI in scope]. Manual verification is
skill invocation:

- [ ] In a Claude Code session, say "grill me on the database migration approach"
- [ ] Observe: first response is exactly one question with 2-4 options
- [ ] Observe: one option is clearly marked as recommended
- [ ] Observe: the agent reads relevant repo files before asking questions answerable
      from existing code
- [ ] After several exchanges, observe: agent summarizes all decisions and signals readiness

Acceptance criterion: all five observations pass. (When run autonomously without an interactive
user, substitute a structural review of `.claude/skills/grill-me/SKILL.md` against each
observation and note the substitution.)

## Fix-All-Issues Instruction

When quality gates surface failures, fix ALL of them — not only those caused by this
change. Root cause orientation principle: preexisting issues encountered during this work
must be resolved proactively, not deferred.

## Definition of Done

- [ ] `.claude/skills/grill-me/SKILL.md` exists with correct frontmatter and body
- [ ] Skill asks one question at a time with 2-4 options, marks recommendation
- [ ] `.claude/agents/plan-checker.md` contains Step 5g (Harness-Neutrality Scan, conditional on
      agent/skill/governance changes) and its OpenCode mirror is regenerated
- [ ] `repo-governance/workflows/plan/plan-quality-gate.md` references Step 5g
- [ ] `repo-governance/workflows/plan/plan-execution.md` references grill-me
- [ ] `repo-governance/development/workflow/test-driven-development.md` has a TDD Shape for
      Delivery Checklists subsection
- [ ] `repo-rules-maker` propagation complete and reviewed
- [ ] `npm run lint:md` exits 0
- [ ] `npx nx affected -t typecheck lint test:quick spec-coverage` passes
- [ ] `repo-rules-quality-gate` zero CRITICAL + HIGH findings
- [ ] All changes committed thematically with Conventional Commits
- [ ] CI passes after push to `origin main`
- [ ] Manual behavioral assertions all pass (or structurally substituted with note)

## Plan Archival

Once every item above is complete and CI is green, archive the plan:

- [ ] Move the plan folder to `done/` with completion date prefix:
      `git mv plans/in-progress/planning-and-dev-practice plans/done/2026-05-25__planning-and-dev-practice`
- [ ] Verify `plans/in-progress/README.md` — confirm no `planning-and-dev-practice` entry exists
      (none was added to the active-plans list, so no edit is expected)
- [ ] Update `plans/done/README.md` — add an entry of this exact form (a link to the archived
      folder, description, and completion date):

  ```text
  - [Planning and Dev Practice Improvement](./2026-05-25__planning-and-dev-practice/) — grill-me skill, TDD delivery-checklist shape, harness-neutrality plan-checker Step 5g. Completed 2026-05-25.
  ```

- [ ] Search for orphaned references to `plans/in-progress/planning-and-dev-practice`
      (`rg -n "in-progress/planning-and-dev-practice"`) and fix any found
- [ ] Commit the archival: `rtk git commit -m "chore(plans): move planning-and-dev-practice to done"`
- [ ] Push: `rtk git push origin HEAD:main`

Acceptance criterion: plan folder lives at `plans/done/2026-05-25__planning-and-dev-practice/`, both
plan READMEs are updated, no orphaned `in-progress/planning-and-dev-practice` references remain, and
the archival commit is pushed to `origin main`.
