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

- [ ] [AI] Install dependencies in the worktree
      (`/Users/wkf/ose-projects/ose-primer/worktrees/plan-domain-parity/`): `npm install`
      — acceptance: exits 0, `node_modules/` synchronized.
- [ ] [AI] Converge the toolchain: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift (Rust, Go, Node, Docker all green).
- [ ] [AI] Verify sibling clones exist (read-only merge sources):
      `test -d /Users/wkf/ose-projects/ose-public && test -d /Users/wkf/ose-projects/ose-infra`
      — acceptance: both exist.
- [ ] [AI] Establish the code baseline:
      `npx nx run-many -t typecheck,lint,test:quick,spec-coverage -p rhino-cli-rust,rhino-cli-go`
      — acceptance: baseline pass/fail recorded; all preexisting failures documented.
- [ ] [AI] Establish the markdown/bindings baseline: `npm run lint:md && npm run validate:config`
      — acceptance: exit codes recorded; preexisting failures documented.
- [ ] [AI] Resolve all preexisting failures before proceeding (root-cause orientation;
      commit preexisting fixes separately) — acceptance: zero unresolved baseline failures.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick,spec-coverage -p rhino-cli-rust,rhino-cli-go`
      exits 0 (or every preexisting failure resolved and committed).

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no
> plan work exists yet beyond the plan documents themselves. Safe to stop indefinitely.
> To resume: re-run the baseline command and confirm it is still clean.

## Phase 1: Adopt Merged Governance Canon (Rows 3–16)

> **Important**: Fix ALL failures found during quality gates, not just those caused by
> your changes. Commit preexisting fixes separately with conventional commit messages.

- [ ] [AI] **Sequencing gate (hard)**: refresh the upstream clone
      (`git -C /Users/wkf/ose-projects/ose-public pull --ff-only`) and verify the
      ose-public plan-domain-parity merge has landed: confirm
      `/Users/wkf/ose-projects/ose-public/repo-governance/workflows/plan/plan-establishment-execution.md`
      contains the worktree-default mechanics (`git worktree add -b <identifier>`) AND
      `grep -rn "permission" /Users/wkf/ose-projects/ose-public/apps/rhino-cli/src/internal/agents/converter.rs`
      returns matches — acceptance: both checks pass. **STOP and surface to the human if
      not** (the upstream plan must execute first; this is the documented execution-order
      dependency).
- [ ] [AI] Merge `repo-governance/workflows/plan/plan-establishment-execution.md` from
      the upstream canon (semantic merge, preserve primer link targets) — acceptance:
      primer copy contains the `target-stage` input AND the full worktree-default
      mechanics (provision `worktrees/<identifier>/` via
      `git worktree add -b <identifier> worktrees/<identifier> main` + `npm install` +
      `npm run doctor -- --fix`; commit in worktree; push HEAD to confirmed push-target,
      default `origin main`; remove worktree after delivery);
      `grep -c "target-stage" <file>` ≥ 1.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Merge `repo-governance/workflows/plan/plan-execution.md` — acceptance: merged
      canon adopted; primer-specific agent-selection lists preserved (diff against the
      pre-merge copy shows no primer-only agent name removed).
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Merge `repo-governance/workflows/meta/execution-modes.md` — acceptance:
      matches the merged canon; primer link targets intact.
- [ ] [AI] Merge `.claude/agents/plan-maker.md`, `.claude/agents/plan-checker.md`,
      `.claude/agents/plan-fixer.md`, `.claude/agents/plan-execution-checker.md`
      (one commit-reviewable edit per file; preserve primer repo refs such as
      `rhino-cli-rust` naming and primer app examples) — acceptance: each file matches
      the merged canon modulo enumerated divergences;
      `grep -L "rhino-cli\b" .claude/agents/plan-*.md` shows no upstream-only
      `apps/rhino-cli` paths leaked into primer files.
- [ ] [AI] Reconcile `.claude/agents/repo-setup-manager.md` (row 11): diff the 3-line
      primer divergence against the canon; keep lines that are repo-specific
      (`rhino-cli-rust` naming), merge the rest — acceptance: divergence decision noted
      inline in the commit message.
- [ ] [AI] Merge `.claude/skills/plan-creating-project-plans/SKILL.md` including infra's
      mandatory grilling gates (row 12) — acceptance: pre-write AND post-write grilling
      gate sections present; primer path refs intact.
- [ ] [AI] Merge `.claude/skills/plan-writing-gherkin-criteria/SKILL.md` and
      `.claude/skills/grill-me/SKILL.md` (rows 13–14, trivial drift) — acceptance: match
      merged canon.
- [ ] [AI] Merge `repo-governance/conventions/structure/plans.md` (row 16) — acceptance:
      matches merged canon modulo primer-specific examples; internal anchors used by
      primer agents/workflows still resolve
      (`npx nx run rhino-cli-rust:validate:mermaid` and the link validator pass in the
      Phase 1 gate below).
- [ ] [AI] Regenerate secondary bindings for the changed agent files with the CURRENT
      script: `npm run generate:bindings` — acceptance: exits 0; `.opencode/agents/`
      mirrors of the four plan agents updated.
- [ ] [AI] Commit thematically (Conventional Commits; separate commits for workflows,
      agents, skills, convention; e.g. `docs(workflows): adopt merged plan-establishment
  canon with target-stage and worktree default`) — acceptance: `git log` shows
      domain-split commits, no unrelated bundling.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `grep -n "target-stage" repo-governance/workflows/plan/plan-establishment-execution.md`
      returns ≥ 1 match.
- [ ] [AI] `npm run lint:md` exits 0 and `npm run format:md:check` exits 0.
- [ ] [AI] `npm run validate:sync` exits 0 (mirrors consistent after regeneration).
- [ ] [AI] `npx nx affected -t typecheck lint test:quick` exits 0 (affected projects after
      agent file merges and `npm run generate:bindings` regeneration).
- [ ] [AI] `git status` clean (everything committed).

> **Pause Safety**: the merged governance canon is fully adopted and committed in the
> worktree; no code or script changes yet; bindings are consistent. Safe to stop. To
> resume: `npm run validate:sync && npm run lint:md`.

## Phase 2: New Governance Files and Indexes (Rows 1, 2, 5, 15)

- [ ] [AI] Create `repo-governance/workflows/plan/plan-multi-repo-parity-planning.md`
      (_New file_) from the upstream amended copy at
      `/Users/wkf/ose-projects/ose-public/repo-governance/workflows/plan/plan-multi-repo-parity-planning.md`,
      adapting only repo-local link targets — acceptance: file exists; step sequence is
      Survey → Matrix → First Grill (hard gate) → web-research-maker (conditional) →
      Second Grill (post-research) → Author → Gate → Deliver; workflow-naming validator
      passes (`npx nx run rhino-cli-rust:validate:naming-workflows` exits 0).
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Create `repo-governance/development/workflow/grilling-with-options.md`
      (_New file_) from the upstream merged convention (public `grilling-with-options.md`
      merged with infra's broader-scope `grilling.md`), adapting repo-local links —
      acceptance: file exists; the multi-options HARD RULE (2–4 options, one recommended,
      one question at a time) and infra's broader scope are both present.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Update `repo-governance/workflows/plan/README.md` (row 5) — acceptance: indexes
      exactly 4 workflows (establishment, execution, quality-gate, multi-repo parity) and
      the Grilling Format section links the new convention file.
- [ ] [AI] Update `repo-governance/workflows/README.md` — acceptance: parity workflow
      listed in the catalog table with description and participating agents.
- [ ] [AI] Update `repo-governance/development/workflow/README.md` — acceptance:
      `grilling-with-options.md` indexed.
- [ ] [AI] Update `AGENTS.md`: plan-maker catalog wording (grilling-with-options
      reference) and workflow references — acceptance: `grep -n "grilling-with-options" AGENTS.md`
      ≥ 1; no stale reference to a nonexistent convention remains.
- [ ] [AI] Commit thematically — acceptance: separate commits for new workflow, new
      convention, index/catalog updates.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `test -f repo-governance/workflows/plan/plan-multi-repo-parity-planning.md && test -f repo-governance/development/workflow/grilling-with-options.md` exits 0.
- [ ] [AI] `npx nx run rhino-cli-rust:validate:naming-workflows` exits 0.
- [ ] [AI] `npm run lint:md` exits 0; link validation on the touched files passes
      (`npx nx run rhino-cli-rust:validate:mermaid` for any new diagrams; repo link
      validator per pre-commit hook).
- [ ] [AI] `git status` clean.

> **Pause Safety**: governance surface is complete (canon + new files + indexes); CLIs
> and scripts untouched. Safe to stop. To resume: `npm run lint:md && git status`.

## Phase 3: Rust Emitter Modernization (Rows 18–19)

> _Suggested executor: `swe-rust-dev`_

### Row 18 — OpenCode `permission` Object (TDD)

- [ ] [AI] **RED**: read the landed upstream implementation
      (`/Users/wkf/ose-projects/ose-public/apps/rhino-cli/src/internal/agents/converter.rs`)
      to fix the exact permission-object shape, then add a failing unit test
      (_New test_, e.g. `convert_permission_maps_granted_tools_to_allow`) in the tests
      module of `apps/rhino-cli-rust/src/internal/agents/converter.rs` asserting the
      converter output frontmatter contains a `permission` object (granted tool →
      `allow`, matching the upstream shape) and NO boolean `tools` map. Run
      `npx nx run rhino-cli-rust:test:unit` — acceptance: the new test FAILS, all others
      pass.
- [ ] [AI] **GREEN**: implement the change in
      `apps/rhino-cli-rust/src/internal/agents/converter.rs` (replace `convert_tools`
      boolean map emission) and `apps/rhino-cli-rust/src/internal/agents/types.rs`
      (struct field), updating the serializer emission order accordingly. Run
      `npx nx run rhino-cli-rust:test:unit` — acceptance: the new test PASSES, zero
      regressions.
- [ ] [AI] **REFACTOR**: update
      `apps/rhino-cli-rust/src/internal/agents/sync_validator.rs` (and its tests) to
      validate the new frontmatter shape; remove dead boolean-map code; run
      `npx nx run rhino-cli-rust:test:unit && npx nx run rhino-cli-rust:lint` —
      acceptance: both exit 0; no `convert_tools` boolean emission remains
      (`grep -n "tools" apps/rhino-cli-rust/src/internal/agents/converter.rs` shows only
      parsing of Claude-side `tools` input).
- [ ] [AI] Regenerate all OpenCode mirrors: `npm run generate:bindings` — acceptance:
      every `.opencode/agents/*.md` frontmatter contains `permission` and no boolean
      `tools` map (`grep -L "permission" .opencode/agents/*.md` returns only README-type
      files, if any); `npm run validate:sync` exits 0.

### Row 19 — Codex `config.toml` Migration

- [ ] [AI] Migrate the `ci-monitor-subagent` configuration: inline the contents of
      `.codex/agents/ci-monitor-subagent.toml` into the existing
      `[agents.ci-monitor-subagent]` sub-table in `.codex/config.toml` per the official
      Codex config reference (where a key cannot be inlined, relocate the per-agent TOML
      to `.codex/ci-monitor-subagent.toml` and update `config_file` to the new
      non-`agents/` path) — acceptance: `.codex/config.toml` parses
      (`python3 -c "import tomllib;tomllib.load(open('.codex/config.toml','rb'))"` exits 0) and carries the agent config.
- [ ] [AI] Delete the unofficial directory: `git rm -r .codex/agents/` — acceptance:
      `test ! -d .codex/agents` exits 0.
- [ ] [AI] Sweep code and tests for stale references:
      `grep -rn "\.codex/agents" apps/ .claude/ .opencode/ repo-governance/ docs/ AGENTS.md CLAUDE.md package.json`
      — acceptance: zero matches outside `plans/` history; any rhino-cli code/test
      reference found is updated under its existing test suite
      (`npx nx run rhino-cli-rust:test:unit` stays green).
- [ ] [AI] Update binding docs for rows 18–19: `docs/reference/platform-bindings.md`
      (lines 63, 65, 87 — `.codex/agents/` references and the Codex layout note),
      `repo-governance/conventions/structure/multi-harness-binding.md` (codex layout),
      `AGENTS.md` lines 48/86/215 wording (boolean flags → permission object), and
      `CLAUDE.md` line 51 — acceptance: no doc claims boolean `tools` flags or
      `.codex/agents/` as the current format; `npm run lint:md` exits 0.
  - _Suggested executor: `docs-maker`_
- [ ] [AI] Commit thematically — acceptance: separate commits for the Rust emitter
      change (`feat(rhino-cli-rust): emit opencode permission object`), the mirror
      regeneration, the Codex migration, and the doc updates.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `npx nx run rhino-cli-rust:test:unit && npx nx run rhino-cli-rust:test:quick && npx nx run rhino-cli-rust:lint && npx nx run rhino-cli-rust:typecheck` all exit 0.
- [ ] [AI] `npm run validate:sync && npm run validate:harness-bindings` exit 0.
- [ ] [AI] `test ! -d .codex/agents` exits 0 and the grep sweep above returns zero live references.
- [ ] [AI] `git status` clean.

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

- [ ] [AI] **RED**: add a failing unit test (_New test_, e.g.
      `TestConvertPermission_MapsGrantedToolsToAllow`) in
      `apps/rhino-cli-go/internal/agents/converter_test.go` asserting the Go converter
      emits the same `permission` object shape as the Rust implementation (use a fixture
      mirroring the Rust test). Run `npx nx run rhino-cli-go:test:unit` — acceptance:
      the new test FAILS, all others pass.
- [ ] [AI] **GREEN**: implement in `apps/rhino-cli-go/internal/agents/converter.go`
      (replace `ConvertTools` boolean-map emission) and
      `apps/rhino-cli-go/internal/agents/types.go`. Run
      `npx nx run rhino-cli-go:test:unit` — acceptance: new test PASSES, zero
      regressions.
- [ ] [AI] **REFACTOR**: update `apps/rhino-cli-go/internal/agents/sync_validator.go`
      (and tests) for the new shape; remove dead code; verify `.codex` handling in
      `apps/rhino-cli-go/internal/agents/bindings.go` needs no change (the `.codex` dir
      entry at line 61 remains valid). Run
      `npx nx run rhino-cli-go:test:unit && npx nx run rhino-cli-go:lint` — acceptance:
      both exit 0.
- [ ] [AI] Run the Go integration suite: `npx nx run rhino-cli-go:test:integration` —
      acceptance: exits 0.

### Row 20 — `generate:bindings` Direct-Cargo Invocation

- [ ] [AI] Edit `package.json` (scripts at lines 44–47 and the validate family): switch
      `generate:bindings`, `sync:agents`, `sync:skills`, `sync:dry-run`, `validate:sync`,
      `validate:claude`, and `validate:harness-bindings` from the
      `nx run rhino-cli-rust:build … ./apps/rhino-cli-rust/dist/rhino-cli` pattern to the
      direct-cargo pattern used by ose-public [Repo-grounded — tech-docs §Survey
      Corrections], substituting the primer manifest, e.g.:
      `cargo run --release --quiet --manifest-path apps/rhino-cli-rust/Cargo.toml -- agents sync && cargo run --release --quiet --manifest-path apps/rhino-cli-rust/Cargo.toml -- agents emit-bindings`
      — acceptance: `npm run generate:bindings` exits 0; the Go CLI is NOT referenced by
      any of these scripts (row 21: Rust stays canonical).
- [ ] [AI] Determinism check: run `npm run generate:bindings` a second time —
      acceptance: `git status --short` is empty afterward.
- [ ] [AI] Update any doc that quotes the old script form (check `CONTRIBUTING.md`,
      `AGENTS.md`, `docs/reference/platform-bindings.md`,
      `apps/rhino-cli-rust/README.md`, `apps/rhino-cli-go/README.md`:
      `grep -rn "dist/rhino-cli agents" *.md docs/ apps/*/README.md`) — acceptance: zero
      stale quotes of the nx-build+dist invocation for the binding scripts.
- [ ] [AI] Commit thematically — acceptance: separate commits for the Go port
      (`feat(rhino-cli-go): emit opencode permission object`) and the script switch
      (`build: switch binding scripts to direct cargo run`).

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick -p rhino-cli-rust,rhino-cli-go` exits 0.
- [ ] [AI] `npx nx run-many -t spec-coverage -p rhino-cli-rust,rhino-cli-go` exits 0 (Gherkin spec parity between both CLIs confirmed after Go port).
- [ ] [AI] `npx nx run rhino-cli-rust:validate:cross-vendor-parity && npx nx run rhino-cli-go:validate:cross-vendor-parity` both exit 0 (dual-CLI parity guard green).
- [ ] [AI] `npm run generate:bindings` exits 0 twice consecutively with clean `git status` after the second run.
- [ ] [AI] `git status` clean.

