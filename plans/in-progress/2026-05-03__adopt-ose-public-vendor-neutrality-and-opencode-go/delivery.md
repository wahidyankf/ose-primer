---
title: Delivery — Adopt ose-public Vendor-Neutrality, OpenCode Go, and Companion Tooling
---

# Delivery Checklist

Execute phases in order. Each `- [ ]` is one tick — one concrete action.
Do not batch ticks across phase boundaries. Use one Conventional-Commits
commit per thematic phase unless explicitly grouped below. Each
code-touching tick follows Red→Green→Refactor per the W9 convention
(once W9 lands; for W1–W8, follow the same discipline as a forward
courtesy to the convention adopting itself).

**Publish path**: direct push to `origin main` per
[Git Push Default Convention](../../../governance/development/workflow/git-push-default.md)
Standards 1, 2, 6. No draft PR is opened — the user has not requested one.
Worktree is optional; if used, push via `git push origin HEAD:main` per Standard 6.

**Reference reading per phase**: see the per-workstream "Source" section in
[tech-docs.md](./tech-docs.md). Inside an ose-primer-rooted Claude session,
`../../ose-public/...` is empty per the bare-gitlink contract — read via
the GitHub UI URLs in tech-docs.md.

## Phase 0 — Worktree, baseline, environment

- [ ] Decide worktree-or-not. Recommended for parallel-safety; skip if
      single-session work on `main`.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] If worktree: `cd /Users/wkf/ose-projects/ose-primer && claude --worktree adopt-ose-public-batch`.
      Confirm the session lands inside `.claude/worktrees/adopt-ose-public-batch/`.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run `npm install` from the working tree root.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run `npm run doctor -- --fix` (mandatory worktree convergence).
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Confirm `go version` reports Go ≥ 1.22.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Confirm `node --version` reports 24.13.1 and `npm --version` reports 11.10.1.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run `nx affected -t typecheck lint test:quick spec-coverage` from working tree root.
      Capture failures (if any) in `local-temp/baseline.txt`. Must be clean.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run `nx run rhino-cli:test:unit`. Must pass at baseline.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run `nx run rhino-cli:test:integration`. Must pass at baseline.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Snapshot current state: `git rev-parse HEAD > local-temp/baseline-sha.txt`.
      _Date **/ Status:** / Files: **/ Notes:**_

## Phase 1 — W1: Sync correctness (singular → plural)

### 1A — Tests first (Red)

- [ ] Add a failing assertion in `apps/rhino-cli/internal/agents/converter_test.go`
      that `OpenCodeAgentDir == ".opencode/agents"` (plural). Run the test —
      it should fail because the constant is currently singular.
      _Date **/ Status:** / Files: apps/rhino-cli/internal/agents/converter_test.go / Notes: \_\_ _
- [ ] Add a failing assertion in `apps/rhino-cli/cmd/agents_sync.integration_test.go`
      that `.opencode/agents/<agent>.md` exists post-sync and `.opencode/agent/`
      does not. Run — should fail.
      \_Date **/ Status:** / Files: apps/rhino-cli/cmd/agents_sync.integration_test.go / Notes: \_\_\_
- [ ] Add a failing assertion in `apps/rhino-cli/internal/agents/sync_test.go`
      that `Sync()` does not create `.opencode/skill/`. Run — should fail.
      _Date **/ Status:** / Files: apps/rhino-cli/internal/agents/sync_test.go / Notes: \_\_ _

### 1B — Implementation (Green)

- [ ] In `apps/rhino-cli/internal/agents/converter.go`, change
      `OpenCodeAgentDir` constant from `.opencode/agent` to `.opencode/agents`.
      Update all doc comments mentioning the singular path.
      \_Date **/ Status:** / Files: apps/rhino-cli/internal/agents/converter.go / Notes: \_\_\_
- [ ] In `apps/rhino-cli/internal/agents/sync.go`, drop the
      `CopyAllSkills` invocation from `Sync()`. Update doc comment.
      \_Date **/ Status:** / Files: apps/rhino-cli/internal/agents/sync.go / Notes: \_\_\_
- [ ] Delete `apps/rhino-cli/internal/agents/copier.go`.
      \_Date **/ Status:** / Files: apps/rhino-cli/internal/agents/copier.go / Notes: \_\_\_
- [ ] Delete `apps/rhino-cli/internal/agents/copier_test.go`.
      _Date **/ Status:** / Files: apps/rhino-cli/internal/agents/copier_test.go / Notes: \_\_ _
