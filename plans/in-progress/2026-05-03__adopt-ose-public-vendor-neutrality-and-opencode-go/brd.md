---
title: BRD — Adopt ose-public Vendor-Neutrality, OpenCode Go, and Companion Tooling
---

# Business Rationale

## Why this plan exists

`ose-primer` is the upstream MIT template that downstream consumers clone or
cherry-pick from when bootstrapping an OSE-style polyglot Nx monorepo. Its
governance, agents, workflows, and rhino-cli together form the _contract_
that template consumers inherit on day one. Between 2026-04-30 and
2026-05-03, `ose-public` (the product monorepo using this same template
shape) shipped a coherent batch of changes that:

1. **Make the Claude Code ↔ OpenCode sync actually work.** The current
   sync writes to `.opencode/agent/` (singular) and `.opencode/skill/`
   (singular). Both paths are non-canonical per the
   [opencode.ai/docs/agents](https://opencode.ai/docs/agents/) and
   [opencode.ai/docs/skills](https://opencode.ai/docs/skills/) spec
   (accessed 2026-05-02 by ose-public). Files written by `npm run sync:claude-to-opencode`
   in the template are likely invisible to OpenCode the moment a
   consumer launches an OpenCode session. The bug is silent, hard to
   detect without spec-reading, and the template propagates the bug
   to every clone. Note: a partial dual-population already exists in
   the working tree — `.opencode/agents/` and `.opencode/skills/` are
   partially present alongside the singular `.opencode/agent/` and
   `.opencode/skill/` directories. W1 must reconcile all four paths
   to the single canonical plural form.
2. **Pick a financially and technically sustainable OpenCode model.**
   ose-primer ships with `zai-coding-plan/glm-5.1` as `model` and
   `zai-coding-plan/glm-5-turbo` as `small_model`. Z.ai's coding plan
   is not the only option, and the OpenCode Go provider
   (opencode.ai/go) routes a curated set of models including
   MiniMax-M2.7 (SWE-Pro 56.22%) and GLM-5 across multiple labs.
   Consumers should inherit the choice the upstream consumer made
   after due diligence rather than the older default.
3. **Operationalize vendor-neutrality.** Today, `ose-primer/governance/`
   reads as if it belongs to one specific AI coding agent (Claude Code)
   with a thin OpenCode mirror — 54 of ~100 governance markdown files
   contain vendor terms (`Claude Code`, `OpenCode`, `Anthropic`,
   `Sonnet`/`Opus`/`Haiku`, `.claude/`, `.opencode/`, capitalized
   `Skills`). This is the _opposite_ of what a template should ship.
   A template tells the consumer "here is how to organize a polyglot
   monorepo so any AI coding agent or human contributor can work in
   it" — the moment governance prose names a vendor in load-bearing
   text, the consumer inherits a coupling they did not ask for.
4. **Provide enforcement, not aspiration.** A vendor-neutrality rule
   without a scanner is governance theatre. ose-public landed
   `rhino-cli governance vendor-audit` (the scanner), the
   `governance-vendor-independence` convention (the rule), and the
   `repo-cross-vendor-parity-quality-gate` workflow with two new
   agents (the enforcement loop). Together these three artefacts
   make the rule mechanically enforceable in pre-push and CI.
5. **Tighten the plans authoring contract.** ose-public's
   `plans.md` was rewritten to make the five-document multi-file
   layout the explicit DEFAULT and to require all four "trivially
   small" criteria be satisfied before single-file is allowed. The
   rewrite closes a drift in which authors collapsed multi-concern
   plans into single READMEs because the existing language read
   like a soft suggestion.

## Cost of the drift

| Drift                                                      | Cost                                                                                                                                                                                                                                                                                                                                                    |
| ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `.opencode/agent/` singular + partial dual-population (W1) | Synced agents likely invisible to OpenCode in every clone of this template; bug propagates silently; opencode-go migration would hard-code wrong path deeper. A partial `.opencode/agents/` and `.opencode/skills/` already exist alongside the singular directories, creating an inconsistent state the executor must reconcile before W1 is complete. |
| `zai-coding-plan/*` model defaults (W2)                    | Template forces a vendor billing decision on every consumer; consumers who don't have a Z.ai subscription must change defaults before their first OpenCode session.                                                                                                                                                                                     |
| Vendor terms in `governance/` (W4)                         | Template excludes contributors using Cursor / Codex CLI / Gemini CLI / Aider / Copilot from day one; couples template correctness to one vendor's product lifecycle; creates rewrite debt every time a vendor renames.                                                                                                                                  |
| No vendor-audit scanner (W3)                               | New violations land in `governance/` with no signal; pre-push gate is silent on the most expensive long-term coupling.                                                                                                                                                                                                                                  |
| No cross-vendor parity gate (W5)                           | The five behavioral-parity invariants (sync no-op, count parity, color-map coverage, tier-map coverage, Aider catalog accuracy) regress silently between releases; consumers inherit the regression.                                                                                                                                                    |
| Permissive single-file plans default (W6)                  | Multi-concern plans collapse to single READMEs; BRD/PRD content is lost or buried; plan-checker downstream loses signal because the structure is non-standard.                                                                                                                                                                                          |
| Missing worktree-path convention (W7)                      | Template ships no rule for where worktrees land; consumers reinvent it per repo; existing `worktree-setup.md` is stale relative to ose-public's current toolchain-init and parallel-safety language.                                                                                                                                                    |
| Stale plan workflows + missing companions (W8)             | `plan-execution.md` is ~76 lines behind ose-public's current iteration loop; consumers inheriting the stale workflow miss recent termination-rule and Iron-Rules clarifications. `ci-monitoring.md`, `ci-post-push-verification.md` are missing entirely.                                                                                               |
| TDD convention missing (W9)                                | `governance/development/workflow/test-driven-development.md` exists in ose-public (316 lines, mandates Red→Green→Refactor) but never reached primer; consumers inherit a template that pays lip-service to TDD via `implementation.md` without an authoritative convention codifying the practice.                                                      |

## Why this plan adds W7 (worktree), W8 (plan + workflow), and W9 (TDD)

The original five workstreams were extended after a re-scan of the
ose-public ↔ ose-primer delta. Three additional categories of generic
template scaffolding were found behind the upstream:

1. **Worktree standard (W7).** ose-public introduced
   `governance/conventions/structure/worktree-path.md` — an explicit,
   217-line convention pinning the on-disk location for worktree
   branches and the rationale for the override (root `worktrees/<name>/`
   for `ose-public`, `.claude/worktrees/<name>/` for primer/infra). The
   primer never adopted it. The primer-side adaptation flips the
   override-vs-default polarity: ose-public _overrides_ the default,
   primer _follows_ the default `.claude/worktrees/<name>/`. Either way,
   the rule is documented authoritatively at the convention layer
   instead of buried in CLAUDE.md prose. ose-public also iterated
   `governance/development/workflow/worktree-setup.md` after the primer
   forked it; the primer's copy is stale on minor frontmatter and
   cross-reference targets.
2. **Plan and workflow refresh (W8).** The primer's
   `governance/workflows/plan/` tree (plan-execution.md,
   plan-quality-gate.md, README.md) all diverge from ose-public's
   current versions. ose-public also added two companion development
   workflows the primer never saw — `ci-monitoring.md` and
   `ci-post-push-verification.md`. These are template-grade
   scaffolding, not product content, and `ose-primer-sync` classifies
   `governance/workflows/**` and `governance/development/**` as
   `bidirectional identity`.
3. **TDD convention (W9).** ose-public ships
   `governance/development/workflow/test-driven-development.md` (316
   lines) which mandates Red→Green→Refactor as the required practice
   for all code changes, cross-references the three-level testing
   standard, and codifies the Gherkin-scenario → failing-step →
   passing-implementation chain that drives every rhino-cli unit and
   integration test today. The primer never adopted the convention,
   so consumers inherit a template that practices TDD informally
   (delivery checklists are TDD-shaped) but never codifies it as a
   rule that the `plan-checker` agent or the maintainer can cite.

## Affected roles

- **Template consumer (clone-and-customize)**: gets a working OpenCode
  setup on first session, vendor-neutral governance to extend, an
  enforcement scanner and parity gate that survives their first
  feature plan.
- **Template consumer (cherry-pick-and-merge)**: gets atomic, file-level
  porting maps in this plan's `tech-docs.md` so they can grab one
  workstream at a time.
- **ose-primer maintainer (this plan's executor)**: works through a
  granular phased delivery checklist; each phase ends in a known-good
  state (tests green, scanner clean, sync no-op).
- **Future contributor on a non-Claude AI agent (Cursor, Codex CLI,
  Gemini CLI, Aider, Copilot, Continue, Sourcegraph Cody)**: reads
  `governance/` as vendor-neutral prose; inherits a behavioral-parity
  guarantee at clone time.
- **Plan agents** (`plan-maker`, `plan-checker`, `plan-execution-checker`):
  inherit the stricter five-doc DEFAULT, judge new plans against
  four explicit single-file criteria.

## Success metrics (observable after execution)

- `nx run rhino-cli:test:unit` and `nx run rhino-cli:test:integration` both green (≥90% coverage holds).
- `npm run sync:claude-to-opencode` is a no-op on a clean tree.
- `ls .opencode/agent` returns "no such file"; `ls .opencode/agents` returns the synced agent set.
- `ls .opencode/skill` returns "no such file"; `ls .opencode/skills` returns the synced skill set
  (or `.opencode/skills` empty if the post-W1 decision is "OpenCode reads `.claude/skills/` natively, no copy needed").
- `cat .opencode/opencode.json | jq -r .model` returns `opencode-go/minimax-m2.7`; `.small_model`
  returns `opencode-go/glm-5`; `.provider["opencode-go"].options.apiKey` resolves via env var.
- `rhino-cli governance vendor-audit governance/` returns 0 violations.
- `rhino-cli governance vendor-audit AGENTS.md CLAUDE.md` returns 0 violations
  (with binding-example fences allowlisted).
- `nx run rhino-cli:validate:cross-vendor-parity` returns 0 findings on two consecutive runs.
- Pre-push hook runs both new Nx targets without exceeding existing time budget.
- `governance/conventions/structure/plans.md` opens with five-doc DEFAULT prose and
  enumerates four explicit single-file exception criteria.

## Risks and mitigations

- **R1 — opencode-go subscription not yet provisioned by every consumer.** Template
  ships defaults consumers may not be authenticated for.
  _Mitigation_: opencode.json keeps the env-var-resolved provider block
  (`{env:OPENCODE_GO_API_KEY}`) so an unauthenticated session fails fast with a
  clear error rather than running silently against the wrong provider. Document
  the env var in `.env.example` and the AGENTS.md tech-stack section.
- **R2 — vendor-audit scanner surfaces too many violations to remediate in one pass.**
  W4 remediation could become a long tail.
  _Mitigation_: the scanner has an allowlist mechanism (binding-example fences,
  `forbiddenConvention` self-exemption); start with a baseline audit, classify findings
  by file, batch remediation by directory in delivery.md. Coverage threshold (≥90%) is
  not affected.
- **R3 — `.opencode/skill/` to `.opencode/skills/` migration creates a "should we even
  copy skills?" decision.** OpenCode reads `.claude/skills/` natively per
  [opencode.ai/docs/skills](https://opencode.ai/docs/skills/), so the copy may be redundant.
  _Mitigation_: defer the "stop copying skills" call to a follow-on plan; W1 only
  fixes the path (singular → plural) so behavior is identical except canonical.
- **R4 — Vendor-neutralizing AGENTS.md mid-template breaks existing consumer instructions.**
  AGENTS.md changes how OpenCode/Codex CLI/Aider read the repo.
  _Mitigation_: AGENTS.md becomes the canonical root file; CLAUDE.md becomes a thin
  shim importing `@AGENTS.md` (existing pattern in ose-public). Vendor-specific
  examples remain visible inside `binding-example` fences. No instruction is lost,
  only relocated and labelled.
- **R5 — Two new pre-push Nx targets balloon the time budget.** Pre-push already runs
  typecheck + lint + test:quick + spec-coverage for affected projects.
  _Mitigation_: both new targets are cacheable (input-keyed); first push pays the cache-warm
  cost, subsequent pushes are sub-second. If wall-clock pain materializes, gate one of the
  two behind a "pre-push:strict" alias and keep the default lighter.

## Non-goals

- Migrating `.claude/agents/*.md` content. The Claude Code aliases
  (`sonnet`, `haiku`, omit) are not changing.
- Renaming `.claude/skills/` or `.opencode/agents/` paths beyond the
  singular → plural canonicalization required by W1.
- Touching `apps/` business code (the polyglot CRUD demo apps) — none
  of these workstreams require changes there.
- Adding new platform bindings (`.cursor/`, `.continue/`, `.github/copilot-instructions.md`).
  Tracked as future work.
- Adopting DDD enforcement (`bc validate`, `ul validate`) — the registry
  schema is product-specific, not template scaffolding.

## Decision log inputs

- Five workstreams (W1–W5) come from five distinct ose-public plans archived between
  2026-04-30 and 2026-05-03; W6 comes from a 2026-04-18 plans-convention rewrite that
  primer never adopted.
- The execution order (W1 → {W2, W3} → W4 → W5; W6 parallel) is determined by data-flow
  dependency:
  - W1 establishes canonical sync output paths so W2's regenerated agents land in the right place.
  - W3 ships the scanner; W4 needs that scanner to mechanically validate remediation.
  - W5 wraps W4 invariants in an iterative quality gate; the gate validates state W4 produces.
  - W6 is independent — it changes prose in `plans.md` only, no code dependency.
