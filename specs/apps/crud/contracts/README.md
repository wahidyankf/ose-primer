# Demo API Contract

OpenAPI 3.1 specification for the crud expense tracker REST API.

## Purpose

This contract defines the exact shape of every request and response across all crud backends
(11 languages) and frontends (3 frameworks). It is the **single source of truth** for API types —
code generators produce language-specific types, encoders, and decoders from this spec.

## Quick Start

```bash
# Lint the contract (bundles first, then runs Spectral)
nx run crud-contracts:lint

# Bundle into single resolved YAML + JSON
nx run crud-contracts:bundle

# Generate browsable API documentation
nx run crud-contracts:docs
# Open specs/apps/crud/contracts/generated/docs/index.html
```

## File Structure

```
contracts/
├── openapi.yaml          # Root spec with $ref mappings
├── .spectral.yaml        # Linting rules (camelCase enforcement)
├── redocly.yaml          # Documentation theme config
├── project.json          # Nx project targets
├── paths/                # Endpoint definitions by domain
├── schemas/              # Data type definitions
├── examples/             # Example request/response pairs
└── generated/            # Output (gitignored)
    ├── openapi-bundled.yaml
    ├── openapi-bundled.json
    └── docs/index.html
```

## Modifying the Contract

1. Edit the relevant file in `schemas/` or `paths/`
2. Run `nx run crud-contracts:lint` to validate
3. Run `nx run crud-contracts:bundle` to regenerate the bundled spec
4. Run codegen for affected apps: `nx affected -t codegen`
5. Fix any compile errors in affected apps
6. Commit the contract changes (generated code is gitignored)

## Nx Cache Integration

Generated contract paths are explicit Nx cache inputs for `test:unit` and `test:quick` in all 11
backends and all 3 frontends. This ensures that re-running codegen (which changes the generated
files) triggers a cache miss and re-runs affected test targets.

The canonical input pattern used in backend `project.json` files:

```
"{projectRoot}/generated-contracts/**/*"
```

Python and Clojure backends use underscores (`generated_contracts`) to follow language naming
conventions. TypeScript frontends include `{projectRoot}/src/generated-contracts/**/*`. The
`codegen` target is also a `dependsOn` for both `typecheck` and `build` in every crud app.

## Adoption Status

The contract is consumed by all crud apps (11 backends, 3 frontends):

| App                       | Codegen target | generated-contracts in inputs |
| ------------------------- | :------------: | :---------------------------: |
| crud-be-golang-gin        |      yes       |              yes              |
| crud-be-java-springboot   |      yes       |              yes              |
| crud-be-java-vertx        |      yes       |              yes              |
| crud-be-elixir-phoenix    |      yes       |              yes              |
| crud-be-python-fastapi    |      yes       |              yes              |
| crud-be-rust-axum         |      yes       |              yes              |
| crud-be-fsharp-giraffe    |      yes       |              yes              |
| crud-be-ts-effect         |      yes       |              yes              |
| crud-be-kotlin-ktor       |      yes       |              yes              |
| crud-be-csharp-aspnetcore |      yes       |              yes              |
| crud-be-clojure-pedestal  |      yes       |              yes              |
| crud-fe-ts-nextjs         |      yes       |              yes              |
| crud-fe-ts-tanstack-start |      yes       |              yes              |
| crud-fe-dart-flutterweb   |      yes       |              yes              |

## Rules

- All JSON field names use **strict camelCase** — zero exceptions
- Every schema must have a `description`
- Changes to this contract trigger codegen for all crud apps via Nx dependency graph
