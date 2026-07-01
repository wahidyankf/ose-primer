---
title: "elixir-cabbage — System Context"
description: C4 Level 1 System Context diagram for elixir-cabbage
category: specs
---

# System Context — elixir-cabbage

C4 Level 1 system context for `elixir-cabbage`.

## Actors and consumers

- **Elixir test authors** — `use Cabbage.Feature, file: "..."` in an ExUnit test module and
  implement `defgiven`/`defwhen`/`defthen` step clauses.
- **`elixir-gherkin`** — upstream parsing dependency (local path dependency, not a Hex package);
  `Cabbage.Feature` delegates all `.feature`-file parsing to `Gherkin.parse/1`.
- **`rhino-cli`** — listed as an `implicitDependencies` entry in `elixir-cabbage`'s `project.json`
  because Nx target orchestration (governance validation) touches this library's spec tree.

`elixir-cabbage` has no runtime dependency on any backend or network service; it operates purely
at Elixir compile time and during `mix test` execution.

See [context.md](./context.md) for the C4 context diagram placeholder.

## Related

- [elixir-cabbage spec root](../README.md)
- [containers/](../containers/README.md) — C4 Level 2
- [components/](../components/README.md) — C4 Level 3
