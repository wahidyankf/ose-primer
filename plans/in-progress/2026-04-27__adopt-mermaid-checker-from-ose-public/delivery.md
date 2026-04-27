---
title: Delivery — Adopt ose-public Mermaid Checker Enhancements
---

# Delivery Checklist

Execute phases in order. Each `- [ ]` is one tick — one concrete
action. Do not batch ticks across phase boundaries. Use one
Conventional-Commits commit per thematic phase unless explicitly
grouped below.

**Publish path**: direct push to `origin main` per
[Git Push Default Convention](../../../governance/development/workflow/git-push-default.md)
Standards 1, 2, 6. No draft PR is opened — the user has not
requested one for this plan. Worktree is optional; if used, push
via `git push origin HEAD:main` per Standard 6.

**Reference source for ports**: read
`https://github.com/wahidyankf/ose-public/tree/main/apps/rhino-cli/internal/mermaid`
in the GitHub UI, or open a parent-rooted Claude session for
side-by-side filesystem comparison. Inside an ose-primer-rooted
worktree, `../../ose-public/...` is empty per the bare-gitlink
contract.

## Phase 0 — Worktree and environment (optional but recommended)

- [x] Decide worktree-or-not. For parallel-safety with other
      sessions: create the worktree via `claude --worktree adopt-mermaid-checker`.
      For single-session work: skip and operate on `main`. Both
      paths satisfy Git Push Default Convention Standards 1 + 6.
      _Date 2026-04-27 / Status: done / Files: none / Notes: single-session, work directly on main_
- [x] If worktree: confirm session lands inside
      `ose-primer/.claude/worktrees/adopt-mermaid-checker/` on
      branch `worktree-adopt-mermaid-checker`.
      _Date 2026-04-27 / Status: N/A / Files: none / Notes: no worktree used (P0.1)_
- [x] Run `npm install` from the working tree root.
      _Date 2026-04-27 / Status: done / Files: none / Notes: 1586 pkgs audited, doctor verified 19/19 tools_
- [x] Run `npm run doctor -- --fix` to converge polyglot toolchains
      (mandatory for worktree setup per
      `governance/development/workflow/worktree-setup.md`; harmless
      if already converged).
      _Date 2026-04-27 / Status: done / Files: none / Notes: 19/19 tools OK, nothing to fix_
- [x] Confirm `go version` reports Go ≥ 1.22.
      _Date 2026-04-27 / Status: done / Files: none / Notes: go1.26.1 darwin/arm64_
- [x] Confirm `node --version` reports 24.13.1 and `npm --version`
      reports 11.10.1.
      _Date 2026-04-27 / Status: done / Files: none / Notes: node v24.13.1, npm 11.10.1_

## Phase 1 — Baseline snapshot

- [x] Run `nx affected -t typecheck lint test:quick spec-coverage`
      from the working tree root. Capture failures (if any) in
      `local-temp/baseline.txt`.
      _Date 2026-04-27 / Status: done / Files: local-temp/baseline.txt / Notes: nothing affected — clean tree vs origin/main, EXIT=0_
- [x] Run `nx run rhino-cli:test:unit`. Must pass at baseline.
      _Date 2026-04-27 / Status: done / Files: none / Notes: all 13 packages pass_
- [x] Run `nx run rhino-cli:test:integration`. Must pass at baseline.
      _Date 2026-04-27 / Status: done (preexisting bug fixed) / Files: specs/apps/rhino/cli/gherkin/docs-validate-mermaid.feature, apps/rhino-cli/cmd/steps_common_test.go, apps/rhino-cli/cmd/docs_validate_mermaid.integration_test.go / Notes: LR4 scenario fixture wrong — chain diagram (depth=4) now used; test passes_
- [x] Run `nx run rhino-cli:validate:mermaid` on the current
      (governance-only) scan. Record violation/warning counts as
      baseline.
      _Date 2026-04-27 / Status: done / Files: none / Notes: 0 violations, 0 warnings, 20 files, 110 blocks scanned_

## Phase 2 — Port `internal/mermaid/types.go`

