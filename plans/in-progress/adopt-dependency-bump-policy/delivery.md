# Delivery Checklist — Adopt Dependency Bump Policy & Planning Workflow

**Execution markers**: `[AI]` = an agent performs the step (default). `[HUMAN]` = only a human can
perform it. No `[HUMAN]` steps exist in this plan — it is fully agent-executable.

## Worktree

Worktree path: `worktrees/adopt-dependency-bump-policy/`

Provision before execution (run from repo root):

```bash
claude --worktree adopt-dependency-bump-policy
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

> **Trunk-based note**: Per [AGENTS.md](../../../AGENTS.md), worktree work and direct-on-`main`
> work both push to `origin main`. This is a docs + minor-validator change; direct-to-`main`
> execution is sanctioned.

---

## Phase 0: Environment Setup and Baseline

- [x] [AI] Provision/confirm worktree and toolchain: from repo root run `npm install` then
      `npm run doctor -- --fix`. Acceptance: both commands exit 0 (doctor may report drift it then
      fixes).
  - _Suggested executor: `repo-setup-manager`_
  - **Implementation Notes**: Ran `npm install` (OK) and `npm run doctor -- --fix`
    (18/19 tools OK; "Nothing to fix"). Rust v1.94.0 ✓, Go v1.26.1 ✓ — both in-scope toolchains
    present. One preexisting warning: python v3.13.1 (< 3.13.12) — outside this plan's affected
    set (no Python touched), doctor reports nothing auto-fixable.
  - **Date**: 2026-06-04
  - **Status**: Completed
  - **Files Changed**: none (toolchain check only)
- [x] [AI] Establish baseline for the only code-bearing projects in scope: run
      `npx nx run rhino-cli-rust:test` and `npx nx run rhino-cli-go:test`. Acceptance: record
      pass/fail; any preexisting failure is noted for Iron-Rule-3 fixing.
  - _Suggested executor: `repo-setup-manager`_
  - **Implementation Notes**: Actual Nx target is `test:unit` (there is no `:test` target — the
    plan reference was an authoring approximation; real targets confirmed via
    `jq '.targets|keys[]' apps/rhino-cli-*/project.json`). Ran `npx nx run rhino-cli-rust:test:unit`
    and `npx nx run rhino-cli-go:test:unit` — both "Successfully ran target test:unit". Baseline
    green. Subsequent gate items use `test:unit`. Note: the workflow-naming validator is the
    `validate:naming-workflows` target on both projects.
  - **Date**: 2026-06-04
  - **Status**: Completed
  - **Files Changed**: none (baseline only)

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [x] [AI] `npm run doctor -- --fix` — exits 0. _Done 2026-06-04: 18/19 OK, exit 0._
- [x] [AI] `git status --porcelain` — shows only the plan folder (clean working tree otherwise).
      _Done 2026-06-04: only `plans/in-progress/adopt-dependency-bump-policy/delivery.md` modified._

> **Pause Safety**: Toolchain converged and baseline recorded; no governance files changed yet.
> Safe to stop indefinitely. To resume: `npm run doctor -- --fix`.

---

## Phase 1: Dependency Bump Policy Document

- [ ] [AI] Create `repo-governance/development/workflow/dependency-bump-policy.md` _New file_,
      adopting the upstream
      [Dependency Bump Stability & Safety Policy](https://github.com/wahidyankf/ose-public/blob/main/repo-governance/development/workflow/dependency-bump-policy.md)
      verbatim in substance (three-path tree A/B/C, KEV Fast-Track, EPSS Escalation, Rule 5a
      recency, Rule 5b functional stability, exact-pin hard-rule table, CVE clearance five-source
      process, cutoff computation, clearance statuses CLEAR / CLEAR (patch-of) / WAIVER /
      FUNCTIONAL-HOLD). Adapt the "What This Policy Covers" examples to `ose-primer` manifests per
      `tech-docs.md` (Cargo: `crud-be-rust-axum`, `rhino-cli-rust`; .NET: `crud-be-csharp-aspnetcore`,
      `crud-be-fsharp-giraffe` with `global.json`; Go: `crud-be-golang-gin`, `rhino-cli-go`,
      `golang-commons`). Keep all relative links pointing at the verified existing targets listed in
      `tech-docs.md`. Acceptance: file exists; `grep -c "Three-Path Decision Tree" <file>` ≥ 1;
      every relative link resolves (`Bash test -f` on each).
  - _Suggested executor: `repo-rules-maker`_
  - **Implementation Notes**: Created via `repo-rules-maker` adopting the upstream policy verbatim
    in substance (three-path tree, KEV Fast-Track, EPSS Escalation, Rule 5a/5b, exact-pin table,
    five-source CVE clearance, clearance statuses). All 10 relative links verified with `test -f`.
    `grep -c "Three-Path Decision Tree"` = 1. Prettier clean; markdownlint 0 errors.
  - **Date**: 2026-06-04
  - **Status**: Completed
  - **Files Changed**: `repo-governance/development/workflow/dependency-bump-policy.md` (new)
- [ ] [AI] Add an index entry for the new policy to
      `repo-governance/development/workflow/README.md` under "Documents", linking
      `./dependency-bump-policy.md` with a one-line description. Acceptance:
      `grep -c "dependency-bump-policy.md" repo-governance/development/workflow/README.md` ≥ 1.
  - **Implementation Notes**: Added a Documents entry after the Reproducible Environments line.
    `grep -c` = 1; prettier clean. **Date**: 2026-06-04. **Status**: Completed.
    **Files Changed**: `repo-governance/development/workflow/README.md`.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [x] [AI] `test -f repo-governance/development/workflow/dependency-bump-policy.md` — exits 0. _Done 2026-06-04._
- [x] [AI] `npx prettier --check repo-governance/development/workflow/dependency-bump-policy.md repo-governance/development/workflow/README.md` — passes (or `--write` then re-check). _Done: "All matched files use Prettier code style"._
- [x] [AI] `npx markdownlint-cli2 repo-governance/development/workflow/dependency-bump-policy.md` — no errors. _Done: 0 error(s)._

> **Pause Safety**: Policy document exists and is indexed; no workflow file yet (naming gate not
> yet exercised). Safe to stop indefinitely. To resume: re-run the Phase 1 gate checks.

---

## Phase 2: `planning` Workflow Type Support

- [x] [AI] Edit `repo-governance/conventions/structure/workflow-naming.md`: add a `planning` row to
      the **Type Vocabulary** table (semantics: "single forward planning procedure whose terminal
      deliverable is a plan document; not an iterative maker/checker/fixer loop"; example:
      `repo-dependency-bump-planning`), and update the enforcement regex from
      `'-(quality-gate|execution|setup)$'` to `'-(quality-gate|execution|setup|planning)$'` in BOTH
      the Enforcement section command and any duplicate. Acceptance:
      `grep -c "planning" repo-governance/conventions/structure/workflow-naming.md` ≥ 2 and the
      regex includes `planning`.
  - _Suggested executor: `repo-rules-maker`_
  - **Implementation Notes**: Added `planning` row to the Type Vocabulary table, updated the
    enforcement regex at the Enforcement section AND the "Enforceable by checker" design-rationale
    bullet (line ~20) to `-(quality-gate|execution|setup|planning)$`. `grep -c planning` = 3;
    two regex occurrences now include `planning`. Prettier clean; markdownlint 0 errors.
  - **Date**: 2026-06-04
  - **Status**: Completed
  - **Files Changed**: `repo-governance/conventions/structure/workflow-naming.md`
- [x] [AI] Edit `apps/rhino-cli-rust/src/commands/workflows.rs`: add `"planning"` to
      `const WORKFLOW_TYPES`. Update/extend the adjacent unit tests so a `*-planning.md` filename is
      accepted. Acceptance: `npx nx run rhino-cli-rust:test` exits 0 and a test exercises a
      `-planning` suffix.
  - _Suggested executor: `swe-rust-dev`_
  - **Implementation Notes**: `swe-rust-dev` added `"planning"` to `WORKFLOW_TYPES` (workflows.rs:41)
    and two unit tests (`validate_naming_accepts_planning_suffix`, `..._rejects_bogus_suffix...`).
    `test:unit` → 527 passed; fmt + clippy `-D warnings` clean. Actual target is `test:unit`.
    No usage string enumerates suffixes, so none needed updating.
  - **Date**: 2026-06-04
  - **Status**: Completed
  - **Files Changed**: `apps/rhino-cli-rust/src/commands/workflows.rs`
- [x] [AI] Edit `apps/rhino-cli-go/cmd/workflows_validate_naming.go`: add `"planning"` to
      `var workflowTypes` and update the help/long-description string that enumerates allowed
      suffixes. Update `apps/rhino-cli-go/cmd/workflows_validate_naming_test.go` expectations
      (the message listing allowed suffixes) to include `planning`. Acceptance:
      `npx nx run rhino-cli-go:test` exits 0.
  - _Suggested executor: `swe-golang-dev`_
  - **Implementation Notes**: `swe-golang-dev` added `"planning"` to `workflowTypes` (line 17),
    updated the Long help string and the test's expected message to `(quality-gate, execution,
setup, planning)`, and added a `*-planning` accept case. `test:unit` passes; golangci-lint 0
    issues; gofmt clean. Actual target is `test:unit`.
  - **Date**: 2026-06-04
  - **Status**: Completed
  - **Files Changed**: `apps/rhino-cli-go/cmd/workflows_validate_naming.go`, `apps/rhino-cli-go/cmd/workflows_validate_naming_test.go`
- [x] [AI] Run `rhino-cli workflows validate-naming` (via `npx nx run rhino-cli-rust:...` built
      binary or the repo's wired command) against the workflows tree. Acceptance: command reports no
      violations for existing files (the new workflow lands in Phase 4).
  - **Implementation Notes**: Wired target is `validate:naming-workflows`. Ran for both projects:
    "Workflows naming validation: VALIDATION PASSED (0 violations)" on each. **Date**: 2026-06-04.
    **Status**: Completed. **Files Changed**: none (validation run).

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [x] [AI] `npx nx run rhino-cli-rust:test:unit` — exits 0. _Done: 527 passed._
- [x] [AI] `npx nx run rhino-cli-go:test:unit` — exits 0. _Done: all packages ok._
- [x] [AI] `printf 'repo-dependency-bump-planning\n' | grep -E -- '-(quality-gate|execution|setup|planning)$'` — prints the line (regex now accepts the suffix). _Done: REGEX_OK._

> **Pause Safety**: The `planning` type is accepted by the convention and both validators, but no
> workflow file uses it yet — naming validation is green. Safe to stop indefinitely. To resume:
> `npx nx run rhino-cli-rust:test && npx nx run rhino-cli-go:test`.

---

## Phase 3: Subagent Orchestration Convention

- [x] [AI] Create `repo-governance/development/agents/subagent-orchestration.md` _New file_,
      adopting the upstream subagent-orchestration convention referenced by the planning workflow:
      document the rule that delegated research/analysis subagents are capped at **3 concurrent**,
      grouped by batch rather than one-agent-per-item, with rationale and the principles it
      respects. Keep links to existing targets only (`web-research-delegation.md`,
      `automation-over-manual.md`). Acceptance: file exists;
      `grep -ci "concurren" <file>` ≥ 1; all relative links resolve.
  - _Suggested executor: `repo-rules-maker`_
  - **Implementation Notes**: Adopted the upstream ose-public convention verbatim in substance
    (Standard 1 cap=3, Standard 2 3-minute mtime stuck-detection, Standard 3 chunk sizing,
    Standard 4 agentId/task-notification handling, anti-patterns, tooling table). All 9 relative
    links verified with `test -f` (deliberate-problem-solving, root-cause-orientation,
    simplicity-over-complexity, explicit-over-implicit, quality, file-naming,
    agent-workflow-orchestration, ai-agents, ci-monitoring). `grep -ci concurren` = 8; prettier +
    markdownlint clean. (The upstream cites `ci-monitoring`/`agent-workflow-orchestration` rather
    than `web-research-delegation`; kept the faithful upstream link set, all of which exist here.)
  - **Date**: 2026-06-04
  - **Status**: Completed
  - **Files Changed**: `repo-governance/development/agents/subagent-orchestration.md` (new)
- [x] [AI] Add an index entry to `repo-governance/development/agents/README.md` linking
      `./subagent-orchestration.md`. Acceptance:
      `grep -c "subagent-orchestration.md" repo-governance/development/agents/README.md` ≥ 1.
  - **Implementation Notes**: Added Documents entry after Agent Workflow Orchestration. `grep -c` = 1;
    prettier clean. **Date**: 2026-06-04. **Status**: Completed.
    **Files Changed**: `repo-governance/development/agents/README.md`.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [x] [AI] `test -f repo-governance/development/agents/subagent-orchestration.md` — exits 0. _Done._
- [x] [AI] `npx prettier --check repo-governance/development/agents/subagent-orchestration.md repo-governance/development/agents/README.md` — passes. _Done._
- [x] [AI] `npx markdownlint-cli2 repo-governance/development/agents/subagent-orchestration.md` — no errors. _Done: 0 error(s)._

> **Pause Safety**: The concurrency-cap convention the planning workflow depends on now exists and
> is indexed. Safe to stop indefinitely. To resume: re-run the Phase 3 gate checks.

---

## Phase 4: Repository Dependency Bump Planning Workflow

- [x] [AI] Create `repo-governance/workflows/repo/repo-dependency-bump-planning.md` _New file_,
      adopting the upstream
      [Repository Dependency Bump Planning Workflow](https://github.com/wahidyankf/ose-public/blob/main/repo-governance/workflows/repo/repo-dependency-bump-planning.md)
      with frontmatter (`name: repo-dependency-bump-planning`, goal, termination, inputs, outputs)
      and all phases (0 Pre-flight → 6 Hand-back), the Gherkin success criteria, and the
      Related/Principles/Conventions sections. Adapt the Phase 1 Inventory scope to `ose-primer`'s
      real manifests per `tech-docs.md` (replace all `ose-public` app names; keep `.opencode/package.json`;
      use `infra/dev/**` Docker paths; inventory `global.json` in both .NET apps). Point every
      relative link at the verified existing targets in `tech-docs.md`, the new
      `dependency-bump-policy.md`, the new `subagent-orchestration.md`, and the new
      `security-waivers.md`. Acceptance: file exists; frontmatter `name:` equals the basename;
      every relative link resolves.
  - _Suggested executor: `repo-workflow-maker`_
  - **Implementation Notes**: Authored directly from the verbatim upstream workflow with ose-primer
    adaptations: frontmatter (`name: repo-dependency-bump-planning`, goal, termination, inputs,
    outputs), phases 0–6, Gherkin criteria, Related/Principles/Conventions. Phase 1 Inventory lists
    ose-primer's real manifests (Cargo: crud-be-rust-axum, rhino-cli-rust; .NET:
    crud-be-csharp-aspnetcore, crud-be-fsharp-giraffe + global.json; Go: crud-be-golang-gin,
    rhino-cli-go, golang-commons; Docker apps/\_ + infra/dev/\_\_; .opencode/package.json kept). All 16
    relative links verified with `test -f` (security-waivers.md resolves after Phase 5). Frontmatter
    `name:` = basename. Prettier + markdownlint clean.
  - **Date**: 2026-06-04
  - **Status**: Completed
  - **Files Changed**: `repo-governance/workflows/repo/repo-dependency-bump-planning.md` (new)
- [x] [AI] **Invocability adaptation (explicit user requirement)**: in the workflow body, adapt
      Phase 5 to `ose-primer`'s `plan-establishment-execution`, which has **no `target-stage`
      input** and places plans in `plans/in-progress/`. The adopted Phase 5 MUST: (a) invoke
      `plan-establishment-execution` as-is, (b) `git mv` the resulting plan to
      `plans/backlog/<YYYY-MM-DD>__<identifier>/` (backlog date-prefix per the Plans convention),
      (c) update `plans/in-progress/README.md` and `plans/backlog/README.md`. Verify every
      referenced primitive resolves in-repo: `test -f .claude/agents/web-research-maker.md`,
      `test -f repo-governance/workflows/plan/plan-establishment-execution.md`. Acceptance:
      `grep -c "target-stage" repo-governance/workflows/repo/repo-dependency-bump-planning.md` — the
      file does NOT pass an unsupported `target-stage: backlog` param; instead it documents the
      in-progress→backlog relocation.
  - _Suggested executor: `repo-workflow-maker`_
  - **Implementation Notes**: Phase 5 of the workflow body documents the repository adaptation —
    no `target-stage` arg is passed; instead it invokes `plan-establishment-execution` (lands in
    `in-progress/`), then `git mv`s to `plans/backlog/<YYYY-MM-DD>__<identifier>/` and updates both
    plan index READMEs. `grep -c target-stage` finds only the two "no target-stage" explanatory
    mentions, never a passed param. web-research-maker agent + plan-establishment-execution both
    verified present. **Date**: 2026-06-04. **Status**: Completed.
    **Files Changed**: `repo-governance/workflows/repo/repo-dependency-bump-planning.md`.
- [x] [AI] Add an index entry to `repo-governance/workflows/repo/README.md` linking
      `./repo-dependency-bump-planning.md` with a one-line description. Acceptance:
      `grep -c "repo-dependency-bump-planning" repo-governance/workflows/repo/README.md` ≥ 1.
  - **Implementation Notes**: Added to the Workflows list. `grep -c` = 1; prettier clean.
    **Date**: 2026-06-04. **Status**: Completed.
    **Files Changed**: `repo-governance/workflows/repo/README.md`.
- [x] [AI] If `repo-governance/workflows/README.md` contains a workflow catalog/table, add the new
      workflow there too. Acceptance: either the catalog lists it, or a note confirms the top-level
      README has no per-workflow catalog (verify by reading the file).
  - **Implementation Notes**: Top-level catalog DOES have an Available Workflows table, a Type
    Vocabulary table, and a Workflow Families list — added the workflow to all three, and added the
    `planning` row to the Type Vocabulary table (mirrors workflow-naming.md). `grep -c` in
    workflows/README.md = 3; prettier + markdownlint clean. **Date**: 2026-06-04.
    **Status**: Completed. **Files Changed**: `repo-governance/workflows/README.md`.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [x] [AI] `find repo-governance/workflows -name '*.md' -not -name 'README.md' -not -path '*/meta/*' | sed 's|.*/||; s|\.md$||' | grep -vE -- '-(quality-gate|execution|setup|planning)$'` — prints nothing (naming gate green, including the new file). _Done: no output._
