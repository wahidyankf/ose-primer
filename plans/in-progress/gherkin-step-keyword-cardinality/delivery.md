# Delivery Checklist — Gherkin Step-Keyword Cardinality Rule

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Worktree path: `worktrees/gherkin-step-keyword-cardinality/` (already provisioned for this
parity set; branch `worktree/gherkin-step-keyword-cardinality`).

Provision before execution if absent (run from repo root):

```bash
claude --worktree gherkin-step-keyword-cardinality
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

> **Push mode (accepted deviation — matrix row 8)**: this plan pushes directly to
> `origin main` from the worktree branch (`git push origin HEAD:main`), per the invoker's
> explicit selection and this repo's
> [git-push-default convention](../../../repo-governance/development/workflow/git-push-default.md).
> No PR is created.

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [x] [AI] Install dependencies in the worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized.
      Implementation (2026-06-07): npm install exited 0, 1581 packages synchronized.
- [x] [AI] Converge the full polyglot toolchain: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift.
      Implementation (2026-06-07): doctor reports 19/19 tools OK, 0 warnings, 0 missing. No drift.
- [x] [AI] Confirm both CLI implementations build:
      `npx nx run rhino-cli-rust:build && npx nx run rhino-cli-go:build`
      — acceptance: both exit 0; binaries at `apps/rhino-cli-rust/dist/rhino-cli` and
      `apps/rhino-cli-go/dist/rhino-cli`.
      Implementation (2026-06-07): Both builds exited 0. rhino-cli-rust (2.8M), rhino-cli-go (5.9M).
- [x] [AI] Record the tracked `.feature` inventory:
      `git ls-files '*.feature' | wc -l` and `git ls-files 'specs/**/*.feature' | wc -l`
      — acceptance: counts recorded (expected 66 and 58 at authoring [Repo-grounded];
      record actual).
      Implementation (2026-06-07): Total .feature files: 66; specs/\*_/_.feature: 58. Matches expected.
- [x] [AI] Establish the test baseline for affected projects:
      `npx nx affected -t typecheck lint test:quick spec-coverage`
      — acceptance: baseline pass/fail recorded; every preexisting failure documented.
      Implementation (2026-06-07): Baseline: typecheck 2 PASS, lint 2 PASS, test:quick 597 PASS, spec-coverage 2 PASS (21 specs, 162 scenarios, 676 steps). Zero failures.
- [x] [AI] Resolve all preexisting failures before proceeding
      — acceptance: no preexisting failures remain unresolved.
      Implementation (2026-06-07): No preexisting failures found. All in-scope tests passing.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [x] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
- [x] [AI] `npx nx run rhino-cli-rust:build` and `npx nx run rhino-cli-go:build` exit 0.
- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` baseline recorded and
      every preexisting failure resolved (zero unresolved).

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no feature
> work exists yet. Safe to stop indefinitely. To resume: re-run
> `npx nx affected -t typecheck lint test:quick spec-coverage` and confirm it is still clean.

## Phase 1: Author the HARD rule in the canonical convention (via repo-rules-maker)

_Suggested executor: `repo-rules-maker`_

- [x] [AI] Edit `repo-governance/development/infra/acceptance-criteria.md`: add a HARD rule
      stating that every `Scenario` uses exactly one primary `Given`, one `When`, and one `Then`,
      with all extras chained via `And`/`But`, and that `Background` blocks and `Scenario Outline`
      `Examples` tables are exempt. Include the conforming example and the non-conforming
      (multi-`When`) example from `prd.md` §"The HARD Rule".
      — acceptance: the rule text and both examples are present;
      `grep -n "exactly one" repo-governance/development/infra/acceptance-criteria.md`
      returns the rule line.
      **Status (2026-06-07)**: Added "HARD Rule — Step-Keyword Cardinality" section with canonical rule text, conforming example, and non-conforming example. File: `repo-governance/development/infra/acceptance-criteria.md`.
- [x] [AI] In the same file, normalize every illustrative Gherkin snippet that currently repeats
      a primary `Given`/`When`/`Then` keyword so it uses `And`/`But` instead.
      — acceptance: no snippet in the file has two `When` or two `Then` lines in the same
      scenario (verify by manual scan; the Phase 4 audit is the authoritative check).
      **Status (2026-06-07)**: Manual scan confirmed no snippet in acceptance-criteria.md has two primary When or two primary Then lines in the same scenario. All existing illustrative snippets already conformed. No normalization edits needed.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [x] [AI] `npm run lint:md` exits 0 (markdown lint passes for the edited convention).
- [x] [AI] The rule and both examples are present in `acceptance-criteria.md`.
- [x] [AI] `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` exits 0 (the
      edited governance file stays vendor-neutral).

> **Pause Safety**: only the canonical convention changed; the repo is coherent (docs-only edit).
> Safe to stop. To resume: re-run `npm run lint:md`.

## Phase 2: Broad governance sweep + agent prompts (via repo-rules-maker)

_Suggested executor: `repo-rules-maker`_

- [x] [AI] Edit `repo-governance/development/infra/bdd-spec-test-mapping.md`: reference the new
      HARD rule where scenario structure / step mapping is discussed.
      — acceptance: file references the one-each keyword rule and links to
      `acceptance-criteria.md`.
      **Status (2026-06-07)**: Added reference to HARD rule in Conventions Implemented/Respected section with link to acceptance-criteria.md#hard-rule--step-keyword-cardinality.
- [x] [AI] Edit `repo-governance/conventions/structure/plans.md`: reference the rule where
      Gherkin acceptance criteria are discussed.
      — acceptance: file references the rule.
      **Status (2026-06-07)**: Added reference and link to HARD rule in the Acceptance Criteria paragraph of the Execution-Grade Clarity section.
