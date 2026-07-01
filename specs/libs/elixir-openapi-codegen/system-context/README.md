---
title: "elixir-openapi-codegen — System Context"
description: C4 Level 1 System Context diagram for elixir-openapi-codegen
category: specs
---

# System Context — elixir-openapi-codegen

C4 Level 1 system context for `elixir-openapi-codegen`.

## Actors and consumers

- **Elixir backend developers** — invoke `OpenApiCodegen.generate/3` (typically via a `mix`
  codegen task) against a bundled OpenAPI contract spec to produce struct modules consumed by
  their application code.
- **`crud-contracts`** — declared as an `implicitDependencies` entry in this library's
  `project.json`; the bundled OpenAPI YAML spec that `elixir-openapi-codegen` reads is produced by
  the contracts pipeline.
- **`rhino-cli`** — also listed as an `implicitDependencies` entry for governance/spec-tree
  validation orchestration.

`elixir-openapi-codegen` has no runtime dependency on any backend or network service; it reads a
YAML file from the local filesystem and writes `.ex` files to the local filesystem.

See [context.md](./context.md) for the C4 context diagram placeholder.

## Related

- [elixir-openapi-codegen spec root](../README.md)
- [containers/](../containers/README.md) — C4 Level 2
- [components/](../components/README.md) — C4 Level 3
