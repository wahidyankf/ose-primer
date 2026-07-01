---
title: "clojure-openapi-codegen — Behavior"
description: Index of behavioral specifications for clojure-openapi-codegen
category: specs
---

# Behavior — clojure-openapi-codegen

Gherkin behavioral specifications for
[clojure-openapi-codegen](../../../../libs/clojure-openapi-codegen/deps.edn), the OpenAPI schema
code generator.

## Structure

```
specs/libs/clojure-openapi-codegen/behavior/
└── gherkin/
    └── generate/
        └── generate-schema-files.feature
```

## Status

No test runner currently consumes this scenario — `specs:behavior:coverage` is an `echo`
placeholder for this codegen library (see the top-level [README.md](../README.md#status)).
`clojure-openapi-codegen`'s own correctness is instead exercised via
`clojure -M:test -m kaocha.runner unit`, which runs `openapi-codegen.core/generate` and its
namespaces against fixtures under `test/openapi_codegen/`.

## Related

- [clojure-openapi-codegen spec root](../README.md)
- [components/](../components/README.md) — C4 Level 3 component catalogue
