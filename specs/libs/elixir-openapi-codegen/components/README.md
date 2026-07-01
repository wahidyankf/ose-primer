---
title: "elixir-openapi-codegen — Components"
description: C4 Level 3 Component catalogue for elixir-openapi-codegen
category: specs
---

# Components — elixir-openapi-codegen

C4 Level 3 components for `elixir-openapi-codegen`.

| Module                     | Export                                | Purpose                                                      |
| -------------------------- | ------------------------------------- | ------------------------------------------------------------ |
| `OpenApiCodegen`           | `generate/3`                          | Entry point: parses a spec and writes generated `.ex` files  |
| `OpenApiCodegen.Parser`    | `parse_file/1`, `parse_string/1`      | Reads/parses a bundled OpenAPI YAML spec into schema structs |
| `OpenApiCodegen.Generator` | `generate_module/1`, `write_module/3` | Renders and writes one `.ex` struct-module file per schema   |

See [../behavior/gherkin/generate/](../behavior/gherkin/generate/) for the behavioral spec.
See [component-elixir-openapi-codegen.md](./component-elixir-openapi-codegen.md) for the C4
component diagram placeholder.

## Related

- [elixir-openapi-codegen spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
- [containers/](../containers/README.md) — C4 Level 2