- [x] [AI] `npx nx run rhino-cli-rust:validate:naming-workflows` and `npx nx run rhino-cli-go:validate:naming-workflows` — both exit 0 (the validators accept the new `-planning` file). _Done: "VALIDATION PASSED (0 violations)" on both._
- [x] [AI] **Invocability check**: every primitive the workflow references resolves in-repo —
      `test -f .claude/agents/web-research-maker.md`,
      `test -f repo-governance/workflows/plan/plan-establishment-execution.md`,
      `test -f repo-governance/development/workflow/dependency-bump-policy.md`,
      `test -f repo-governance/development/agents/subagent-orchestration.md` — all exit 0; and the
      workflow is indexed in `repo-governance/workflows/repo/README.md`. _Done: all 4 exist; indexed._
- [x] [AI] `npx prettier --check repo-governance/workflows/repo/repo-dependency-bump-planning.md repo-governance/workflows/repo/README.md` and `npx markdownlint-cli2 repo-governance/workflows/repo/repo-dependency-bump-planning.md` — pass. _Done: prettier clean; 0 markdownlint errors._

> **Pause Safety**: The planning workflow exists and passes naming validation; it references the
> policy, the concurrency convention, and the waiver register (created next). Safe to stop. To
> resume: re-run the Phase 4 gate naming command.