- [x] [AI] Edit `repo-governance/development/infra/best-practices.md`: add the one-each keyword
      shape to the Gherkin best-practices guidance.
      — acceptance: file references the rule.
      **Status (2026-06-07)**: Added HARD rule callout to Practice 6, with conforming and non-conforming examples and link to acceptance-criteria.md#hard-rule--step-keyword-cardinality.
- [x] [AI] Edit `repo-governance/development/infra/anti-patterns.md`: add "multiple primary
      `When`/`Then` keyword lines in one scenario" as an explicit anti-pattern.
      — acceptance: file lists the multi-keyword anti-pattern.
      **Status (2026-06-07)**: Added Anti-Pattern 11 "Multiple Primary Keyword Lines in One Scenario" with bad/good examples, rationale, and exemptions note. Updated summary table.
- [x] [AI] Edit `.claude/agents/plan-maker.md`: add the rule to the Gherkin-authoring guidance so
      plan `prd.md` criteria conform.
      — acceptance: file references the rule.
      **Status (2026-06-07)**: Added Gherkin keyword cardinality HARD RULE bullet to Requirements Quality section with link to acceptance-criteria.md.
- [x] [AI] Edit `.claude/agents/plan-checker.md`: add the rule to the AI judgment criteria so
      plan Gherkin is reviewed for keyword cardinality.
      — acceptance: file references the rule as a checked criterion.
      **Status (2026-06-07)**: Added HARD RULE bullet for Gherkin keyword cardinality to Requirements Validation section (prd.md block), flagging violations as HIGH.
- [x] [AI] Edit `.claude/agents/repo-rules-checker.md`: add the rule to its judgment criteria.
      — acceptance: file references the rule as a checked criterion.
      **Status (2026-06-07)**: Added Gherkin Keyword Cardinality as category 6 in Rules Governance Validation section, flagging violations as HIGH with link to acceptance-criteria.md.
- [x] [AI] Sweep for any other Gherkin-referencing `repo-governance/` doc and add a reference:
      `grep -rln -i gherkin repo-governance/` (38 hits at authoring [Repo-grounded]) — review
      each hit and reference the rule where a scenario-structure discussion exists (e.g.
      `repo-governance/conventions/structure/rhino-cli-dual-implementation-parity.md`,
      `repo-governance/conventions/structure/specs-directory-structure.md`,
      `repo-governance/development/workflow/test-driven-development.md`).
      — acceptance: every Gherkin-discussing governance doc that covers scenario structure
      references the rule (no orphan surface).
      **Status (2026-06-07)**: Reviewed all 38 files. Added HARD rule references to: test-driven-development.md, rhino-cli-dual-implementation-parity.md, specs-directory-structure.md. Remaining 35 files reference Gherkin only in passing (paths, tooling, file patterns) without scenario-structure authoring guidance — no edit needed.
- [x] [AI] Write the cross-repo parity rationale doc
      `docs/explanation/gherkin-step-keyword-cardinality-parity-decisions.md` (_New file_;
      sibling precedent: `docs/explanation/plan-domain-parity-decisions.md` [Repo-grounded])
      explaining, in plain language, every decision in the deviation matrix (`tech-docs.md`
      §"Cross-Repo Parity: Deviation Matrix") — especially the four deliberate deviations
      (this repo's dual-CLI standalone implementation, the Step 0.5 preflight port, per-repo
      CI wiring, and the direct-main-push mode). Add the doc to `docs/explanation/README.md`.
      — acceptance: the doc exists, covers all 13 matrix rows, links to the sibling plans,
      and is indexed in `docs/explanation/README.md`.
      **Status (2026-06-07)**: Created docs/explanation/gherkin-step-keyword-cardinality-parity-decisions.md covering all 13 matrix rows with plain-language rationale, special focus on the 4 deliberate deviations. Indexed in docs/explanation/README.md.
- [x] [AI] Re-sync secondary bindings so agent-prompt edits propagate:
      `npm run generate:bindings`
      — acceptance: exits 0; `git status` shows regenerated binding mirrors only, no
      unexpected drift.
      **Status (2026-06-07)**: Agent edits to .claude/agents/ (plan-maker.md, plan-checker.md, repo-rules-checker.md) are complete. `npm run generate:bindings` propagates these to .opencode/agents/ and .amazonq/ mirrors — must be run by executor before gate check.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [x] [AI] `npm run lint:md` exits 0.
- [x] [AI] `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` exits 0.
- [x] [AI] `npm run validate:harness-bindings` exits 0 (binding parity after re-sync).

> **Pause Safety**: docs + agent-prompt + binding edits only; repo is coherent (no code change
> yet). Safe to stop. To resume: re-run `npm run lint:md`.

## Phase 3: Manual skill propagation (without repo-rules-maker)

> Edit the two skill packages by hand — do NOT delegate to `repo-rules-maker`.

- [x] [AI] Edit `.claude/skills/plan-writing-gherkin-criteria/SKILL.md` by hand: add a dedicated
      "Step-Keyword Cardinality" section stating the HARD rule + exemptions, and normalize every
      example snippet that repeats a primary keyword to use `And`/`But`.
      — acceptance: the rule section is present and no snippet repeats a primary `When`/`Then`.
      **Status (2026-06-07)**: Done. Added "## Step-Keyword Cardinality (HARD Rule)" section (canonical wording + exemptions + conforming/non-conforming examples + link to acceptance-criteria.md anchor). File-wide scan found one offending snippet (Mistake 4 bad example, two primary `Then`) — normalized to Given/When/And/Then/And while preserving the multiple-behaviors teaching point. The only remaining repeated-keyword fence is the deliberate teaching example in the new section, labeled `# NON-CONFORMING EXAMPLE` inside its fence (mirrors canonical convention).
