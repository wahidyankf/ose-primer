# Delivery — Mermaid State Diagram Validation (ose-primer)

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Worktree path: `worktrees/mermaid-state-diagram-validation/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree mermaid-state-diagram-validation
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized.
- [ ] [AI] Converge the toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift (Rust toolchain present).
- [ ] [AI] Build rhino-cli: `npx nx build rhino-cli`
      — acceptance: exits 0, release/debug binary produced.
- [ ] [AI] Establish the test baseline: `npx nx run rhino-cli:test:unit`
      — acceptance: pass/fail count recorded; all preexisting failures documented.
- [ ] [AI] Record the current mermaid baseline: `npx nx run rhino-cli:validate:mermaid`
      — acceptance: exit code and any current flowchart findings recorded as the baseline.
- [ ] [AI] Resolve all preexisting failures before proceeding
      — acceptance: no unresolved preexisting failures remain.

### Phase 0 Gate

> All checks below must pass before starting Phase A.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
- [ ] [AI] `npx nx run rhino-cli:test:unit` baseline recorded; every preexisting failure resolved.
- [ ] [AI] `npx nx run rhino-cli:validate:mermaid` baseline recorded (current pass/fail captured).

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no code
> changed. Safe to stop indefinitely. To resume: re-run `npx nx run rhino-cli:test:unit` and
> confirm it matches the recorded baseline.

## Phase A: Unify onto the fresh kind-agnostic module design

> _Behavior-preserving refactor. Every existing flowchart test stays green. No state support yet._
> _Suggested executor: `swe-rust-dev`_

- [ ] [AI] **RED**: Add a failing test in `apps/rhino-cli/src/internal/mermaid/diagram.rs` (_New
      file_) asserting `DiagramKind::detect("flowchart TB")` returns `Flowchart`.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: test fails to compile (`diagram` module / `DiagramKind` not defined).
- [ ] [AI] **GREEN**: Add `DiagramKind { Flowchart, State }` to
      `apps/rhino-cli/src/internal/mermaid/types.rs` and a `detect(header: &str) -> Option<DiagramKind>`
      in new `apps/rhino-cli/src/internal/mermaid/diagram.rs`; register both in `mod.rs`.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: the detect test passes; no other test broken.
- [ ] [AI] **REFACTOR**: Move the flowchart parsing logic from `parser.rs` into new
      `apps/rhino-cli/src/internal/mermaid/flowchart.rs` (siblings: `graph.rs`, `validator.rs`);
      have `diagram.rs` dispatch `Flowchart` → `flowchart::parse`. Delete `parser.rs` once empty;
      update `mod.rs` re-exports.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: all pre-existing flowchart tests still pass; `parser.rs` no longer referenced.
- [ ] [AI] **REFACTOR**: Confirm `graph.rs`, `validator.rs`, `reporter.rs` consume `ParsedDiagram`
      kind-agnostically — grep each for flowchart-only assumptions (e.g. `Direction::Td`) and remove
      any that would block a state diagram.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: all tests pass; no flowchart-only branch remains in the shared core.

### Local Quality Gates (Before Push) — Phase A

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck`
- [ ] [AI] Run affected linting: `npx nx affected -t lint`
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick`
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage`
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by these changes.
- [ ] [AI] Verify zero failures before pushing.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes. This follows the root cause orientation principle — proactively fix preexisting errors.
> Commit preexisting fixes separately with appropriate conventional commit messages.

### Commit Guidelines — Phase A

- [ ] [AI] Commit thematically; Conventional Commits format `refactor(rhino-cli): <description>`.
- [ ] [AI] Keep the re-shape in its own commit(s), separate from any preexisting fix.

### Post-Push CI Verification — Phase A

- [ ] [AI] Push changes to `main`.
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push.
- [ ] [AI] Verify ALL CI checks pass — no exceptions.
- [ ] [AI] If any CI check fails, fix immediately and push a follow-up commit; repeat until green.
- [ ] [AI] Do NOT proceed to Phase B until CI is fully green.

