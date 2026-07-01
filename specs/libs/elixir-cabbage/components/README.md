---
title: "elixir-cabbage — Components"
description: C4 Level 3 Component catalogue for elixir-cabbage
category: specs
---

# Components — elixir-cabbage

C4 Level 3 components for `elixir-cabbage`.

| Module                               | Export                           | Purpose                                                      |
| ------------------------------------ | -------------------------------- | ------------------------------------------------------------ |
| `Cabbage.Feature`                    | `use Cabbage.Feature, file: ...` | Compiles a `.feature` file into ExUnit tests at compile time |
| `Cabbage.Feature`                    | `defgiven`, `defwhen`, `defthen` | Step-definition macros matched against parsed step text      |
| `Cabbage.Feature.Loader`             | `.feature` file discovery        | Resolves the configured features base path                   |
| `Cabbage.Feature.Parameter`          | parameter extraction             | Extracts `{string}`, `{int}`, etc. tokens from step text     |
| `Cabbage.Feature.ParameterType`      | custom parameter types           | Registers custom cucumber-expression parameter types         |
| `Cabbage.Feature.CucumberExpression` | cucumber expression matching     | Matches named-parameter step expressions (`{name}` syntax)   |
| `Cabbage.Feature.MissingStepError`   | compile-time error               | Raised when a `.feature` step has no matching macro clause   |
| `Cabbage`                            | `base_path/0`, `global_tags/0`   | Reads `:elixir_cabbage` application config                   |

See [../behavior/gherkin/compile/](../behavior/gherkin/compile/) for the behavioral spec.
See [component-elixir-cabbage.md](./component-elixir-cabbage.md) for the C4 component diagram
placeholder.

## Related

- [elixir-cabbage spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
- [containers/](../containers/README.md) — C4 Level 2
