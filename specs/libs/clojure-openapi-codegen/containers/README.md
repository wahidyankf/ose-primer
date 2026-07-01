---
title: "clojure-openapi-codegen — Containers"
description: C4 Level 2 Container diagram for clojure-openapi-codegen
category: specs
---

# Containers — clojure-openapi-codegen

C4 Level 2 containers for `clojure-openapi-codegen`.

`clojure-openapi-codegen` ships as a single container: a `deps.edn`-based Clojure library
(`openapi-codegen.core`) invoked as a code-generation step (via `clojure -M -e`), reading a YAML
file and writing `.clj` files to the local filesystem. It has no persistent runtime — it is a
build-time generator, not a long-running service.

See [container.md](./container.md) for the C4 container diagram placeholder.

## Related

- [clojure-openapi-codegen spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
- [components/](../components/README.md) — C4 Level 3