### Phase A Gate

> All checks below must pass before starting Phase B.

- [ ] [AI] `npx nx run rhino-cli:test:unit` — every pre-existing flowchart test passes; zero
      regressions.
- [ ] [AI] `npx nx run rhino-cli:validate:mermaid` — output is byte-identical to the Phase 0
      baseline (pure behavior-preserving refactor).
- [ ] [AI] Module layout matches the fresh design: `diagram.rs`, `flowchart.rs` exist; `parser.rs`
      removed; `state.rs` not yet added.

> **Pause Safety**: the validator is re-shaped onto the fresh design with flowchart behavior
> unchanged and pushed green. Safe to stop. To resume: `npx nx run rhino-cli:test:unit`.

## Phase B: State support + shared golden corpus

> _Add the state front-end and the parity-locking corpus._
> _Suggested executor: `swe-rust-dev`_

- [ ] [AI] **RED**: Add a failing test in `apps/rhino-cli/src/internal/mermaid/diagram.rs`
      asserting `DiagramKind::detect("stateDiagram-v2")` and `detect("stateDiagram")` both return
      `State`.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: test fails (state headers not yet recognized).
- [ ] [AI] **GREEN**: Extend `diagram.rs` detection to recognize `stateDiagram-v2` and
      `stateDiagram`.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: the detect test passes.
- [ ] [AI] **RED**: Add a failing test in new `apps/rhino-cli/src/internal/mermaid/state.rs`
      (_New file_) parsing an 11-state `stateDiagram-v2 direction LR` chain and asserting 11
      `Node`s on one rank.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: test fails to compile (`state` module / `parse` not defined).
- [ ] [AI] **GREEN**: Implement `state::parse(block) -> ParsedDiagram` covering bare id, `id : desc`,
      `state "desc" as id`, `direction TB|BT|LR|RL`, and `A --> B : lbl` per `tech-docs.md` mapping;
      have `diagram.rs` dispatch `State` → `state::parse`.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: the 11-state chain test passes.
- [ ] [AI] **RED**: Add a failing test in `state.rs` asserting `[*]` and `state X <<choice>>` each
      produce a counted `Node` (D-MAP / D-STEREO).
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: test fails (pseudostate/stereotype not yet parsed).
- [ ] [AI] **GREEN**: Parse `[*]` and `<<choice>>`/`<<fork>>`/`<<join>>` (and `[[…]]` aliases) as
      `Node`s in `state.rs`.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: the pseudostate/stereotype test passes.
- [ ] [AI] **RED**: Add a failing test in `state.rs` asserting a composite `state X { … }` becomes a
      recursed `Subgraph`, and `note … end note`, `%%`, `#`, and `--` are skipped.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: test fails (composite/notes/comments not yet handled).
- [ ] [AI] **GREEN**: Handle composite recursion and skip notes/comments/`--` (match `-->` before
      `--`) in `state.rs`.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: the composite/notes test passes.
- [ ] [AI] **RED**: Add a failing test in `apps/rhino-cli/src/internal/mermaid/validator.rs`
      asserting a transition label `A --> B : <31+ chars>` yields a `label_too_long` violation while
      states `A`/`B` are not flagged (D-LABEL).
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: test fails (transition label not yet checked).
- [ ] [AI] **GREEN**: Extend the label check in `validator.rs` to state transition labels in
      addition to display labels.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: the transition-label test passes.
- [ ] [AI] **REFACTOR**: Remove duplication between `flowchart.rs` and `state.rs` label/edge
      handling; keep the shared core in `graph.rs`/`validator.rs`.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: all tests still pass; no duplicated rule logic.