> **Pause Safety**: both CLIs emit the modern formats, the parity guard is green, and the
> script family is aligned. Safe to stop. To resume:
> `npx nx run rhino-cli-rust:validate:cross-vendor-parity`.

## Phase 5: Full Repo-Wide Binding Audit (Row 17)

> _Suggested executor: `repo-harness-compatibility-checker` (via the harness
> compatibility quality-gate workflow)_

- [ ] [AI] Enumerate the agent surface: list the 50 `.claude/agents/*.md` definitions
      (excluding `README.md`) and the 50 `.opencode/agents/*.md` mirrors; verify
      post-regeneration parity (50:50 match expected; no gap was present at plan authoring
      [Repo-grounded — 2026-06-06]) — acceptance: every `.claude/agents/*.md` definition
      has a corresponding `.opencode/agents/*.md` mirror, or its absence is documented as
      an intentional exclusion (with the excluding rule cited) in the audit record under
      `generated-reports/`.
- [ ] [AI] Verify `.amazonq` bridge artifacts byte-exact:
      `npm run validate:harness-bindings` — acceptance: exits 0
      (`.amazonq/rules/00-agents-md.md`, `.amazonq/cli-agents/ose-default.json` match
      emitter expectations).
- [ ] [AI] Verify `.codex` conforms post-migration: `.codex/config.toml` parses and no
      `.codex/agents/` exists (re-run the Phase 3 checks) — acceptance: both pass.