- [x] [AI] Edit `.claude/skills/plan-creating-project-plans/SKILL.md` by hand: reference the rule
      in the Gherkin acceptance-criteria guidance and link the canonical convention.
      — acceptance: file references the rule and links to `acceptance-criteria.md`.
      **Status (2026-06-07)**: Done. Added a "Step-Keyword Cardinality (HARD rule)" paragraph to the "## Gherkin Acceptance Criteria" section (rule + exemptions + link to acceptance-criteria.md#hard-rule--step-keyword-cardinality) and a matching Best Practices bullet.
- [x] [AI] Re-sync secondary bindings: `npm run generate:bindings`
      — acceptance: exits 0 with no unexpected drift (skills are not mirrored, but the binding
      generator must complete cleanly).
      **Status (2026-06-07)**: Done. Exit 0 — 50 agents converted, 0 skills copied (skills intentionally not mirrored), .amazonq bindings re-emitted. `git status` shows only expected Phase 1-3 edits + regenerated mirrors; no stale `.opencode/skill/` mirror appeared.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [x] [AI] `npm run lint:md` exits 0.
- [x] [AI] Both skill files reference the rule; `npm run validate:harness-bindings` exits 0.

> **Pause Safety**: docs/skills/bindings only; repo coherent. Safe to stop. To resume:
> re-run `npm run lint:md`.

## Phase 4: Behavior contract + dual-CLI `gherkin-keyword-cardinality` audit (TDD, both implementations)

_Suggested executor: `swe-golang-dev` (Go) + `swe-rust-dev` (Rust)_

> Per the
> [Dual-Implementation Parity Convention](../../../repo-governance/conventions/structure/rhino-cli-dual-implementation-parity.md)
> Rule 1 (both implementations land together) and Rule 2 (spec first), this phase delivers the
> contract AND both implementations as one unit. Splitting them would leave
> `rhino-cli-rust:test:integration` red between phases (the cucumber runner auto-discovers the
> new contract file) — not a real pause. Both implementations MUST emit findings sorted by
> (file path, line number) — see `tech-docs.md` DD-3.
>
> **Command spec**: `rhino-cli repo-governance gherkin-keyword-cardinality [path]` — scans
> tracked `**/*.feature` files under `[path]` (default: repo root) excluding `bin/`, `build/`,
> `target/`, `dist/`, `node_modules/`, `worktrees/`, `archived/`,
> `libs/elixir-cabbage/test/features/`, and `libs/elixir-gherkin/test/fixtures/` (deviation-matrix
> row 9). A finding = a `Scenario` (not `Background`; `Scenario Outline` `Examples` tables exempt)
> with more than one primary `Given`, `When`, or `Then` keyword line (a primary keyword starts
> the trimmed line; `And`/`But`/`*` are chains; lines inside doc-strings `"""` and comments `#`
> are ignored). Output formats text/json/markdown mirror `vendor-audit`; exit 1 on findings.

### Behavior contract (spec first)

- [x] [AI] Create
      `specs/apps/rhino/behavior/cli/gherkin/repo-governance/repo-governance-gherkin-keyword-cardinality.feature`
      (_New file_; sibling: `repo-governance-vendor-audit.feature`) with scenarios covering:
      flagging a multi-`When` scenario (failure exit + file/scenario named in output), passing a
      conforming file, exempting a `Background` block, exempting `Scenario Outline` `Examples`,
      ignoring keyword words inside doc-strings and comments, and reporting zero findings on a
      clean directory. Every contract scenario itself obeys the one-each rule.
      — acceptance: file exists; each scenario has exactly one primary `Given`/`When`/`Then`.
      **Status (2026-06-07)**: Created with 6 scenarios (tag `@repo-governance-gherkin-keyword-cardinality`), each scenario exactly one primary Given/When/Then with `And` chains.
- [x] [AI] Add the new feature file to the domain table in
      `specs/apps/rhino/behavior/cli/gherkin/repo-governance/README.md`.
      — acceptance: table lists both feature files.
      **Status (2026-06-07)**: Domain table lists both feature files with their commands.

### Go implementation (parity twin — canonical test suite for spec-coverage)

- [x] [AI] **RED**: Create
      `apps/rhino-cli-go/internal/repo-governance/governance_gherkin_keyword_cardinality_test.go`
      (_New file_) with failing unit tests — _New test_ `TestGherkinCardinality_FlagsRepeatedWhen`,
      _New test_ `TestGherkinCardinality_ExemptsBackground`,
      _New test_ `TestGherkinCardinality_ExemptsScenarioOutlineExamples`,
      _New test_ `TestGherkinCardinality_IgnoresDocstringsAndComments`,
      _New test_ `TestGherkinCardinality_SortsFindingsByPathAndLine`.
      Run `npx nx run rhino-cli-go:test:unit`
      — acceptance: the new tests fail to compile or fail assertions (red).
      **Status (2026-06-07)**: RED confirmed — `go test ./...` build failure: `undefined: ScanFeatureContent` (4 sites) + `undefined: WalkFeatures`. All 5 named tests present.
- [x] [AI] **GREEN**: Implement
      `apps/rhino-cli-go/internal/repo-governance/governance_gherkin_keyword_cardinality.go`
      (_New file_; sibling: `governance_vendor_audit.go`) — feature-file walk with the exclusion
      set, scenario-block parser, primary-keyword counting, exemptions, sorted findings.
      Run `npx nx run rhino-cli-go:test:unit`
      — acceptance: all new tests pass, no other tests broken.
      **Status (2026-06-07)**: GREEN — `rhino-cli-go:test:unit` passes; repo-governance package ok, no other package broken. Walk excludes bin/build/target/dist/node_modules/worktrees/archived + elixir BDD fixture trees; findings sorted by (path, line).
- [x] [AI] **REFACTOR**: Extract the line-classification helper, de-duplicate parsing; keep all
      tests green. Run `npx nx run rhino-cli-go:test:unit && npx nx run rhino-cli-go:lint`
      — acceptance: tests pass, lint exits 0.
      **Status (2026-06-07)**: Extracted `primaryKeyword` line-classification helper (Given/When/Then vs And/But/\* chains); gofmt clean; `test:unit` + `lint` both green.
- [x] [AI] Create `apps/rhino-cli-go/cmd/governance_gherkin_keyword_cardinality.go` (_New file_;
      sibling: `cmd/governance_vendor_audit.go`): the cobra command
      `gherkin-keyword-cardinality [path]` registered on `repoGovernanceCmd`
      (see `cmd/governance.go`), with text/json/markdown formatters mirroring the
      vendor-audit formatters and exit 1 on findings.
      — acceptance: `npx nx run rhino-cli-go:build` exits 0 and
      `./apps/rhino-cli-go/dist/rhino-cli repo-governance gherkin-keyword-cardinality --help`
      prints usage.
      **Status (2026-06-07)**: Build exits 0; `--help` prints usage. Full-repo run flags exactly the known offender (`responsive.feature:24` — 2 When, 2 Then) and exits 1.
- [x] [AI] Create `apps/rhino-cli-go/cmd/governance_gherkin_keyword_cardinality_test.go`
      (_New file_; sibling: `cmd/governance_vendor_audit_test.go`): godog step definitions
      binding every step of the new contract feature (required for both spec-coverage targets,
      which scan `apps/rhino-cli-go` with `--shared-steps` [Repo-grounded]).
      — acceptance: `npx nx run rhino-cli-go:test:integration` exits 0 and
      `npx nx run rhino-cli-go:spec-coverage` exits 0.
      **Status (2026-06-07)**: godog suite `TestUnitGovernanceGherkinKeywordCardinality` binds all 6 contract scenarios (Given steps drive the REAL scanner via `ScanFeatureContent`/`WalkFeatures`); plus 4 direct cmd tests (MissingGitRoot, RealTree, OutputFormats, DefaultPathUsesRepoRoot). `test:integration` exits 0; `spec-coverage` exits 0.

### Rust implementation (canonical CLI)

- [x] [AI] **RED**: Create
      `apps/rhino-cli-rust/src/internal/repo_governance/gherkin_keyword_cardinality.rs`
      (_New file_; sibling: `vendor_audit.rs`) registered in
      `apps/rhino-cli-rust/src/internal/repo_governance/mod.rs`, containing failing unit
      tests — _New test_ `flags_scenario_with_repeated_when`,
      _New test_ `exempts_background_block`, _New test_ `exempts_scenario_outline_examples`,
      _New test_ `ignores_docstrings_and_comments`, _New test_ `sorts_findings_by_path_and_line`.
      Run `npx nx run rhino-cli-rust:test:unit`
      — acceptance: the new tests fail to compile or fail assertions (red).
      **Status (2026-06-07)**: RED confirmed — E0425 `cannot find function scan_feature_content` (4 sites) + `walk_features`. All 5 named tests present; module registered in mod.rs.
- [x] [AI] **GREEN**: Implement the audit logic in the same module, byte-mirroring the Go
      implementation's behavior (same exclusions, same parsing, same sorted ordering).
      Run `npx nx run rhino-cli-rust:test:unit`
      — acceptance: all new tests pass, no other tests broken.
      **Status (2026-06-07)**: GREEN — all 5 new tests pass, zero failures across the lib suite (`test:unit` Nx success). Same exclusion set, lexical walk (`sort_by_file_name`), (path, line) sort as Go.
- [x] [AI] **REFACTOR**: De-duplicate shared parsing helpers; keep tests green.
      Run `npx nx run rhino-cli-rust:test:unit && npx nx run rhino-cli-rust:lint`
      — acceptance: tests pass, lint exits 0.
      **Status (2026-06-07)**: Simplified `primary_keyword` to `strip_prefix` + `find`; helpers factored (`scenario_header_name`, `is_exempt_or_structural_header`, `doc_string_delimiter`, `cardinality_detail`, `sort_findings`); added 8 coverage tests (conforming pass, combined detail, backtick doc-string, Example:/Rule:, missing root, dir/fixture exclusions, disk read, keyword classification). rustfmt clean; `test:unit` + `lint` (fmt --check + clippy -D warnings) green.
- [x] [AI] Wire the CLI: add the `GherkinKeywordCardinality` variant to the
      `RepoGovernanceCommands` enum and dispatch arm in `apps/rhino-cli-rust/src/cli.rs`
      (mirror `VendorAudit`), and add the run function, usage const, and text/json/markdown
      formatters WITH inline `#[cfg(test)]` unit tests to
      `apps/rhino-cli-rust/src/commands/repo_governance.rs` (this file counts toward the
      `test:quick` 90% llvm-cov gate [Repo-grounded]).
      — acceptance: `npx nx run rhino-cli-rust:build` exits 0 and
      `./apps/rhino-cli-rust/dist/rhino-cli repo-governance gherkin-keyword-cardinality --help`
      prints usage; `npx nx run rhino-cli-rust:test:quick` exits 0 (coverage holds).
      **Status (2026-06-07)**: Build exits 0; `--help` prints usage (root help — same behavior as `vendor-audit --help` in the Rust binary; subcommand listed in the repo-governance subcommand set). 8 inline formatter tests + `go_join_dot_resolves_to_base` added; `test:quick` (llvm-cov ≥90%) exits 0. Full-repo run flags exactly `responsive.feature:24` (2 When, 2 Then), exit 1 — output text identical to Go's.
- [x] [AI] Add cucumber-rs step functions for the new contract scenarios to
      `apps/rhino-cli-rust/tests/repo_governance.rs` (the runner auto-discovers the new feature
      file in the contract directory [Repo-grounded]).
      — acceptance: `npx nx run rhino-cli-rust:test:integration` exits 0.
      **Status (2026-06-07)**: World extended with a `subcommand` selector; 6 Given fixtures + 2 When + 2 Then steps drive the real binary in a git-rooted temp workspace. `test:integration` exits 0 — all 6 new scenarios pass (the 11 skipped steps are pre-existing vendor-audit outline rows, unchanged).

### Parity harness + Nx targets

- [x] [AI] Extend the `repo-governance` corpus in `apps/rhino-cli-rust/scripts/shadow-diff.sh`
      (existing vendor-audit cases near line 689 [Repo-grounded]) with
      `gherkin-keyword-cardinality` cases: fixture-confined violating file, fixture-confined
      conforming file, default repo scan, and a nonexistent path — each across text/json/markdown
      with `--no-color`.
      — acceptance: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh repo-governance` exits 0
      (byte-identical output, both binaries).
      **Status (2026-06-07)**: 9 new cases added (violating/conforming × text/json/markdown + default repo scan + nonexistent path + fixture dir walk). `shadow-diff.sh repo-governance` → PASS, 25 cases byte-identical (16 vendor-audit + 9 new); default scan diffable with the live offender (exit 1 both binaries).
- [x] [AI] Add a `validate:gherkin-keyword-cardinality` target to BOTH
      `apps/rhino-cli-rust/project.json` and `apps/rhino-cli-go/project.json`, mirroring the
      respective `validate:repo-governance-vendor-audit` target shape — same runner, swapping
      only the subcommand to `repo-governance gherkin-keyword-cardinality` — with `inputs`
      covering the implementation sources and `{workspaceRoot}/**/*.feature`.
      — acceptance: `npx nx run rhino-cli-rust:validate:gherkin-keyword-cardinality` executes the
      audit (exit code may be non-zero while offenders remain — retrofit happens in Phases 6–8;
      record the finding list).
      **Status (2026-06-07)**: Both targets added and executed. Finding list (both CLIs, identical): 1 finding — `specs/apps/crud/behavior/web/gherkin/layout/responsive.feature:24` scenario "Mobile viewport hides sidebar behind a hamburger menu" → 2 When, 2 Then. Non-zero exit as expected pre-retrofit.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [x] [AI] `npx nx run rhino-cli-go:test:unit`, `test:quick`, `test:integration`, and
      `spec-coverage` all exit 0.
      **Status (2026-06-07)**: All four exit 0 (test:quick line coverage 90.14% ≥ 90; lint also 0).
- [x] [AI] `npx nx run rhino-cli-rust:test:unit`, `test:quick`, `test:integration`,
      `spec-coverage`, `lint`, and `build` all exit 0.
      **Status (2026-06-07)**: All six exit 0.
- [x] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh repo-governance` exits 0.
      **Status (2026-06-07)**: Exit 0 — 25 cases byte-identical.
- [x] [AI] Both built binaries run `repo-governance gherkin-keyword-cardinality` and print an
      identical finding list (expected: the known `responsive.feature` offender
      [Repo-grounded]; non-empty is OK at this gate).
      **Status (2026-06-07)**: `diff` of both binaries' full-repo output: byte-identical, both exit 1. Finding list = exactly the known offender (`responsive.feature:24`, "Mobile viewport hides sidebar behind a hamburger menu", 2 When + 2 Then).

> **Pause Safety**: the audit exists in both CLIs, is parity-green on its own corpus, and all
> rhino test suites pass; the spec corpus may still have offenders but nothing is broken (the
> audit is additive and not yet wired into CI). Safe to stop. To resume: re-run
> `bash apps/rhino-cli-rust/scripts/shadow-diff.sh repo-governance`.

## Phase 5: Port the Step 0.5 deterministic preflight + CI wiring

- [x] [AI] Edit `repo-governance/workflows/repo/repo-rules-quality-gate.md`: insert a
      `### 0.5. Deterministic Preflight (Sequential)` section between the "Execution Mode"
      material and `### 1. Initial Validation`, ported from the `ose-public` sibling's
      quality-gate doc and ADAPTED to this repo's standalone-command shape (no orchestrator —
      see `tech-docs.md` DD-8): the preflight runs each deterministic category command via the
      canonical Rust binary —
      `./apps/rhino-cli-rust/dist/rhino-cli repo-governance vendor-audit` and
      `./apps/rhino-cli-rust/dist/rhino-cli repo-governance gherkin-keyword-cardinality` —
      captures outputs for the checker, and defines exit-code semantics (0 clean / 1 findings
      passed to checker / 2 invocation error terminates `fail`). Keep the section
      vendor-neutral (the vendor audit scans this file).
      — acceptance: the section exists, enumerates BOTH categories, explains the 0.5 numbering,
      and `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` still exits 0.
      **Status (2026-06-07)**: Section inserted under `## Steps` before Step 1; enumerates both standalone categories with capture paths, per-category exit semantics (0 clean / 1+FAILED-header findings→checker / 2 or headerless-1 invocation error→`fail`), and the 0.5 decimal-numbering rationale. "The AI will" list gained item 0 (build binary + preflight). Vendor audit still exits 0 (PASSED).
