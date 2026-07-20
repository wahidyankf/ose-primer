# Standardize App Spec Trees — Parity Decisions

Plain-language explanation of every decision in the cross-repo deviation matrix
for the "Standardize App Spec Trees" parity effort.
Follows the `*-parity-decisions.md` precedents in this directory.

`ose-public` is the **canonical reference** for this parity set. Every divergence
below is intentional and recorded; zero silent deviations.

---

## R1 — Parity set

Three repos participate: `ose-public` (canonical reference), `ose-primer`, and
`ose-infra`. All three adopt flat product-surface behavior dir naming. Deviations
are per-repo: each repo renames its own families, and only `ose-public` has a
multi-product family consolidation.

---

## R2 — Flat product-surface naming scheme

Every `behavior/` subdirectory now follows the pattern
`behavior/<product>-<surface>/gherkin/`. The `<surface>` token is one of `be`
(backend HTTP), `web` (frontend), or `cli` (command-line interface).

**Why flat product-surface over bare-surface**: the dir name now states both
which product and which perspective, making the path self-descriptive. A reader
scanning `specs/apps/crud/behavior/crud-be/gherkin/` immediately knows they are
in the CRUD family's backend HTTP specs without needing any external context.

**ose-primer families restructured**:

| Family  | Old behavior dir | New behavior dir      |
| ------- | ---------------- | --------------------- |
| `crud`  | `behavior/be/`   | `behavior/crud-be/`   |
| `crud`  | `behavior/web/`  | `behavior/crud-web/`  |
| `rhino` | `behavior/cli/`  | `behavior/rhino-cli/` |

For single-product families (all ose-primer families are single-product), the
family name serves as the product token directly.

---

## R3 — `be` over `api`

The backend perspective is always named `be`, never `api`. `ose-primer` already
used `be` (no `api` surface existed in this repo), so no additional rename was
needed here. The convention amendment still carries the `be`-over-`api` rule so
the convention text is identical to `ose-public`'s.

---

## R4 — Multi-product family consolidation (N/A for ose-primer)

`ose-public` has a multi-product family (`ose-app` + `ose-platform` → `ose`) that
requires a tree consolidation alongside the surface rename. `ose-primer` has no
multi-product families — `crud` and `rhino` are both single-product — so this
step is N/A for this repo.

---

## R5 — Convention text parity

The amended subsection of
`repo-governance/conventions/structure/specs-directory-structure.md` (the flat
product-surface rule, the `be`-over-`api` rule, and the worked examples) is
authored to be **byte-identical** between `ose-primer` and `ose-public`. This is
the normative cross-repo identity decision recorded in the deviation matrix:
`ose-primer` adopts the same convention text verbatim.

`ose-primer` authored its amendment first (Plan Phase 4); `ose-public` Phase G
will adopt the same text. Until `ose-public` Phase G ships, the diff check
(`diff <ose-primer subsection> <ose-public subsection>`) is deferred.

---

## R6 — Delivery mode — main-to-main deviation

`ose-primer`'s Sync Convention Safety Invariant 6 mandates that every mutation
reaching `ose-primer` MUST flow through a PR (PR-only default). This plan instead
delivered **main-to-main** (the mode name at the time of this 2026-06-11 decision;
the same primary-checkout-direct-push mode was later renamed `main-to-origin-main`
in the canonical four-mode Delivery Mode vocabulary — see
[Plans Organization Convention §Delivery Mode](../../repo-governance/conventions/structure/plans.md#delivery-mode)):
commits pushed directly to `origin main` via the worktree branch, no PR.

**Why the invoker accepted the deviation**: this plan is a docs-and-structure
change — plan markdown, a rationale doc, a convention amendment, agent-doc
updates, and `git mv` relocations with mechanical consumer-path rewiring. No new
application behavior, no runtime code logic, no config that changes deployed
behavior. The `spec-coverage` tooling takes paths as arguments, so the rename is
a pure path substitution. The risk of a path-rewiring slip is caught locally by
`nx affected -t spec-coverage test:quick` before push and by CI after push — the
PR review ceremony adds little risk reduction for a mechanical move.

**Scope**: this deviation is scoped to this plan only. It does not relax the
PR-only default for any future ose-primer mutation.

---

## R7 — TDD-shaped Rust test update

The Rust integration tests in `apps/rhino-cli/tests/*.rs` hardcode the gherkin
path via `.join("../../specs/apps/rhino/behavior/cli/gherkin/...")`. The update
followed a TDD cycle:

1. **RED**: confirmed old path gone on disk (directory moved), tests would fail
2. **GREEN**: bulk sed replaced all 23 `.join()` path literals across 12 test
   files; `npx nx run rhino-cli:test:quick` passed (648 tests)
3. **REFACTOR**: `npx nx run rhino-cli:spec-coverage` passed (23 specs, 181
   scenarios, 755 steps — all covered)

This TDD shape preserves the semantic intent: the test suite verifies behavior
against the moved gherkin tree, not a copy.

---

## Cross-Repo Deviation Matrix Reference

See `plans/in-progress/standardize-app-spec-trees/tech-docs.md` §Cross-Repo
Deviation Matrix for the full table spanning all three repos and all dimensions.
