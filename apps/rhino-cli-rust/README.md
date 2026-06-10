# rhino-cli

**Repository Hygiene & INtegration Orchestrator** — the Rust CLI that CI and the developer toolchain invoke to keep this monorepo consistent: coverage gates, spec coverage, markdown/link/diagram validation, agent-binding sync, governance audits, and environment-file management.

## What it is

`rhino-cli` is a single, self-contained Rust binary. Its observable behavior is defined by the Gherkin contract in [`specs/apps/rhino/`](../../specs/apps/rhino/README.md), and every command is exercised by the cucumber integration suite in [`tests/`](./tests). All `package.json` scripts and Husky hooks that mention `rhino` shell out to this binary.

## Quick start

```bash
# Build the release binary (also copied to dist/rhino-cli)
nx run rhino-cli-rust:build

# Run any command via cargo
cargo run --release -q --manifest-path apps/rhino-cli-rust/Cargo.toml -- doctor

# Check all required polyglot toolchains are installed
cargo run --release -q --manifest-path apps/rhino-cli-rust/Cargo.toml -- doctor --fix
```

## Commands

| Command                                                                                                    | Purpose                                                                      |
| ---------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| `doctor [--fix]`                                                                                           | Probe and converge the polyglot toolchain (Go, Java, Rust, …).               |
| `test-coverage validate <lcov\|cover.out> <threshold>`                                                     | Enforce a line-coverage threshold; also `diff` and `merge`.                  |
| `spec-coverage validate <gherkin-dir> <impl-dir> [--shared-steps]`                                         | Assert every Gherkin step has a matching step definition.                    |
| `docs validate-links \| validate-mermaid \| validate-heading-hierarchy`                                    | Markdown link, diagram, and heading-hierarchy validation.                    |
| `agents sync \| validate-claude \| validate-sync \| validate-naming \| emit-bindings \| validate-bindings` | Sync `.claude/` → `.opencode/` + `.amazonq/` bindings and validate them.     |
| `repo-governance vendor-audit <path> \| gherkin-keyword-cardinality`                                       | Governance vendor-neutrality and Gherkin step-keyword cardinality audits.    |
| `workflows validate-naming`                                                                                | Validate workflow-document naming.                                           |
| `git pre-commit`                                                                                           | Orchestrate the staged-file pre-commit checks (formatting, link validation). |
| `contracts java-clean-imports \| dart-scaffold`                                                            | Post-process generated OpenAPI contract code.                                |
| `java validate-annotations`                                                                                | Enforce `@NullMarked` in generated Java `package-info.java`.                 |
| `env init \| backup \| restore \| validate`                                                                | Manage `.env*` files and validate declared-vs-read env-var drift.            |

Run `rhino-cli <command> --help` for full flags. Global flags: `--output text\|json\|markdown`, `--quiet`, `--verbose`, `--no-color`.

## Develop

```bash
nx run rhino-cli-rust:lint           # rustfmt --check + clippy -D warnings
nx run rhino-cli-rust:test:quick     # library coverage gate (90% lines)
nx run rhino-cli-rust:test:integration   # cucumber suites against specs/apps/rhino
nx run rhino-cli-rust:spec-coverage  # every Gherkin step has a step definition
```

The binary is implemented with [clap](https://docs.rs/clap) and follows a hexagonal layout: thin command adapters in `src/commands/` over domain logic in `src/internal/`. See the [Hexagonal Architecture (CLI)](../../repo-governance/development/pattern/hexagonal-architecture-cli.md) pattern.

## Related

- [`specs/apps/rhino/`](../../specs/apps/rhino/README.md) — the behavior contract this CLI satisfies.
- [Nx Targets](../../repo-governance/development/infra/nx-targets.md) — the full target catalog.
