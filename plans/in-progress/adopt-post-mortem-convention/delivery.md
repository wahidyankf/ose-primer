# Delivery Checklist — Adopt Post-Mortem Convention

**Execution markers**: `[AI]` = an agent performs the step (default). `[HUMAN]` = only a human can
perform it. No `[HUMAN]` steps exist in this plan — it is fully agent-executable.

## Worktree

Worktree path: `worktrees/adopt-post-mortem-convention/`

Provision before execution (run from repo root):

```bash
claude --worktree adopt-post-mortem-convention
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

> **Trunk-based note**: Per [AGENTS.md](../../../AGENTS.md), worktree work and direct-on-`main` work
> both push to `origin main`. This is a docs + governance-only change; direct-to-`main` execution is
> sanctioned. No PR is created (none requested).

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Install dependencies in the root worktree: `npm install` — acceptance: exits 0,
      `node_modules/` synchronized.
- [ ] [AI] Converge the toolchain in the root worktree: `npm run doctor -- --fix` — acceptance:
      exits 0 (doctor may report drift it then fixes); record any preexisting drift.
- [ ] [AI] Confirm the docs/governance baseline is clean: run `git status --porcelain` — acceptance:
      output shows only the plan folder `plans/in-progress/adopt-post-mortem-convention/`.
- [ ] [AI] Record the markdown-tooling baseline: run
      `npx prettier --check "repo-governance/**/*.md" "docs/**/*.md"` and note the result —
      acceptance: result recorded; any preexisting formatting failures documented (this plan does
      not introduce new ones).
- [ ] [AI] Resolve any preexisting failures encountered above before proceeding — acceptance: no
      preexisting failure tied to files this plan will touch remains unresolved.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm run doctor -- --fix` — exits 0.
- [ ] [AI] `git status --porcelain` — shows only the plan folder.

> **Pause Safety**: Toolchain converged and markdown baseline recorded; no convention or docs files
> created yet. Safe to stop indefinitely. To resume: `npm run doctor -- --fix`.

---

## Phase 1: Post-Mortem Convention Document