- [ ] Update `apps/rhino-cli/internal/agents/sync_validator.go` to validate
      against `.opencode/agents/` (plural) and flag singular paths as drift.
      _Date **/ Status:** / Files: apps/rhino-cli/internal/agents/sync_validator.go / Notes: \_\_ _
- [ ] Update `apps/rhino-cli/internal/agents/sync_validator_test.go` fixtures.
      \_Date **/ Status:** / Files: apps/rhino-cli/internal/agents/sync_validator_test.go / Notes: \_\_\_
- [ ] Update `apps/rhino-cli/cmd/agents_sync.go` help text and doc comments.
      _Date **/ Status:** / Files: apps/rhino-cli/cmd/agents_sync.go / Notes: \_\_ _
- [ ] Update `apps/rhino-cli/cmd/agents_sync_test.go` and
      `apps/rhino-cli/cmd/agents_sync.integration_test.go` assertions.
      \_Date **/ Status:** / Files: as listed / Notes: \_\_\_
- [ ] Update `apps/rhino-cli/cmd/agents_validate_sync.go`,
      `agents_validate_sync_test.go`, `agents_validate_sync.integration_test.go`
      to plural path.
      \_Date **/ Status:** / Files: as listed / Notes: \_\_\_
- [ ] Update Gherkin specs at
      `specs/apps/rhino/cli/gherkin/agents-sync.feature` and
      `agents-validate-sync.feature` to plural path.
      \_Date **/ Status:** / Files: as listed / Notes: \_\_\_
- [ ] Run `nx run rhino-cli:test:unit`. All tests pass (Green).
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run `nx run rhino-cli:test:integration`. All tests pass.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run `npm run sync:claude-to-opencode`. Sync writes to `.opencode/agents/`.
      \_Date **/ Status:** / Files: .opencode/agents/\* / Notes: \_\_\_
- [ ] `git rm -r .opencode/agent .opencode/skill` (delete legacy singular paths).
      \_Date **/ Status:** / Files: as listed / Notes: \_\_\_
- [ ] `git add .opencode/agents` and any other modified files.
      _Date **/ Status:** / Files: **/ Notes:**_

### 1C — Refactor + verify

- [ ] Run `nx run rhino-cli:test:unit` and `nx run rhino-cli:test:integration` again.
      Coverage holds ≥90%.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run `npm run sync:claude-to-opencode` a second time. Should be a no-op
      (no diff).
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Commit: `feat(rhino-cli): migrate sync output to canonical .opencode/agents/ plural path`.
      _Date **/ Status:** / Files: **/ Notes:**_

## Phase 2 — W2: OpenCode Go provider

### 2A — Tests first (Red)

- [ ] In `apps/rhino-cli/internal/agents/converter_test.go`, update
      `TestConvertModel` expectations to `opencode-go/minimax-m2.7` (opus/sonnet/omitted)
      and `opencode-go/glm-5` (haiku). Run — fails.
      _Date **/ Status:** / Files: apps/rhino-cli/internal/agents/converter_test.go / Notes: \_\_ _
- [ ] In `apps/rhino-cli/internal/agents/types_test.go`, update
      `TestOpenCodeAgent` model expectation. Run — fails.
      _Date **/ Status:** / Files: apps/rhino-cli/internal/agents/types_test.go / Notes: \_\_ _

### 2B — Implementation (Green)

- [ ] Update `ConvertModel()` in `apps/rhino-cli/internal/agents/converter.go`
      to return `opencode-go/*` IDs.
      \_Date **/ Status:** / Files: apps/rhino-cli/internal/agents/converter.go / Notes: \_\_\_
- [ ] Update doc comments in `apps/rhino-cli/internal/agents/types.go`,
      `cmd/agents_sync.go`, `cmd/agents_validate_sync.go` to reference
      OpenCode Go IDs.
      \_Date **/ Status:** / Files: as listed / Notes: \_\_\_
- [ ] Update `apps/rhino-cli/internal/agents/sync_validator_test.go` and
      `cmd/agents_sync.integration_test.go`,
      `cmd/agents_validate_sync.integration_test.go`,
      `cmd/agents_validate_naming.integration_test.go` model assertions/fixtures.
      \_Date **/ Status:** / Files: as listed / Notes: \_\_\_
- [ ] Update `apps/rhino-cli/cmd/steps_common_test.go` step regex if any
      reference Z.ai model IDs.
      \_Date **/ Status:** / Files: apps/rhino-cli/cmd/steps_common_test.go / Notes: \_\_\_
