# Technical Documentation — Plan Domain Parity (ose-primer)

## Architecture of the Change

The merged canon is produced upstream (ose-public) and adopted here by manual semantic
3-way merge per file, preserving primer-specific divergences. Code changes land in both
rhino CLIs under TDD, with the existing dual-CLI parity guard as the equivalence oracle.

```mermaid
flowchart LR
  accTitle: Canon flow into ose-primer
  accDescr: ose-public lands merged canon first; primer adopts via semantic merge; emitters regenerate bindings; parity guard validates both CLIs.
  A[ose-public canon] --> B[Semantic merge into primer]
  B --> C[rhino-cli emitters]
  C --> D[Bindings regen + audit]
  D --> E[Parity guard]

  style A fill:#0072B2,color:#FFFFFF
  style B fill:#E69F00,color:#000000
  style C fill:#009E73,color:#FFFFFF
  style D fill:#56B4E9,color:#000000
  style E fill:#CC79A7,color:#000000
```

## Deviation Matrix (Verbatim)

The following is the complete resolved deviation matrix produced by the
plan-multi-repo-parity-planning workflow on 2026-06-06 (source:
`ose-public/local-temp/plan-domain-parity-matrix.md`, a gitignored working file —
embedded here verbatim so the decision record survives in git).

> Objective: same/similar quality and behavior of `repo-governance/workflows/plan/` and
> its related agents and skills across ose-public, ose-primer, ose-infra. Mode:
> worktree-to-main. Gate: strict (double-zero). Slug: `plan-domain-parity`. Stage:
> `plans/in-progress/`.
>
> Sibling repo roots (local clones): `/Users/wkf/ose-projects/ose-public`,
> `/Users/wkf/ose-projects/ose-primer`, `/Users/wkf/ose-projects/ose-infra` (bare +
> worktrees layout).

### Survey Facts (Empirical, 2026-06-06)

- `plan-quality-gate.md`: byte-identical in all 3 repos (no row).
- `plan-multi-repo-parity-planning.md`: exists only in ose-public.
- Pairwise drift (changed lines, diff): plan-establishment-execution 92–143;
  plan-execution 30–46; workflows/plan/README 7–31; meta/execution-modes 40–102;
  plan-maker 106–134; plan-checker 96–118; plan-fixer 125–170; plan-execution-checker
  41–81; repo-setup-manager 0 (pub↔inf), 3 (primer); plan-creating-project-plans SKILL
  169–243; plan-writing-gherkin-criteria SKILL 2–10; grill-me SKILL 25–52;
  conventions/structure/plans.md 107–125.
- primer `plan-establishment-execution.md` lacks the `target-stage` input (public+infra
  have it).
- Grilling convention: public
  `repo-governance/development/workflow/grilling-with-options.md`; infra
  `repo-governance/development/workflow/grilling.md` (different name, broader wording);
  primer none.
- Harness dirs already aligned in all 3: `.opencode/`, `.amazonq/{rules,cli-agents}`,
  `.codex/{agents,config.toml}`.
- `generate:bindings`: public
  `cargo run --manifest-path apps/rhino-cli/Cargo.toml -- agents sync && … emit-bindings`;
  primer `nx run rhino-cli-rust:build && ./apps/rhino-cli-rust/dist/rhino-cli …`; infra
  `nx run rhino-cli:build && ./apps/rhino-cli/dist/rhino-cli …`.
