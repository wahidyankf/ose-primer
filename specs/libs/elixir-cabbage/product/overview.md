# elixir-cabbage — Product Overview

`elixir-cabbage` provides `Cabbage.Feature`, a `use`-able macro that:

1. Reads a `.feature` file from the configured `features` base path
   (`config :elixir_cabbage, features: "specs/apps/my-app/"`).
2. Parses it via [`elixir-gherkin`](../../elixir-gherkin/README.md) (`Gherkin.parse/1`,
   `Gherkin.flatten/1`) into `Scenario`/`Step` structs, expanding any `Scenario Outline` into one
   scenario per example row.
3. Generates one ExUnit `test` per scenario at compile time, executing each step in order by
   matching its text against the consuming module's `defgiven ~r/.../`, `defwhen ~r/.../`, and
   `defthen ~r/.../` clauses (via `Cabbage.Feature.CucumberExpression` for named-parameter
   expressions, or plain regex).
4. Raises `Cabbage.Feature.MissingStepError` at compile time if a step in the `.feature` file has
   no matching macro clause in the consuming module.

`elixir-cabbage` is the OSE fork's local replacement for the `cabbage` Hex package, forked
2026-03-09 because upstream `cabbage-ex/cabbage` had gone dormant; see
[FORK_NOTES.md](../../../../libs/elixir-cabbage/FORK_NOTES.md).

See [README.md](./README.md) for C4 L1 product framing.
