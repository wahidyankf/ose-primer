---
title: "elixir-gherkin — System Context"
description: C4 Level 1 System Context diagram for elixir-gherkin
category: specs
---

# System Context — elixir-gherkin

C4 Level 1 system context for `elixir-gherkin`.

## Actors and consumers

- **`elixir-cabbage`** — its sole consumer in this workspace; depends on `elixir-gherkin` via a
  local path dependency (`{:elixir_gherkin, path: "../../libs/elixir-gherkin"}`) and calls
  `Gherkin.parse/1`/`Gherkin.flatten/1` inside `Cabbage.Feature`'s compile-time macro expansion.
- **Elixir developers** — may call `Gherkin.parse/1` or `Gherkin.parse_file/1` directly to inspect
  a `.feature` file's structure outside of `Cabbage.Feature`.

`elixir-gherkin` has no runtime dependency on any backend or network service, and (unlike
`elixir-cabbage`) has zero external Hex dependencies at runtime.

See [context.md](./context.md) for the C4 context diagram placeholder.

## Related

- [elixir-gherkin spec root](../README.md)
- [containers/](../containers/README.md) — C4 Level 2
- [components/](../components/README.md) — C4 Level 3
