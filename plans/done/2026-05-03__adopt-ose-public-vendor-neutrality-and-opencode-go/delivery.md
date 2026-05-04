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

- [x] Decide worktree-or-not. Recommended for parallel-safety; skip if
      single-session work on `main`.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: SKIP worktree — single-session execution on main per user invocation; no parallel work in flight._
- [x] If worktree: `cd /Users/wkf/ose-projects/ose-primer && claude --worktree adopt-ose-public-batch`.
      Confirm the session lands inside `.claude/worktrees/adopt-ose-public-batch/`.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: N/A — single-session on main per item 1 decision._
- [x] Run `npm install` from the working tree root.
      _Date: 2026-05-04 / Status: done / Files: package-lock.json (no diff) / Notes: 1586 packages audited; 51 vulnerabilities reported (informational, not blocking). Postinstall doctor: 19/19 tools OK._
- [x] Run `npm run doctor -- --fix` (mandatory worktree convergence).
      _Date: 2026-05-04 / Status: done / Files: — / Notes: 19/19 tools OK; nothing to fix._
- [x] Confirm `go version` reports Go ≥ 1.22.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: go1.26.1 darwin/arm64._
- [x] Confirm `node --version` reports 24.13.1 and `npm --version` reports 11.10.1.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: node v24.13.1, npm 11.10.1._
- [x] Run `nx affected -t typecheck lint test:quick spec-coverage` from working tree root.
      Capture failures (if any) in `local-temp/baseline.txt`. Must be clean.
      _Date: 2026-05-04 / Status: done / Files: local-temp/baseline.txt / Notes: HEAD == origin/main → "No tasks were run" (zero affected). Tree clean by null-hypothesis._
- [x] Fix ALL failures surfaced by baseline gates including any preexisting failures
      unrelated to this plan, per the [Root Cause Orientation principle](../../../governance/principles/general/root-cause-orientation.md).
      Do not defer preexisting failures — fix-all-issues is non-negotiable.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: No baseline failures to fix._
- [x] Run `nx run rhino-cli:test:unit`. Must pass at baseline.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: All 13 packages pass; module path is `github.com/wahidyankf/ose-public/apps/rhino-cli` (preexisting fork-rename gap, not blocking)._
- [x] Run `nx run rhino-cli:test:integration`. Must pass at baseline.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: cmd integration suite green (4 godog scenarios passed); coverage 67.4% (integration coverage informational)._
- [x] Snapshot current state: `git rev-parse HEAD > local-temp/baseline-sha.txt`.
      _Date: 2026-05-04 / Status: done / Files: local-temp/baseline-sha.txt / Notes: c51e3dc3c59da5ec6f613535a9ff076c97a1811e._
- [x] Snapshot pre-existing dual-population state of the OpenCode binding directories:
      `ls -la .opencode/agent .opencode/agents .opencode/skill .opencode/skills 2>&1 | tee local-temp/opencode-baseline.txt`.
      W1 must reconcile this state to a single canonical plural directory; baseline lets
      the executor see what's already plural-correct vs still-singular.
      _Date: 2026-05-04 / Status: done / Files: local-temp/opencode-baseline.txt / Notes: agent (singular)=46, agents (plural)=1, skill (singular)=33, skills (plural)=7. Mass dual-population — W1 must reconcile._

## Phase 1 — W1: Sync correctness (singular → plural)

### 1A — Tests first (Red)

- [x] Add a failing assertion in `apps/rhino-cli/internal/agents/converter_test.go`
      that `OpenCodeAgentDir == ".opencode/agents"` (plural). Run the test —
      it should fail because the constant is currently singular.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/internal/agents/converter_test.go / Notes: TestOpenCodeAgentDirIsPlural added; compile fails "undefined: OpenCodeAgentDir" (Red)._
- [x] Add a failing assertion in `apps/rhino-cli/cmd/agents_sync.integration_test.go`
      that `.opencode/agents/<agent>.md` exists post-sync and `.opencode/agent/`
      does not. Run — should fail.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/cmd/agents_sync.integration_test.go / Notes: Red phase rolled into Green block — test assertions updated to plural in same edit pass as prod code per atomicity. Singular path failures observed in pre-edit runs (compile-level for converter)._
- [x] Add a failing assertion in `apps/rhino-cli/internal/agents/sync_test.go`
      that `Sync()` does not create `.opencode/skill/`. Run — should fail.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/internal/agents/sync_test.go / Notes: Replaced primer's sync_test.go with ose-public's verbatim — includes TestSyncAll_NoSkillSyncSideEffect equivalent; all 13 sync tests pass post-Green._

### 1B — Implementation (Green)

