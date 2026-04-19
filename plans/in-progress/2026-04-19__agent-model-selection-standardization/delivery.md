# Delivery Checklist

## Prerequisites

- [x] Update `plans/in-progress/README.md` — add this plan entry (required before execution;
    removed at archival step 9.5d)
<!-- 2026-04-19 | Status: Done | Files: plans/in-progress/README.md | Already completed by plan-fixer during quality gate (F1 fix) -->
- [x] Run `npm install` to install workspace dependencies
<!-- 2026-04-19 | Status: Done | Completed: npm install ran, audit warnings only (not errors) -->
- [x] Run `npm run doctor -- --fix` to converge the Go and Node toolchains (required
    before any `validate:*` or `sync:*` script can invoke the rhino-cli binary)
<!-- 2026-04-19 | Status: Done | 19/19 tools OK, 0 missing -->
- [x] Confirm running on `main` branch (no worktree — governance-only changes, no code)
<!-- 2026-04-19 | Status: Done | Branch: main confirmed -->
- [x] Confirm `npm run validate:claude` passes on clean branch (baseline)
<!-- 2026-04-19 | Status: Done | 719/719 checks passed -->
- [x] Confirm `npm run validate:sync` passes on clean branch (baseline)
<!-- 2026-04-19 | Status: Done | Fixed pre-existing drift (plan-checker, plan-maker, skill), now 79/79 passed. Committed separately as fix(sync) -->
- [x] Confirm `nx run rhino-cli:test:quick` passes on clean branch (baseline)
<!-- 2026-04-19 | Status: Done | 90.02% >= 90% threshold, all packages pass -->

### Commit Guidelines

- Commit changes thematically — group related changes into logically cohesive commits
- Follow Conventional Commits format: `<type>(<scope>): <description>`
- Split different domains/concerns into separate commits
- Do NOT bundle unrelated fixes into a single commit
- Example: separate `docs(model-selection): ...` from `fix(agents): ...` commits

---

## Phase 1: Update `governance/development/agents/model-selection.md`

- [x] **1.1** Budget-Adaptive Inheritance block added to Opus tier section
    — explains omit = session inherit, account-tier table (Max/Team → Opus 4.7,
    Pro/Standard → Sonnet 4.6), warning not to add `model: opus`
<!-- 2026-04-19 | Status: Done | File: governance/development/agents/model-selection.md | Added Budget-Adaptive Inheritance subsection with account-tier table and warning -->
- [x] **1.2** "Current Model Versions (April 2026)" table added with Opus 4.7, Sonnet
    4.6, Haiku 4.5-20251001; Haiku 3 retirement note (2026-04-19)
<!-- 2026-04-19 | Status: Done | File: governance/development/agents/model-selection.md | Added "Current Model Versions (April 2026)" section with table and Haiku 3 retirement note -->
- [x] **1.3** "OpenCode / GLM Equivalents" section added with mapping table + 3-to-2
    collapse explanation
<!-- 2026-04-19 | Status: Done | File: governance/development/agents/model-selection.md | Added "OpenCode / GLM Equivalents" section with mapping table, collapse explanation, GLM-5-turbo warning -->
- [x] **1.4** "Adding `model: opus` to opus-tier agents" row added to Common Mistakes
<!-- 2026-04-19 | Status: Done | File: governance/development/agents/model-selection.md | Added row to Common Mistakes table -->
- [x] **1.5** "Last Updated" set to 2026-04-19
<!-- 2026-04-19 | Status: Done | File: governance/development/agents/model-selection.md | Updated Last Updated from 2026-04-12 to 2026-04-19 -->
- [x] **1.6** Run `npm run lint:md` — confirm zero errors
<!-- 2026-04-19 | Status: Done | 742 files linted, 0 errors -->
- [ ] **1.7** Commit: `docs(model-selection): add budget-adaptive inheritance note + opencode glm mapping`

---

## Phase 2: CLAUDE.md

- [ ] **2.1** Plans Organization section updated — add inline 5-doc format description
      (README.md, brd.md, prd.md, tech-docs.md, delivery.md + single-README collapse rule).
      See `tech-docs.md § Change 2` for exact text.
- [ ] **2.2** Format Differences models row updated — add `opus` alias, add budget-adaptive
      note, add link to model-selection.md. See `tech-docs.md § Change 2` for exact diff.
- [ ] **2.3** Run `npm run lint:md` — confirm zero errors
- [ ] **2.4** Commit: `docs(CLAUDE.md): add plan format description + update model format differences for opus alias`

---

## Phase 3: Propagate to Related Governance Docs

- [ ] **3.1** `governance/development/agents/ai-agents.md` — add budget-adaptive note to
      model field spec and Model Selection Guidelines: opus-tier agents omit model by
      design; warn against adding `model: opus`
