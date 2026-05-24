# Business Requirements — Two Rhino Versions

> WHY this work matters. See [prd.md](./prd.md) for WHAT and
> [tech-docs.md](./tech-docs.md) for HOW.

## Business Goal

Give the repository **two co-equal, behaviorally-identical implementations** of
its core hygiene/orchestration CLI — one in **Go** (`rhino-cli-go`) and one in
**Rust** (`rhino-cli-rust`) — both driven by the single behavior contract in
[`specs/apps/rhino/`](../../../specs/apps/rhino/README.md). The Rust build
becomes the one CI and the developer toolchain invoke; the Go build is retained
permanently as a parity twin.

This makes `rhino-cli` a **first-class member of the polyglot demonstration
family** the repo already maintains for backend services (`crud-be-*`): the same
contract, multiple languages, provable parity. As a _template_ repo (`ose-primer`
is MIT-licensed and copied into downstream forks), showing how to keep two CLI
implementations on par is itself a reusable teaching artifact.

## Why Now

- The sibling `ose-public` repo has already proven the Go→Rust technique for
  this exact CLI, so the path is de-risked and a concrete reference exists.
  _[Web-cited: `/Users/wkf/ose-projects/ose-public/plans/done/2026-05-23__rhino-cli-rust-rewrite/` — sibling repo, not verifiable in ose-primer.]_
- The Rust toolchain is already a first-class citizen here: a `setup-rust` CI
  composite action exists and `doctor` already probes `rustc` + `cargo-llvm-cov`.
  _[Repo-grounded: `.github/actions/setup-rust/action.yml`;
  `apps/rhino-cli/internal/doctor/checker.go` rust probes]_
- Adding the twin now, while the Go surface is the canonical reference, avoids
  re-porting later against a moving target.

## Affected Roles

| Role                        | Impact                                                                                                   |
| --------------------------- | -------------------------------------------------------------------------------------------------------- |
| Repo maintainer / AI agents | Run `rhino-cli` via Nx/husky/npm scripts daily; after cutover these invoke the Rust binary.              |
| Contributors to either CLI  | Must keep both implementations on par — any behavior change lands in both before the parity gate passes. |
| Downstream fork owners      | Inherit a worked example of dual-implementation CLI parity to copy or adapt.                             |
| CI                          | Gains a permanent shadow-diff parity job; loses nothing (Go gate replaced by Rust gate at cutover).      |

## Success Criteria (Business-Level)

- **Functional parity**: every command the Go CLI supports today behaves
  identically in Rust (same stdout, stderr, exit codes) — verified, not asserted.
- **No regression at cutover**: after the Go→Rust flip, every existing quality
  gate (pre-commit, pre-push, CI, the ~23 dependent projects' `test:quick` and
  `spec-coverage`) stays green.
- **Drift protection**: a parity gate runs on every PR so the two
  implementations cannot silently diverge.
- **Both retained**: the Go implementation remains buildable and tested; it is
  not archived or deleted.

> **Note (solo-maintainer repo)**: no sign-off/sponsor/stakeholder ceremony.
> "Success" is measured by green gates and the observable parity checks above,
> not by KPIs. No revenue/usage metrics apply. _[Judgment call]_

## Business-Scope Non-Goals

- Not expanding the CLI's feature set — strictly a second implementation of the
  existing surface.
- Not removing Go — the dual-implementation demonstration is the point.
- Not changing the behavior contract in `specs/apps/rhino/`.
- Not migrating other tooling to Rust.

## Business Risks

| Risk                                                                  | Severity | Mitigation                                                                                                                      |
| --------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------- |
| Cutover breaks CI / hooks because a caller reference was missed.      | High     | Enumerate callers by grep (not a hardcoded list); single atomic cutover commit; full `nx affected` + husky dry-run before push. |
| Rust port is subtly non-identical (e.g., output ordering, exit code). | High     | Shadow-diff harness asserts byte-identical output per command before its caller is flipped.                                     |
| Two implementations drift over time after cutover.                    | Medium   | Permanent shadow-diff parity gate in CI; both consume the same specs + spec-coverage target.                                    |
| Maintenance doubles (every change in two languages).                  | Medium   | Accepted cost — it is the explicit demonstration value; parity gate makes drift loud and early.                                 |
| Scope creep into a full ose-public-style rewrite (extra commands).    | Low      | Non-goal fixed: parity target is ose-primer's _current_ Go surface only.                                                        |
