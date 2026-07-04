defmodule OpenApiCodegen.GenerateSchemaModulesBehaviorTest do
  @moduledoc """
  Real Cabbage.Feature (BDD step-registration) binding for
  specs/libs/elixir-openapi-codegen/behavior/gherkin/generate/generate-schema-modules.feature.

  Every step below is dispatched from the literal Gherkin text via
  `defgiven`/`defwhen`/`defthen` regex matching (see config/config.exs for the
  `:elixir_cabbage, :features` base path pointing at the specs/ tree) — the
  same convention already adopted by apps/crud-be-elixir-phoenix. Assertions
  mirror test/openapi_codegen_test.exs's plain-ExUnit coverage for these three
  scenarios; nothing is weakened.
  """

  use Cabbage.Feature, file: "generate/generate-schema-modules.feature"

  alias OpenApiCodegen

  @fixtures_dir Path.expand("fixtures", __DIR__)

  setup do
    tmp_dir =
      System.tmp_dir!() |> Path.join("codegen_behavior_test_#{System.unique_integer([:positive])}")

    File.mkdir_p!(tmp_dir)
    on_exit(fn -> File.rm_rf!(tmp_dir) end)
    {:ok, %{tmp_dir: tmp_dir}}
  end

  # @covers specs/libs/elixir-openapi-codegen/behavior/gherkin/generate/generate-schema-modules.feature:Generating a schema with required and optional properties
  defgiven ~r/^a bundled OpenAPI spec whose "(?<schema>[^"]+)" schema requires "(?<f1>[^"]+)", "(?<f2>[^"]+)", and "(?<f3>[^"]+)"$/,
           %{schema: schema, f1: f1, f2: f2, f3: f3},
           state do
    {:ok,
     Map.merge(state, %{
       spec_path: Path.join(@fixtures_dir, "sample.yaml"),
       schema_name: schema,
       required_fields: [f1, f2, f3]
     })}
  end

  defwhen ~r/^I call OpenApiCodegen\.generate with the spec path, an output directory, and namespace "(?<namespace>[^"]+)"$/,
          %{namespace: namespace},
          %{spec_path: spec_path, tmp_dir: tmp_dir} = state do
    {:ok, Map.put(state, :result, OpenApiCodegen.generate(spec_path, tmp_dir, namespace: namespace))}
  end

  defthen ~r/^the result is "\{:ok, paths\}" with one written file path per schema$/,
          _vars,
          %{result: result} = state do
    assert {:ok, paths} = result
    assert length(paths) == 3
    {:ok, Map.put(state, :paths, paths)}
  end

  defthen ~r/^the generated "(?<file_name>[^"]+)" file declares "@enforce_keys \[[^\]]*\]"$/,
          %{file_name: file_name},
          %{paths: paths, required_fields: required_fields} = state do
    file_path = Enum.find(paths, &String.ends_with?(&1, file_name))
    assert file_path != nil, "Expected #{file_name} among generated paths: #{inspect(paths)}"

    content = File.read!(file_path)
    assert String.contains?(content, "@enforce_keys")
    Enum.each(required_fields, fn field -> assert String.contains?(content, ":#{field}") end)
    {:ok, state}
  end

  # @covers specs/libs/elixir-openapi-codegen/behavior/gherkin/generate/generate-schema-modules.feature:A spec with no components key fails to generate
  defgiven ~r/^a bundled OpenAPI spec with no "components" key$/, _vars, state do
    {:ok,
     Map.merge(state, %{
       spec_path: Path.join(@fixtures_dir, "no_components.yaml"),
       expected_reason_substring: "components"
     })}
  end

  # @covers specs/libs/elixir-openapi-codegen/behavior/gherkin/generate/generate-schema-modules.feature:A spec with components but no schemas key fails to generate
  defgiven ~r/^a bundled OpenAPI spec with a "components" key but no "schemas" key$/, _vars, state do
    {:ok,
     Map.merge(state, %{
       spec_path: Path.join(@fixtures_dir, "no_schemas.yaml"),
       expected_reason_substring: "components.schemas"
     })}
  end

  defwhen ~r/^I call OpenApiCodegen\.generate with the spec path and an output directory$/,
          _vars,
          %{spec_path: spec_path, tmp_dir: tmp_dir} = state do
    {:ok, Map.put(state, :result, OpenApiCodegen.generate(spec_path, tmp_dir))}
  end

  defthen ~r/^the result is "\{:error, reason\}"$/,
          _vars,
          %{result: result, expected_reason_substring: substring} = state do
    assert {:error, reason} = result
    assert reason =~ substring
    {:ok, state}
  end
end