- [ ] Replace `.opencode/opencode.json`:
  - `model: "opencode-go/minimax-m2.7"`
  - `small_model: "opencode-go/glm-5"`
  - Add `provider.opencode-go.options.apiKey: "{env:OPENCODE_GO_API_KEY}"`
  - Remove any Z.ai MCPs
    \_Date **/ Status:** / Files: .opencode/opencode.json / Notes: \_\_\_
- [ ] Update `governance/development/agents/model-selection.md` OpenCode
      Equivalents table to `opencode-go/*`.
      \_Date **/ Status:** / Files: governance/development/agents/model-selection.md / Notes: \_\_\_
- [ ] Update `.env.example` to document `OPENCODE_GO_API_KEY` env var.
      \_Date **/ Status:** / Files: .env.example / Notes: \_\_\_
- [ ] Run `npm run sync:claude-to-opencode`. Regenerates all
      `.opencode/agents/*.md` files with new model IDs.
      \_Date **/ Status:** / Files: .opencode/agents/\* / Notes: \_\_\_
- [ ] Run `nx run rhino-cli:test:unit`. Pass.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run `nx run rhino-cli:test:integration`. Pass.
      _Date **/ Status:** / Files: **/ Notes:**_

### 2C — Refactor + verify

- [ ] Run `npm run sync:claude-to-opencode` a second time — must be no-op.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Commit: `feat(rhino-cli,opencode): migrate OpenCode model provider to OpenCode Go`.
      _Date **/ Status:** / Files: **/ Notes:**_

## Phase 3 — W3: rhino-cli vendor-audit scanner

### 3A — Port (Red via copy)

- [ ] Create `apps/rhino-cli/internal/governance/governance_vendor_audit.go`
      from ose-public verbatim.
      \_Date **/ Status:** / Files: apps/rhino-cli/internal/governance/governance_vendor_audit.go / Notes: \_\_\_
- [ ] Create `apps/rhino-cli/internal/governance/governance_vendor_audit_test.go`
      from ose-public verbatim. Includes `\bSkills\b` test.
      \_Date **/ Status:** / Files: as above / Notes: \_\_\_
- [ ] Run `go test ./apps/rhino-cli/internal/governance/...`. Tests pass.
      _Date **/ Status:** / Files: **/ Notes:**_

### 3B — CLI binding (Green)

- [ ] Create `apps/rhino-cli/cmd/governance.go` (Cobra group).
      \_Date **/ Status:** / Files: apps/rhino-cli/cmd/governance.go / Notes: \_\_\_
- [ ] Create `apps/rhino-cli/cmd/governance_vendor_audit.go` (subcommand).
      \_Date **/ Status:** / Files: as above / Notes: \_\_\_
- [ ] Create `apps/rhino-cli/cmd/governance_vendor_audit_test.go`.
      \_Date **/ Status:** / Files: as above / Notes: \_\_\_
- [ ] Create `apps/rhino-cli/cmd/governance_vendor_audit.integration_test.go`.
      \_Date **/ Status:** / Files: as above / Notes: \_\_\_
- [ ] Update `apps/rhino-cli/cmd/steps_common_test.go` with new step constants.
      \_Date **/ Status:** / Files: apps/rhino-cli/cmd/steps_common_test.go / Notes: \_\_\_
- [ ] Update `apps/rhino-cli/cmd/root_test.go` to register the new `governance` Cobra group.
      _Date **/ Status:** / Files: apps/rhino-cli/cmd/root_test.go / Notes: \_\_ _
- [ ] Create `specs/apps/rhino/cli/gherkin/governance-vendor-audit.feature`.
      \_Date **/ Status:** / Files: as above / Notes: \_\_\_
- [ ] Run `nx run rhino-cli:test:unit`. Pass.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run `nx run rhino-cli:test:integration`. Pass.
      _Date **/ Status:** / Files: **/ Notes:**_

### 3C — Nx target wiring + docs

- [ ] Add `validate:vendor-audit` Nx target to `apps/rhino-cli/project.json`.
      Cacheable; inputs include `governance/**`. Command:
      `rhino-cli governance vendor-audit governance/`.
      \_Date **/ Status:** / Files: apps/rhino-cli/project.json / Notes: \_\_\_
- [ ] Update `apps/rhino-cli/README.md` with a "Governance vendor-audit" subsection.
      \_Date **/ Status:** / Files: apps/rhino-cli/README.md / Notes: \_\_\_
