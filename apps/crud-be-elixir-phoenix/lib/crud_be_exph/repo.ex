defmodule CrudBeExph.Repo do
  use Ecto.Repo,
    otp_app: :crud_be_exph,
    adapter: Ecto.Adapters.Postgres
end