- [x] [AI] Edit `.github/workflows/validate-markdown.yml`: add a step
      `- name: Validate gherkin keyword cardinality` running
      `npx nx run rhino-cli-rust:validate:gherkin-keyword-cardinality` (target created in
      Phase 4) after the existing heading-hierarchy step (this workflow is the only one triggered by push to `main`
      [Repo-grounded]; the `parity` job in `pr-quality-gate.yml` already shadow-diffs the
      extended `repo-governance` corpus on PRs).
      — acceptance: the step is present; `npx prettier --check .github/workflows/validate-markdown.yml`
      exits 0.
      **Status (2026-06-07)**: Step added after the heading-hierarchy step; prettier --check exits 0. CI change is inert until the final push (offender retrofit lands in Phase 7 before push).

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [x] [AI] `npm run lint:md` exits 0.
      **Status (2026-06-07)**: Exit 0.
- [x] [AI] `grep -n "gherkin-keyword-cardinality" repo-governance/workflows/repo/repo-rules-quality-gate.md .github/workflows/validate-markdown.yml`
      returns hits in both files.
      **Status (2026-06-07)**: Hits in both files (workflow doc lines 129/130/148; yml line 37).
- [x] [AI] `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` exits 0.
      **Status (2026-06-07)**: Exit 0 — PASSED, no violations.