---

## Phase 5: Security Waivers Register

- [x] [AI] Create `docs/reference/security-waivers.md` _New file_: a long-lived waiver register with
      an intro paragraph, a schema/columns description (Package, Pinned Version, CVE(s) + URL,
      Severity, Release Date, Justification, Sign-off, KEV dateAdded, EPSS), and an explicit
      "No waivers recorded yet." line. Link back to `dependency-bump-policy.md`. Acceptance: file
      exists; `grep -ci "waiver" <file>` ≥ 1; links resolve.
  - _Suggested executor: `docs-maker`_
  - **Implementation Notes**: Created the register with "When to add an entry" (WAIVER /
    FUNCTIONAL-HOLD / KEV-listed), a full-width table header (Date, Package, Pinned Version, Status,
    CVE+URL, Severity, Release Date, EPSS, KEV dateAdded, KEV ransomware use, Justification,
    Sign-off), "No waivers recorded yet.", a field reference, and References. Links back to the
    policy and the planning workflow (both resolve). `grep -ci waiver` = 9; prettier + markdownlint
    clean. **Date**: 2026-06-04. **Status**: Completed.
    **Files Changed**: `docs/reference/security-waivers.md` (new).
- [x] [AI] Add an index entry to `docs/reference/README.md` linking `./security-waivers.md`.
      Acceptance: `grep -c "security-waivers.md" docs/reference/README.md` ≥ 1.
  - **Implementation Notes**: Added under "Quality Infrastructure". `grep -c` = 1; prettier clean.
    **Date**: 2026-06-04. **Status**: Completed. **Files Changed**: `docs/reference/README.md`.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [x] [AI] `test -f docs/reference/security-waivers.md` — exits 0. _Done._
