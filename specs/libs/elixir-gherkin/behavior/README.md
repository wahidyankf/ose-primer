---
title: "elixir-gherkin — Behavior"
description: Index of behavioral specifications for elixir-gherkin
category: specs
---

# Behavior — elixir-gherkin

Gherkin behavioral specifications for
[elixir-gherkin](../../../../libs/elixir-gherkin/README.md), the OSE fork of the `gherkin`
`.feature`-file parser.

## Structure

```
specs/libs/elixir-gherkin/behavior/
└── gherkin/
    └── parse/
        └── feature-parsing.feature
```

## Status

No test runner currently consumes this scenario — `specs:behavior:coverage` is an `echo`
placeholder (see the top-level [README.md](../README.md#status)). `elixir-gherkin`'s own
correctness is instead exercised via `mix test`, which parses real fixture files (e.g.
`test/fixtures/coffee.feature`) through the very `Gherkin.parse/1` function this scenario
documents.

## Related

- [elixir-gherkin spec root](../README.md)
- [components/](../components/README.md) — C4 Level 3 component catalogue
