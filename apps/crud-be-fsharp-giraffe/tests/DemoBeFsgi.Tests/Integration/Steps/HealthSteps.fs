module DemoBeFsgi.Tests.Integration.Steps.HealthSteps

open System.Text.Json
open TickSpec
open Xunit
open DemoBeFsgi.Tests.State
open DemoBeFsgi.Tests.DirectServices

[<When>]
let ``an operations engineer sends GET /health`` (state: StepState) =
    let status, body = health ()

    { state with
        Response = Some { Status = status; Body = body }
        ResponseBody = Some body }

[<When>]
let ``an unauthenticated engineer sends GET /health`` (state: StepState) =
    let status, body = health ()

    { state with
        Response = Some { Status = status; Body = body }
        ResponseBody = Some body }

// @covers specs/apps/crud/behavior/crud-be/gherkin/health/health-check.feature:Health endpoint reports the service as UP
[<Then>]
let ``the health status should be "(.+)"`` (status: string) (state: StepState) =
    let doc = JsonDocument.Parse(state.ResponseBody.Value)
    let actual = doc.RootElement.GetProperty("status").GetString()
    Assert.Equal(status, actual)
    state

// @covers specs/apps/crud/behavior/crud-be/gherkin/health/health-check.feature:Anonymous health check does not expose component details
[<Then>]
let ``the response should not include detailed component health information`` (state: StepState) =
    let doc = JsonDocument.Parse(state.ResponseBody.Value)
    let mutable hasComponents = false

    for prop in doc.RootElement.EnumerateObject() do
        if prop.Name = "components" || prop.Name = "details" then
            hasComponents <- true

    Assert.False(hasComponents, "Response should not include component details")
    state