- [x] [AI] `npx prettier --check docs/reference/security-waivers.md docs/reference/README.md` and `npx markdownlint-cli2 docs/reference/security-waivers.md` — pass. _Done: prettier clean; 0 markdownlint errors._

> **Pause Safety**: All five governance/reference artifacts now exist and are indexed. Safe to stop.
> To resume: re-run the Phase 5 gate checks.

---

## Phase 6: Cross-Reference Validation and Operational Readiness

### Local Quality Gates (Before Push)

- [x] [AI] Run affected typecheck/lint/test: `npx nx affected -t typecheck lint test:quick` —
      exits 0. Fix ALL failures found, including any preexisting (root cause orientation).
  - **Implementation Notes**: "Successfully ran targets typecheck, lint, test:quick for 23 projects"
    (+18 dependency tasks). rhino-cli-go coverage 90.09% ≥ 90% threshold. No failures. (One
    cosmetic Nx "flaky task" note for `crud-be-elixir-phoenix:codegen`; the run still succeeded.)
    **Date**: 2026-06-04. **Status**: Completed. **Files Changed**: none (gate run).
- [x] [AI] Validate internal links in all created/edited markdown files (resolve every relative
      link via `Bash test -f`/`test -d`). Acceptance: zero broken internal links.
  - _Suggested executor: `docs-link-checker`_
  - **Implementation Notes**: Resolved every `./`/`../` `.md` link across all 10 new/edited files
    via `test -f` → `broken_count=0`. The repo-wide `rhino-cli git pre-commit` link checker also
    passed on every commit (it blocked the Phase 4 commit until `security-waivers.md` existed,
    confirming it is active). **Date**: 2026-06-04. **Status**: Completed.
    **Files Changed**: none (validation).
