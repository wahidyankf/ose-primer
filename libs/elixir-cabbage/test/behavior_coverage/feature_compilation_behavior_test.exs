defmodule Cabbage.FeatureCompilationBehaviorTest do
  @moduledoc """
  Real Cabbage.Feature (BDD step-registration) binding for
  specs/libs/elixir-cabbage/behavior/gherkin/compile/feature-compilation.feature.

  elixir-cabbage IS the Cabbage BDD framework, so this file dogfoods itself the
  same way test/feature_execution_test.exs and test/feature_suggestion_test.exs
  already do: `use Cabbage.Feature` compiles the actual scenarios below into
  real ExUnit tests, and each `defwhen`/`defthen` step dynamically compiles a
  second, nested `use Cabbage.Feature` module to exercise the exact compile-time
  behavior the spec describes (all-steps-matched vs. a-step-is-missing),
  reusing the framework's own `CabbageTestHelper` runner (test/test_helper.exs)
  and the `simple.feature` fixture already used by feature_suggestion_test.exs.
  Nothing here replaces feature_suggestion_test.exs — it stays as-is; this adds
  literal step-text coverage on top of it.

  `file:` below uses a `../../../../` prefix instead of a bare feature-file
  name: `Cabbage.base_path/0` defaults to `"test/features/"` (this project's
  OWN existing fixtures use that default, e.g. `file: "simple.feature"`), and
  it cannot be repointed at the specs/ tree here without breaking those 8
  existing test files, which all rely on the default. Escaping it locally with
  `..` (verified: resolves to
  `specs/libs/elixir-cabbage/behavior/gherkin/compile/feature-compilation.feature`
  from this project's `mix test` cwd) is the narrower, non-breaking fix.

  This file lives in its own `test/behavior_coverage/` directory (not
  `test/` directly) so the coverage-checker's app-dir argument can point at
  just this directory: elixir-cabbage's OWN `test/*.exs` fixtures declare
  dozens of unrelated `defgiven`/`defwhen`/`defthen` step patterns (e.g. "I
  provide Given") to exercise the framework mechanics, which the checker's
  regex-based extractor would otherwise flag as orphan step implementations
  against this specific 2-scenario feature file.
  """

  use Cabbage.Feature,
    file: "../../../../specs/libs/elixir-cabbage/behavior/gherkin/compile/feature-compilation.feature"

  # @covers specs/libs/elixir-cabbage/behavior/gherkin/compile/feature-compilation.feature:A scenario with all steps matched compiles into a passing ExUnit test
  defgiven ~r/^a \.feature file with a scenario whose every step matches a defgiven, defwhen, or defthen clause$/,
           _vars,
           state do
    {:ok, Map.put(state, :every_step_matches?, true)}
  end

  # @covers specs/libs/elixir-cabbage/behavior/gherkin/compile/feature-compilation.feature:A step with no matching macro clause fails at compile time
  defgiven ~r/^a \.feature file with a step whose text matches no defgiven, defwhen, or defthen clause$/,
           _vars,
           state do
    {:ok, Map.put(state, :every_step_matches?, false)}
  end

  defwhen ~r/^the consuming module compiles with "use Cabbage\.Feature, file: \.\.\."$/,
          _vars,
          %{every_step_matches?: matches?} = state do
    if matches? do
      defmodule FullyMatchedCompilationCase do
        use Cabbage.Feature, file: "simple.feature"

        # Plain-string (Cucumber Expression) patterns, not `~r/.../` sigils, so
        # this throwaway fixture-double module's steps (mirroring simple.feature's
        # own wording) are never mistaken by rhino-cli's regex-based Elixir
        # extractor for step definitions covering THIS feature file's scenarios.
        defgiven("I provide Given", _vars, _state, do: nil)
        defgiven("I provide And", _vars, _state, do: nil)
        defwhen("I provide When", _vars, _state, do: nil)
        defthen("I provide Then", _vars, _state, do: nil)
      end

      {result, _output} = CabbageTestHelper.run([], [FullyMatchedCompilationCase])
      {:ok, Map.put(state, :run_result, result)}
    else
      error =
        assert_raise Cabbage.Feature.MissingStepError, fn ->
          defmodule MissingStepCompilationCase do
            use Cabbage.Feature, file: "simple.feature"
            # Deliberately omits a "Then" step so compilation raises.

            defgiven("I provide Given", _vars, _state, do: nil)
            defgiven("I provide And", _vars, _state, do: nil)
            defwhen("I provide When", _vars, _state, do: nil)
          end
        end

      {:ok, Map.put(state, :raised_error, error)}
    end
  end

  defthen ~r/^one ExUnit test is generated for the scenario$/, _vars, %{run_result: result} = state do
    assert result.total == 1
    {:ok, state}
  end

  defthen ~r/^running "mix test" passes that generated test$/, _vars, %{run_result: result} = state do
    assert result.failures == 0
    {:ok, state}
  end

  defthen ~r/^compilation raises a Cabbage\.Feature\.MissingStepError$/, _vars, %{raised_error: error} = state do
    assert %Cabbage.Feature.MissingStepError{} = error
    {:ok, state}
  end
end
