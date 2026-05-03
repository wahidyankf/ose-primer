---
title: Tech Docs — Adopt ose-public Vendor-Neutrality, OpenCode Go, and Companion Tooling
---

# Technical Approach

This document maps each workstream to its file-level porting scope,
decision log, and rollback plan. Read alongside `delivery.md` —
`tech-docs.md` is the _what_, `delivery.md` is the _how_ and the
_ticking order_.

## Source of truth

Canonical reference for every workstream is the tip of `wahidyankf/ose-public:main`
on the dates the source plans archived (2026-04-30 to 2026-05-03). Where
ose-public uses the import path
`github.com/wahidyankf/ose-public/apps/rhino-cli/...`, ose-primer uses
the same path — primer's `go.mod` already names
`github.com/wahidyankf/ose-public/apps/rhino-cli` so there is **no
import-path rewrite** required when porting Go code. (Confirmed by
the prior `2026-04-25__ose-public-governance-adoption` plan.)

Inside an ose-primer-rooted Claude session, `../../ose-public/...`
resolves to an empty directory by the bare-gitlink contract. Read
ose-public source files via:

- the GitHub UI: `https://github.com/wahidyankf/ose-public/tree/main/<path>`
- a parent-rooted Claude session opened from `/Users/wkf/ose-projects/`
- a temporary `git clone` of `wahidyankf/ose-public` into `/tmp/`

## Workstream W1 — Sync correctness

### Source

- `ose-public/apps/rhino-cli/internal/agents/converter.go`
  - `OpenCodeAgentDir = ".opencode/agents"` (plural)