- [x] [AI] Run the workflow-naming validators one final time:
      `npx nx run rhino-cli-rust:validate:naming-workflows && npx nx run rhino-cli-go:validate:naming-workflows`
      — both exit 0; also `npx nx run rhino-cli-rust:test:unit && npx nx run rhino-cli-go:test:unit` — both exit 0.
  - **Implementation Notes**: Both validators "VALIDATION PASSED (0 violations)"; both `test:unit`
    pass (covered by the affected run). **Date**: 2026-06-04. **Status**: Completed.
    **Files Changed**: none.

### Commit Guidelines

- [x] [AI] Commit thematically with Conventional Commits: e.g.
      `docs(governance): adopt dependency-bump policy from ose-public`,
      `feat(rhino-cli): add planning workflow type token`,
      `docs(workflows): adopt repo-dependency-bump-planning workflow`. Split concerns; do not bundle
      the validator change with unrelated docs.
  - **Implementation Notes**: Five thematic commits — establish plan; `docs(governance)` policy;
    `feat(rhino-cli)` planning type token (rust+go validators, isolated from docs);
    `docs(governance)` subagent-orchestration; `docs(workflows)` planning workflow + waivers
    register. **Date**: 2026-06-04. **Status**: Completed.

### Post-Push Verification

- [ ] [AI] Push to `origin main`. Monitor GitHub Actions for the push using `ScheduleWakeup` +
      single `gh run view` per the [CI Monitoring Convention](../../../repo-governance/development/workflow/ci-monitoring.md).