- [ ] Verify `nx run rhino-cli:validate:vendor-audit` runs (will return violations until W4).
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Commit: `feat(rhino-cli): add governance vendor-audit scanner with \\bSkills\\b term`.
      _Date **/ Status:** / Files: **/ Notes:**_

## Phase 4 — W4: Vendor-neutral governance

### 4A — Convention port

- [ ] Create `governance/conventions/structure/governance-vendor-independence.md`
      verbatim from ose-public, scoped for primer (single-repo).
      \_Date **/ Status:** / Files: governance/conventions/structure/governance-vendor-independence.md / Notes: \_\_\_
- [ ] Update `governance/conventions/structure/README.md` to link to the new convention.
      \_Date **/ Status:** / Files: governance/conventions/structure/README.md / Notes: \_\_\_
- [ ] Commit: `docs(governance): add governance-vendor-independence convention`.
      _Date **/ Status:** / Files: **/ Notes:**_

### 4B — AGENTS.md / CLAUDE.md restructure

- [ ] Rewrite `AGENTS.md` to be the canonical vendor-neutral root instruction file.
      Vendor-specific content goes inside ` ```binding-example ` fences.
      \_Date **/ Status:** / Files: AGENTS.md / Notes: \_\_\_
- [ ] Rewrite `CLAUDE.md` to a thin Claude Code shim. First non-frontmatter
      line: `@AGENTS.md`. Body retains only Claude-Code-specific notes inside
      `binding-example` fences.
      \_Date **/ Status:** / Files: CLAUDE.md / Notes: \_\_\_
- [ ] Run `rhino-cli governance vendor-audit AGENTS.md CLAUDE.md`. Must return 0 violations.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Commit: `refactor(governance): make AGENTS.md canonical, CLAUDE.md a thin shim`.
      _Date **/ Status:** / Files: **/ Notes:**_

### 4C — Governance prose remediation

- [ ] Run `nx run rhino-cli:validate:vendor-audit`. Capture full violation list to
      `local-temp/vendor-audit-baseline.txt`.
      \_Date **/ Status:** / Files: local-temp/vendor-audit-baseline.txt / Notes: \_\_\_
- [ ] Group violations by directory. Plan remediation order: principles →
      conventions → development → workflows → vision (if any).
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Remediate `governance/principles/` violations. Run scanner; commit.
      \_Date **/ Status:** / Files: governance/principles/\*\* / Notes: \_\_\_
- [ ] Remediate `governance/conventions/` violations. Run scanner; commit.
      \_Date **/ Status:** / Files: governance/conventions/\*\* / Notes: \_\_\_
- [ ] Remediate `governance/development/` violations including
      `governance/development/agents/ai-agents.md` (heavy lift). Commit.
      \_Date **/ Status:** / Files: governance/development/\*\* / Notes: \_\_\_
- [ ] Remediate `governance/workflows/` violations. Commit.
      \_Date **/ Status:** / Files: governance/workflows/\*\* / Notes: \_\_\_
- [ ] Remediate `governance/vision/` and `governance/README.md` if flagged. Commit.
      \_Date **/ Status:** / Files: as flagged / Notes: \_\_\_
- [ ] Update `governance/development/agents/model-selection.md` to use capability
      tiers as canonical vocabulary; vendor IDs only inside `binding-example` fences.
      \_Date **/ Status:** / Files: as above / Notes: \_\_\_
- [ ] Final sweep: `nx run rhino-cli:validate:vendor-audit` returns 0 violations.
      _Date **/ Status:** / Files: **/ Notes:**_

## Phase 5 — W5: Cross-vendor parity gate

### 5A — Agent ports

- [ ] Create `.claude/agents/repo-parity-checker.md` from ose-public verbatim.
      \_Date **/ Status:** / Files: .claude/agents/repo-parity-checker.md / Notes: \_\_\_
- [ ] Create `.claude/agents/repo-parity-fixer.md` from ose-public verbatim.
      \_Date **/ Status:** / Files: .claude/agents/repo-parity-fixer.md / Notes: \_\_\_
- [ ] Run `npm run sync:claude-to-opencode`. Verify
      `.opencode/agents/repo-parity-{checker,fixer}.md` are generated.
      \_Date **/ Status:** / Files: .opencode/agents/\* / Notes: \_\_\_
- [ ] Run `nx run rhino-cli:test:unit` and `nx run rhino-cli:test:integration`.
      Both green.
      _Date **/ Status:** / Files: **/ Notes:**_

### 5B — Workflow port

- [ ] Create `governance/workflows/repo/repo-cross-vendor-parity-quality-gate.md`
      verbatim from ose-public.
      \_Date **/ Status:** / Files: as above / Notes: \_\_\_
- [ ] Update `governance/workflows/repo/README.md` to link to the new workflow.
      \_Date **/ Status:** / Files: governance/workflows/repo/README.md / Notes: \_\_\_

### 5C — Nx target + pre-push wiring

- [ ] Add Nx target `validate:cross-vendor-parity` to `apps/rhino-cli/project.json`
      (or top-level `package.json`). Invokes the five invariants.
      \_Date **/ Status:** / Files: apps/rhino-cli/project.json / Notes: \_\_\_
- [ ] Add `nx affected -t validate:cross-vendor-parity` to `.husky/pre-push`.
      \_Date **/ Status:** / Files: .husky/pre-push / Notes: \_\_\_
- [ ] Run `nx run rhino-cli:validate:cross-vendor-parity`. Must return 0 findings.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run it a second time — must still return 0 (two consecutive zero passes).
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Commit: `feat(governance,rhino-cli): add cross-vendor parity gate (agents, workflow, Nx target, pre-push)`.
      _Date **/ Status:** / Files: **/ Notes:**_

## Phase 6 — W6: Plans convention refresh

- [ ] Replace the "Multi-File Structure" / "Single-File Structure" section in
      `governance/conventions/structure/plans.md` with ose-public's stricter wording.
      Five-doc DEFAULT, four named single-file criteria.
      \_Date **/ Status:** / Files: governance/conventions/structure/plans.md / Notes: \_\_\_
- [ ] Verify `nx run rhino-cli:validate:vendor-audit` still returns 0 violations.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Commit: `docs(plans): adopt ose-public's stricter five-doc default and four-criteria single-file rule`.
      _Date **/ Status:** / Files: **/ Notes:**_

