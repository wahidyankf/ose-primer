defmodule CrudBeExph.MixProject do
  use Mix.Project

  def project do
    [
      app: :crud_be_exph,
      version: "0.1.0",
      elixir: "~> 1.15",
      elixirc_paths: elixirc_paths(Mix.env()),
      start_permanent: Mix.env() == :prod,
      aliases: aliases(),
      deps: deps(),
      test_coverage: [tool: ExCoveralls],
      test_paths: test_paths(Mix.env()),
      test_pattern: "**/*_{test,steps}.exs",
      test_load_filters: [~r/_test\.exs$/, ~r/_steps\.exs$/]
    ]
  end

  # Configuration for the OTP application.
  def application do
    [
      mod: {CrudBeExph.Application, []},
      extra_applications: [:logger, :runtime_tools]
    ]
  end

  def cli do
    [
      preferred_envs: [
        precommit: :test,
        "test:unit": :test,
        "test:integration": :integration,
        coveralls: :test,
        "coveralls.lcov": :test
      ]
    ]
  end

  # Specifies which paths to compile per environment.
  defp elixirc_paths(:test), do: ["lib", "test/support", "generated-contracts"]
  defp elixirc_paths(:integration), do: ["lib", "test/support", "generated-contracts"]
  defp elixirc_paths(_), do: ["lib", "generated-contracts"]

  # Specifies test paths per environment.
  # :integration runs only the integration step definitions (real PostgreSQL via docker).
  # :test runs unit steps, existing unit tests, and controller coverage tests.
  defp test_paths(:integration), do: ["test/integration"]
  defp test_paths(_), do: ["test"]

  # Specifies your project dependencies.
  defp deps do
    [
      # Phoenix framework + Ecto
      {:phoenix, "== 1.7.23"},
      {:phoenix_ecto, "== 4.7.0"},
      {:ecto_sql, "== 3.13.4"},
      {:postgrex, "== 0.22.2"},
      {:telemetry_metrics, "== 1.1.0"},
      {:telemetry_poller, "== 1.3.0"},
      {:gettext, "== 1.0.2"},
      {:jason, "== 1.4.4"},
      {:dns_cluster, "== 0.2.0"},
      # Plug pinned explicitly (CVE-2026-8468); previously transitive via Phoenix/Bandit
      {:plug, "== 1.19.2"},
      # Phoenix 1.7+ defaults to Bandit (not Cowboy)
      {:bandit, "== 1.11.1"},
      # Auth: JWT + password hashing
      {:guardian, "== 2.4.0"},
      {:bcrypt_elixir, "== 3.3.2"},
      # Test / BDD — vendored forks (local path deps, not Hex)
      {:elixir_gherkin, path: "../../libs/elixir-gherkin", only: [:test, :integration]},
      {:elixir_cabbage, path: "../../libs/elixir-cabbage", only: [:test, :integration]},
      {:excoveralls, "== 0.18.5", only: :test},
      # Dev / quality
      {:credo, "== 1.7.17", only: [:dev, :test], runtime: false}
    ]
  end

  defp aliases do
    [
      setup: ["deps.get", "ecto.setup"],
      "ecto.setup": ["ecto.create", "ecto.migrate", "run priv/repo/seeds.exs"],
      "ecto.reset": ["ecto.drop", "ecto.setup"],
      precommit: ["compile --warnings-as-errors", "deps.unlock --unused", "format", "test"]
    ]
  end
end
