---
title: "elixir-gherkin Specs"
description: Gherkin behavioral specifications for the elixir-gherkin feature-file parser
category: specs
---

# elixir-gherkin Specs

Gherkin behavioral specifications for
[elixir-gherkin](../../../libs/elixir-gherkin/README.md), the OSE fork of
[cabbage-ex/gherkin](https://github.com/cabbage-ex/gherkin) — a `.feature` file parser that
translates Gherkin text into native Elixir terms.

## Purpose

These specs define the **observable behavior** of `Gherkin.parse/1`: given the text of a
`.feature` file, what `%Gherkin.Elements.Feature{}` struct (name, description, scenarios, steps)
is produced, and how `Scenario Outline` examples are flattened into concrete scenarios.

## Structure

```
specs/libs/elixir-gherkin/
├── README.md
├── product/               # C4 L1 product framing
├── system-context/        # C4 L1 actors and consumers
├── containers/             # C4 L2 deployable units
├── components/             # C4 L3 component catalogue
└── behavior/
    └── gherkin/            # Gherkin feature files
        └── parse/
```

## Spec Artifacts

- **[product/](./product/README.md)** — C4 Level 1 product framing
- **[system-context/](./system-context/README.md)** — C4 Level 1 actors and consumers
- **[containers/](./containers/README.md)** — C4 Level 2 deployable units
- **[components/](./components/README.md)** — C4 Level 3 component catalogue
- **[behavior/](./behavior/README.md)** — Gherkin feature files

## Status

`test:unit` (`mix test`) exercises `Gherkin.parse/1` directly via ExUnit tests under `test/`
(parsing real fixture files, e.g. `test/fixtures/coffee.feature`); no separate Cucumber/Gherkin
runner consumes the scenario below — `specs:behavior:coverage` is an `echo` placeholder until
that lands. See [FORK_NOTES.md](../../../libs/elixir-gherkin/FORK_NOTES.md) for fork rationale.
