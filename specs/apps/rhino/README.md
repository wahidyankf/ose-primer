# rhino-cli Specs

Gherkin behavioral specifications for the Repository Hygiene & INtegration
Orchestrator CLI, which ships as a single implementation that consumes these
specs:

- [`rhino-cli-rust`](../../../apps/rhino-cli-rust/README.md) — Rust; the implementation CI and the developer toolchain invoke.

## Purpose

These specs define the **observable behavior** of every rhino-cli command:
what inputs the command accepts, what it writes to stdout, and what exit code
it returns. They are the single source of truth for correctness and serve as
the contract between the CLI implementation and its consumers.

## Structure

```
specs/apps/rhino/
├── README.md
├── product/               # PM-first product docs (placeholder)
├── system-context/        # C4 Level 1 system context (placeholder)
├── containers/            # C4 Level 2 containers (placeholder)
├── components/
│   └── cli/               # C4 Level 3 CLI component (placeholder)
└── behavior/
    └── cli/
        └── gherkin/       # Gherkin feature files (11 domain subdirs)
            ├── agents/
            ├── contracts/
            ├── docs/
            ├── env/
            ├── git/
            ├── java/
            ├── repo-governance/
            ├── spec-coverage/
            ├── system/
            ├── test-coverage/
            └── workflows/
```

See [behavior/cli/gherkin/README.md](./behavior/cli/gherkin/README.md) for the full file inventory.

## Running the Tests

Both unit and integration tests consume these specs. Unit tests run with mocked
dependencies; integration tests run against real filesystem fixtures.

```bash
# Run all unit tests (includes BDD scenarios with mocked I/O)
nx run rhino-cli-rust:test:quick

# Run unit tests directly
cd apps/rhino-cli-rust
cargo test --lib

# Run all BDD integration tests (real filesystem fixtures)
nx run rhino-cli-rust:test:integration

# Run a specific integration suite during development
cd apps/rhino-cli-rust
cargo test --test integration doctor
```

The `test:integration` target is cached — it only re-runs when source files in
`src/**/*.rs` or `specs/apps/rhino/**/*.feature` change. The `test:unit` target
(via `test:quick`) is also cache-invalidated when these spec files change.

## Adding New Specs

1. Create `specs/apps/rhino/behavior/cli/gherkin/<domain>/<domain>-<action>.feature` (create the domain subdir if it does not exist)
2. Create the unit test under `apps/rhino-cli-rust/src/commands/` (mocked I/O):
   - Include a `// Scenario: <title>` comment for every scenario
   - Register step definitions using mocked function injection for all I/O
   - Name the test module `unit_<command>`
3. Create the integration test under `apps/rhino-cli-rust/tests/` (real filesystem fixtures):
   - Include a `// Scenario: <title>` comment for every scenario
   - Register step definitions that drive the command handler against real `/tmp` fixtures
   - Name the test `integration_<command>`
4. Verify with:

   ```bash
   cd apps/rhino-cli-rust
   cargo run -- spec-coverage validate specs/apps/rhino/behavior/cli/gherkin apps/rhino-cli-rust
   ```

## Dual Consumption

Every feature file in this directory is consumed at two test levels. The step implementations
differ but the Gherkin scenarios are identical:

| Level       | Test File Pattern                      | Step Implementation                 | Nx Target          |
| ----------- | -------------------------------------- | ----------------------------------- | ------------------ |
| Unit        | `src/commands/{domain}.rs` test module | Mocked I/O via function injection   | `test:unit`        |
| Integration | `tests/{domain}_{action}.rs`           | Real filesystem via `/tmp` fixtures | `test:integration` |

Coverage is measured at the unit level only (≥95% line coverage).

## Convention

See
[BDD Spec-to-Test Mapping Convention](../../../repo-governance/development/infra/bdd-spec-test-mapping.md)
for the mandatory 1:1 mapping between commands and `@tags`, file naming patterns, and coverage
enforcement rules.
