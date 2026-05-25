---
title: "Multi-Harness Binding Convention"
description: Strategy for keeping the repository compatible with many AI coding-agent harnesses while keeping governance vendor-neutral.
category: explanation
subcategory: conventions
tags:
  - conventions
  - governance
  - platform-bindings
  - vendor-independence
  - multi-harness
---

# Multi-Harness Binding Convention

This convention defines how the repository stays compatible with multiple AI coding-agent harnesses simultaneously without coupling governance content to any single vendor's product lifecycle. It documents the architecture decisions that govern how binding files are created, maintained, and kept in sync with the canonical instruction surface.

## Principles Implemented/Respected

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: One canonical instruction surface (`AGENTS.md`) eliminates the need to maintain parallel, redundant instruction sets. Every binding is either absent (the harness reads `AGENTS.md` natively) or a thin pointer that delegates to `AGENTS.md`.
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Binding tier classification, the no-shadowing rule, and generation provenance are all stated directly in this convention and in `docs/reference/platform-bindings.md`. No binding file exists without an explicit record.
- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**: Binding files that must exist are generated from `AGENTS.md` by the CLI, not hand-written. This prevents the silent drift that arises when humans maintain two copies of the same content.
- **[Reproducibility First](../../principles/software-engineering/reproducibility.md)**: The mechanical generation rule means any binding file can be deleted and regenerated at any time, producing a byte-identical result. No binding file encodes knowledge that does not already live in `AGENTS.md`.

## Purpose

The repository is designed to be usable from any AI coding-agent harness a contributor chooses. The majority of current harnesses read the root `AGENTS.md` file natively. A minority require an explicit bridge. Without a clear convention, teams tend to add binding files reactively and inconsistently, which leads to:

- Instruction content fragmented across multiple files that drift out of sync with each other.
- Higher-precedence harness files silently shadowing `AGENTS.md`, so contributors using that harness see different rules than everyone else.
- Binding files maintained by hand that inevitably fall behind `AGENTS.md` changes.

This convention resolves all three problems with four architecture decisions applied consistently across every harness.

## Scope

### What This Convention Covers

- The canonical status of `AGENTS.md` as the single instruction surface (AD1).
- The two-tier classification of harnesses by whether they read `AGENTS.md` natively (AD2).
- The no-shadowing rule prohibiting higher-precedence files with divergent content (AD3).
- The mechanical-generation requirement for any binding file that must exist (AD4).
- The dual-implementation parity requirement for the CLI commands that generate and validate bindings (AD8).
- The pre-push deterministic guard that checks binding files before each push.

> **On the decision numbering**: this convention documents the binding-strategy decisions AD1, AD2, AD3, AD4, and AD8. The intermediate decisions from the source design live in their natural homes rather than here: AD5 (reconciling pre-existing tool-provided bindings) is recorded in the [platform-bindings catalog](../../../docs/reference/platform-bindings.md); AD6 (the compatibility-audit workflow) is the [`repo-harness-compatibility-quality-gate`](../../workflows/repo/repo-harness-compatibility-quality-gate.md) workflow itself; and AD7 (the deterministic pre-push guard) is implemented as the `validate:harness-bindings` npm script wired into `.husky/pre-push`. The numbering is preserved for traceability to the originating plan.

### What This Convention Does NOT Cover

- The full catalog of individual harness details and their binding paths — see [`docs/reference/platform-bindings.md`](../../../docs/reference/platform-bindings.md).
- The vendor-neutrality scanning rules for `repo-governance/` prose — see [`governance-vendor-independence.md`](./governance-vendor-independence.md).
- Workflow naming rules for the compatibility-audit workflow — see [`workflow-naming.md`](./workflow-naming.md).
- Individual agent definition format — see `repo-governance/development/agents/ai-agents.md`.

## Standards

### AD1 — `AGENTS.md` Is the Single Canonical Instruction Surface

`AGENTS.md` at the repository root is the only location where instruction content lives. Every harness-specific file is either:

- **Unnecessary**: the harness reads `AGENTS.md` natively, so no binding file is needed.
- **A thin pointer**: the harness does not read `AGENTS.md` natively and an explicit bridge is required, but that bridge's body consists solely of a reference or import of `AGENTS.md` — it adds no independent content of its own.

