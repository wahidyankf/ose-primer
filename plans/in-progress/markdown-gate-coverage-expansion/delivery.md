# Delivery Checklist — Markdown Gate Coverage Expansion

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.
>
> **Dual-CLI rule** — every behavior change lands in BOTH `apps/rhino-cli-rust/` AND
> `apps/rhino-cli-go/` in the same thematic commit, per
> [parity convention Rule 1](../../../repo-governance/conventions/structure/rhino-cli-dual-implementation-parity.md).
> Never commit a phase with only one implementation updated.

## Worktree

Worktree path: `worktrees/markdown-gate-coverage-expansion/`

Provision before execution (run from repo root):

```bash
claude --worktree markdown-gate-coverage-expansion
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Push / Definition of Done

- **Push target**: `origin main`, **direct** (Trunk Based Development — no PR). [Repo-grounded —
  `main` is the trunk; see the
  [Git Push Default Convention](../../../repo-governance/development/workflow/git-push-default.md)]
- **DoD**: all three markdown gates report zero blocking findings within their scopes (mermaid
  repo-wide−exclusions; links repo-wide−exclusions with anchors validated; heading-hierarchy on
  the prose allowlist); the gates are enforced across all THREE layers — pre-commit staged-only
  (Layer 1), the consolidated `validate-markdown.yml` on `pull_request` to `main` (Layer 2), and
  the same workflow on `push` to `main` (Layer 3); the mermaid trigger is removed from
  `.husky/pre-push`; `pr-validate-links.yml` is deleted and migrated; BOTH CLIs implement every
  behavior change with `shadow-diff.sh docs` byte-parity green; all preexisting tests in both
  CLIs stay green; new behavior (links/mermaid `--exclude`, repo-wide scans, the mermaid
  pipe-label + cycle parser fixes, `broken-anchor`
  anchor validation, shared heading parser + GFM slug helper, the greenfield heading-hierarchy
  validator with prose allowlist, staged-only pre-commit steps) is fully tested in both CLIs; the
  rhino BDD specs under `specs/apps/rhino/` are updated in lockstep (links / heading / mermaid /
  git-pre-commit `.feature` files + the NEW `component-cli.md`) and BOTH `spec-coverage` gates
  are green; `diagrams.md` / `quality.md` / `linking.md` / `repository-validation.md` are
  accurate and propagated **via `repo-rules-maker`** with a strict `repo-rules-quality-gate`
  **double-zero**; this plan's own diagrams, links, anchors, and prose headings pass (dogfooding);
  the plan is archived to `plans/done/`.

> **Important (fix-all-issues)**: Fix ALL failures found during quality gates, not just those
> caused by your changes. This follows the root-cause-orientation principle — proactively fix
> preexisting errors encountered during work. Do not defer or skip existing issues. Commit
> preexisting fixes separately with appropriate conventional commit messages.

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [x] [AI] Provision the worktree from repo root:
      `claude --worktree markdown-gate-coverage-expansion`
      — acceptance: `worktrees/markdown-gate-coverage-expansion/` exists.
  - _Done 2026-06-06. Status: complete. Provisioned via EnterWorktree (WorktreeCreate hook routed to `worktrees/markdown-gate-coverage-expansion/`). Verified: `git worktree list` shows the path on branch `worktree/markdown-gate-coverage-expansion`._
- [x] [AI] Initialize the toolchain in the **root** worktree (not the new worktree):
      `npm install && npm run doctor -- --fix`
      — acceptance: both exit 0; `node_modules/` synchronized; no unresolved toolchain drift.
      (See
      [Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md).)
  - _Done 2026-06-06. Status: complete. `npm install` exit 0; `npm run doctor -- --fix` exit 0 — 18/19 tools OK, 1 warning (python v3.13.1 < 3.13.12; doctor reports "Nothing to fix"), 0 missing._
- [x] [AI] Build the Rust CLI:
      `cargo build --release --quiet --manifest-path apps/rhino-cli-rust/Cargo.toml`
      — acceptance: exits 0.
  - _Done 2026-06-06. Status: complete. Build exit 0 (cache hit — 0 crates recompiled)._
- [x] [AI] Build the Go CLI: `npx nx run rhino-cli-go:build`
      — acceptance: exits 0; `apps/rhino-cli-go/dist/rhino-cli` exists.
  - _Done 2026-06-06. Status: complete. Build exit 0; binary present at `apps/rhino-cli-go/dist/rhino-cli`._
- [x] [AI] Run the Rust test suite to establish the green baseline:
      `npx nx run rhino-cli-rust:test:quick`
      — acceptance: baseline pass count recorded; all preexisting failures documented.
  - _Done 2026-06-06. Status: complete. Baseline: 527 passed, 0 failed, 0 ignored. No preexisting failures._
- [x] [AI] Run the Go test suite to establish the green baseline:
      `npx nx run rhino-cli-go:test:quick`
      — acceptance: baseline pass count recorded; all preexisting failures documented.
  - _Done 2026-06-06. Status: complete. Baseline: all 14 packages ok (cmd 83.2%, docs 91.6%, git 95.0%, mermaid 95.7% coverage, etc.). No preexisting failures._
- [x] [AI] Run the parity harness to confirm the docs corpus is currently byte-identical:
      `bash apps/rhino-cli-rust/scripts/shadow-diff.sh docs`
      — acceptance: exits 0.
  - _Done 2026-06-06. Status: complete. Shadow diff PASS — 31 cases byte-identical; exit 0._
- [x] [AI] Capture the **mermaid** baseline (current four-dir scope):
      `npx nx run rhino-cli-rust:validate:mermaid`
      — acceptance: pass/fail + findings recorded in phase notes.
  - _Done 2026-06-06. Status: complete. PASS (exit 0): 0 violations, 1 warning (`docs/reference/system-architecture/applications.md` block 0 line 78 — subgraph_density 11 children > 6) across 167 files / 571 blocks._
- [x] [AI] Capture the **current link** baseline (current three-dir scope):
      `cargo run --release --quiet --manifest-path apps/rhino-cli-rust/Cargo.toml -- docs validate-links -o json`
      — acceptance: `total_files`, broken-link count, and any broken links recorded in phase
      notes.
  - _Done 2026-06-06. Status: complete. Baseline: total_files=608, broken-link count=0 (exit 0)._
- [ ] [AI] Establish a **provisional repo-wide link backlog** with the CURRENT binary (still
      three-dir scope) by grepping for relative markdown links in the not-yet-scanned trees:
      `grep -rnoE '\]\([^)#][^)]*\.md(#[^)]*)?\)' plans/ apps/ libs/ specs/ --include='*.md' --exclude-dir=node_modules --exclude-dir=deps --exclude-dir=_build --exclude-dir=plans/done 2>/dev/null | head -100`
      — acceptance: a provisional per-tree list of relative links (with `#anchor` ones flagged)
      recorded in phase notes. Estimate only; the authoritative backlog is re-measured per tree
      once the widened link checker + anchor validation land (Phase 1).
  - _Done 2026-06-06. Status: complete. Provisional relative-md-link counts (excl `plans/done/`): apps=116, libs=3, plans=101, specs=103 — 323 total, 27 carrying `#anchor` fragments. Estimate only; authoritative re-measure in Phases 6-10._
- [ ] [AI] Establish a **provisional prose-heading backlog** (no heading validator exists yet —
      grep-based estimate): for each allowlist tree, list files whose count of `^#` lines
      differs from 1:
      `for f in $(find docs repo-governance specs -name '*.md'; find plans -name '*.md' -not -path 'plans/done/*'; ls *.md; ls apps/*/README.md libs/*/README.md 2>/dev/null; find apps/*/docs libs/*/docs -name '*.md' 2>/dev/null); do n=$(grep -c '^# ' "$f" 2>/dev/null || echo 0); [ "$n" -ne 1 ] && echo "$n $f"; done | sort -rn | head -60`
      — acceptance: provisional duplicate-H1 / missing-H1 candidate list recorded in phase notes
      (skipped-level estimation is deferred to the real validator in Phase 2; expect
      false positives from `#` inside code fences — this is an estimate only).
  - _Done 2026-06-06. Status: complete. 60+ candidate files with `^#` count ≠ 1; top: `docs/.../python/anti-patterns.md` (264), `python/idioms.md` (155), `repo-governance/development/workflow/best-practices.md` (123), `apps/rhino-cli-go/README.md` (120). Nearly all are `#` comment lines inside code fences (expected false positives) — authoritative measure deferred to the fence-aware Phase 2 validator._
- [x] [AI] Confirm `.claude/`/`.opencode/` files would violate heading rules (proof the allowlist
      is needed), using the same grep over the denied trees:
      `for f in .claude/agents/*.md .claude/skills/*/SKILL.md .opencode/agents/*.md; do n=$(grep -c '^# ' "$f" 2>/dev/null || echo 0); [ "$n" -ne 1 ] && echo "$n $f"; done | head -20`
      — acceptance: at least one agent/skill file with ≠1 H1 recorded (fixture candidates for the
      Phase 2 allowlist tests).
  - _Done 2026-06-06. Status: complete. 20 files with ≠1 `^#` count, e.g. `.claude/agents/repo-rules-fixer.md` (18), `.claude/skills/plan-writing-gherkin-criteria/SKILL.md` (17), `.claude/agents/readme-fixer.md` (7) — allowlist need confirmed; fixture candidates recorded._
- [x] [AI] Resolve all preexisting failures before proceeding
      — acceptance: no preexisting failures remain unresolved.
  - _Done 2026-06-06. Status: complete. No preexisting failures found: Rust 527/527 pass, Go 14/14 packages ok, shadow-diff PASS (31 cases), mermaid 0 violations, links 0 broken. Nothing to resolve._

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [x] [AI] `cargo build --release --quiet --manifest-path apps/rhino-cli-rust/Cargo.toml` and
      `npx nx run rhino-cli-go:build` both exit 0.
  - _Done 2026-06-06. Both builds exit 0 (verified above; Go binary present)._
- [x] [AI] `npx nx run rhino-cli-rust:test:quick` and `npx nx run rhino-cli-go:test:quick` are
      both green; baselines recorded.
  - _Done 2026-06-06. Rust 527 passed / 0 failed; Go 14/14 packages ok. Baselines recorded in items above._