- [ ] [AI] Create `repo-governance/conventions/structure/post-mortems.md` _New file_, faithfully
      adopting the `ose-infra` convention with H1 title **"Post-Mortem Convention"**. YAML
      frontmatter: `title: "Post-Mortem Convention"`, `category: explanation`,
      `subcategory: conventions`, tags including `post-mortem`, `incidents`, `blameless`,
      `reliability`, `structure`. Body MUST contain, keeping structure identical to the source:
      (a) a **Principles Implemented/Respected** section tracing to Documentation First, Root Cause
      Orientation, Deliberate Problem-Solving, Explicit Over Implicit;
      (b) **Location and Naming** — location `docs/explanation/post-mortems/`, filename pattern
      `YYYY-MM-DD-<system>-<short-failure>.md`, flat directory, lowercase kebab-case;
      (c) a **Blameless Principle** section (the "second story" framing);
      (d) a **Mandatory Sections** subsection enumerating exactly the **14 sections in order** from
      `tech-docs.md §Decision 5` (Frontmatter, Metadata Table, Summary, Impact, Detection, Timeline,
      Root Cause, Trigger, Contributing Factors, Resolution & Mitigations, Action Items, What Went
      Well, Lessons Learned, References) plus an **Optional Sections** subsection (Background,
      Supporting Data);
      (e) a **Severity Scale (Authoritative)** table (Sev-1 Critical, Sev-2 Major, Sev-3 Moderate,
      Sev-4 Minor);
      (f) a **No Secrets Rule** section linking
      [No Secrets in Committed Files](../../../repo-governance/development/quality/no-secrets-in-committed-files.md);
      (g) a **Diagrams** section referencing the diagrams + color-accessibility conventions;
      (h) a `doc_status` lifecycle (`draft` → `reviewed` → `closed`). Generalize illustrative
      examples to the app/service domain (no Tailscale/Proxmox specifics). Cross-link to
      [Root Cause Orientation](../../../repo-governance/principles/general/root-cause-orientation.md),
      [Proactive Preexisting Error Resolution](../../../repo-governance/development/practice/proactive-preexisting-error-resolution.md),
      and [Diátaxis Framework](../../../repo-governance/conventions/structure/diataxis-framework.md).
      Acceptance: file exists; `grep -c "Blameless" <file>` ≥ 1; `grep -c "Severity Scale" <file>`
      ≥ 1; all 14 section names appear in the Mandatory Sections list (`grep -c "Action Items"
<file>` ≥ 1, `grep -c "Trigger" <file>` ≥ 1, `grep -c "Contributing Factors" <file>` ≥ 1 as
      spot checks).
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Add an alphabetical index entry for the new convention to
      `repo-governance/conventions/structure/README.md` under the "Documents" list, linking
      `./post-mortems.md` with a one-line description (place it alphabetically — between
      `plans.md` and `programming-language-docs-separation.md` style ordering by title
      "Post-Mortem Convention"). Acceptance:
      `grep -c "post-mortems.md" repo-governance/conventions/structure/README.md` ≥ 1.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Add an alphabetical entry to `repo-governance/conventions/README.md` under the
      "🗂️ Structure" section, linking `./structure/post-mortems.md` with a one-line description
      (alphabetical by title "Post-Mortem Convention"). Acceptance:
      `grep -c "structure/post-mortems.md" repo-governance/conventions/README.md` ≥ 1.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Verify every relative link inside `post-mortems.md` resolves: for each `](../...)` or
      `](./...)` target, run `Bash test -f` on the resolved path. Acceptance: every link target
      exists (zero `MISSING` results). Confirm specifically that the no-secrets link resolves to
      `repo-governance/development/quality/no-secrets-in-committed-files.md`.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `test -f repo-governance/conventions/structure/post-mortems.md` — exits 0.
- [ ] [AI] `npx prettier --check repo-governance/conventions/structure/post-mortems.md repo-governance/conventions/structure/README.md repo-governance/conventions/README.md`
      — reports no changes.
- [ ] [AI] `npx markdownlint-cli2 repo-governance/conventions/structure/post-mortems.md`
      — zero errors (if `markdownlint-cli2` is unavailable, use the repo's configured markdownlint
      invocation; acceptance is zero errors either way).
- [ ] [AI] All 14 mandatory section names present in the convention's Mandatory Sections list (the
      grep spot checks above return ≥ 1 for each).

> **Pause Safety**: The authoritative convention document exists, is indexed in both convention
> READMEs, and passes markdown gates; the template/index and sample do not yet exist. Safe to stop.
> To resume: `npx prettier --check repo-governance/conventions/structure/post-mortems.md`.

---

## Phase 2: Explanation-Tier Template and Index

