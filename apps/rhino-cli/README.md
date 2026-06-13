# rhino-cli

**Repository Hygiene & INtegration Orchestrator** — the Rust CLI that CI and the developer toolchain invoke to keep this monorepo consistent: coverage gates, spec coverage, markdown/link/diagram validation, agent-binding sync, governance audits, and environment-file management.

## What it is

`rhino-cli` is a single, self-contained Rust binary. Its observable behavior is defined by the Gherkin contract in [`specs/apps/rhino/`](../../specs/apps/rhino/README.md), and every command is exercised by the cucumber integration suite in [`tests/`](./tests). All `package.json` scripts and Husky hooks that mention `rhino` shell out to this binary.

## Quick start

```bash
# Build the release binary (also copied to dist/rhino-cli)
nx run rhino-cli:build

# Run any command via cargo
cargo run --release -q --manifest-path apps/rhino-cli/Cargo.toml -- doctor

# Check all required polyglot toolchains are installed
cargo run --release -q --manifest-path apps/rhino-cli/Cargo.toml -- doctor --fix
```

## Commands

Commands follow the `{group} {verb} [{noun}]` form. The old `validate:*` prefix was
abolished in P10 (2026-06-12); all callers use the canonical `{domain}:{work}` Nx
target form instead.

| Group / Command                                                              | Nx target(s)                                                        | Purpose                                                                      |
| ---------------------------------------------------------------------------- | ------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| `doctor [--fix]`                                                             | —                                                                   | Probe and converge the polyglot toolchain (Go, Java, Rust, …).               |
| `test-coverage validate <lcov\|cover.out> <threshold>`                       | —                                                                   | Enforce a line-coverage threshold; also `diff` and `merge`.                  |
| `specs:coverage validate <gherkin-dir> <impl-dir> [--shared-steps]`          | `specs:coverage`                                                    | Assert every Gherkin step has a matching step definition.                    |
| `specs:tree-validation`, `specs:links-validation`, `specs:counts-validation` | `specs:tree-validation`, `specs:links-validation`, …                | Spec directory structure and link integrity audits.                          |
| `specs:adoption-validation`, `specs:gherkin-cardinality-validation`          | `specs:adoption-validation`, `specs:gherkin-cardinality-validation` | Spec adoption and Gherkin keyword cardinality checks.                        |
| `md validate-links`                                                          | `links:validation`                                                  | Validate internal markdown links and `#fragment` anchors repo-wide.          |
| `md validate-mermaid`                                                        | `mermaid:validation`                                                | Validate Mermaid diagram width, labels, syntax (flowchart + state diagrams). |
| `md validate-heading-hierarchy`                                              | `headings:hierarchy-validation`                                     | Validate heading nesting on prose allowlist paths.                           |
| `harness sync \| validate-claude \| validate-sync \| emit-bindings`          | `harness:bindings-validation`                                       | Sync `.claude/` → `.opencode/` + `.amazonq/` bindings and validate them.     |
| `convention vendor-audit <path>`                                             | `governance:vendor-audit-validation`                                | Governance vendor-neutrality audit on `repo-governance/` docs.               |
| `convention gherkin-keyword-cardinality`                                     | `specs:gherkin-cardinality-validation`                              | Gherkin step-keyword cardinality audit.                                      |
| `convention validate-cross-vendor-parity`                                    | `cross-vendor:parity-validation`                                    | Cross-vendor behavioral parity check across all binding trees.               |
| `convention validate-naming-harness`                                         | `naming:harness-validation`                                         | Validate agent definition file names against the naming convention.          |
| `convention validate-naming-workflows`                                       | `naming:workflows-validation`                                       | Validate workflow-document naming.                                           |
| `env init \| backup \| restore \| validate`                                  | `env:validation`                                                    | Manage `.env*` files and validate declared-vs-read env-var drift.            |
| `lang java-clean-imports \| dart-scaffold`                                   | —                                                                   | Post-process generated OpenAPI contract code.                                |
| `lang java-validate-annotations`                                             | —                                                                   | Enforce `@NullMarked` in generated Java `package-info.java`.                 |
| `git pre-commit`                                                             | —                                                                   | Orchestrate the staged-file pre-commit checks (formatting, link validation). |

`docs` is a **RESERVED** group namespace — no targets are defined under `docs:*`.

Run `rhino-cli <command> --help` for full flags. Global flags: `--output text\|json\|markdown`, `--quiet`, `--verbose`, `--no-color`.

## Develop

```bash
nx run rhino-cli:lint             # rustfmt --check + clippy -D warnings
nx run rhino-cli:test:quick       # library coverage gate (90% lines)
nx run rhino-cli:test:integration # integration suites against specs/apps/rhino
nx run rhino-cli:specs:coverage   # every Gherkin step has a step definition
```

The binary is implemented with [clap](https://docs.rs/clap) and follows a four-layer
hexagonal layout:

```
src/
├── domain/         # Pure business logic — no I/O
├── application/    # Use-case orchestration; ports (trait interfaces)
├── infrastructure/ # All I/O: filesystem, network, env
└── commands/       # Inbound adapter: CLI arg parsing → application calls
```

No file in `src/domain/` may import from `src/infrastructure/`. Violations fail `clippy`.

See the [Hexagonal Architecture (CLI)](../../repo-governance/development/pattern/hexagonal-architecture-cli.md) pattern for full rationale.

## Related

- [`specs/apps/rhino/`](../../specs/apps/rhino/README.md) — the behavior contract this CLI satisfies.
- [Nx Targets](../../repo-governance/development/infra/nx-targets.md) — the full target catalog.