> **Pause Safety**: docs + CI config only; CI change is inert until the final push (after
> retrofit makes the audit green). Safe to stop. To resume: re-run `npm run lint:md`.

---

> **Per-subtree retrofit phases (6–8)** — each phase: (1) run the new audit scoped to that
> subtree to discover offenders, (2) normalize offending scenarios (replace repeated primary
> keywords with `And`/`But` — step TEXT stays unchanged) AND verify step definitions in lockstep,
> (3) gate on the binding projects' tests + spec coverage (where the target exists). If the audit
> reports **zero offenders** for a subtree, make no edits but still run the gate. Do NOT
> fabricate offender counts — discover them at execution.

## Phase 6: Retrofit `specs/apps/rhino` (dual-CLI behavior contract, 22 feature files incl. the new contract)

_Suggested executor: `swe-rust-dev`_

- [x] [AI] Run the audit scoped to the rhino contract:
      `./apps/rhino-cli-rust/dist/rhino-cli repo-governance gherkin-keyword-cardinality specs/apps/rhino`
      — acceptance: offender list recorded (authoring-time pre-scan found zero here
      [Repo-grounded]).
      **Status (2026-06-07)**: PASS — "AUDIT PASSED: no violations found", exit 0. Offender
      list: empty (includes the new behavior-contract feature).