- [ ] [AI] Create `docs/explanation/post-mortems/README.md` _New file + new dir_ with explanation
      frontmatter (`title: Post-Mortems`, `category: explanation`, `subcategory: post-mortem`, tags
      `index`, `explanation`, `post-mortems`, `incidents`, `reliability`). Body MUST: (a) explain
      what a post-mortem is (Diátaxis explanation tier); (b) point to the authoritative
      Post-Mortem Convention at `repo-governance/conventions/structure/post-mortems.md` (created in
      Phase 1) as the source of truth — render it as a relative markdown link in the produced file;
      (c) contain a single fenced ` ```markdown ` code block holding a
      **complete copy-paste template** with one heading per mandatory section (all 14 from
      `tech-docs.md §Decision 5`, in order), each with a short fill-in prompt comment, plus the two
      optional sections marked `(optional)`; (d) state filing conventions (filename pattern, flat
      layout, `doc_status` lifecycle, no-secrets, blameless tone); (e) carry an **Index** section
      linking the sample at `./2025-01-15-sample-be-service-db-pool-exhaustion.md`. Acceptance:
      file exists; the fenced template block contains a heading for each mandatory section
      (`grep -c "## Summary" <file>` ≥ 1, `grep -c "## Action Items" <file>` ≥ 1,
      `grep -c "## Trigger" <file>` ≥ 1, `grep -c "## References" <file>` ≥ 1 as spot checks).
  - _Suggested executor: `docs-maker`_
- [ ] [AI] Cross-check template ⇄ convention parity: confirm every one of the 14 mandatory section
      names listed under "Mandatory Sections" in
      `repo-governance/conventions/structure/post-mortems.md` also appears as a heading inside the
      template block of `docs/explanation/post-mortems/README.md`. Acceptance: for each of the 14
      section names, `grep` finds it in both files; zero mismatches.
- [ ] [AI] Add a Post-Mortems subdir entry to `docs/explanation/README.md` under the
      "📋 Documentation Index" (new subsection or under an appropriate heading), linking
      `./post-mortems/README.md` with a one-line description. Acceptance:
      `grep -c "post-mortems/README.md" docs/explanation/README.md` ≥ 1.
  - _Suggested executor: `docs-maker`_
- [ ] [AI] Verify every relative link inside `docs/explanation/post-mortems/README.md` resolves via
      `Bash test -f` on each resolved target. Acceptance: zero `MISSING` results. (Note: the sample
      link target is created in Phase 3; if running this check before Phase 3, exempt that one
      target and re-verify it at the Phase 3 gate.)

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `test -f docs/explanation/post-mortems/README.md` — exits 0.
- [ ] [AI] `npx prettier --check docs/explanation/post-mortems/README.md docs/explanation/README.md`
      — reports no changes.
- [ ] [AI] Template/convention section parity holds — all 14 mandatory section names present in the
      template block (the parity grep above returns zero mismatches).

> **Pause Safety**: The explanation-tier template and index exist and are wired into
> `docs/explanation/README.md`; only the sample post-mortem (and its inbound link verification)
> remains. Safe to stop. To resume:
> `npx prettier --check docs/explanation/post-mortems/README.md`.

---

## Phase 3: Illustrative Sample Post-Mortem

- [ ] [AI] Create
      `docs/explanation/post-mortems/2025-01-15-sample-be-service-db-pool-exhaustion.md` _New file_
      with explanation frontmatter (`title: "Post-Mortem: sample-be-service — DB Connection Pool
Exhaustion"`, `category: explanation`, `subcategory: post-mortem`, `doc_status: closed`, tags
      `post-mortem`, `example`, `sample`). The Summary MUST open with an explicit banner stating
      this is a **fabricated illustrative example, not a real incident**. Use the placeholder
      service name `sample-be-service` (verified non-existent under `apps/`). Scenario: the service
      exhausted its database connection pool under load on 2025-01-15, classified `Sev-3 — Moderate`.
      Populate **all 14** mandatory sections from `tech-docs.md §Decision 5` in order with
      realistic-but-fictional content; the Action Items table must use the
      `# | Action | Owner | Priority | Ticket | Status` columns with P0/P1/P2 priorities and a
      `plans/` reference or `—` placeholder per row. Acceptance: file exists;
      `grep -ci "not a real incident" <file>` ≥ 1; `grep -c "sample-be-service" <file>` ≥ 1;
      `grep -c "Sev-3" <file>` ≥ 1; each mandatory section heading present
      (`grep -c "## Root Cause" <file>` ≥ 1, `grep -c "## Trigger" <file>` ≥ 1,
      `grep -c "## Lessons Learned" <file>` ≥ 1 as spot checks).
  - _Suggested executor: `docs-maker`_
- [ ] [AI] Confirm the sample is linked from the index in
      `docs/explanation/post-mortems/README.md` (added in Phase 2). Acceptance:
      `grep -c "2025-01-15-sample-be-service-db-pool-exhaustion.md" docs/explanation/post-mortems/README.md`
      ≥ 1 and the linked path resolves (`Bash test -f` on the target).
