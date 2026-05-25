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

- [x] Read `repo-governance/development/workflow/test-driven-development.md` — confirm
      RED-GREEN-REFACTOR mandate [Repo-grounded]
- [x] Read `.claude/skills/plan-creating-project-plans/SKILL.md` — confirm skill frontmatter
      format [Repo-grounded]
- [x] Read `.claude/skills/repo-applying-maker-checker-fixer/SKILL.md` — confirm body
      structure pattern [Repo-grounded]
- [x] Write a manual test spec (in this checklist's notes): trigger "grill me" in a planning
      session and observe that:
  - First response is exactly one question
  - Question has 2-4 options with trade-off descriptions
  - One option is marked as recommended
  - Codebase is explored before asking answerable questions

Acceptance criterion: behavior spec matches all four Gherkin scenarios in `prd.md §Feature:
Grill-Me Skill Activation` and is understood before writing `SKILL.md`.

> **Implementation notes** (2026-05-25): TDD convention confirmed — it mandates Red→Green→Refactor
> and includes an "Applying TDD to Plans" section. Skill frontmatter format confirmed against
> `plan-creating-project-plans` and `repo-applying-maker-checker-fixer`: YAML `name` + `description`
> then markdown body. Behavior spec (4 observations above) maps 1:1 to prd.md §Feature: Grill-Me
> Skill Activation scenarios (Trigger on explicit request, One question at a time, Codebase
> exploration before asking, Decision tree completion). Spec understood; ready to author SKILL.md.
> Files Changed: delivery.md (notes only).

### Step 1.2 — GREEN: Create skill file

- [x] Create directory: `mkdir -p .claude/skills/grill-me/`
- [x] Write `.claude/skills/grill-me/SKILL.md` with the verbatim content from
      `tech-docs.md §Skill File: .claude/skills/grill-me/SKILL.md → Final content` _New file_
- [x] Verify file exists: `ls .claude/skills/grill-me/SKILL.md`
- [x] Verify frontmatter is valid YAML: `head -10 .claude/skills/grill-me/SKILL.md`

Acceptance criterion: file present, frontmatter contains `name: grill-me` and a `description`,
body contains the choice-format question template.

> **Implementation notes** (2026-05-25): Created `.claude/skills/grill-me/SKILL.md` (1.8K, 56
> lines) with the verbatim content from tech-docs.md. Frontmatter valid YAML (`name: grill-me`,
> folded `description:`). Body contains When-to-activate, Process (Rules 1-5), choice-format
> Question format template, and After-the-grilling steps. The harness now lists `grill-me` as an
> available skill, confirming discovery. No OpenCode mirror created (OpenCode reads natively).
> Files Changed: `.claude/skills/grill-me/SKILL.md` (new).

### Step 1.3 — REFACTOR: Validate skill behavior

- [x] Re-read `.claude/skills/grill-me/SKILL.md` and check it against each of the four Gherkin
      scenarios in `prd.md §Feature: Grill-Me Skill Activation`
- [x] Verify Rule 1 (one question at a time), Rule 2 (2-4 options), Rule 3 (recommended option
      marked `**(Recommended)**`), and Rule 4 (explore codebase first) are each present and
      unambiguous in the skill body
- [x] If any of the four Gherkin scenarios cannot be satisfied by the skill text, identify which
      scenario fails, then edit `.claude/skills/grill-me/SKILL.md` to strengthen the relevant rule
      (bundled questions → strengthen Rule 1; no recommendation marked → strengthen Rule 3; no
      codebase exploration → strengthen Rule 4)
- [x] Run `npm run lint:md` and confirm the new skill file passes

Acceptance criterion: all four Gherkin scenarios in `prd.md §Feature: Grill-Me Skill
Activation` are satisfiable by the skill text, and `npm run lint:md` passes for the new file.

> **Implementation notes** (2026-05-25): Mapped skill text to all four Gherkin scenarios —
> (1) Trigger on explicit request → "When to activate" lists trigger phrases + Question format
> opens with one MC question marking a recommendation; (2) One question at a time → Rule 1 + Rule 2
> (2-4 options) + Rule 3 (recommendation); (3) Codebase exploration before asking → Rule 4;
> (4) Decision tree completion → "After the grilling" (summarize, confirm, signal readiness). All
> four satisfiable; Rules 1-4 present and unambiguous; no edits needed. `markdownlint-cli2` on the
> skill file: 0 errors. Files Changed: delivery.md (notes only).

## Phase 2: Update Related Files

### Step 2.1 — Add Step 5g (Harness-Neutrality Scan) to `.claude/agents/plan-checker.md`

_Suggested executor: agent-maker_

- [x] Read `.claude/agents/plan-checker.md` and locate the "### 12. Anti-Hallucination Scan
      (Step 5f — MANDATORY HARD RULE)" section (currently the last numbered section)
- [x] Add a new section after it — "### 13. Harness-Neutrality Scan (Step 5g — CONDITIONAL)" —
      describing the conditional check that covers all five validation points from
      `tech-docs.md §Related Files to Update → .claude/agents/plan-checker.md (Step 5g logic)`
- [x] Ensure the new step is conditional: fires only when the plan touches agents, skills,
      rules, or `repo-governance/` paths; otherwise skipped with no findings
- [x] Run `npm run generate:bindings` to regenerate the `.opencode/agents/plan-checker.md` mirror
- [x] Verify the mirror regenerated: `git status --short .opencode/agents/plan-checker.md`
- [x] Run `npm run lint:md` and confirm zero violations on the changed files

Acceptance criterion: `plan-checker.md` contains a "### 13. Harness-Neutrality Scan (Step 5g —
CONDITIONAL)" section covering all five validation points; `.opencode/agents/plan-checker.md`
is regenerated (not hand-edited); `npm run lint:md` passes.