- primer has dual CLIs: `apps/rhino-cli-rust` (canonical for bindings) +
  `apps/rhino-cli-go` (no bindings emission). _(See
  [Survey Corrections](#survey-corrections-2026-06-06-plan-authoring) — partially
  outdated for the Go CLI.)_
- primer has in-progress plan `planning-system-overhaul` (adopting resolved ose-public
  planning gaps) — overlaps this objective.
- infra constraint: private repo, self-hosted CI runners
  `[self-hosted, linux, ose-infra-runner]`, no ubuntu-latest.

### Resolved Decisions (All Grilled With Invoker, 2026-06-06; Zero Undecided Rows)

| #   | Dimension                                                          | Resolution                                                                                                                                                                                                                                                                                                                                                                                                                             | Justification                                                                                                                               |
| --- | ------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | parity workflow existence (public only)                            | Propagate `plan-multi-repo-parity-planning.md` to primer + infra                                                                                                                                                                                                                                                                                                                                                                       | Workflow must be invocable from any anchor repo                                                                                             |
| 2   | parity workflow grill structure                                    | Amend ALL copies: steps become Survey → Matrix → **First Grill (hard gate)** → **web-research-maker (conditional)** → **Second Grill (post-research)** → Author → Gate → Deliver — mirroring plan-establishment-execution's two-grill+research pattern                                                                                                                                                                                 | Invoker requirement (2026-06-06)                                                                                                            |
| 3   | plan-establishment-execution drift; primer missing target-stage    | 3-way best-of merge; merged version keeps `target-stage`; **NEW default behavior in merged version (all repos)**: plan authored in designated worktree `worktrees/<identifier>/`, provisioned if absent via `git worktree add -b <identifier> worktrees/<identifier> main` + `npm install` + `npm run doctor -- --fix`; commit in worktree; push HEAD to confirmed push-target (default `origin main`); remove worktree after delivery | Invoker directives (2026-06-06): worktree default + branch-wt-push-main mechanics                                                           |
| 4   | plan-execution.md drift                                            | 3-way best-of merge; repo-specific agent-selection lists preserved                                                                                                                                                                                                                                                                                                                                                                     | Best content from all repos, no improvement lost                                                                                            |
| 5   | workflows/plan/README.md index                                     | Align post-propagation: 4 workflows indexed everywhere                                                                                                                                                                                                                                                                                                                                                                                 | Follows row 1                                                                                                                               |
| 6   | workflows/meta/execution-modes.md drift                            | 3-way best-of merge                                                                                                                                                                                                                                                                                                                                                                                                                    |                                                                                                                                             |
| 7   | plan-maker agent drift                                             | 3-way best-of merge; repo-specific refs preserved                                                                                                                                                                                                                                                                                                                                                                                      |                                                                                                                                             |
| 8   | plan-checker agent drift                                           | 3-way best-of merge                                                                                                                                                                                                                                                                                                                                                                                                                    |                                                                                                                                             |
| 9   | plan-fixer agent drift                                             | 3-way best-of merge                                                                                                                                                                                                                                                                                                                                                                                                                    |                                                                                                                                             |
| 10  | plan-execution-checker agent drift                                 | 3-way best-of merge                                                                                                                                                                                                                                                                                                                                                                                                                    |                                                                                                                                             |
| 11  | repo-setup-manager primer 3-line drift                             | Keep if repo-specific (rhino-cli-rust naming), else merge                                                                                                                                                                                                                                                                                                                                                                              | Likely intentional                                                                                                                          |
| 12  | plan-creating-project-plans skill drift; infra adds grilling gates | 3-way best-of merge **including infra's mandatory grilling gates**                                                                                                                                                                                                                                                                                                                                                                     | Sibling improvement adopted                                                                                                                 |
| 13  | plan-writing-gherkin-criteria skill drift                          | 3-way merge (trivial)                                                                                                                                                                                                                                                                                                                                                                                                                  |                                                                                                                                             |
| 14  | grill-me skill drift                                               | 3-way best-of merge                                                                                                                                                                                                                                                                                                                                                                                                                    |                                                                                                                                             |
| 15  | grilling convention naming                                         | Merged content lands as `grilling-with-options.md` in all 3; **infra renames `grilling.md` → `grilling-with-options.md` + full link sweep**; primer gains the file                                                                                                                                                                                                                                                                     | Public name already cited by all public workflows + AGENTS.md; sweep cost confined to infra                                                 |
| 16  | conventions/structure/plans.md drift                               | 3-way best-of merge                                                                                                                                                                                                                                                                                                                                                                                                                    |                                                                                                                                             |
| 17  | harness binding coverage                                           | **Full repo-wide binding audit** per repo: all agents × .opencode/.amazonq/.codex + `validate:harness-bindings` (or equivalent) passes                                                                                                                                                                                                                                                                                                 | Invoker chose maximal scope                                                                                                                 |
| 18  | OpenCode emitter format                                            | Modernize rhino-cli OpenCode emitter: deprecated boolean `tools` flags → `permission` object; regenerate mirrors                                                                                                                                                                                                                                                                                                                       | Research: opencode.ai/docs/agents (2026-06-05) deprecates boolean flags                                                                     |
| 19  | .codex/agents/ unofficial                                          | Migrate per-agent Codex config to `.codex/config.toml` `agents.<name>` sub-tables; **stop emitting `.codex/agents/`**                                                                                                                                                                                                                                                                                                                  | Research: official convention is config.toml sub-tables (developers.openai.com/codex/config-reference); .codex/agents/ not Codex-recognized |
| 20  | generate:bindings invocation                                       | Align all 3 to direct `cargo run --manifest-path <rhino-cli manifest>`; primer uses `apps/rhino-cli-rust/Cargo.toml`                                                                                                                                                                                                                                                                                                                   | Uniform invocation; accepted loss of nx build caching wrapper                                                                               |
| 21  | primer dual-CLI emitters                                           | Rust stays canonical in script; **port bindings emission (agents sync + emit-bindings) to rhino-cli-go** for capability parity, validated by the dual-CLI parity guard (NOT wired into generate:bindings script)                                                                                                                                                                                                                       | Invoker chose go-port scope; script stays rust-canonical (confirmed in second grill)                                                        |
| 22  | primer PR-only sync convention vs worktree-to-main                 | **Deviation accepted**: primer plan pushed direct to its `origin main` from worktree; recorded here + in rationale doc                                                                                                                                                                                                                                                                                                                 | Invoker-approved; plan files low-risk; Safety Invariant 6 deviation documented                                                              |
| 23  | primer planning-system-overhaul overlap                            | **Supersede + absorb**: primer parity plan absorbs remaining overhaul items; old plan closed/archived with pointer to the parity plan                                                                                                                                                                                                                                                                                                  | Single source of truth for primer planning-system work                                                                                      |
| 24  | rationale doc location                                             | `docs/explanation/plan-domain-parity-decisions.md` in all 3                                                                                                                                                                                                                                                                                                                                                                            | Uniform; infra docs/explanation tree exists                                                                                                 |
| 25  | slug / stage / gate                                                | `plan-domain-parity`; `plans/in-progress/`; plan-quality-gate strict double-zero                                                                                                                                                                                                                                                                                                                                                       |                                                                                                                                             |
| 26  | drift guard                                                        | **Drop** — upstream-first editing left implicit; no automated cross-repo drift checker added                                                                                                                                                                                                                                                                                                                                           | Invoker decision; recorded so the drop is deliberate, not silent                                                                            |

### Research Findings (web-research-maker, 2026-06-06, Cited)

- OpenCode (official docs, accessed 2026-06-05): agents at `.opencode/agents/` (plural);
  `tools` boolean flags **deprecated** → `permission` object (`allow`/`ask`/`deny` per
  tool); reads `.claude/skills/<name>/SKILL.md` natively (no skill mirroring needed —
  current repo pattern vindicated).
  <https://opencode.ai/docs/agents/> (accessed 2026-06-05),
  <https://opencode.ai/docs/skills/> (accessed 2026-06-05)
- Amazon Q Developer CLI: `.amazonq/rules/` (IDE context rules) +
  `.amazonq/cli-agents/*.json` (CLI custom agents) — separation correct; does NOT read
  AGENTS.md natively → generated bridge `.amazonq/rules/00-agents-md.md` is the right
  mechanism.
  <https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-custom-agents.html>
  (accessed 2026-06-06)
- OpenAI Codex CLI: reads AGENTS.md natively (directory-walk, `AGENTS.override.md`,
  `project_doc_fallback_filenames`, 32 KiB default cap); `.codex/agents/` per-agent dirs
  are NOT an official convention — official path is `config.toml` `agents.<name>`
  sub-tables (`config_file`, `description`).
  <https://developers.openai.com/codex/guides/agents-md> (accessed 2026-06-06),
  <https://developers.openai.com/codex/config-reference> (accessed 2026-06-06)
- Multi-repo sync prior art: _[Judgment call]_ — no OSS tool does 3-way semantic merge
  of hand-edited governance docs (repo-file-sync-action = overwrite; cruft = .rej-masked
  partials; copier = scaffold-only; symlinks fail for cloud agents). Manual semantic
  3-way merge per file is the justified approach.

### Cross-Plan Facts

- Sibling plan paths (cross-link in every plan README):
  - ose-public: `plans/in-progress/plan-domain-parity/README.md`
  - ose-primer: `plans/in-progress/plan-domain-parity/README.md`
  - ose-infra: `plans/in-progress/plan-domain-parity/README.md`
- Recommended execution order: ose-public plan first (merged canon lands upstream), then
  primer and infra adopt; each plan remains self-contained with its own merge steps
  referencing sibling clone paths.
- Each plan's delivery checklist MUST include: (a) full deviation matrix verbatim in
  tech-docs.md, (b) sibling cross-links, (c) rationale doc
  `docs/explanation/plan-domain-parity-decisions.md`, (d) updates to
  governance/convention docs touched (AGENTS.md catalog text, workflow indexes,
  multi-harness binding docs affected by rows 18–20), (e) own-repo `generate:bindings`
  regeneration + binding audit, (f) Phase 0 (repo-setup-manager) first.

## Survey Corrections (2026-06-06, Plan Authoring)

Pre-write verification against the `plan-domain-parity` worktree (branch
`plan-domain-parity`, clean, parented on primer `main`) found one matrix premise that
needs refinement:

- **Row 21 premise partially outdated.** `apps/rhino-cli-go` ALREADY ships
  `agents sync` and `agents emit-bindings` commands with tests
  (`apps/rhino-cli-go/cmd/agents_sync.go`, `apps/rhino-cli-go/cmd/agents_emit_bindings.go`,
  `apps/rhino-cli-go/internal/agents/` — 19 Go files including `bindings.go`,
  `converter.go`, `sync_validator.go` and their `_test.go` siblings) [Repo-grounded].
  The remaining row-21 work is therefore **porting the row-18/row-19 emitter changes**
  (permission object, Codex layout) to the Go implementation so capability parity holds
  after the Rust emitter changes — not building emission from scratch. The parity-guard
  validation requirement is unchanged.
- All other matrix premises verified against the worktree: `target-stage` absent from
  primer's `plan-establishment-execution.md` (0 grep matches); no grilling convention
  file among the 16 files in `repo-governance/development/workflow/`; `workflows/plan/`
  holds exactly 3 workflows + README; `generate:bindings` uses the nx-build+dist pattern
  (`package.json` line 44); `.codex/agents/` holds exactly one file
  (`ci-monitor-subagent.toml`) referenced from an existing `[agents.ci-monitor-subagent]`
  sub-table in `.codex/config.toml` via `config_file = "agents/ci-monitor-subagent.toml"`
  [Repo-grounded].

Additional grounded facts used by delivery steps:

- `.claude/agents/` holds 50 agent definitions (excluding `README.md`);
  `.opencode/agents/` holds 50 mirrors — no gap; 50:50 parity verified empirically
  against the `plan-domain-parity` worktree on 2026-06-06 [Repo-grounded].
- Rust boolean-tools emission lives in
  `apps/rhino-cli-rust/src/internal/agents/converter.rs` (`convert_tools`, lines ~28–39;
  serializer emission order `description`, `model`, `tools`, `color`, `skills` at
  ~line 183) [Repo-grounded].
- Go boolean-tools emission lives in
  `apps/rhino-cli-go/internal/agents/converter.go` (`ConvertTools`, ~lines 102–113)
  [Repo-grounded].
- Expected binding-dir lists: `apps/rhino-cli-rust/src/internal/agents/bindings.rs`
  line 58 and `apps/rhino-cli-go/internal/agents/bindings.go` line 61, both
  `[".claude", ".opencode", ".codex", ".github", ".amazonq"]` — `.codex` stays valid
  post-migration because `config.toml` remains in `.codex/` [Repo-grounded].
- ose-public's already-landed direct-cargo script family (the row-20 target shape),
  verbatim from `ose-public/package.json` [Repo-grounded]:

  ```json
  "generate:bindings": "cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- agents sync && cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- agents emit-bindings"
  ```

  Primer substitutes `apps/rhino-cli-rust/Cargo.toml` as the manifest path (row 20).

- `planning-system-overhaul` has only archival items unchecked
  (`plans/in-progress/planning-system-overhaul/delivery.md` lines 216–232); all
  substantive delivery items are ticked [Repo-grounded].
- Doc surfaces referencing the affected binding formats: `AGENTS.md` lines 48, 86, 215
  (boolean-flag wording); `CLAUDE.md` line 51; `docs/reference/platform-bindings.md`
  lines 63, 65, 87 (`.codex/agents/` references);
  `repo-governance/conventions/structure/multi-harness-binding.md` (codex references)
  [Repo-grounded].
- Safety Invariant 6 text lives in ose-public's
  `repo-governance/workflows/plan/plan-multi-repo-parity-planning.md` (~line 161): every
  mutation reaching ose-primer must flow through a worktree + branch + draft PR;
  `worktree-to-main` is a documented, invoker-approved deviation [Repo-grounded].

## Design Decisions

1. **Upstream-first adoption with a hard sequencing gate.** Primer does not re-derive
   the merged canon; it adopts ose-public's landed result (sibling clone at
   `/Users/wkf/ose-projects/ose-public`) and re-applies primer-specific divergences.
   Phase 1 opens with a gate verifying the upstream plan's merge commits exist on
   ose-public `main`. Rationale: one canon source eliminates three-way re-merge skew.
