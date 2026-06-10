# Technical Documentation — rhino-cli Hexagonal Migration

## Architecture: Hybrid Kernel + Per-Feature Vertical Slices

Each layer holds a shared kernel subpackage plus one subpackage per feature.

```mermaid
flowchart LR
  %% Colour-blind-friendly palette (blue / orange)
  subgraph Inbound["Inbound adapter (CLI parser)"]
    RS_CMD["Rust: src/commands/"]
  end
  subgraph App["Application (use cases + port defs)"]
    RS_APP["Rust: src/application/"]
  end
  subgraph Dom["Domain (pure, zero IO)"]
    RS_DOM["Rust: src/domain/"]
  end
  subgraph Out["Outbound adapters (IO behind ports)"]
    RS_OUT["Rust: src/infrastructure/"]
  end
  Inbound --> App
  App --> Dom
  Out -. implements ports defined by .-> App
  Out --> Dom

  style Dom fill:#0072B2,color:#fff
  style App fill:#56B4E9,color:#000
  style Inbound fill:#E69F00,color:#000
  style Out fill:#E69F00,color:#000
```

### Layer directories (verified present)

| Layer         | Rust                                         |
| ------------- | -------------------------------------------- |
| Domain        | `src/domain/` (exists) [Repo-grounded]       |
| Application   | `src/application/` (exists per inventory)    |
| Outbound      | `src/infrastructure/` (exists per inventory) |
| Inbound (CLI) | `src/commands/` (exists) [Repo-grounded]     |

The Rust `src/domain/`, `src/application/`, `src/infrastructure/` directories
exist per the supplied inventory (`.gitkeep` placeholders) [Repo-grounded for the
inventory; verify the exact placeholder files in Phase 0].

## Shared-Kernel Rule (2+ consumers)

A type or util enters the shared kernel ONLY if used by 2+ features; a
single-consumer item stays feature-local.

| Shared kernel       | Rationale                                                                  |
| ------------------- | -------------------------------------------------------------------------- |
| `{mermaid, cliout}` | `mermaid` by docs+git; `cliout` by doctor, envbackup, mermaid (inventory). |

Single-consumer, stays feature-local: `naming` (agents-only).

## Port Mechanism

### Rust — trait objects (`Box<dyn Trait>`)

Formalizes the existing Rust `git` `Deps<'a>` which already uses `Box<dyn Fn...>`
and `Box<dyn Write>` [Repo-grounded — `apps/rhino-cli-rust/src/internal/git/runner.rs`
lines 48–76] into named multi-method traits. The application layer defines small
traits; adapters implement them; wiring is done once at `main()`/`cli::run()`. Do
NOT use generics/monomorphization for injection (avoids the lifetime/verbosity
trap).

```rust
// src/application/git/ports.rs (illustrative)
pub trait StagedFileProvider { fn staged_files(&self, git_root: &Path) -> Result<Vec<String>>; }
pub trait ToolProber { fn run(&self, name: &str, args: &[&str], cwd: &Path) -> io::Result<ExitStatus>; }
```

> If any port needs `async fn`, note that `async fn` in traits (Rust 1.75) cannot
> be used with `dyn` without the `async-trait` crate (heap alloc) [Web-cited —
>
> > see Research Basis]. No current `git` seam is async; flag per feature if one
> > arises.

## Port Naming Rule (domain role, not technology)

Name ports for the **domain role** they play, never the technology:

| Good (domain role)   | Bad (technology)  |
| -------------------- | ----------------- |
| `StagedFileProvider` | `FileSystem`      |
| `ToolProber`         | `CommandExecutor` |
| `CoverageReader`     | `FileReader`      |

This rule is codified in the convention doc in the final phase.

## Design Decision: Maximal Port Depth (accepted trade-off)

**Decision (final, user-chosen)**: EVERY IO boundary becomes a named port —
filesystem read/write, process/exec spawn, network — including single-function
seams. The domain layer is pure (zero IO imports).

**Trade-off, recorded honestly**: research consensus is that named ports belong
on multi-method cross-boundary collaborators, while single-function seams (a
one-off file read, `time.Now`) are fine as plain function parameters; a minority
view calls CLI hex "overkill" for trivial scripts [Web-cited — Research Basis].
The maximal approach therefore accepts some single-implementation ports and extra
boilerplate in exchange for **uniform domain purity and a single, predictable
seam pattern across all 13 features**. The over-engineering
risk is real and documented; the maintainer chose uniformity over leanness
deliberately. This trade-off is one line in the convention doc, not an argument
against the decision.

## Enforcement (language tooling only)