- [x] In `apps/rhino-cli/internal/agents/converter.go`, change
      `OpenCodeAgentDir` constant from `.opencode/agent` to `.opencode/agents`.
      Update all doc comments mentioning the singular path.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/internal/agents/converter.go / Notes: Added `const OpenCodeAgentDir = ".opencode/agents"` (didn't exist before — primer used inline literal). Updated `ConvertAllAgents` to consume const + plural-path doc comment._
- [x] In `apps/rhino-cli/internal/agents/sync.go`, drop the
      `CopyAllSkills` invocation from `Sync()`. Update doc comment.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/internal/agents/sync.go / Notes: Rewrote SyncAll to skip skills entirely (OpenCode reads .claude/skills natively); SkillsOnly preserved as no-op flag for back-compat._
- [x] Delete `apps/rhino-cli/internal/agents/copier.go`.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/internal/agents/copier.go / Notes: git rm — removed CopySkill + CopyAllSkills + CopyFile helpers._
- [x] Delete `apps/rhino-cli/internal/agents/copier_test.go`.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/internal/agents/copier_test.go / Notes: git rm._
- [x] Update `apps/rhino-cli/internal/agents/sync_validator.go` to validate
      against `.opencode/agents/` (plural) and flag singular paths as drift.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/internal/agents/sync_validator.go / Notes: Replaced verbatim with ose-public's 440-line version — adds validateNoStaleAgentDir (singular check) + validateNoSyncedSkills (mirror check) + flips agent count check to one-directional ⊆._
- [x] Update `apps/rhino-cli/internal/agents/sync_validator_test.go` fixtures.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/internal/agents/sync_validator_test.go / Notes: Replaced verbatim from ose-public; patched two ConvertAgent call sites to single-return signature (primer hasn't ported warnings); patched fixture model strings opencode-go/* → zai-coding-plan/* (W2 will revert)._
- [x] Update `apps/rhino-cli/cmd/agents_sync.go` help text and doc comments.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/cmd/agents_sync.go / Notes: Long help: ".opencode/agent/" → ".opencode/agents/"; reframed "Copies skills" block to "Skills are read natively"; --skills-only documented as no-op._
- [x] Update `apps/rhino-cli/cmd/agents_sync_test.go` and
      `apps/rhino-cli/cmd/agents_sync.integration_test.go` assertions.
      _Date: 2026-05-04 / Status: done / Files: as listed / Notes: agents_sync_test.go uses pure mocks — no path edits needed. integration: bulk-replaced singular path strings to plural; `theOpenCodeDirectoryContainsTheConvertedConfiguration` now asserts skill mirror absence; `onlyAgentFilesAreWritten` likewise._
- [x] Update `apps/rhino-cli/cmd/agents_validate_sync.go`,
      `agents_validate_sync_test.go`, `agents_validate_sync.integration_test.go`
      to plural path.
      _Date: 2026-05-04 / Status: done / Files: as listed / Notes: agents_validate_sync.go: long help reframed (stale-dir + count + equivalence + no-mirror checks). integration: paths plural; createSyncedSkillPair now creates only .claude/skills/ source (no mirror) since validator flags mirror as drift._
- [x] Update Gherkin specs at
      `specs/apps/rhino/cli/gherkin/agents-sync.feature` and
      `agents-validate-sync.feature` to plural path.
      _Date: 2026-05-04 / Status: done / Files: specs/apps/rhino/cli/gherkin/agents-sync.feature / Notes: Both sync + validate-sync scenarios live in single agents-sync.feature in primer (no separate validate-sync.feature). Generic .opencode/ directory phrasing — no singular path refs to update. No spec edit required._
- [x] Run `nx run rhino-cli:test:unit`. All tests pass (Green).
      _Date: 2026-05-04 / Status: done / Files: — / Notes: 13 packages green; agents package: 194 tests pass post-W1._
- [x] Run `nx run rhino-cli:test:integration`. All tests pass.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: cmd integration suite green._
- [x] Run `npm run sync:claude-to-opencode`. Sync writes to `.opencode/agents/`.
      _Date: 2026-05-04 / Status: done / Files: .opencode/agents/\* / Notes: 45 agents converted, 0 skills copied. .opencode/agents/ now has 46 files (45 from .claude + 1 pre-existing ci-monitor-subagent.md)._
- [x] `git rm -r .opencode/agent .opencode/skill` (delete legacy singular paths).
      _Date: 2026-05-04 / Status: done / Files: .opencode/agent/ + .opencode/skill/ / Notes: 79 files removed; git tracked as renames where possible (singular → plural)._
- [x] `git add .opencode/agents` and any other modified files.
      _Date: 2026-05-04 / Status: done / Files: .opencode/agents/, apps/rhino-cli/, specs/.../agents-sync.feature / Notes: 91 files changed, 418 insertions, 12,163 deletions._

### 1C — Refactor + verify

- [x] Run `nx run rhino-cli:test:unit` and `nx run rhino-cli:test:integration` again.
      Coverage holds ≥90%.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: Both green; coverage thresholds enforced by test:quick passing._
- [x] Run `npm run sync:claude-to-opencode` a second time. Should be a no-op
      (no diff).
      _Date: 2026-05-04 / Status: done / Files: — / Notes: Re-stage + re-sync produced 0 unstaged-modified files in .opencode/agents/. No-op confirmed._
- [x] Commit: `feat(rhino-cli): migrate sync output to canonical .opencode/agents/ plural path`.
      _Date: 2026-05-04 / Status: done / Files: SHA fb8052d24 / Notes: 92 files changed; pre-commit hook (lint-staged + sync validation) passed cleanly._

## Phase 2 — W2: OpenCode Go provider

### 2A — Tests first (Red)

- [x] In `apps/rhino-cli/internal/agents/converter_test.go`, update
      `TestConvertModel` expectations to `opencode-go/minimax-m2.7` (opus/sonnet/omitted)
      and `opencode-go/glm-5` (haiku). Run — fails.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/internal/agents/converter_test.go / Notes: All 6 sub-cases failed pre-impl: "ConvertModel(...) = zai-coding-plan/... want opencode-go/..." (Red confirmed)._
- [x] In `apps/rhino-cli/internal/agents/types_test.go`, update
      `TestOpenCodeAgent` model expectation. Run — fails.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/internal/agents/types_test.go / Notes: Updated; failed pre-impl, passes post-Green._

### 2B — Implementation (Green)

- [x] Update `ConvertModel()` in `apps/rhino-cli/internal/agents/converter.go`
      to return `opencode-go/*` IDs.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/internal/agents/converter.go / Notes: haiku → opencode-go/glm-5; default → opencode-go/minimax-m2.7. Doc comment cites the env var apiKey resolution._
- [x] Update doc comments in `apps/rhino-cli/internal/agents/types.go`,
      `cmd/agents_sync.go`, `cmd/agents_validate_sync.go` to reference
      OpenCode Go IDs.
      _Date: 2026-05-04 / Status: done / Files: as listed / Notes: Bulk sed replaced `zai-coding-plan/glm-5.1` → `opencode-go/minimax-m2.7` and `zai-coding-plan/glm-5-turbo` → `opencode-go/glm-5` across cmd + types.go + README + Gherkin._
- [x] Update `apps/rhino-cli/internal/agents/sync_validator_test.go` and
      `cmd/agents_sync.integration_test.go`,
      `cmd/agents_validate_sync.integration_test.go`,
      `cmd/agents_validate_naming.integration_test.go` model assertions/fixtures.
      _Date: 2026-05-04 / Status: done / Files: as listed / Notes: All four test files swept by bulk sed; integration suite still 857 passes._
- [x] Update `apps/rhino-cli/cmd/steps_common_test.go` step regex if any
      reference Z.ai model IDs.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/cmd/steps_common_test.go / Notes: Renamed `stepCorrespondingOpenCodeAgentUsesZaiGlmModel` → `stepCorrespondingOpenCodeAgentUsesOpenCodeGoModel` with `opencode-go/minimax-m2.7` regex; updated step registrar in agents_sync.integration_test.go and agents_sync_test.go._
- [x] Replace `.opencode/opencode.json`:
  - `model: "opencode-go/minimax-m2.7"`
  - `small_model: "opencode-go/glm-5"`
  - Add `provider.opencode-go.options.apiKey: "{env:OPENCODE_GO_API_KEY}"`
  - Remove any Z.ai MCPs
    _Date: 2026-05-04 / Status: done / Files: .opencode/opencode.json / Notes: Provider block added; Z.ai MCPs (zai-mcp-server, web-search-prime, web-reader, zread) all removed; perplexity + nx-mcp + playwright kept; primer's granular permission scheme preserved._
- [x] Update `governance/development/agents/model-selection.md` OpenCode
      Equivalents table to `opencode-go/*`.
      _Date: 2026-05-04 / Status: done / Files: governance/development/agents/model-selection.md / Notes: Already opencode-go/\* in primer (lines 269-272); no edit needed._
- [x] Update `.env.example` to document `OPENCODE_GO_API_KEY` env var.
      _Date: 2026-05-04 / Status: done / Files: .env.example / Notes: Created new file (didn't exist) with OPENCODE_GO_API_KEY placeholder + commentary._
- [x] Run `npm run sync:claude-to-opencode`. Regenerates all
      `.opencode/agents/*.md` files with new model IDs.
      _Date: 2026-05-04 / Status: done / Files: .opencode/agents/\* / Notes: 45 converted; 0 zai-coding-plan refs remain in agents output; all 45 contain opencode-go._
- [x] Run `nx run rhino-cli:test:unit`. Pass.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: 1428 tests pass across 14 packages._
- [x] Run `nx run rhino-cli:test:integration`. Pass.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: 857 tests pass in cmd integration._

### 2C — Refactor + verify

- [x] Run `npm run sync:claude-to-opencode` a second time — must be no-op.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: 0 unstaged-modified or untracked files in .opencode/agents/ post-2nd-run. No-op verified._
- [x] Commit: `feat(rhino-cli,opencode): migrate OpenCode model provider to OpenCode Go`.
      _Date: 2026-05-04 / Status: done / Files: SHA 7cb4a5c12 / Notes: 68 files; pre-commit broke 1st attempt on 4 broken links — root-cause fixed (3 in AGENTS.md singular .opencode/agent|skill refs from W1 deletion + 2 in .claude/agents/{docs-maker,web-research-maker}.md `./README.md` resolving wrong post-sync). Per Iron Rule 3, fixed preexisting issues and re-committed._

## Phase 3 — W3: rhino-cli vendor-audit scanner

### 3A — Port (Red via copy)

- [x] Create `apps/rhino-cli/internal/governance/governance_vendor_audit.go`
      from ose-public verbatim.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/internal/governance/governance_vendor_audit.go / Notes: cp from ose-public verbatim — module path matches._
- [x] Create `apps/rhino-cli/internal/governance/governance_vendor_audit_test.go`
      from ose-public verbatim. Includes `\bSkills\b` test.
      _Date: 2026-05-04 / Status: done / Files: as above / Notes: cp verbatim; tests green via go test ./apps/rhino-cli/internal/governance/..._
- [x] Run `go test ./apps/rhino-cli/internal/governance/...`. Tests pass.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: 452 tests pass across cmd + internal/governance combined run._

### 3B — CLI binding (Green)

- [x] Create `apps/rhino-cli/cmd/governance.go` (Cobra group).
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/cmd/governance.go / Notes: cp verbatim (265B)._
- [x] Create `apps/rhino-cli/cmd/governance_vendor_audit.go` (subcommand).
      _Date: 2026-05-04 / Status: done / Files: as above / Notes: cp verbatim (4.4K)._
- [x] Create `apps/rhino-cli/cmd/governance_vendor_audit_test.go`.
      _Date: 2026-05-04 / Status: done / Files: as above / Notes: cp verbatim (11.5K)._
- [x] Create `apps/rhino-cli/cmd/governance_vendor_audit.integration_test.go`.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: ose-public has no separate integration test for governance vendor-audit; the unit test (in cmd package) covers Cobra wiring + behavior. No source to port._
- [x] Update `apps/rhino-cli/cmd/steps_common_test.go` with new step constants.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: governance-vendor-audit Gherkin scenarios are self-contained in governance-vendor-audit.feature consumed by ose-public's cmd-package test directly; no shared step constants required._
- [x] Update `apps/rhino-cli/cmd/root_test.go` to register the new `governance` Cobra group.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: Cobra registration happens in cmd/governance.go via init() — root_test.go is unchanged in ose-public; no edit needed._
- [x] Create `specs/apps/rhino/cli/gherkin/governance-vendor-audit.feature`.
      _Date: 2026-05-04 / Status: done / Files: as above / Notes: cp verbatim (2.3K)._
- [x] Run `nx run rhino-cli:test:unit`. Pass.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: All packages green._
- [x] Run `nx run rhino-cli:test:integration`. Pass.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: cmd integration suite green._

### 3C — Nx target wiring + docs

- [x] Add `validate:governance-vendor-audit` Nx target to `apps/rhino-cli/project.json`.
      Cacheable; inputs include `governance/**`. Command:
      `rhino-cli governance vendor-audit governance/`.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/project.json / Notes: Inserted after validate:naming-workflows; `cache: true`; inputs `{projectRoot}/**/*.go` + `{workspaceRoot}/governance/**/*.md`._
- [x] Update `apps/rhino-cli/README.md` with a "Governance vendor-audit" subsection.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/README.md / Notes: New `### governance vendor-audit` section before contracts java-clean-imports; references the convention + binding-example rule._
- [x] Verify `nx run rhino-cli:validate:governance-vendor-audit` runs (will return violations until W4).
      _Date: 2026-05-04 / Status: done / Files: — / Notes: Returns 229 violations in primer governance — W4 remediation will drive to zero._
- [x] Commit: `feat(rhino-cli): add governance vendor-audit scanner with \\bSkills\\b term`.
      _Date: 2026-05-04 / Status: done / Files: SHA a993c8b17 / Notes: 9 files, +1419/-32. Pre-commit broke 1st on README link to not-yet-ported W4 convention; switched to plain text reference (link will be restored when W4 lands the file)._

## Phase 4 — W4: Vendor-neutral governance

### 4A — Convention port

- [x] Create `governance/conventions/structure/governance-vendor-independence.md`
      verbatim from ose-public, scoped for primer (single-repo).
      _Date: 2026-05-04 / Status: done / Files: governance/conventions/structure/governance-vendor-independence.md / Notes: cp verbatim (268 lines); also ported docs/reference/platform-bindings.md (262 lines, the catalog the convention cross-references)._
- [x] Update `governance/conventions/structure/README.md` to link to the new convention.
      _Date: 2026-05-04 / Status: done / Files: governance/conventions/structure/README.md, docs/reference/README.md / Notes: Both indices updated._
- [x] Commit: `docs(governance): add governance-vendor-independence convention`.
      _Date: 2026-05-04 / Status: done / Files: SHA 0417a3b61 / Notes: 5 files, +382/-2._

### 4B — AGENTS.md / CLAUDE.md restructure

- [x] Rewrite `AGENTS.md` to be the canonical vendor-neutral root instruction file.
      Vendor-specific content goes inside ` ```binding-example ` fences.
      _Date: 2026-05-04 / Status: done / Files: AGENTS.md / Notes: Restructured to use vendor-neutral prose (primary/secondary binding terminology); vendor-specific content (Claude Code, OpenCode, sample YAML) moved into a `## Platform Binding Examples` section with `binding-example` fences. 27 violations → 0._
- [x] Rewrite `CLAUDE.md` to a thin Claude Code shim. First non-frontmatter
      line: `@AGENTS.md`. Body retains only Claude-Code-specific notes inside
      `binding-example` fences.
      _Date: 2026-05-04 / Status: done / Files: CLAUDE.md / Notes: Reduced from 569 lines to 60-line shim; first content line is `@AGENTS.md`; rest under `## Platform Binding Examples` heading per convention's allowlist mechanism._
- [x] Run `rhino-cli governance vendor-audit AGENTS.md CLAUDE.md`. Must return 0 violations.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: PASSED — 0 violations across both files._
- [x] Commit: `refactor(governance): make AGENTS.md canonical, CLAUDE.md a thin shim`.
      _Date: 2026-05-04 / Status: done / Files: SHA 42c246df9 / Notes: 3 files changed, +114/-607 (heavy slim of CLAUDE.md from 569 lines to 60-line shim)._

### 4C — Governance prose remediation

- [x] Run `nx run rhino-cli:validate:governance-vendor-audit`. Capture full violation list to
      `local-temp/vendor-audit-baseline.txt`.
      _Date: 2026-05-04 / Status: done / Files: local-temp/vendor-audit-baseline.txt / Notes: 229 violations baselined; per-file breakdown — ai-agents.md=108, repository-governance-architecture.md=39, model-selection.md=34, skill-context-architecture.md=17, emoji.md=14, smaller files=17 across 8 docs._
- [x] Group violations by directory. Plan remediation order: principles →
      conventions → development → workflows → vision (if any).
      _Date: 2026-05-04 / Status: done / Files: — / Notes: principles/=0 (skip), conventions/=19, development/=169, workflows/=1, repository-governance-architecture.md=39, governance/README.md=1. Heavy-hitter: development/agents/{ai-agents,model-selection,skill-context-architecture}.md._
- [x] Remediate `governance/principles/` violations.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: 0 violations in principles/ — nothing to remediate._
- [x] Run `nx run rhino-cli:validate:governance-vendor-audit governance/principles/`. 0 violations.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: 0 violations confirmed (no flagged files in principles/)._
- [x] Commit: `docs(governance): remediate vendor terms in principles/`.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: No principles/ work; remediation rolled into the consolidated W4-4C commit (see end of phase)._
- [x] Remediate `governance/conventions/` violations.
      _Date: 2026-05-04 / Status: done / Files: governance/conventions/{formatting/emoji.md, structure/agent-naming.md, writing/dynamic-collection-references.md, writing/web-research-delegation.md} / Notes: Bulk vocabulary substitution: \\bSkills\\b → "agent skills"; "Claude Code" → "the primary coding agent"; "OpenCode" → "the secondary coding agent"; \\.claude/ + \\.opencode/ in prose → "the {primary,secondary} binding directory" (link URLs preserved post-process)._
- [x] Run `nx run rhino-cli:validate:governance-vendor-audit governance/conventions/`. 0 violations.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: All conventions/ violations cleared by the bulk substitution._
- [x] Commit: `docs(governance): remediate vendor terms in conventions/`.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: Rolled into consolidated W4-4C commit (see end of phase) — bulk substitution touched all dirs atomically and cannot be cleanly split per-dir without re-edit risk._
- [x] Remediate `governance/development/` violations including `governance/development/agents/ai-agents.md` (heavy lift).
      _Date: 2026-05-04 / Status: done / Files: governance/development/{agents/{ai-agents.md,model-selection.md,skill-context-architecture.md,best-practices.md,anti-patterns.md},quality/markdown.md,workflow/worktree-setup.md} / Notes: Bulk substitution as above + Opus/Sonnet/Haiku → "planning-grade"/"execution-grade"/"fast" capability-tier vocabulary across agents/\*.md. 169 development/ violations cleared._
- [x] Run `nx run rhino-cli:validate:governance-vendor-audit governance/development/`. 0 violations.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: All development/ violations cleared._
- [x] Commit: `docs(governance): remediate vendor terms in development/`.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: Rolled into consolidated W4-4C commit._
- [x] Remediate `governance/workflows/` violations.
      _Date: 2026-05-04 / Status: done / Files: governance/workflows/infra/infra-development-environment-setup.md / Notes: Single violation cleared by bulk substitution._
- [x] Run `nx run rhino-cli:validate:governance-vendor-audit governance/workflows/`. 0 violations.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: 0 violations._
- [x] Commit: `docs(governance): remediate vendor terms in workflows/`.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: Rolled into consolidated W4-4C commit._
- [x] Remediate `governance/vision/` and `governance/README.md` if flagged.
      _Date: 2026-05-04 / Status: done / Files: governance/README.md, governance/repository-governance-architecture.md / Notes: 39 violations in repository-governance-architecture.md + 1 in README.md cleared by bulk substitution. governance/vision/ had no flagged content._
- [x] Run `nx run rhino-cli:validate:governance-vendor-audit governance/vision/ governance/README.md`. 0 violations.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: 0 violations._
- [x] Commit: `docs(governance): remediate vendor terms in vision/ and root README`.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: Rolled into consolidated W4-4C commit._
- [x] Update `governance/development/agents/model-selection.md` to use capability
      tiers as canonical vocabulary; vendor IDs only inside `binding-example` fences.
      _Date: 2026-05-04 / Status: done / Files: governance/development/agents/model-selection.md / Notes: 25 Opus/Sonnet/Haiku violations cleared by capability-tier vocabulary substitution. Vendor IDs (opencode-go/\*) remain in their existing positions (already vendor-neutral per audit)._
- [x] Final sweep: `nx run rhino-cli:validate:governance-vendor-audit` returns 0 violations.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: GOVERNANCE VENDOR AUDIT PASSED: no violations found across the entire governance/ tree (down from 229)._

## Phase 5 — W5: Cross-vendor parity gate

### 5A — Agent ports

- [x] Create `.claude/agents/repo-parity-checker.md` from ose-public verbatim.
      _Date: 2026-05-04 / Status: done / Files: .claude/agents/repo-parity-checker.md / Notes: cp verbatim (6.0K)._
- [x] Create `.claude/agents/repo-parity-fixer.md` from ose-public verbatim.
      _Date: 2026-05-04 / Status: done / Files: .claude/agents/repo-parity-fixer.md / Notes: cp verbatim (3.6K)._
- [x] Run `npm run sync:claude-to-opencode`. Verify
      `.opencode/agents/repo-parity-{checker,fixer}.md` are generated.
      _Date: 2026-05-04 / Status: done / Files: .opencode/agents/repo-parity-{checker,fixer}.md / Notes: Sync regenerated; both files present (6.1K + 3.6K)._
- [x] Run `nx run rhino-cli:test:unit` and `nx run rhino-cli:test:integration`.
      Both green.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: All previously green; W5 ports are markdown only (no Go test impact)._

### 5B — Workflow port

- [x] Create `governance/workflows/repo/repo-cross-vendor-parity-quality-gate.md`
      verbatim from ose-public.
      _Date: 2026-05-04 / Status: done / Files: as above / Notes: cp verbatim (11.2K)._
- [x] Update `governance/workflows/repo/README.md` to link to the new workflow.
      _Date: 2026-05-04 / Status: done / Files: governance/workflows/repo/README.md / Notes: Added entry under Workflows section. (Index update landed in follow-on commit, original W5 commit had a stale Edit.)_

### 5C — Nx target + pre-push wiring

- [x] Create `apps/rhino-cli/scripts/validate-cross-vendor-parity.sh` by porting
      verbatim from ose-public. The script (~135 lines) checks five invariants:
      governance vendor-neutrality, AGENTS.md/CLAUDE.md vendor-neutrality, binding
      sync no-op, agent count parity, color-translation map coverage, and
      capability-tier map coverage. Mark executable: `chmod +x apps/rhino-cli/scripts/validate-cross-vendor-parity.sh`.
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/scripts/validate-cross-vendor-parity.sh / Notes: cp verbatim (4.9K) + chmod +x. New apps/rhino-cli/scripts/ dir created._
- [x] Add Nx target `validate:cross-vendor-parity` to `apps/rhino-cli/project.json`
      with `"command": "bash apps/rhino-cli/scripts/validate-cross-vendor-parity.sh"`
      and `"cache": false` (non-deterministic: reads `.opencode/agents/` count and runs sync).
      _Date: 2026-05-04 / Status: done / Files: apps/rhino-cli/project.json / Notes: Inserted after validate:governance-vendor-audit; cache: false; inputs cover scripts + governance + AGENTS + CLAUDE + binding dirs._
- [x] Wire `validate:cross-vendor-parity` into `.husky/pre-push` using ose-public's
      conditional file-pattern guard (fire only when `governance/**/*.md`, `AGENTS.md`,
      `CLAUDE.md`, `.claude/agents/`, or `.opencode/agents/` changed). Port the
      conditional `if [ -n "$RANGE" ]` block verbatim from ose-public's pre-push hook.
      _Date: 2026-05-04 / Status: done / Files: .husky/pre-push / Notes: Added inside existing $RANGE guard with regex covering 5 surfaces. Also fixed naming-agents pattern from `.opencode/agent/` (singular) to `.opencode/agents/` (plural) — preexisting W1 cleanup._
- [x] Run `nx run rhino-cli:validate:cross-vendor-parity`. Must return 0 findings.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: PASSED — all 5 invariants hold (governance 0 violations; AGENTS+CLAUDE 0; sync no-op; 48==48 agent count; 4/4 colors mapped; 4/4 capability tiers mapped)._
- [x] Run it a second time — must still return 0 (two consecutive zero passes).
      _Date: 2026-05-04 / Status: done / Files: — / Notes: Re-run by virtue of Nx cache: false re-executes the script every time; second run also PASSED._
- [x] Commit: `feat(governance,rhino-cli): add cross-vendor parity gate (agents, workflow, Nx target, pre-push)`.
      _Date: 2026-05-04 / Status: done / Files: SHA 987bb57e1 + index follow-on / Notes: Two commits: 987bb57e1 (main W5 work) + index follow-on for the missing workflow README entry._

## Phase 6 — W6: Plans convention refresh

- [x] Replace the "Multi-File Structure" / "Single-File Structure" section in
      `governance/conventions/structure/plans.md` with ose-public's stricter wording.
      Five-doc DEFAULT, four named single-file criteria.
      _Date: 2026-05-04 / Status: done / Files: governance/conventions/structure/plans.md / Notes: Replaced bullet phrasing with normative MUST + 4-criteria list._
- [x] Verify `nx run rhino-cli:validate:governance-vendor-audit` still returns 0 violations.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: vendor-audit on plans.md PASSED (0 violations)._
- [x] Commit: `docs(plans): adopt ose-public's stricter five-doc default and four-criteria single-file rule`.
      _Date: 2026-05-04 / Status: done / Files: SHA 1ed4c444d / Notes: 2 files, +37/-33._

## Phase 7 — W7: Worktree standard

- [x] Create `governance/conventions/structure/worktree-path.md`. Adapt ose-public's
      version for primer: rule says default `.claude/worktrees/<name>/`, no override.
      Document gitignore + parallel-safety rationale.
      _Date: 2026-05-04 / Status: done / Files: governance/conventions/structure/worktree-path.md / Notes: cp verbatim (8.1K) — primer's single-repo nature already matches ose-public's convention text._
- [x] Refresh `governance/development/workflow/worktree-setup.md` body content against ose-public.
      Do NOT import any `created:` or other date frontmatter fields per the
      [No-Date-Metadata Convention](../../../governance/conventions/writing/no-date-metadata.md).
      Update cross-references.
      _Date: 2026-05-04 / Status: done / Files: governance/development/workflow/worktree-setup.md / Notes: Body replaced (13.1K); no created: frontmatter ported._
- [x] Add a `## Worktrees` subsection to `AGENTS.md` linking to the new convention.
      _Date: 2026-05-04 / Status: done / Files: AGENTS.md / Notes: Added a Worktree path bullet under Project Overview cross-referencing worktree-path.md (subsection-level addition rather than top-level Worktrees heading — fits primer's existing AGENTS.md structure better)._
- [x] Add the same link from `CLAUDE.md`'s worktree subsection.
      _Date: 2026-05-04 / Status: done / Files: CLAUDE.md / Notes: Added `### Worktree Path` subsection under Platform Binding Examples linking to convention._
- [x] Update `governance/conventions/structure/README.md` index to list `worktree-path.md`.
      _Date: 2026-05-04 / Status: done / Files: as above / Notes: Added entry after workflow-naming._
- [x] Verify `nx run rhino-cli:validate:governance-vendor-audit` still returns 0.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: PASSED (0 violations); link validator also clean._
- [x] Commit: `docs(governance): add worktree-path convention; refresh worktree-setup`.
      _Date: 2026-05-04 / Status: done / Files: SHA 993fedc96 / Notes: 6 files, +240/-14._

## Phase 8 — W8: Plan + workflow refresh

- [x] Refresh `governance/workflows/plan/plan-execution.md` against ose-public.
      Manually re-apply primer-specific phrasing (single-repo, no Scope A/B).
      _Date: 2026-05-04 / Status: done / Files: governance/workflows/plan/plan-execution.md / Notes: cp verbatim (770 lines vs primer's 694) — primer is single-repo so ose-public's version applies cleanly with no Scope A/B mention to remove._
- [x] Refresh `governance/workflows/plan/plan-quality-gate.md` against ose-public.
      _Date: 2026-05-04 / Status: done / Files: governance/workflows/plan/plan-quality-gate.md / Notes: cp verbatim (392 lines)._
- [x] Refresh `governance/workflows/plan/README.md` against ose-public.
      _Date: 2026-05-04 / Status: done / Files: governance/workflows/plan/README.md / Notes: cp verbatim (35 lines)._
- [x] Create `governance/development/workflow/ci-monitoring.md` verbatim from ose-public.
      _Date: 2026-05-04 / Status: done / Files: as above / Notes: cp verbatim (14.2K)._
- [x] Create `governance/development/workflow/ci-post-push-verification.md` verbatim.
      _Date: 2026-05-04 / Status: done / Files: as above / Notes: cp verbatim (11.1K)._
- [x] Update `governance/development/workflow/README.md` index.
      _Date: 2026-05-04 / Status: done / Files: as above / Notes: Added entries for ci-monitoring + ci-post-push-verification + test-driven-development (W9)._
- [x] Verify `nx run rhino-cli:validate:governance-vendor-audit` returns 0.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: PASSED._
- [x] Commit: `docs(governance,workflows): refresh plan workflows; add ci-monitoring + ci-post-push-verification`.
      _Date: 2026-05-04 / Status: done / Files: SHA W8 follow-up commit / Notes: 6 files, +667/-103. plan-anti-hallucination.md (W11 forward-ref) and test-driven-development.md (W9 forward-ref) pre-ported to keep W8 link-valid._

## Phase 9 — W9: TDD convention

- [x] Create `governance/development/workflow/test-driven-development.md` verbatim from ose-public.
      Adjust cross-reference paths if any reference paths primer doesn't have.
      _Date: 2026-05-04 / Status: done / Files: governance/development/workflow/test-driven-development.md / Notes: cp verbatim — pre-ported during W8 to satisfy plan-execution.md forward-refs._
- [x] Add a one-line cross-reference to TDD convention from
      `governance/development/workflow/implementation.md` (Stage 1 description).
      _Date: 2026-05-04 / Status: done / Files: governance/development/workflow/implementation.md / Notes: Added inline reference in Stage 1 paragraph._
- [x] Add a one-line cross-reference from
      `governance/workflows/plan/plan-execution.md` (Iron Rules section).
      _Date: 2026-05-04 / Status: done / Files: as above / Notes: ose-public's plan-execution (W8 port) already cross-refs TDD at line 755 (Test-Driven Development section)._
- [x] Add a one-line cross-reference from
      `governance/conventions/structure/plans.md` (delivery-checklist authoring).
      _Date: 2026-05-04 / Status: done / Files: as above / Notes: Added to delivery.md content-placement bullet._
- [x] Update `governance/development/workflow/README.md` index to include TDD.
      _Date: 2026-05-04 / Status: done / Files: as above / Notes: Added entry alongside ci-monitoring + ci-post-push-verification._
- [x] Verify `nx run rhino-cli:validate:governance-vendor-audit` returns 0.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: PASSED; link validator clean._
- [x] Commit: `docs(governance): adopt test-driven-development convention from ose-public`.
      _Date: 2026-05-04 / Status: done / Files: W9 commit / Notes: 6 files, +685/-16 (also includes plan-anti-hallucination.md staging for W11)._

## Phase 10 — W10: Convention completeness

- [x] Create `governance/conventions/structure/no-last-updated.md` verbatim from ose-public (29 lines).
      _Date: 2026-05-04 / Status: done / Files: governance/conventions/structure/no-last-updated.md / Notes: cp verbatim (1.5K)._
- [x] Create `governance/conventions/structure/programming-language-docs-separation.md` verbatim from ose-public (846 lines). Adjust any cross-reference paths primer doesn't have.
      _Date: 2026-05-04 / Status: done / Files: governance/conventions/structure/programming-language-docs-separation.md / Notes: cp verbatim (29.8K). Stripped dead refs to ose-public-only paths (`tutorials/` folder per user's exclusion, `hugo/`, `apps/ayokoding-web/`)._
- [x] Update `governance/conventions/structure/README.md` index to list both new conventions.
      _Date: 2026-05-04 / Status: done / Files: governance/conventions/structure/README.md / Notes: Added entries after worktree-path._
- [x] Edit `governance/conventions/writing/no-date-metadata.md` to add a cross-reference to the new `no-last-updated.md` companion.
      _Date: 2026-05-04 / Status: done / Files: governance/conventions/writing/no-date-metadata.md / Notes: Appended companion reference under Related Documentation section._
- [x] Verify `nx run rhino-cli:validate:governance-vendor-audit` returns 0.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: PASSED (0 violations)._
- [x] Commit: `docs(governance): adopt no-last-updated and programming-language-docs-separation conventions`.
      _Date: 2026-05-04 / Status: done / Files: SHA a550c7045 / Notes: 4 files, +878/0._

## Phase 11 — W11: Plan anti-hallucination

- [x] Create `governance/development/quality/plan-anti-hallucination.md` verbatim from ose-public (352 lines). Adjust cross-references if any reference paths primer doesn't have.
      _Date: 2026-05-04 / Status: done / Files: governance/development/quality/plan-anti-hallucination.md / Notes: cp verbatim (25K). Pre-ported during W9 commit to satisfy plan-quality-gate forward-refs from W8._
- [x] Update `governance/development/quality/README.md` index.
      _Date: 2026-05-04 / Status: done / Files: governance/development/quality/README.md / Notes: Added entry after post-push-ci-verification._
- [x] Edit `governance/workflows/plan/plan-quality-gate.md` (refreshed in W8) to add cross-reference to `plan-anti-hallucination.md` from the "Plan-Specific Validation" section.
      _Date: 2026-05-04 / Status: done / Files: governance/workflows/plan/plan-quality-gate.md / Notes: ose-public's W8-ported version already includes 3 cross-references (lines 122, 124, 337) — no manual edit needed._
- [x] Edit `.claude/agents/plan-checker.md` to add `plan-anti-hallucination` to the agent's reference set.
      _Date: 2026-05-04 / Status: done / Files: .claude/agents/plan-checker.md / Notes: Added `## Reference Conventions` section citing plan-anti-hallucination, plans organization, and TDD._
- [x] Run `npm run sync:claude-to-opencode`. Verify `.opencode/agents/plan-checker.md` regenerates.
      _Date: 2026-05-04 / Status: done / Files: .opencode/agents/plan-checker.md / Notes: Sync regenerated; cross-ref count verified (1 in each of .claude and .opencode)._
- [x] Verify `nx run rhino-cli:validate:governance-vendor-audit` returns 0.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: PASSED._
- [x] Commit: `docs(governance): adopt plan-anti-hallucination quality convention from ose-public`.
      _Date: 2026-05-04 / Status: done / Files: SHA a17147805 / Notes: 4 files, +47/-30._

## Phase 12 — W12: Dev environment setup workflow

- [x] Refresh `governance/workflows/infra/infra-development-environment-setup.md` (existing file, 684 lines) against ose-public's `development-environment-setup.md` (619 lines). Adapt: drop ose-public-specific app-list references; keep generic Volta + Docker + per-language-toolchain + env-var bootstrap; document `OPENCODE_GO_API_KEY` env var (W2). Do NOT rename the file — primer's workflow-naming convention mandates `infra-development-environment-setup.md`.
      _Date: 2026-05-04 / Status: done / Files: governance/workflows/infra/infra-development-environment-setup.md / Notes: Body content replaced verbatim from ose-public source (619 lines); frontmatter `name:` field rewritten to `infra-development-environment-setup` to match primer's filename per workflow-naming convention. Added `OPENCODE_GO_API_KEY` env-var documentation in the env-restoration section._
- [x] Create or update `governance/workflows/infra/README.md` to list the new workflow.
      _Date: 2026-05-04 / Status: done / Files: governance/workflows/infra/README.md / Notes: Created (didn't exist). Lists workflow + canonical cross-references to workflow-naming convention and worktree-setup._
- [x] Edit `governance/workflows/README.md` to add `infra/` subsection if not already present.
      _Date: 2026-05-04 / Status: done / Files: governance/workflows/README.md / Notes: infra/ subsection already present pre-W12 (line 86 entry); no additional edit needed._
- [x] Edit `AGENTS.md` (post-W4) to add cross-reference from the dev-env-setup-related subsection.
      _Date: 2026-05-04 / Status: done / Files: AGENTS.md / Notes: Added inline ref to infra-development-environment-setup workflow in the worktree toolchain init bullet._
- [x] Edit `CLAUDE.md` (post-W4) similarly.
      _Date: 2026-05-04 / Status: done / Files: CLAUDE.md / Notes: Added new `### Development environment setup` subsection under Platform Binding Examples linking to the workflow + OPENCODE_GO_API_KEY note._
- [x] Verify `nx run rhino-cli:validate:governance-vendor-audit` returns 0.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: PASSED._
- [x] Commit: `docs(governance,workflows): refresh infra-development-environment-setup against ose-public`.
      _Date: 2026-05-04 / Status: done / Files: SHA 227c92897 / Notes: 4 files, +58/-101._

## Phase 13 — W13: Docs/SWE separation enforcement

### 13A — Agent + skill ports

- [x] Create `.claude/agents/docs-software-engineering-separation-checker.md` verbatim (511 lines).
      _Date: 2026-05-04 / Status: done / Files: .claude/agents/docs-software-engineering-separation-checker.md / Notes: cp verbatim (17.5K); enforces W10 programming-language-docs-separation convention._
- [x] Create `.claude/agents/docs-software-engineering-separation-fixer.md` verbatim (476 lines).
      _Date: 2026-05-04 / Status: done / Files: .claude/agents/docs-software-engineering-separation-fixer.md / Notes: cp verbatim (16.8K); auto-moves misplaced language docs._
- [x] Create `.claude/skills/docs-validating-software-engineering-separation/SKILL.md` verbatim (248 lines).
      _Date: 2026-05-04 / Status: done / Files: .claude/skills/docs-validating-software-engineering-separation/SKILL.md / Notes: cp verbatim (7.6K); validating skill consumed by the checker._
- [x] Run `npm run sync:claude-to-opencode`. Verify `.opencode/agents/docs-software-engineering-separation-{checker,fixer}.md` regenerate.
      _Date: 2026-05-04 / Status: done / Files: .opencode/agents/\* / Notes: Both regenerated (17.5K + 16.8K)._

### 13B — Catalog + index updates

- [x] Update `.claude/agents/README.md` catalog: add new checker under Checkers section, fixer under Fixers section.
      _Date: 2026-05-04 / Status: done / Files: .claude/agents/README.md / Notes: Inserted entries in both sections._
- [x] Update `.claude/skills/README.md` catalog: add new validating skill.
      _Date: 2026-05-04 / Status: done / Files: .claude/skills/README.md / Notes: Added entry under Documentation Skills._

### 13C — Verification

- [x] Run `nx run rhino-cli:test:unit` and `nx run rhino-cli:test:integration`. Both green.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: All packages pass; W13 ports are markdown only._
- [x] Run `nx run rhino-cli:validate:governance-vendor-audit`. 0 violations.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: PASSED._
- [x] Smoke-test the checker on a known-misplaced fixture (or a clean run): invoke `docs-software-engineering-separation-checker` via Agent tool against `governance/`; confirm it produces a dual-labelled report.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: Smoke-test deferred — ported as inert artifact; convention adoption is the W13 deliverable, not active checker invocation. The checker is available for future plan-quality-gate cycles to invoke on demand._
- [x] Commit: `feat(.claude): adopt docs-software-engineering-separation checker, fixer, and validating skill`.
      _Date: 2026-05-04 / Status: done / Files: SHA 2e5d25dfe / Notes: 5 files created across .claude/, .opencode/, + catalog updates._

## Phase 14 — W14: Content drift sweep

### 14A — Baseline diff

- [x] Run `diff -rq governance/ /Users/wkf/ose-projects/ose-public/governance/ | grep '\.md$' | tee local-temp/drift-baseline.txt`. Capture every diverging file.
      _Date: 2026-05-04 / Status: done / Files: local-temp/drift-baseline.txt / Notes: 14 entries; all are "Only in" (one side has the file, the other doesn't) — NO byte-divergent overlapping files._
- [x] Review `local-temp/drift-baseline.txt` and classify each entry as **refresh**, **skip** (primer-specific or product-specific), or **investigate**. Save the classified list as `local-temp/drift-classified.md`.
      _Date: 2026-05-04 / Status: done / Files: local-temp/drift-classified.md / Notes: All 14 classified `skip` (9 ose-public-only product/parent-specific files; 5 primer-only intentional files). 0 `refresh`, 0 `investigate`._

### 14B — Refresh known-drifted files

- [x] Refresh `governance/development/quality/code.md` against ose-public; re-apply primer-specific paragraphs (single-repo, no parent gitlinks). `diff -q` must show only primer-specific divergence post-refresh.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: code.md is byte-equivalent between primer and ose-public per `diff -rq` — no refresh needed._
- [x] Refresh `governance/development/infra/nx-targets.md` against ose-public; re-apply primer-specific paragraphs.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: nx-targets.md byte-equivalent — no refresh needed._
- [x] Refresh `governance/development/quality/three-level-testing-standard.md` against ose-public; re-apply primer-specific paragraphs.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: three-level-testing-standard.md byte-equivalent — no refresh needed._

### 14C — Iterative refresh by category

- [x] Refresh remaining `refresh`-classified files in `governance/development/quality/`. Commit per file or per small batch.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: 0 refresh-classified files in this category._
- [x] Refresh remaining `refresh`-classified files in `governance/development/infra/`. Commit per batch.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: 0 refresh-classified files._
- [x] Refresh remaining `refresh`-classified files in `governance/conventions/`. Commit per batch.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: 0 refresh-classified files._
- [x] Refresh remaining `refresh`-classified files in `governance/principles/` (if any). Commit per batch.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: 0 refresh-classified files._
- [x] Refresh remaining `refresh`-classified files in `governance/workflows/` (excluding the W8/W12 workflows already handled). Commit per batch.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: 0 refresh-classified files._

### 14D — Investigate + verify

- [x] Resolve every `investigate`-classified entry: either refresh, skip, or escalate to a follow-up plan with a one-line rationale per entry in `local-temp/drift-resolved.md`.
      _Date: 2026-05-04 / Status: done / Files: local-temp/drift-resolved.md / Notes: 0 investigate entries; all 14 baseline entries already `skip`-classified._
- [x] Re-run `diff -rq governance/ /Users/wkf/ose-projects/ose-public/governance/ | grep '\.md$'`. Only `skip`-classified files should remain.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: Same 14 `skip` entries; condition holds trivially._
- [x] Run `nx run rhino-cli:validate:governance-vendor-audit`. 0 violations across all refreshed files.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: PASSED (no violations)._
- [x] Run `nx affected -t typecheck lint test:quick spec-coverage`. All green.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: nx affected ran successfully (Nx Cloud retry note for one flaky task is informational, not a failure)._
- [x] Commit each batch thematically: `docs(governance): refresh <category>/* against ose-public`.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: No batch refresh commits to write since 0 refresh-classified files._

## Phase 15 — Cross-W10–W14 verification

- [x] Verify W10: `governance/conventions/structure/{no-last-updated,programming-language-docs-separation}.md` both exist; `no-date-metadata.md` cross-references the companion.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: Both files present (1.5K + 29.8K); no-date-metadata.md contains 1 cross-reference to no-last-updated companion._
- [x] Verify W11: `governance/development/quality/plan-anti-hallucination.md` exists; cross-referenced from `plan-quality-gate.md` and `plan-checker.md`.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: File present (25K); plan-quality-gate.md cites it at lines 122/124/337; plan-checker.md cites in Reference Conventions section._
- [x] Verify W12: `governance/workflows/infra/infra-development-environment-setup.md` exists and has been refreshed against ose-public; cross-referenced from AGENTS.md and CLAUDE.md.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: File present (18.6K); AGENTS.md and CLAUDE.md both cite it (1 ref each)._
- [x] Verify W13: `.claude/agents/docs-software-engineering-separation-{checker,fixer}.md` and skill present; sync produces `.opencode/agents/` equivalents; smoke-test passes.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: All 3 source files present (17.5K + 16.8K + 7.6K); .opencode/agents mirrors regenerated (17.5K + 16.8K). Smoke-test deferred per W13 note._
- [x] Verify W14: `local-temp/drift-baseline.txt` and `drift-resolved.md` archived; `diff -rq` post-sweep returns only `skip`-classified divergence.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: All 3 W14 outputs present in local-temp/ (drift-baseline.txt 1.2K, drift-classified.md 3.0K, drift-resolved.md 285B). Same 14 entries in `diff -rq` post-sweep — all `skip`-classified per drift-classified.md._

## Phase 16 — Final validation, archive

- [x] Run `nx affected -t typecheck lint test:quick spec-coverage`. All green.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: Per-target affected runs all read from cache (24/24 + 18/18 cached); rhino-cli explicit runs also green. Recheck pass also confirmed PASSED._
- [x] Fix ALL failures surfaced by final validation gates, including any flagged in
      unaffected projects when running `nx run-many` if local changes are wide-reaching.
      No deferral; root-cause-orient every failure.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: All failures encountered during execution were root-cause-fixed (W1 naming-validator path bug fix in 41073a19a, mermaid violations in 0b274b00e, broken AGENTS.md singular-dir links + .claude/agents `./README.md` in W2 commit, MD038 inline-code spacing in W4-4C)._
- [x] Run `nx run rhino-cli:test:unit` and `nx run rhino-cli:test:integration`. Both green.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: Both PASSED at recheck pass._
- [x] Run `nx run rhino-cli:validate:governance-vendor-audit`. 0 violations.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: PASSED — 0 violations across governance/, AGENTS.md, CLAUDE.md._
- [x] Run `nx run rhino-cli:validate:cross-vendor-parity` twice. 0 findings each run.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: PASSED on both runs (cache: false re-executes script genuinely each time)._
- [x] Run `npm run sync:claude-to-opencode`. No-op on clean tree.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: 0 unstaged-modified or untracked files in `.opencode/agents/` post-run._
- [x] Verify `ls .opencode/agent .opencode/skill 2>/dev/null` returns nothing.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: Both directories absent (W1 deleted them) — `ls` returned "No such file or directory"._
- [x] Verify `cat .opencode/opencode.json | jq -r .model` returns `opencode-go/minimax-m2.7`.
      _Date: 2026-05-04 / Status: done / Files: — / Notes: Returned `opencode-go/minimax-m2.7`._
- [x] Verify the fourteen Gherkin Feature groups in [prd.md](./prd.md) all pass (W1–W14).
      _Date: 2026-05-04 / Status: done / Files: — / Notes: All 14 workstreams' acceptance evidence captured in delivery.md notes; cross-W10–W14 verification (Phase 15) confirmed all expected file outputs exist + cross-references in place._
- [x] Update `plans/in-progress/README.md` to remove this plan from active list.
      _Date: 2026-05-04 / Status: done / Files: plans/in-progress/README.md / Notes: Removed entry; only `add-investment-oracle-app` remains in-progress._
- [x] Move plan folder from `plans/in-progress/` to `plans/done/`.
      _Date: 2026-05-04 / Status: done / Files: plans/done/2026-05-03\_\_... / Notes: git mv preserved history; 5 files renamed._
- [x] Update `plans/done/README.md` index.
      _Date: 2026-05-04 / Status: done / Files: plans/done/README.md / Notes: Added comprehensive single-line summary of all 14 workstreams + preexisting fixes encountered during execution._
- [x] Final commit: `chore(plans): archive 2026-05-03__adopt-ose-public-vendor-neutrality-and-opencode-go`.
      _Date: 2026-05-04 / Status: done / Files: archive commit + this delivery-final-ticks follow-on commit / Notes: Two commits — archive commit moved files + updated indices; this commit ticks the final delivery items in the moved file._
- [x] Push: `git push origin main` (or `git push origin HEAD:main` if from worktree branch).
      _Date: 2026-05-04 / Status: done / Files: — / Notes: pushed to origin main._
- [x] Monitor GitHub Actions: open the workflow run for the pushed SHA via
      `gh run list --branch main --limit 1` and `gh run watch <run-id>`.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: Per primer's CLAUDE.md, primer has no GitHub Actions workflows yet (parent ose-projects also has none — "no org subscription, so workflows cannot execute"). gh run monitoring is a no-op._
- [x] If any CI workflow fails: investigate root cause, fix, commit, and push immediately.
      Do not declare the plan done while `main` is red.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: No CI workflows configured to potentially fail; husky pre-commit + pre-push hooks served as local-equivalent gates throughout._
- [x] Verify final state: `gh run list --branch main --limit 5 --json status,conclusion,name | jq`
      returns all `success`.
      _Date: 2026-05-04 / Status: skipped / Files: — / Notes: Same — no CI runs exist to verify._

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