- [x] Add `WarningSubgraphDense WarningKind = "subgraph_density"`
      to the `WarningKind` const block in
      `apps/rhino-cli/internal/mermaid/types.go`.
      _Date 2026-04-27 / Status: done / Files: apps/rhino-cli/internal/mermaid/types.go / Notes: added WarningSubgraphDense const_
- [x] Add the `Subgraph` struct (`ID, Label, NodeIDs, StartLine`).
      _Date 2026-04-27 / Status: done / Files: apps/rhino-cli/internal/mermaid/types.go / Notes: Subgraph struct added_
- [x] Add `Subgraphs []Subgraph` to `ParsedDiagram`.
      _Date 2026-04-27 / Status: done / Files: apps/rhino-cli/internal/mermaid/types.go / Notes: Subgraphs field added to ParsedDiagram_
- [x] Add `SubgraphLabel string`, `SubgraphNodeCount int`,
      `MaxSubgraphNodes int` to `Warning`.
      _Date 2026-04-27 / Status: done / Files: apps/rhino-cli/internal/mermaid/types.go / Notes: subgraph_density fields added to Warning struct_
- [x] Update package doc comment from "It enforces three rules"
      to "It enforces four rules — three blocking violations and
      one density warning".
      _Date 2026-04-27 / Status: done / Files: apps/rhino-cli/internal/mermaid/types.go / Notes: doc comment updated_
- [x] Run `go build ./...` from `apps/rhino-cli/`. Must compile.
      _Date 2026-04-27 / Status: done / Files: none / Notes: go build exits 0_
- [x] Commit: `feat(rhino-cli): add Subgraph types and density warning kind to mermaid package`
      _Date 2026-04-27 / Status: done / Files: apps/rhino-cli/internal/mermaid/types.go / Notes: commit 679230f14 already in git history_
- [x] `git push origin main` (or `git push origin HEAD:main` from
      worktree per Standard 6).
      _Date 2026-04-27 / Status: done / Files: none / Notes: 679230f14 already on origin/main at plan start_

## Phase 3 — Port `internal/mermaid/parser.go`

- [x] Replace `apps/rhino-cli/internal/mermaid/parser.go` with the
      ose-public version, preserving the package import path.
      _Date 2026-04-27 / Status: done / Files: apps/rhino-cli/internal/mermaid/parser.go / Notes: full replacement with subgraph-aware version_
- [x] Verify `slices` import resolves (Go ≥ 1.21).
      _Date 2026-04-27 / Status: done / Files: none / Notes: go.mod uses go 1.26, slices resolves fine_
- [x] Replace `apps/rhino-cli/internal/mermaid/parser_test.go` with
      the ose-public version.
      _Date 2026-04-27 / Status: done / Files: apps/rhino-cli/internal/mermaid/parser_test.go / Notes: adds SubgraphCapture, NestedOuterDirectChildren, AmpExpansion tests_
- [x] Run `nx run rhino-cli:test:unit`. All parser tests pass.
      _Date 2026-04-27 / Status: done / Files: none / Notes: 80 tests pass, all 13 packages ok_
- [x] Confirm coverage on the package stays ≥ 90% via the test
      output's coverage line.
      _Date 2026-04-27 / Status: done / Files: none / Notes: mermaid package 95.9% total_
- [ ] Commit: `feat(rhino-cli): port subgraph-aware mermaid parser from ose-public`
- [ ] Push direct to main.

## Phase 4 — Port `internal/mermaid/validator.go`

- [ ] Replace `validator.go` with ose-public version. Confirm:
      direction-mapped `horizontal`/`vertical` are assigned to the
      `complex_diagram` warning's `ActualWidth`/`ActualDepth`.
- [ ] Confirm Rule 4 loop appends `subgraph_density` warnings only
      when `MaxSubgraphNodes > 0`.
- [ ] Replace `validator_test.go` with ose-public version.
- [ ] Run `nx run rhino-cli:test:unit`. All validator tests pass.
- [ ] Commit: `feat(rhino-cli): add subgraph-density rule and direction-mapped warning fields`
- [ ] Push direct to main.

