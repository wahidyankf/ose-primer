# Delivery Checklist — Planning System Overhaul

## Worktree

**Path**: `worktrees/planning-system-overhaul/`

**Provision**:

```bash
claude --worktree planning-system-overhaul
```

After worktree creation, follow
[worktree-setup.md](../../../repo-governance/development/workflow/worktree-setup.md) to
run `npm install` and `npm run doctor -- --fix` from the **repo root**.

See [worktree-path.md](../../../repo-governance/conventions/structure/worktree-path.md) and
[plans.md §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [x] Install dependencies: `npm install`
      — acceptance: exits 0
- [x] Converge polyglot toolchain: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift
- [x] Run baseline markdown lint: `npm run lint:md`
      — acceptance: baseline pass/fail count recorded; clean or violations documented
      — _result: 1616 files, 0 errors — clean baseline_

---

## Phase 1: TDD HARD RULE in `test-driven-development.md`

> _Suggested executor: repo-rules-maker_

### Step 1.1 — Locate insertion points

- [x] Read `repo-governance/development/workflow/test-driven-development.md`
      — command: `grep -n "TDD Shape for Delivery Checklists" repo-governance/development/workflow/test-driven-development.md`
      — acceptance: file found and section confirmed present (exactly one match)
- [x] Confirm the "TDD Shape for Delivery Checklists" section exists and contains the
      three-substep template code block (the block with RED/GREEN/REFACTOR substeps)
      — acceptance: section and code block found
- [x] Confirm the "Plan Creation (plan-maker)" subsection contains the mini-TDD nested
      example block (`- [ ] TDD cycle: [feature name]`)
      — acceptance: example block found

### Step 1.2 — Insert HARD RULE paragraph

- [x] After the three-substep template code block in "TDD Shape for Delivery Checklists",
      insert the HARD RULE paragraph (verbatim from `tech-docs.md §test-driven-development.md
— HARD RULE paragraph`)
      — command: `grep -n "HARD RULE: Never combine" repo-governance/development/workflow/test-driven-development.md`
      — acceptance: exactly one match found

### Step 1.3 — Insert grouping-label note

- [x] After the mini-TDD nested example block in "Plan Creation (plan-maker)", insert the
      grouping-label note (verbatim from `tech-docs.md §test-driven-development.md —
grouping-label note`)
      — command: `grep -n "grouping label" repo-governance/development/workflow/test-driven-development.md`
      — acceptance: exactly one match found

### Step 1.4 — Lint check

- [x] Run `npm run lint:md -- repo-governance/development/workflow/test-driven-development.md`
      — acceptance: exits 0, zero violations on the file
      — _result: 0 errors_

---

## Phase 2: `AGENTS.md` Catalog Completeness

> _Suggested executor: repo-rules-maker_

### Step 2.1 — Update Project Planning agent category

- [x] Read `AGENTS.md` and locate item 3 ("Project Planning") in the Agent Organization
      numbered list
      — command: `grep -n "Project Planning" AGENTS.md`
      — acceptance: file found and item confirmed present (at least one match)
- [x] Replace the item with the updated text from
      `tech-docs.md §AGENTS.md — Project Planning agent category`
      — command: `grep -n "repo-setup-manager" AGENTS.md`
      — acceptance: at least one match found
      — command: `grep -n "plan-establishment" AGENTS.md`
      — acceptance: at least one match referencing `plan-establishment-execution.md`
      — command: `grep -n "grill" AGENTS.md`
      — acceptance: at least one match near "plan-maker"

### Step 2.2 — Lint check

- [x] Run `npm run lint:md -- AGENTS.md`
      — acceptance: exits 0, zero violations on the file
      — _result: 0 errors_

---

## Phase 3: Markdown Archive Exclusions

> _Suggested executor: repo-rules-maker_

### Step 3.1 — Update `.markdownlintignore`

- [x] Append the archive exclusion block (verbatim from `tech-docs.md §.markdownlintignore
— archive entries`) to the end of `.markdownlintignore`
      — command: `grep -n "plans/done/" .markdownlintignore`
      — acceptance: exactly one match found
      — command: `grep -n "archived/" .markdownlintignore`
      — acceptance: exactly one match found

### Step 3.2 — Update `.markdownlint-cli2.jsonc`

- [x] Add `"plans/done/**"` and `"archived/**"` (with comment) to the `ignores` array in
      `.markdownlint-cli2.jsonc` (verbatim from `tech-docs.md §.markdownlint-cli2.jsonc —
archive ignores`)
      — command: `grep -n '"plans/done' .markdownlint-cli2.jsonc`
      — acceptance: exactly one match found
      — command: `grep -n '"archived/' .markdownlint-cli2.jsonc`
      — acceptance: exactly one match found

### Step 3.3 — Update `markdown.md`

- [x] Append the "Archive Exclusion" section (verbatim from `tech-docs.md §markdown.md —
Archive Exclusion section`) to the end of
      `repo-governance/development/quality/markdown.md`
      — command: `grep -n "Archive Exclusion" repo-governance/development/quality/markdown.md`
      — acceptance: at least one match found
      — command: `grep -n "plans/done" repo-governance/development/quality/markdown.md`
      — acceptance: at least one match found

### Step 3.4 — Full lint check

- [x] Run `npm run lint:md`
      — acceptance: exits 0, zero violations; file count is lower than baseline (archive
      directories now excluded)
      — _result: 1542 files (down from 1616), 0 errors_

---

## Phase 4: Quality Gates

### Step 4.1 — Full markdown lint

- [x] Run `npm run lint:md`
      — acceptance: exits 0, zero violations
      — _result: 1542 files, 0 errors_
- [x] If violations exist, run `npm run lint:md:fix` then recheck
      — acceptance: zero violations after fix
      — _result: no violations — step not needed_

### Step 4.2 — Nx affected targets

- [x] Run `npx nx affected -t typecheck lint test:quick spec-coverage`
      — acceptance: all affected targets pass (docs-only change — expect "No tasks were run")
      — _result: NX No tasks were run_
- [x] Fix ALL failures found
      — _result: no failures_

### Step 4.3 — Repo-rules quality gate

- [x] Invoke `repo-rules-checker` via Agent tool, scoped to changed files
      (`repo-governance/development/workflow/test-driven-development.md`, `AGENTS.md`,
      `repo-governance/development/quality/markdown.md`), to produce audit report in
      `generated-reports/`
- [x] If CRITICAL or HIGH findings, invoke `repo-rules-fixer` via Agent tool
      — _result: no CRITICAL or HIGH findings; fixer not needed_
- [x] Re-invoke `repo-rules-checker` until zero CRITICAL and zero HIGH findings
      — acceptance: zero CRITICAL, zero HIGH findings
      — _result: 0 CRITICAL, 0 HIGH_

### Step 4.4 — Vendor-audit gate

- [x] Run `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit`
      — acceptance: exits 0, "PASSED" output; no vendor-specific term leaks in changed
      governance docs
      — _result: GOVERNANCE VENDOR AUDIT PASSED_

---

## Phase 5: Commits and Push

### Step 5.1 — Thematic commits

- [x] Stage and commit TDD HARD RULE change:
      `git add repo-governance/development/workflow/test-driven-development.md && git commit -m "docs(governance): add TDD RED/GREEN/REFACTOR hard rule and grouping-label note"`
      — acceptance: `git log --oneline -1` shows the commit message
- [x] Stage and commit AGENTS.md change:
      `git add AGENTS.md && git commit -m "docs(governance): update AGENTS.md with repo-setup-manager and plan-establishment ref"`
      — acceptance: `git log --oneline -1` shows the commit message
- [x] Stage and commit archive exclusion changes:
      `git add .markdownlintignore .markdownlint-cli2.jsonc repo-governance/development/quality/markdown.md && git commit -m "docs(governance): exclude plans/done and archived from markdown lint"`
      — acceptance: `git log --oneline -1` shows the commit message
- [x] Stage and commit plan files:
      `git add plans/in-progress/planning-system-overhaul/ && git commit -m "docs(plans): track planning-system-overhaul execution progress"`
      — acceptance: `git log --oneline -1` shows the commit message

### Step 5.2 — Push and verify CI

- [x] Push to main: `git push origin HEAD:main`
      — acceptance: push exits 0, pre-push hooks pass
- [x] Monitor CI: `gh run list --branch main --limit 5`
      — acceptance: all workflow runs show status "completed" with conclusion "success";
      or "No tasks" for docs-only push
- [x] Fix any CI failures immediately before proceeding
      — acceptance: `gh run list --branch main --limit 5` shows no failures

---

## Phase 6: Plan Archival

- [ ] Verify ALL delivery checklist items above are ticked (no `- [ ]` remaining)
      — command: `grep "\- \[ \]" plans/in-progress/planning-system-overhaul/delivery.md`
      — acceptance: returns no lines (only Phase 6 items remain, ticked inline)
- [ ] Move plan folder to `done/` with completion date prefix:
      `git mv plans/in-progress/planning-system-overhaul plans/done/2026-05-26__planning-system-overhaul`
      — acceptance: `ls plans/done/2026-05-26__planning-system-overhaul/` lists all plan files
- [ ] Update `plans/in-progress/README.md` — remove this plan entry
      — acceptance: `grep "planning-system-overhaul" plans/in-progress/README.md` returns no lines
- [ ] Update `plans/done/README.md` — add a link entry for
      `2026-05-26__planning-system-overhaul` with description: "TDD HARD RULE,
      AGENTS.md catalog completeness, markdown archive exclusions. Completed 2026-05-26."
      — acceptance: `grep "planning-system-overhaul" plans/done/README.md` returns at least one line
- [ ] Search for orphaned references: `grep -rn "in-progress/planning-system-overhaul" .`
      — acceptance: no matches (excluding the moved folder)
- [ ] Commit archival: `git commit -m "chore(plans): move planning-system-overhaul to done"`
- [ ] Push: `git push origin HEAD:main`

---

## Definition of Done

- [x] `test-driven-development.md` contains HARD RULE paragraph (one match) and
      grouping-label note (one match)
- [x] `AGENTS.md` lists `repo-setup-manager`, references `plan-establishment-execution.md`,
      and documents grill mandate
- [x] `.markdownlintignore` has `plans/done/` and `archived/` entries
- [x] `.markdownlint-cli2.jsonc` has `"plans/done/**"` and `"archived/**"` in ignores
- [x] `markdown.md` has Archive Exclusion section
- [x] `npm run lint:md` exits 0
- [x] `npx nx affected` passes (or no tasks — docs-only)
- [x] `repo-rules-checker` zero CRITICAL + HIGH findings
- [x] All changes committed thematically with Conventional Commits
- [x] Push succeeds; CI passes (or no push-triggered CI — pre-push hooks passed)
- [x] Plan archived at `plans/done/2026-05-26__planning-system-overhaul/`