## Phase 7 — W7: Worktree standard

- [ ] Create `governance/conventions/structure/worktree-path.md`. Adapt ose-public's
      version for primer: rule says default `.claude/worktrees/<name>/`, no override.
      Document gitignore + parallel-safety rationale.
      \_Date **/ Status:** / Files: governance/conventions/structure/worktree-path.md / Notes: \_\_\_
- [ ] Refresh `governance/development/workflow/worktree-setup.md` against ose-public.
      Restore `created:` frontmatter where missing; update cross-references.
      \_Date **/ Status:** / Files: governance/development/workflow/worktree-setup.md / Notes: \_\_\_
- [ ] Add a `## Worktrees` subsection to `AGENTS.md` linking to the new convention.
      \_Date **/ Status:** / Files: AGENTS.md / Notes: \_\_\_
- [ ] Add the same link from `CLAUDE.md`'s worktree subsection.
      \_Date **/ Status:** / Files: CLAUDE.md / Notes: \_\_\_
- [ ] Update `governance/conventions/structure/README.md` index to list `worktree-path.md`.
      \_Date **/ Status:** / Files: as above / Notes: \_\_\_
- [ ] Verify `nx run rhino-cli:validate:vendor-audit` still returns 0.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Commit: `docs(governance): add worktree-path convention; refresh worktree-setup`.
      _Date **/ Status:** / Files: **/ Notes:**_

## Phase 8 — W8: Plan + workflow refresh

- [ ] Refresh `governance/workflows/plan/plan-execution.md` against ose-public.
      Manually re-apply primer-specific phrasing (single-repo, no Scope A/B).
      \_Date **/ Status:** / Files: governance/workflows/plan/plan-execution.md / Notes: \_\_\_
- [ ] Refresh `governance/workflows/plan/plan-quality-gate.md` against ose-public.
      \_Date **/ Status:** / Files: governance/workflows/plan/plan-quality-gate.md / Notes: \_\_\_
- [ ] Refresh `governance/workflows/plan/README.md` against ose-public.
      \_Date **/ Status:** / Files: governance/workflows/plan/README.md / Notes: \_\_\_
- [ ] Create `governance/development/workflow/ci-monitoring.md` verbatim from ose-public.
      \_Date **/ Status:** / Files: as above / Notes: \_\_\_
- [ ] Create `governance/development/workflow/ci-post-push-verification.md` verbatim.
      \_Date **/ Status:** / Files: as above / Notes: \_\_\_
