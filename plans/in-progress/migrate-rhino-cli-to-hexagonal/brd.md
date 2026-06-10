# Business Requirements — rhino-cli Hexagonal Migration

## Business Goal

Make both `rhino-cli` binaries (Go and Rust) easier to maintain, test, and
evolve in lockstep by adopting a uniform hexagonal (ports-and-adapters)
architecture, so that the dual-language parity burden becomes a structural
invariant rather than an act of constant manual vigilance.

## Why This Exists

The repository deliberately maintains two structurally parallel CLI binaries
that must stay byte-for-byte identical (enforced by `shadow-diff.sh`)
[Repo-grounded — `apps/rhino-cli-rust/scripts/shadow-diff.sh`]. Today domain
logic, IO, and CLI wiring are interleaved in flat per-feature packages, with only
the `git` feature demonstrating clean dependency injection via a `Deps` struct
[Repo-grounded — `apps/rhino-cli-go/internal/git/runner.go`,
`apps/rhino-cli-rust/src/internal/git/runner.rs`]. The result:

- **Pain — test friction**: IO-heavy features (envbackup, doctor, testcoverage)
  are hard to unit-test without real filesystem/process effects, pushing coverage
  onto integration tests and the hardcoded Rust coverage-ignore allowlist
  [Repo-grounded — `apps/rhino-cli-rust/project.json` `test:quick`
  `--ignore-filename-regex` list].
- **Pain — parity drift risk**: with no enforced layering, a change in one
  language can diverge from the other in ways `shadow-diff.sh` catches only at
  the output layer, not at the structural layer.
- **Pain — onboarding friction**: a contributor must learn each feature's bespoke
  internal shape; there is no consistent "where does domain logic live" answer.

Hexagonal architecture for CLI tools is canonical, not novel: Cockburn's original
2005 ports-and-adapters paper explicitly lists the CLI/console as a driving
(primary) adapter alongside HTTP and tests [Web-cited — see tech-docs.md
"Research Basis" for the full citation set, accessed 2026-06-09]. The argument
parser (cobra/clap) is the inbound adapter; the use-case service it calls lives
inside the hexagon. No surveyed source disagrees with CLI-as-inbound-adapter.

## Business Impact

| Dimension             | Before                                                           | After (target)                                                                     |
| --------------------- | ---------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| Unit-test reach       | IO seams hard to fake; coverage leans on integration + allowlist | Every IO boundary is a fakeable named port; domain is pure and fully unit-testable |
| Cross-language parity | Maintained manually + caught only at output layer by shadow-diff | Structurally parallel layers in both langs; divergence visible at the seam         |
| Onboarding            | Per-feature bespoke shapes                                       | One uniform layout: `domain/shared` kernel + per-feature slices                    |
| Change locality       | Logic + IO + wiring entangled                                    | Logic changes touch `domain/`; IO changes touch adapters only                      |

## Affected Roles (solo-maintainer repo — hats, not sign-offs)

- **Maintainer-as-Go-developer** — works in `internal/domain|application|adapter`.
- **Maintainer-as-Rust-developer** — works in `src/domain|application|infrastructure`.
- **Maintainer-as-reviewer** — relies on shadow-diff + parity script as the gate.
- **Consuming agents** — `swe-golang-dev`, `swe-rust-dev`, `repo-setup-manager`,
  and the plan-execution orchestrator consume `delivery.md`
  [Repo-grounded — `.claude/agents/swe-golang-dev.md`,
  `.claude/agents/swe-rust-dev.md`, `.claude/agents/repo-setup-manager.md` all
  exist].

## Business-Level Success Metrics

- **Behavior preserved** — `shadow-diff.sh` exits GREEN at the end of every
  feature phase and at plan completion (observable fact: script exit code 0).
- **Quality gates intact** — `test:unit`, `test:integration`, `test:quick`
  (coverage ≥90% both langs), `lint`, `typecheck`, and
  `validate:cross-vendor-parity` remain green throughout [Repo-grounded — all
  are real Nx targets in both `project.json` files].
- **Uniform structure achieved** [Judgment call] — every feature in both apps has
  domain logic in `domain/`, ports defined in `application/`, and IO in adapters;
  the domain layer imports zero IO packages (verifiable by `grep` for IO imports
  under `domain/`).
- **Coverage allowlist shrinks or holds** [Judgment call] — as IO moves behind
  ports, the Rust `test:quick` ignore allowlist should not need to grow; ideally
  some entries become removable. Tracked, not a hard target.

> No fabricated numeric KPIs. The coverage floor (90%) is an existing, measured
> gate, not a new target invented here.

## Business-Scope Non-Goals

- NOT a rewrite — behavior is frozen; this is a purely structural refactor with
  zero visible output change.
- NOT introducing a new architecture lint — language tooling enforces boundaries.
- NOT changing the dual-language parity contract — `shadow-diff.sh` remains the
  golden master.
- NOT optimizing performance or adding features.

## Business Risks and Mitigations

| Risk                                               | Likelihood | Mitigation                                                                                             |
| -------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------ |
| Behavior regression during a move                  | Medium     | Per-feature shadow-diff GREEN before AND after; phase gate blocks progress on any red.                 |
| Coverage gate false-break when Rust files relocate | High       | Mandatory lockstep update of the Rust `test:quick` `--ignore-filename-regex` allowlist in every phase. |
| Over-engineering (maximal ports add boilerplate)   | Medium     | Accepted trade-off (see tech-docs.md); maximal chosen deliberately over lean; mitigated by uniformity. |
| Go/Rust structural divergence during migration     | Medium     | Each feature phase migrates BOTH languages together; parity script + shadow-diff gate per phase.       |
| Vendor-specific terms leak into the convention doc | Low        | `validate:repo-governance-vendor-audit` + `validate:cross-vendor-parity` gate the final phase.         |

## Cross-Cutting Note

The maximal-vs-lean ports decision is a **product/technical** trade-off; its
factual basis (over-engineering is the documented dominant risk for hex-on-CLI)
lives here as a business risk, and its testable assertions (domain purity,
inward-only dependencies) live in `prd.md` as Gherkin scenarios.
