# Hexagonal Architecture — CLI Apps

Canonical hexagonal layer conventions for the CLI app in ose-primer:
`rhino-cli-rust` (Rust).

See [Hexagonal Architecture](./hexagonal-architecture.md) for shared principles and
terminology that apply across all app types.

## Scope

| App                   | Language |
| --------------------- | -------- |
| `apps/rhino-cli-rust` | Rust     |

## Rust CLI Layer Directories

Source root: `apps/rhino-cli-rust/src/`

| Layer                    | Directory             | Purpose                                                 |
| ------------------------ | --------------------- | ------------------------------------------------------- |
| Domain                   | `src/domain/`         | Pure business logic, no framework dependencies          |
| Application              | `src/application/`    | Use-case orchestration, port interfaces                 |
| Infrastructure           | `src/infrastructure/` | Outbound adapters (file system, external APIs)          |
| Entry point / CLI parser | `src/commands/`       | Inbound adapter — parses CLI flags, invokes application |

The `src/commands/` directory already exists and is the inbound adapter layer.

## Why `commands/` and Not `infrastructure/`

For a CLI tool, the "infrastructure" concern is the command-line parser itself — it reads
flags and translates them into use-case calls. The hexagonal convention for CLI apps names
this inbound adapter `commands/` to make its inbound role explicit. This differs from backend
apps where outbound adapters are named `infrastructure/`.

## References

- [Hexagonal Architecture](./hexagonal-architecture.md) — shared principles and dependency rule

## Principles Implemented/Respected

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)** —
  Consistent layer names minimize cognitive overhead when navigating the CLI codebase.
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)** —
  The `commands/` name makes the inbound adapter role explicit for CLI tools, distinguishing
  it clearly from backend outbound adapters named `infrastructure/`.

## Conventions Implemented/Respected

- **[Hexagonal Architecture](./hexagonal-architecture.md)** — This document specializes the
  shared dependency rule and terminology for CLI application structure (Rust).
