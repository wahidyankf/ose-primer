---
title: Hexagonal Architecture — CLI Apps
description: Hexagonal architecture specialization for CLI apps — commands as inbound adapters, layer responsibilities, and forbidden imports
category: explanation
subcategory: development
tags:
  - architecture
  - hexagonal
  - cli
  - rust
  - fsharp
created: 2026-05-26
---

# Hexagonal Architecture — CLI Apps

Canonical hexagonal layer conventions for the CLI app in ose-primer:
`rhino-cli` (Rust).

See [Hexagonal Architecture](./hexagonal-architecture.md) for shared principles and
terminology that apply across all app types.

## Scope

| App              | Language |
| ---------------- | -------- |
| `apps/rhino-cli` | Rust     |

## Rust CLI Layer Directories

Source root: `apps/rhino-cli/src/`

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

## rhino-cli Reference Layout

`apps/rhino-cli/src/` contains the canonical four-layer layout:

```
src/
├── domain/         # Pure business logic — no std::fs, no HTTP, no env reads
├── application/    # Use-case orchestration; calls domain; injects infra via trait
├── infrastructure/ # All I/O: filesystem, network, env reads
└── commands/       # Inbound adapter: parses CLI args, delegates to application
```

No file in `src/domain/` may import from `src/infrastructure/`. This constraint is
enforced by `clippy` — a cross-layer import is a lint error, not just a code smell.

## Shared-Kernel Rule

Types and traits shared between layers (e.g., result types, port trait definitions)
live in a `shared_kernel` or `ports` module accessible to all layers. They must
contain no I/O code. Place them in `src/domain/` or a sibling `src/ports/` module,
never in `src/infrastructure/`.

## Golden-Master Enforcement

Integration tests for rhino-cli commands compare CLI output against golden-master
snapshots in `tests/fixtures/`. A command's observable output (stdout, stderr, exit
code) is part of its Gherkin contract. Update the golden master when the contract
changes intentionally; a diff against the master is a CI-blocking violation when
unexpected.

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
