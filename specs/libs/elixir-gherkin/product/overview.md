# elixir-gherkin — Product Overview

`elixir-gherkin` provides the `Gherkin` module:

- `Gherkin.parse(string_or_stream)` — parses the full text (or a file stream) of a `.feature`
  file into a `%Gherkin.Elements.Feature{}` struct containing its name, description, and a list of
  `Scenario`/`ScenarioOutline` structs, each with an ordered list of `Step` structs
  (`keyword`, `text`, `line`).
- `Gherkin.parse_file(file_name)` — reads the file at `file_name` and delegates to `parse/1`.
- `Gherkin.flatten(feature)` — expands every `ScenarioOutline` in a parsed feature into one
  concrete `Scenario` per `Examples` row, substituting `<placeholder>` tokens with each row's
  values, so all scenarios can be executed uniformly.
- `Gherkin.scenarios_for(outline)` — the underlying per-outline expansion function used by
  `flatten/1`.

`elixir-gherkin` is the OSE fork's local replacement for the `gherkin` Hex package, forked
2026-03-09 because upstream `cabbage-ex/gherkin` had gone dormant since 2023; see
[FORK_NOTES.md](../../../../libs/elixir-gherkin/FORK_NOTES.md).

See [README.md](./README.md) for C4 L1 product framing.