- [ ] **3.2** `governance/development/agents/best-practices.md` — add budget-adaptive
      rationale; ensure plan-maker example shows omit (not `model: opus`)
- [ ] **3.3** `.claude/agents/README.md` — add "Opus-tier agents omit `model` by design"
      note to the model selection section
- [ ] **3.4** Run `npm run lint:md` — confirm zero errors
- [ ] **3.5** Commit: `docs(agents): propagate budget-adaptive model inheritance note`

---

## Phase 4: Create Model Benchmark Reference Document

Create `docs/reference/ai-model-benchmarks.md` — the project's canonical benchmark
reference. All subsequent files that cite benchmark numbers link to this document. This
document links to primary sources. Structure and all benchmark data are fully specified in
`tech-docs.md` under "Model Benchmark Data" and "Benchmark Reference Document Specification".

- [ ] **4.1** Create `docs/reference/ai-model-benchmarks.md` following the spec in
      `tech-docs.md § Benchmark Reference Document Specification`
- [ ] **4.2** Verify every benchmark number has: source URL, publication date, confidence
      level (`[Verified]` / `[Self-reported]` / `[Needs Verification]`)
- [ ] **4.3** Verify the GLM-5-turbo section prominently flags that no standard benchmarks
      are published for this model
- [ ] **4.4** Verify the model capability summary table is present
- [ ] **4.5** Add `ai-model-benchmarks.md` entry to `docs/reference/README.md`
- [ ] **4.6** Run `npm run lint:md` — confirm zero errors on new file and updated README
- [ ] **4.7** Commit: `docs(reference): add ai-model-benchmarks reference with cited scores`

---

## Phase 5: Agent Tier Audit — Right-Size 1 Agent

Apply the definitive tier mapping from `tech-docs.md § Complete Agent Tier Mapping`.
**Do not re-reason the mapping** — the table is the authoritative decision.

**1 agent changes tier**:

| Agent              | Change      | Reason                                                      |
| ------------------ | ----------- | ----------------------------------------------------------- |
| `repo-rules-maker` | OMIT→SONNET | Layer hierarchy templates drive output, not open creativity |

**Result**: opus-inherit 15→14 (−1), sonnet 28→29 (+1), haiku 2→2, total 45 unchanged.

- [ ] **5.1** For `repo-rules-maker.md`:
  - Update `model:` frontmatter from empty to `sonnet`
  - Update Model Selection Justification block text to reference sonnet tier and cite
    the benchmark comparison from `docs/reference/ai-model-benchmarks.md` where relevant
  - Do NOT change color, tools, or any other frontmatter field
- [ ] **5.2** Spot-check 5 random unchanged agents to confirm their Model Selection
      Justification blocks are present and consistent with their tier
- [ ] **5.3** Run `npm run validate:claude` — expect zero errors
- [ ] **5.4** Commit: `fix(agents): right-size repo-rules-maker tier OMIT→SONNET`

---

## Phase 6: Propagate Benchmark Citations via repo-rules-maker

Add benchmark citations (with links to `docs/reference/ai-model-benchmarks.md`) to all
policy docs that make tier-based claims.

- [ ] **6.1** Invoke `repo-rules-maker` to update `governance/development/agents/model-selection.md`:
  - In the Tier Comparison Summary table, add benchmark score column citing the reference
    doc
  - In "Current Model Versions", add inline links to the reference doc for each score
  - In the OpenCode / GLM Equivalents section, add caveat about GLM-5-turbo having no
    standard benchmarks (link to reference doc)
  - **Verify**: `model-selection.md` Tier Comparison Summary table contains a benchmark
    score column with links to `docs/reference/ai-model-benchmarks.md`; "Current Model
    Versions" rows each have an inline link; OpenCode/GLM section has the GLM-5-turbo
    no-benchmarks caveat
- [ ] **6.2** Invoke `repo-rules-maker` to update `.claude/agents/README.md`:
  - Add a "Model Benchmark Context" note (2-3 lines) pointing to the reference doc for
    anyone who wants to understand WHY each tier was chosen
  - **Verify**: `.claude/agents/README.md` contains a "Model Benchmark Context" paragraph
    with a link to `docs/reference/ai-model-benchmarks.md`
- [ ] **6.3** Verify every benchmark number cited in `model-selection.md` has a link to
      `docs/reference/ai-model-benchmarks.md` with the anchor for the relevant model
- [ ] **6.4** Run `npm run lint:md` — confirm zero errors
- [ ] **6.5** Commit: `docs(governance): add benchmark citations to model-selection + agents README`

---

## Phase 7: repo-rules-checker OCD Validation

Run `repo-rules-checker` in OCD mode after all changes (Phases 1-6).