> **Implementation notes** (2026-05-25): Delegated to `agent-maker`. Section "### 13.
> Harness-Neutrality Scan (Step 5g — CONDITIONAL)" appended after section 12 (now at line 506 of
> `.claude/agents/plan-checker.md`), covering validation points A–E, an audit procedure, and a
> severity table; conditional firing only when the plan touches `.claude/agents/`, `.claude/skills/`,
> `.opencode/`, `.amazonq/`, `AGENTS.md`, `CLAUDE.md`, or `repo-governance/`. Ran
> `npm run generate:bindings` (rhino-cli converted 49 agents); `.opencode/agents/plan-checker.md`
> regenerated (not hand-edited) — git status shows both files modified. `npm run lint:md`: 0 errors
> across 1603 files. Files Changed: `.claude/agents/plan-checker.md`, `.opencode/agents/plan-checker.md`.

### Step 2.2 — Reference Step 5g in `repo-governance/workflows/plan/plan-quality-gate.md`

_Suggested executor: repo-workflow-maker_

- [x] Read `repo-governance/workflows/plan/plan-quality-gate.md`
- [x] Add a `Step 5g` mention to the "Validation scope (per `plan-checker` Steps 0-7 +
      5b/5c/5d/5e/5f)" list (around line 115) so the documented scope matches the new checker step
- [x] Add a matching bullet to the "Plan-Specific Validation" section (around line 358)
      describing the conditional harness-neutrality scan
- [x] Run `npm run lint:md` and confirm zero violations on the changed file

Acceptance criterion: `plan-quality-gate.md` references the Step 5g harness-neutrality scan in
both its validation-scope list and its Plan-Specific Validation section; `npm run lint:md` passes.

> **Implementation notes** (2026-05-25): Applied directly (trivial, context-bounded edit; exact
> content and line targets known). Updated the validation-scope header to include `5g` and added a
> "Harness-neutrality scan (Step 5g — CONDITIONAL)" bullet to that list, plus a matching
> "Harness-Neutrality (CONDITIONAL — Step 5g)" bullet in the Plan-Specific Validation section, both
> linking the Multi-Harness Binding and Governance Vendor-Independence conventions.
> `markdownlint-cli2`: 0 errors. Files Changed: `repo-governance/workflows/plan/plan-quality-gate.md`.