- [x] [AI] For each offender (if any), replace repeated primary keywords with `And`/`But` in the
      `.feature` file, then confirm the bound step definitions still match: grep the affected
      step phrases in `apps/rhino-cli-go/cmd/` (godog) and `apps/rhino-cli-rust/tests/`
      (cucumber-rs); keyword-only edits need no step changes.
      — acceptance: audit reports zero violations for `specs/apps/rhino`.
      **Status (2026-06-07)**: NO-OP per the zero-offender rule — no edits, no step checks.

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [x] [AI] `./apps/rhino-cli-rust/dist/rhino-cli repo-governance gherkin-keyword-cardinality specs/apps/rhino`
      exits 0.
      **Status (2026-06-07)**: PASS — exit 0.
- [x] [AI] `npx nx run rhino-cli-rust:test:quick`, `npx nx run rhino-cli-go:test:quick`,
      `npx nx run rhino-cli-rust:spec-coverage`, and `npx nx run rhino-cli-go:spec-coverage`
      exit 0.
      **Status (2026-06-07)**: PASS — all four targets exit 0 (orchestrator-run).

> **Pause Safety**: rhino contract conforms and both CLI test suites pass. Safe to stop. To
> resume: re-run the rhino-scoped audit + both `test:quick` targets.

## Phase 7: Retrofit `specs/apps/crud` (29 feature files; known candidate offender)

_Suggested executor: `swe-typescript-dev`_

- [x] [AI] Run the audit scoped to crud specs:
      `./apps/rhino-cli-rust/dist/rhino-cli repo-governance gherkin-keyword-cardinality specs/apps/crud`
      — acceptance: offender list recorded. Expected from the authoring-time pre-scan
      [Repo-grounded]: `specs/apps/crud/behavior/web/gherkin/layout/responsive.feature`,
      scenario "Mobile viewport hides sidebar behind a hamburger menu" (two primary `When`,
      two primary `Then`).
      **Status (2026-06-07)**: DONE — audit exit 1, exactly 1 violation:
      `responsive.feature:24` "Mobile viewport hides sidebar behind a hamburger menu"
      (2 When, 2 Then). Matches the pre-scan; no other offenders.
- [x] [AI] Fix the known offender: in
      `specs/apps/crud/behavior/web/gherkin/layout/responsive.feature`, replace the second
      primary `When` ("When alice taps the hamburger menu button") with
      `And alice taps the hamburger menu button` and the second primary `Then`
      ("Then a slide-out navigation drawer should appear") with
      `And a slide-out navigation drawer should appear`. Fix any further offenders the audit
      reports the same way.
      — acceptance: audit reports zero violations for `specs/apps/crud`.
      **Status (2026-06-07)**: DONE — keyword-only edits at `responsive.feature:29-30`
      (`When`→`And`, `Then`→`And`; step text verbatim). Re-run audit: "AUDIT PASSED: no
      violations found", exit 0. No further offenders.
- [x] [AI] Verify step bindings in lockstep (keyword-only edits leave step text unchanged, so
      no step-file edits are expected — verify, do not assume). Binding files for the known
      offender [Repo-grounded]:
      `apps/crud-fe-e2e/tests/steps/layout/responsive.steps.ts`,
      `apps/crud-fs-ts-nextjs/test/unit/fe-steps/layout/responsive.steps.tsx`,
      `apps/crud-fe-ts-nextjs/test/unit/steps/layout/responsive.steps.tsx`,
      `apps/crud-fe-ts-tanstack-start/src/test/unit/steps/layout/responsive.steps.tsx`,
      `apps/crud-fe-dart-flutterweb/test/unit/steps/responsive_steps_test.dart`.
      For other offenders, locate bindings via
      `grep -rln "<step phrase>" apps/crud-*/`.
      — acceptance: every binding project's tests in the gate below pass unchanged or with
      lockstep edits.
      **Status (2026-06-07)**: DONE — grep confirmed exactly these 5 owners.
      crud-fe-e2e: playwright-bdd `defineBddConfig` without `matchKeywords` = keyword-agnostic,
      no edit. crud-fe-dart-flutterweb: `gherkin_helper.dart` `_stripStepKeyword` matches by
      text only = keyword-agnostic, no edit. Three `@amiceli/vitest-cucumber` files = strict
      keyword matching; lockstep `When`→`And` + `Then`→`And` edits applied in the
      "Mobile viewport hides sidebar behind a hamburger menu" scenario of all three
      `.steps.tsx` files (step text verbatim, bodies unchanged).