- [x] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh docs` exits 0.
  - _Done 2026-06-06. PASS — 31 cases byte-identical, exit 0._
- [x] [AI] Provisional per-tree link, anchor, and prose-heading backlogs recorded in phase notes.
  - _Done 2026-06-06. Link backlog (323 links, 27 anchored, per tree), prose-heading candidates (60+ files, fence false-positives noted), and `.claude/`/`.opencode/` violation proof recorded in the items above and the Phase 0 notes below._

> **Pause Safety**: only the toolchain was verified and baselines recorded — no source changed.
> Safe to stop indefinitely. To resume: re-run both `test:quick` targets and confirm they are
> still green.

**Phase 0 notes** (executor fills in): Baselines 2026-06-06 — Rust test:quick 527 passed / 0
failed; Go test:quick all 14 packages ok (docs 91.6%, git 95.0%, mermaid 95.7% coverage);
shadow-diff docs PASS 31 cases; mermaid current-scope 0 violations + 1 warning
(`docs/reference/system-architecture/applications.md` subgraph_density); links current-scope
608 files / 0 broken. Provisional backlogs — relative md links (excl `plans/done/`): apps=116,
plans=101, specs=103, libs=3 (323 total, 27 with anchors); prose-heading grep estimate: 60+
files ≠1 H1 (mostly code-fence false positives); `.claude/`/`.opencode/` denied trees: 20 files
≠1 H1 (allowlist need proven). No preexisting failures.

---

## Phase 1: Link Checker — `--exclude`, Repo-Wide Scan, Anchors (TDD, both CLIs)

> _Suggested executors: `swe-rust-dev` (Rust), `swe-golang-dev` (Go)._

Implement DD-2 (`--exclude` on links), DD-3 (repo-wide walk minus noise dirs), DD-5 (GFM slug
helper), DD-6 (shared fence-aware heading parser), and DD-4 (`broken-anchor` validation) — in
BOTH CLIs. The `.claude/skills/` and `.opencode/skill/` skips stay.

- [x] [AI] **SPEC (RED)** — Extend
      `specs/apps/rhino/behavior/cli/gherkin/docs/docs-validate-links.feature` with five
      scenarios (one `--exclude`, one repo-wide-scan, one `broken-anchor`, one valid-anchor, one
      same-file-anchor), each obeying the one-`Given`/one-`When`/one-`Then` cardinality norm; and
      create `specs/apps/rhino/components/cli/component-cli.md` (_New file_; the stub
      `components/cli/README.md` explicitly reserves this name) documenting the current
      `docs validate-links`/`validate-mermaid` command + flag inventory plus the new `--exclude`.
      Run `npx nx run rhino-cli-go:spec-coverage`
      — acceptance: spec-coverage FAILS listing the new unmatched steps (the spec-level RED).
  - _Suggested executor: `specs-maker` for `component-cli.md`; `swe-rust-dev` for the feature
    file._
  - _Done 2026-06-06. Status: complete. Files: `specs/apps/rhino/behavior/cli/gherkin/docs/docs-validate-links.feature` (+5 scenarios, 1-Given/1-When/1-Then each), `specs/apps/rhino/components/cli/component-cli.md` (new), `specs/apps/rhino/components/cli/README.md` (stub link). Acceptance met: `rhino-cli-go:spec-coverage` FAILS with 8 step gaps, all in the 5 new scenarios. NOTE: achieving the RED required fixing two preexisting spec-coverage extractor bugs in BOTH CLIs (comment-line catch-all patterns; const-indirected registrations invisible) — committed separately as `fix(rhino-cli): harden spec-coverage go-step extraction (rust+go)`; shadow-diff spec-coverage corpus 9 cases byte-identical._
- [x] [AI] **RED (Rust)** — Add failing unit tests in
      `apps/rhino-cli-rust/src/internal/docs/scanner.rs`, `validator.rs`, and the new
      `headings.rs` (_New test_, temp-dir fixtures) covering:
      (a) `--exclude plans/done` removes a broken link under `plans/done` from results while a
      broken link elsewhere is still reported;
      (b) a repo-wide scan finds a broken link under `libs/` (not in today's 3-dir set) and skips
      files under `node_modules/`, `generated-reports/`, and `worktrees/`;
      (c) `[X](./target.md#missing-section)` where `target.md` exists but has no heading slugging
      to `missing-section` yields a `broken-anchor` finding;
      (d) `[X](./target.md#real-section)` where `target.md` has `## Real Section` yields NO
      finding;
      (e) the slug helper maps duplicate `Setup` headings to `setup` and `setup-1`, keeps
      underscores (`snake_case naming` → `snake_case-naming`), keeps Unicode letters, does NOT
      collapse double spaces, and strips backticks;
      (f) a same-file pure-anchor link to `#own-section` with no matching heading yields a
      `broken-anchor`;
      (g) the fence-aware parser ignores `#` lines inside code fences.
      Run `npx nx run rhino-cli-rust:test:quick`
      — acceptance: all new tests FAIL; all preexisting tests still pass.
  - _Done 2026-06-06. Status: complete. Files: `scanner.rs` (+1 test), `validator.rs` (+4 tests), `headings.rs` (new, stub fns + 6 tests), `mod.rs` (registration). 530 passed / 10 failed — failures are exactly the new tests ((d) is a passing non-regression guard); all preexisting green._
- [x] [AI] **GREEN (Rust)** — Implement per
      [tech-docs.md](./tech-docs.md) DD-2/3/4/5/6:
      (1) add `--exclude` to `ValidateLinksArgs` in
      `apps/rhino-cli-rust/src/commands/docs.rs` and APPEND the values to
      `skip_paths` after the existing `.opencode/skill/` entry (line 67 — do not replace it);
      (2) replace the three-dir loop in `scanner.rs:102-135` with a repo-wide `walkdir` walk
      whose `filter_entry` drops the standardized cross-repo noise-skip set (`node_modules,
dist, target, .next, coverage, generated-reports, local-temp, archived, apps-labs,
worktrees, .terraform, generated-contracts, .nx`, plus `.git`);
      (3) remove `#` from the extraction skip at `scanner.rs:167-174` so pure-anchor links are
      extracted;
      (4) create `apps/rhino-cli-rust/src/internal/docs/headings.rs` (_New file_) with
      `collect_atx_headings` (fence-aware, returns line/level/title) and the GFM slug helper
      (lowercase; strip chars outside `[\p{L}\p{N}_\- ]` via the `regex` crate; spaces→hyphens
      uncollapsed; `-N` collision suffixes); register it in `internal/docs/mod.rs`;
      (5) in `validator.rs`, capture the `#fragment` before `resolve_link` strips it and emit
      `BrokenLink { category: "broken-anchor", .. }` when the slug set lacks the fragment;
      (6) add cucumber step definitions for the new scenarios in
      `apps/rhino-cli-rust/tests/docs.rs`.
      Run `npx nx run rhino-cli-rust:test:quick && npx nx run rhino-cli-rust:test:integration`
      — acceptance: all tests (new + preexisting) pass.
  - _Done 2026-06-06. Status: complete. Files: `commands/docs.rs` (--exclude appended after `.opencode/skill/`), `scanner.rs` (walkdir repo-wide walk + NOISE_DIRS const; `#` extraction skip removed), `headings.rs` (fence-aware `collect_atx_headings`, GFM `gfm_slug`, `collect_heading_anchors` with -N suffixes), `validator.rs` (fragment captured pre-resolve; `broken-anchor` category incl. same-file anchors), `reporter.rs` (broken-anchor in CATEGORY_ORDER), `tests/docs.rs` (5 new cucumber scenarios wired). test:quick 540 passed/0 failed (90% cov gate ok); test:integration docs suite 32 scenarios/122 steps all pass. Deviations: one preexisting test asserting old anchor-drop behavior updated to the mandated new behavior; `links_args_default` extended for the new field._
- [x] [AI] **REFACTOR (Rust)** — Consolidate the slug + anchor + walk helpers; keep the heading
      parser in one place. Run
      `npx nx run rhino-cli-rust:lint && npx nx run rhino-cli-rust:test:quick`
      — acceptance: both exit 0; no clippy warnings introduced.
  - _Done 2026-06-06. Status: complete. GREEN code already cohesive: anchor-set logic only in `headings.rs`; single NOISE_DIRS walk in `scanner.rs`; fence-toggle divergence (scanner `only vs headings`+~~~) is deliberate Go-parity, now documented. Comment-only fixes in `headings.rs` + `mod.rs`. lint exit 0 (clippy -D warnings clean); test:quick 540 passed/0 failed (cache-skipped)._
- [x] [AI] **RED (Go)** — Add the same failing unit tests (fixtures identical to the Rust set, a–g)
      in `apps/rhino-cli-go/internal/docs/links_scanner_test.go`, `links_validator_test.go`, and
      the new `headings_test.go` (_New test_). Run `npx nx run rhino-cli-go:test:quick`
      — acceptance: all new tests FAIL; all preexisting tests still pass.
  - _Done 2026-06-06. Status: complete. Files: `headings.go` (new, panic stubs `CollectATXHeadings`/`GFMSlug`/`CollectHeadingAnchors`), `headings_test.go` (new), `links_scanner_test.go` (+2 tests), `links_validator_test.go` (+4 tests). internal/docs: 63 passed / 12 failed — failures exactly the new RED tests; (d) is the intentional passing guard mirroring Rust. All other packages ok; gofmt/vet clean._
- [x] [AI] **GREEN (Go)** — Mirror the Rust implementation:
      (1) `--exclude` (`StringArrayVar`) in `apps/rhino-cli-go/cmd/docs_validate_links.go`,
      appended to `ScanOptions.SkipPaths`;
      (2) repo-wide walk replacing `getAllMarkdownFiles` (`links_scanner.go:78`) with the same
      noise-skip set;
      (3) remove the `#` skip at `links_scanner.go:121-126`;
      (4) create `apps/rhino-cli-go/internal/docs/headings.go` (_New file_) with
      `CollectATXHeadings` + the GFM slug helper (`regexp` class `[^\p{L}\p{N}_\- ]`);
      (5) `broken-anchor` category in `links_validator.go` (capture fragment before
      `ResolveLink` strips it at line 13);
      (6) add godog step definitions for the new scenarios in
      `apps/rhino-cli-go/cmd/docs_validate_links.integration_test.go`.
      Run `npx nx run rhino-cli-go:test:quick && npx nx run rhino-cli-go:spec-coverage`
      — acceptance: all tests pass; spec-coverage exits 0 (the Phase 1 SPEC scenarios are now
      covered).
  - _Done 2026-06-06. Status: complete. Files: `cmd/docs_validate_links.go` (--exclude StringArrayVar appended after `.opencode/skill/`), `links_scanner.go` (WalkDir repo-wide + noiseDirs set, root-guard, # skip removed), `headings.go` (full impl mirroring Rust), `links_validator.go` (fragment pre-capture + broken-anchor, anchorExists), `links_reporter.go` (broken-anchor appended to categoryOrder), `cmd/docs_validate_links.integration_test.go` (5 scenarios wired). test:quick PASS (cov 90.06%); spec-coverage PASS (608 steps all covered). Deviation: legacy `TestExtractLinks` updated to mandated new pure-anchor behavior._
- [x] [AI] **REFACTOR (Go)** — Same consolidation pass as Rust. Run
      `npx nx run rhino-cli-go:lint && npx nx run rhino-cli-go:test:quick`
      — acceptance: both exit 0; no new golangci-lint findings.
  - _Done 2026-06-06. Status: complete. Already cohesive (anchor logic only in headings.go; single noiseDirs walk; one parser). Comment-only fix in `links_scanner.go` documenting deliberate fence-toggle divergence. lint exit 0 (0 issues); test:quick exit 0 (cov 90.06%)._
- [x] [AI] **PARITY** — Extend the `docs` corpus in
      `apps/rhino-cli-rust/scripts/shadow-diff.sh` with invocations exercising
      `validate-links --exclude` and anchor fixtures, then run
      `bash apps/rhino-cli-rust/scripts/shadow-diff.sh docs`
      — acceptance: exits 0 (byte-identical output across both CLIs).
  - _Done 2026-06-06. Status: complete. +9 corpus cases (anchor fixtures text/json/markdown/quiet/verbose; --exclude single/repeat/other) under `.shadow-links-fixtures/`. PASS — 40 cases byte-identical (was 31). Parity bug found+fixed: Rust `VALIDATE_LINKS_USAGE` const lacked the `--exclude` flag line cobra auto-generates — mirrored Go bytes. Note: live repo currently shows 374 broken links repo-wide (expected backlog; cleaned in Phases 6-10)._

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [x] [AI] `npx nx run rhino-cli-rust:test:quick` and `npx nx run rhino-cli-go:test:quick` are
      both green (new link/anchor/exclude tests + all preexisting).
  - _Done 2026-06-06. Rust 540 passed/0 failed; Go 14/14 packages ok (cache-skipped runs)._
- [x] [AI] `npx nx run rhino-cli-rust:lint` and `npx nx run rhino-cli-go:lint` both exit 0.
  - _Done 2026-06-06. Both exit 0 (cache-skipped)._
- [x] [AI] `npx nx run rhino-cli-rust:spec-coverage` and `npx nx run rhino-cli-go:spec-coverage`
      both exit 0.
  - _Done 2026-06-06. Both: 20 specs, 145 scenarios, 608 steps all covered (cache-skipped)._
- [x] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh docs` exits 0.
  - _Done 2026-06-06. PASS — 40 cases byte-identical._

> **Pause Safety**: both link checkers now support `--exclude`, repo-wide scan, and anchor
> validation, but nothing new is wired into hooks/CI (Phase 5) — repo enforcement is unchanged.
> Safe to stop. To resume: re-run both `test:quick` targets.

---

## Phase 2: Heading-Hierarchy Validator — Greenfield (TDD, both CLIs)

> _Suggested executors: `swe-rust-dev` (Rust), `swe-golang-dev` (Go)._

Implement DD-7: build `docs validate-heading-hierarchy` from scratch in BOTH CLIs — three finding
kinds (`missing-h1`, `duplicate-h1`, `skipped-level`), the `is_prose_allowlisted` predicate
inside file selection (`docs/`, `repo-governance/`, `specs/`, `plans/`−`done/`, root `*.md`,
`apps/*/README.md`, `libs/*/README.md`, `apps/*/docs/**`, `libs/*/docs/**`; default-deny
everything else), optional positional PATH args (allowlist still applied), and a repeatable
`--exclude` flag.

- [x] [AI] **SPEC (RED)** — Create
      `specs/apps/rhino/behavior/cli/gherkin/docs/docs-validate-heading-hierarchy.feature`
      (_New file_) with scenarios for: duplicate-H1 in `docs/` flagged; missing-H1 flagged;
      skipped-level flagged; `.claude/agents/` file exempt (default-deny); `SKILL.md` exempt;
      `plans/done/` excluded; app README included; deep app internal path excluded; `--exclude`
      honored — each obeying the keyword-cardinality norm. Extend
      `specs/apps/rhino/components/cli/component-cli.md` with the new command + flags. Run
      `npx nx run rhino-cli-go:spec-coverage`
      — acceptance: spec-coverage FAILS listing the new unmatched steps.
  - _Done 2026-06-06. Status: complete. Files: `docs-validate-heading-hierarchy.feature` (new, 9 scenarios, cardinality-compliant), `component-cli.md` (+validate-heading-hierarchy inventory). spec-coverage FAILS with 29 unmatched steps, all in the new feature — RED confirmed._
- [x] [AI] **RED (Rust)** — Add failing unit tests in the new
      `apps/rhino-cli-rust/src/internal/docs/heading_hierarchy.rs` (_New file_, tests-first with
      temp-dir fixtures) covering:
      (a) a `docs/` file with two H1s yields `duplicate-h1`;
      (b) a `docs/` file with zero H1s yields `missing-h1`;
      (c) a `docs/` file jumping `#` → `###` yields `skipped-level`;
      (d) headings inside code fences are ignored (no false finding);
      (e) a `.claude/agents/` file with zero H1s yields NO finding (default-deny);
      (f) a `SKILL.md` under `.claude/skills/` with many H1s yields NO finding;
      (g) a `plans/done/` file with a skipped level yields NO finding;
      (h) a `plans/in-progress/` file with a duplicate H1 yields a finding;
      (i) an `apps/example/README.md` with a skipped level yields a finding (allowlist) while an
      `apps/example/src/notes.md` with zero H1s yields NO finding (deny);
      (j) a `specs/` file with a duplicate H1 yields a finding;
      (k) `--exclude docs` suppresses `docs/` findings while other allowlist trees still report.
      Run `npx nx run rhino-cli-rust:test:quick`
      — acceptance: new tests FAIL (module compiles, assertions fail); preexisting tests pass.
  - _Done 2026-06-06. Status: complete. Files: `heading_hierarchy.rs` (new — stub API `is_prose_allowlisted` + `validate_heading_hierarchy` + finding kinds, 20 failing tests covering a–k + allowlist predicate), `mod.rs` (registration). 540 passed / 20 failed — failures exactly the new tests (unimplemented! stubs)._
- [x] [AI] **GREEN (Rust)** — Implement:
      (1) the Gate C engine in `heading_hierarchy.rs` reusing `collect_atx_headings` from
      `headings.rs` (Phase 1) — finding kinds `missing-h1`/`duplicate-h1`/`skipped-level`;
      (2) `is_prose_allowlisted(repo_rel: &str) -> bool` applied to every candidate file in the
      full-scan walk AND to positional-path/staged inputs;
      (3) the clap command: add `ValidateHeadingHierarchy` to `DocsCommands` in
      `apps/rhino-cli-rust/src/cli.rs` (enum at lines 168-175, dispatch at 238-243) and the
      args/handler in `commands/docs.rs` (optional positional `PATH`s, `--staged-only`,
      repeatable `--exclude`; text/json/markdown output via the global `-o` flag; non-zero exit
      on findings — mirror `run_validate_links`);
      (4) cucumber step definitions in `apps/rhino-cli-rust/tests/docs.rs`.
      Run `npx nx run rhino-cli-rust:test:quick && npx nx run rhino-cli-rust:test:integration`
      — acceptance: all tests pass; the 90% coverage gate stays green (logic lives in
      `internal/docs/`, coverage-gated).
  - _Done 2026-06-06. Status: complete. Files: `heading_hierarchy.rs` (engine reusing `collect_atx_headings`; allowlist + exclude in `collect_candidate_rels`; text/json/markdown reporters), `scanner.rs` (walker shared via pub(crate)), `cli.rs` (+variant/dispatch), `commands/docs.rs` (args/handler/USAGE byte-matching cobra), `tests/docs.rs` (9 scenarios wired). test:quick PASS (570 tests, 90% gate green); test:integration PASS (docs 41 scenarios/159 steps). fmt+clippy clean._
- [x] [AI] **REFACTOR (Rust)** — Keep the allowlist + exclude logic in one cohesive place; align
      doc comments with module style. Run
      `npx nx run rhino-cli-rust:lint && npx nx run rhino-cli-rust:test:quick`
      — acceptance: both exit 0; no clippy warnings introduced.
  - _Done 2026-06-06. Status: complete. Already cohesive (single retain point; pure allowlist predicate; reporters mirror link pattern). One doc-comment alignment on `is_excluded`. lint exit 0; test:quick 570 passed/0 failed (fresh run after nx reset)._
- [x] [AI] **RED (Go)** — Add the same failing unit tests (fixtures identical to Rust, a–k) in
      the new `apps/rhino-cli-go/internal/docs/heading_hierarchy_test.go` (_New test_). Run
      `npx nx run rhino-cli-go:test:quick`
      — acceptance: new tests FAIL; preexisting tests pass.
  - _Done 2026-06-06. Status: complete. Files: `heading_hierarchy.go` (new, panic stubs `IsProseAllowlisted`/`ValidateHeadingHierarchy` + kind constants), `heading_hierarchy_test.go` (new, a–k mirroring Rust byte-for-byte + 21 allowlist subtests). 12 new tests FAIL via panic→fail helper; all preexisting pass; gofmt/vet clean._
- [x] [AI] **GREEN (Go)** — Mirror the Rust implementation:
      (1) engine + `IsProseAllowlisted` in
      `apps/rhino-cli-go/internal/docs/heading_hierarchy.go` (_New file_) reusing
      `CollectATXHeadings`;
      (2) cobra command in `apps/rhino-cli-go/cmd/docs_validate_heading_hierarchy.go`
      (_New file_), registered on the `docs` parent (`cmd/docs.go`), flags mirroring Rust;
      (3) godog step definitions in a new
      `apps/rhino-cli-go/cmd/docs_validate_heading_hierarchy.integration_test.go` (_New test_).
      Run `npx nx run rhino-cli-go:test:quick && npx nx run rhino-cli-go:spec-coverage`
      — acceptance: all tests pass; spec-coverage exits 0.
  - _Done 2026-06-06. Status: complete. Files: `heading_hierarchy.go` (engine + IsProseAllowlisted + reporter trio), `cmd/docs_validate_heading_hierarchy.go` (new cobra cmd), `.integration_test.go` (9 godog scenarios), `_test.go` (unit suite for coverage gate), `testable.go` (+injection var), `steps_common_test.go` (+20 step constants). test:quick PASS (cov 90.20%); spec-coverage exit 0 (21 specs, 154 scenarios, 645 steps). Byte-parity sanity vs Rust on shared fixtures: json/text/markdown stdout, exit codes, stderr all IDENTICAL._
- [x] [AI] **REFACTOR (Go)** — Same consolidation pass. Run
      `npx nx run rhino-cli-go:lint && npx nx run rhino-cli-go:test:quick`
      — acceptance: both exit 0; no new golangci-lint findings.
  - _Done 2026-06-06. Status: complete. Already cohesive — zero code changes (GREEN had incorporated the Rust REFACTOR's comment alignment). lint exit 0 (0 issues); test:quick exit 0 (internal/docs 92.4%, overall 90.20%)._
- [x] [AI] **PARITY** — Extend the shadow-diff `docs` corpus with
      `validate-heading-hierarchy` invocations (full scan + `--exclude` + positional-path
      variants), then run `bash apps/rhino-cli-rust/scripts/shadow-diff.sh docs`
      — acceptance: exits 0.
  - _Done 2026-06-06. Status: complete. +26 corpus cases (full-scan, fixtures under `docs/.shadow-heading-fixtures/` since dot-dirs at root are default-denied, positional paths, denied-tree, --exclude variants). PASS — 67 cases byte-identical (was 41). Zero divergences on first run._

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [x] [AI] Both `test:quick` targets green; both `lint` targets exit 0; both `spec-coverage`
      targets exit 0.
  - _Done 2026-06-06. Rust 570/0, Go 14/14 pkgs; both lints 0 issues; both spec-coverage 645 steps covered (all cache-skipped)._
- [x] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh docs` exits 0.
  - _Done 2026-06-06. PASS — 67 cases byte-identical._
- [x] [AI] Spot-check:
      `cargo run --release --quiet --manifest-path apps/rhino-cli-rust/Cargo.toml -- docs validate-heading-hierarchy .claude/ .opencode/`
      reports ZERO findings (allowlist protects agent/skill files) — acceptance: exits 0.
  - _Done 2026-06-06. Exit 0 — "All heading hierarchies valid! No findings found." Allowlist default-deny protects agent/skill files._

> **Pause Safety**: the heading validator now exists in both CLIs, self-scopes to prose, and
> protects agent/skill files, but it is NOT yet wired into any hook/CI (Phase 5). Safe to stop.
> To resume: re-run both `test:quick` targets.

---

## Phase 3: Mermaid — Repo-Wide Scan + `--exclude` (TDD, both CLIs)

> _Suggested executors: `swe-rust-dev` (Rust), `swe-golang-dev` (Go)._

Implement DD-2/DD-3 for the mermaid gate (repeatable `--exclude` and a repo-wide default scan
minus the standardized noise-skip set) plus DD-14 (the two upstream parser fixes from the
2026-06-06 cross-repo alignment: pipe-labeled edges, cyclic-diagram ranking). The mermaid CHECK
SET is unchanged — no upstream extras are ported; the parser fixes correct edge extraction and
ranking bugs, not checks.

- [x] [AI] **SPEC (RED)** — Extend
      `specs/apps/rhino/behavior/cli/gherkin/docs/docs-validate-mermaid.feature` with four
      scenarios (one repo-wide-scan: a violation outside the old four-dir set is reported; one
      `--exclude`: a violation under an excluded tree is not reported; one pipe-labeled-edge:
      `A -->|text| B` parses as an edge with `B` ranked below `A`; one cyclic-diagram:
      `A-->B-->C-->A` ranks as a chain with span 1 and depth 3), each obeying the
      keyword-cardinality norm; extend `component-cli.md`. Run
      `npx nx run rhino-cli-go:spec-coverage`
      — acceptance: spec-coverage FAILS listing the new unmatched steps.
  - _Done 2026-06-06. Status: complete. Files: `docs-validate-mermaid.feature` (+4 scenarios), `component-cli.md` (--exclude row + repo-wide scan note). spec-coverage FAILS with 8 unmatched steps, all in the 4 new scenarios — RED confirmed._
- [x] [AI] **RED (Rust)** — Add failing unit tests in
      `apps/rhino-cli-rust/src/commands/docs.rs` test module (temp-dir fixtures, a–c) and in the
      `apps/rhino-cli-rust/src/internal/mermaid/` parser/graph test modules (d–e) covering:
      (a) the default scan now collects a `*.md` under a tree outside the old four-dir set;
      (b) the walk skips `worktrees/`, `archived/`, and the rest of the standardized
      noise-skip set;
      (c) `--exclude plans/done` drops files under `plans/done` from the collected set;
      (d) a pipe-labeled edge `A -->|text| B` parses as an edge — target node `B` is extracted
      and ranked one level below `A` (DD-14);
      (e) the cyclic diagram `A-->B-->C-->A` ranks as a chain — the back edge is removed via
      iterative DFS in node-declaration order and Kahn ranks the remaining DAG with span 1 and
      depth 3 (DD-14).
      Run `npx nx run rhino-cli-rust:test:quick`
      — acceptance: new tests FAIL; preexisting tests pass.
  - _Done 2026-06-06. Status: complete. Files: `commands/docs.rs` (+3 tests + sanctioned `filter_mermaid_excluded` stub), `internal/mermaid/parser.rs` (+2 tests), `internal/mermaid/graph.rs` (+1 test). 570 passed / 6 failed — failures exactly the new tests pinning DD-2/DD-3/DD-14 desired behavior. GREEN note: preexisting `cycle_nodes_rank_zero` pins OLD fallback and must be updated to chain semantics._
- [x] [AI] **GREEN (Rust)** — Implement in `apps/rhino-cli-rust/src/commands/docs.rs` and
      `apps/rhino-cli-rust/src/internal/mermaid/`:
      (1) change `collect_md_default_dirs` (lines 291-308) to a repo-wide walk; expand
      `walk_md_files` (lines 312-333) to the full standardized noise-skip set (share the
      skip-set constant with the Phase 1 links walker — one definition per CLI);
      (2) add repeatable `--exclude` to `ValidateMermaidArgs` and filter the collected file list
      by prefix (reuse the `filter_skip_paths` semantics);
      (3) in the mermaid parser, strip `|label|` segments following arrows before edge splitting
      so pipe-labeled edges extract their target nodes (DD-14);
      (4) in the mermaid graph ranking, detect back edges via iterative DFS in node-declaration
      order, remove them, then run Kahn longest-path ranking on the remaining DAG (DD-14);
      (5) cucumber step definitions in `apps/rhino-cli-rust/tests/docs.rs`.
      Run `npx nx run rhino-cli-rust:test:quick && npx nx run rhino-cli-rust:test:integration`
      — acceptance: all tests pass.
  - _Done 2026-06-06. Status: complete. Files: `commands/docs.rs` (repo-wide `collect_md_default_dirs` via shared walk; `--exclude` wired through `filter_skip_paths`; USAGE updated cobra-style), `internal/docs/scanner.rs` (NOISE_DIRS + filter_skip_paths pub(crate)), `internal/mermaid/parser.rs` (PIPE_LABEL_RE strip before edge split), `internal/mermaid/graph.rs` (iterative-DFS back-edge removal + Kahn), `tests/docs.rs` (4 scenarios wired; --changed-only fixture made upstream-faithful). Preexisting `cycle_nodes_rank_zero` renamed/updated to chain semantics (mandated DD-14 change). test:quick PASS (576 tests, 90% gate); test:integration PASS (45 scenarios/174 steps); lint clean. Live-repo note: repo-wide scan surfaces 33 violations/7 warnings with standardized flags — cleanup in Phases 6-10._
- [x] [AI] **REFACTOR (Rust)** — Single shared noise-skip constant; no duplicated walkers. Run
      `npx nx run rhino-cli-rust:lint && npx nx run rhino-cli-rust:test:quick`
      — acceptance: both exit 0.
  - _Done 2026-06-06. Status: complete. Duplicate `walk_md_files` deleted — mermaid path now delegates to `scanner::get_all_markdown_files` (byte-identical walk proven). NOISE_DIRS + filter_skip_paths single-definition confirmed. lint exit 0; test:quick 576 passed/0 failed._
- [x] [AI] **RED (Go)** — Same failing tests (a–c) in
      `apps/rhino-cli-go/cmd/docs_validate_mermaid_test.go` and (d–e) in the
      `apps/rhino-cli-go/internal/mermaid/` parser/graph test files (fixtures identical to the
      Rust set). Run
      `npx nx run rhino-cli-go:test:quick`
      — acceptance: new tests FAIL; preexisting tests pass.
  - _Done 2026-06-06. Status: complete. Files: `cmd/docs_validate_mermaid_test.go` (+3 tests + sanctioned panicking `filterMermaidExcluded` stub), `internal/mermaid/parser_test.go` (+2), `internal/mermaid/graph_test.go` (+1, GREEN-conflict comments on two old cycle pins). 6 new tests FAIL for right reasons; all 14 packages otherwise pass; vet/gofmt clean._
- [x] [AI] **GREEN (Go)** — Mirror: `collectMDDefaultDirs`
      (`docs_validate_mermaid.go:205-227`) → repo-wide walk; `skipDirs` (lines 229-234) → full
      standardized noise-skip set (share with the links walker via `internal/fileutil`);
      `--exclude` flag; the same pipe-label stripping and DFS back-edge removal + Kahn ranking
      in `apps/rhino-cli-go/internal/mermaid/` (DD-14, byte-parity with Rust);
      godog steps in `docs_validate_mermaid.integration_test.go`. Run
      `npx nx run rhino-cli-go:test:quick && npx nx run rhino-cli-go:spec-coverage`
      — acceptance: all tests pass; spec-coverage exits 0.
  - _Done 2026-06-06. Status: complete. Files: `internal/docs/links_scanner.go` (walker exported `GetAllMarkdownFiles`/`FilterSkipPaths`), `cmd/docs_validate_mermaid.go` (repo-wide default scan via shared walker; --exclude StringArrayVar + filterMermaidExcluded; cmd-level skipDirs/walkMDFiles deleted), `internal/mermaid/parser.go` (pipeLabelRe), `internal/mermaid/graph.go` (findBackEdges DFS + Kahn), graph_test GREEN-conflict pins updated, integration steps + 8 step constants. test:quick PASS (cov 90.22%); spec-coverage exit 0 (158 scenarios, 660 steps). Byte-parity sanity: json/text/usage byte-identical; multi-file text group order = preexisting documented Go map-order divergence._
- [x] [AI] **REFACTOR (Go)** — Same consolidation pass. Run
      `npx nx run rhino-cli-go:lint && npx nx run rhino-cli-go:test:quick`
      — acceptance: both exit 0.
  - _Done 2026-06-06. Status: complete. Already cohesive (no cmd-level walker; single noiseDirs). Three comment-only edits (exported-symbol docs naming consumers; arrowTokenRe doc style). lint 0 issues; test:quick exit 0 (cov 90.22%)._
- [x] [AI] **PARITY** — Extend the shadow-diff `docs` corpus with
      `validate-mermaid --exclude` + repo-wide variants plus pipe-labeled-edge and
      cyclic-diagram fixtures, then run
      `bash apps/rhino-cli-rust/scripts/shadow-diff.sh docs`
      — acceptance: exits 0 (byte-identical output, including the parser-fix fixtures).
  - _Done 2026-06-06. Status: complete. +20 cases under `docs/.shadow-mermaid-fixtures/` (pipe-label span1/depth2, cycle span1/depth3 via threshold device, default repo-wide, --exclude variants). PASS — 86 cases byte-identical (was ~67 incl. interim). 5 preexisting corpus cases fixed: live tree now has 36 finding files repo-wide, breaking old zero-finding TEXT assumptions vs the documented Go map-order nondeterminism — moved to json/markdown (markdown verified deterministic in both CLIs)._

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [x] [AI] Both `test:quick` targets green; both `lint` targets exit 0; both `spec-coverage`
      targets exit 0.
  - _Done 2026-06-06. Rust 576/0, Go 14/14 pkgs; lints clean; spec-coverage 660 steps covered both CLIs (cache-skipped)._
- [x] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh docs` exits 0.
  - _Done 2026-06-06. PASS — 86 cases byte-identical._

> **Pause Safety**: all three validators now have their final CLI behavior in both CLIs, but
> hooks/CI still run the old wiring (Phase 5). Safe to stop. To resume: re-run both `test:quick`
> targets.

---

## Phase 4: Pre-Commit Staged-Only Steps (Mermaid + Heading) (TDD, both CLIs)

> _Suggested executors: `swe-rust-dev` (Rust), `swe-golang-dev` (Go)._

Implement DD-8: add staged-only mermaid + heading steps to BOTH `git pre-commit` runners,
mirroring the existing link step; extend the link step's skip paths.

- [x] [AI] **SPEC (RED)** — Extend
      `specs/apps/rhino/behavior/cli/gherkin/git/git-pre-commit.feature` with four scenarios
      (staged-mermaid-blocks, staged-prose-heading-blocks, staged-skill-file-exempt,
      link-step-honors-exclusions), each obeying the keyword-cardinality norm. Run
      `npx nx run rhino-cli-go:spec-coverage`
      — acceptance: spec-coverage FAILS listing the new unmatched steps.
  - _Done 2026-06-06. Status: complete. File: `git-pre-commit.feature` (+4 scenarios). spec-coverage FAILS with 8 unmatched steps (Given + final And per scenario) — RED confirmed._
- [x] [AI] **RED (Rust)** — Add failing injected-Deps unit tests in
      `apps/rhino-cli-rust/src/internal/git/runner.rs` covering:
      (a) a staged `*.md` with a malformed flowchart makes the new mermaid step return an error;
      (b) a staged `docs/` file with a duplicate H1 makes the new heading step return an error;
      (c) a staged `SKILL.md` with many H1s makes the heading step return OK (allowlist);
      (d) the link step's skip paths now include `plans/done` (alongside the existing entries).
      Run `npx nx run rhino-cli-rust:test:quick`
      — acceptance: new tests FAIL; preexisting pre-commit tests pass.
  - _Done 2026-06-06. Status: complete. File: `internal/git/runner.rs` (+4 injected-Deps tests + sanctioned step6m/step6h unimplemented stubs). 576 passed / 4 failed — failures exactly the new tests; (d) fails on skip-paths content diff (captured via injected closure), (a–c) via stub panic._
- [x] [AI] **GREEN (Rust)** — Implement in `runner.rs`:
      (1) `step6m_validate_mermaid` (staged `*.md`, minus `plans/done` + noise dirs, block on
      findings) and `step6h_validate_heading_hierarchy` (staged `*.md`, filtered by
      `is_prose_allowlisted`, block on findings), registered in `run()` (lines 118-150) between
      step 5b and step 7;
      (2) extend `step7_validate_links` skip paths (lines 410-413): the final value is
      `vec![".opencode/skill/", ".claude/worktrees/", "plans/done"]` — do NOT drop the existing
      entries;
      (3) cucumber steps for the new scenarios in `apps/rhino-cli-rust/tests/git.rs` (error path,
      mirroring the existing git corpus style).
      Run `npx nx run rhino-cli-rust:test:quick`
      — acceptance: all tests pass.
  - _Done 2026-06-06. Status: complete. Files: `runner.rs` (step6m staged mermaid w/ max_depth 4 + noise/plans-done skip; step6h staged headings via is_prose_allowlisted; both registered between 5b and 7; step7 skip paths = [.opencode/skill/, .claude/worktrees/, plans/done]), `tests/git.rs` (4 scenarios, stub-bin PATH device). test:quick 580/0; integration git suite 5 scenarios/20 steps; lint clean. spec-coverage stays RED (8 gaps) until GREEN (Go) — expected intermediate state._
- [x] [AI] **REFACTOR (Rust)** — Factor the staged-`*.md` collection shared by the three steps;
      align step naming/comments. Run
      `npx nx run rhino-cli-rust:lint && npx nx run rhino-cli-rust:test:quick`
      — acceptance: both exit 0.
  - _Done 2026-06-06. Status: complete. Extracted `staged_markdown_files(root, staged, pred)` helper used by 6m/6h (step 7 delegates to the validator's staged_only — no third duplicate, documented). Comments aligned to `Mirrors Go` phrasing. lint exit 0; test:quick 580/0._
- [x] [AI] **RED (Go)** — Same failing tests (a–d) in
      `apps/rhino-cli-go/internal/git/runner_test.go`. Run `npx nx run rhino-cli-go:test:quick`
      — acceptance: new tests FAIL; preexisting tests pass.
  - _Done 2026-06-06. Status: complete. Files: `runner.go` (sanctioned panic stubs step6m/step6h), `runner_test.go` (+4 tests, injected-Deps style; (d) captures SkipPaths via closure). 4 new tests FAIL (3 panic, 1 content diff); all preexisting pass; vet/gofmt clean._
- [x] [AI] **GREEN (Go)** — Mirror in `apps/rhino-cli-go/internal/git/runner.go`:
      `step6mValidateMermaid` + `step6hValidateHeadingHierarchy` registered in `Run`; extend
      `step7ValidateLinks` `SkipPaths` (line 333) with `"plans/done"`; update the step list in
      the `git pre-commit` long help (`cmd/git_pre_commit.go`); godog steps in
      `cmd/git_pre_commit.integration_test.go`. Run
      `npx nx run rhino-cli-go:test:quick && npx nx run rhino-cli-go:spec-coverage`
      — acceptance: all tests pass; spec-coverage exits 0.
  - _Done 2026-06-06. Status: complete. Files: `runner.go` (step6m/step6h impl + stagedMarkdownFiles + isStagedMermaidSkipped; step7 SkipPaths + plans/done; registration strings byte-parity), `links_scanner.go` (+IsNoiseDir export), `git_pre_commit.go` (long help: +6m/6h, stale step-6 line replaced, step 7 annotated), integration test rewritten (stub-bin PATH, in-process Deps capture), `steps_common_test.go` (+constants). test:quick PASS (git 54/54, cov 90.06%); spec-coverage exit 0 (162 scenarios, 676 steps). Rust help-gap check: no gap (Rust help is single-line, doesn't enumerate steps)._
- [x] [AI] **REFACTOR (Go)** — Same consolidation pass. Run
      `npx nx run rhino-cli-go:lint && npx nx run rhino-cli-go:test:quick`
      — acceptance: both exit 0.
  - _Done 2026-06-06. Status: complete. Already cohesive — GREEN landed pre-factored to the Rust REFACTOR shape (shared stagedMarkdownFiles; step7 StagedOnly delegation documented; comments word-for-word parity). Zero edits. lint 0 issues; test:quick exit 0 (git 92.7%)._
- [x] [AI] **PARITY** — Run the git + docs shadow-diff corpora:
      `bash apps/rhino-cli-rust/scripts/shadow-diff.sh docs git`
      — acceptance: exits 0.
  - _Done 2026-06-06. Status: complete. PASS — 91 cases byte-identical (docs + git corpora)._

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [x] [AI] Both `test:quick` targets green; both `lint` targets exit 0; both `spec-coverage`
      targets exit 0.
  - _Done 2026-06-06. Rust 580/0, Go 14/14 pkgs; lints clean; spec-coverage 676 steps covered both CLIs (cache-skipped)._
- [x] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh docs git` exits 0.
  - _Done 2026-06-06. PASS — 91 cases byte-identical._

> **Pause Safety**: both pre-commit suite binaries now contain all three staged-only steps, but
> the installed hook still runs the previously-built binary and `.husky/`/CI are not rewired
> (Phase 5) — repo enforcement is visibly unchanged until rebuild + Phase 5. Safe to stop. To
> resume: re-run both `test:quick` targets.

---

## Phase 5: Wire Enforcement — Pre-Push, Nx Targets, Consolidated CI

> _Suggested executor: `swe-rust-dev`._
>
> Wires all THREE layers (DD-1/DD-9/DD-10): Layer 1 = pre-commit (the rebuilt suite from
> Phase 4) + remove mermaid from pre-push; Layer 2 = `validate-markdown.yml` on `pull_request`
> to `main`; Layer 3 = the same workflow on `push` to `main`.

- [x] [AI] **Layer 1 (pre-push removal)** — Edit `.husky/pre-push`: remove the mermaid trigger
      block (lines 22-24: the `if echo "$CHANGED" | grep -qE '\.md$'` block running
      `npx nx run rhino-cli-rust:validate:mermaid`). Leave every other block intact. Verify:
      `grep -c "validate:mermaid" .husky/pre-push`
      — acceptance: 0 matches.
  - _Done 2026-06-06. Status: complete. Mermaid trigger block removed; all other blocks intact; grep returns 0 matches._
- [x] [AI] Add `validate:links` and `validate:heading-hierarchy` Nx targets to
      `apps/rhino-cli-rust/project.json` (after the `validate:mermaid` entry at lines 153-165,
      mirroring its shape: bare `command`, `cache: true`, `inputs`, `outputs: []`): - `validate:links` command:
      `cargo run --release -q --manifest-path apps/rhino-cli-rust/Cargo.toml -- docs validate-links --exclude plans/done` - `validate:heading-hierarchy` command:
      `cargo run --release -q --manifest-path apps/rhino-cli-rust/Cargo.toml -- docs validate-heading-hierarchy` - inputs for both: `["{projectRoot}/src/**/*.rs", "{workspaceRoot}/**/*.md"]` - also update `validate:mermaid`: set its command to run
      `docs validate-mermaid --max-depth=4 --exclude plans/done` (the standardized cross-repo
      gate invocation — `--max-depth=4` demotes wide+deep diagrams from error to warning
      identically across all three aligned repos) and replace
      its enumerated dir inputs with `{workspaceRoot}/**/*.md`.
      Verify: `npx nx run rhino-cli-rust:validate:links` and
      `npx nx run rhino-cli-rust:validate:heading-hierarchy` execute (pass/fail acceptable
      here) — acceptance: both targets resolve and run; findings reported.
  - _Done 2026-06-06. Status: complete. `validate:links` + `validate:heading-hierarchy` added after `validate:mermaid` (bare command, cache true, src+workspace-md inputs, outputs []); `validate:mermaid` command now `--max-depth=4 --exclude plans/done` with `{workspaceRoot}/**/*.md` inputs. Both targets resolve and run: links reports 404 broken (backlog), heading-hierarchy 37 findings (backlog)._
- [x] [AI] Mirror the same three target changes in `apps/rhino-cli-go/project.json` (after
      `validate:mermaid` at lines 116-128), using the Go command form
      (`CGO_ENABLED=0 go run -C apps/rhino-cli-go main.go docs validate-links --exclude plans/done`,
      etc.; the Go `validate:mermaid` command becomes
      `CGO_ENABLED=0 go run -C apps/rhino-cli-go main.go docs validate-mermaid --max-depth=4 --exclude plans/done`;
      inputs `["{projectRoot}/**/*.go", "{workspaceRoot}/**/*.md"]`). Verify:
      `npx nx run rhino-cli-go:validate:links` executes
      — acceptance: target resolves and runs.
  - _Suggested executor: `swe-golang-dev`._
  - _Done 2026-06-06. Status: complete. Three Go targets mirrored (validate:mermaid → --max-depth=4 --exclude plans/done + workspace-md inputs; validate:links, validate:heading-hierarchy new). `rhino-cli-go:validate:links` runs and reports 404 broken — byte-matching the Rust count._
- [x] [AI] **Layers 2 & 3 (consolidated CI)** — Create
      `.github/workflows/validate-markdown.yml` (_New file_), mirroring the structure of the
      existing `pr-validate-links.yml` (read it first: `actions/checkout@v6` →
      `./.github/actions/setup-node` → `./.github/actions/setup-rust`,
      `permissions: contents: read`, `ubuntu-latest`). Two differences:
  - The `on:` block has BOTH triggers:

    ```yaml
    on:
      pull_request:
        branches: [main]
      push:
        branches: [main]
    ```

  - The job has three named run steps:
    `npx nx run rhino-cli-rust:validate:mermaid`,
    `npx nx run rhino-cli-rust:validate:links`, and
    `npx nx run rhino-cli-rust:validate:heading-hierarchy`.
    Verify: `npx prettier --check .github/workflows/validate-markdown.yml` exits 0; run
    `actionlint` if available (skip gracefully if not)
    — acceptance: the file exists; prettier passes; the `on:` block has BOTH triggers; all
    three validators are invoked.
  - _Done 2026-06-06. Status: complete. `.github/workflows/validate-markdown.yml` created (checkout@v6 → setup-node → setup-rust, contents: read, ubuntu-latest; `on:` has BOTH pull_request+push to main; three named steps run mermaid/links/heading-hierarchy Nx targets). Prettier passes; actionlint not installed — skipped gracefully._

- [x] [AI] **Migrate the legacy link workflow** — Delete
      `.github/workflows/pr-validate-links.yml` (its link check now runs inside
      `validate-markdown.yml`). Verify: `test ! -f .github/workflows/pr-validate-links.yml`
      — acceptance: the file no longer exists.
  - _Done 2026-06-06. Status: complete. `git rm` applied; file no longer exists._
- [x] [AI] Rebuild the release binary so the installed pre-commit hook picks up the Phase 4
      steps: `cargo build --release --quiet --manifest-path apps/rhino-cli-rust/Cargo.toml`
      — acceptance: exits 0; a scratch staged malformed-flowchart commit is blocked by the local
      pre-commit hook (then unstage the scratch change).
  - _Done 2026-06-06. Status: complete. Rebuild exit 0 (already current). Scratch `docs/scratch-mermaid-test.md` with >30-char node label staged + commit attempted → BLOCKED: "Error: found 1 mermaid violation(s); husky - pre-commit script failed (code 1)"; HEAD unchanged. Scratch unstaged and deleted._
- [x] [AI+HUMAN] **Behavioral acceptance (observed at execution)** — Confirm a
      deliberately-broken markdown change makes the `validate-markdown` CI check FAIL. This
      requires a real GitHub Actions event: on a throwaway branch, introduce one broken relative
      link (or a broken `#anchor`, or a duplicate H1 in a `docs/` file), open a PR to `main`, and
      observe `validate-markdown` go RED; then revert/close and confirm it goes GREEN on the
      clean state. Agent prepares the scratch change + PR and reports the run URLs; the human
      confirms the observed RED→GREEN arc and authorizes any throwaway push. Resume signal: the
      human replies confirming both run conclusions — acceptance: the `validate-markdown` check
      reports failure on the broken markdown and success once reverted. (If the operator prefers
      not to open a scratch PR, this may be deferred to the Phase 12 push-to-`main` run, where
      Layer 3 fires for real — record the decision in phase notes.)
  - _Done 2026-06-06. Status: complete via the sanctioned deferral clause. Decision recorded: no scratch PR opened (operator directive is continuous unattended execution); behavioral acceptance defers to the Phase 12 push-to-`main` run where the `validate-markdown` workflow fires for real (Layer 3 first trigger). The RED half of the arc was demonstrated locally: the rebuilt pre-commit hook blocked a staged malformed flowchart (item above), and all three validators currently report the live backlog (404 links / 37 headings / 33 mermaid violations) — proving the gates detect violations. The GREEN half lands when Phase 12 CI passes on the cleaned repo._

### Phase 5 Gate

> All checks below must pass before starting Phase 6. The validators are EXPECTED to report
> findings here (the fix-all has not run) — that is acceptable; what must hold is that the wiring
> is correct across all three layers.

- [x] [AI] `grep -c "validate:mermaid" .husky/pre-push` returns 0 (Layer 1 removal).
  - _Done 2026-06-06. 0 matches._
- [x] [AI] `npx nx run rhino-cli-rust:validate:links` and
      `npx nx run rhino-cli-rust:validate:heading-hierarchy` execute against full scope
      (pass/fail acceptable).
  - _Done 2026-06-06. Both execute full-scope (exit 1 with backlog findings — acceptable here)._
- [x] [AI] `.github/workflows/validate-markdown.yml` exists with BOTH triggers and all three
      validators; `pr-validate-links.yml` is deleted;
      `npx prettier --check .github/workflows/validate-markdown.yml` exits 0.
  - _Done 2026-06-06. Both triggers present (pull_request + push to main); three validator steps; legacy file deleted; prettier passes._

> **Pause Safety**: wiring is in place but the repo has known markdown findings — do NOT push
> from here, because pre-commit/CI would now block on the unfixed backlog. This is a coherent
> **local** stopping point (no half-edited files). To resume: re-run the three Rust validate
> targets and proceed to per-tree cleanup.

---

## Per-Tree Fix-All Phases (gated)

> For EACH tree below: re-measure with all THREE expanded validators (within scope), then for
> every blocking finding apply ONE of — (mermaid) shorten labels / restructure the diagram;
> (link) fix the path or correct the target; (anchor) fix the `#fragment` to match a real
> slugified heading or update the destination heading; (heading) restructure to a single H1 with
> non-skipping nesting. Re-measure each tree at execution — do NOT rely on authoring-time counts.
> Heading findings apply ONLY to prose-allowlist trees.
>
> Measurement commands (the link gate has no positional path — measure per tree by filtering the
> full-scan JSON output by `source_file` prefix):
>
> ```bash
> RHINO="cargo run --release --quiet --manifest-path apps/rhino-cli-rust/Cargo.toml --"
> $RHINO docs validate-mermaid --max-depth=4 --exclude plans/done -o json <tree>/
> $RHINO docs validate-links --exclude plans/done -o json   # filter findings by source_file prefix <tree>/
> $RHINO docs validate-heading-hierarchy -o json <tree>/
> ```
>
> _Suggested executor per tree: `repo-rules-maker` for `repo-governance/`; `docs-maker` for
> `docs/`; `specs-maker` for `specs/`; `swe-rust-dev` for `apps/`/`libs/` (code-adjacent);
> otherwise a generic edit._

### Phase 6: Fix-all `repo-governance/`

- [x] [AI] Re-measure all three gates for `repo-governance/` using the measurement commands above
      — acceptance: per-finding lists (mermaid / broken-link / broken-anchor / heading) recorded
      in phase notes.
  - _Done 2026-06-06. Status: complete. Initial measure showed 35 heading findings — root-caused to a fence-tracking bug (naive toggle desyncs on nested ``/`fences) and FIXED in both CLIs (`fix(rhino-cli): track code fences with CommonMark open/close semantics`, shadow-diff 86/86 green) rather than mangling docs. Post-fix authoritative measure: mermaid = 1 violation (`conventions/formatting/diagrams.md`block 61 line 1806, width_exceeded 6>4); links = 9 broken-anchor findings (`conventions/README.md` L71/73/74 — emoji-heading slugs; `repository-governance-architecture.md` L25/27/35/36/39; `development/workflow/worktree-setup.md` L164 — renamed anchor in trunk-based-development.md); headings = 0._
- [x] [AI] For each finding: apply the resolution per the preamble; after each fix, re-run the
      applicable validator for that file. Acceptance: re-running all three measurement commands
      shows zero findings for `repo-governance/`.
  - _Suggested executor: `repo-rules-maker`._
  - _Done 2026-06-06. Status: complete (via repo-rules-maker). Files: `conventions/formatting/diagrams.md` (block 61 restructured 6-wide → max 3 per rank, pedagogy intact), `conventions/README.md` (3 emoji-slug anchors → `#-formatting` etc.), `repository-governance-architecture.md` (5 TOC anchors corrected incl. `#agent-skills-…`), `development/workflow/worktree-setup.md` (renamed anchor → `#worktree-mode-direct-push-to-main-draft-pr-opt-in`). Verified: mermaid 0/0 (17 files, 103 blocks), links 0 repo-governance findings, headings 0._

### Phase 6 Gate

- [x] [AI] All three measurement commands report zero findings for `repo-governance/`
      (mermaid exits 0 for the tree; the link full-scan lists no finding with a
      `repo-governance/` source file; heading exits 0 for the tree).
  - _Done 2026-06-06. Gate green: mermaid exit 0 (0 violations/0 warnings), links full-scan 0 repo-governance findings, heading exit 0._

> **Pause Safety**: `repo-governance/` is clean under the new rules; other trees may still have
> findings (don't push yet). Safe to stop. To resume: re-run the three measurement commands.

### Phase 7: Fix-all `docs/`

- [x] [AI] Re-measure all three gates for `docs/` — acceptance: per-finding lists recorded.
  - _Done 2026-06-06 (corrected). Status: complete. mermaid = 24 width_exceeded violations (widths 5-11; full list in run log); headings = 0; links = 346 broken-anchor findings across 66 docs files (CORRECTION: an initial JSON-parse error read the wrong schema key and reported 0 — authoritative count from `broken_count`/`categories`: heaviest files `elixir-phoenix/best-practices.md` 25, `jvm-spring-boot/best-practices.md` 20, `elixir-phoenix/anti-patterns.md` 18; mostly stale TOC anchors referencing renamed/nonexistent sections)._
- [x] [AI] For each finding: apply the resolution per the preamble; re-run per file. Acceptance:
      re-running all three measurement commands shows zero findings for `docs/`.
  - _Suggested executor: `docs-maker`._
  - _Done 2026-06-07. Status: complete. Mermaid: 24 diagrams restructured across 24 files (docs-maker; LR→TD, fan-out staging, node merges, 1 label shortened). Links: 346 broken anchors fixed across 66 files via 3 parallel docs-maker passes (stale TOC entries repointed to real GFM slugs; emoji headings need leading-hyphen slugs; closest-covering-section used where target sections no longer exist) + 12 orchestrator-direct fixes (10 `#-problem-*` emoji slugs in jvm-spring/anti-patterns.md, 1 in jvm-spring-boot/configuration.md, and 1 REAL doc bug: unbalanced ````typescript fence in typescript/test-driven-development.md swallowing the rest of the file — opener normalized + missing closer added). Verified: mermaid 0 violations / 3 advisory warnings, headings 0, links 0 for docs/._

### Phase 7 Gate

- [x] [AI] All three measurement commands report zero findings for `docs/`.
  - _Done 2026-06-07. Gate green: mermaid exit 0 (0 violations), links full-scan 0 docs/ findings, headings exit 0._

> **Pause Safety**: `docs/` clean; remaining trees pending. Safe to stop. To resume: re-run the
> three measurement commands.

### Phase 8: Fix-all `plans/` (excludes `plans/done/`; includes this plan — dogfooding)

- [x] [AI] Re-measure all three gates for `plans/` (pass `--exclude plans/done` to mermaid and
      links — mermaid also pins `--max-depth=4`; heading-hierarchy already excludes
      `plans/done/` via the allowlist)
      — acceptance: per-finding lists recorded.
  - _Done 2026-06-07. Status: complete. mermaid = 3 violations (add-investment-oracle-app/README.md L268 span 5; plan-domain-parity/tech-docs.md L9 span 5; planning-system-overhaul/tech-docs.md L8 label 31 chars); links = 2 broken anchors (plans/README.md L40, plans/in-progress/README.md L33 — both `#-plan-folder-naming` vs real slug `plan-folder-naming`); headings = 0._
- [x] [AI] For each finding: apply the resolution per the preamble; re-run per file. Acceptance:
      zero findings for `plans/` (excluding `plans/done/`), including this plan's own five docs
      (dogfooding).
  - _Done 2026-06-07. Status: complete. Two over-wide LR flowcharts converted to TD (investment-oracle README also staged its provider fan-out behind a `Model providers` node; plan-domain-parity canon-flow chain), one 31-char label shortened, two `#-plan-folder-naming` anchors corrected to `#plan-folder-naming`. Verified: mermaid 0/0 (6 files, 9 blocks), links 0 plans findings, headings 0 — including this plan's five docs (dogfooding)._

### Phase 8 Gate

- [x] [AI] All three measurement commands report zero findings for `plans/` outside
      `plans/done/`, including `plans/in-progress/markdown-gate-coverage-expansion/`.
  - _Done 2026-06-07. Gate green: mermaid exit 0, links 0 plans findings, headings exit 0._

> **Pause Safety**: `plans/` clean; `specs/`, `apps/`, `libs/`, root pending. Safe to stop. To
> resume: re-run the three measurement commands.

### Phase 9: Fix-all `specs/`, `apps/`, and `libs/`

> Heading-hierarchy applies to `specs/**`, `apps/*/README.md`, `libs/*/README.md`, and
> `apps|libs/*/docs/**` ONLY; deeper app/lib paths get mermaid + links only.

- [x] [AI] Re-measure mermaid + links for `specs/`, `apps/`, `libs/` and heading-hierarchy for
      `specs/` + the README/docs allowlist subset — acceptance: per-finding lists recorded.
  - _Suggested executors: `specs-maker` (`specs/`), `swe-rust-dev` (`apps/`/`libs/`)._
  - _Done 2026-06-07. Status: complete. specs: 6 mermaid violations (container.md 3 labels + span 10; context.md span 6; component-cli.md VH label 34) + 7 broken links (2 behavior READMEs short one `../`; 4 stale relative paths in context.md; rhino README → missing rhino-cli-rust README); headings 0. apps: 14 links, libs: 21 links — ALL under gitignored vendored/build trees (`*/deps/`, `crud-be-kotlin-ktor/build/`); 0 mermaid. Also surfaced: 1 broken anchor in `.claude/agents/plan-maker.md` (+generated mirror)._
- [x] [AI] For each finding: apply the resolution per the preamble. Gitignored vendored trees
      (e.g. Elixir `deps/`) are NOT in the standardized cross-repo noise-skip set and never
      reach CI checkouts; if a local re-measure surfaces findings under such a tree, exclude it
      at the call site via `--exclude` — never edit vendored files. Acceptance: re-running the
      measurement commands shows zero findings for these trees.
  - _Done 2026-06-07. Status: complete. specs (via specs-maker): 3 path labels shortened in container.md + TD→LR reorientation; context.md TD→LR; component-cli.md VH label shortened; 7 links repointed to real paths (behavior READMEs gained the missing `../`; context.md → ../containers/ + ../components/; rhino README → rhino-cli-go README). plan-maker.md anchor fixed + `npm run generate:bindings` synced the mirror. Vendored trees (all verified gitignored via git check-ignore): 5 `--exclude` prefixes added at the call site to BOTH CLIs' `validate:links` Nx targets per the sanctioned rule. Verified: specs mermaid 0/4-warnings, links full-scan 0 findings repo-wide (`✓ All links valid!`), headings 0._

### Phase 9 Gate

- [x] [AI] All applicable measurement commands report zero findings for `specs/`, `apps/`, and
      `libs/`.
  - _Done 2026-06-07. Gate green: specs/apps/libs mermaid 0 violations; links full-scan 0 findings (vendored trees call-site-excluded); headings 0._

> **Pause Safety**: only root files pending. Safe to stop. To resume: re-run the measurement
> commands.

### Phase 10: Fix-all root instruction files (`AGENTS.md`, `CLAUDE.md`, `README.md`, other root `*.md`)

> Root `*.md` ARE in the prose allowlist, so all three gates apply.

- [x] [AI] Re-measure all three gates for the root `*.md` files — acceptance: per-finding lists
      recorded.
  - _Done 2026-06-07. Status: complete. Root files (AGENTS, CLAUDE, CONTRIBUTING, LICENSING-NOTICE, README, SECURITY): mermaid 0 (no blocks), headings 0, links 0 (root files carried no findings in the full scan). Nothing to fix._
- [x] [AI] For each finding: apply the resolution per the preamble; re-run per file. Acceptance:
      zero findings for the root files.
  - _Done 2026-06-07. Status: complete. Zero findings — no fixes needed._

### Phase 10 Gate

- [x] [AI] All three measurement commands report zero findings for the root `*.md` files, and the
      three full-scan Nx targets (`npx nx run rhino-cli-rust:validate:mermaid`,
      `:validate:links`, `:validate:heading-hierarchy`) now exit 0 repo-wide.
  - _Done 2026-06-07. Gate green: all three full-scan Nx targets exit 0 repo-wide (links cache-fresh; heading cache-skipped; mermaid green)._

> **Pause Safety**: all trees individually clean; the full-repo gates pass. Safe to stop. To
> resume: re-run the three Nx validate targets.

---

## Phase 11: Update Governance Docs + Propagate via `repo-rules-maker`

> _Executor: `repo-rules-maker` (governance propagation sweep)._
>
> Update all related governance `.md` files, then propagate the change **through
> `repo-rules-maker`** so the sweep reaches every governance surface (conventions,
> check-inventory, indexes, and any agent/skill text) — not only the obvious files.

- [x] [AI] Edit `repo-governance/conventions/formatting/diagrams.md`: state that the mermaid gate
      runs **repo-wide minus `plans/done/` + noise dirs**, at **pre-commit staged-only** + the
      consolidated CI workflow (NOT pre-push) — acceptance: the doc matches the Phase 5 wiring;
      no stale pre-push claim remains.
  - _Done 2026-06-07 (via repo-rules-maker). New "Gate scope and enforcement layers" subsection; only pre-push mention is the corrective "does **not** run at pre-push" statement._
- [x] [AI] Edit `repo-governance/conventions/writing/quality.md`: note that single-H1 and
      non-skipping heading nesting are now **machine-enforced for prose** via
      `docs validate-heading-hierarchy` (both CLIs), scoped to the prose allowlist (`docs/`,
      `repo-governance/`, `plans/`−`done/`, root `*.md`, `specs/`, app/lib READMEs + `docs/`
      subtrees), and explicitly exempt for `.claude/**`/`.opencode/**`/`.amazonq/**` prompt/skill
      artifacts — acceptance: the scope + exemption are stated.
  - _Done 2026-06-07 (via repo-rules-maker). "Machine Enforcement" subsection added with allowlist scope + .claude/.opencode/.amazonq exemption (greps verified)._
- [x] [AI] Edit `repo-governance/conventions/formatting/linking.md`: note that `#fragment`
      anchors are now validated against the target file's headings (`broken-anchor` finding,
      GFM-correct slug algorithm) — acceptance: anchor enforcement is documented.
  - _Done 2026-06-07 (via repo-rules-maker). "Fragment Validation (Automated)" subsection + Link Validation extended (broken-anchor ×3 grep hits)._
- [x] [AI] Edit `repo-governance/development/quality/repository-validation.md`: list the three
      markdown gates, the three enforcement layers, and the consolidated `validate-markdown.yml`
      workflow — acceptance: gates + workflow are listed.
  - _Done 2026-06-07 (via repo-rules-maker). New "Markdown Quality Gates" section with three-layer table; gate greps validate-heading-hierarchy/pre-commit/anchor all ≥1._
- [x] [AI] **Propagate via `repo-rules-maker`** — run the governance propagation sweep so the new
      enforcement is reflected across every related surface (conventions, check-inventory,
      governance indexes, and any agent/skill prompt text that references the old enforcement) —
      acceptance: the sweep is complete; no related surface still describes the stale
      pre-push-only / no-anchor / no-heading-enforcement state.
  - _Done 2026-06-07 (via repo-rules-maker). 10 surfaces updated: docs/reference/system-architecture/ci-cd.md (pr-validate-links → validate-markdown + pre-commit gates), github-actions-workflow-naming.md (workflow table), development/quality/code.md (+6m/6h, 11 steps), markdown.md (pre-commit section), post-push-ci-verification.md (+validate-markdown row), rhino-cli-go README + component-cli.md (--changed-only no longer pre-push), 3 specs READMEs (scenario counts + new feature row). No .claude/ files changed._
- [x] [AI] If any `.claude/` agent/skill text changed during propagation, run
      `npm run generate:bindings` — acceptance: `git status` shows the generated `.opencode/`
      mirrors updated in lockstep (or no `.claude/` change occurred and this is a no-op).
  - _Done 2026-06-07. No-op for the sweep (no .claude/ change). The earlier Phase-9 plan-maker.md anchor fix already ran generate:bindings and synced its mirror._
- [x] [AI] Verify the edited governance docs pass all three gates (run the three measurement
      commands over `repo-governance/`) — acceptance: all exit 0.
  - _Done 2026-06-07. mermaid 0/0 (17 files, 103 blocks); headings 0; links full-scan "All links valid"._

### Phase 11 Gate

- [x] [AI] `npm run lint:md` exits 0.
  - _Done 2026-06-07. Exit 0._
- [x] [AI] All documented facts are present in
      `repo-governance/development/quality/repository-validation.md` — run
      `grep -c "validate-heading-hierarchy" repo-governance/development/quality/repository-validation.md`
      returns ≥ 1, AND
      `grep -c "pre-commit" repo-governance/development/quality/repository-validation.md`
      returns ≥ 1, AND
      `grep -c "anchor" repo-governance/development/quality/repository-validation.md`
      returns ≥ 1 — acceptance: all three grep commands exit 0 with count ≥ 1.
  - _Done 2026-06-07. Counts: validate-heading-hierarchy=1, pre-commit=2, anchor=3 — all ≥1._

> **Pause Safety**: governance docs now match the tooling. Safe to stop. To resume: re-run the
> three Nx validate targets full-scan.

---

## Phase 12: Full-Repo Verification, Quality Gates, Push, CI, Archival

### Repo-Rules Quality Gate (strict, double-zero)

- [ ] [AI] Run the strict
      [repo-rules-quality-gate](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md)
      over the changed governance surface (`diagrams.md`, `quality.md`, `linking.md`,
      `repository-validation.md`, and any `.claude/` bindings), requiring a **double-zero** pass
      — acceptance: the checker reports zero findings AND a follow-up fixer pass produces zero
      changes on a clean re-run.
  - _Suggested executor: `repo-rules-checker` then `repo-rules-fixer` (double-zero)._

### Local Quality Gates (Before Push)

- [ ] [AI] Run all three markdown gates full-scan:
      `npx nx run rhino-cli-rust:validate:mermaid && npx nx run rhino-cli-rust:validate:links && npx nx run rhino-cli-rust:validate:heading-hierarchy`
      — acceptance: all three exit 0 (zero findings within scope).
- [ ] [AI] Run the full parity harness over the changed corpora:
      `bash apps/rhino-cli-rust/scripts/shadow-diff.sh docs git`
      — acceptance: exits 0.
- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck` — acceptance: exits 0.
- [ ] [AI] Run affected linting: `npx nx affected -t lint` — acceptance: exits 0.
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick` — acceptance: exits 0.
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage` — acceptance: exits 0.
- [ ] [AI] Run both CLIs' integration suites:
      `npx nx run rhino-cli-rust:test:integration && npx nx run rhino-cli-go:test:integration`
      — acceptance: both exit 0.
- [ ] [AI] Run markdown lint: `npm run lint:md` — acceptance: exits 0.
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by these changes — and
      re-run the failing checks to confirm resolution. Verify zero failures before pushing.

### Commit Guidelines

- [ ] [AI] Commit changes thematically (Conventional Commits `<type>(<scope>): <description>`),
      each dual-CLI change in ONE commit covering BOTH implementations, for example:
  - `feat(rhino-cli): add --exclude flag and repo-wide scan to validate-links (rust+go)`
  - `feat(rhino-cli): validate markdown anchors against target headings (rust+go)`
  - `feat(rhino-cli): add validate-heading-hierarchy with prose allowlist (rust+go)`
  - `feat(rhino-cli): widen validate-mermaid repo-wide with --exclude (rust+go)`
  - `fix(rhino-cli): parse pipe-labeled edges and rank cyclic diagrams (rust+go)`
  - `feat(rhino-cli): add staged-only mermaid and heading pre-commit steps (rust+go)`
  - `chore(husky): remove mermaid trigger from pre-push`
  - `feat(rhino-cli): add validate:links and validate:heading-hierarchy Nx targets`
  - `ci: consolidate markdown gates into validate-markdown workflow`
  - `ci: remove migrated pr-validate-links workflow`
  - `fix(<scope>): clean markdown gate violations in <tree>` (one per tree as appropriate)
  - `docs(specs): add markdown-gate scenarios and component-cli inventory`
  - `docs(governance): document pre-commit mermaid, prose heading rules, and anchor validation`
  - Preexisting fixes get their own separate commits.
    — acceptance: no unrelated changes bundled into a single commit; no commit touches only one
    CLI for a behavior change.

### Push and Post-Push CI Verification

- [ ] [AI] Push directly to `main`: `git push origin main`
      — acceptance: push succeeds (pre-commit hook green for the staged set; pre-push green).
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every 3 minutes; one
      `gh run list`/`gh run view --json status,conclusion` per wakeup; do NOT use `gh run watch`)
      — acceptance: every workflow run observed to completion, INCLUDING the new
      `validate-markdown` workflow — this push is the first-ever push-to-`main` trigger in this
      repo (Layer 3 fires for real).
- [ ] [AI] Verify the `validate-markdown` run passes and ALL other CI checks pass
      — acceptance: zero failures; the `validate-markdown` run is green.
- [ ] [AI] If any CI check fails, investigate root cause, fix, and push a follow-up commit;
      repeat until ALL GitHub Actions are green — acceptance: full CI green.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI).
- [ ] [AI] Verify all three markdown gates report zero findings within scope.
- [ ] [AI] Move:
      `git mv plans/in-progress/markdown-gate-coverage-expansion plans/done/YYYY-MM-DD__markdown-gate-coverage-expansion`
      (use the actual completion date, NOT the creation date).
- [ ] [AI] Update `plans/in-progress/README.md` — remove the `markdown-gate-coverage-expansion`
      entry.
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date.
- [ ] [AI] Update any other READMEs that reference this plan (e.g. `plans/README.md`).
- [ ] [AI] Commit the archival: `chore(plans): move markdown-gate-coverage-expansion to done`,
      then push to `origin main`.

### Phase 12 Gate

> All checks below must pass — this is the final gate.

- [ ] [AI] `npx nx run rhino-cli-rust:validate:mermaid`, `:validate:links`, and
      `:validate:heading-hierarchy` all exit 0 (full repo clean within scope).
- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` exits 0 and
      `npm run lint:md` passes.
- [ ] [AI] `bash apps/rhino-cli-rust/scripts/shadow-diff.sh docs git` exits 0 (byte parity).
- [ ] [AI] The `repo-rules-quality-gate` double-zero pass is clean.
- [ ] [AI] All GitHub Actions for the push are green, including the new `validate-markdown`
      workflow run (push-to-main trigger).
- [ ] [AI] Plan archived to `plans/done/` and READMEs updated.

> **Pause Safety**: work is complete, pushed, CI green, plan archived. This is the terminal
> state. To re-verify at any later time: run the three markdown validators full-scan and the
> shadow-diff docs corpus.
