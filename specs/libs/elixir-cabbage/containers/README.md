---
title: "elixir-cabbage — Containers"
description: C4 Level 2 Container diagram for elixir-cabbage
category: specs
---

# Containers — elixir-cabbage

C4 Level 2 containers for `elixir-cabbage`.

`elixir-cabbage` ships as a single container: a Mix application (`:elixir_cabbage`) compiled into
each consuming Elixir project at build time via a local path dependency
(`{:elixir_cabbage, path: "../../libs/elixir-cabbage"}`). It has no separate deployable runtime of
its own — it is a compile-time test-generation library, not an executable.

See [container.md](./container.md) for the C4 container diagram placeholder.

## Related

- [elixir-cabbage spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
- [components/](../components/README.md) — C4 Level 3