- [ ] [AI] Re-verify the sample link in `docs/explanation/post-mortems/README.md` now resolves:
      `test -f docs/explanation/post-mortems/2025-01-15-sample-be-service-db-pool-exhaustion.md` —
      exits 0.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `test -f docs/explanation/post-mortems/2025-01-15-sample-be-service-db-pool-exhaustion.md`
      — exits 0.
- [ ] [AI] `npx prettier --check docs/explanation/post-mortems/2025-01-15-sample-be-service-db-pool-exhaustion.md`
      — reports no changes.
- [ ] [AI] `grep -ci "not a real incident" docs/explanation/post-mortems/2025-01-15-sample-be-service-db-pool-exhaustion.md`
      ≥ 1 — the illustrative banner is present.
- [ ] [AI] All 14 mandatory section headings present in the sample (the section grep spot checks
      above return ≥ 1 for each).

> **Pause Safety**: All three documents exist, are indexed, and pass markdown gates; only reciprocal
> back-links from existing governance remain. Safe to stop. To resume:
> `test -f docs/explanation/post-mortems/2025-01-15-sample-be-service-db-pool-exhaustion.md`.

---

## Phase 4: Reciprocal Cross-Links

- [ ] [AI] Edit `repo-governance/principles/general/root-cause-orientation.md`: add a reference to
      the new convention in a "Related" / "See also" location (preserve existing structure; do not
      remove content). Acceptance:
      `grep -c "post-mortems" repo-governance/principles/general/root-cause-orientation.md` ≥ 1.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Edit `repo-governance/development/practice/proactive-preexisting-error-resolution.md`: add
      a reference to the new post-mortem convention (preserve existing content). Acceptance:
      `grep -c "post-mortems" repo-governance/development/practice/proactive-preexisting-error-resolution.md`
      ≥ 1.
  - _Suggested executor: `repo-rules-maker`_

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] Both back-links present (the two `grep -c ... ≥ 1` checks above pass).
- [ ] [AI] `npx prettier --check repo-governance/principles/general/root-cause-orientation.md repo-governance/development/practice/proactive-preexisting-error-resolution.md`
      — reports no changes.

> **Pause Safety**: All documents and bidirectional links are in place. Safe to stop. To resume:
> run the governance quality gate (Phase 5).

---

## Phase 5: Governance Quality Gate (repo-rules-quality-gate, strict)

> _Run by the orchestrator (calling context) — this is a workflow invocation, not a single-agent
> step. The orchestrator drives
> [repo-rules-quality-gate](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md) and
> delegates finding fixes to `repo-rules-fixer` as that workflow prescribes._

All three new documents and the reciprocal cross-links now exist (Phases 1–4). Before committing
and pushing, validate the new convention plus every index and back-link edit for repo-wide
governance consistency by running the governance quality-gate workflow at `strict` mode.

- [ ] [AI] Run the
      [repo-rules-quality-gate](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md)
      workflow at `strict` mode over the changed governance + docs surface (the new
      `repo-governance/conventions/structure/post-mortems.md`, both convention READMEs,
      `docs/explanation/**`, and the two edited principle/practice files). Acceptance:
      repo-rules-quality-gate returns `pass` — zero CRITICAL/HIGH/MEDIUM findings on two
      consecutive validations (LOW findings may exist and are not blocking).
- [ ] [AI] Fix ALL CRITICAL/HIGH/MEDIUM findings the gate surfaces — delegate the fixes to
      `repo-rules-fixer` (as the gate workflow does), then re-run the gate until it returns `pass`.
      Acceptance: no CRITICAL/HIGH/MEDIUM finding remains; the gate's two-consecutive-clean check
      is satisfied.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `repo-governance/workflows/repo/repo-rules-quality-gate.md` (strict) returns `pass` —
      zero CRITICAL/HIGH/MEDIUM findings on two consecutive validations.
