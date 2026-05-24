# Delivery — Multi-Harness Compatibility

## Worktree

Worktree path: `worktrees/multi-harness-compatibility/`

Provision before execution (run from repo root):

```bash
claude --worktree multi-harness-compatibility
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

> **Execution note**: The user explicitly directed this plan to run **in the current branch (`main`)**, not
> in a worktree. The worktree gate is therefore intentionally bypassed; work proceeds in the repo root on
> `main` per the trunk-based-development default. The `## Worktree` section is retained to satisfy the plan
> convention and document the canonical path for anyone who later runs this in isolation.

## Environment Setup

- [ ] Confirm execution context: working in repo root on `main` (worktree gate bypassed per user
      instruction). Acceptance: `git rev-parse --abbrev-ref HEAD` prints `main`.
- [ ] Initialize toolchain: `npm install && npm run doctor -- --fix`. Verify `npm run doctor` exits 0. See
      [Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md).
- [ ] Build both rhino CLIs once: `npx nx build rhino-cli-rust && npx nx build rhino-cli-go`. Acceptance:
      both builds succeed.
- [ ] Verify the existing vendor-audit baseline is green in **both** CLIs:
      `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` and
      `npx nx run rhino-cli-go:validate:repo-governance-vendor-audit` — both exit 0 before any changes.
- [ ] Verify existing rhino tests pass before changes: `npx nx run rhino-cli-rust:test:quick` and
      `npx nx run rhino-cli-go:test:quick` — both exit 0.
- [ ] Verify the shadow-diff parity baseline is green:
      `bash apps/rhino-cli-rust/scripts/shadow-diff.sh agents repo-governance` — exits 0 before any changes.

## Phase 1 — Governance neutrality (vendor-audit + convention)

- [ ] Edit `repo-governance/conventions/structure/governance-vendor-independence.md`: add the new
      coding-agent product names (`\bJunie\b`, `\bJetBrains\b`, `Amazon Q\b`, `\bAntigravity\b`,
      `Pi Coding Agent`, `pi\.dev`, `\bEarendil\b`) to the "Coding-agent / harness product names" table, add
      binding paths (`\.junie/`, `\.amazonq/`, `\.pi/`, `\.gemini/`, `\.agent/`, `\.agents/`) to the
      "Vendor-specific binding directory paths" table, update the combined audit regex, add FP notes for
      `Amazon Q`/`pi`/`agy`, and add Vocabulary-Map rows. Per `tech-docs.md` §Vendor-Audit Extension.
  - _Suggested executor: `repo-rules-maker`_
  - Acceptance: the file's forbidden-terms tables and combined regex include every new term; FP notes added;
    `grep -E 'Junie|Antigravity|Amazon Q|Earendil|\\.amazonq/' repo-governance/conventions/structure/governance-vendor-independence.md`
    returns matches.
- [ ] TDD (Red): add failing detection coverage in **both** CLIs and a failing Gherkin scenario asserting
      that a seeded `Junie` / `Amazon Q` / `Antigravity` string in a temp governance fixture is reported by
      the vendor-audit. Edit
      `specs/apps/rhino/behavior/cli/gherkin/repo-governance/repo-governance-vendor-audit.feature` (existing
      file — extend it), add in-module tests to
      `apps/rhino-cli-rust/src/internal/repo_governance/vendor_audit.rs` (existing file — extend its
      `#[cfg(test)]` module), and add tests to `apps/rhino-cli-go/cmd/governance_vendor_audit_test.go`
      (existing file — extend it). Verify both **fail**:
      `npx nx run rhino-cli-rust:test:unit` and `npx nx run rhino-cli-go:test:unit`.
  - _Suggested executor: `swe-rust-dev`_ (Rust + Gherkin); Go test pairing delegated to `swe-golang-dev`.
  - Acceptance: both test suites show the new tests failing for the right reason (patterns absent).