## Phase 5 — Port `internal/mermaid/reporter.go`

- [ ] Replace `reporter.go` with ose-public version. Confirm
      text/JSON/markdown formatters all render `subgraph_density`
      warnings.
- [ ] Replace `reporter_test.go` with ose-public version.
- [ ] Run `nx run rhino-cli:test:unit`. All reporter tests pass.
- [ ] Commit: `feat(rhino-cli): render subgraph_density in all three reporter formats`
- [ ] Push direct to main.

## Phase 6 — Port `cmd/docs_validate_mermaid*.go`

- [ ] Replace `cmd/docs_validate_mermaid.go` with the ose-public
      version. Confirm `validateMermaidMaxSubgraphNodes int` is
      declared and the `--max-subgraph-nodes 6` flag is registered.
- [ ] Confirm the command's `Long` description mentions four rules.
- [ ] Confirm `collectMDDefaultDirs` includes
      `docs/`, `governance/`, `.claude/`, `plans/` (the port should
      already widen the default scan).
- [ ] Replace `cmd/docs_validate_mermaid_test.go` with the
      ose-public version. Adjust test paths if needed.
- [ ] Replace `cmd/docs_validate_mermaid.integration_test.go` with
      the ose-public version. Adjust test paths if needed.
- [ ] Diff `cmd/docs_validate_mermaid_helpers_test.go` against
      ose-public — touch only if helper signatures shifted.
- [ ] Run `nx run rhino-cli:test:unit`. All pass.
- [ ] Run `nx run rhino-cli:test:integration`. All pass.
- [ ] Commit: `feat(rhino-cli): wire --max-subgraph-nodes flag and broaden default scan to plans/`
- [ ] Push direct to main.

## Phase 7 — Update Nx target

- [ ] Edit `apps/rhino-cli/project.json` `validate:mermaid.command`:
      drop the positional `governance/ .claude/` arguments so the
      CLI uses its (now-widened) default scan.
- [ ] Edit `apps/rhino-cli/project.json` `validate:mermaid.inputs`:
      replace the existing two-tree list with five entries
      (`{workspaceRoot}/docs/**/*.md`,
      `{workspaceRoot}/governance/**/*.md`,
      `{workspaceRoot}/.claude/**/*.md`,
      `{workspaceRoot}/plans/**/*.md`,
      `{workspaceRoot}/*.md`) plus the existing
      `{projectRoot}/**/*.go` entry.
- [ ] Run `nx run rhino-cli:validate:mermaid`. The target should now
      scan all five trees. Capture violation + warning count for
      Phase 8.
- [ ] Commit: `chore(rhino-cli): broaden validate:mermaid scan to docs/ and plans/`
- [ ] Push direct to main.

## Phase 8 — Repository remediation

- [ ] Save the violation+warning report from Phase 7 to
      `local-temp/mermaid-violations.txt` for triage.
- [ ] Triage each entry into one of three buckets per
      `tech-docs.md` § Repository remediation: fix in place
      (default), justify and accept warning (forbidden — must hit
      zero), justify and accept violation (forbidden — blocks CI).
- [ ] For each violation entry: edit the source `.md`. Re-run
      `nx run rhino-cli:validate:mermaid` after each edit until the
      file is clean.
- [ ] For each warning entry: edit the source `.md` to bring the
      diagram below threshold. Zero-warning is the success bar.
- [ ] Re-run `nx run rhino-cli:validate:mermaid` once on the full
      repo. Exit code 0 and zero warnings required.
- [ ] Commit per domain: AI primers, BDD/TDD docs, programming-language
      docs, plans, governance — one Conventional-Commits commit per
      domain touched, body listing specific diagrams restructured.
- [ ] Push each commit direct to main.

## Phase 9 — Local quality gates

- [ ] Run `nx affected -t typecheck lint test:quick spec-coverage`
      from working tree root. All targets pass.
- [ ] Run `npm run lint:md`. Zero markdownlint findings.
- [ ] Run `nx run rhino-cli:validate:mermaid`. Exit 0, zero
      warnings.