The existing shim in the primary binding's root instruction file — which reduces to a single import directive pointing at `AGENTS.md` — is the canonical example of a thin pointer. All future bridges follow the same pattern.

**Rationale**: Duplication of instruction content between `AGENTS.md` and binding files is the root cause of drift. Eliminating duplication at the source is less fragile than any sync mechanism.

### AD2 — Two Binding Tiers

All coding-agent harnesses fall into one of two tiers:

**Tier 1 — native `AGENTS.md` readers**

Harnesses in this tier read `AGENTS.md` from the repository root without any additional configuration. The default action for a Tier 1 harness is to add nothing beyond documenting its native-reader status in the platform-bindings catalog (`docs/reference/platform-bindings.md`). A thin pointer file may be added when it materially improves harness-specific discoverability, but only if it cannot shadow `AGENTS.md` (see AD3).

**Tier 2 — non-readers**

Harnesses in this tier do not read `AGENTS.md` natively. Each Tier 2 harness requires an explicit committed bridge: a generated file in the harness's native configuration directory whose body points to `AGENTS.md` and whose content is derived entirely from `AGENTS.md` by the CLI. No hand-written bridge files are permitted (see AD4).

**Default posture**: when a new harness is evaluated, it is Tier 1 unless web research confirms it does not read `AGENTS.md` natively. Tier 2 classification requires a cited source recorded in the platform-bindings catalog.

**Tier changes**: when a Tier 2 harness ships native `AGENTS.md` support, the generated bridge file is deleted and the harness is re-classified to Tier 1. The platform-bindings catalog records the transition date and citation.

### AD3 — No-Shadowing Rule (Hard)

Some harnesses rank a tool-specific file above `AGENTS.md` in their instruction-precedence order. If such a file exists in the repository with content that differs from `AGENTS.md`, contributors using that harness see a silently divergent instruction set. This is a hard rule with no exceptions:

**The repository must not commit any higher-precedence harness file with content that differs from `AGENTS.md`.**

The three known higher-precedence filename forms are documented in the Platform Binding Examples section below. The default position for all of them is to not create them at all — native `AGENTS.md` readers already apply `AGENTS.md` regardless of whether those files exist. If a future operational need forces one of these files to exist, it must be a pure pointer that imports `AGENTS.md` and adds no independent content.

**Enforcement**: `rhino-cli agents validate-bindings` asserts that no tracked higher-precedence file diverges from `AGENTS.md`. It runs as part of the pre-push hook (see `validate:harness-bindings` in `package.json`) and as a CI quality-gate step.

### AD4 — Mechanical Generation Over Hand-Maintenance

Any binding file that must exist (Tier 2 bridges, any thin pointer) is generated by `rhino-cli agents emit-bindings` from `AGENTS.md`. The command derives the expected content in memory and writes it to the target path. The `validate-bindings` subcommand re-derives the content and asserts byte-equality with the committed file.

Hand-writing or hand-editing generated binding files is prohibited. Changes to the instruction content belong in `AGENTS.md`. The CLI propagates them to binding files on the next generation run. This extends the existing generator model where the CLI's `agents sync` command produces the secondary binding directory from the primary binding directory.

**Pre-push guard**: `npm run validate:harness-bindings` wraps `rhino-cli agents validate-bindings` and fires in the pre-push hook when any binding surface changes. It exits non-zero if any generated binding file diverges from what `emit-bindings` would produce, or if any binding directory present on disk lacks a row in the platform-bindings catalog.

### AD8 — Dual-Implementation Byte-Parity (ose-primer-Specific)

This repository maintains two co-equal CLI implementations: one in Rust and one in Go. A shadow-diff parity harness asserts byte-identical stdout, stderr, and exit codes for every command in the corpus. Any binding-emitter or binding-validation behavior implemented in one CLI must be implemented identically in the other.

Consequences:

- `agents emit-bindings` and `agents validate-bindings` are implemented in both CLI codebases.
- The shadow-diff corpus includes `emit-bindings --dry-run` cases and `validate-bindings` cases (clean and drifted fixtures).
- Each change to the Rust implementation is paired with the identical change in the Go implementation in the same delivery phase; the shadow-diff gate must pass before push.

