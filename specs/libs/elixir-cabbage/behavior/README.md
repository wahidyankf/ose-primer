---
title: "elixir-cabbage — Behavior"
description: Index of behavioral specifications for elixir-cabbage
category: specs
---

# Behavior — elixir-cabbage

Gherkin behavioral specifications for
[elixir-cabbage](../../../../libs/elixir-cabbage/README.md), the OSE fork of the `cabbage` story
BDD library.

## Structure

```
specs/libs/elixir-cabbage/behavior/
└── gherkin/
    └── compile/
        └── feature-compilation.feature
```

## Status

No test runner currently consumes this scenario — `specs:behavior:coverage` is an `echo`
placeholder (see the top-level [README.md](../README.md#status)). `elixir-cabbage`'s own
correctness is instead exercised via `mix test`, which compiles real `.feature` fixture files
under `test/features/` (`simple.feature`, `outline.feature`, `tags.feature`, etc.) into ExUnit
tests via the very `Cabbage.Feature` macro this scenario documents.

## Related

- [elixir-cabbage spec root](../README.md)
- [components/](../components/README.md) — C4 Level 3 component catalogue
