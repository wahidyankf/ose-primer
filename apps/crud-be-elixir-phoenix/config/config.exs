# This file is responsible for configuring your application
# and its dependencies with the aid of the Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
import Config

config :crud_be_exph,
  ecto_repos: [CrudBeExph.Repo],
  generators: [timestamp_type: :utc_datetime]

config :crud_be_exph, :accounts_module, CrudBeExph.Accounts
config :crud_be_exph, :token_module, CrudBeExph.Token.TokenContext
config :crud_be_exph, :expense_module, CrudBeExph.Expense.ExpenseContext
config :crud_be_exph, :attachment_module, CrudBeExph.Attachment.AttachmentContext

# Configure the endpoint
config :crud_be_exph, CrudBeExphWeb.Endpoint,
  url: [host: "localhost"],
  adapter: Bandit.PhoenixAdapter,
  render_errors: [
    formats: [json: CrudBeExphWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: CrudBeExph.PubSub,
  live_view: [signing_salt: "8pJ4Iu2a"]

# Configure Elixir's Logger
config :logger, :default_formatter,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{config_env()}.exs"
