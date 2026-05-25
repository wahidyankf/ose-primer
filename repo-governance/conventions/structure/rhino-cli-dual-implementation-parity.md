---
title: "Rhino CLI Dual-Implementation Parity Convention"
description: Defines the model, roles, parity enforcement harness, and contributor rules for maintaining two byte-identical rhino-cli implementations (Rust and Go) from a single behavior contract.
category: explanation
subcategory: conventions
tags:
  - conventions
  - rhino-cli
  - parity
  - dual-implementation
  - rust
  - go
  - bdd
---

# Rhino CLI Dual-Implementation Parity Convention

The `rhino-cli` (Repository Hygiene & INtegration Orchestrator) ships as two byte-identical implementations — Rust and Go — both driven by a single Gherkin behavior contract. This convention defines the model, the roles of each implementation, how parity is enforced automatically, and the contributor obligations that keep the two implementations in lockstep.

## Principles Implemented/Respected

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: The dual-implementation relationship and parity rules are stated here rather than left as tribal knowledge. A contributor arriving at either `apps/rhino-cli-rust/` or `apps/rhino-cli-go/` can read this document and understand their obligations without asking.
- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**: The shadow-diff harness and the permanent `parity` CI job enforce byte-level output identity automatically on every pull request. Manual eyeballing of diffs is not the primary gate.
- **[Reproducibility First](../../principles/software-engineering/reproducibility.md)**: A single behavior contract (`specs/apps/rhino/behavior/cli/gherkin/`) is the canonical source of truth. Both implementations reproduce the same observable behavior deterministically.
- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: One spec, two implementations, one diff script. The model is deliberately minimal: no shared runtime, no code generation, no protocol buffers — just independent reimplementations converging on the same output.

## The Model

```
specs/apps/rhino/behavior/cli/gherkin/   ← single behavior contract
        │
        ├── apps/rhino-cli-rust/         ← canonical CLI (Rust)
        └── apps/rhino-cli-go/           ← parity twin (Go)
```

The behavior contract lives in `specs/apps/rhino/` and is independent of any implementation language. Both implementations must satisfy all scenarios in that contract. Neither implementation owns the contract; the contract owns both implementations.

This mirrors the multi-language parity model used by the `crud-be-*` family, where `apps/crud-be-e2e/` provides a shared end-to-end test suite that runs against every backend implementation. The rhino-cli equivalent of that shared suite is the shadow-diff harness described below, augmented by per-implementation spec-coverage gates.

For the BDD (Behavior-Driven Development) mapping that governs how Gherkin scenarios connect to implementation tests, see [BDD Spec-Test Mapping](../../development/infra/bdd-spec-test-mapping.md).

## Roles

### Rust — canonical CLI

`apps/rhino-cli-rust/` is the implementation that CI and the developer toolchain invoke:

- All `package.json` scripts that shell out to `rhino` call the Rust binary.
- Husky hooks invoke the Rust binary.
- All `~23` dependent projects whose `test:quick` and `spec-coverage` targets run `rhino` use the Rust binary.