- [ ] [AI] Verify ALL CI workflows conclude `success`. If any fails, fix immediately (including
      preexisting failures), push a follow-up, and re-monitor. Do NOT proceed until CI is green.

### Phase 6 Gate

> All checks below must pass before archival.

- [ ] [AI] `npx nx affected -t typecheck lint test:quick` — exits 0.
- [ ] [AI] `git diff --name-only origin/main...HEAD` contains no dependency-manifest version change
      (no `package.json`/`Cargo.toml`/`rust-toolchain.toml`/`go.mod`/`*.csproj`/`*.fsproj`/
      `global.json`/`Dockerfile`/lockfile pin edits) — confirming Out-of-Scope held.
- [ ] [AI] GitHub Actions for the latest push all conclude `success`.

> **Pause Safety**: Adoption is complete, validated, pushed, and CI-green. Safe to stop. To resume:
> proceed to Plan Archival.

---

## Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI).
- [ ] [AI] Move plan folder to done via
      `git mv plans/in-progress/adopt-dependency-bump-policy plans/done/2026-06-04__adopt-dependency-bump-policy`.
- [ ] [AI] Update `plans/in-progress/README.md` — remove this plan's entry.
- [ ] [AI] Update `plans/done/README.md` — add the entry with completion date and summary.
- [ ] [AI] Search for orphaned references to the old in-progress path and fix them
      (`grep -rn "in-progress/adopt-dependency-bump-policy" .` returns nothing outside history).
- [ ] [AI] Commit: `chore(plans): move adopt-dependency-bump-policy to done` and push to
      `origin main`; verify CI green.