- [ ] [AI] Run the full config validation chain: `npm run validate:config` (validate:claude
      → generate:bindings → validate:opencode) — acceptance: exits 0.
- [ ] [AI] Run both repo-governance vendor audits:
      `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit && npx nx run rhino-cli-go:validate:repo-governance-vendor-audit`
      — acceptance: both exit 0 (no vendor-specific leakage introduced into
      `repo-governance/` by the Phase 1–2 merges).
- [ ] [AI] Commit any audit-driven fixes thematically — acceptance: `git status` clean.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `npm run validate:config` exits 0.
- [ ] [AI] `npx nx run-many -t validate:cross-vendor-parity,validate:repo-governance-vendor-audit -p rhino-cli-rust,rhino-cli-go` exits 0.
- [ ] [AI] Audit record written to `generated-reports/` documenting the mirror-gap reconciliation.

> **Pause Safety**: the entire binding surface (all agents × .opencode/.amazonq/.codex)
> is audited and green. Safe to stop. To resume: `npm run validate:config`.

## Phase 6: Supersede planning-system-overhaul (Row 23)

- [ ] [AI] Re-inventory `plans/in-progress/planning-system-overhaul/delivery.md` for
      unchecked items: `grep -n "^- \[ \]" plans/in-progress/planning-system-overhaul/delivery.md`
      — acceptance: confirmed that only archival items remain (lines 216–232 as of
      2026-06-06 [Repo-grounded]); if any NEW substantive unchecked item appears, absorb
      it into this checklist before proceeding and record the absorption in the commit
      message.