2. **Primer-specific divergences preserved during merges** (rows 4, 7, 11): the
   `rhino-cli-rust`/`rhino-cli-go` naming (vs upstream `rhino-cli`), primer
   agent-selection lists in `plan-execution.md`, and `repo-setup-manager`'s 3-line
   primer divergence if repo-specific. Each merge step names its divergences explicitly.
3. **Permission-object shape mirrors the upstream emitter.** OpenCode's official docs
   define `permission` with `allow`/`ask`/`deny` values [Web-cited — see research
   findings]. The exact key set and value mapping (granted tool → `allow`; ungranted →
   omitted or `deny`) is whatever ose-public's landed `apps/rhino-cli/src/internal/agents/converter.rs`
   implements — primer's Rust and Go emitters replicate that shape so `.opencode`
   mirrors are structurally identical across repos. This is deliberately deferred detail,
   resolved at execution time by reading the upstream implementation, not re-decided.
4. **Codex migration is config + docs, code only where referenced.** Neither primer CLI
   emits `.codex/agents/` today [Repo-grounded]; the migration inlines the
   `ci-monitor-subagent` configuration into the `.codex/config.toml`
   `[agents.ci-monitor-subagent]` sub-table per the official config reference (or, where
   a key cannot be inlined, relocates the per-agent TOML out of `.codex/agents/` to a
   path adjacent to `config.toml` and updates `config_file`), deletes `.codex/agents/`,
   and sweeps code/tests/docs for stale references. The expected-binding-dirs lists keep
   `.codex` (the directory itself remains).
