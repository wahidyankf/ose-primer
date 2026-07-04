using FluentAssertions;
using Reqnroll;
using Xunit;

namespace DemoBeCsas.Tests.Unit.Steps;

/// <summary>
/// Unit-level step bindings for cross-cutting steps (status codes, API running).
/// Mirrors <c>Integration.Steps.CommonSteps</c> but uses <see cref="UnitServiceLayer"/>
/// (in-memory repositories) and is scoped to scenarios tagged <c>@unit</c>.
/// </summary>
[Binding]
[Trait("Category", "Unit")]
[Scope(Tag = "unit")]
public class UnitCommonSteps(UnitSharedState state, UnitTestHost host)
{
    [BeforeScenario]
    public void ResetInMemoryStores()
    {
        // Clear all in-memory stores between scenarios so each scenario starts clean.
        host.Clear();
    }

    [Given("the API is running")]
    public void GivenTheApiIsRunning()
    {
        // UnitServiceLayer is always ready — nothing to do.
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