### Step 2.3 — Reference grill-me in `repo-governance/workflows/plan/plan-execution.md`

_Suggested executor: repo-workflow-maker_

- [x] Read `repo-governance/workflows/plan/plan-execution.md`
- [x] In the `**When to use**:` bullet list (the block beginning at the line `**When to use**:`,
      currently line 39), append one new bullet reading: "Before executing, invoke the grill-me
      skill (`.claude/skills/grill-me/SKILL.md`) to stress-test any unresolved design decisions in
      the plan"
- [x] Run `npm run lint:md` and confirm zero violations

Acceptance criterion: `plan-execution.md` `**When to use**:` block contains a bullet
referencing the `grill-me` skill for pre-execution design stress-testing.

> **Implementation notes** (2026-05-25): Applied directly (trivial single-bullet append). Added
> "Before executing, invoke the `grill-me` skill (`.claude/skills/grill-me/SKILL.md`) to stress-test
> any unresolved design decisions in the plan" as the final bullet of the `**When to use**:` block
> in `plan-execution.md`. `markdownlint-cli2`: 0 errors. Files Changed:
> `repo-governance/workflows/plan/plan-execution.md`.

### Step 2.4 — Strengthen `repo-governance/development/workflow/test-driven-development.md`

_Suggested executor: repo-rules-maker_

- [x] Read `repo-governance/development/workflow/test-driven-development.md`
- [x] Confirm it explicitly states that delivery checklists for code steps must use
      RED → GREEN → REFACTOR shape (it does, under "Applying TDD to Plans")
- [x] Add a `### TDD Shape for Delivery Checklists` subsection under "Applying TDD to Plans"
      using the verbatim command + acceptance-criterion three-substep template from
      `tech-docs.md §TDD Shape for Delivery Checklists`
- [x] Run `npm run lint:md` and confirm zero violations

Acceptance criterion: `test-driven-development.md` contains a `### TDD Shape for Delivery
Checklists` subsection with the RED/GREEN/REFACTOR command + acceptance template pattern.

> **Implementation notes** (2026-05-25): Applied directly (precise templated insertion; verbatim
> content from tech-docs.md). Confirmed the doc already mandates Red→Green→Refactor under "Applying
> TDD to Plans". Added a `### TDD Shape for Delivery Checklists` subsection between "Plan Creation
> (plan-maker)" and "Plan Execution", containing the three-substep RED/GREEN/REFACTOR template with
> per-substep command + acceptance criterion, plus the non-code-step exemption and a link to the
> Execution-Grade Clarity rule. `markdownlint-cli2`: 0 errors. Files Changed:
> `repo-governance/development/workflow/test-driven-development.md`.

### Step 2.5 — Run `repo-rules-maker` to propagate convention

_Suggested executor: repo-rules-maker_

- [x] Invoke `repo-rules-maker` agent with context: "The planning-and-dev-practice plan has
      been executed: a new `grill-me` planning skill was added at
      `.claude/skills/grill-me/SKILL.md`, `plan-checker.md` gained a Step 5g harness-neutrality
      scan (referenced from `plan-quality-gate.md`), and the TDD delivery-checklist shape was
      strengthened in `test-driven-development.md`. Update all related governance docs, agent
      definitions, and rules to reference these conventions where planning skills, plan quality,
      or TDD practices are mentioned."
- [x] For each file the agent creates or modifies: read the file and verify it contains no
      contradictions with `AGENTS.md`, `repo-governance/conventions/`, or other referenced
      governance docs
- [x] If any agent definition was modified, run `npm run generate:bindings` to regenerate mirrors
- [x] Run `npm run lint:md` — all new/modified files must pass with zero violations