5. **Script family alignment, not just `generate:bindings`.** Row 20's justification is
   "uniform invocation"; ose-public's pattern applies the direct-cargo form to the whole
   sync/validate script family [Repo-grounded — public `package.json`]. Primer aligns
   `generate:bindings`, `sync:agents`, `sync:skills`, `sync:dry-run`, `validate:sync`,
   `validate:claude`, and `validate:harness-bindings`. Accepted trade-off (per matrix):
   loss of the nx build-caching wrapper.
6. **Supersession is absorption of archival work only.** The overhaul plan's substantive
   items are all ticked [Repo-grounded]; what this plan absorbs is the close-out:
   supersession pointer, archival `git mv` with completion-date prefix, README index
   updates, orphan-reference sweep.
7. **Direct push to main (row 22).** Worktree `worktrees/plan-domain-parity/` on branch
   `plan-domain-parity`; delivery pushes `HEAD:main` to primer `origin`. The deviation
   from Safety Invariant 6 is recorded three times: matrix row 22 (embedded above),
   README Deviation Notice, and the rationale doc.

## File Impact

### Modified (semantic merge — rows 3–16)

| File                                                             | Row | Divergences to preserve                                    |
| ---------------------------------------------------------------- | --- | ---------------------------------------------------------- |
| `repo-governance/workflows/plan/plan-establishment-execution.md` | 3   | none known; gains `target-stage` + worktree default        |
| `repo-governance/workflows/plan/plan-execution.md`               | 4   | primer agent-selection lists                               |
| `repo-governance/workflows/plan/README.md`                       | 5   | primer link targets                                        |
| `repo-governance/workflows/meta/execution-modes.md`              | 6   | none known                                                 |
| `.claude/agents/plan-maker.md`                                   | 7   | primer repo refs (app/CLI names)                           |
| `.claude/agents/plan-checker.md`                                 | 8   | primer repo refs                                           |
| `.claude/agents/plan-fixer.md`                                   | 9   | primer repo refs                                           |
| `.claude/agents/plan-execution-checker.md`                       | 10  | primer repo refs                                           |
| `.claude/agents/repo-setup-manager.md`                           | 11  | 3-line primer divergence if repo-specific (rhino-cli-rust) |
| `.claude/skills/plan-creating-project-plans/SKILL.md`            | 12  | primer repo refs; gains infra grilling gates               |
| `.claude/skills/plan-writing-gherkin-criteria/SKILL.md`          | 13  | trivial                                                    |
| `.claude/skills/grill-me/SKILL.md`                               | 14  | none known                                                 |
| `repo-governance/conventions/structure/plans.md`                 | 16  | primer-specific examples                                   |

