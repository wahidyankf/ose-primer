---
title: "clojure-openapi-codegen — System Context"
description: C4 Level 1 System Context diagram for clojure-openapi-codegen
category: specs
---

# System Context — clojure-openapi-codegen

C4 Level 1 system context for `clojure-openapi-codegen`.

## Actors and consumers

- **Clojure backend developers** — invoke `openapi-codegen.core/generate` (typically via a build
  alias) against a bundled OpenAPI contract spec to produce Malli schema files consumed by their
  application code.
- **`crud-contracts`** — declared as an `implicitDependencies` entry in this library's
  `project.json`; the bundled OpenAPI YAML spec that `clojure-openapi-codegen` reads is produced
  by the contracts pipeline.
- **`rhino-cli`** — also listed as an `implicitDependencies` entry for governance/spec-tree
  validation orchestration.

`clojure-openapi-codegen` has no runtime dependency on any backend or network service; it reads a
YAML file from the local filesystem (via SnakeYAML) and writes `.clj` files to the local
filesystem.

See [context.md](./context.md) for the C4 context diagram placeholder.

## Related

- [clojure-openapi-codegen spec root](../README.md)
- [containers/](../containers/README.md) — C4 Level 2
- [components/](../components/README.md) — C4 Level 3
