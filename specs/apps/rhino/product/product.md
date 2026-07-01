# Product Overview: rhino-cli

Repository governance CLI for the `ose-public` / `ose-primer` monorepo. Validates documentation,
specifications, repository structure, and code quality gates that cannot be expressed in a
language-specific linter.

## Goals

- Enforce Gherkin coverage parity between specs and implementation across all projects
- Validate Mermaid diagrams, markdown links, and heading hierarchy at pre-commit and CI time
- Gate specs structure (five-folder C4 layout, DDD bounded-contexts registry, behavior feature files)
- Provide artifact generation utilities (e.g. link-status cache, naming audit reports)

## Scope

| Dimension  | In scope                                                                    |
| ---------- | --------------------------------------------------------------------------- |
| Commands   | `docs`, `specs`, `git`, `md`, `harness`, `env`, `links`, `names`, `version` |
| Targets    | All Nx projects in the repo workspace                                       |
| Deployment | CLI binary (`rhino-cli`); runs locally and in CI via `cargo run --release`  |

## Actors

- **Developer** — runs validators locally via pre-commit/pre-push hooks and on-demand
- **CI Pipeline** — invokes `rhino-cli` as part of `main-ci.yml` and `pr-quality-gate.yml`
- **Nx Targets** — `specs:behavior:coverage`, `specs:structure:validation`, `harness:duplication` targets

## Related

- **System context**: [context.md](../system-context/context.md)
- **Container diagram**: [container.md](../containers/container.md)
- **CLI component diagram**: [component-cli.md](../components/cli/component-cli.md)
- **Parent**: [rhino specs](../README.md)