- [ ] Fix any preexisting failures encountered in this phase, even
      if unrelated to mermaid work — Root Cause Orientation
      principle.

## Phase 10 — Pre-push hook verification

- [ ] Stage a touch-only edit to one `*.md` under each of `docs/`,
      `governance/`, `.claude/`, `plans/` (a whitespace-or-format
      tweak that survives `npm run format:md`).
- [ ] Run `git push --dry-run`. Confirm the Husky pre-push hook
      fires `nx run rhino-cli:validate:mermaid` and exits 0.
- [ ] Revert the touch-only edits before any further commits.

## Phase 11 — Post-push CI verification

- [ ] After every push direct to main during phases 2–8, watch
      the GitHub Actions run on `wahidyankf/ose-primer:main`.
- [ ] If any check fails, fix the root cause in a follow-up commit
      pushed direct to main. Do not bypass with `--no-verify` or
      `skip-checks`.
- [ ] Confirm the post-merge run remains green for at least the
      most recent five commits in this plan.

## Phase 12 — Plan archival

- [ ] `git mv plans/in-progress/2026-04-27__adopt-mermaid-checker-from-ose-public
plans/done/2026-04-27__adopt-mermaid-checker-from-ose-public`.
- [ ] Update `plans/in-progress/README.md` to remove the entry.
- [ ] Update `plans/done/README.md` to add the entry.
- [ ] Commit: `chore(plans): archive adopt-mermaid-checker-from-ose-public plan`.
- [ ] Push direct to main.

## Manual verification checklist (CLI surface)

This plan touches no UI and no HTTP endpoint, so neither Playwright
MCP nor curl-based assertion applies. The CLI is the verification
surface.

- [ ] After Phase 4 (validator port), run
      `cd apps/rhino-cli && go test ./internal/mermaid/... -run TestRule4SubgraphDensity -v`
      and confirm the Gherkin scenarios in `prd.md` map to passing
      tests one-for-one.
- [ ] After Phase 6 (CLI port), run
      `cd apps/rhino-cli && go run main.go docs validate-mermaid --max-subgraph-nodes 0 docs/`
      and confirm the rule disables (no `subgraph_density`
      warnings emitted).
- [ ] After Phase 7 (Nx target update), run
      `nx run rhino-cli:validate:mermaid -o json` and confirm the
      JSON output contains the new fields when warnings exist.

## Commit message conventions

- Follow Conventional Commits: `<type>(<scope>): <description>`.
- One thematic commit per delivery phase. Phase 8 splits per
  domain.
- No `--no-verify`. No `--no-gpg-sign`.
- Bodies wrap at 100 characters per `commitlint`.
- Each commit must include the
  `Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>`
  trailer when the work is AI-assisted.

## Iron rules

- Iron rule 1 — **Direct-to-main default**: every commit pushed
  during this plan goes direct to `origin main` per Git Push
  Default Convention Standards 1, 2, 6. The user has not requested
  a draft PR for this plan; do not open one.
- Iron rule 2 — **No skip flags**: never `--no-verify`, never
  `git commit -n`. If a hook fires, fix the root cause.
- Iron rule 3 — **Worktree optional, isolation if used**: if a
  worktree is used, all source edits run inside the worktree;
  never edit ose-primer source through the parent gitlink. If no
  worktree, work directly on `main`.
- Iron rule 4 — **Coverage floor**: rhino-cli Go coverage stays
  ≥ 90% throughout the plan. Failing coverage blocks the next
  phase.
- Iron rule 5 — **Zero-warning bar**: by Phase 9, the full-repo
  validate-mermaid run reports zero warnings AND zero violations.
  Warning-level findings are not acceptable trailing state.
- Iron rule 6 — **Root cause for unrelated failures**: any
  unrelated failure surfaced by `nx affected` during Phase 9 is
  fixed in this plan, not deferred (Root Cause Orientation
  principle, Proactive Preexisting Error Resolution practice).
- Iron rule 7 — **Linear history**: rebase before push if
  `origin/main` has moved (Git Push Default Convention Standard 4).
  Never create merge commits on `main`.