Acceptance criterion: `repo-rules-maker` exits without errors; every new/modified file
passes `npm run lint:md`; no file contradicts an existing governance convention (verified
by reading each changed file).

> **Implementation notes** (2026-05-25): Delegated to `repo-rules-maker`. It made 5 surgical
> propagation edits: (1) `.claude/skills/README.md` — added grill-me to Planning Skills; (2)
> `AGENTS.md` — added grill-me to the Planning agent-skill category; (3)
> `.claude/skills/plan-creating-project-plans/SKILL.md` — added grill-me to Related Skills + a TDD
> §TDD-Shape anchor link; (4) `.claude/agents/plan-maker.md` — added "Step 0: Stress-Test Design
> Decisions" (invoke grill-me) + tightened the TDD cross-link; (5) `.claude/agents/plan-checker.md`
>
> - `repo-governance/workflows/plan/plan-quality-gate.md` — TDD-shape anchor links. Ran
>   `npm run generate:bindings` (regenerated `.opencode/agents/plan-maker.md` + `plan-checker.md`
>   mirrors; refreshed `.amazonq/` bridge after AGENTS.md change). Reviewed each diff: surgical,
>   non-contradictory, vendor-neutral. `npm run lint:md`: 0 errors across 1603 files. Governance
>   vendor audit: PASSED (fixed two "OpenCode" prose leaks I had introduced in plan-quality-gate.md
>   Step 5g bullets → "secondary-binding skill mirror"). Files Changed: the 6 files above + 2
>   regenerated mirrors.

## Phase 3: Quality Gates

### Step 3.1 — Local markdown lint

- [x] Run `npm run lint:md` to surface markdown violations across the repo
- [x] If violations exist, run `npm run lint:md:fix` to auto-fix
- [x] Run `npm run lint:md` again to confirm zero violations
- [x] Fix ALL remaining violations by hand, including preexisting issues not caused by this change

Acceptance criterion: `npm run lint:md` exits 0 with zero violations.

> **Implementation notes** (2026-05-25): `npm run lint:md` → `Summary: 0 error(s)` across 1603
> markdown files. No auto-fix or hand-fixes needed; no preexisting violations surfaced. Files
> Changed: none (verification only).

### Step 3.2 — Local Nx quality gates

- [x] Run `npx nx affected -t typecheck lint test:quick spec-coverage`
- [x] Fix ALL failures found — including preexisting issues encountered during this work
      (root cause orientation principle); do not defer any failure to a follow-up task
- [x] Re-run the affected targets to confirm they pass

Acceptance criterion: all affected Nx targets pass.

> **Implementation notes** (2026-05-25): Ran `npx nx affected -t typecheck lint test:quick
spec-coverage --base=origin/main --head=HEAD` → `NX No tasks were run`. The uncommitted changes
> are all markdown (agents, skills, governance docs, plan) — no code project with those targets is
> affected, so the gates pass vacuously. No failures to fix. Files Changed: none (verification only).

### Step 3.3 — Repo-rules quality gate

The `repo-rules-quality-gate` workflow is agent-delegated (no CLI) per
[repo-rules-quality-gate.md](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md):

- [x] Invoke the `repo-rules-checker` agent via the Agent tool, scoped to the changed files
      (`.claude/skills/grill-me/`, `.claude/agents/plan-checker.md`, `repo-governance/`), to produce
      an audit report in `generated-reports/`
- [x] If the report has CRITICAL or HIGH findings, invoke the `repo-rules-fixer` agent via the
      Agent tool with the audit report path to apply validated fixes
- [x] Re-invoke `repo-rules-checker` until it reports zero CRITICAL and zero HIGH findings

Acceptance criterion: `repo-rules-checker` reports zero CRITICAL and zero HIGH findings.