The Rust implementation uses [clap](https://docs.rs/clap) for argument parsing.

### Go — parity twin

`apps/rhino-cli-go/` is retained permanently as a behaviorally-identical twin. It is not a prototype, not a legacy artifact, and not scheduled for removal. Its role is to prove, on every CI run, that the behavior contract is expressible in at least two independent implementation languages without drift.

The Go implementation uses [cobra](https://cobra.dev) for argument parsing.

Both implementations must build, pass all unit and integration tests, and pass spec-coverage validation on every pull request.

## Parity Enforcement

### Shadow-diff harness

`apps/rhino-cli-rust/scripts/shadow-diff.sh` is the primary parity tool. It:

1. Builds both binaries from source.
2. Runs every supported command — across all `--output` formats — against both binaries with identical inputs.
3. Diffs stdout, stderr, and exit codes.
4. Masks only inherently non-deterministic JSON fields (`timestamp` and `duration_ms`) before diffing. All other fields are compared byte-for-byte.

Any stdout, stderr, or exit-code divergence fails the script with a non-zero exit code.

### CI integration

The shadow-diff harness runs as the permanent `parity` job in `.github/workflows/pr-quality-gate.yml`. It is not optional, not skippable, and not conditional on file paths changed. Every pull request that touches any file in `apps/rhino-cli-rust/`, `apps/rhino-cli-go/`, or `specs/apps/rhino/` must pass the `parity` job before merge.

## Contributor Rules

**Rule 1 — Both implementations must land together.**
Any behavior change — new command, changed flag, modified output format, updated exit code — must be implemented in both `apps/rhino-cli-rust/` and `apps/rhino-cli-go/` within the same pull request. A PR that changes one implementation without the other will fail the `parity` CI job and must not be merged.

This rule applies when a harness convention change requires updating generator logic. When the `repo-harness-compatibility-quality-gate` workflow runs, two categories of CLI updates can arise:

- **Regenerated data** (catalog content, runtime-read tables): the harness-compatibility fixer handles this automatically — no code change required, no dual-implementation obligation.
- **Generator-logic change** (a translation rule in `apps/rhino-cli-go/internal/agents/` or `apps/rhino-cli-rust/src/`): this is a code change and falls under Rule 1. The fixer surfaces it as a coupled both-CLI finding rather than applying it automatically. The identical change must land in both implementations in the same delivery.

**Rule 2 — The spec is the source of truth.**
Behavior is defined in `specs/apps/rhino/behavior/cli/gherkin/`. When the desired behavior changes, update the spec first, then update both implementations to match. Do not add behavior to an implementation that is not expressed in the spec.

**Rule 3 — Spec-coverage must pass for both.**
Each implementation has its own spec-coverage target. Both must pass. A green Rust spec-coverage result does not substitute for a Go spec-coverage result.

**Rule 4 — The parity gate is the arbiter.**
When the shadow-diff harness reports a diff, that is a bug, not a known divergence. Fix the diverging implementation; do not modify the harness to ignore the diff unless a formal accepted-divergence entry is added to this document (see below).

## Accepted Divergences

The following output differences are acknowledged and excluded from the parity gate by design.

### Root `--help` chrome

`rhino --help` (with no subcommand) produces different chrome — header text, section ordering, spacing — when rendered by clap (Rust) versus cobra (Go). This is a rendering artifact of the argument-parsing library, not a functional difference. All subcommand outputs and all `--output json` / `--output text` formatted outputs are byte-identical. The shadow-diff harness excludes root `--help` from its comparison set.

No other divergences are accepted at this time. Any new proposed divergence requires an entry in this section before the harness exclusion is added.

## Template Reuse Note

`ose-primer` is an MIT-licensed repository template (see [Repository Ecosystem Convention](./repository-ecosystem.md)). This dual-implementation pattern — single behavior contract, multiple independent implementations, shadow-diff parity gate — is deliberately designed to be reusable by downstream forks that want to maintain a CLI in more than one language. The shadow-diff harness, the Gherkin contract layout under `specs/apps/<app-name>/behavior/cli/gherkin/`, and this convention document are all MIT-licensed and may be adapted without restriction.

## Related

- [Specs Directory Structure](./specs-directory-structure.md) — Canonical layout for the Gherkin behavior contract at `specs/apps/rhino/`.
- [BDD Spec-Test Mapping](../../development/infra/bdd-spec-test-mapping.md) — How Gherkin scenarios connect to implementation-level tests.
- [Repository Ecosystem Convention](./repository-ecosystem.md) — Sibling repo relationships and the ose-primer template role.
- [File Naming Convention](./file-naming.md) — Kebab-case naming applies to all files in both implementation directories.
- [Repository Governance Architecture](../../repository-governance-architecture.md) — Six-layer governance hierarchy that this convention sits within (Layer 2: Conventions).
- [Harness Compatibility Quality Gate](../../workflows/repo/repo-harness-compatibility-quality-gate.md) — Workflow that triggers generator-logic changes (Rule 1 extension: coupled both-CLI findings surface here).

## Conventions Implemented/Respected

- **[File Naming Convention](./file-naming.md)**: This file uses kebab-case.
- **[Governance Vendor Independence](./governance-vendor-independence.md)**: This file contains no forbidden vendor terms in load-bearing prose.
- **[Content Quality Principles](../writing/quality.md)**: Active voice, proper heading hierarchy, single H1.
