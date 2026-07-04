import Config

# Elixir Cabbage BDD — feature files relative to workspace root.
# Same convention as apps/crud-be-elixir-phoenix/config/test.exs.
config :elixir_cabbage,
  features: Path.expand("../../../specs/libs/elixir-openapi-codegen/behavior/gherkin/", __DIR__) <> "/"
