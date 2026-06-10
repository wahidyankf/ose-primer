# Migrate rhino-cli (Rust) to Hexagonal Architecture

> **Status**: In Progress (planning artifact — NOT yet executed)
> **Identifier**: `migrate-rhino-cli-to-hexagonal`
> **Stage**: `in-progress`
> **Worktree**: `worktrees/migrate-rhino-cli-to-hexagonal/`

## Context

The CLI application `apps/rhino-cli-rust` (Rust, clap, ~19,410 LOC non-test)
[Repo-grounded] implements 13 features. Today the app mixes domain logic, IO, and
CLI wiring inside flat `internal/<feature>` and `commands/` packages. The `git`
feature is already the partial exemplar: it injects all IO through a `Deps`
struct of function fields [Repo-grounded —
`apps/rhino-cli-rust/src/internal/git/runner.rs`].

This plan migrates the app to **hexagonal (ports-and-adapters) architecture**
in a phased, behavior-preserving fashion, formalizing the existing `git`
dependency-injection pattern into named ports across every feature.

## Scope

**In scope**:

- Full migration of all 13 features in `apps/rhino-cli-rust` to a hybrid
  `domain/shared/` kernel + per-feature vertical-slice layout.
- Maximal port extraction: every IO boundary (filesystem, process/exec spawn,
  network) becomes a named port (Rust `Box<dyn Trait>`).
- A purely structural refactor: the output surface is **frozen** (zero visible
  change), verified by the golden-master CLI suite against the Phase 0 baseline
  throughout.
- Updating the convention document
  `repo-governance/development/pattern/hexagonal-architecture-cli.md`
  [Repo-grounded].

**Out of scope**:

- Adding a new architecture/import-direction lint (enforcement via language
  tooling only — Rust module privacy + clippy).
- Any change to golden CLI output (the output surface is frozen — no bytes change
  during the migration; the corpus is never re-baselined).
- Performance optimization, new features, dependency upgrades unrelated to
  layering.

## Document Map

| File                             | Purpose                                                                          |
| -------------------------------- | -------------------------------------------------------------------------------- |
| [`brd.md`](./brd.md)             | WHY — business goal, impact, risks, success metrics                              |
| [`prd.md`](./prd.md)             | WHAT — personas, user stories, Gherkin acceptance criteria, product scope        |
| [`tech-docs.md`](./tech-docs.md) | HOW — layout, port mechanism, the maximal-vs-lean trade-off, migration recipe    |
| [`delivery.md`](./delivery.md)   | DO — phased checklist (Phase 0 baseline → git pilot → features → convention doc) |

## Approach Summary

1. **Phase 0** — establish a green baseline on the app (build, unit,
   integration, coverage ≥90%).
2. **Phase 1** — PILOT: migrate the `git` feature as the proof gate, formalizing
   its `Deps` into named ports.
3. **Phases 2–N** — migrate the shared kernels (`mermaid`; `cliout`) early,
   then the remaining features, grouping by IO-heaviness and dependency order.
4. **Final phase** — update the hexagonal-CLI convention doc (vendor-neutral).

Every feature phase runs the golden-master CLI suite GREEN before AND after the
move, keeps all suites + coverage green, and updates the `test:quick`
coverage-ignore allowlist in lockstep as files relocate.

## Constraint

This is a **planning artifact only**. Do NOT execute it now. Execution proceeds
via the [plan-execution workflow](../../../repo-governance/workflows/plan/plan-execution.md)
in a later step, after the orchestrator resolves the items in
`delivery.md` → "## Decisions Requiring User Approval".
