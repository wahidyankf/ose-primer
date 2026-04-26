defmodule CrudBeExphWeb.HealthController do
  use CrudBeExphWeb, :controller

  alias GeneratedSchemas.HealthResponse

  def index(conn, _params) do
    _ = %HealthResponse{status: "UP"}
    json(conn, %{status: "UP"})
  end
end