- [ ] TDD (Green — Rust): edit `apps/rhino-cli-rust/src/internal/repo_governance/vendor_audit.rs` to add the
      new term and path patterns with FP guards (no bare `\bQ\b`/`\bpi\b`/`\bagy\b`). Verify
      `npx nx run rhino-cli-rust:test:unit` — new tests pass.
  - _Suggested executor: `swe-rust-dev`_
  - Acceptance: `nx run rhino-cli-rust:test:unit` and `:test:quick` both exit 0.
- [ ] TDD (Green — Go): make the **identical** change in `apps/rhino-cli-go/cmd/governance_vendor_audit.go`
      (same terms, same paths, same FP guards, same finding/suggestion output). Verify
      `npx nx run rhino-cli-go:test:unit` — new tests pass.
  - _Suggested executor: `swe-golang-dev`_
  - Acceptance: `nx run rhino-cli-go:test:unit` and `:test:quick` both exit 0.
- [ ] Prove byte-parity: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh repo-governance` — exits 0 (Rust
      and Go vendor-audit emit byte-identical output for the corpus). (AC11)
  - _Suggested executor: `swe-rust-dev`_
  - Acceptance: shadow-diff `repo-governance` corpus exits 0.
- [ ] TDD (Refactor): run the full audit on the repo with both binaries —
      `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` and
      `npx nx run rhino-cli-go:validate:repo-governance-vendor-audit` — both exit 0 (existing prose stays
      neutral; any newly-flagged leak is fixed at source or allowlisted).
  - _Suggested executor: `swe-rust-dev`_
  - Acceptance: both targets exit 0.
- [ ] Add FP-safety scenarios (AC2) to the vendor-audit feature file and paired tests in both CLIs: math
      constant `pi` in plain prose, bare capital `Q`, and a vendor name inside a "Platform Binding Examples"
      section are NOT reported. Verify `npx nx run rhino-cli-rust:test:unit` and
      `npx nx run rhino-cli-go:test:unit` pass, and `shadow-diff.sh repo-governance` exits 0.
  - _Suggested executor: `swe-rust-dev`_ (+ `swe-golang-dev` for the Go test pairing)
  - Acceptance: both unit suites green; shadow-diff `repo-governance` exits 0.

## Phase 2 — Multi-harness binding convention + catalog

- [ ] Create `repo-governance/conventions/structure/multi-harness-binding.md` _New file_ documenting: the
      two-tier strategy (AD2), AGENTS.md-canonical rule (AD1), no-shadowing rule (AD3 — `GEMINI.md`,
      `.junie/AGENTS.md`, `AGENTS.override.md` must never carry divergent content), mechanical-generation rule
      (AD4), and the dual-implementation parity rule (AD8). Include a Principles/Conventions-respected section
      per convention-writing standards. Keep all concrete vendor names/paths inside a `## Platform Binding
Examples` section so the vendor-audit stays green.
  - _Suggested executor: `repo-rules-maker`_
  - Acceptance: file exists, kebab-case, single H1, links resolve, vendor names only inside allowlisted
    regions; **both** `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` and
    `npx nx run rhino-cli-go:validate:repo-governance-vendor-audit` exit 0.
- [ ] Update `docs/reference/platform-bindings.md`: expand the Platform Binding Directories table to all nine
      named harnesses + OpenCode (+ keep the reserved Aider row) with columns from `tech-docs.md` §Harness
      Compatibility Matrix; replace the stale "Gemini CLI" row with "Antigravity CLI"; document provenance of
      pre-existing `.github/{agents,prompts,skills}` and `.codex/{config.toml,agents/}` bindings; add the
      no-shadowing note linking the new convention. Verify links: `npm run lint:md` — exits 0.
  - _Suggested executor: `docs-maker`_
  - Acceptance: each of the nine + OpenCode has a row recording root instruction file + AGENTS.md-native
    status + binding status (AC3); no "Gemini CLI" row remains; `npm run lint:md` exits 0.
- [ ] Run `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` and
      `npx nx run rhino-cli-go:validate:repo-governance-vendor-audit` — both exit 0 after the new convention
      is added.
  - Acceptance: both targets exit 0.

## Phase 3 — Binding emitter (both CLIs) + binding files

- [ ] TDD (Red): add a failing Gherkin feature + tests for the Amazon Q bridge emitter — given `AGENTS.md`,
      `agents emit-bindings` writes `.amazonq/rules/00-agents-md.md` pointing to `AGENTS.md` and a default
      agent JSON whose `resources` glob `file://AGENTS.md`. Create
      `specs/apps/rhino/behavior/cli/gherkin/agents/agents-bindings.feature` _New file_ + paired tests in
      `apps/rhino-cli-rust/` (in-module or `apps/rhino-cli-rust/tests/`) and `apps/rhino-cli-go/cmd/`. Verify
      both **fail**: `npx nx run rhino-cli-rust:test:unit` and `npx nx run rhino-cli-go:test:unit`.
  - _Suggested executor: `swe-rust-dev`_ (+ `swe-golang-dev` for the Go test pairing)
  - Acceptance: both suites show the new emitter tests failing for the right reason.
