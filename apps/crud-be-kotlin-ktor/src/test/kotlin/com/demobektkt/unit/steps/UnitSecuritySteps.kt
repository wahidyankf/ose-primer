package com.demobektkt.unit.steps

import io.cucumber.java.en.Then
import io.cucumber.java.en.When
import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.Assertions.assertEquals

class UnitSecuritySteps {

  // @covers specs/apps/crud/behavior/crud-be/gherkin/admin/admin.feature:Admin disables a user account
  // @covers specs/apps/crud/behavior/crud-be/gherkin/admin/admin.feature:Admin re-enables a disabled user account
  // @covers specs/apps/crud/behavior/crud-be/gherkin/security/security.feature:Account is locked after exceeding the maximum failed login threshold
  @Then("alice's account status should be {string}")
  fun alicesAccountStatusShouldBe(expectedStatus: String) {
    val user =
      runBlocking { UnitTestWorld.userRepo.findByUsername("alice") } ?: error("alice not found")
    assertEquals(
      expectedStatus.lowercase(),
      user.status.name.lowercase(),
      "alice's account status should be $expectedStatus",
    )
  }

  @When("^the admin sends POST /api/v1/admin/users/\\{alice_id\\}/unlock$")
  fun theAdminSendsPostUnlockAlice() {
    val aliceId = UnitTestWorld.userIds["alice"] ?: error("alice id not stored")
    val adminToken = UnitTestWorld.accessTokens["superadmin"] ?: error("admin token not stored")
    val (status, body) = UnitServiceDispatcher.unlockUser(adminToken, aliceId)
    UnitTestWorld.lastResponseStatus = status
    UnitTestWorld.lastResponseBody = body
  }
}
