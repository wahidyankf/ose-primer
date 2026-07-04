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

    // @covers specs/apps/crud/behavior/crud-be/gherkin/admin/admin.feature:Disabled user's access token is rejected with 401
    // @covers specs/apps/crud/behavior/crud-be/gherkin/authentication/token-lifecycle.feature:Logout is idempotent — repeating logout on the same token returns 200
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Delete attachment returns 204
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Upload attachment to another user's entry returns 403
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:List attachments on another user's entry returns 403
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Delete attachment on another user's entry returns 403
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/attachments.feature:Delete non-existent attachment returns 404
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Delete an entry returns 204
    // @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/expense-management.feature:Unauthenticated request to create an entry returns 401
    // @covers specs/apps/crud/behavior/crud-be/gherkin/security/security.feature:Admin unlocks a locked account
    // @covers specs/apps/crud/behavior/crud-be/gherkin/token-management/tokens.feature:Blacklisted access token is rejected with 401 on protected endpoints
    // @covers specs/apps/crud/behavior/crud-be/gherkin/token-management/tokens.feature:Deactivating a user revokes all their active tokens
    // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Successful password change returns 200
    // @covers specs/apps/crud/behavior/crud-be/gherkin/user-lifecycle/user-account.feature:Authenticated user self-deactivates their account
    [Then("the response status code should be {int}")]
    public void ThenStatusCode(int expectedCode)
    {
        state.LastResponse.Should().NotBeNull();
        state
            .LastResponse!.StatusCode.Should()
            .Be(expectedCode, $"Response body: {state.LastResponse.Body}");
    }
}