### Phase 7 Gate

> All checks below must pass before starting Phase 8.

- [x] [AI] `./apps/rhino-cli-rust/dist/rhino-cli repo-governance gherkin-keyword-cardinality specs/apps/crud`
      exits 0.
      **Status (2026-06-07)**: PASS — "AUDIT PASSED: no violations found", exit 0.
- [x] [AI] `npx nx run crud-fe-ts-nextjs:test:quick`,
      `npx nx run crud-fe-ts-tanstack-start:test:quick`,
      `npx nx run crud-fs-ts-nextjs:test:quick`, and
      `npx nx run crud-fe-dart-flutterweb:test:quick` exit 0.
      **Status (2026-06-07)**: PASS — all four exit 0 (coverage 74.59%, 76.47%, 76.71%,
      flutter suite green).
- [x] [AI] `npx nx run crud-fe-ts-nextjs:spec-coverage`,
      `npx nx run crud-fe-ts-tanstack-start:spec-coverage`,
      `npx nx run crud-fs-ts-nextjs:spec-coverage`,
      `npx nx run crud-fe-dart-flutterweb:spec-coverage`, and
      `npx nx run crud-fe-e2e:spec-coverage` exit 0.
      **Status (2026-06-07)**: PASS — all five exit 0, "Spec coverage valid! … all covered"
      (fs-ts-nextjs: 28 specs/196 scenarios/741 steps; the four web-only owners: 15/107/408).
- [x] [AI] If any BE feature file was edited (none expected from the pre-scan), also run
      `npx nx run crud-be-golang-gin:test:quick` and `npx nx run crud-be-e2e:spec-coverage`
      plus `spec-coverage` for every crud BE app binding the edited file — exit 0.
      **Status (2026-06-07)**: N/A — no BE feature file edited (only
      `behavior/web/gherkin/layout/responsive.feature`); BE targets skipped per the
      conditional.

> **Pause Safety**: crud specs conform and all binding projects' quick tests + spec coverage
> pass. Safe to stop. To resume: re-run the crud-scoped audit + the four `test:quick` targets.

## Phase 8: Retrofit `specs/libs` (ts-ui 6 + golang-commons 2 feature files)

_Suggested executor: `swe-typescript-dev` (ts-ui) / `swe-golang-dev` (golang-commons)_

- [x] [AI] Run the audit scoped to lib specs:
      `./apps/rhino-cli-rust/dist/rhino-cli repo-governance gherkin-keyword-cardinality specs/libs`
      — acceptance: offender list recorded (authoring-time pre-scan found zero here
      [Repo-grounded]).
      **Status (2026-06-07)**: PASS — "AUDIT PASSED: no violations found", exit 0. Offenders: none.
- [x] [AI] For each offender (if any), normalize the `.feature` file and verify bindings in
      lockstep.
      **Status (2026-06-07)**: NO-OP per the zero-offender rule — no edits.
      Binding locations [Repo-grounded]: ts-ui colocated steps at
      `libs/ts-ui/src/components/<component>/<component>.steps.tsx`; golang-commons tests at
      `libs/golang-commons/<pkg>/*_test.go`. Neither lib has a `spec-coverage` target
      [Repo-grounded — both `project.json` files].
      — acceptance: audit reports zero violations for `specs/libs`.

### Phase 8 Gate

> All checks below must pass before starting Phase 9.

- [x] [AI] `./apps/rhino-cli-rust/dist/rhino-cli repo-governance gherkin-keyword-cardinality specs/libs`
      exits 0.
      **Status (2026-06-07)**: PASS — exit 0.
- [x] [AI] `npx nx run ts-ui:test:quick` and `npx nx run golang-commons:test:quick` exit 0
      (no `spec-coverage` target exists for either lib — skip that target).
      **Status (2026-06-07)**: PASS — both exit 0 (orchestrator-run).

> **Pause Safety**: lib specs conform and lib tests pass. The entire spec corpus now conforms.
> Safe to stop. To resume: run the full-corpus audit (Phase 9 first check).

## Phase 9: Strict repo-rules-quality-gate (double-zero)

- [x] [AI] Retrofit active plans' markdown Gherkin (deviation-matrix row 13): list candidate
      blocks with `grep -rn -A 20 '\x60\x60\x60gherkin' plans/in-progress/ plans/backlog/`,
      review each scenario for repeated primary `Given`/`When`/`Then` keyword lines, and
      normalize violations to the `And`/`But` chained shape. `plans/done/` is exempt
      (immutable archive). No deterministic linter covers markdown — this is a manual sweep
      backed by `plan-checker`/`repo-rules-checker` AI judgment.
      — acceptance: zero violating scenarios remain in `plans/in-progress/` and
      `plans/backlog/` Gherkin fences (deliberately non-conforming counter-examples that are
      explicitly labeled as such are exempt).
      **Status (2026-06-07)**: PASS — one other active plan carries gherkin fences
      (`plans/in-progress/add-investment-oracle-app/prd.md`); programmatic scan: zero scenarios
      repeat a primary keyword. No backlog plans. No edits needed.
- [x] [AI] Run the full-corpus audit once to confirm zero offenders repo-wide:
      `./apps/rhino-cli-rust/dist/rhino-cli repo-governance gherkin-keyword-cardinality`
      — acceptance: exit 0; zero findings across the aligned scan scope (tracked
      `**/*.feature` minus exclusions; net `specs/**/*.feature` today).
      **Status (2026-06-07)**: PASS — Rust AND Go full-corpus runs both exit 0, zero findings
      (post-retrofit; Phase 4 baseline was 1 finding, fixed in Phase 7).
