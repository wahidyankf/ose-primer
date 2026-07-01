---
title: "clojure-openapi-codegen — Components"
description: C4 Level 3 Component catalogue for clojure-openapi-codegen
category: specs
---

# Components — clojure-openapi-codegen

C4 Level 3 components for `clojure-openapi-codegen`.

| Namespace                   | Export                                 | Purpose                                                      |
| --------------------------- | -------------------------------------- | ------------------------------------------------------------ |
| `openapi-codegen.core`      | `generate`                             | Entry point: parses a spec and writes generated `.clj` files |
| `openapi-codegen.parser`    | `parse-schemas`                        | Reads/parses a bundled OpenAPI YAML spec into schema maps    |
| `openapi-codegen.generator` | `openapi-type->malli`, `schema->malli` | Maps OpenAPI types/schemas to Malli `[:map ...]` forms       |
| `openapi-codegen.generator` | `generate-schema-files`                | Renders and writes one `.clj` schema file per parsed schema  |

See [../behavior/gherkin/generate/](../behavior/gherkin/generate/) for the behavioral spec.
See [component-clojure-openapi-codegen.md](./component-clojure-openapi-codegen.md) for the C4
component diagram placeholder.

## Related

- [clojure-openapi-codegen spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
- [containers/](../containers/README.md) — C4 Level 2