- [ ] TDD (Green — Rust): implement the emitter. Create
      `apps/rhino-cli-rust/src/internal/agents/bindings.rs` _New file_ with a single-source
      `expected_bindings()` derivation, create `apps/rhino-cli-rust/src/commands/agents_emit_bindings.rs`
      _New file_, and add an `EmitBindings` variant to the `AgentsCommands` enum in
      `apps/rhino-cli-rust/src/cli.rs` (existing file — wired in `apps/rhino-cli-rust/src/commands/agents.rs`).
      Verify `npx nx run rhino-cli-rust:test:unit` — passes.
  - _Suggested executor: `swe-rust-dev`_
  - Acceptance: `nx run rhino-cli-rust:test:unit` + `:test:quick` exit 0.
- [ ] TDD (Green — Go): implement the **byte-identical** emitter in Go. Create
      `apps/rhino-cli-go/internal/agents/bindings.go` _New file_ (mirror derivation), create
      `apps/rhino-cli-go/cmd/agents_emit_bindings.go` _New file_, and register the `emit-bindings` subcommand
      in `apps/rhino-cli-go/cmd/agents.go` (existing file). Verify `npx nx run rhino-cli-go:test:unit` —
      passes.
  - _Suggested executor: `swe-golang-dev`_
  - Acceptance: `nx run rhino-cli-go:test:unit` + `:test:quick` exit 0.