- [ ] [AI] Create the shared golden corpus under
      `apps/rhino-cli/src/internal/mermaid/testdata/state-corpus/` (_New directory_): `.md` fixtures
      covering over-wide LR chain, `[*]` rank, stereotype rank, over-long state label, over-long
      transition label, composite density, and a clean diagram, each with an `expected.json`.
      The fixtures and `expected.json` MUST be byte-identical to the ose-public reference corpus.
      — command: `ls apps/rhino-cli/src/internal/mermaid/testdata/state-corpus/`
      — acceptance: each fixture has a matching `expected.json`; contents match the ose-public
      reference corpus.
- [ ] [AI] **RED**: Add a corpus test (in `state.rs` or a new `mod tests` block) that runs
      `validate_blocks` over each fixture and asserts the produced violations equal `expected.json`.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: test fails for any fixture whose output the parser does not yet match.
- [ ] [AI] **GREEN**: Fix `state.rs`/`validator.rs` until every corpus fixture matches its
      `expected.json`.
      — command: `npx nx run rhino-cli:test:unit`
      — acceptance: every corpus fixture matches; output identical to the ose-public reference.

### Local Quality Gates (Before Push) — Phase B

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck`
- [ ] [AI] Run affected linting: `npx nx affected -t lint`
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick`
      — acceptance: exits 0; library line coverage stays ≥90%.
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage`
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by these changes.

### Commit Guidelines — Phase B

- [ ] [AI] Commit thematically; `feat(rhino-cli): validate mermaid state diagrams`.
- [ ] [AI] Keep the corpus fixtures in a logically cohesive commit.

### Post-Push CI Verification — Phase B

- [ ] [AI] Push changes to `main`; monitor ALL GitHub Actions workflows.
- [ ] [AI] Verify ALL CI checks pass; fix and re-push until green.
- [ ] [AI] Do NOT proceed to Phase C until CI is fully green.

### Phase B Gate

> All checks below must pass before starting Phase C.

- [ ] [AI] `npx nx run rhino-cli:test:unit` — all state, flowchart, and corpus tests pass.
- [ ] [AI] `npx nx run rhino-cli:test:quick` — coverage ≥90%, exits 0.
- [ ] [AI] Corpus `expected.json` fixtures are byte-identical to the ose-public reference corpus.

> **Pause Safety**: state-diagram validation works and is parity-locked to the reference, with
> flowchart behavior intact and pushed green. Safe to stop. To resume:
> `npx nx run rhino-cli:test:unit`.

## Phase C: Cleanup every violating state diagram repo-wide

> _D-CLEAN: fix every state diagram repo-wide, including `plans/done/` and gate-excluded paths._
> _Suggested executor: `swe-rust-dev` for the scan; doc fixes per-file._

- [ ] [AI] Run a no-exclusion repo-wide scan to enumerate violating state diagrams:
      `cargo run --release -q --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid -o json .`
      — acceptance: a JSON list of every state-diagram violation across the whole repo is captured
      (no `--exclude` flags; this deliberately reaches beyond the gate's default scan).
- [ ] [AI] For each violating state diagram (run
      `grep -rln stateDiagram --include='*.md' | grep -v node_modules` at Phase C start for the
      candidate set [Repo-grounded: command verified live]; only those actually >4 nodes/rank or >30-char labels need edits), rewrite the diagram to obey ≤4 nodes/rank and ≤30-char labels —
      split wide chains, shorten labels, or use composites.
      — command: re-run the scan from the previous step
      — acceptance: the diagram no longer appears in the violation list.
- [ ] [AI] Re-run the full no-exclusion scan until clean.
      — command: `cargo run --release -q --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid -o json .`
      — acceptance: zero state-diagram violations repo-wide (including `plans/done/`).
- [ ] [AI] Verify this plan's OWN diagrams pass the validator:
      `npx nx run rhino-cli:validate:mermaid`
      — acceptance: no violation reported for any diagram in
      `plans/in-progress/mermaid-state-diagram-validation/`.

### Local Quality Gates (Before Push) — Phase C

- [ ] [AI] Run markdown lint: `npm run lint:md` (MD028 no blank lines in blockquotes; MD038 no
      spaces in code spans) — exits 0.
- [ ] [AI] Run `npx nx run rhino-cli:validate:mermaid` — exits 0.
- [ ] [AI] Fix ALL failures, including preexisting markdown issues encountered.

### Commit Guidelines — Phase C

- [ ] [AI] Commit thematically; `docs(diagrams): narrow over-wide state diagrams`.
- [ ] [AI] Group cleanup edits by area; do not bundle with validator code changes.

### Post-Push CI Verification — Phase C

- [ ] [AI] Push changes to `main`; monitor ALL GitHub Actions workflows.
- [ ] [AI] Verify ALL CI checks pass; fix and re-push until green.

### Phase C Gate

> All checks below must pass before starting Phase D.

- [ ] [AI] No-exclusion repo-wide scan reports zero state-diagram violations.
- [ ] [AI] `npm run lint:md` exits 0.
- [ ] [AI] `npx nx run rhino-cli:validate:mermaid` exits 0.

> **Pause Safety**: every state diagram in the repo is within discipline and pushed green. Safe to
> stop. To resume: re-run the no-exclusion scan and confirm zero violations.

## Phase D: Gate

> _State diagrams ride the existing `validate:mermaid` target + pre-commit + CI — no new gate
> wiring. This phase confirms the rule is now live in the default path._
>
> _Note on the temp-file path: the negative-control files below use `scratch/` (a tracked,
> non-ignored path) DELIBERATELY, not the conventional `local-temp/`. `local-temp/` is gitignored,
> so (a) `git add local-temp/...` is refused, breaking the staged-only check, and (b) the validator's
> git-aware walk skips it, so the full-tree scan would never see the file — a silent false-negative
> that defeats the negative control. `scratch/` is removed in the same step, so nothing is committed._

- [ ] [AI] Confirm the default `validate:mermaid` target now validates state diagrams.
      Create `scratch/mermaid-state-test-TEMP.md` with a single fenced mermaid block containing
      `stateDiagram-v2 / direction LR / [*] --> S1 / S1 --> S2 / S2 --> S3 / S3 --> S4 / S4 --> S5 /