### New Files

| File                                                                | Row    | Source baseline                                                                                     |
| ------------------------------------------------------------------- | ------ | --------------------------------------------------------------------------------------------------- |
| `repo-governance/workflows/plan/plan-multi-repo-parity-planning.md` | 1, 2   | ose-public copy, restructured to the 8-step sequence (the amended version landed upstream first)    |
| `repo-governance/development/workflow/grilling-with-options.md`     | 15     | merge of public `grilling-with-options.md` + infra `grilling.md` broader scope (as landed upstream) |
| `docs/explanation/plan-domain-parity-decisions.md`                  | 22, 24 | _New file_ — authored in this plan                                                                  |

### Modified (code + config — rows 17–21)

| File                                                        | Row    | Change                                                                                      |
| ----------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------- |
| `apps/rhino-cli-rust/src/internal/agents/converter.rs`      | 18     | boolean `tools` map → `permission` object emission                                          |
| `apps/rhino-cli-rust/src/internal/agents/types.rs`          | 18     | OpenCode agent struct: `tools` field → `permission` field                                   |
| `apps/rhino-cli-rust/src/internal/agents/sync_validator.rs` | 18     | mirror validation updated to the new frontmatter shape                                      |
| `apps/rhino-cli-go/internal/agents/converter.go`            | 21     | Go port of the permission-object emission                                                   |
| `apps/rhino-cli-go/internal/agents/types.go`                | 21     | Go struct change mirroring Rust                                                             |
| `apps/rhino-cli-go/internal/agents/sync_validator.go`       | 21     | Go validator parity                                                                         |
| `.opencode/agents/*.md` (50 files)                          | 17, 18 | regenerated with `permission` object; 50:50 parity already present (no gap to reconcile)    |
| `.codex/config.toml`                                        | 19     | `ci-monitor-subagent` config migrated into the `agents.<name>` sub-table                    |
| `.codex/agents/ci-monitor-subagent.toml`                    | 19     | **deleted** (directory removed)                                                             |
| `package.json`                                              | 20     | script family switched to direct `cargo run --manifest-path apps/rhino-cli-rust/Cargo.toml` |