- `ose-public/apps/rhino-cli/internal/agents/copier.go` — **does not exist**;
  ose-public removed it in the validate-claude-opencode-sync-correctness plan
  on the rationale that OpenCode reads `.claude/skills/` natively per
  [opencode.ai/docs/skills](https://opencode.ai/docs/skills/).
- `ose-public/apps/rhino-cli/cmd/agents_sync.go` — help text references
  `.opencode/agents/` (plural), no `.opencode/skill/` mention.

### File-level porting map

| File                                                          | Action  | Notes                                                                                                                                    |
| ------------------------------------------------------------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `apps/rhino-cli/internal/agents/converter.go`                 | replace | Switch `OpenCodeAgentDir` constant from `.opencode/agent` to `.opencode/agents`. Update doc comments. Remove deprecated path references. |
| `apps/rhino-cli/internal/agents/converter_test.go`            | update  | Update path assertions to plural; add a regression test asserting singular path is never written.                                        |
| `apps/rhino-cli/internal/agents/copier.go`                    | delete  | Remove file. OpenCode reads `.claude/skills/` natively — copy is redundant and writes to non-canonical singular path.                    |
| `apps/rhino-cli/internal/agents/copier_test.go`               | delete  | Remove file together with `copier.go`.                                                                                                   |
| `apps/rhino-cli/internal/agents/sync.go`                      | update  | Drop `CopyAllSkills` invocation. `Sync()` only converts agents.                                                                          |
| `apps/rhino-cli/internal/agents/sync_test.go`                 | update  | Remove skill-copy assertions; add assertion that `.opencode/skill/` is never created by the sync.                                        |
| `apps/rhino-cli/internal/agents/sync_validator.go`            | update  | Validate `.opencode/agents/` (plural) presence and parity. Flag `.opencode/agent/` and `.opencode/skill/` as drift.                      |
| `apps/rhino-cli/internal/agents/sync_validator_test.go`       | update  | Test fixtures use plural path.                                                                                                           |
| `apps/rhino-cli/cmd/agents_sync.go`                           | update  | Help text describes plural path; remove "copies skills to .opencode/skill/" lines.                                                       |
| `apps/rhino-cli/cmd/agents_sync_test.go`                      | update  | Update assertions.                                                                                                                       |
| `apps/rhino-cli/cmd/agents_sync.integration_test.go`          | update  | Path assertions plural.                                                                                                                  |
| `apps/rhino-cli/cmd/agents_validate_sync.go`                  | update  | Same — drift detection at plural path.                                                                                                   |
| `apps/rhino-cli/cmd/agents_validate_sync_test.go`             | update  | Path assertions plural.                                                                                                                  |
| `apps/rhino-cli/cmd/agents_validate_sync.integration_test.go` | update  | Path assertions plural.                                                                                                                  |
| `.opencode/agent/` (existing directory)                       | delete  | Replaced by `.opencode/agents/` after first sync.                                                                                        |
| `.opencode/skill/` (existing directory)                       | delete  | Removed (no replacement; OpenCode reads `.claude/skills/` natively).                                                                     |
| `.opencode/agents/` (new directory)                           | create  | Populated by `npm run sync:claude-to-opencode` post-W1.                                                                                  |
| `specs/apps/rhino/cli/gherkin/agents-sync.feature`            | update  | Add scenarios for plural-path output and singular-path absence.                                                                          |
| `specs/apps/rhino/cli/gherkin/agents-validate-sync.feature`   | update  | Add scenarios for plural-path drift detection.                                                                                           |

### Decisions

- **D1.1 — Stop copying skills entirely.** Adopt ose-public's decision
  per [opencode.ai/docs/skills](https://opencode.ai/docs/skills/):
  OpenCode reads `.claude/skills/` natively, so the copy is non-canonical
  AND redundant. This shrinks the sync surface and removes a drift
  vector. Risk: an OpenCode setting or version that doesn't read
  `.claude/skills/` natively breaks. Mitigation: documented in
  `AGENTS.md` and tested by the cross-vendor parity gate (W5).
- **D1.2 — Hard delete the singular directories**, do not leave them as
  empty stubs. Empty stubs invite re-population by accident.

### Rollback

If W1 breaks an OpenCode session for any reason, revert the single
commit that swaps `OpenCodeAgentDir`. The sync regenerates singular-path
files; manually restore singular `.opencode/skill/` from `.claude/skills/`
if the W1 commit also removed the copy logic.

## Workstream W2 — OpenCode Go provider

### Source

- `ose-public/apps/rhino-cli/internal/agents/converter.go` — `ConvertModel()`
  returns `opencode-go/minimax-m2.7` for opus/sonnet/omitted, `opencode-go/glm-5` for haiku.
- `ose-public/.opencode/opencode.json` — `model: opencode-go/minimax-m2.7`,
  `small_model: opencode-go/glm-5`, `provider.opencode-go.options.apiKey: {env:OPENCODE_GO_API_KEY}`,
  `mcp.perplexity` retained, no Z.ai MCPs.

### File-level porting map

| File                                                            | Action  | Notes                                                                                                                               |
| --------------------------------------------------------------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `apps/rhino-cli/internal/agents/converter.go`                   | update  | `ConvertModel()` returns `opencode-go/*` IDs (lines ~110-123 in primer's current file). Update doc comment.                         |
| `apps/rhino-cli/internal/agents/types.go`                       | update  | Update `OpenCodeAgent` doc comment.                                                                                                 |
| `apps/rhino-cli/cmd/agents_sync.go`                             | update  | Update model-mapping comment.                                                                                                       |
| `apps/rhino-cli/cmd/agents_validate_sync.go`                    | update  | Update model-mapping comment.                                                                                                       |
| `apps/rhino-cli/internal/agents/converter_test.go`              | update  | `TestConvertModel` expectations: `opus → opencode-go/minimax-m2.7`, `haiku → opencode-go/glm-5`.                                    |
| `apps/rhino-cli/internal/agents/types_test.go`                  | update  | `TestOpenCodeAgent` model expectation.                                                                                              |
| `apps/rhino-cli/internal/agents/sync_validator_test.go`         | update  | Model-string assertions.                                                                                                            |
| `apps/rhino-cli/cmd/steps_common_test.go`                       | update  | Rename step constant + regex if any reference Z.ai model IDs.                                                                       |
| `apps/rhino-cli/cmd/agents_sync.integration_test.go`            | update  | Model assertions.                                                                                                                   |
| `apps/rhino-cli/cmd/agents_validate_sync.integration_test.go`   | update  | Model assertions.                                                                                                                   |
| `apps/rhino-cli/cmd/agents_validate_naming.integration_test.go` | update  | Model in fixture.                                                                                                                   |
| `.opencode/opencode.json`                                       | replace | Switch `model`/`small_model`; add `provider.opencode-go.options.apiKey: {env:OPENCODE_GO_API_KEY}` block; remove any Z.ai MCPs.     |
| `governance/development/agents/model-selection.md`              | update  | Refresh OpenCode equivalents table to reference `opencode-go/*` IDs.                                                                |
| `.opencode/agents/*.md`                                         | regen   | `npm run sync:claude-to-opencode` regenerates all agent files at the canonical plural path with new model IDs in one atomic commit. |
| `.env.example`                                                  | update  | Document `OPENCODE_GO_API_KEY` env var.                                                                                             |

### Decisions

- **D2.1 — Tier collapse preserved.** Three Claude tiers (opus / sonnet / haiku)
  collapse to two OpenCode IDs (minimax-m2.7 / glm-5). Same shape as
  the existing Z.ai mapping; no agent definitions need editing.
- **D2.2 — Provider block rather than vendor-bundled MCP.** OpenCode
  Go's native Exa integration replaces Z.ai's bundled web-search MCPs.
  Perplexity MCP retained as a documented fallback per opencode-go's
  own setup guide.
- **D2.3 — Env var resolution.** API key resolved via
  `{env:OPENCODE_GO_API_KEY}` so unauthenticated sessions fail fast.

### Rollback

Revert the W2 commit; `ConvertModel()` returns Z.ai IDs again, and
`opencode.json` is restored to Z.ai. Re-run sync. Note: rollback
strands the W1 path migration since the model regeneration depends on
it. Do not roll back W1 without rolling back W2 first.

## Workstream W3 — rhino-cli vendor-audit scanner

### Source

- `ose-public/apps/rhino-cli/internal/governance/governance_vendor_audit.go` (313 lines)
- `ose-public/apps/rhino-cli/internal/governance/governance_vendor_audit_test.go` (502 lines)
- `ose-public/apps/rhino-cli/cmd/governance.go` (13 lines — Cobra group)
- `ose-public/apps/rhino-cli/cmd/governance_vendor_audit.go` (151 lines)
- `ose-public/apps/rhino-cli/cmd/governance_vendor_audit_test.go` (332 lines)
- `ose-public/specs/apps/rhino/cli/gherkin/governance-vendor-audit.feature`

### File-level porting map

| File                                                                 | Action | Notes                                                                                                                                             |
| -------------------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `apps/rhino-cli/internal/governance/governance_vendor_audit.go`      | create | New file. Copy from ose-public verbatim — module path matches.                                                                                    |
| `apps/rhino-cli/internal/governance/governance_vendor_audit_test.go` | create | New file. Copy from ose-public verbatim. Includes `\bSkills\b` test from `2026-05-03__rhino-cli-skills-vendor-term`.                              |
| `apps/rhino-cli/cmd/governance.go`                                   | create | New file. Cobra group registration.                                                                                                               |
| `apps/rhino-cli/cmd/governance_vendor_audit.go`                      | create | New file. Cobra subcommand binding scanner to CLI.                                                                                                |
| `apps/rhino-cli/cmd/governance_vendor_audit_test.go`                 | create | Unit test for command wiring.                                                                                                                     |
| `apps/rhino-cli/cmd/governance_vendor_audit.integration_test.go`     | create | Integration test against /tmp fixture trees.                                                                                                      |
| `apps/rhino-cli/cmd/steps_common_test.go`                            | update | Add step constants for `governance vendor-audit` Gherkin steps.                                                                                   |
| `apps/rhino-cli/cmd/root_test.go`                                    | update | Register the new `governance` Cobra group in command-tree assertions.                                                                             |
| `apps/rhino-cli/project.json`                                        | update | New target `validate:governance-vendor-audit` invokes `rhino-cli governance vendor-audit governance/`. Cacheable: inputs include `governance/**`. |
| `specs/apps/rhino/cli/gherkin/governance-vendor-audit.feature`       | create | Copy verbatim from ose-public; same scenarios.                                                                                                    |
| `.husky/pre-push`                                                    | update | Add `nx affected -t validate:governance-vendor-audit` to the pre-push chain (or include via `validate:cross-vendor-parity` umbrella in W5).       |
| `apps/rhino-cli/README.md`                                           | update | Add a "Governance vendor-audit" subsection documenting the new subcommand.                                                                        |

### Decisions

- **D3.1 — Pre-push wiring deferred to W5 umbrella.** Wire
  `validate:governance-vendor-audit` into the pre-push hook only via the
  `validate:cross-vendor-parity` quality gate from W5 to avoid double-running
  the scanner.
- **D3.2 — Allowlist mechanism preserved.** The
  `forbiddenConvention` constant exempts the convention definition
  file itself from the scanner. Same value as ose-public.
- **D3.3 — `\bSkills\b` term included from day one.** Adopt the
  `rhino-cli-skills-vendor-term` plan's nine-term forbidden list; do
  not ship the eight-term version that ose-public initially had.

### Rollback

Revert the W3 commit; the four new Cobra files and the `internal/governance/`
package are dropped. Pre-push hook returns to its prior state.

## Workstream W4 — Vendor-neutral governance

### Source

- `ose-public/governance/conventions/structure/governance-vendor-independence.md` (268 lines)
- `ose-public/AGENTS.md` (canonical, vendor-neutral with binding-example fences)
- `ose-public/CLAUDE.md` (thin shim with `@AGENTS.md` import directive)

### File-level porting map

| File                                                                 | Action  | Notes                                                                                                                                                |
| -------------------------------------------------------------------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `governance/conventions/structure/governance-vendor-independence.md` | create  | Port verbatim with primer-specific re-scoping (single-repo, no parent reference).                                                                    |
| `AGENTS.md`                                                          | rewrite | Replace current OpenCode-binding-only file with canonical vendor-neutral root instruction. Wrap vendor specifics in `binding-example` fences.        |
| `CLAUDE.md`                                                          | rewrite | Thin Claude Code binding shim. Top of file: `@AGENTS.md`. Body retains only Claude-Code-specific notes (e.g., model tier aliases) inside fences.     |
| `governance/**.md` (any file flagged by W3 scanner)                  | edit    | Remediate forbidden vendor terms: rewrite as capability tier (planning-grade / execution-grade / fast) or wrap in `binding-example` fence as needed. |
| `governance/development/agents/ai-agents.md`                         | edit    | Heavy lift — explicitly listed in ose-public's plan. Keep vendor-specific bits inside `binding-example` fences only.                                 |
| `governance/README.md`                                               | edit    | Update layer-test guidance to reference vendor-audit scanner.                                                                                        |
| `governance/development/agents/model-selection.md`                   | edit    | Use capability-tier vocabulary as canonical; vendor model IDs as binding examples.                                                                   |

### Decisions

- **D4.1 — AGENTS.md becomes canonical, CLAUDE.md becomes shim.**
  Mirrors ose-public's pattern; aligns with Linux Foundation Agentic
  AI Foundation AGENTS.md standard. Lets OpenCode / Codex CLI / Aider
  read the same canonical instructions natively.
- **D4.2 — Capability tiers over model names.** Governance prose uses
  `planning-grade` / `execution-grade` / `fast`. Vendor model IDs
  appear only inside `binding-example` fences in AGENTS.md / CLAUDE.md
  / `model-selection.md`.
- **D4.3 — Plans/ remains out of scope.** Per ose-public's convention,
  `plans/` may reference vendor specifics. No remediation under
  `plans/`.

### Rollback

Revert the W4 commits; AGENTS.md / CLAUDE.md restored, governance/
prose restored. The W3 scanner remains green only on the pre-W4
allowlist; rollback also requires reinstating the previous allowlist
or accepting a non-zero scanner exit code.

## Workstream W5 — Cross-vendor parity gate

### Source

- `ose-public/.claude/agents/repo-parity-checker.md`
- `ose-public/.claude/agents/repo-parity-fixer.md`
- `ose-public/governance/workflows/repo/repo-cross-vendor-parity-quality-gate.md`

### File-level porting map

| File                                                                   | Action | Notes                                                                                                                                                                                                                                                                                                                                                                                                    |
| ---------------------------------------------------------------------- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `.claude/agents/repo-parity-checker.md`                                | create | Port verbatim. Sonnet, green. Skills auto-load.                                                                                                                                                                                                                                                                                                                                                          |
| `.claude/agents/repo-parity-fixer.md`                                  | create | Port verbatim. Sonnet, yellow.                                                                                                                                                                                                                                                                                                                                                                           |
| `.opencode/agents/repo-parity-checker.md`                              | regen  | Generated by `npm run sync:claude-to-opencode`. Verify after sync.                                                                                                                                                                                                                                                                                                                                       |
| `.opencode/agents/repo-parity-fixer.md`                                | regen  | Same.                                                                                                                                                                                                                                                                                                                                                                                                    |
| `governance/workflows/repo/repo-cross-vendor-parity-quality-gate.md`   | create | Port verbatim. Iterative check-fix-verify. Terminates on two consecutive zero-finding runs. Default max-iterations=7, escalation=5.                                                                                                                                                                                                                                                                      |
| `apps/rhino-cli/scripts/validate-cross-vendor-parity.sh`               | create | Port verbatim from ose-public (~135 lines). Thin shell wrapper checking five invariants: (1) governance prose vendor-neutrality, (2) AGENTS.md/CLAUDE.md vendor-neutrality, (3) binding sync no-op, (4) agent count parity, (5a) color-translation map coverage, (5b) capability-tier map coverage. `cache: false` — reads `.opencode/agents/` count and runs `npm run sync` which is non-deterministic. |
| `apps/rhino-cli/project.json` (or new top-level `package.json` script) | update | Add Nx target `validate:cross-vendor-parity` with `"command": "bash apps/rhino-cli/scripts/validate-cross-vendor-parity.sh"` and `"cache": false`.                                                                                                                                                                                                                                                       |
| `.husky/pre-push`                                                      | update | Add conditional fire (see D3.1 / F3): fire `validate:cross-vendor-parity` only when `governance/**/*.md`, `AGENTS.md`, `CLAUDE.md`, `.claude/agents/`, or `.opencode/agents/` changed — mirrors ose-public's conditional `if [ -n "$RANGE" ]` pattern.                                                                                                                                                   |

### Decisions

- **D5.1 — Adopt agents and workflow verbatim.** Both agents are
  vendor-neutral by construction (model: sonnet, color: green/yellow,
  skill list); they don't need primer-specific edits.
- **D5.2 — Single Nx target, not five.** The five invariants are checked
  inside one `validate:cross-vendor-parity` target. Pre-push runs one
  target, not five.

### Rollback

Revert the W5 commits; agents and workflow files dropped. Pre-push hook
returns to previous chain. W4 remediation remains in place.

## Workstream W6 — Plans convention refresh

### Source

- `ose-public/governance/conventions/structure/plans.md` (lines ~180-220 cover the
  five-doc-DEFAULT and four-criteria single-file rule)

### File-level porting map

| File                                        | Action | Notes                                                                                                                                                       |
| ------------------------------------------- | ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `governance/conventions/structure/plans.md` | edit   | Replace the "Multi-File Structure" / "Single-File Structure" section with ose-public's stricter wording. Five-doc DEFAULT, four named single-file criteria. |

### Decisions

- **D6.1 — Wholesale replace the structure section, not a surgical word swap.**
  ose-public's rewrite reorganized the section; surgical edits would diverge.

### Rollback

Revert the single edit commit.

## Workstream W7 — Worktree standard

### Source

- `ose-public/governance/conventions/structure/worktree-path.md` (217 lines)
- `ose-public/governance/development/workflow/worktree-setup.md` (166 lines)

### File-level porting map

| File                                                | Action  | Notes                                                                                                                                                                                                                                  |
| --------------------------------------------------- | ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `governance/conventions/structure/worktree-path.md` | create  | Port adapted for primer: rule says **default** path is `.claude/worktrees/<name>/` (no override). Document gitignore + parallel-safety rationale. Cross-link to `worktree-setup.md`.                                                   |
| `governance/development/workflow/worktree-setup.md` | refresh | Resync against ose-public version. Do NOT import any `created:` or other date frontmatter fields per the [No-Date-Metadata Convention](../../../governance/conventions/writing/no-date-metadata.md). Update internal cross-references. |
| `AGENTS.md` (post-W4)                               | update  | Add a `## Worktrees` subsection linking to the new convention. One-paragraph summary.                                                                                                                                                  |
| `CLAUDE.md` (post-W4)                               | update  | Same — link to new convention from existing worktree subsection.                                                                                                                                                                       |
| `governance/conventions/structure/README.md`        | update  | Add `worktree-path.md` to the convention index.                                                                                                                                                                                        |

### Decisions

- **D7.1 — Primer documents the _default_ path, not an override.**
  ose-public overrides to root `worktrees/<name>/`; primer keeps the
  Claude Code default `.claude/worktrees/<name>/`. The convention is
  worth shipping anyway because consumers (and agents) need an
  authoritative source for the choice rather than CLAUDE.md prose.
- **D7.2 — No hook routing required.** ose-public installs a hook to
  re-route `claude --worktree`. Primer needs no hook because the
  default path matches the convention.

### Rollback

Revert the two file additions and the worktree-setup.md refresh.

## Workstream W8 — Plan + workflow refresh

### Source

- `ose-public/governance/workflows/plan/plan-execution.md` (770 lines)
- `ose-public/governance/workflows/plan/plan-quality-gate.md` (392 lines)
- `ose-public/governance/workflows/plan/README.md` (35 lines)
- `ose-public/governance/development/workflow/ci-monitoring.md` (285 lines)
- `ose-public/governance/development/workflow/ci-post-push-verification.md` (217 lines)

### File-level porting map

| File                                                           | Action  | Notes                                                                                                                                    |
| -------------------------------------------------------------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `governance/workflows/plan/plan-execution.md`                  | refresh | Replace with ose-public version; manually re-apply primer-specific phrasing (single-repo, no parent gitlinks, no Scope A/B distinction). |
| `governance/workflows/plan/plan-quality-gate.md`               | refresh | Same — replace with ose-public version. Termination rule: two consecutive zero-finding runs. Default max-iterations=7, escalation=5.     |
| `governance/workflows/plan/README.md`                          | refresh | Replace with ose-public version.                                                                                                         |
| `governance/development/workflow/ci-monitoring.md`             | create  | Port verbatim.                                                                                                                           |
| `governance/development/workflow/ci-post-push-verification.md` | create  | Port verbatim.                                                                                                                           |
| `governance/development/workflow/README.md`                    | update  | Add new files to the workflow index.                                                                                                     |

### Decisions

- **D8.1 — Refresh, not surgical merge.** Diff is too wide to merge
  surgically; replace then re-apply primer-specific phrasing.
- **D8.2 — CI workflows ship as-is.** ose-public's CI workflows describe
  generic patterns (poll a workflow run, verify a deployed URL); no
  primer adaptation needed.

### Rollback

Revert the W8 commits; previous plan/CI workflow files restored.

## Workstream W9 — TDD convention

### Source

- `ose-public/governance/development/workflow/test-driven-development.md` (316 lines)

### File-level porting map

| File                                                            | Action | Notes                                                                                                                  |
| --------------------------------------------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------- |
| `governance/development/workflow/test-driven-development.md`    | create | Port verbatim. Adjust cross-reference paths if any reference paths primer doesn't have (e.g., acceptance-criteria.md). |
| `governance/development/workflow/implementation.md`             | edit   | Add a one-line cross-reference to `test-driven-development.md` next to the existing Stage 1 description.               |
| `governance/development/workflow/README.md`                     | update | Add `test-driven-development.md` to the workflow index.                                                                |
| `governance/workflows/plan/plan-execution.md` (post-W8 refresh) | edit   | Add a one-line cross-reference to `test-driven-development.md` from the Iron Rules section.                            |
| `governance/conventions/structure/plans.md` (post-W6)           | edit   | One-line cross-reference from the delivery-checklist authoring section.                                                |

### Decisions

- **D9.1 — Sequencing.** W9 lands after W8 so the plan-execution and
  README cross-references can be added against the refreshed workflow
  files.
- **D9.2 — No code-level enforcement in this plan.** A future plan can
  extend `plan-checker` to mechanically check that delivery checklists
  follow Red→Green→Refactor; this plan ships only the convention.

### Rollback

Revert the W9 commits; `test-driven-development.md` removed,
cross-references reverted.

## Workstream W10 — Convention completeness

### Source

- `ose-public/governance/conventions/structure/no-last-updated.md` (29 lines)
- `ose-public/governance/conventions/structure/programming-language-docs-separation.md` (846 lines)

### File-level porting map

| File                                                                       | Action | Notes                                                                                                                                |
| -------------------------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------ |
| `governance/conventions/structure/no-last-updated.md`                      | create | Port verbatim. Tiny (29 lines); pairs with existing `no-date-metadata.md`.                                                           |
| `governance/conventions/structure/programming-language-docs-separation.md` | create | Port verbatim. Defines boundary between generic dev docs and language-specific docs (Go, TypeScript, Rust, etc.). Heavy (846 lines). |
| `governance/conventions/structure/README.md`                               | edit   | Add both new files to the convention index.                                                                                          |
| `governance/conventions/writing/no-date-metadata.md`                       | edit   | Add cross-reference to new `no-last-updated.md` companion.                                                                           |

### Decisions

- **D10.1 — Verbatim port for both.** Both conventions are template-grade
  scaffolding by ose-primer-sync classification (`governance/conventions/**`
  is `bidirectional identity`); no primer-specific adaptation needed.
- **D10.2 — W10 lands before W13.** W13's checker enforces W10's
  programming-language-docs-separation rule. Reverse ordering would ship
  enforcement of a non-existent rule.

### Rollback

Revert the W10 commits; both convention files dropped, cross-references
reverted. W13 (if landed) becomes a checker enforcing a missing rule —
revert W13 first, then W10.

## Workstream W11 — Plan anti-hallucination

### Source

- `ose-public/governance/development/quality/plan-anti-hallucination.md` (352 lines)

### File-level porting map

| File                                                        | Action | Notes                                                                                                                   |
| ----------------------------------------------------------- | ------ | ----------------------------------------------------------------------------------------------------------------------- |
| `governance/development/quality/plan-anti-hallucination.md` | create | Port verbatim. Enumerates concrete hallucination failure modes and verification checks each finding category must pass. |
| `governance/development/quality/README.md`                  | edit   | Add new file to quality index.                                                                                          |
| `governance/workflows/plan/plan-quality-gate.md` (post-W8)  | edit   | Add cross-reference to plan-anti-hallucination.md from the "Plan-Specific Validation" section.                          |
| `.claude/agents/plan-checker.md`                            | edit   | Add `plan-anti-hallucination` to the agent's reference set so audits can cite it.                                       |

### Decisions

- **D11.1 — Sequencing after W8.** plan-quality-gate.md is refreshed in
  W8; W11's cross-reference goes into the refreshed file, not the stale
  primer copy.
- **D11.2 — Skill alternative deferred.** ose-public doesn't ship a
  separate `plan-validating-anti-hallucination` skill; the convention
  is the authority and `plan-checker` reads it directly. If the
  convention proves long enough to slow `plan-checker` runs, a
  follow-up plan can extract a skill.

### Rollback

Revert the W11 commits; convention file dropped; cross-references
reverted. plan-checker still functions; only loses citation authority.

## Workstream W12 — Dev environment setup workflow

### Source

- `ose-public/governance/workflows/infra/development-environment-setup.md` (619 lines)

### File-level porting map

| File                                                                | Action         | Notes                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ------------------------------------------------------------------- | -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `governance/workflows/infra/infra-development-environment-setup.md` | refresh        | File already exists (684 lines). Refresh body content against ose-public's `development-environment-setup.md` (619 lines). Adapt: drop ose-public-specific app-list references; keep generic Volta + Docker + per-language-toolchain + env-var bootstrap. Filename stays `infra-development-environment-setup.md` per primer's workflow-naming convention (scope=`infra`, qualifiers=`development-environment`, type=`setup`). |
| `governance/workflows/infra/README.md`                              | edit-or-create | Add new file to infra workflow index. Create README if missing.                                                                                                                                                                                                                                                                                                                                                                |
| `governance/workflows/README.md`                                    | edit           | Add `infra/` subsection if not already present.                                                                                                                                                                                                                                                                                                                                                                                |
| `AGENTS.md` (post-W4)                                               | edit           | Add a one-line cross-reference from the dev-env-setup-related subsection.                                                                                                                                                                                                                                                                                                                                                      |
| `CLAUDE.md` (post-W4)                                               | edit           | Same — link from the dev-env-setup-related subsection.                                                                                                                                                                                                                                                                                                                                                                         |

### Decisions

- **D12.1 — Adapt, do not copy verbatim.** ose-public's workflow lists
  product-specific apps (organiclever, ayokoding, oseplatform); primer's
  list is the polyglot CRUD demo apps. Generic body (Volta, Docker,
  language toolchains, env vars, dependency install, doctor sweep) ports
  unchanged.
- **D12.2 — Sequencing after W2.** Document `OPENCODE_GO_API_KEY`
  env-var setup as part of W12's env-var section; depends on W2 having
  defined the env var.
- **D12.3 — Sequencing after W8.** Cross-references point at refreshed
  ci-monitoring.md and ci-post-push-verification.md.

### Rollback

Revert W12 commits; `infra-development-environment-setup.md` is restored to its pre-refresh state; cross-references reverted.

## Workstream W13 — Docs/SWE separation enforcement

### Source

- `ose-public/.claude/agents/docs-software-engineering-separation-checker.md` (511 lines)
- `ose-public/.claude/agents/docs-software-engineering-separation-fixer.md` (476 lines)
- `ose-public/.claude/skills/docs-validating-software-engineering-separation/SKILL.md` (248 lines)

### File-level porting map

| File                                                                       | Action | Notes                                                                                                                                             |
| -------------------------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `.claude/agents/docs-software-engineering-separation-checker.md`           | create | Port verbatim. Sonnet, green. Enforces W10 `programming-language-docs-separation.md` rule.                                                        |
| `.claude/agents/docs-software-engineering-separation-fixer.md`             | create | Port verbatim. Sonnet, yellow. Auto-moves misplaced language docs to canonical destination.                                                       |
| `.claude/skills/docs-validating-software-engineering-separation/SKILL.md`  | create | Port verbatim. Validating skill consumed by the checker.                                                                                          |
| `.opencode/agents/docs-software-engineering-separation-{checker,fixer}.md` | regen  | Generated by `npm run sync:claude-to-opencode`. Verify after sync.                                                                                |
| `.claude/agents/README.md`                                                 | edit   | Add the two new agents to the catalog under Checkers and Fixers sections.                                                                         |
| `.claude/skills/README.md`                                                 | edit   | Add the new skill to the skill catalog.                                                                                                           |
| `apps/rhino-cli/project.json` (or similar)                                 | edit   | Optional — wire a `validate:docs-swe-separation` Nx target if the agent supports CLI invocation. Defer to a future plan if pure-agent invocation. |

### Decisions

- **D13.1 — Hard dependency on W10.** Both agents and the skill cite
  `governance/conventions/structure/programming-language-docs-separation.md`.
  W10 must land before W13.
- **D13.2 — No CLI Nx-target wiring in this plan.** The agents run via
  the Agent tool; Nx target wiring is a future enhancement once the
  rhino-cli supports invoking checker agents from a Cobra command.

### Rollback

Revert W13 commits; agents and skill dropped from `.claude/`. Run
`npm run sync:claude-to-opencode` to regenerate `.opencode/agents/`
without the dropped agents. W10 remains as aspirational rule.

## Workstream W14 — Content drift sweep

### Source

- N/A — this workstream produces and consumes its own baseline diff. The
  authoritative reference is whatever `wahidyankf/ose-public:main` ships
  at the moment Phase 14A runs.

### File-level porting map

| Phase | Action                                                                                                                                                                                                                                                                                                                                                 |
| ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 14A   | Run `diff -rq governance/ /Users/wkf/ose-projects/ose-public/governance/` (filter to `*.md`); write the categorized report to `local-temp/drift-baseline.txt`. Classify each diverging file: **refresh** (port ose-public version with primer-specific re-phrase), **skip** (primer-specific override or product-specific), **investigate** (unclear). |
| 14B   | Refresh the three known-drifted files (`governance/development/quality/code.md`, `governance/development/infra/nx-targets.md`, `governance/development/quality/three-level-testing-standard.md`) against ose-public. Re-apply primer-specific paragraphs.                                                                                              |
| 14C   | Iterate refresh batches by directory (quality/, infra/, conventions/, principles/, workflows/) until the 14A report's `refresh` list is exhausted.                                                                                                                                                                                                     |
| 14D   | Verify post-refresh: re-run `diff -rq` against ose-public; only `skip`-classified files should diverge. Run `nx run rhino-cli:validate:governance-vendor-audit` against refreshed files; must return 0.                                                                                                                                                |

### Decisions

- **D14.1 — Phase 14A is mandatory baseline.** Skipping it and refreshing
  ad-hoc loses the audit trail and risks reverting primer-specific overrides.
- **D14.2 — Conservative `skip` classification.** When in doubt, classify
  as `investigate` not `refresh`. False positives (refreshing a file that
  should be primer-specific) are worse than false negatives.
- **D14.3 — Drift sweep is hygienic, not feature work.** Phase 14C may
  expand if the baseline reveals more drift than three files; cap at the
  4-iteration plan-quality-gate's max if the sweep exceeds 50 files.

### Rollback

Revert W14 commits one batch at a time; each batch was committed
thematically, so rollback is granular. Baseline diff in
`local-temp/drift-baseline.txt` survives as audit trail.

## Cross-workstream invariants

After all fourteen workstreams land:

- `nx affected -t typecheck lint test:quick spec-coverage` is green for affected projects.
- `nx run rhino-cli:validate:governance-vendor-audit` returns 0 violations.
- `nx run rhino-cli:validate:cross-vendor-parity` returns 0 findings on two consecutive runs.
- `npm run sync:claude-to-opencode` is a no-op on a clean tree.
- `ls .opencode/agent .opencode/skill 2>/dev/null` returns nothing.
- `ls .opencode/agents` returns the synced agent set including the two new W13 agents.
- `ls .opencode/skills` does not exist (W1 D1.1 decision).
- Every file in `governance/development/workflow/` and `governance/workflows/plan/`
  matches the ose-public version modulo primer-specific phrasing.
- The two new W10 conventions, the W11 quality doc, the W12 workflow (`infra-development-environment-setup.md` refreshed), the W13 triad
  files, and all W14 `refresh`-classified files are in place.
- Plan archived; gitlink-bump for parent is **out of scope** (parent tracks
  ose-primer separately; bumping is a parent-side decision after this plan ships).

## Reference: ose-public source plan SHAs (informational)

| ose-public source plan                                                                                  | Anchor              |
| ------------------------------------------------------------------------------------------------------- | ------------------- |
| 2026-04-30\_\_adopt-opencode-go                                                                         | W2                  |
| 2026-05-02\_\_governance-vendor-independence                                                            | W3, W4              |
| 2026-05-02\_\_validate-claude-opencode-sync-correctness                                                 | W1, W2 prerequisite |
| 2026-05-03\_\_cross-vendor-agent-parity                                                                 | W4 amendment, W5    |
| 2026-05-03\_\_rhino-cli-skills-vendor-term                                                              | W3 (`\bSkills\b`)   |
| (plans-convention rewrite, 2026-04-18 lineage)                                                          | W6                  |
| (worktree-path convention introduction)                                                                 | W7                  |
| (plan-execution / plan-quality-gate iteration commits)                                                  | W8                  |
| (test-driven-development convention introduction, 2026-05-02)                                           | W9                  |
| (no-last-updated + programming-language-docs-separation lineage)                                        | W10                 |
| (plan-anti-hallucination quality doc lineage)                                                           | W11                 |
| (development-environment-setup workflow lineage; primer file: `infra-development-environment-setup.md`) | W12                 |
| (docs-software-engineering-separation triad lineage)                                                    | W13                 |
| (content drift sweep — no single source plan; ongoing hygiene)                                          | W14                 |
