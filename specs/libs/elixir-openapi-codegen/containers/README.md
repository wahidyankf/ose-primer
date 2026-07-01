---
title: "elixir-openapi-codegen — Containers"
description: C4 Level 2 Container diagram for elixir-openapi-codegen
category: specs
---

# Containers — elixir-openapi-codegen

C4 Level 2 containers for `elixir-openapi-codegen`.

`elixir-openapi-codegen` ships as a single container: a Mix application (`:openapi_codegen`)
invoked as a code-generation step (either via `mix run` or a consuming project's `codegen` Nx
target), reading a YAML file and writing `.ex` files to the local filesystem. It has no persistent
runtime — it is a build-time generator, not a long-running service.

See [container.md](./container.md) for the C4 container diagram placeholder.

## Related

- [elixir-openapi-codegen spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
- [components/](../components/README.md) — C4 Level 3
