---
title: "clojure-openapi-codegen Specs"
description: Gherkin behavioral specifications for the clojure-openapi-codegen schema generator
category: specs
---

# clojure-openapi-codegen Specs

Gherkin behavioral specifications for
[clojure-openapi-codegen](../../../libs/clojure-openapi-codegen/deps.edn), the OpenAPI schema code
generator that produces Malli schema files from a bundled OpenAPI YAML spec.

## Purpose

These specs define the **observable behavior** of `openapi-codegen.core/generate`: given a bundled
OpenAPI spec path and an output directory, which Malli `[:map ...]` schema `.clj` files are
written, and how OpenAPI types map to Malli types.

## Structure

```
specs/libs/clojure-openapi-codegen/
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

`test:unit` (`clojure -M:test -m kaocha.runner unit`) exercises `openapi-codegen.core/generate`,
`parser`, and `generator` directly via `clojure.test` under `test/`; no separate Cucumber/Gherkin
runner consumes the scenario below — `specs:behavior:coverage` is an `echo` placeholder for this
codegen library until that lands.