- [ ] **7.1** Invoke `repo-rules-checker` with `strictness: ocd`
- [ ] **7.2** Review findings — fix all CRITICAL, HIGH, and MEDIUM findings
- [ ] **7.3** Re-run until two consecutive zero-finding passes (per quality gate workflow)
- [ ] **7.4** Commit any fixes: `fix(governance): repo-rules-checker ocd findings`

---

## Phase 8: Validation

> **Purpose**: Validate agent-format compliance and Claude↔OpenCode sync consistency.
> This is distinct from Phase 9's Nx quality gates which validate code compilation,
> linting, and test coverage. Both phases serve different concerns and are intentionally
> separate.

- [ ] **8.1** Run `npm run validate:claude` — expect zero errors
- [ ] **8.2** Run `npm run validate:sync` — expect zero errors
- [ ] **8.3** Run `nx run rhino-cli:test:quick` — expect pass

---

## Phase 9: Sync + Final Gate

- [ ] **9.1** Run `npm run sync:dry-run` — `repo-rules-maker` should show OpenCode model
      unchanged (both omit and sonnet map to `glm-5.1`; dry-run confirms no regressions
      in other agents)
- [ ] **9.2** Run `npm run sync:claude-to-opencode` — apply sync
- [ ] **9.3** Run `npm run validate:sync` — final pass

### Local Quality Gates (Before Push)

- [ ] Run `npm run lint:md` — lint all markdown files
- [ ] Run `npm run lint:md:fix` — auto-fix any violations
- [ ] Run `nx affected -t typecheck lint test:quick spec-coverage` — the agent file change
      triggers rhino-cli as affected; all targets must pass
- [ ] Fix ALL failures found — including preexisting issues not caused by your changes

> **Important**: Fix ALL failures, not just those caused by your changes. Root cause
> orientation: proactively fix preexisting errors encountered during work.

- [ ] **9.4** Push directly to `origin/main`

### Post-Push Verification

- [ ] Confirm `git log --oneline -5` shows all expected commits on `origin/main`
- [ ] Note: ose-primer GitHub Actions workflows are PR-only (`on: pull_request` /
      `on: workflow_dispatch`). No workflows fire on direct push to `main`. Post-push
      CI monitoring does not apply to direct-to-main commits.
- [ ] Local quality gates (Phase 8 `validate:claude` / `validate:sync` and Phase 9
      `nx affected` targets) are the authoritative gate for this plan.

### Plan Archival

- [ ] **9.5a** Verify ALL delivery checklist items are ticked
- [ ] **9.5b** Verify ALL quality gates pass (local + CI)
- [ ] **9.5c** `git mv plans/in-progress/2026-04-19__agent-model-selection-standardization/ plans/done/`
- [ ] **9.5d** Update `plans/in-progress/README.md` — remove this plan entry
- [ ] **9.5e** Update `plans/done/README.md` — add this plan entry with completion date
- [ ] **9.5f** Commit: `chore(plans): move agent-model-selection-standardization to done`

---

## Acceptance Criteria Checklist

> **Note**: This checklist is the executor's tracking tool — check each item off as work
> completes. The `prd.md` Gherkin scenarios (`Feature: Budget-Adaptive …` etc.) are the
> formal testable specification; both documents describe the same end state and are
> complementary, not competing.

- [ ] `model-selection.md` Opus tier section explicitly documents omit-as-budget-adaptive
- [ ] `model-selection.md` Common Mistakes includes "Adding `model: opus` to opus-tier agents"
- [ ] `model-selection.md` has "OpenCode / GLM Equivalents" section
- [ ] `model-selection.md` has "Current Model Versions (April 2026)" table
- [ ] `CLAUDE.md` Plans Organization section includes inline 5-doc format description
- [ ] `CLAUDE.md` Format Differences models row includes `opus` alias
- [ ] Related governance docs propagated with budget-adaptive note
- [ ] `docs/reference/ai-model-benchmarks.md` exists with cited scores for all 5 models
- [ ] Every benchmark number in the reference doc has source URL + date + confidence level
- [ ] GLM-5-turbo section notes no standard benchmarks exist for this model
- [ ] `repo-rules-maker` has updated frontmatter (`model: sonnet`) + updated Justification
- [ ] `model-selection.md` benchmark claims link to `docs/reference/ai-model-benchmarks.md`
- [ ] `.claude/agents/README.md` has "Model Benchmark Context" pointer to reference doc
- [ ] `repo-rules-checker` OCD passes with zero findings
- [ ] `npm run validate:claude` passes
- [ ] `npm run validate:sync` passes
- [ ] Opus-inherit count = 14 (down from 15)
- [ ] All 14 opus-inherit tier agents retain empty `model:` field (no explicit `model: opus`
      anywhere); `repo-rules-maker` has `model: sonnet`