### Modified (docs touched by rows 1–2, 5, 15, 18–20)

`AGENTS.md` (lines 48, 86, 215 binding wording + workflow/convention references),
`CLAUDE.md` (line 51), `docs/reference/platform-bindings.md` (lines 63, 65, 87),
`repo-governance/conventions/structure/multi-harness-binding.md`,
`repo-governance/workflows/README.md`,
`repo-governance/workflows/plan/README.md`,
`repo-governance/development/workflow/README.md`, `docs/explanation/README.md`,
`plans/in-progress/README.md`, `plans/done/README.md`,
`plans/in-progress/planning-system-overhaul/README.md` (supersession pointer, then
archived).

## Dependencies

- **Sequencing**: ose-public `plan-domain-parity` plan must land its merged canon on
  ose-public `main` before Phase 1 here (hard gate).
- **Sibling clones present locally**: `/Users/wkf/ose-projects/ose-public`,
  `/Users/wkf/ose-projects/ose-infra` (read-only references for merge provenance).
- **Toolchain**: Rust (cargo), Go, Node 24 / npm 11 via Volta, Nx — all converged by
  `npm run doctor -- --fix` in Phase 0.
- No new third-party dependencies are introduced.

## Testing Strategy

| Acceptance criterion (prd.md)     | Test level                                                                                                     |
| --------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| Permission-object emission (Rust) | Unit: _New test_ in `converter.rs` tests module; `nx run rhino-cli-rust:test:unit`                             |
| Permission-object emission (Go)   | Unit: _New test_ in `converter_test.go`; `nx run rhino-cli-go:test:unit`                                       |
| Mirror validation under new shape | Unit: updated `sync_validator` tests in both CLIs                                                              |
| Regeneration determinism          | Behavioral: run `npm run generate:bindings` twice; `git status` clean after second run                         |
| Codex layout post-migration       | Behavioral: `test ! -d .codex/agents` + grep sweep; `validate:harness-bindings` exits 0                        |
| Dual-CLI parity                   | Integration: `nx run rhino-cli-{rust,go}:validate:cross-vendor-parity`                                         |
| Governance merges                 | Markdown gates (prettier, markdownlint, validate:mermaid/links/heading equivalents) + plan-quality-gate strict |
| Full binding audit                | `validate:sync`, `validate:harness-bindings`, `validate:config` npm scripts                                    |

Gherkin scenarios in `prd.md` are the source of the first failing tests for the code
phases, per the TDD convention.

## Rollback

All changes are git-revertible markdown/config/code on `main`. If a pushed commit breaks
CI: fix-forward per CI-blocker-resolution; if fix-forward is not viable, `git revert` the
offending commit range (no destructive history rewriting on `main`). The
`.codex/agents/` deletion is restorable from history; binding mirrors are regenerable
from `.claude/` sources at any commit.
