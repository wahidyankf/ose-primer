---
title: "elixir-openapi-codegen — Behavior"
description: Index of behavioral specifications for elixir-openapi-codegen
category: specs
---

# Behavior — elixir-openapi-codegen

Gherkin behavioral specifications for
[elixir-openapi-codegen](../../../../libs/elixir-openapi-codegen/mix.exs), the OpenAPI schema
code generator.

## Structure

```
specs/libs/elixir-openapi-codegen/behavior/
└── gherkin/
    └── generate/
        └── generate-schema-modules.feature
```

## Status

No test runner currently consumes this scenario — `specs:behavior:coverage` is an `echo`
placeholder for this codegen library (see the top-level [README.md](../README.md#status)).
`elixir-openapi-codegen`'s own correctness is instead exercised via `mix test`, which runs
`OpenApiCodegen.generate/3` against real fixture specs under `test/fixtures/` (`sample.yaml`,
`no_components.yaml`, `no_schemas.yaml`).

## Related

- [elixir-openapi-codegen spec root](../README.md)
- [components/](../components/README.md) — C4 Level 3 component catalogue