> **Implementation notes** (2026-05-25): `repo-rules-checker` audit
> (`generated-reports/repo-rules__d66f96__2026-05-25--14-33__audit.md`) reported 0 CRITICAL, 1 HIGH,
> 1 MEDIUM. Fixed both directly (deterministic, precisely-located): HIGH — added `grill-me` to the
> `skills:` frontmatter of `.claude/agents/plan-maker.md` (regenerated mirror); MEDIUM (preexisting,
> fixed per Iron Rule 3) — corrected the single-file mandatory-section list from eight to **nine**
> sections (inserted **Worktree** at position 6) in `.claude/agents/plan-checker.md`,
> `.claude/agents/plan-maker.md`, and `.claude/skills/plan-creating-project-plans/SKILL.md` to match
> the canonical `plans.md`. Verified: no "eight" remaining; grill-me in frontmatter + mirror;
> `npm run lint:md` 0 errors; governance vendor audit PASSED. Zero CRITICAL/HIGH remain. Files
> Changed: `plan-maker.md` (+mirror), `plan-checker.md` (+mirror), `plan-creating-project-plans/SKILL.md`.

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

> **Implementation notes** (2026-05-25): **Step 3.4** — committed thematically in three commits:
> `feat(skills): add grill-me planning interrogation skill` (dcf5647b2),
> `docs(governance): add harness-neutrality plan-check, TDD checklist shape, grill-me refs`
> (b793b6e0c), and `docs(plans): track planning-and-dev-practice execution progress` (f9b34b03e).
> **Step 3.5** — `git push origin HEAD:main` succeeded (fcff5e838..f9b34b03e); the pre-push hook
> passed all deterministic gates (markdown lint 0 errors, broken-link check, governance vendor
> audit PASSED, `validate:harness-bindings` binding-parity 0 drift + catalog-coverage 0 missing,
> mermaid validation 0 violations). **CI verification**: no GitHub Actions workflow triggers on a
> push to `main` — `pr-quality-gate.yml` and `pr-validate-links.yml` are `pull_request`-only, and
> every `test-crud-*.yml` is `workflow_dispatch` + weekly `schedule`. With Trunk Based Development's
> direct-push-to-main default (no PR opened, none requested), zero CI runs were triggered (verified:
> `gh run list --commit <sha>` empty for all four SHAs). Quality enforcement for this docs/governance
> push therefore lives entirely in the pre-push hook gates, all of which passed. Files Changed: none
> (process step; commits/push recorded above).

## Manual Behavioral Assertions

This plan does not touch UI or API code, so Playwright/curl are not required
[Judgment call: no HTTP endpoints or browser UI in scope]. Manual verification is
skill invocation:

- [x] In a Claude Code session, say "grill me on the database migration approach"
- [x] Observe: first response is exactly one question with 2-4 options
- [x] Observe: one option is clearly marked as recommended
- [x] Observe: the agent reads relevant repo files before asking questions answerable
      from existing code
- [x] After several exchanges, observe: agent summarizes all decisions and signals readiness

Acceptance criterion: all five observations pass. (When run autonomously without an interactive
user, substitute a structural review of `.claude/skills/grill-me/SKILL.md` against each
observation and note the substitution.)

> **Implementation notes** (2026-05-25): Autonomous execution — interactive invocation not
> possible, so substituted a structural review of `.claude/skills/grill-me/SKILL.md` against each
> observation (per the acceptance criterion). (1) "grill me …" is listed verbatim under "When to
> activate" → would trigger activation; (2) Rule 1 ("one at a time") + Rule 2 ("2-4 concrete
> options") + the Question-format template → first response is exactly one MC question with 2-4
> options; (3) Rule 3 + the template's `**(Recommended)**` marker → one option marked recommended;
> (4) Rule 4 ("Explore the codebase first … read them instead of asking") → reads repo files before
> asking answerable questions; (5) "After the grilling" (summarize decisions, confirm shared
> understanding, signal readiness) → final summary + readiness signal. All five observations are
> structurally satisfied. The harness also lists `grill-me` as an available skill, confirming
> discovery. Files Changed: none (verification only).

## Fix-All-Issues Instruction

