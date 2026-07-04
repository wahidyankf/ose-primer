using DemoBeCsas.Tests.ScenarioContext;
using FluentAssertions;
using Reqnroll;

namespace DemoBeCsas.Tests.Integration.Steps;

[Binding]
public class CommonSteps(ServiceLayer svc, SharedState state)
{
    [BeforeScenario]
    public async Task CleanDatabase(Reqnroll.ScenarioContext scenarioContext)
    {
        // This class is unscoped (its step defs like "the API is running" and "the response
        // status code should be {int}" are shared by every scenario), but the shared Gherkin
        // tree now tags scenarios with all applicable tiers (@unit @integration @e2e). A
        // @unit-tagged scenario is wired by ReqnrollHooks.RegisterServiceLayer onto
        // Unit.UnitTestHost, which never registers AppDbContext — so cleanup must be skipped
        // there; it only applies to the shared SQLite-backed IntegrationTestHost.
        if (scenarioContext.ScenarioInfo.Tags.Contains("unit"))
        {
            return;
        }

        await svc.CleanDatabaseAsync();
    }

    [Given("the API is running")]
    public void GivenTheApiIsRunning()
    {
        // ServiceLayer is always ready — nothing to do
    }

    [Then("the response status code should be {int}")]
    public void ThenStatusCode(int expectedCode)
    {
        state.LastResponse.Should().NotBeNull();
        state
            .LastResponse!.StatusCode.Should()
            .Be(expectedCode, $"Response body: {state.LastResponse.Body}");
    }
}
