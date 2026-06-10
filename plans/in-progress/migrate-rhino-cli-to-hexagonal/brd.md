# Business Requirements — rhino-cli Hexagonal Migration

## Business Goal

Make the `rhino-cli` binary easier to maintain, test, and evolve by adopting a
uniform hexagonal (ports-and-adapters) architecture, so that domain logic, IO,
and CLI wiring become structurally separated rather than interleaved.

## Why This Exists

The repository maintains a single CLI binary whose visible behavior must be
preserved exactly during refactors. Today domain logic, IO, and CLI wiring are
interleaved in flat per-feature packages, with only the `git` feature
demonstrating clean dependency injection via a `Deps` struct
[Repo-grounded — `apps/rhino-cli-rust/src/internal/git/runner.rs`]. The result:

- **Pain — test friction**: IO-heavy features (envbackup, doctor, testcoverage)
  are hard to unit-test without real filesystem/process effects, pushing coverage
  onto integration tests and the hardcoded coverage-ignore allowlist
  [Repo-grounded — `apps/rhino-cli-rust/project.json` `test:quick`
  `--ignore-filename-regex` list].
- **Pain — regression risk**: with no enforced layering, a change can entangle
  domain logic with IO in ways that the test suite catches only at the output
  layer, not at the structural layer.
- **Pain — onboarding friction**: a contributor must learn each feature's bespoke
  internal shape; there is no consistent "where does domain logic live" answer.

Hexagonal architecture for CLI tools is canonical, not novel: Cockburn's original
2005 ports-and-adapters paper explicitly lists the CLI/console as a driving
(primary) adapter alongside HTTP and tests [Web-cited — see tech-docs.md
"Research Basis" for the full citation set, accessed 2026-06-09]. The argument
parser (clap) is the inbound adapter; the use-case service it calls lives
inside the hexagon. No surveyed source disagrees with CLI-as-inbound-adapter.

## Business Impact

| Dimension       | Before                                                           | After (target)                                                                     |
| --------------- | ---------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| Unit-test reach | IO seams hard to fake; coverage leans on integration + allowlist | Every IO boundary is a fakeable named port; domain is pure and fully unit-testable |
| Layer clarity   | Logic + IO + wiring entangled in flat packages                   | Structurally separated layers; entanglement visible at the seam                    |
| Onboarding      | Per-feature bespoke shapes                                       | One uniform layout: `domain/shared` kernel + per-feature slices                    |
| Change locality | Logic + IO + wiring entangled                                    | Logic changes touch `domain/`; IO changes touch adapters only                      |

## Affected Roles (solo-maintainer repo — hats, not sign-offs)

- **Maintainer-as-Rust-developer** — works in `src/domain|application|infrastructure`.
- **Maintainer-as-reviewer** — relies on the golden-master CLI suite as the gate.
- **Consuming agents** — `swe-rust-dev`, `repo-setup-manager`, and the
  plan-execution orchestrator consume `delivery.md`
  [Repo-grounded — `.claude/agents/swe-rust-dev.md`,
  `.claude/agents/repo-setup-manager.md` both exist].

## Business-Level Success Metrics

- **Behavior preserved** — the golden-master CLI suite exits GREEN at the end of
  every feature phase and at plan completion (observable fact: suite exit code 0).
- **Quality gates intact** — `test:unit`, `test:integration`, `test:quick`
  (coverage ≥90%), `lint`, and `typecheck` remain green throughout
  [Repo-grounded — all are real Nx targets in `project.json`].
- **Uniform structure achieved** [Judgment call] — every feature has
  domain logic in `domain/`, ports defined in `application/`, and IO in adapters;
  the domain layer imports zero IO packages (verifiable by `grep` for IO imports
  under `domain/`).
- **Coverage allowlist shrinks or holds** [Judgment call] — as IO moves behind
  ports, the `test:quick` ignore allowlist should not need to grow; ideally
  some entries become removable. Tracked, not a hard target.

> No fabricated numeric KPIs. The coverage floor (90%) is an existing, measured
> gate, not a new target invented here.

## Business-Scope Non-Goals

- NOT a rewrite — behavior is frozen; this is a purely structural refactor with
  zero visible output change.
- NOT introducing a new architecture lint — language tooling enforces boundaries.
- NOT changing the visible behavior contract — the golden-master CLI suite
  remains the behavior gate.
- NOT optimizing performance or adding features.

## Business Risks and Mitigations

| Risk                                               | Likelihood | Mitigation                                                                                             |
| -------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------ |
| Behavior regression during a move                  | Medium     | Per-feature golden-master suite GREEN before AND after; phase gate blocks progress on any red.         |
| Coverage gate false-break when files relocate      | High       | Mandatory lockstep update of the `test:quick` `--ignore-filename-regex` allowlist in every phase.      |
| Over-engineering (maximal ports add boilerplate)   | Medium     | Accepted trade-off (see tech-docs.md); maximal chosen deliberately over lean; mitigated by uniformity. |
| Vendor-specific terms leak into the convention doc | Low        | `validate:repo-governance-vendor-audit` gates the final phase.                                         |

## Cross-Cutting Note

The maximal-vs-lean ports decision is a **product/technical** trade-off; its
factual basis (over-engineering is the documented dominant risk for hex-on-CLI)
lives here as a business risk, and its testable assertions (domain purity,
inward-only dependencies) live in `prd.md` as Gherkin scenarios.