- **Rust**: module privacy (`pub`/non-`pub`) + `cargo clippy -- -D warnings`
  (part of the `lint` target) [Repo-grounded — `rhino-cli-rust` `lint`].

No new architecture/import-direction lint is added.

## Behavior-Preserving Migration Recipe (per feature)

Feathers' characterization-test (golden master) discipline + strangler-fig,
seam-by-seam, tests green after each step [Web-cited — Research Basis]:

1. Confirm the golden-master CLI suite GREEN (golden corpus captured at Phase 0).
2. Extract the pure core into `domain/<feature>/` (move pure functions; strip IO).
3. Define inbound port (use-case entry) + outbound ports in `application/<feature>/`.
4. Implement outbound adapters (`src/infrastructure/<feature>/`).
5. Wire the entry point (`commands`) to the application use case.
6. Re-run the golden-master CLI suite GREEN; run unit + integration + coverage;
   update the coverage-ignore allowlist if any listed file moved.

Migrate most-business-logic-rich first; the `git` pilot proves the recipe.

## Phase-Ordering Constraint

`mermaid` (shared kernel) is imported by `docs` and `git`. The `git` pilot
already consumes mermaid validators via `Deps`. Therefore migrate the shared
kernel(s) (`mermaid`, `cliout`) early, before or together with `docs`. IO-heavy
features (envbackup, doctor, testcoverage, git) get their own phases given port
volume; lighter features are grouped. The exact grouping is proposed in
`delivery.md` for user confirmation.

## File-Impact Summary

| Path                                                                | Change                                                                                       |
| ------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `apps/rhino-cli-rust/src/{domain,application,infrastructure}/`      | Populated with per-feature + shared modules                                                  |
| `apps/rhino-cli-rust/src/internal/<feature>/`                       | Logic relocated into layers; thin shims removed                                              |
| `apps/rhino-cli-rust/project.json`                                  | `test:quick` `--ignore-filename-regex` allowlist updated per phase [Repo-grounded — line 83] |
| `repo-governance/development/pattern/hexagonal-architecture-cli.md` | Final-phase content update [Repo-grounded — file exists]                                     |

## Quality Gates (Nx targets — all verified present)

[Repo-grounded — `apps/rhino-cli-rust/project.json`]: `build`, `test:unit`,
`test:quick` (coverage ≥90%), `test:integration`, `lint`, `typecheck`,
`spec-coverage`, plus other `validate:*`. Behavior gate: the golden-master CLI
suite.

## Testing Strategy

- **Unit** — domain logic tested directly with fakes substituted for ports
  (Rust inline `#[cfg(test)]`). Maps to the domain-purity Gherkin scenarios.
- **Integration** — Rust `tests/*.rs` cucumber files. Maps to inbound-adapter
  scenarios.
- **Golden master** — the golden-master CLI suite byte-diffs command output
  against the Phase 0 baseline. Maps to the behavior-preservation scenarios.
- **TDD shape** — for any new port/adapter code, write the failing fake-backed
  unit test first (RED), implement (GREEN), refactor; delivery.md expresses code
  items as RED/GREEN/REFACTOR.

## Research Basis (cited)

- CLI-as-inbound-adapter is canonical: Cockburn ports-and-adapters (2005);
  corroborated by Herberto Graça (herbertograca.com) and AWS Prescriptive
  Guidance [Web-cited, accessed 2026-06-09].
- Over-engineering is the dominant documented risk for hex-on-CLI; ports belong
  on multi-method collaborators; minority "overkill" view (skoredin.pro) applies
  to trivial scripts only [Web-cited, accessed 2026-06-09].
- Rust idiom: traits as ports; `Box<dyn>` appropriate for a fixed adapter graph
  wired once; generics-everywhere is a lifetime/verbosity trap (howtocodeit.com;
  tuttlem.github.io; HN 41518698); `async fn` in traits needs `async-trait` for
  `dyn` [Web-cited, accessed 2026-06-09].
- Port-naming anti-pattern: name for the concept, not the implementation (Rust
  Users Forum; pitfalls catalogs) [Web-cited, accessed 2026-06-09].
- Behavior-preserving refactor: Feathers' characterization tests + strangler-fig
  (understandlegacycode.com; ro-che.info golden tests) [Web-cited, accessed
  2026-06-09].

> All external claims above were supplied as pre-verified research with the task.
> Re-verify via `web-research-maker` before execution if any becomes load-bearing
> for an implementation decision.

## Rollback

Each phase is a natural pause with a green git state. Rollback = `git revert` the
phase's commits; the golden-master CLI suite + the test suites confirm
restoration. No data migrations or irreversible steps are involved.