- [ ] Add `emit-bindings --dry-run` cases to the `agents` corpus in
      `apps/rhino-cli-rust/scripts/shadow-diff.sh` (mirroring the existing `agents sync --dry-run` cases) and
      prove byte-parity: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh agents` — exits 0. (AC11)
  - _Suggested executor: `swe-rust-dev`_
  - Acceptance: shadow-diff `agents` corpus includes the new dry-run cases and exits 0.
- [ ] Generate the Amazon Q bridge files (`.amazonq/rules/00-agents-md.md` + default agent JSON) _New,
      generated_ by running the emitter against the real tree (Rust binary):
      `./apps/rhino-cli-rust/dist/rhino-cli agents emit-bindings`. Confirm both files exist and reference
      `AGENTS.md` (AC4); do NOT duplicate `AGENTS.md` content verbatim. Acceptance:
      `test -f .amazonq/rules/00-agents-md.md && grep -q 'AGENTS.md' .amazonq/rules/00-agents-md.md` exits 0.
  - _Suggested executor: `swe-rust-dev`_
  - Acceptance: both bridge files exist; the grep exits 0; neither file copies an `AGENTS.md` body paragraph.
- [ ] Decide on optional thin pointers (default = none per AD2); record the decision in a new
      "Optional thin pointers" subsection of `docs/reference/platform-bindings.md`. Acceptance: a sentence
      recording the decision exists in `docs/reference/platform-bindings.md` and `npm run lint:md` exits 0.
  - _Suggested executor: `docs-maker`_
  - Acceptance: the decision sentence is present; `npm run lint:md` exits 0.
- [ ] If (and only if) thin pointers were decided in the previous item: emit each pointer
      (`.github/copilot-instructions.md`, `.cursor/rules/000-agents-md.mdc`,
      `.windsurf/rules/000-agents-md.md`) via the emitter and verify each is a pure pointer to `AGENTS.md`
      (no copied body paragraph). If the decision was "none", tick this item with a Not-Applicable note.
  - _Suggested executor: `swe-rust-dev`_
  - Acceptance: either each pointer is a verified pure pointer, OR an N/A note records the "none" decision.
- [ ] Verify `.gitignore` tracks new binding dirs: `git check-ignore .amazonq/rules/00-agents-md.md` returns
      nothing (not ignored). Fix `.gitignore` if needed. Also confirm prettier does not fight the emitter on
      the generated JSON (add `.amazonq/` to `.prettierignore` if a format conflict appears).
  - _Suggested executor: direct execution_
  - Acceptance: `git check-ignore .amazonq/rules/00-agents-md.md` prints nothing; `npx prettier --check .amazonq/`
    does not report the emitter-generated files (or `.amazonq/` is prettier-ignored).
- [ ] Confirm no shadowing file was created:
      `test ! -f GEMINI.md && test ! -f AGENTS.override.md && test ! -f .junie/AGENTS.md` (or, if any exists,
      it is a pure `AGENTS.md` pointer) (AC5).
  - _Suggested executor: direct execution_
  - Acceptance: the test command exits 0.

## Phase 3.5 — Deterministic pre-push parity guard (no agent, both CLIs)

Implements AD7: a deterministic, agent-free `agents validate-bindings` check (in both CLIs) that fails when a
committed binding file drifts from `AGENTS.md` or when a binding directory lacks a catalog row. Distinct from
the Phase 4 agent workflow (which handles external convention drift).

- [ ] TDD (Red): extend `specs/apps/rhino/behavior/cli/gherkin/agents/agents-bindings.feature` with
      `validate-bindings` scenarios and add failing tests in both CLIs — given a committed
      `.amazonq/rules/00-agents-md.md` deliberately mutated to differ from a regenerate, the command exits
      non-zero; given a binding dir with no catalog row, it exits non-zero. Verify both **fail**:
      `npx nx run rhino-cli-rust:test:unit` and `npx nx run rhino-cli-go:test:unit`.
  - _Suggested executor: `swe-rust-dev`_ (+ `swe-golang-dev` for the Go test pairing)
  - Acceptance: both suites show the new guard tests failing for the right reason.
- [ ] TDD (Green — Rust): implement the deterministic guard. Extend
      `apps/rhino-cli-rust/src/internal/agents/bindings.rs` (reuse `expected_bindings()`), add
      `apps/rhino-cli-rust/src/commands/agents_validate_bindings.rs` _New file_, and add a `ValidateBindings`
      variant to the `AgentsCommands` enum in `apps/rhino-cli-rust/src/cli.rs` (existing file). The guard
      re-derives each generated binding from `AGENTS.md` in memory, asserts byte-equality with the committed
      file, and asserts every binding dir on disk has a row in `docs/reference/platform-bindings.md`. NO
      network calls, NO agent. Verify `npx nx run rhino-cli-rust:test:unit` — passes.
  - _Suggested executor: `swe-rust-dev`_
  - Acceptance: `nx run rhino-cli-rust:test:unit` exits 0; guard exits 0 on the clean tree, non-zero on drift.
- [ ] TDD (Green — Go): implement the **byte-identical** guard in Go. Extend
      `apps/rhino-cli-go/internal/agents/bindings.go`, add
      `apps/rhino-cli-go/cmd/agents_validate_bindings.go` _New file_, register the `validate-bindings`
      subcommand in `apps/rhino-cli-go/cmd/agents.go` (existing file). Same derivation, same output. Verify
      `npx nx run rhino-cli-go:test:unit` — passes.
  - _Suggested executor: `swe-golang-dev`_
  - Acceptance: `nx run rhino-cli-go:test:unit` exits 0; guard output matches Rust.
- [ ] Add `validate-bindings` cases (clean + drifted fixture) to the `agents` corpus in
      `apps/rhino-cli-rust/scripts/shadow-diff.sh` and prove byte-parity:
      `bash apps/rhino-cli-rust/scripts/shadow-diff.sh agents` — exits 0. (AC11)
  - _Suggested executor: `swe-rust-dev`_
  - Acceptance: shadow-diff `agents` corpus includes the new cases and exits 0.
- [ ] Add a `validate:harness-bindings` Nx target to **both** `apps/rhino-cli-rust/project.json` and
      `apps/rhino-cli-go/project.json` (mirror the existing `validate:repo-governance-vendor-audit` target
      shape verbatim — Rust command:
      `cargo run --release -q --manifest-path apps/rhino-cli-rust/Cargo.toml -- agents validate-bindings`; Go
      command: `CGO_ENABLED=0 go run -C apps/rhino-cli-go main.go agents validate-bindings`; both with
      `"cache": false` since they read the working tree), preserving the identical-target-set invariant.
  - _Suggested executor: `swe-rust-dev`_ (+ `swe-golang-dev` for the Go target)
  - Acceptance: both `project.json` files contain a `validate:harness-bindings` target;
    `npx nx run rhino-cli-rust:validate:harness-bindings` and `npx nx run rhino-cli-go:validate:harness-bindings`
    both exit 0 on the clean tree.
- [ ] Add a `validate:harness-bindings` script to `package.json` wrapping the canonical Rust binary
      (mirror the existing `validate:sync` script:
      `nx run rhino-cli-rust:build --skip-nx-cache && ./apps/rhino-cli-rust/dist/rhino-cli agents validate-bindings`).
      Acceptance: `npm run validate:harness-bindings` exits 0 on the clean tree.
  - _Suggested executor: `swe-rust-dev`_
  - Acceptance: the script exists and `npm run validate:harness-bindings` exits 0.
- [ ] Wire the guard into `.husky/pre-push`: add a conditional block (mirroring the existing
      `validate:cross-vendor-parity` block) that runs `npm run validate:harness-bindings` when a binding
      surface changes (`.amazonq/`, `AGENTS.md`, `docs/reference/platform-bindings.md`, `.claude/`,
      `.opencode/`, `.codex/`, `.github/`). Acceptance: `grep 'validate:harness-bindings' .husky/pre-push`
      returns a match.
  - _Suggested executor: `swe-rust-dev`_
  - Acceptance: the grep returns a match; a clean-tree run of the hook chain exits 0.
- [ ] Prove the guard blocks drift: mutate `.amazonq/rules/00-agents-md.md`, run
      `npm run validate:harness-bindings` — exits non-zero; restore the file — exits 0 (AC9).
  - _Suggested executor: direct execution_
  - Acceptance: mutated → non-zero; restored → exit 0.

## Phase 4 — Compatibility-audit workflow + agents

- [ ] Create `.claude/agents/repo-harness-compatibility-checker.md` _New file_ — checker that, for each supported
      harness, delegates to `web-research-maker` to fetch current config conventions, diffs against
      `docs/reference/platform-bindings.md` + committed binding files, and writes a dual-labelled drift audit
      to `generated-reports/`. Follow agent frontmatter + naming conventions.
  - _Suggested executor: `agent-maker`_
  - Acceptance: `test -f .claude/agents/repo-harness-compatibility-checker.md` succeeds and
    `npx nx run rhino-cli-rust:validate:naming-agents` exits 0 (validates all agents, no path argument).
- [ ] Create `.claude/agents/repo-harness-compatibility-fixer.md` _New file_ — fixer that applies validated
      catalog/binding updates from a drift audit and re-validates before applying.
  - _Suggested executor: `agent-maker`_
  - Acceptance: `test -f .claude/agents/repo-harness-compatibility-fixer.md` succeeds and
    `npx nx run rhino-cli-rust:validate:naming-agents` exits 0.
- [ ] Create `repo-governance/workflows/repo/repo-harness-compatibility-quality-gate.md` _New file_ following the
      workflow pattern (frontmatter: name/title/goal/termination/inputs/outputs; phases; Gherkin success
      criteria), delegating to the two new agents and `web-research-maker` (AC6). Link the new
      `multi-harness-binding.md` convention. Add it to `repo-governance/workflows/repo/README.md`.
  - _Suggested executor: `repo-workflow-maker`_
  - Acceptance: `npx nx run rhino-cli-rust:validate:naming-workflows` exits 0 (validates all workflows, no
    path argument); the workflow is listed in `repo-governance/workflows/repo/README.md`.
- [ ] Sync agents to OpenCode: `npm run sync:claude-to-opencode` then `npm run validate:opencode` — both exit
      0; `.opencode/agents/repo-harness-compatibility-checker.md` and `...-fixer.md` generated.
  - _Suggested executor: `swe-rust-dev`_
  - Acceptance: both scripts exit 0; both OpenCode mirror files exist.
- [ ] Add the two agents to the `AGENTS.md` agent-family roster (family 6 "Cross-Vendor Parity", alongside
      `repo-parity-checker`/`repo-parity-fixer`) and to `.claude/agents/README.md` (Checkers + Fixers
      sections).
  - _Suggested executor: `repo-rules-maker`_
  - Acceptance: `grep 'repo-harness-compatibility-checker' AGENTS.md` and
    `grep 'repo-harness-compatibility-fixer' AGENTS.md` both return matches; same greps pass against
    `.claude/agents/README.md`.
- [ ] Validate workflow naming: `npx nx run rhino-cli-rust:validate:naming-workflows` and
      `npx nx run rhino-cli-go:validate:naming-workflows` — both exit 0 (name matches
      `repo-harness-compatibility-quality-gate`).
  - Acceptance: both targets exit 0.
- [ ] Invoke `repo-workflow-checker` on
      `repo-governance/workflows/repo/repo-harness-compatibility-quality-gate.md`; resolve all findings.
      Acceptance: `repo-workflow-checker` reports zero HIGH or CRITICAL findings.
  - _Suggested executor: `repo-workflow-checker`_
  - Acceptance: zero HIGH/CRITICAL findings after fixes; vendor-audit + naming validators still exit 0.

## Phase 5 — Specs coverage

- [ ] Ensure every new/changed rhino behavior has a paired Gherkin feature under `specs/apps/rhino/`
      (vendor-audit extension scenarios in `repo-governance/repo-governance-vendor-audit.feature`,
      binding-emitter + guard feature in `agents/agents-bindings.feature`). Update the relevant
      `agents/README.md` index. Run `npx nx run rhino-cli-rust:spec-coverage` and
      `npx nx run rhino-cli-go:spec-coverage` — both exit 0 (AC7).
  - _Suggested executor: `specs-maker`_
  - Acceptance: feature files cover all new behaviors; both spec-coverage targets exit 0.
- [ ] Run `npx nx run rhino-cli-rust:test:quick` and `npx nx run rhino-cli-go:test:quick` — both exit 0.
  - Acceptance: both exit 0.

## Phase 5.5 — Update all related Markdown files

Closing documentation sweep so no `.md` references a stale binding/vendor/agent/workflow set (AC10).

- [ ] Build the authoritative target list by grep: run
      `grep -rln --include='*.md' -e 'Platform Binding' -e 'platform-bindings' -e 'Gemini CLI' -e 'repo-parity-checker' . | grep -v node_modules | grep -v plans/done`
      and review each hit for staleness. Acceptance: a reviewed list exists (paste into the commit body or an
      Open Questions note in this file).
  - _Suggested executor: direct execution_
  - Acceptance: a reviewed list of actionable stale targets exists.
- [ ] Update `AGENTS.md`: confirm the two new agents are in the agent-family roster (added in Phase 4) and
      that no binding list is stale. (`ose-primer`'s `AGENTS.md` has no "Platform Bindings Catalog" sub-list
      or "**Future**:" bindings line, so only the roster needs to be current.) Acceptance:
      `grep 'repo-harness-compatibility' AGENTS.md` returns matches and
      `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` exits 0.
  - _Suggested executor: `repo-rules-maker`_
  - Acceptance: grep returns matches; vendor-audit exits 0.
- [ ] Update `CLAUDE.md` only if a new binding affects its documented dual-mode Claude↔OpenCode
      format-differences section (expected: no change). Acceptance:
      `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` still exits 0.
  - _Suggested executor: `repo-rules-maker`_
  - Acceptance: vendor-audit exits 0 (CLAUDE.md prose stays neutral outside allowlisted regions).
- [ ] Update index docs: `.claude/agents/README.md`, `repo-governance/workflows/repo/README.md`,
      `repo-governance/workflows/README.md`, `repo-governance/conventions/README.md`,
      `repo-governance/conventions/structure/README.md`, and `docs/reference/README.md`. Acceptance: each
      index references the new convention/workflow/agents as applicable and `npm run lint:md` exits 0.
  - _Suggested executor: `docs-maker`_
  - Acceptance: indexes reference the new artifacts; `npm run lint:md` exits 0.
- [ ] Add a downstream-propagation note to
      `repo-governance/conventions/structure/repository-ecosystem.md` that the new multi-harness binding
      scaffolding (convention, Amazon Q bridge, parity guard, compatibility workflow + agents) propagates
      downstream to forks of `ose-primer`. Acceptance: the file mentions the harness bindings and
      `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` exits 0.
  - _Suggested executor: `repo-rules-maker`_
  - Acceptance: the file mentions the harness binding propagation; vendor-audit exits 0.
- [ ] Re-grep for staleness: the Phase-5.5 grep returns no remaining stale references outside `plans/done`
      and intentional supersession notes (AC10). Run `npm run lint:md` — exits 0.
  - _Suggested executor: direct execution_
  - Acceptance: no stale references remain; `npm run lint:md` exits 0.

## Local Quality Gates (Before Push)

- [ ] Run affected typecheck: `npx nx affected -t typecheck` — exits 0.
- [ ] Run affected linting: `npx nx affected -t lint` — exits 0.
- [ ] Run affected quick tests: `npx nx affected -t test:quick` — exits 0.
- [ ] Run affected spec coverage: `npx nx affected -t spec-coverage` — exits 0.
- [ ] Run markdown lint: `npm run lint:md` — exits 0 (run `npm run lint:md:fix` first if needed).
- [ ] Run the governance vendor-audit in both CLIs:
      `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` and
      `npx nx run rhino-cli-go:validate:repo-governance-vendor-audit` — both exit 0.
- [ ] Run the full shadow-diff parity gate:
      `bash apps/rhino-cli-rust/scripts/shadow-diff.sh agents repo-governance` — exits 0.
- [ ] Run the deterministic binding-parity guard: `npm run validate:harness-bindings` — exits 0.
- [ ] Fix ALL failures found — including preexisting issues not caused by these changes (root cause
      orientation).

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes. This
> follows the root cause orientation principle — proactively fix preexisting errors encountered during work.

## Phase 6 — Governance rule propagation + validation

- [ ] Invoke `repo-rules-maker` to finalize/propagate the governance rules authored in Phases 1–2 and 4
      (vendor-independence update, multi-harness-binding convention, catalog/agent index entries), ensuring
      cross-links and indexes are consistent. Acceptance: `npm run lint:md` exits 0 and
      `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` exits 0 after propagation.
  - _Suggested executor: `repo-rules-maker`_
  - Acceptance: both checks exit 0.
- [ ] Run the `repo-rules-quality-gate` workflow in strict mode over the changed governance scope (invoke
      `repo-rules-checker` → `repo-rules-fixer` iteratively). Per
      [Repository Rules Quality Gate](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md).
      Acceptance: two consecutive `repo-rules-checker` runs over the changed governance files both return zero
      HIGH or CRITICAL findings (AC8).
  - _Suggested executor: `repo-rules-checker` → `repo-rules-fixer`_
  - Acceptance: double-zero (two consecutive zero-HIGH/CRITICAL checks) achieved.
- [ ] Re-run the vendor-audit (both CLIs), the shadow-diff parity gate, and
      `npx nx affected -t test:quick lint typecheck spec-coverage` — all exit 0.
  - Acceptance: all exit 0.

## Manual Behavioral Verification (CLI)

This plan touches a CLI and governance docs, not web UI or HTTP APIs — Playwright MCP and curl assertions are
**not applicable**. CLI behavior is verified by running the binaries directly:

- [ ] `npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit` — exits 0; seed a temp
      `Junie`/`Amazon Q` string in a temp `repo-governance/_tmp_seed_check.md` and confirm it is reported by
      both CLIs, then remove it.
  - _Suggested executor: direct execution_
  - Acceptance: seeded → both audits exit 1 and report the term; removed → both exit 0.
- [ ] Run the Amazon Q bridge emitter and inspect `.amazonq/rules/00-agents-md.md` — it references
      `AGENTS.md` and does not duplicate its body; the agent JSON `resources` glob `file://AGENTS.md`.
  - _Suggested executor: direct execution_
  - Acceptance: pointer references `AGENTS.md` with no body copy; JSON resources include `file://AGENTS.md`.