S5 --> S6 / S6 --> S7 / S7 --> S8 / S8 --> S9 / S9 --> [*]` (11-state LR chain — all nodes on
      one rank, exceeds the 4-node width limit). Run `npx nx run rhino-cli:validate:mermaid` and
      confirm it exits non-zero. Then remove: `rm scratch/mermaid-state-test-TEMP.md && git checkout -- .`
      — command: `npx nx run rhino-cli:validate:mermaid`
      — acceptance: exits non-zero with `width_exceeded` reported while the file is present; exits 0
      after removal.
- [ ] [AI] Confirm pre-commit scans state diagrams in staged files.
      Create `scratch/mermaid-state-test-TEMP.md` with the same 11-state `stateDiagram-v2 direction LR`
      content as above. Stage it with `git add scratch/mermaid-state-test-TEMP.md`. Run the staged-only
      invocation. Confirm it reports the violation. Clean up:
      `git rm --force scratch/mermaid-state-test-TEMP.md`
      — command: `git add scratch/mermaid-state-test-TEMP.md && cargo run --release -q --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --staged-only`
      — acceptance: staged scan reports `width_exceeded`; after cleanup `git status` shows no
      temporary file remaining.

### Phase D Gate

> All checks below must pass before starting Phase E.

- [ ] [AI] `npx nx run rhino-cli:validate:mermaid` fails on an over-wide state diagram and passes
      without it.
- [ ] [AI] No temporary test diagrams remain in the working tree (`git status` clean of them).

> **Pause Safety**: the live gate now enforces state-diagram width/label rules. Safe to stop. To
> resume: `npx nx run rhino-cli:validate:mermaid`.

## Phase E: Governance propagation

> _D-GOV: propagate the new rule into governance and re-sync platform bindings._
> _Suggested executor: `repo-rules-maker`_

- [ ] [AI] Invoke `repo-rules-maker` to add the state-diagram validation rule to
      `repo-governance/conventions/formatting/diagrams.md`: document that `stateDiagram-v2` and
      `stateDiagram` are now width-checked (≤4 nodes/rank, ≤30-char labels including transition
      labels), that `[*]` and stereotype states count as nodes, and that composites are treated as
      subgraphs.
      — command: `npx nx run rhino-cli:validate:links` and `npm run lint:md`
      — acceptance: `diagrams.md` describes the state-diagram rule; both commands exit 0.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Update any repo-rules registers/checkers that enumerate Mermaid rules so the
      state-diagram rule is listed (search: `grep -rln "width_exceeded\|validate-mermaid\|Mermaid"
