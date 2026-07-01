---
title: "elixir-gherkin — Components"
description: C4 Level 3 Component catalogue for elixir-gherkin
category: specs
---

# Components — elixir-gherkin

C4 Level 3 components for `elixir-gherkin`.

| Module                             | Export                               | Purpose                                                      |
| ---------------------------------- | ------------------------------------ | ------------------------------------------------------------ |
| `Gherkin`                          | `parse/1`, `parse_file/1`            | Parses `.feature` text/stream/file into a `Feature` struct   |
| `Gherkin`                          | `flatten/1`, `scenarios_for/1`       | Expands `ScenarioOutline` examples into concrete `Scenario`s |
| `Gherkin.Elements.Feature`         | struct                               | Parsed feature: name, description, line, scenarios           |
| `Gherkin.Elements.Scenario`        | struct                               | Parsed scenario: name, tags, line, steps                     |
| `Gherkin.Elements.ScenarioOutline` | struct                               | Parsed outline: name, tags, steps, examples                  |
| `Gherkin.Elements.Step`            | struct                               | Parsed step: keyword, text, line                             |
| `Gherkin.Elements.Rule`            | struct                               | Parsed `Rule:` block (groups related scenarios)              |
| `Gherkin.Parser`                   | `parse_feature/1`, `parse_feature/2` | Top-level parser entry point invoked by `Gherkin.parse/1`    |

See [../behavior/gherkin/parse/](../behavior/gherkin/parse/) for the behavioral spec.
See [component-elixir-gherkin.md](./component-elixir-gherkin.md) for the C4 component diagram
placeholder.

## Related

- [elixir-gherkin spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
- [containers/](../containers/README.md) — C4 Level 2