- [ ] (Optional, manual) Trigger the `repo-harness-compatibility-quality-gate` workflow and confirm it emits a
      drift report under `generated-reports/` citing web sources. If not run (network-heavy), tick with a
      note that the workflow wiring was validated by `repo-workflow-checker` instead.
  - _Suggested executor: direct execution_
  - Acceptance: either a drift report exists, or a note records that wiring was validated by
    `repo-workflow-checker`.

## Commit Guidelines

- [ ] Commit changes thematically — group related changes into logically cohesive commits.
- [ ] Follow Conventional Commits format: `<type>(<scope>): <description>`.
- [ ] Suggested split: (1) `feat(rhino): extend vendor-audit for new harness vendors (rust+go)`,
      (2) `docs(governance): add multi-harness-binding convention`,
      (3) `docs(reference): expand platform-bindings catalog to nine harnesses`,
      (4) `feat(rhino): emit Amazon Q binding bridge (rust+go)`,
      (5) `feat(rhino): add deterministic binding-parity pre-push guard (rust+go)`,
      (6) `feat(agents): add harness-compatibility checker/fixer + workflow`,
      (7) `test(rhino): spec coverage for harness bindings`,
      (8) `docs: update related markdown for multi-harness bindings`.
- [ ] Do NOT bundle unrelated fixes into a single commit (preexisting fixes get their own commits).

## Post-Push Verification

- [ ] Push changes to `main`.
- [ ] Monitor GitHub Actions workflows for the push (poll every 3 minutes via `ScheduleWakeup` + single
      `gh run view`; do not use `gh run watch` for long jobs). Pay attention to the `parity` job in
      `pr-quality-gate.yml`.
- [ ] Verify all CI checks pass.
- [ ] If any CI check fails, fix immediately and push a follow-up commit.
- [ ] Do NOT proceed to archival until CI is green (or, if no remote workflow is triggered for a direct
      `main` push of this change set, confirm the pre-push gate served as the quality gate and record that).

## Plan Archival

- [ ] Verify ALL delivery checklist items are ticked.
- [ ] Verify ALL quality gates pass (local + CI).
- [ ] Move plan folder from `plans/in-progress/` to `plans/done/` via `git mv` with completion-date prefix:
      `git mv plans/in-progress/multi-harness-compatibility plans/done/2026-05-25__multi-harness-compatibility`.
- [ ] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] Update `plans/done/README.md` — add the plan entry with completion date.
- [ ] Update any other READMEs that reference this plan.
- [ ] Commit: `chore(plans): move multi-harness-compatibility to done`.
