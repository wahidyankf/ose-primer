# Product Requirements — rhino-cli Hexagonal Migration

## Product Overview

Both `rhino-cli` binaries (Go + Rust) are refactored into a hexagonal layout:
a `domain/shared/` kernel plus per-feature vertical slices, with every IO
boundary expressed as a named port. The observable CLI behavior is preserved
byte-for-byte (verified by `shadow-diff.sh`): the output surface is **frozen**
(zero visible change), so the golden corpus is never re-baselined.

## Personas (maintainer hats + consuming agents)

| Persona                      | Need                                                                          |
| ---------------------------- | ----------------------------------------------------------------------------- |
| Maintainer-as-Go-developer   | A predictable place for domain logic, ports, and adapters per feature.        |
| Maintainer-as-Rust-developer | The same layout mirrored in Rust, keeping the two binaries parallel.          |
| Maintainer-as-reviewer       | A green shadow-diff + parity script as the single behavior-preservation gate. |
| `swe-golang-dev` agent       | Unambiguous, execution-grade steps for Go file moves and port extraction.     |
| `swe-rust-dev` agent         | The same for Rust, plus the coverage-allowlist update reminder.               |

## User Stories

- **As the maintainer-as-Go-developer**, I want every IO boundary behind a named
  Go interface owned by the application layer, **so that** I can unit-test domain
  logic with fakes and never reach a real filesystem or subprocess in a unit test.
- **As the maintainer-as-Rust-developer**, I want the Rust binary to mirror the Go
  port structure using `Box<dyn Trait>`, **so that** the two binaries stay
  structurally parallel and divergence is visible at the seam.
- **As the maintainer-as-reviewer**, I want `shadow-diff.sh` to stay GREEN before
  and after each feature move, **so that** I have byte-level proof no behavior
  changed during the refactor.
- **As a consuming agent**, I want each delivery step to name exact paths and
  verification commands, **so that** I can execute the migration without guessing.
- **As the maintainer**, I want ports named for their domain role (e.g.
  `StagedFileProvider`), never for technology (`FileSystem`), **so that** the
  abstraction communicates intent and resists leaking implementation details.

## Acceptance Criteria (Gherkin)

> Step-keyword cardinality: each scenario uses exactly one primary `Given`, one
> `When`, one `Then`; extras chain with `And`.

### Behavior preservation

```gherkin
Scenario: A feature move preserves byte-level CLI output
  Given a feature has been migrated to its hexagonal slice in both Go and Rust
  When shadow-diff.sh runs the full command corpus against both rebuilt binaries
  Then the script exits 0 with no divergence reported
  And both binaries produce identical text, json, and markdown output under --no-color
```

```gherkin
Scenario: Structural refactor leaves the golden corpus unchanged
  Given no approved CLI-output improvement applies to the feature being moved
  When the feature is migrated to its hexagonal slice
  Then the shadow-diff golden corpus output is unchanged from the Phase 0 baseline
  And no command's stdout or stderr bytes differ
```

### Domain purity (per-layer assertion)

```gherkin
Scenario: Go domain layer imports zero IO packages
  Given a Go feature has been migrated to internal/domain/<feature>/
  When the domain package source is scanned for import statements
  Then no import references os, io, os/exec, net, path/filepath for IO, or any adapter package
  And go vet ./... reports no errors
```

```gherkin
Scenario: Rust domain layer imports zero IO modules
  Given a Rust feature has been migrated to src/domain/<feature>/
  When the domain module source is scanned for use statements
  Then no use references std::fs, std::process, std::net, or any infrastructure module
  And cargo clippy -- -D warnings reports no warnings
```

### Inward-only dependency direction

```gherkin
Scenario: Adapters depend inward only
  Given a feature's inbound and outbound adapters have been migrated
  When the adapter source imports are inspected
  Then adapters import application and domain types but domain imports neither application nor adapters
  And the Go internal/ wall and Rust module privacy compile without boundary violations
```

### Ports are named for domain role

```gherkin
Scenario: Outbound ports carry domain-role names
  Given a feature's IO boundaries have been extracted into ports
  When the port interface or trait names are reviewed
  Then each name describes a domain role such as StagedFileProvider, ToolProber, or CoverageReader
  And no port name describes a technology such as FileSystem or CommandExecutor
```

### Maximal port depth

```gherkin
Scenario: Every IO boundary becomes a named port
  Given a feature performs filesystem, process-spawn, or network IO
  When the migration extracts its IO seams
  Then each distinct IO boundary is represented by a named port, including single-function seams
  And the domain and application layers reference only ports, never concrete IO
```

### Git pilot proof gate

```gherkin
Scenario: The git pilot validates the migration recipe in both languages
  Given the git feature has been migrated to hexagonal slices in Go and Rust
  When the Phase 1 gate runs all suites, coverage, shadow-diff, and the parity script
  Then every gate check passes for both binaries
  And the git Deps struct is replaced by named consumer-owned ports in both languages
```

### Coverage allowlist lockstep

```gherkin
Scenario: Rust coverage allowlist tracks relocated files
  Given a Rust file listed in the test:quick --ignore-filename-regex allowlist is moved
  When the migration relocates that file to its hexagonal slice
  Then the allowlist entry in apps/rhino-cli-rust/project.json is updated to the new path in the same phase
  And nx run rhino-cli-rust:test:quick reports coverage at or above 90% without false breakage
```

### Output surface is frozen (zero visible change)

```gherkin
Scenario: The migration introduces no visible output change
  Given the user froze the output surface with an empty approved-change list
  When any feature is migrated to its hexagonal slice
  Then no command's stdout or stderr bytes differ from the Phase 0 baseline
  And the shadow-diff golden corpus is never re-captured to a new baseline
```

### Convention doc update

```gherkin
Scenario: The convention doc records the chosen architecture and stays vendor-neutral
  Given all features have been migrated
  When repo-governance/development/pattern/hexagonal-architecture-cli.md is updated
  Then it documents the hybrid kernel-plus-slices layout, the maximal-ports rule, the domain-role naming rule, and the 2-plus-consumer shared-kernel rule
  And validate:repo-governance-vendor-audit and validate:cross-vendor-parity both pass
```

## Product Scope

**In scope (features migrated — all 13, both languages)**: agents, contracts,
docs, doctor, env/envbackup, git, java, mermaid, naming, repo-governance,
speccoverage, testcoverage, workflows [Repo-grounded — present as `internal/`
dirs in both apps; `workflows` has Go logic in `cmd/` only, `repo-governance`
has Rust logic in `commands/` only].

**In scope (other)**: shared-kernel extraction (Go `{mermaid}`; Rust
`{mermaid, cliout}`); maximal port extraction; the convention-doc update.

**Out of scope**: new architecture lint; behavior change outside the frozen list;
new features; performance work; dependency upgrades unrelated to layering;
`fileutil` (Go, docs-only) and `naming` (Go unused / Rust agents-only) promotion
to the kernel — these stay feature-local per the 2+-consumer rule.

## Product-Level Risks

| Risk                                                     | Mitigation                                                                                                                                         |
| -------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| A "structural-only" move silently changes visible output | Output frozen (empty change list); shadow-diff GREEN required before AND after every move AND must stay GREEN vs the Phase 0 baseline every phase. |
| Maximal ports over-abstract trivial seams                | Accepted trade-off; uniformity is the deliberate benefit (tech-docs.md).                                                                           |
| Coverage gate breaks on file relocation                  | Mandatory allowlist update step in every Rust-touching phase.                                                                                      |