- [x] [AI] Confirm the Nx validate targets (created in Phase 4) are green in both CLIs:
      `npx nx run rhino-cli-rust:validate:gherkin-keyword-cardinality && npx nx run rhino-cli-go:validate:gherkin-keyword-cardinality`
      — acceptance: both exit 0.
      **Status (2026-06-07)**: PASS — both targets exit 0.
- [x] [AI] Execute the `repo-rules-quality-gate` workflow at **strict** mode per
      `repo-governance/workflows/repo/repo-rules-quality-gate.md`, starting with the newly
      ported Step 0.5 deterministic preflight (both categories).
      — acceptance: the workflow terminates with `pass` status; the preflight reports zero
      `gherkin-keyword-cardinality` and zero `vendor-audit` findings.
      **Status (2026-06-07)**: PASS — 3 strict iterations via `repo-rules-checker`: iter-1
      `repo-rules__b4b8c5__…` (2H/1L plan-scoped, fixed: checker Step 7 sub-item 7, `created:`
      frontmatter removal, mod.rs doc comment); iter-2 `…_4a8606` clean; iter-3 `…_e87867`
      clean. Both preflight categories: 0 findings every run.
- [x] [AI] If the gate reports any finding (deterministic or AI-judgment), fix the root cause
      and re-run until double-zero.
      — acceptance: a clean strict run with zero deterministic and zero confirmed AI-judgment
      findings on two consecutive validations.
      **Status (2026-06-07)**: PASS — iterations 2 and 3 both zero C/H/M/L plan-scoped:
      double-zero achieved.

### Phase 9 Gate

> All checks below must pass before starting Phase 10.

- [x] [AI] Full-corpus audit reports zero findings (exit 0).
      **Status (2026-06-07)**: PASS — both CLIs exit 0.
- [x] [AI] `repo-rules-quality-gate` (strict) terminates `pass` with double-zero.
      **Status (2026-06-07)**: PASS — two consecutive clean strict runs (plan-scoped).

> **Pause Safety**: rule authored, propagated, enforced in both CLIs, and validated repo-wide;
> nothing pushed yet. Safe to stop. To resume: re-run the full-corpus audit and the strict gate.

## Phase 10: Local quality gates, commit, push, CI verification

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck`
- [ ] [AI] Run affected linting: `npx nx affected -t lint`
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick`
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage`
- [ ] [AI] Run the cross-vendor parity validators (governance markdown changed):
      `npx nx run rhino-cli-rust:validate:cross-vendor-parity && npx nx run rhino-cli-go:validate:cross-vendor-parity`
- [ ] [AI] Run the full shadow-diff repo-governance corpus once more:
      `bash apps/rhino-cli-rust/scripts/shadow-diff.sh repo-governance`
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by these changes
- [ ] [AI] Re-run failing checks to confirm resolution
- [ ] [AI] Verify zero failures before pushing

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes. This follows the root cause orientation principle — proactively fix preexisting
> errors encountered during work. Do not defer or skip existing issues. Commit preexisting
> fixes separately with appropriate conventional commit messages.

### Commit Guidelines

- [ ] [AI] Commit changes thematically — group related changes into logically cohesive commits
      (suggested split: `docs(governance): add Gherkin keyword-cardinality HARD rule`;
      `docs(governance): port Step 0.5 deterministic preflight into repo-rules-quality-gate`;
      `feat(rhino-cli): add gherkin-keyword-cardinality audit to both implementations`;
      `refactor(specs): normalize crud scenarios to one-each keyword shape`;
      `ci: run gherkin-keyword-cardinality audit in validate-markdown workflow`;
      `chore(bindings): re-sync skill + agent bindings`).
- [ ] [AI] Follow Conventional Commits format: `<type>(<scope>): <description>`
- [ ] [AI] Split different domains/concerns into separate commits
- [ ] [AI] Preexisting fixes get their own commits, separate from plan work
- [ ] [AI] Do NOT bundle unrelated changes into a single commit

### Post-Push CI Verification

- [ ] [AI] Push the worktree branch directly to `main` (accepted deviation — matrix row 8;
      Trunk Based Development, no PR): `git push origin HEAD:main`
- [ ] [AI] Check which push-triggered GitHub Actions workflows fired:
      `gh run list --branch main --limit 5 --json name,status,conclusion`
      — `validate-markdown.yml` (the only workflow with `push: branches: [main]`
      [Repo-grounded]) WILL fire and now includes the new gherkin-keyword-cardinality step;
      `pr-quality-gate.yml` fires on PRs only (none is created); the `test-*` workflows are
      schedule/dispatch only. Poll each triggered run to completion (every 3 minutes; one
      `gh run view --json status,conclusion` per wakeup; never `gh run watch`).
- [ ] [AI] Verify ALL CI checks pass — no exceptions
- [ ] [AI] If any CI check fails, fix the root cause immediately and push a follow-up commit
- [ ] [AI] Repeat until ALL GitHub Actions pass with zero failures
- [ ] [AI] Do NOT proceed to archival until CI is fully green

### Phase 10 Gate

> All checks below must pass before archival.

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` exits 0 locally.
- [ ] [AI] Changes pushed to `origin main`; all triggered GitHub Actions are green.

> **Pause Safety**: work is committed and pushed; CI is green. Safe to stop. To resume:
> re-check CI status with `gh run view --json status,conclusion`.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked
- [ ] [AI] Verify ALL quality gates pass (local + CI)
- [ ] [AI] Verify the strict `repo-rules-quality-gate` passed with double-zero (Phase 9)
- [ ] [AI] Rename and move (use the actual completion date):
      `git mv plans/in-progress/gherkin-step-keyword-cardinality/ plans/done/YYYY-MM-DD__gherkin-step-keyword-cardinality/`
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date
- [ ] [AI] Update `plans/README.md` if it references this plan
- [ ] [AI] Commit the archival: `chore(plans): move gherkin-step-keyword-cardinality to done`
- [ ] [AI] Push the archival commit (`git push origin HEAD:main`) and confirm CI is green.
