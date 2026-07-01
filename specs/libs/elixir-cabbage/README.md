---
title: "elixir-cabbage Specs"
description: Gherkin behavioral specifications for the elixir-cabbage story BDD library
category: specs
---

# elixir-cabbage Specs

Gherkin behavioral specifications for
[elixir-cabbage](../../../libs/elixir-cabbage/README.md), the OSE fork of
[cabbage-ex/cabbage](https://github.com/cabbage-ex/cabbage) — a story BDD tool that compiles
`.feature` files to ExUnit tests at compile time.

## Purpose

These specs define the **observable behavior** of `Cabbage.Feature`: given a `.feature` file and a
module that `use`s it with `defgiven`/`defwhen`/`defthen` step macros, what ExUnit test(s) the
compiler generates and how they execute.

## Structure

```
specs/libs/elixir-cabbage/
├── README.md
├── product/               # C4 L1 product framing
├── system-context/        # C4 L1 actors and consumers
├── containers/             # C4 L2 deployable units
├── components/             # C4 L3 component catalogue
└── behavior/
    └── gherkin/            # Gherkin feature files
        └── compile/
```

## Spec Artifacts

- **[product/](./product/README.md)** — C4 Level 1 product framing
- **[system-context/](./system-context/README.md)** — C4 Level 1 actors and consumers
- **[containers/](./containers/README.md)** — C4 Level 2 deployable units
- **[components/](./components/README.md)** — C4 Level 3 component catalogue
- **[behavior/](./behavior/README.md)** — Gherkin feature files

## Status

`test:unit` (`mix test`) exercises `Cabbage.Feature` directly via ExUnit tests under `test/`
(compiled from real `.feature` fixture files in `test/features/`); no separate Cucumber/Gherkin
runner consumes the scenario below — `specs:behavior:coverage` is an `echo` placeholder until
that lands. See [FORK_NOTES.md](../../../libs/elixir-cabbage/FORK_NOTES.md) for fork rationale.