- [ ] [AI] Add a supersession pointer to
      `plans/in-progress/planning-system-overhaul/README.md`: a `## Superseded` section
      stating the plan is closed by `plans/in-progress/plan-domain-parity/` (matrix
      row 23) with a relative link — acceptance: section present; `npm run lint:md`
      passes on the file.
- [ ] [AI] Archive with completion-date prefix (today's date at execution time):
      `git mv plans/in-progress/planning-system-overhaul "plans/done/$(date +%F)__planning-system-overhaul"`
      — acceptance: folder exists under `plans/done/` with the date prefix; nothing
      remains under `plans/in-progress/planning-system-overhaul/`.
- [ ] [AI] Update `plans/in-progress/README.md` (remove the entry) and
      `plans/done/README.md` (add the entry with completion date and supersession note)
      — acceptance: both lists accurate.
- [ ] [AI] Sweep orphaned references:
      `grep -rn "in-progress/planning-system-overhaul" . --include="*.md" | grep -v plans/done`
      — acceptance: zero live references.
- [ ] [AI] Commit: `chore(plans): archive planning-system-overhaul as superseded by plan-domain-parity`
      — acceptance: single archival commit.

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] `test ! -d plans/in-progress/planning-system-overhaul` exits 0.
- [ ] [AI] The orphan-reference sweep returns zero live references; `npm run lint:md` exits 0.

