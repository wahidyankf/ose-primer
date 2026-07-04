defmodule CrudBeExphWeb.Integration.RegistrationSteps do
  use Cabbage.Feature, async: false, file: "user-lifecycle/registration.feature"

  use CrudBeExph.DataCaseIntegration

  alias CrudBeExph.Integration.Helpers
  alias CrudBeExph.Integration.ServiceLayer

  @moduletag :integration

  defgiven ~r/^the API is running$/, _vars, state do
    {:ok, state}
  end

  defgiven ~r/^a user "(?<username>[^"]+)" is registered with email "(?<email>[^"]+)" and password "(?<password>[^"]+)"$/,
           %{username: username, email: email, password: password},
           state do
    Helpers.register_user!(username, email, password)
    {:ok, state}
  end

  defwhen ~r/^the client sends POST .api.v1.auth.register with body \{ "username": "(?<username>[^"]+)", "email": "(?<email>[^"]+)", "password": "(?<password>[^"]*)" \}$/,
          %{username: username, email: email, password: password},
          state do
    response =
      ServiceLayer.register(%{"username" => username, "email" => email, "password" => password})

    {:ok, Map.put(state, :response, response)}
  end

  defthen ~r/^the response status code should be (?<code>\d+)$/,
          %{code: code},
          %{response: response} = state do
    assert response.status == String.to_integer(code)
    {:ok, state}
  end

  defthen ~r/^the response body should contain "(?<field>[^"]+)" equal to "(?<value>[^"]+)"$/,
          %{field: field, value: value},
          %{response: response} = state do
    assert response.body[field] == value
    {:ok, state}
  end

  # @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Successful registration returns created user profile without password
  defthen ~r/^the response body should not contain a "(?<field>[^"]+)" field$/,
          %{field: field},
          %{response: response} = state do
    refute Map.has_key?(response.body, field)
    {:ok, state}
  end

  # @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Successful registration response includes non-null user ID
  defthen ~r/^the response body should contain a non-null "(?<field>[^"]+)" field$/,
          %{field: field},
          %{response: response} = state do
    assert Map.has_key?(response.body, field)
    assert response.body[field] != nil
    {:ok, state}
  end

  # @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration when username already exists
  defthen ~r/^the response body should contain an error message about duplicate username$/,
          _vars,
          %{response: response} = state do
    assert response.body["message"] =~ ~r/[Uu]sername.*exist|already|[Dd]uplicate/i
    {:ok, state}
  end

  # @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration with invalid email format
  # @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration with empty password
  # @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/registration.feature:Reject registration with weak password — no uppercase letter
  defthen ~r/^the response body should contain a validation error for "(?<field>[^"]+)"$/,
          %{field: field},
          %{response: response} = state do
    assert Map.has_key?(response.body, "errors")
    errors = response.body["errors"]
    assert Map.has_key?(errors, field)
    assert errors[field] != []
    {:ok, state}
  end
end
