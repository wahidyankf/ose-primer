# Hexagonal Architecture — CLI Apps

Canonical hexagonal layer conventions for the two CLI apps in ose-primer:
`rhino-cli-rust` (Rust) and `rhino-cli-go` (Go).

See [Hexagonal Architecture](./hexagonal-architecture.md) for shared principles and
terminology that apply across all app types.

## Scope

| App                   | Language |
| --------------------- | -------- |
| `apps/rhino-cli-rust` | Rust     |
| `apps/rhino-cli-go`   | Go       |

## Rust CLI Layer Directories

Source root: `apps/rhino-cli-rust/src/`

| Layer                    | Directory             | Purpose                                                 |
| ------------------------ | --------------------- | ------------------------------------------------------- |
| Domain                   | `src/domain/`         | Pure business logic, no framework dependencies          |
| Application              | `src/application/`    | Use-case orchestration, port interfaces                 |
| Infrastructure           | `src/infrastructure/` | Outbound adapters (file system, external APIs)          |
| Entry point / CLI parser | `src/commands/`       | Inbound adapter — parses CLI flags, invokes application |

The `src/commands/` directory already exists and is the inbound adapter layer.

## Go CLI Layer Directories

Source root: `apps/rhino-cli-go/`

| Layer       | Directory                   | Purpose                                              |
| ----------- | --------------------------- | ---------------------------------------------------- |
| Domain      | `internal/domain/`          | Pure business logic, no framework dependencies       |
| Application | `internal/application/`     | Use-case orchestration, port interfaces              |
| CLI adapter | `internal/adapter/command/` | Inbound adapter — parses flags, invokes application  |
| Entry point | `cmd/`                      | `main` package; delegates to application via adapter |

The `cmd/` directory already exists as the entry point.

## Go `internal/` Compiler Constraint

Go's `internal/` wrapper is compiler-enforced: packages inside `internal/` cannot be
imported by code outside the module. All application code for `rhino-cli-go` lives under
`internal/` to prevent accidental external consumption of internal packages.

## Why `adapter/command/` and Not `infrastructure/`

For a CLI tool, the "infrastructure" concern is the command-line parser itself — it reads
flags and translates them into use-case calls. The hexagonal convention for CLI apps names
this adapter `adapter/command/` to make its inbound role explicit. This differs from backend
apps where outbound adapters are named `infrastructure/`.

## References

- [Hexagonal Architecture](./hexagonal-architecture.md) — shared principles and dependency rule
