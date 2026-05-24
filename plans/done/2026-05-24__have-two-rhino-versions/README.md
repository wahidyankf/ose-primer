# Two Rhino Versions: `rhino-cli-go` + `rhino-cli-rust`

**Status**: In Progress
**Created**: 2026-05-24
**Plan identifier**: `have-two-rhino-versions`
**Git workflow**: Trunk Based Development — direct push to `main` (worktree is isolation only).

---

## Context

This repo ships one CLI today: `apps/rhino-cli/`, a **Go** application (the
"Repository Hygiene & INtegration Orchestrator") that powers pre-commit hooks,
agent sync, doctor, link/mermaid validation, naming validators, vendor audits,
spec-coverage, and test-coverage validation. It is wired into CI, husky hooks,
`package.json` scripts, and the `test:quick`/`spec-coverage` targets of ~23
other projects. _[Repo-grounded: `apps/rhino-cli/project.json`,
`.husky/pre-commit`, `.husky/pre-push`, `package.json`, `.github/workflows/`]_

We want **two co-equal implementations of the same CLI**, mirroring how the
`crud-be-*` apps maintain many language implementations of one behavior
contract:

- **`rhino-cli-go`** — the current Go implementation, renamed.
- **`rhino-cli-rust`** — a new Rust port, built command-for-command to parity.

Both consume the **same** behavior specs at
[`specs/apps/rhino/`](../../../specs/apps/rhino/README.md). The **Rust** version
becomes the one CI and the developer toolchain actually invoke; the **Go**
version is kept permanently as a behaviorally-identical twin, validated against
Rust by a shadow-diff parity gate on every PR.

The sibling repo [`ose-public`](https://github.com/wahidyankf/ose-public) already
performed a Go→Rust rewrite of its `rhino-cli` (plan
`plans/done/2026-05-23__rhino-cli-rust-rewrite/`). We reuse its **technique**
(clap derive tree, sealed `OutputFormat` enum, `cucumber-rs` BDD, shadow-diff
harness, `cargo llvm-cov` coverage floor) but **not** its outcome: ose-public
_archived_ its Go version, whereas we **keep both forever**. Our parity target
is **ose-primer's own Go command surface**, not ose-public's larger surface.
_[Web-cited: `/Users/wkf/ose-projects/ose-public/apps/rhino-cli/` is Rust;
`/Users/wkf/ose-projects/ose-public/archived/rhino-cli/` is the archived Go — sibling repo, not verifiable in ose-primer.]_

## Scope

**In scope**

- Rename `apps/rhino-cli/` (Go) → `apps/rhino-cli-go/` and repoint every caller.
- Create `apps/rhino-cli-rust/` (Rust) porting the full Go command surface.
- A permanent shadow-diff **parity gate** (Go vs Rust, byte-identical output).
- Big-bang cutover flipping every caller from `rhino-cli-go` → `rhino-cli-rust`.
- Docs + governance convention documenting the dual-implementation model.

**Out of scope**

- Adding _new_ CLI commands beyond the current Go surface.
- Archiving or deleting the Go implementation (it stays as the parity twin).
- Changing the `specs/apps/rhino/` behavior contract.
- Porting ose-public-only commands (`ddd`, `specs validate-*`, extra governance
  auditors) that do not exist in ose-primer's Go CLI.

**Affected projects**: `apps/rhino-cli` (→ `apps/rhino-cli-go`), new
`apps/rhino-cli-rust`, plus ~23 caller `project.json` files, `package.json`,
`.husky/`, `.github/workflows/pr-quality-gate.yml`,
`.github/workflows/pr-validate-links.yml`, governance docs.

## Approach Summary

Big-bang cutover (chosen over incremental flip):

1. **Rename + repoint** (Phase 1): `rhino-cli` → `rhino-cli-go` everywhere. CI
   stays green on Go.
2. **Scaffold Rust** (Phase 2): empty `rhino-cli-rust` crate with full target
   set, not yet wired into any caller.
3. **Port per domain** (Phases 3–8): each command group ported to Rust, gated by
   `cucumber-rs` BDD against `specs/apps/rhino/` + a shadow-diff byte-identical
   check vs Go.
4. **Parity gate** (Phase 9): permanent CI job running shadow-diff Go-vs-Rust.
5. **Cutover** (Phase 10): one commit flips all callers Go→Rust.
6. **Docs + convention** (Phase 11), then **quality gates + archival** (Phase 12).

## Document Map

| File                           | Purpose                                            |
| ------------------------------ | -------------------------------------------------- |
| [brd.md](./brd.md)             | WHY — business goal, affected roles, success, risk |
| [prd.md](./prd.md)             | WHAT — personas, user stories, Gherkin criteria    |
| [tech-docs.md](./tech-docs.md) | HOW — architecture, parity model, file impact      |
| [delivery.md](./delivery.md)   | DO — phased, execution-grade checklist             |

## References

- Behavior specs: [`specs/apps/rhino/`](../../../specs/apps/rhino/README.md)
- CRUD parity model: [`apps/crud-be-e2e/`](../../../apps/crud-be-e2e/README.md),
  [`repo-governance/development/infra/bdd-spec-test-mapping.md`](../../../repo-governance/development/infra/bdd-spec-test-mapping.md)
- Nx target standards: [`repo-governance/development/infra/nx-targets.md`](../../../repo-governance/development/infra/nx-targets.md)
- ose-public Rust reference: `/Users/wkf/ose-projects/ose-public/apps/rhino-cli/`
  and `plans/done/2026-05-23__rhino-cli-rust-rewrite/`
- Plans convention: [`repo-governance/conventions/structure/plans.md`](../../../repo-governance/conventions/structure/plans.md)
