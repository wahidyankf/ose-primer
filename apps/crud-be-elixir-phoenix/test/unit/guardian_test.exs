defmodule CrudBeExph.Auth.GuardianTest do
  use ExUnit.Case, async: true

  alias CrudBeExph.Auth.Guardian

  @moduletag :unit

  describe "env var naming convention" do
    test "runtime.exs consumes CRUD_BE_ELIXIR_PHOENIX_JWT_SECRET, not APP_JWT_SECRET" do
      # Verify the runtime config source reads from the namespaced env var.
      # config/test.exs supplies a static secret so the app boots without the env
      # var present during tests; this assertion documents the expected var name.
      runtime_source = File.read!(Path.expand("../../config/runtime.exs", __DIR__))
      assert String.contains?(runtime_source, "CRUD_BE_ELIXIR_PHOENIX_JWT_SECRET")
      refute String.contains?(runtime_source, "APP_JWT_SECRET")
    end
  end

  describe "subject_for_token/2" do
    test "returns ok tuple with stringified id for user map" do
      assert {:ok, "42"} = Guardian.subject_for_token(%{id: 42}, %{})
    end

    test "returns error tuple for unknown resource type" do
      assert {:error, :unknown_resource} = Guardian.subject_for_token("not_a_map", %{})
    end
  end

  describe "resource_from_claims/1" do
    test "returns error tuple for claims without sub key" do
      assert {:error, :missing_subject} = Guardian.resource_from_claims(%{})
    end
  end
end