> **Pause Safety**: exactly one in-progress plan owns the planning-system concern; the
> old plan is archived with a pointer. Safe to stop. To resume: re-run the orphan sweep.

## Phase 7: Rationale Doc (Rows 22, 24)

> _Suggested executor: `docs-maker`_

- [ ] [AI] Create `docs/explanation/plan-domain-parity-decisions.md` (_New file_,
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
- [ ] [AI] Index the new doc in `docs/explanation/README.md` — acceptance: entry present
      with a one-line description.
- [ ] [AI] Commit: `docs(explanation): add plan-domain-parity decision rationale` —
      acceptance: committed.

### Phase 7 Gate

> All checks below must pass before starting Phase 8.

- [ ] [AI] `test -f docs/explanation/plan-domain-parity-decisions.md` exits 0;
      `grep -n "Safety Invariant 6" docs/explanation/plan-domain-parity-decisions.md` ≥ 1.
- [ ] [AI] `npm run lint:md` exits 0; `git status` clean.

> **Pause Safety**: the decision record is durable in git; only delivery (push + CI +
> archival) remains. Safe to stop. To resume: `git status && npm run lint:md`.

## Phase 8: Final Gates, Delivery Push, and Plan Archival

### Local Quality Gates (Before Push)

> **Important**: Fix ALL failures found during quality gates, not just those caused by
> your changes (root-cause orientation; preexisting fixes get separate commits).

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck` — exits 0.
- [ ] [AI] Run affected linting: `npx nx affected -t lint` — exits 0.
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick` — exits 0.
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage` — exits 0.
- [ ] [AI] Run markdown gates: `npm run lint:md && npm run format:md:check` — exit 0.
- [ ] [AI] Run the full binding chain once more: `npm run validate:config` — exits 0.
- [ ] [AI] Re-run any failing check after fixing — acceptance: zero failures remain.

### Manual CLI Verification (No UI/API in Scope)

This plan touches no web UI or HTTP API, so Playwright MCP / curl sections do not apply.
CLI behavior is asserted directly:

- [ ] [AI] Smoke-check a regenerated mirror: read `.opencode/agents/plan-maker.md`
      frontmatter — acceptance: `permission` object present, no boolean `tools` map,
      `opencode-go/*` model ID intact.
- [ ] [AI] Smoke-check Codex config: parse `.codex/config.toml` and confirm the
      `agents.ci-monitor-subagent` sub-table — acceptance: parse exits 0, sub-table
      present, `test ! -d .codex/agents` exits 0.

### Delivery Push (Worktree-to-Main, Row 22 Deviation)

- [ ] [AI] Confirm the push target one final time (primer `origin main`; deviation
      recorded in README, tech-docs, and the rationale doc) — acceptance: the rationale
      doc exists on this branch before the push.
- [ ] [AI] Push: `git push origin HEAD:main` — acceptance: push accepted by
      `ose-primer` origin.

### Post-Push CI Verification

> **Note**: primer's GitHub Actions workflows trigger on `pull_request`,
> `workflow_dispatch`, or `schedule` only — none trigger on a push to `main`
> [Repo-grounded — `.github/workflows/` contains 24 files: `pr-quality-gate.yml`,
>
> > `pr-validate-links.yml`, and 22 `test-crud-*` / `_reusable-*` files, none with a
> > `push:` trigger]. Since this plan delivers via direct push to main (row 22 deviation),
> > no CI workflows will be automatically triggered by the push.

- [ ] [AI] After pushing, verify no unexpected CI runs were triggered:
      `gh run list --branch main --limit 5` — confirm zero runs are queued or in progress
      from the push (expected: only pre-existing scheduled or manual runs visible); poll
      `gh run view --json status,conclusion` every 3 minutes for any active runs.
- [ ] [AI] If any CI check fails: fix root-cause, commit, push follow-up, repeat until
      ALL green — acceptance: zero failing workflows.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items above are ticked (no `- [ ]` remaining
      except in this archival section while executing it).
- [ ] [AI] Verify ALL quality gates pass (local + CI) and the plan-quality-gate strict
      double-zero held during authoring.
- [ ] [AI] Archive this plan with the completion date:
      `git mv plans/in-progress/plan-domain-parity "plans/done/$(date +%F)__plan-domain-parity"`
      — acceptance: folder moved.
- [ ] [AI] Update `plans/in-progress/README.md` (remove entry) and
      `plans/done/README.md` (add entry) — acceptance: both accurate.
- [ ] [AI] Commit and push the archival:
      `git commit -m "chore(plans): move plan-domain-parity to done" && git push origin HEAD:main`
      — acceptance: pushed; CI green per the monitoring rule above.
- [ ] [AI] Remove the worktree after delivery (run from the primer main checkout, per the
      row-3 mechanics): `git -C /Users/wkf/ose-projects/ose-primer worktree remove worktrees/plan-domain-parity`
      — acceptance: `git worktree list` no longer shows it (use `--force` only if the
      tree is clean but locked).

### Phase 8 Gate

> Plan complete when all checks below pass.

- [ ] [AI] Primer `origin main` contains all plan commits; ALL triggered CI workflows green.
- [ ] [AI] Plan folder lives under `plans/done/` with completion-date prefix; READMEs updated.
- [ ] [AI] Worktree removed; `git -C /Users/wkf/ose-projects/ose-primer worktree list` shows no `plan-domain-parity` entry.

> **Pause Safety**: the plan is fully delivered, archived, and the worktree is gone —
> terminal state. To re-verify: `git -C /Users/wkf/ose-projects/ose-primer log --oneline -5`
> and `gh run list --limit 5`.

## Commit Guidelines (All Phases)

- [ ] [AI] Commit changes thematically — related changes grouped into logically cohesive
      commits; different domains/concerns split (workflows vs agents vs skills vs Rust vs
      Go vs scripts vs docs vs plans).
- [ ] [AI] Follow Conventional Commits: `<type>(<scope>): <description>`.
- [ ] [AI] Preexisting fixes get their own commits, separate from plan work.