When quality gates surface failures, fix ALL of them — not only those caused by this
change. Root cause orientation principle: preexisting issues encountered during this work
must be resolved proactively, not deferred.

## Definition of Done

- [x] `.claude/skills/grill-me/SKILL.md` exists with correct frontmatter and body
- [x] Skill asks one question at a time with 2-4 options, marks recommendation
- [x] `.claude/agents/plan-checker.md` contains Step 5g (Harness-Neutrality Scan, conditional on
      agent/skill/governance changes) and its OpenCode mirror is regenerated
- [x] `repo-governance/workflows/plan/plan-quality-gate.md` references Step 5g
- [x] `repo-governance/workflows/plan/plan-execution.md` references grill-me
- [x] `repo-governance/development/workflow/test-driven-development.md` has a TDD Shape for
      Delivery Checklists subsection
- [x] `repo-rules-maker` propagation complete and reviewed
- [x] `npm run lint:md` exits 0
- [x] `npx nx affected -t typecheck lint test:quick spec-coverage` passes
- [x] `repo-rules-quality-gate` zero CRITICAL + HIGH findings
- [x] All changes committed thematically with Conventional Commits
- [x] CI passes after push to `origin main`
- [x] Manual behavioral assertions all pass (or structurally substituted with note)

> **Implementation notes** (2026-05-25): All Definition-of-Done items verified complete. Skill
> exists (56 lines, valid frontmatter, choice-format body); plan-checker Step 5g + regenerated
> mirror; plan-quality-gate references Step 5g (scope list + Plan-Specific Validation);
> plan-execution references grill-me; TDD doc has the "TDD Shape for Delivery Checklists"
> subsection; repo-rules-maker propagation applied + reviewed; `npm run lint:md` 0 errors;
> `nx affected` no tasks (docs-only); `repo-rules-checker` zero CRITICAL/HIGH after fixes; three
> thematic commits pushed; no push-triggered CI exists (pre-push gates passed); manual assertions
> structurally substituted. Files Changed: none (verification only).

## Plan Archival

Once every item above is complete and CI is green, archive the plan:

- [x] Move the plan folder to `done/` with completion date prefix:
      `git mv plans/in-progress/planning-and-dev-practice plans/done/2026-05-25__planning-and-dev-practice`
- [x] Verify `plans/in-progress/README.md` — confirm no `planning-and-dev-practice` entry exists
      (none was added to the active-plans list, so no edit is expected)
- [x] Update `plans/done/README.md` — add an entry of this exact form (a link to the archived
      folder, description, and completion date):

  ```text
  - [Planning and Dev Practice Improvement](./2026-05-25__planning-and-dev-practice/) — grill-me skill, TDD delivery-checklist shape, harness-neutrality plan-checker Step 5g. Completed 2026-05-25.
  ```

- [x] Search for orphaned references to `plans/in-progress/planning-and-dev-practice`
      (`rg -n "in-progress/planning-and-dev-practice"`) and fix any found
- [x] Commit the archival: `rtk git commit -m "chore(plans): move planning-and-dev-practice to done"`
- [x] Push: `rtk git push origin HEAD:main`

Acceptance criterion: plan folder lives at `plans/done/2026-05-25__planning-and-dev-practice/`, both
plan READMEs are updated, no orphaned `in-progress/planning-and-dev-practice` references remain, and
the archival commit is pushed to `origin main`.

> **Implementation notes** (2026-05-25): `git mv` moved the folder to
> `plans/done/2026-05-25__planning-and-dev-practice/` (completion-date prefix). Verified
> `plans/in-progress/README.md` had no entry to remove (`grep` empty). Added the completion entry
> to the top of `plans/done/README.md` Completed Projects. Orphan-reference search
> (`rg -n "in-progress/planning-and-dev-practice"` excluding the moved folder) returned none.
> Archival committed and pushed to `origin main` in the final step. Files Changed: plan folder
> moved (5 files), `plans/done/README.md`.
