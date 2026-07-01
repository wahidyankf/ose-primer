---
title: "elixir-gherkin — Containers"
description: C4 Level 2 Container diagram for elixir-gherkin
category: specs
---

# Containers — elixir-gherkin

C4 Level 2 containers for `elixir-gherkin`.

`elixir-gherkin` ships as a single container: a Mix application (`:elixir_gherkin`) compiled into
each consuming Elixir project at build time via a local path dependency. Its only consumer today
is `elixir-cabbage`. It has no separate deployable runtime of its own — it is a pure parsing
library, not an executable.

See [container.md](./container.md) for the C4 container diagram placeholder.

## Related

- [elixir-gherkin spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
- [components/](../components/README.md) — C4 Level 3