- [ ] [AI] All CRITICAL/HIGH/MEDIUM findings surfaced by the gate are fixed (via `repo-rules-fixer`)
      and confirmed resolved on re-run.

> **Pause Safety**: The new convention and all index/back-link edits are validated repo-wide and the
> governance gate is green; nothing is committed or pushed yet. Safe to stop. To resume:
> `npx nx affected -t typecheck lint test:quick spec-coverage` (Phase 6).

---

## Phase 6: Quality Gates, Commit, and Push

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected quality gates: `npx nx affected -t typecheck lint test:quick spec-coverage` — exits 0 with no failures.
- [ ] [AI] Run repo markdown check on touched trees:
      `npx prettier --check "repo-governance/**/*.md" "docs/**/*.md"` — reports no changes.
- [ ] [AI] Run `npm run lint:md` — exits 0, zero errors reported across all markdown files including the three new ones.
- [ ] [AI] Verify every relative cross-link across all new/edited files resolves: run a `Bash` loop
      executing `test -f` on each resolved link target. Acceptance: zero `MISSING` results.
- [ ] [AI] Fix ALL failures found — including preexisting issues not caused by these changes — then
      re-run the failing checks to confirm resolution.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.
> This follows the root cause orientation principle — proactively fix preexisting errors encountered
> during work. Commit preexisting fixes separately with appropriate conventional commit messages.

### Commit Guidelines

- [ ] [AI] Commit changes thematically — group related changes into logically cohesive commits.
- [ ] [AI] Follow Conventional Commits format: `<type>(<scope>): <description>` (e.g.,
      `docs(governance): adopt blameless post-mortem convention`).
- [ ] [AI] Split distinct concerns into separate commits (convention doc vs template/sample vs
      cross-link wiring) where it improves history clarity.
- [ ] [AI] Preexisting fixes get their own commits, separate from plan work.
- [ ] [AI] Do NOT bundle unrelated changes into a single commit.

### Post-Push CI Verification

- [ ] [AI] Push changes to `main`: `git push origin HEAD:main`.
- [ ] [AI] Check which runs the push triggered: `gh run list --limit 5` — note the workflow names and
      statuses. For a docs-only push to `main`, no push-triggered workflows are defined in this repo
      (all `test-crud-*.yml` workflows are `schedule`/`workflow_dispatch` only; `pr-quality-gate.yml`
      and `pr-validate-links.yml` are PR-only and will NOT trigger on a direct push). Expect zero
      new runs triggered; if any unexpected run appears, monitor it to green.
- [ ] [AI] Verify ALL triggered CI checks pass — no exceptions.
- [ ] [AI] If any CI check fails, fix immediately and push a follow-up commit.
- [ ] [AI] Repeat until ALL triggered GitHub Actions pass with zero failures.
- [ ] [AI] Do NOT proceed to archival until CI is fully green (or confirmed no workflows triggered).

### Phase 6 Gate

> All checks below must pass before archival.

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — exits 0.
- [ ] [AI] `npx prettier --check "repo-governance/**/*.md" "docs/**/*.md"` — reports no changes.
- [ ] [AI] `npm run lint:md` — exits 0 with zero errors reported.
- [ ] [AI] All GitHub Actions workflows for the push are green.

> **Pause Safety**: Work is committed and pushed; CI is green. Safe to stop. To resume: proceed to
> Plan Archival.

---

## Plan Archival

- [ ] [AI] Verify ALL delivery checklist items above are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI).
- [ ] [AI] Rename and move:
      `git mv plans/in-progress/adopt-post-mortem-convention/ plans/done/2026-06-05__adopt-post-mortem-convention/`
      (use today's date as the completion date).
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date 2026-06-05.
- [ ] [AI] Confirm no other READMEs reference this plan by name (`plans/README.md` is a landing-page
      overview that does not list individual plans — no update needed there).
- [ ] [AI] Commit the archival: `chore(plans): move adopt-post-mortem-convention to done`.
- [ ] [AI] Push the archival commit and confirm CI is green.
