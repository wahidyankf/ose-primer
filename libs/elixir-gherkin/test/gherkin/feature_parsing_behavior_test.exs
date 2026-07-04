defmodule Gherkin.FeatureParsingBehaviorTest.Steps do
  @moduledoc """
  A minimal, dependency-free step-registration DSL with the same do-block
  call shape as `Cabbage.Feature`'s `defgiven`/`defwhen`/`defthen`, so
  `mix format` never rewraps these calls into a parenthesized form that
  rhino-cli's Elixir step-text extractor can't match. See the moduledoc on
  `Gherkin.FeatureParsingBehaviorTest` below for the full explanation.
  """

  defmacro defgiven(regex, do: block), do: step_ast(regex, block)
  defmacro defwhen(regex, do: block), do: step_ast(regex, block)
  defmacro defthen(regex, do: block), do: step_ast(regex, block)

  # `var!(state)` binds non-hygienically so the literal `state` references
  # inside each caller-supplied `block` (their own AST, not part of this
  # quote) resolve to the same variable this macro introduces.
  defp step_ast(regex, block) do
    quote do
      {unquote(regex), fn var!(state) -> unquote(block) end}
    end
  end

  # Plain-raise assertion helpers: `ExUnit.Assertions.assert/1` is a macro,
  # not callable as a remote function without `require`, and this module
  # deliberately stays dependency-free (no `require ExUnit.Assertions`).
  def check!(true), do: :ok
  def check!(false), do: raise("assertion failed")

  def registry do
    [
      defgiven ~r/^the text of a \.feature file with one Feature and one Scenario with two steps$/ do
        _ = state

        text = """
        Feature: Say hello

          Scenario: Greeting a user
            Given a user named "Alice"
            Then the greeting is "Hello, Alice"
        """

        %{text: text}
      end,
      defwhen ~r/^I call Gherkin\.parse on the text$/ do
        %{text: text} = state
        Map.put(state, :feature, Gherkin.parse(text))
      end,
      defthen ~r/^the result is a Gherkin\.Elements\.Feature struct$/ do
        %{feature: %Gherkin.Elements.Feature{}} = state
        state
      end,
      defthen ~r/^the feature's scenarios list contains 1 scenario$/ do
        %{feature: feature} = state
        check!(length(feature.scenarios) == 1)
        state
      end,
      defthen ~r/^that scenario's steps list contains 2 steps$/ do
        %{feature: feature} = state
        check!(length(hd(feature.scenarios).steps) == 2)
        state
      end,
      defgiven ~r/^a parsed feature containing a Scenario Outline with 3 Examples rows$/ do
        _ = state

        text = """
        Feature: Serve coffee

          Scenario Outline: Buy coffee
            Given there are <coffees> coffees left in the machine
            Then I should be served <served> coffees

            Examples:
              | coffees | served |
              |  12     |  12    |
              |  2      |  2     |
              |  0      |  0     |
        """

        %{feature: Gherkin.parse(text)}
      end,
      defwhen ~r/^I call Gherkin\.flatten on the feature$/ do
        %{feature: feature} = state
        Map.put(state, :flattened, Gherkin.flatten(feature))
      end,
      defthen ~r/^the flattened feature's scenarios list contains 3 scenarios$/ do
        %{flattened: flattened} = state
        check!(length(flattened.scenarios) == 3)
        state
      end,
      defthen ~r/^each scenario's step text has its "<placeholder>" tokens replaced by the row's values$/ do
        %{flattened: flattened} = state
        step_texts = Enum.flat_map(flattened.scenarios, fn s -> Enum.map(s.steps, & &1.text) end)
        check!(!Enum.any?(step_texts, &String.contains?(&1, "<")))
        state
      end
    ]
  end
end

defmodule Gherkin.FeatureParsingBehaviorTest do
  @moduledoc """
  Real, executed BDD step-registration binding for
  specs/libs/elixir-gherkin/behavior/gherkin/parse/feature-parsing.feature.

  elixir-gherkin cannot take `elixir_cabbage` as a test dependency: Cabbage's
  own mix.exs depends on `elixir_gherkin` via
  `{:elixir_gherkin, path: "../../libs/elixir-gherkin"}` — a hard, non-test-only
  production dependency (Cabbage needs a Gherkin parser to parse `.feature`
  files). Adding `elixir_cabbage` here therefore points straight back at this
  exact project. This is empirically confirmed circular, not just theorized:
  temporarily adding the dependency and running `mix deps.get` fails with

      Error while loading project :elixir_gherkin at .../libs/elixir-gherkin
      ** (Mix) Trying to load ElixirGherkin.Mixfile from ".../mix.exs" but
      another project with the same name was already defined at ".../mix.exs"

  No BDD framework built on top of Gherkin parsing can test the Gherkin parser
  itself without this cycle, so this module is the narrower, honest
  alternative: `Steps` above is a minimal, dependency-free step registry
  using real macros with the exact do-block call shape `Cabbage.Feature`
  itself uses (real macros, not plain functions, because `mix format`
  unconditionally parenthesizes plain multi-arg function calls — verified
  directly, even a short two-arg call gets rewritten `foo(a, b)` — which
  breaks rhino-cli's `ex_step_re()` extractor; a macro invoked with a
  `do...end` block is left unparenthesized by `mix format`, also verified
  directly, matching why the Cabbage-based bindings elsewhere in this repo
  never hit this problem).

  This test parses the REAL spec file via `Gherkin.parse_file/1` (dogfooding
  the library under test, same pattern as test/gherkin/gherkin_test.exs) and
  dispatches every one of its steps to a registered pattern — raising loudly
  if any step has no match, so this test fails (not silently passes) if the
  spec file's wording ever drifts from these bindings.
  """

  use ExUnit.Case
  alias Gherkin.FeatureParsingBehaviorTest.Steps

  @spec_path Path.expand(
               "../../../../specs/libs/elixir-gherkin/behavior/gherkin/parse/feature-parsing.feature",
               __DIR__
             )

  defp run_step!(step_text, state) do
    case Enum.find(Steps.registry(), fn {regex, _fun} -> Regex.match?(regex, step_text) end) do
      {_regex, fun} -> fun.(state)
      nil -> flunk("no registered step definition matches: #{inspect(step_text)}")
    end
  end

  defp scenario_step_texts!(scenario_title) do
    feature = Gherkin.parse_file(@spec_path)

    scenario =
      Enum.find(feature.scenarios, &(&1.name == scenario_title)) ||
        flunk("scenario not found in spec file: #{inspect(scenario_title)}")

    Enum.map(scenario.steps, & &1.text)
  end

  # @covers specs/libs/elixir-gherkin/behavior/gherkin/parse/feature-parsing.feature:Parsing a simple feature returns its name and scenarios
  test "Parsing a simple feature returns its name and scenarios" do
    "Parsing a simple feature returns its name and scenarios"
    |> scenario_step_texts!()
    |> Enum.reduce(%{}, &run_step!/2)
  end

  # @covers specs/libs/elixir-gherkin/behavior/gherkin/parse/feature-parsing.feature:Flattening a Scenario Outline expands one scenario per example row
  test "Flattening a Scenario Outline expands one scenario per example row" do
    "Flattening a Scenario Outline expands one scenario per example row"
    |> scenario_step_texts!()
    |> Enum.reduce(%{}, &run_step!/2)
  end
end
