---
title: "elixir-openapi-codegen Specs"
description: Gherkin behavioral specifications for the elixir-openapi-codegen schema generator
category: specs
---

# elixir-openapi-codegen Specs

Gherkin behavioral specifications for
[elixir-openapi-codegen](../../../libs/elixir-openapi-codegen/mix.exs), the OpenAPI schema code
generator that produces Elixir struct modules from a bundled OpenAPI YAML spec.

## Purpose

These specs define the **observable behavior** of `OpenApiCodegen.generate/3`: given a bundled
OpenAPI spec path and an output directory, which `.ex` struct-module files are written, and what
happens when the spec has no `components.schemas` section.

## Structure

```
specs/libs/elixir-openapi-codegen/
├── README.md
├── product/               # C4 L1 product framing
├── system-context/        # C4 L1 actors and consumers
├── containers/             # C4 L2 deployable units
├── components/             # C4 L3 component catalogue
└── behavior/
    └── gherkin/            # Gherkin feature files
        └── generate/
```

## Spec Artifacts

- **[product/](./product/README.md)** — C4 Level 1 product framing
- **[system-context/](./system-context/README.md)** — C4 Level 1 actors and consumers
- **[containers/](./containers/README.md)** — C4 Level 2 deployable units
- **[components/](./components/README.md)** — C4 Level 3 component catalogue
- **[behavior/](./behavior/README.md)** — Gherkin feature files

## Status

`test:unit` (`mix test`) exercises `OpenApiCodegen.generate/3`, `Parser`, and `Generator` directly
via ExUnit tests under `test/` (against real fixture specs in `test/fixtures/`); no separate
Cucumber/Gherkin runner consumes the scenario below — `specs:behavior:coverage` is an `echo`
placeholder for this codegen library until that lands.