repo-governance/ .claude/agents/`).
      — command: `grep -rln "validate-mermaid" repo-governance/`
      — acceptance: every enumerating surface lists the state-diagram rule.
- [ ] [AI] Re-sync platform bindings: `npm run generate:bindings`
      — acceptance: exits 0; `.opencode/` and `.amazonq/` artifacts regenerated; `git diff` shows
      only intended binding updates.
- [ ] [AI] Verify cross-vendor parity guard still passes:
      `npx nx run rhino-cli:validate:cross-vendor-parity`
      — acceptance: exits 0.

### Local Quality Gates (Before Push) — Phase E

- [ ] [AI] Run `npm run lint:md` — exits 0.
- [ ] [AI] Run `npx nx run rhino-cli:validate:links` — exits 0.
- [ ] [AI] Run `npx nx affected -t lint typecheck test:quick` — exits 0.

### Commit Guidelines — Phase E

- [ ] [AI] Commit thematically; `docs(governance): document mermaid state-diagram validation`.
- [ ] [AI] Keep binding re-sync output in a cohesive commit.

### Post-Push CI Verification — Phase E

- [ ] [AI] Push changes to `main`; monitor ALL GitHub Actions workflows.
- [ ] [AI] Verify ALL CI checks pass; fix and re-push until green.

### Phase E Gate

> All checks below must pass before starting Phase F.

- [ ] [AI] `diagrams.md` documents the state-diagram rule; `npm run lint:md` exits 0.
- [ ] [AI] `npm run generate:bindings` produced a clean, intended diff; binding parity guard passes.
- [ ] [AI] `npx nx affected -t lint typecheck test:quick` exits 0.

> **Pause Safety**: governance and bindings describe the live rule and are pushed green. Safe to
> stop. To resume: `npm run lint:md && npx nx run rhino-cli:validate:links`.

## Phase F: Verify and archive

> _Suggested executor: `plan-execution-checker` for verification._

- [ ] [AI] Run `plan-execution-checker` against this plan.
      — acceptance: zero findings (all delivery items ticked, all acceptance criteria met).
- [ ] [AI] Final full local gate: `npx nx affected -t typecheck lint test:quick spec-coverage`
      — acceptance: exits 0.
- [ ] [AI] Final mermaid gate: `npx nx run rhino-cli:validate:mermaid` — exits 0.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI).
- [ ] [AI] Verify the golden corpus matches the ose-public reference.
- [ ] [AI] Rename and move:
      `git mv plans/in-progress/mermaid-state-diagram-validation/ plans/done/2026-06-12__mermaid-state-diagram-validation/`
      using today's date as the completion date.
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date.
- [ ] [AI] Update any other READMEs that reference this plan.
- [ ] [AI] Commit the archival: `chore(plans): move mermaid-state-diagram-validation to done`.

### Phase F Gate

> Final gate. All checks below must pass to declare the plan complete.

- [ ] [AI] `plan-execution-checker` reports zero findings.
- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` exits 0.
- [ ] [AI] Plan folder lives under `plans/done/2026-06-12__mermaid-state-diagram-validation/` and all
      READMEs are updated.

> **Pause Safety**: the objective is complete, verified, and archived in this repo. Safe to stop.
> To resume (if reopened): `npx nx run rhino-cli:test:unit`.