- [ ] Update `governance/development/workflow/README.md` index.
      \_Date **/ Status:** / Files: as above / Notes: \_\_\_
- [ ] Verify `nx run rhino-cli:validate:vendor-audit` returns 0.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Commit: `docs(governance,workflows): refresh plan workflows; add ci-monitoring + ci-post-push-verification`.
      _Date **/ Status:** / Files: **/ Notes:**_

## Phase 9 — W9: TDD convention

- [ ] Create `governance/development/workflow/test-driven-development.md` verbatim from ose-public.
      Adjust cross-reference paths if any reference paths primer doesn't have.
      \_Date **/ Status:** / Files: governance/development/workflow/test-driven-development.md / Notes: \_\_\_
- [ ] Add a one-line cross-reference to TDD convention from
      `governance/development/workflow/implementation.md` (Stage 1 description).
      \_Date **/ Status:** / Files: governance/development/workflow/implementation.md / Notes: \_\_\_
- [ ] Add a one-line cross-reference from
      `governance/workflows/plan/plan-execution.md` (Iron Rules section).
      \_Date **/ Status:** / Files: as above / Notes: \_\_\_
- [ ] Add a one-line cross-reference from
      `governance/conventions/structure/plans.md` (delivery-checklist authoring).
      \_Date **/ Status:** / Files: as above / Notes: \_\_\_
- [ ] Update `governance/development/workflow/README.md` index to include TDD.
      \_Date **/ Status:** / Files: as above / Notes: \_\_\_
- [ ] Verify `nx run rhino-cli:validate:vendor-audit` returns 0.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Commit: `docs(governance): adopt test-driven-development convention from ose-public`.
      _Date **/ Status:** / Files: **/ Notes:**_

## Phase 10 — Final validation, archive

- [ ] Run `nx affected -t typecheck lint test:quick spec-coverage`. All green.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run `nx run rhino-cli:test:unit` and `nx run rhino-cli:test:integration`. Both green.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run `nx run rhino-cli:validate:vendor-audit`. 0 violations.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run `nx run rhino-cli:validate:cross-vendor-parity` twice. 0 findings each run.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Run `npm run sync:claude-to-opencode`. No-op on clean tree.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Verify `ls .opencode/agent .opencode/skill 2>/dev/null` returns nothing.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Verify `cat .opencode/opencode.json | jq -r .model` returns `opencode-go/minimax-m2.7`.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Verify the nine Gherkin Feature groups in [prd.md](./prd.md) all pass.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Update `plans/in-progress/README.md` to remove this plan from active list.
      \_Date **/ Status:** / Files: plans/in-progress/README.md / Notes: \_\_\_
- [ ] Move plan folder from `plans/in-progress/` to `plans/done/`.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Update `plans/done/README.md` index.
      \_Date **/ Status:** / Files: plans/done/README.md / Notes: \_\_\_
- [ ] Final commit: `chore(plans): archive 2026-05-03__adopt-ose-public-vendor-neutrality-and-opencode-go`.
      _Date **/ Status:** / Files: **/ Notes:**_
- [ ] Push: `git push origin main` (or `git push origin HEAD:main` if from worktree branch).
      _Date **/ Status:** / Files: **/ Notes:**_

## Iron Rules

1. **No phase boundary skipping.** Each phase ends with the tree in a known-good state
   (tests green, scanner clean, sync no-op as applicable). Do not start phase N+1 until
   phase N is fully ticked.
2. **Red→Green→Refactor for code-touching ticks.** Every code change in W1–W3, W5 has a
   failing-test tick before the implementation tick.
3. **One Conventional-Commits commit per phase** unless the phase explicitly enumerates
   multiple commits (Phase 4 has five commits; Phase 1 has one).
4. **No skipping the second-run sync verification.** The "no-op on clean tree" guarantee
   is load-bearing — if a second sync run produces a diff, something else is wrong.
5. **No bypass of pre-push.** If `--no-verify` feels necessary, fix the root cause instead.
6. **Worktree, if used, pushes via `git push origin HEAD:main`** per Git Push Default
   Convention Standard 6. The worktree branch is isolation, not a feature branch.
7. **Direct push to `origin main`** per Git Push Default Convention Standards 1–2; no
   draft PR opened (user has not requested one).
8. **Adopt verbatim where possible.** ose-public is the source of truth; deviations
   require an explicit decision entry in [tech-docs.md](./tech-docs.md) and a note in
   the affected commit message.
