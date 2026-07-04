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
  alternative: a minimal, dependency-free, genuinely-executed step registry
  (`defgiven`/`defwhen`/`defthen`, the same call shape as Cabbage.Feature) that
  parses the REAL spec file via `Gherkin.parse_file/1` (dogfooding the library
  under test, same pattern as test/gherkin/gherkin_test.exs) and dispatches
  every one of its steps to a registered pattern — raising loudly if any step
  has no match, so this test fails (not silently passes) if the spec file's
  wording ever drifts from these bindings.
  """

  use ExUnit.Case

  @spec_path Path.expand(
               "../../../../specs/libs/elixir-gherkin/behavior/gherkin/parse/feature-parsing.feature",
               __DIR__
             )

  # --- minimal, dependency-free step registry (same call shape as Cabbage.Feature) ---

  defp defgiven(regex, fun), do: {regex, fun}
  defp defwhen(regex, fun), do: {regex, fun}
  defp defthen(regex, fun), do: {regex, fun}

  defp step_registry do
    g1 =
      defgiven(
        ~r/^the text of a \.feature file with one Feature and one Scenario with two steps$/,
        fn _state ->
          text = """
          Feature: Say hello

            Scenario: Greeting a user
              Given a user named "Alice"
              Then the greeting is "Hello, Alice"
          """

          %{text: text}
        end
      )

    w1 =
      defwhen(~r/^I call Gherkin\.parse on the text$/, fn %{text: text} = state ->
        Map.put(state, :feature, Gherkin.parse(text))
      end)

    t1 =
      defthen(~r/^the result is a Gherkin\.Elements\.Feature struct$/, fn %{feature: feature} =
                                                                            state ->
        assert %Gherkin.Elements.Feature{} = feature
        state
      end)

    t2 =
      defthen(~r/^the feature's scenarios list contains 1 scenario$/, fn %{feature: feature} =
                                                                           state ->
        assert length(feature.scenarios) == 1
        state
      end)

    t3 =
      defthen(~r/^that scenario's steps list contains 2 steps$/, fn %{feature: feature} = state ->
        assert length(hd(feature.scenarios).steps) == 2
        state
      end)

    g2 =
      defgiven(
        ~r/^a parsed feature containing a Scenario Outline with 3 Examples rows$/,
        fn _state ->
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
        end
      )

    w2 =
      defwhen(~r/^I call Gherkin\.flatten on the feature$/, fn %{feature: feature} = state ->
        Map.put(state, :flattened, Gherkin.flatten(feature))
      end)

    t4 =
      defthen(~r/^the flattened feature's scenarios list contains 3 scenarios$/, fn %{
                                                                                      flattened:
                                                                                        flattened
                                                                                    } = state ->
        assert length(flattened.scenarios) == 3
        state
      end)

    t5 =
      defthen(
        ~r/^each scenario's step text has its "<placeholder>" tokens replaced by the row's values$/,
        fn %{flattened: flattened} = state ->
          step_texts =
            Enum.flat_map(flattened.scenarios, fn s -> Enum.map(s.steps, & &1.text) end)

          refute Enum.any?(step_texts, &String.contains?(&1, "<"))
          state
        end
      )

    [g1, w1, t1, t2, t3, g2, w2, t4, t5]
  end

  defp run_step!(step_text, state) do
    case Enum.find(step_registry(), fn {regex, _fun} -> Regex.match?(regex, step_text) end) do
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