This requirement exists because both CLIs are published from `ose-primer` as reference implementations. A behavioral divergence between them would undermine their use as a trustworthy pair.

**Generator-logic vs regenerated data**: When a harness convention change requires updating the CLI, the type of change determines who handles it. Two categories exist:

- **Regenerated data** — the translation tables or catalog content that the CLI reads at runtime change, but the code that reads and applies them does not. The harness-compatibility fixer handles this automatically by updating the catalog and re-running `npm run sync:claude-to-opencode`.
- **Generator-logic change** — a translation rule itself changes (a new field mapping, a new output format rule, a new validation predicate). This is a code change and must land identically in both `apps/rhino-cli-go/internal/agents/` and `apps/rhino-cli-rust/src/` within the same delivery. The harness-compatibility fixer does not make code changes; it surfaces the requirement as a coupled both-CLI finding for human or language-dev-agent authorship. The shadow-diff gate must pass before push.

## Validation

The following commands verify compliance with this convention:

```bash
# Check that all generated binding files match what emit-bindings would produce
npm run validate:harness-bindings

# Check that repo-governance/ prose contains no vendor terms outside allowlisted regions
npx nx run rhino-cli-rust:validate:repo-governance-vendor-audit
```

Both commands run automatically in `.husky/pre-push` when the relevant surfaces change and in the CI quality gate.

## Platform Binding Examples

This section names concrete vendor products, harness-specific filenames, and binding directory paths. The vendor-audit scanner skips everything in this section. See [`docs/reference/platform-bindings.md`](../../../docs/reference/platform-bindings.md) for the full catalog with per-harness status and citations.

### Tier 1 — Native AGENTS.md Readers

The following harnesses read `AGENTS.md` from the repository root without any additional bridge file:

- **OpenCode** — reads `AGENTS.md` natively; agents live in `.opencode/agents/` (auto-generated by `rhino-cli agents sync` from `.claude/agents/`).
- **OpenAI Codex CLI** — reads `AGENTS.md` natively (since April 2025); override file `AGENTS.override.md` ranks higher than `AGENTS.md` (no-shadowing rule applies — do not create this file with divergent content).
- **GitHub Copilot** — reads `AGENTS.md` natively; supplemental instructions may live in `.github/copilot-instructions.md`.
- **Cursor** — reads `AGENTS.md` natively; additional rules may live in `.cursor/rules/*.mdc`.
- **Windsurf** — reads `AGENTS.md` natively; additional rules may live in `.windsurf/rules/*.md`.
- **JetBrains Junie** — reads `AGENTS.md` natively; `.junie/AGENTS.md` ranks higher than root `AGENTS.md` (no-shadowing rule applies — do not create `.junie/AGENTS.md` with divergent content).
- **Google Antigravity CLI** — reads `AGENTS.md` natively (since v1.20.3); `GEMINI.md` ranks higher than `AGENTS.md` (no-shadowing rule applies — do not create `GEMINI.md` with divergent content).
- **Pi (pi.dev)** — reads `AGENTS.md` natively; also reads `CLAUDE.md`.

### Tier 2 — Non-Readers (Bridge Required)

The following harnesses do not read `AGENTS.md` natively and require an explicit generated bridge:

- **Claude Code** — reads `CLAUDE.md` as its primary instruction file. The existing `CLAUDE.md` shim (`@AGENTS.md` import) is the thin pointer for this harness.
- **Amazon Q Developer** — reads `.amazonq/rules/*.md` files via agent JSON `resources` field. The generated bridge is `.amazonq/rules/00-agents-md.md`, whose body points to `AGENTS.md`.

### Higher-Precedence Filenames (No-Shadowing Rule)

The three known filename forms that rank above `AGENTS.md` for their respective harnesses:

```binding-example
# Codex CLI: AGENTS.override.md > AGENTS.md
# JetBrains Junie: .junie/AGENTS.md > root AGENTS.md
# Google Antigravity CLI: GEMINI.md > AGENTS.md

# Default position: do not create any of these files.
# If forced to exist, they must be pure pointers:

# AGENTS.override.md (Codex CLI bridge, if ever needed)
@AGENTS.md

# .junie/AGENTS.md (Junie bridge, if ever needed)
@../AGENTS.md

# GEMINI.md (Antigravity bridge, if ever needed)
@AGENTS.md
```

