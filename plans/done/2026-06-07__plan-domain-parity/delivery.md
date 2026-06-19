# Delivery Checklist — Plan Domain Parity (ose-primer)

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Worktree path: `worktrees/plan-domain-parity/` (already provisioned on branch
`plan-domain-parity` [Repo-grounded]). Push target: **ose-primer `origin main`**
(invoker-approved deviation from the PR-only primer sync default — matrix row 22; see
[README Deviation Notice](./README.md#deviation-notice)).

Provision before execution (run from repo root):

```bash
claude --worktree plan-domain-parity
```

Provision manually if absent (fallback, per the row-3 mechanics):

```bash
git worktree add -b plan-domain-parity worktrees/plan-domain-parity main
cd worktrees/plan-domain-parity && npm install && npm run doctor -- --fix
```

Delivery push (from the worktree, after all gates are green):

```bash
git push origin HEAD:main
```

See the [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [x] [AI] Install dependencies in the worktree
      (`/Users/wkf/ose-projects/ose-primer/worktrees/plan-domain-parity/`): `npm install`
      — acceptance: exits 0, `node_modules/` synchronized.
  - _Implementation notes (2026-06-06)_: Status DONE. Exit 0; node_modules present.
- [x] [AI] Converge the toolchain: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift (Rust, Go, Node, Docker all green).
  - _Implementation notes (2026-06-06)_: Status DONE. Initial run: 18/19 with a python
    warning (3.13.1 < required 3.13.12; pyenv definitions lag — no 3.13.12 recipe).
    Resolved via the already-installed uv CPython 3.13.12 build symlinked into
    `~/.pyenv/versions/3.13.12` + `pyenv global 3.13.12` (equivalent end-state to the
    doctor fixer's own `pyenv install <req> && pyenv global <req>` path). Re-run:
    19/19 tools OK, 0 warnings. Files changed: none in repo (machine toolchain only).
- [x] [AI] Verify sibling clones exist (read-only merge sources):
      `test -d /Users/wkf/ose-projects/ose-public && test -d /Users/wkf/ose-projects/ose-infra`
      — acceptance: both exist.
  - _Implementation notes (2026-06-06)_: Status DONE. Both exist; ose-public main is at
    the post-parity-execution state (plan archived, canon live).
- [x] [AI] Establish the code baseline:
      `npx nx run-many -t typecheck,lint,test:quick,spec-coverage -p rhino-cli-rust,rhino-cli-go`
      — acceptance: baseline pass/fail recorded; all preexisting failures documented.
  - _Implementation notes (2026-06-06)_: Status DONE. Exit 0 — all four targets green
    for both CLIs. Zero preexisting failures.
- [x] [AI] Establish the markdown/bindings baseline: `npm run lint:md && npm run validate:config`
      — acceptance: exit codes recorded; preexisting failures documented.
  - _Implementation notes (2026-06-06)_: Status DONE. Both exit 0. Zero preexisting
    failures.
- [x] [AI] Resolve all preexisting failures before proceeding (root-cause orientation;
      commit preexisting fixes separately) — acceptance: zero unresolved baseline failures.
  - _Implementation notes (2026-06-06)_: Status DONE. Only environment-level drift found
    (python version, resolved in the doctor item above; machine-level, no repo commit
    needed). All repo baselines green.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [x] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
  - _Implementation notes (2026-06-06)_: Status PASS — 19/19 tools OK, 0 warnings (after
    python 3.13.12 convergence).
- [x] [AI] `npx nx run-many -t typecheck,lint,test:quick,spec-coverage -p rhino-cli-rust,rhino-cli-go`
      exits 0 (or every preexisting failure resolved and committed).
  - _Implementation notes (2026-06-06)_: Status PASS — exit 0.

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no
> plan work exists yet beyond the plan documents themselves. Safe to stop indefinitely.
> To resume: re-run the baseline command and confirm it is still clean.

## Phase 1: Adopt Merged Governance Canon (Rows 3–16)

> **Important**: Fix ALL failures found during quality gates, not just those caused by
> your changes. Commit preexisting fixes separately with conventional commit messages.

- [x] [AI] **Sequencing gate (hard)**: refresh the upstream clone
      (`git -C /Users/wkf/ose-projects/ose-public pull --ff-only`) and verify the
      ose-public plan-domain-parity merge has landed: confirm
      `/Users/wkf/ose-projects/ose-public/repo-governance/workflows/plan/plan-establishment-execution.md`
      contains the worktree-default mechanics (`git worktree add -b <identifier>`) AND
      `grep -rn "permission" /Users/wkf/ose-projects/ose-public/apps/rhino-cli/src/internal/agents/converter.rs`
      returns matches — acceptance: both checks pass. **STOP and surface to the human if
      not** (the upstream plan must execute first; this is the documented execution-order
      dependency).
  - _Implementation notes (2026-06-06)_: Status DONE. Upstream pulled (up-to-date);
    worktree-default mechanics grep = 1 hit; converter.rs permission grep = 25 hits.
    Upstream plan executed, archived, CI green — sequencing dependency satisfied.
- [x] [AI] Merge `repo-governance/workflows/plan/plan-establishment-execution.md` from
      the upstream canon (semantic merge, preserve primer link targets) — acceptance:
      primer copy contains the `target-stage` input AND the full worktree-default
      mechanics (provision `worktrees/<identifier>/` via
      `git worktree add -b <identifier> worktrees/<identifier> main` + `npm install` +
      `npm run doctor -- --fix`; commit in worktree; push HEAD to confirmed push-target,
      default `origin main`; remove worktree after delivery);
      `grep -c "target-stage" <file>` ≥ 1.
  - _Suggested executor: `repo-rules-maker`_
  - _Implementation notes (2026-06-06)_: Status DONE (executor: repo-rules-maker).
    Canon adopted: target-stage input + Stage Resolution section + worktree-default
    Execution Mode block + Step 7 worktree push/removal mechanics + grill-format
    references to Grilling-With-Options Convention (file lands in Phase 2 — link kept
    verbatim, validated at Phase 2 gate). target-stage grep = 15; worktree-add grep = 1;
    prettier + markdownlint 0 errors. Files changed:
    repo-governance/workflows/plan/plan-establishment-execution.md.
- [x] [AI] Merge `repo-governance/workflows/plan/plan-execution.md` — acceptance: merged
      canon adopted; primer-specific agent-selection lists preserved (diff against the
      pre-merge copy shows no primer-only agent name removed).
  - _Suggested executor: `repo-rules-maker`_
  - _Implementation notes (2026-06-06)_: Status DONE (executor: repo-rules-maker).
    Adopted: [HUMAN]-gate Iron Rule 2 title+semantics, clearer executor-tag loop step,
    user-wording consistency, grill blockquote callout. Already present (primer was a
    merge input): extension/framework lists, 2b step 0, phase-gate stopping rule. Kept
    primer-specific: crud-be-fsharp-giraffe example, primer plans.md anchor names (to be
    reconciled in the plans.md merge below). 0 public app leakage; prettier +
    markdownlint 0 errors. Files changed:
    repo-governance/workflows/plan/plan-execution.md.
- [x] [AI] Merge `repo-governance/workflows/meta/execution-modes.md` — acceptance:
      matches the merged canon; primer link targets intact.
  - _Implementation notes (2026-06-06)_: Status DONE (direct adoption — diff showed
    zero primer-specific content; 40 changed lines were canon improvements:
    vendor-neutral "delegated agent" terminology, created field, richer 5-branch
    decision tree, vendor-neutral binding-dir phrasing). File now byte-identical to
    canon; prettier + markdownlint 0 errors. Files changed:
    repo-governance/workflows/meta/execution-modes.md.
- [x] [AI] Merge `.claude/agents/plan-maker.md`, `.claude/agents/plan-checker.md`,
      `.claude/agents/plan-fixer.md`, `.claude/agents/plan-execution-checker.md`
      (one commit-reviewable edit per file; preserve primer repo refs such as
      `rhino-cli-rust` naming and primer app examples) — acceptance: each file matches
      the merged canon modulo enumerated divergences;
      `grep -L "rhino-cli\b" .claude/agents/plan-*.md` shows no upstream-only
      `apps/rhino-cli` paths leaked into primer files.
  - _Implementation notes (2026-06-06)_: Status DONE (executor: agent-maker). Adopted
    per file: plan-maker (grilling description + AskUserQuestion-first grills, executor
    legend section 0, Phase-0/Phase-Gate templates); plan-checker (TDD phase-separation
    - non-code-format bullets, Step 14/15 split + grandfathering); plan-fixer (7-fix
      executor/phase-gate structure); plan-execution-checker (phase-gate completion
      bullet). All plans.md anchor links updated to canon names (resolve after the
      plans.md merge below). Kept primer-specific crud-be-\* examples (8 occurrences) and
      models. Zero upstream path leakage; prettier + markdownlint 0 errors. Files
      changed: .claude/agents/plan-{maker,checker,fixer,execution-checker}.md.
- [x] [AI] Reconcile `.claude/agents/repo-setup-manager.md` (row 11): diff the 3-line
      primer divergence against the canon; keep lines that are repo-specific
      (`rhino-cli-rust` naming), merge the rest — acceptance: divergence decision noted
      inline in the commit message.
  - _Implementation notes (2026-06-06)_: Status DONE (direct verification, no edit).
    The 3-line divergence is primer's house frontmatter style (bracketless `tools:` +
    explicit `skills: []`), consistent across ALL primer agents (verified against
    plan-maker, docs-maker) — repo-specific formatting kept; body content identical to
    canon. Divergence decision recorded here and in the Phase 1 commit message. Files
    changed: none.
- [x] [AI] Merge `.claude/skills/plan-creating-project-plans/SKILL.md` including infra's
      mandatory grilling gates (row 12) — acceptance: pre-write AND post-write grilling
      gate sections present; primer path refs intact.
  - _Implementation notes (2026-06-06)_: Status DONE (executor: agent-maker). Adopted:
    grilling-gates description + Mandatory Pre/Post-Write Grilling section, executor
    legend + Phase-Gate sections under canon heading names, lifecycle gate callouts,
    No-Secrets/Grilling/TDD references, grill-me related-skill. Kept primer crud-be-\*
    examples and primer no-secrets link target. post-write grep = 8; leak grep = 0;
    prettier + markdownlint 0 errors. Files changed:
    .claude/skills/plan-creating-project-plans/SKILL.md.
- [x] [AI] Merge `.claude/skills/plan-writing-gherkin-criteria/SKILL.md` and
      `.claude/skills/grill-me/SKILL.md` (rows 13–14, trivial drift) — acceptance: match
      merged canon.
  - _Implementation notes (2026-06-06)_: Status DONE (direct edits). Gherkin skill:
    Phase-Gate section link text+anchor aligned to canon heading; primer's
    `.opencode/skills/` example line kept (repo-specific variant, recorded in the
    upstream merge as deliberately excluded there). grill-me: canon adopted wholesale
    (canon already carries primer's improvements; richer 6-rule set +
    AskUserQuestion-MUST mechanism; grilling-with-options link resolves after Phase 2).
    prettier + markdownlint 0 errors. Files changed: both SKILL.md files.
- [x] [AI] Merge `repo-governance/conventions/structure/plans.md` (row 16) — acceptance:
      matches merged canon modulo primer-specific examples; internal anchors used by
      primer agents/workflows still resolve
      (`npx nx run rhino-cli-rust:validate:mermaid` and the link validator pass in the
      Phase 1 gate below).
  - _Implementation notes (2026-06-06)_: Status DONE (executor: repo-rules-maker +
    orchestrator follow-up). Canon adopted: no-secrets overview + best-practice
    sections, fuller executor-tagging and phase-gate sections under CANON heading names
    (primer headings renamed; repo-wide anchor sweep updated plan-execution.md,
    workflows/plan/README.md, conventions/structure/README.md — .opencode mirrors
    regenerate next item). Canon's ../security/ links rewritten to primer's
    no-secrets-in-committed-files.md / env-file-access.md paths. Follow-up: because the
    Phase-2 grilling-with-options.md was pulled forward (created now from canon —
    pre-commit link validation would otherwise block the Phase 1 commit), the grilling
    step (Creating Plans step 5) and the Related-Conventions grilling entry were
    restored to match canon. 4 anchor-critical headings present; old-anchor grep = 0;
    leak grep = 0; prettier + markdownlint 0 errors. Files changed: plans.md,
    plan-execution.md, two READMEs, grilling-with-options.md (new, Phase 2 item pulled
    forward).
- [x] [AI] Regenerate secondary bindings for the changed agent files with the CURRENT
      script: `npm run generate:bindings` — acceptance: exits 0; `.opencode/agents/`
      mirrors of the four plan agents updated.
  - _Implementation notes (2026-06-06)_: Status DONE. Exit 0; exactly the four plan
    agent mirrors modified; stale old-anchor occurrences in mirrors now 0. Files
    changed: .opencode/agents/plan-{maker,checker,fixer,execution-checker}.md.
- [x] [AI] Commit thematically (Conventional Commits; separate commits for workflows,
      agents, skills, convention; e.g. `docs(workflows): adopt merged plan-establishment
canon with target-stage and worktree default`) — acceptance: `git log` shows
      domain-split commits, no unrelated bundling.
  - _Implementation notes (2026-06-06)_: Status DONE (deviation noted). Commit 1 landed
    as planned (`docs(workflows): adopt merged plan-establishment canon…`, 4 files).
    The remaining four themes landed as ONE combined
    `docs(governance): adopt plan-domain canon for agents, skills, and conventions`
    commit (14 files) — pre-commit's stash/restore cycle plus two staged-link blockers
    made replaying the five-way split unsafe; the two blockers were root-cause fixed
    first (canon no-secrets path → primer's no-secrets-in-committed-files.md;
    plan-maker trunk-based anchor → primer's #main-branch-vs-worktree-mode) and the
    commit body itemizes each theme. No unrelated bundling: all 14 files are
    plan-domain canon adoption.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [x] [AI] `grep -n "target-stage" repo-governance/workflows/plan/plan-establishment-execution.md`
      returns ≥ 1 match.
  - _Implementation notes (2026-06-06)_: Status PASS — 15 matches.
- [x] [AI] `npm run lint:md` exits 0 and `npm run format:md:check` exits 0.
  - _Implementation notes (2026-06-06)_: Status PASS. lint:md 0 on first run;
    format:md:check initially failed on a PREEXISTING drifted file
    (plans/done/2026-06-04\_\_adopt-dependency-bump-policy/delivery.md) — fixed with
    prettier --write and committed separately
    (`style(plans): fix preexisting prettier drift…`); re-check exit 0.
- [x] [AI] `npm run validate:sync` exits 0 (mirrors consistent after regeneration).
  - _Implementation notes (2026-06-06)_: Status PASS — exit 0.
- [x] [AI] `npx nx affected -t typecheck lint test:quick` exits 0 (affected projects after
      agent file merges and `npm run generate:bindings` regeneration).
  - _Implementation notes (2026-06-06)_: Status PASS — exit 0.
- [x] [AI] `git status` clean (everything committed).
  - _Implementation notes (2026-06-06)_: Status PASS — clean except this plan's own
    progress notes (committed at delivery).

> **Pause Safety**: the merged governance canon is fully adopted and committed in the
> worktree; no code or script changes yet; bindings are consistent. Safe to stop. To
> resume: `npm run validate:sync && npm run lint:md`.

## Phase 2: New Governance Files and Indexes (Rows 1, 2, 5, 15)

- [x] [AI] Create `repo-governance/workflows/plan/plan-multi-repo-parity-planning.md`
      (_New file_) from the upstream amended copy at
      `/Users/wkf/ose-projects/ose-public/repo-governance/workflows/plan/plan-multi-repo-parity-planning.md`,
      adapting only repo-local link targets — acceptance: file exists; step sequence is
      Survey → Matrix → First Grill (hard gate) → web-researcher (conditional) →
      Second Grill (post-research) → Author → Gate → Deliver; workflow-naming validator
      passes (`npx nx run rhino-cli-rust:validate:naming-workflows` exits 0).
  - _Suggested executor: `repo-rules-maker`_
  - _Implementation notes (2026-06-06)_: Status DONE (executor: repo-rules-maker). File
    created from the upstream 8-step canon; 8 Step headings verified; link adaptations:
    no-secrets-in-git → primer's no-secrets-in-committed-files (×2); ose-primer-sync
    convention link → prose (convention absent in primer; bullet removed from
    Conventions list). naming-workflows exit 0; prettier + markdownlint 0 errors.
    Files changed: repo-governance/workflows/plan/plan-multi-repo-parity-planning.md
    (new).
- [x] [AI] Create `repo-governance/development/workflow/grilling-with-options.md`
      (_New file_) from the upstream merged convention (public `grilling-with-options.md`
      merged with infra's broader-scope `grilling.md`), adapting repo-local links —
      acceptance: file exists; the multi-options HARD RULE (2–4 options, one recommended,
      one question at a time) and infra's broader scope are both present.
  - _Suggested executor: `repo-rules-maker`_
  - _Implementation notes (2026-06-06)_: Status DONE — pulled forward into Phase 1
    (commit `docs(governance): adopt plan-domain canon…`) so Phase 1 canon links
    resolved at commit time. File exists; all link targets verified in primer;
    2-4-options hard rule present (7 mentions); broader infra scope included via the
    canon. Files changed: repo-governance/development/workflow/grilling-with-options.md
    (new, already committed).
- [x] [AI] Update `repo-governance/workflows/plan/README.md` (row 5) — acceptance: indexes
      exactly 4 workflows (establishment, execution, quality-gate, multi-repo parity) and
      the Grilling Format section links the new convention file.
  - _Implementation notes (2026-06-06)_: Status DONE (direct edit). Parity workflow
    entry added with the two-grill+research description; Grilling Format section now
    cites the Grilling-With-Options Convention with grill-me as canonical
    implementation. 4 workflow links (grep = 4); prettier + markdownlint 0 errors.
    Files changed: repo-governance/workflows/plan/README.md.
- [x] [AI] Update `repo-governance/workflows/README.md` — acceptance: parity workflow
      listed in the catalog table with description and participating agents.
  - _Implementation notes (2026-06-06)_: Status DONE (direct edit). Catalog row added
    with the two-grill+research description; agents column lists plan-maker,
    web-researcher, plan-checker, plan-fixer. Files changed:
    repo-governance/workflows/README.md.
- [x] [AI] Update `repo-governance/development/workflow/README.md` — acceptance:
      `grilling-with-options.md` indexed.
  - _Implementation notes (2026-06-06)_: Status DONE (direct edit). Convention indexed
    in the Documents list (alphabetical position after commit-messages). Files changed:
    repo-governance/development/workflow/README.md.
- [x] [AI] Update `AGENTS.md`: plan-maker catalog wording (grilling-with-options
      reference) and workflow references — acceptance: `grep -n "grilling-with-options" AGENTS.md`
      ≥ 1; no stale reference to a nonexistent convention remains.
  - _Implementation notes (2026-06-06)_: Status DONE (direct edit). plan-maker catalog
    entry now cites the Grilling-With-Options Convention (grep = 1); no stale
    convention references. prettier + markdownlint 0 errors. Files changed: AGENTS.md.
- [x] [AI] Commit thematically — acceptance: separate commits for new workflow, new
      convention, index/catalog updates.
  - _Implementation notes (2026-06-06)_: Status DONE. Two commits this phase
    (`docs(workflows): add plan-multi-repo-parity-planning workflow`, 620 lines;
    `docs(governance): index parity workflow and grilling convention`, 4 files);
    the new-convention commit landed in Phase 1 (pulled forward). Status clean except
    plan notes.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [x] [AI] `test -f repo-governance/workflows/plan/plan-multi-repo-parity-planning.md && test -f repo-governance/development/workflow/grilling-with-options.md` exits 0.
  - _Implementation notes (2026-06-06)_: Status PASS — both exist.
- [x] [AI] `npx nx run rhino-cli-rust:validate:naming-workflows` exits 0.
  - _Implementation notes (2026-06-06)_: Status PASS — exit 0.
- [x] [AI] `npm run lint:md` exits 0; link validation on the touched files passes
      (`npx nx run rhino-cli-rust:validate:mermaid` for any new diagrams; repo link
      validator per pre-commit hook).
  - _Implementation notes (2026-06-06)_: Status PASS. lint:md exit 0; staged-only link
    validation "All links valid" (pre-commit hook also passed at both Phase 2 commits).
    New files contain NO mermaid diagrams; the repo-wide validate:mermaid run reports
    only PREEXISTING violations in docs/ (testing-ai-apps, elixir-phoenix, fe-nextjs,
    fe-react, jvm-spring\* etc.) — those are owned by the active
    markdown-gate-coverage-expansion plan (its fix-all phases are mid-execution);
    fixing them here would collide with that plan's checklist, so they are explicitly
    deferred to their owning plan rather than silently skipped.
- [x] [AI] `git status` clean.
  - _Implementation notes (2026-06-06)_: Status PASS — clean except plan notes.

> **Pause Safety**: governance surface is complete (canon + new files + indexes); CLIs
> and scripts untouched. Safe to stop. To resume: `npm run lint:md && git status`.

## Phase 3: Rust Emitter Modernization (Rows 18–19)

> _Suggested executor: `swe-rust-dev`_

### Row 18 — OpenCode `permission` Object (TDD)

- [x] [AI] **RED**: read the landed upstream implementation
      (`/Users/wkf/ose-projects/ose-public/apps/rhino-cli/src/internal/agents/converter.rs`)
      to fix the exact permission-object shape, then add a failing unit test
      (_New test_, e.g. `convert_permission_maps_granted_tools_to_allow`) in the tests
      module of `apps/rhino-cli-rust/src/internal/agents/converter.rs` asserting the
      converter output frontmatter contains a `permission` object (granted tool →
      `allow`, matching the upstream shape) and NO boolean `tools` map. Run
      `npx nx run rhino-cli-rust:test:unit` — acceptance: the new test FAILS, all others
      pass.
  - _Implementation notes (2026-06-06)_: Status DONE (executor: swe-rust-dev). Test
    convert_permission_maps_granted_tools_to_allow added (runtime-failing, suite stays
    compilable): panics "expected a permission: block in frontmatter" — 580 passed /
    1 failed. RED proven. Files changed:
    apps/rhino-cli-rust/src/internal/agents/converter.rs (test only).
- [x] [AI] **GREEN**: implement the change in
      `apps/rhino-cli-rust/src/internal/agents/converter.rs` (replace `convert_tools`
      boolean map emission) and `apps/rhino-cli-rust/src/internal/agents/types.rs`
      (struct field), updating the serializer emission order accordingly. Run
      `npx nx run rhino-cli-rust:test:unit` — acceptance: the new test PASSES, zero
      regressions.
  - _Implementation notes (2026-06-06)_: Status DONE (executor: swe-rust-dev).
    convert_permission added (convert_tools removed; tests renamed/retyped);
    OpenCodeAgent.permission in types.rs (field position 3 preserved);
    emit_opencode_yaml emits permission block / `{}`; sync_validator + sync.rs +
    tests/agents.rs fixtures switched to permission: allow shape. 581/581 unit tests;
    fmt + clippy clean. Validator failure label deliberately stays "Tools mismatch"
    (primer-internal consistency with go side until Phase 4 port; cosmetic divergence
    from upstream's "Permission mismatch" recorded). Files changed:
    apps/rhino-cli-rust/src/internal/agents/{types,converter,sync_validator,sync}.rs,
    apps/rhino-cli-rust/tests/agents.rs.
- [x] [AI] **REFACTOR**: update
      `apps/rhino-cli-rust/src/internal/agents/sync_validator.rs` (and its tests) to
      validate the new frontmatter shape; remove dead boolean-map code; run
      `npx nx run rhino-cli-rust:test:unit && npx nx run rhino-cli-rust:lint` —
      acceptance: both exit 0; no `convert_tools` boolean emission remains
      (`grep -n "tools" apps/rhino-cli-rust/src/internal/agents/converter.rs` shows only
      parsing of Claude-side `tools` input).
  - _Implementation notes (2026-06-06)_: Status DONE. sync_validator shape migration
    landed in GREEN; REFACTOR softened the stale "Byte-for-byte port" module docs in
    converter.rs + sync_validator.rs (now note the permission extension pending the Go
    parity port). test:unit 581/581; lint 0; remaining `tools` greps in converter.rs
    are Claude-side input parsing only. Files changed: the two module doc headers.
- [x] [AI] Regenerate all OpenCode mirrors: `npm run generate:bindings` — acceptance:
      every `.opencode/agents/*.md` frontmatter contains `permission` and no boolean
      `tools` map (`grep -L "permission" .opencode/agents/*.md` returns only README-type
      files, if any); `npm run validate:sync` exits 0.
  - _Implementation notes (2026-06-06)_: Status DONE. Regen exit 0; grep -L permission
    = no files; `^tools:` straggler count = 0; validate:sync exit 0. Files changed:
    .opencode/agents/\*.md (regenerated).

### Row 19 — Codex `config.toml` Migration

- [x] [AI] Migrate the `ci-monitor-subagent` configuration: inline the contents of
      `.codex/agents/ci-monitor-subagent.toml` into the existing
      `[agents.ci-monitor-subagent]` sub-table in `.codex/config.toml` per the official
      Codex config reference (where a key cannot be inlined, relocate the per-agent TOML
      to `.codex/ci-monitor-subagent.toml` and update `config_file` to the new
      non-`agents/` path) — acceptance: `.codex/config.toml` parses
      (`python3 -c "import tomllib;tomllib.load(open('.codex/config.toml','rb'))"` exits 0) and carries the agent config.
  - _Implementation notes (2026-06-06)_: Status DONE (relocation branch, matching the
    upstream D4 decision: official `agents.<name>` keys are config_file/description/
    nickname_candidates only — developer_instructions not inlinable). git mv to
    .codex/ci-monitor-subagent.toml; config_file pointer updated; tomllib parse OK;
    moved file byte-identical. Files changed: .codex/config.toml,
    .codex/ci-monitor-subagent.toml (moved).
- [x] [AI] Delete the unofficial directory: `git rm -r .codex/agents/` — acceptance:
      `test ! -d .codex/agents` exits 0.
  - _Implementation notes (2026-06-06)_: Status DONE. git mv removed the only tracked
    file; empty dir rmdir'd; `test ! -d` exits 0.
- [x] [AI] Sweep code and tests for stale references:
      `grep -rn "\.codex/agents" apps/ .claude/ .opencode/ repo-governance/ docs/ AGENTS.md CLAUDE.md package.json`
      — acceptance: zero matches outside `plans/` history; any rhino-cli code/test
      reference found is updated under its existing test suite
      (`npx nx run rhino-cli-rust:test:unit` stays green).
  - _Implementation notes (2026-06-06)_: Status DONE. Sweep found 3 hits, all in
    docs/reference/platform-bindings.md: the live-surface claim (line ~87) rewritten to
    record the removal + config.toml sub-table mechanism; the Cursor/Junie rows kept
    (vendor-capability facts about what those third-party tools scan, mirroring the
    upstream decision). No rhino-cli code/test references; test:unit stays green
    (581/581). Files changed: docs/reference/platform-bindings.md.
- [x] [AI] Update binding docs for rows 18–19: `docs/reference/platform-bindings.md`
      (lines 63, 65, 87 — `.codex/agents/` references and the Codex layout note),
      `repo-governance/conventions/structure/multi-harness-binding.md` (codex layout),
      `AGENTS.md` lines 48/86/215 wording (boolean flags → permission object), and
      `CLAUDE.md` line 51 — acceptance: no doc claims boolean `tools` flags or
      `.codex/agents/` as the current format; `npm run lint:md` exits 0.
  - _Suggested executor: `docs-maker`_
  - _Implementation notes (2026-06-06)_: Status DONE (direct edits — bounded wording
    swaps). platform-bindings.md Codex note rewritten (removal + sub-table mechanism;
    Cursor/Junie vendor rows kept as vendor facts); AGENTS.md line ~87 conversion
    wording + OpenCode binding example switched to permission object with deprecation
    note; CLAUDE.md Tools bullet rewritten to match upstream canon. Post-sweep grep:
    zero non-deprecated boolean-flags/`read: true` claims. lint:md exit 0. Files
    changed: docs/reference/platform-bindings.md, AGENTS.md, CLAUDE.md.
- [x] [AI] Commit thematically — acceptance: separate commits for the Rust emitter
      change (`feat(rhino-cli-rust): emit opencode permission object`), the mirror
      regeneration, the Codex migration, and the doc updates.
  - _Implementation notes (2026-06-06)_: Status DONE. Four clean thematic commits
    (rewritten once locally after a git-mv pre-staging bleed): rust emitter (5 files),
    codex consolidation (rename + pointer), mirror regen (50 files), binding docs
    (3 files). Status clean except plan notes.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [x] [AI] `npx nx run rhino-cli-rust:test:unit && npx nx run rhino-cli-rust:test:quick && npx nx run rhino-cli-rust:lint && npx nx run rhino-cli-rust:typecheck` all exit 0.
  - _Implementation notes (2026-06-06)_: Status PASS — all four exit 0.
- [x] [AI] `npm run validate:sync && npm run validate:harness-bindings` exit 0.
  - _Implementation notes (2026-06-06)_: Status PASS — both exit 0.
- [x] [AI] `test ! -d .codex/agents` exits 0 and the grep sweep above returns zero live references.
  - _Implementation notes (2026-06-06)_: Status PASS — dir absent; sweep (excluding
    plans/ history and docs vendor-fact rows recorded above) = 0 live references.
- [x] [AI] `git status` clean.
  - _Implementation notes (2026-06-06)_: Status PASS — clean except plan notes.

> **Pause Safety**: Rust emitter and the binding surface are modernized and internally
> consistent; Go CLI still emits the old shape but is unreleased here and parity is
> checked only at its own gate. Safe to stop. To resume:
> `npm run validate:sync && npx nx run rhino-cli-rust:test:quick`.

## Phase 4: Go Port and Script Alignment (Rows 20–21)

> _Suggested executor: `swe-golang-dev`_

### Row 21 — Port Emitter Changes to rhino-cli-go (TDD)

Survey correction (tech-docs §Survey Corrections): `rhino-cli-go` already ships
`agents sync` + `agents emit-bindings` [Repo-grounded]; this phase ports the row-18/19
changes so capability parity holds.

- [x] [AI] **RED**: add a failing unit test (_New test_, e.g.
      `TestConvertPermission_MapsGrantedToolsToAllow`) in
      `apps/rhino-cli-go/internal/agents/converter_test.go` asserting the Go converter
      emits the same `permission` object shape as the Rust implementation (use a fixture
      mirroring the Rust test). Run `npx nx run rhino-cli-go:test:unit` — acceptance:
      the new test FAILS, all others pass.
  - _Implementation notes (2026-06-06)_: Status DONE (executor: swe-golang-dev).
    TestConvertPermission_MapsGrantedToolsToAllow added (runtime-failing, 5 assertion
    failures incl. "expected a permission: block"); all other packages pass. RED
    proven. Files changed: apps/rhino-cli-go/internal/agents/converter_test.go.
- [x] [AI] **GREEN**: implement in `apps/rhino-cli-go/internal/agents/converter.go`
      (replace `ConvertTools` boolean-map emission) and
      `apps/rhino-cli-go/internal/agents/types.go`. Run
      `npx nx run rhino-cli-go:test:unit` — acceptance: new test PASSES, zero
      regressions.
  - _Implementation notes (2026-06-06)_: Status DONE (executor: swe-golang-dev).
    ConvertPermission replaces ConvertTools; OpenCodeAgent.Permission
    map[string]string (yaml:"permission", field order preserved); sync_validator
    permissionsMatch (failure label "Tools mismatch" kept verbatim — matches the Rust
    side's deliberate choice); all fixtures (sync_validator_test, types_test, two cmd
    integration tests), doc comments, and README description updated. unit + lint
    exit 0; targeted godog integration scenarios pre-verified (9 passed). Files
    changed: converter.go, types.go, sync_validator.go + 5 test files +
    cmd/agents_sync.go + apps/rhino-cli-go/README.md.
- [x] [AI] **REFACTOR**: update `apps/rhino-cli-go/internal/agents/sync_validator.go`
      (and tests) for the new shape; remove dead code; verify `.codex` handling in
      `apps/rhino-cli-go/internal/agents/bindings.go` needs no change (the `.codex` dir
      entry at line 61 remains valid). Run
      `npx nx run rhino-cli-go:test:unit && npx nx run rhino-cli-go:lint` — acceptance:
      both exit 0.
  - _Implementation notes (2026-06-06)_: Status DONE. sync_validator shape migration +
    dead-code removal landed in GREEN; bindings.go `.codex` dir entry (line 61)
    verified valid unchanged (it tracks the binding DIR `.codex`, not `.codex/agents`).
    unit + lint exit 0. Files changed: none beyond GREEN.
- [x] [AI] Run the Go integration suite: `npx nx run rhino-cli-go:test:integration` —
      acceptance: exits 0.
  - _Implementation notes (2026-06-06)_: Status DONE — exit 0.

### Row 20 — `generate:bindings` Direct-Cargo Invocation

- [x] [AI] Edit `package.json` (scripts at lines 44–47 and the validate family): switch
      `generate:bindings`, `sync:agents`, `sync:skills`, `sync:dry-run`, `validate:sync`,
      `validate:claude`, and `validate:harness-bindings` from the
      `nx run rhino-cli-rust:build … ./apps/rhino-cli-rust/dist/rhino-cli` pattern to the
      direct-cargo pattern used by ose-public [Repo-grounded — tech-docs §Survey
      Corrections], substituting the primer manifest, e.g.:
      `cargo run --release --quiet --manifest-path apps/rhino-cli-rust/Cargo.toml -- agents sync && cargo run --release --quiet --manifest-path apps/rhino-cli-rust/Cargo.toml -- agents emit-bindings`
      — acceptance: `npm run generate:bindings` exits 0; the Go CLI is NOT referenced by
      any of these scripts (row 21: Rust stays canonical).
  - _Implementation notes (2026-06-06)_: Status DONE (direct edit). All seven scripts
    (generate:bindings, sync:agents/skills/dry-run, validate:sync/claude/
    harness-bindings) switched to
    `cargo run --release --quiet --manifest-path apps/rhino-cli-rust/Cargo.toml --`.
    generate:bindings exit 0; only remaining rhino-cli-go ref in package.json is the
    unrelated dev docker-compose script. Files changed: package.json.
- [x] [AI] Determinism check: run `npm run generate:bindings` a second time —
      acceptance: `git status --short` is empty afterward.
  - _Implementation notes (2026-06-06)_: Status DONE. Second run exit 0 and touched
    ZERO regen-managed files (.opencode/.amazonq unchanged) — deterministic. The only
    dirty paths at check time were this phase's not-yet-committed Go sources +
    package.json (committed in the next item), not regen output.
- [x] [AI] Update any doc that quotes the old script form (check `CONTRIBUTING.md`,
      `AGENTS.md`, `docs/reference/platform-bindings.md`,
      `apps/rhino-cli-rust/README.md`, `apps/rhino-cli-go/README.md`:
      `grep -rn "dist/rhino-cli agents" *.md docs/ apps/*/README.md`) — acceptance: zero
      stale quotes of the nx-build+dist invocation for the binding scripts.
  - _Implementation notes (2026-06-06)_: Status DONE. Repo-wide grep (root \*.md, docs/,
    both CLI READMEs, repo-governance/) returns zero stale quotes — no doc edits
    needed. Files changed: none.
- [x] [AI] Commit thematically — acceptance: separate commits for the Go port
      (`feat(rhino-cli-go): emit opencode permission object`) and the script switch
      (`build: switch binding scripts to direct cargo run`).
  - _Implementation notes (2026-06-06)_: Status DONE. Two commits: Go port (10 files,
    +138/-81) and script switch (package.json, 7 scripts). Status clean except plan
    notes.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [x] [AI] `npx nx run-many -t typecheck,lint,test:quick -p rhino-cli-rust,rhino-cli-go` exits 0.
  - _Implementation notes (2026-06-06)_: Status PASS — exit 0.
- [x] [AI] `npx nx run-many -t spec-coverage -p rhino-cli-rust,rhino-cli-go` exits 0 (Gherkin spec parity between both CLIs confirmed after Go port).
  - _Implementation notes (2026-06-06)_: Status PASS — exit 0.
- [x] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity && npx nx run rhino-cli-go:validate:cross-vendor-parity` both exit 0 (dual-CLI parity guard green).
  - _Implementation notes (2026-06-06)_: Status PASS — both exit 0.
- [x] [AI] `npm run generate:bindings` exits 0 twice consecutively with clean `git status` after the second run.
  - _Implementation notes (2026-06-06)_: Status PASS — both runs exit 0; tree clean
    afterwards.
- [x] [AI] `git status` clean.
  - _Implementation notes (2026-06-06)_: Status PASS — clean except plan notes.

> **Pause Safety**: both CLIs emit the modern formats, the parity guard is green, and the
> script family is aligned. Safe to stop. To resume:
> `npx nx run rhino-cli-rust:validate:cross-vendor-parity`.

## Phase 5: Full Repo-Wide Binding Audit (Row 17)

> _Suggested executor: `repo-harness-compatibility-checker` (via the harness
> compatibility quality-gate workflow)_

- [x] [AI] Enumerate the agent surface: list the 50 `.claude/agents/*.md` definitions
      (excluding `README.md`) and the 50 `.opencode/agents/*.md` mirrors; verify
      post-regeneration parity (50:50 match expected; no gap was present at plan authoring
      [Repo-grounded — 2026-06-06]) — acceptance: every `.claude/agents/*.md` definition
      has a corresponding `.opencode/agents/*.md` mirror, or its absence is documented as
      an intentional exclusion (with the excluding rule cited) in the audit record under
      `generated-reports/`.
  - _Implementation notes (2026-06-06)_: Status DONE. 50:50, comm -3 = 0 differences;
    audit record written to
    generated-reports/plan-domain-parity**binding-audit**2026-06-06--23-59\_\_audit.md.
- [x] [AI] Verify `.amazonq` bridge artifacts byte-exact:
      `npm run validate:harness-bindings` — acceptance: exits 0
      (`.amazonq/rules/00-agents-md.md`, `.amazonq/cli-agents/ose-default.json` match
      emitter expectations).
  - _Implementation notes (2026-06-06)_: Status DONE — exit 0.
- [x] [AI] Verify `.codex` conforms post-migration: `.codex/config.toml` parses and no
      `.codex/agents/` exists (re-run the Phase 3 checks) — acceptance: both pass.
  - _Implementation notes (2026-06-06)_: Status DONE — tomllib parse OK; dir absent.
- [x] [AI] Run the full config validation chain: `npm run validate:config` (validate:claude
      → generate:bindings → validate:opencode) — acceptance: exits 0.
  - _Implementation notes (2026-06-06)_: Status DONE — exit 0.
- [x] [AI] Run both repo-governance vendor audits:
      `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit && npx nx run rhino-cli-go:validate:repo-governance-vendor-audit`
      — acceptance: both exit 0 (no vendor-specific leakage introduced into
      `repo-governance/` by the Phase 1–2 merges).
  - _Implementation notes (2026-06-06)_: Status DONE — both exit 0.
- [x] [AI] Commit any audit-driven fixes thematically — acceptance: `git status` clean.
  - _Implementation notes (2026-06-06)_: Status DONE. Zero audit findings — no fixes
    to commit. Status clean except plan notes + the (gitignored-or-committed-later)
    audit record.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [x] [AI] `npm run validate:config` exits 0.
  - _Implementation notes (2026-06-06)_: Status PASS — exit 0.
- [x] [AI] `npx nx run-many -t validate:cross-vendor-parity,validate:repo-governance-vendor-audit -p rhino-cli-rust,rhino-cli-go` exits 0.
  - _Implementation notes (2026-06-06)_: Status PASS — both targets green for both
    CLIs (run individually; equivalent set).
- [x] [AI] Audit record written to `generated-reports/` documenting the mirror-gap reconciliation.
  - _Implementation notes (2026-06-06)_: Status PASS —
    generated-reports/plan-domain-parity**binding-audit**2026-06-06--23-59\_\_audit.md
    (50:50, zero gap; the no-gap finding documented).

> **Pause Safety**: the entire binding surface (all agents × .opencode/.amazonq/.codex)
> is audited and green. Safe to stop. To resume: `npm run validate:config`.

## Phase 6: Supersede planning-system-overhaul (Row 23)

- [x] [AI] Re-inventory `plans/in-progress/planning-system-overhaul/delivery.md` for
      unchecked items: `grep -n "^- \[ \]" plans/in-progress/planning-system-overhaul/delivery.md`
      — acceptance: confirmed that only archival items remain (lines 216–232 as of
      2026-06-06 [Repo-grounded]); if any NEW substantive unchecked item appears, absorb
      it into this checklist before proceeding and record the absorption in the commit
      message.
  - _Implementation notes (2026-06-07)_: Status DONE. 7 unchecked items, all archival
    (lines 216–232 confirmed). KEY DISCOVERY: an authoritative archive ALREADY exists
    at plans/done/2026-05-26\_\_planning-system-overhaul (status Completed; done README
    entry present; its tech-docs carries the newer wording) — the in-progress folder
    was a stale duplicate left by a copy-instead-of-move archival. Nothing substantive
    to absorb.
- [x] [AI] Add a supersession pointer to
      `plans/in-progress/planning-system-overhaul/README.md`: a `## Superseded` section
      stating the plan is closed by `plans/in-progress/plan-domain-parity/` (matrix
      row 23) with a relative link — acceptance: section present; `npm run lint:md`
      passes on the file.
  - _Implementation notes (2026-06-07)_: Status DONE (adapted). The target file was a
    stale duplicate slated for deletion — adding a section to a file being removed
    would be pointless; the supersession/closure rationale is recorded in the cleanup
    commit body and here instead.
- [x] [AI] Archive with completion-date prefix (today's date at execution time):
      `git mv plans/in-progress/planning-system-overhaul "plans/done/$(date +%F)__planning-system-overhaul"`
      — acceptance: folder exists under `plans/done/` with the date prefix; nothing
      remains under `plans/in-progress/planning-system-overhaul/`.
  - _Implementation notes (2026-06-07)_: Status DONE (adapted). A dated archive
    already existed (2026-05-26 prefix); a second dated copy would duplicate the
    archive, so the stale in-progress folder was `git rm`'d instead. Acceptance state
    holds: the plan lives under plans/done/ with a date prefix and nothing remains
    under plans/in-progress/planning-system-overhaul/.
- [x] [AI] Update `plans/in-progress/README.md` (remove the entry) and
      `plans/done/README.md` (add the entry with completion date and supersession note)
      — acceptance: both lists accurate.
  - _Implementation notes (2026-06-07)_: Status DONE. In-progress entry removed (and
    its plans.md anchor drift fixed: #-plan-folder-naming → #plan-folder-naming);
    done README already carried the 2026-05-26 entry — both lists accurate.
- [x] [AI] Sweep orphaned references:
      `grep -rn "in-progress/planning-system-overhaul" . --include="*.md" | grep -v plans/done`
      — acceptance: zero live references.
  - _Implementation notes (2026-06-07)_: Status DONE — zero live references (this
    plan's own historical notes excluded).
- [x] [AI] Commit: `chore(plans): archive planning-system-overhaul as superseded by plan-domain-parity`
      — acceptance: single archival commit.
  - _Implementation notes (2026-06-07)_: Status DONE. Single commit
    `chore(plans): remove stale planning-system-overhaul duplicate superseded by
plan-domain-parity` (7 files, +334/−863) — message adapted to the
    duplicate-cleanup reality.

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [x] [AI] `test ! -d plans/in-progress/planning-system-overhaul` exits 0.
  - _Implementation notes (2026-06-07)_: Status PASS — absent.
- [x] [AI] The orphan-reference sweep returns zero live references; `npm run lint:md` exits 0.
  - _Implementation notes (2026-06-07)_: Status PASS — sweep 0; lint:md exit 0.

> **Pause Safety**: exactly one in-progress plan owns the planning-system concern; the
> old plan is archived with a pointer. Safe to stop. To resume: re-run the orphan sweep.

## Phase 7: Rationale Doc (Rows 22, 24)

> _Suggested executor: `docs-maker`_

- [x] [AI] Create `docs/explanation/plan-domain-parity-decisions.md` (_New file_,
      Diátaxis explanation type, sibling reference: `docs/explanation/README.md` index
      style) explaining EVERY matrix decision (all 26 rows with justifications, sourced
      from [tech-docs.md](../../../plans/in-progress/plan-domain-parity/tech-docs.md) —
      use the path relative to the new doc), and documenting ESPECIALLY: this plan
      reached primer via direct push to `origin main` from worktree
      `worktrees/plan-domain-parity/` — an invoker-approved, recorded deviation from the
      PR-only default for mutations reaching ose-primer (Safety Invariant 6 of the
      plan-multi-repo-parity-planning workflow; matrix row 22) — acceptance: file exists;
      all 26 rows covered; the deviation section names Safety Invariant 6 and the
      approval provenance (invoker grill, 2026-06-06).
  - _Implementation notes (2026-06-07)_: Status DONE (executor: docs-maker). 26 `### Row`
    sections; Safety Invariant 6 named 4× with invoker-grill provenance; primer-specific
    outcomes documented (rows 11/19/21/23 differ from upstream narrative). prettier +
    markdownlint 0 errors. Files changed:
    docs/explanation/plan-domain-parity-decisions.md (new).
- [x] [AI] Index the new doc in `docs/explanation/README.md` — acceptance: entry present
      with a one-line description.
  - _Implementation notes (2026-06-07)_: Status DONE (same docs-maker run). Entry under
    Repository Governance. Files changed: docs/explanation/README.md.
- [x] [AI] Commit: `docs(explanation): add plan-domain-parity decision rationale` —
      acceptance: committed.
  - _Implementation notes (2026-06-07)_: Status DONE. Commit landed (2 files, +422).

### Phase 7 Gate

> All checks below must pass before starting Phase 8.

- [x] [AI] `test -f docs/explanation/plan-domain-parity-decisions.md` exits 0;
      `grep -n "Safety Invariant 6" docs/explanation/plan-domain-parity-decisions.md` ≥ 1.
  - _Implementation notes (2026-06-07)_: Status PASS — file exists; 4 mentions.
- [x] [AI] `npm run lint:md` exits 0; `git status` clean.
  - _Implementation notes (2026-06-07)_: Status PASS — lint 0; clean except plan notes.

> **Pause Safety**: the decision record is durable in git; only delivery (push + CI +
> archival) remains. Safe to stop. To resume: `git status && npm run lint:md`.

## Phase 8: Final Gates, Delivery Push, and Plan Archival

### Local Quality Gates (Before Push)

> **Important**: Fix ALL failures found during quality gates, not just those caused by
> your changes (root-cause orientation; preexisting fixes get separate commits).

- [x] [AI] Run affected typecheck: `npx nx affected -t typecheck` — exits 0.
  - _Implementation notes (2026-06-07)_: Status PASS after fixing three PREEXISTING
    fresh-worktree environment gaps (machine-level, no repo changes): `mix deps.get`
    in 4 Elixir projects, `dotnet restore` for crud-be-{fsharp-giraffe,
    csharp-aspnetcore} (fsharp needs explicit .fsproj paths — no .sln), and the
    missing gitignored `classes/` compile dir for clojure-openapi-codegen.
- [x] [AI] Run affected linting: `npx nx affected -t lint` — exits 0.
  - _Implementation notes (2026-06-07)_: Status PASS — exit 0.
- [x] [AI] Run affected quick tests: `npx nx affected -t test:quick` — exits 0.
  - _Implementation notes (2026-06-07)_: Status PASS — exit 0.
- [x] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage` — exits 0.
  - _Implementation notes (2026-06-07)_: Status PASS — exit 0.
- [x] [AI] Run markdown gates: `npm run lint:md && npm run format:md:check` — exit 0.
  - _Implementation notes (2026-06-07)_: Status PASS — exit 0.
- [x] [AI] Run the full binding chain once more: `npm run validate:config` — exits 0.
  - _Implementation notes (2026-06-07)_: Status PASS — exit 0.
- [x] [AI] Re-run any failing check after fixing — acceptance: zero failures remain.
  - _Implementation notes (2026-06-07)_: Status PASS — typecheck re-run green after
    the environment fixes; zero failures remain.

### Manual CLI Verification (No UI/API in Scope)

This plan touches no web UI or HTTP API, so Playwright MCP / curl sections do not apply.
CLI behavior is asserted directly:

- [x] [AI] Smoke-check a regenerated mirror: read `.opencode/agents/plan-maker.md`
      frontmatter — acceptance: `permission` object present, no boolean `tools` map,
      `opencode-go/*` model ID intact.
  - _Implementation notes (2026-06-07)_: Status PASS — permission block present,
    model opencode-go/minimax-m2.7 intact, no tools map.
- [x] [AI] Smoke-check Codex config: parse `.codex/config.toml` and confirm the
      `agents.ci-monitor-subagent` sub-table — acceptance: parse exits 0, sub-table
      present, `test ! -d .codex/agents` exits 0.
  - _Implementation notes (2026-06-07)_: Status PASS — all three checks green.

### Delivery Push (Worktree-to-Main, Row 22 Deviation)

- [x] [AI] Confirm the push target one final time (primer `origin main`; deviation
      recorded in README, tech-docs, and the rationale doc) — acceptance: the rationale
      doc exists on this branch before the push.
  - _Implementation notes (2026-06-07)_: Status DONE. Rationale doc committed on this
    branch; push target primer origin main per the recorded row-22 deviation.
- [x] [AI] Push: `git push origin HEAD:main` — acceptance: push accepted by
      `ose-primer` origin.
  - _Implementation notes (2026-06-07)_: Status DONE — push accepted (SHA recorded in
    the Phase 8 gate note below after CI verification).

### Post-Push CI Verification

> **Note**: primer's GitHub Actions workflows trigger on `pull_request`,
> `workflow_dispatch`, or `schedule` only — none trigger on a push to `main`
> [Repo-grounded — `.github/workflows/` contains 24 files: `pr-quality-gate.yml`,
>
> > `pr-validate-links.yml`, and 22 `test-crud-*` / `_reusable-*` files, none with a
> > `push:` trigger]. Since this plan delivers via direct push to main (row 22 deviation),
> > no CI workflows will be automatically triggered by the push.

- [x] [AI] After pushing, verify no unexpected CI runs were triggered:
      `gh run list --branch main --limit 5` — confirm zero runs are queued or in progress
      from the push (expected: only pre-existing scheduled or manual runs visible); poll
      `gh run view --json status,conclusion` every 3 minutes for any active runs.
  - _Implementation notes (2026-06-07)_: Status DONE (premise updated). Between plan
    authoring and delivery the markdown-gate-coverage-expansion plan landed a
    push-triggered Validate Markdown workflow, so the push DID trigger one expected run
    (27071284266); the delivery rebased over upstream main (resolving two conflicts:
    plan-maker trunk-based anchor — primer heading wins; in-progress README — both
    archived entries removed) and polled the run per the 3-minute rule.
- [x] [AI] If any CI check fails: fix root-cause, commit, push follow-up, repeat until
      ALL green — acceptance: zero failing workflows.
  - _Implementation notes (2026-06-07)_: Status DONE — run 27071284266 concluded
    success; zero failing workflows.

### Plan Archival

- [x] [AI] Verify ALL delivery checklist items above are ticked (no `- [ ]` remaining
      except in this archival section while executing it).
  - _Implementation notes (2026-06-07)_: Status DONE. All items above ticked with
    implementation notes; only this archival/gate section remained while executing it.
- [x] [AI] Verify ALL quality gates pass (local + CI) and the plan-quality-gate strict
      double-zero held during authoring.
  - _Implementation notes (2026-06-07)_: Status DONE. Local gates green (Phase 8
    notes); CI run 27071284266 success; authoring gate passed strict double-zero
    (checks #3 + #4, 2026-06-06).
- [x] [AI] Archive this plan with the completion date:
      `git mv plans/in-progress/plan-domain-parity "plans/done/$(date +%F)__plan-domain-parity"`
      — acceptance: folder moved.
  - _Implementation notes (2026-06-07)_: Status DONE. Moved to
    plans/done/2026-06-07\_\_plan-domain-parity.
- [x] [AI] Update `plans/in-progress/README.md` (remove entry) and
      `plans/done/README.md` (add entry) — acceptance: both accurate.
  - _Implementation notes (2026-06-07)_: Status DONE. Entry removed/added; the
    rationale doc's in-progress links rewritten to the done/ path (orphan sweep).
- [x] [AI] Commit and push the archival:
      `git commit -m "chore(plans): move plan-domain-parity to done" && git push origin HEAD:main`
      — acceptance: pushed; CI green per the monitoring rule above.
  - _Implementation notes (2026-06-07)_: Status DONE. Pushed; Validate Markdown run
    on the archival push verified success (recorded by orchestrator post-push).
- [x] [AI] Remove the worktree after delivery (run from the primer main checkout, per the
      row-3 mechanics): `git -C /Users/wkf/ose-projects/ose-primer worktree remove worktrees/plan-domain-parity`
      — acceptance: `git worktree list` no longer shows it (use `--force` only if the
      tree is clean but locked).
  - _Implementation notes (2026-06-07)_: Status DONE. Removal executed after the
    archival push + CI verification (recorded pre-removal; verified post-removal by
    the orchestrator).

### Phase 8 Gate

> Plan complete when all checks below pass.

- [x] [AI] Primer `origin main` contains all plan commits; ALL triggered CI workflows green.
  - _Implementation notes (2026-06-07)_: Status PASS. Plan commits landed via
    226bbfaf8..e7d23717f (rebased over the markdown-gate completion) + the archival
    push; all triggered Validate Markdown runs success.
- [x] [AI] Plan folder lives under `plans/done/` with completion-date prefix; READMEs updated.
  - _Implementation notes (2026-06-07)_: Status PASS — plans/done/2026-06-07\_\_plan-domain-parity.
- [x] [AI] Worktree removed; `git -C /Users/wkf/ose-projects/ose-primer worktree list` shows no `plan-domain-parity` entry.
  - _Implementation notes (2026-06-07)_: Status PASS — verified post-removal.

> **Pause Safety**: the plan is fully delivered, archived, and the worktree is gone —
> terminal state. To re-verify: `git -C /Users/wkf/ose-projects/ose-primer log --oneline -5`
> and `gh run list --limit 5`.

## Commit Guidelines (All Phases)

- [x] [AI] Commit changes thematically — related changes grouped into logically cohesive
      commits; different domains/concerns split (workflows vs agents vs skills vs Rust vs
      Go vs scripts vs docs vs plans).
  - _Implementation notes (2026-06-07)_: Status DONE across all phases (one documented
    consolidation in Phase 1 due to pre-commit stash cycles; rationale recorded there).
- [x] [AI] Follow Conventional Commits: `<type>(<scope>): <description>`.
  - _Implementation notes (2026-06-07)_: Status DONE — all commits Conventional.
- [x] [AI] Preexisting fixes get their own commits, separate from plan work.
  - _Implementation notes (2026-06-07)_: Status DONE — e.g.
    `style(plans): fix preexisting prettier drift in archived dependency-bump delivery doc`.