### Thin-Pointer Pattern (Primary Binding Example)

```binding-example
# CLAUDE.md — the canonical thin-pointer pattern
# Entire file body is one import directive:
@AGENTS.md
```

## Quality Gates: Distinct Scopes

Two automated quality gates enforce different invariants for the binding surfaces. They are complementary and non-overlapping — running one does not substitute for running the other.

| Gate                             | Workflow                                  | Scope                                                                                                                                                          | Mechanism                                                 | When to run                                                                         |
| -------------------------------- | ----------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| **Deterministic pre-push guard** | `npm run validate:harness-bindings`       | **Internal byte-drift** — re-derives expected binding file content from `AGENTS.md` and asserts byte-equality with committed files                             | Offline, agent-free, `rhino-cli agents validate-bindings` | Pre-push hook; CI                                                                   |
| **Harness compatibility gate**   | `repo-harness-compatibility-quality-gate` | **External convention drift** — checks whether each harness's current upstream documentation matches the platform-bindings catalog and committed binding files | Web-research-backed, agent-driven                         | Monthly cadence; after a harness ships a major release; before adding a new harness |

The pre-push guard is fast and deterministic: it catches cases where `AGENTS.md` changed but the generated binding files were not regenerated. The compatibility gate is comprehensive and current: it catches cases where an upstream harness changed its conventions without any corresponding local change. Neither gate can substitute for the other.

A third gate — **cross-vendor parity** (`repo-cross-vendor-parity-quality-gate`) — validates that the primary and secondary binding directories agree with each other and with governance prose. It operates entirely offline on locally committed files (no web research) and detects internal agreement rather than external convention drift. All three gates chain but do not overlap.

## Tools and Automation

- **`repo-harness-compatibility-checker`** — Checker agent; delegates to web research, diffs current upstream harness conventions against the platform-bindings catalog and committed binding files. Run via the `repo-harness-compatibility-quality-gate` workflow.
- **`repo-harness-compatibility-fixer`** — Fixer agent; applies validated updates to the catalog and binding files after a checker audit. Also updates `specs/apps/rhino/` when a harness convention change alters rhino-cli behavior that those specs document.
- **`rhino-cli agents emit-bindings`** — Generates all Tier 2 bridge files and any thin pointers from `AGENTS.md`. Implemented in both the Rust and Go CLI implementations.
- **`rhino-cli agents validate-bindings`** — Asserts byte-equality between committed binding files and what `emit-bindings` would produce; also asserts that every binding directory on disk has a row in `docs/reference/platform-bindings.md`.
- **`npm run validate:harness-bindings`** — npm script wrapping `rhino-cli agents validate-bindings`; wired into `.husky/pre-push`.

## Related Conventions

- **[Governance Vendor-Independence Convention](./governance-vendor-independence.md)** — Defines which vendor terms are forbidden in `repo-governance/` prose and the allowlist mechanism that covers this file's Platform Binding Examples section.
- **[Workflow Naming Convention](./workflow-naming.md)** — Governs the filename of the `repo-harness-compatibility-quality-gate` workflow that uses the checker and fixer agents from this convention.
- **[Platform Bindings Catalog](../../../docs/reference/platform-bindings.md)** — Full per-harness catalog: binding paths, native-reader status, MCP config paths, agent skill paths, and current repo status for all supported harnesses.

## Conventions Implemented/Respected

- **[File Naming Convention](./file-naming.md)**: This file uses kebab-case (`multi-harness-binding.md`).
- **[Linking Convention](../formatting/linking.md)**: All cross-references use GitHub-compatible relative `.md` links.
- **[Content Quality Principles](../writing/quality.md)**: Active voice, single H1, proper heading hierarchy.
- **[Governance Vendor-Independence Convention](./governance-vendor-independence.md)**: All vendor product names, binding directory paths, and harness-specific filenames appear only in the Platform Binding Examples section above.
