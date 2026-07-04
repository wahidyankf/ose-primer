"""BDD step definitions for financial reporting feature."""

import json

from pytest_bdd import given, parsers, scenarios, then, when

from tests.integration.service_client import FakeResponse, ServiceClient
from tests.unit.conftest import GHERKIN_ROOT

scenarios(str(GHERKIN_ROOT / "expenses" / "reporting.feature"))

_PASSWORD = "Str0ng#Pass1"


@given(
    '"alice" has logged in and stored the access token',
    target_fixture="alice_tokens",
)
def alice_login_reporting(client: ServiceClient, registered_user: dict) -> dict:
    return client.login_user("alice", _PASSWORD)


@given(
    parsers.parse("alice has created an entry with body {body}"),
    target_fixture="reporting_entry",
)
def alice_create_reporting_entry(client: ServiceClient, alice_tokens: dict, body: str) -> dict:
    data = json.loads(body)
    resp = client.post_expense(f"Bearer {alice_tokens['accessToken']}", data)
    assert resp.status_code == 201, f"Create entry failed: {resp.text}"
    return resp.json()


# --- When steps ---


@when(
    parsers.parse("alice sends GET /api/v1/reports/pl?from={from_}&to={to}&currency={currency}"),
    target_fixture="response",
)
def alice_get_pl_report(
    client: ServiceClient, alice_tokens: dict, from_: str, to: str, currency: str
) -> FakeResponse:
    return client.get_pl_report(
        f"Bearer {alice_tokens['accessToken']}",
        from_,
        to,
        currency,
    )


# --- Then steps ---


# @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary filters by currency without cross-currency mixing
@then(parsers.parse('the response body should contain "totalIncome" equal to "{value}"'))
def check_income_total(response: FakeResponse, value: str) -> None:
    body = response.json()
    assert "totalIncome" in body, f"totalIncome not in: {body}"
    assert str(body["totalIncome"]) == value, (
        f"Expected totalIncome={value}, got {body['totalIncome']}"
    )


# @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:Income entries are excluded from expense total
# @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:Expense entries are excluded from income total
@then(parsers.parse('the response body should contain "totalExpense" equal to "{value}"'))
def check_expense_total(response: FakeResponse, value: str) -> None:
    body = response.json()
    assert "totalExpense" in body, f"totalExpense not in: {body}"
    assert str(body["totalExpense"]) == value, (
        f"Expected totalExpense={value}, got {body['totalExpense']}"
    )


# @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary returns income total, expense total, and net for a period
# @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary for a period with no entries returns zero totals
@then(parsers.parse('the response body should contain "net" equal to "{value}"'))
def check_net(response: FakeResponse, value: str) -> None:
    body = response.json()
    assert "net" in body, f"net not in: {body}"
    assert str(body["net"]) == value, f"Expected net={value}, got {body['net']}"


@then(parsers.parse('the income breakdown should contain "{category}" with amount "{amount}"'))
def check_income_breakdown(response: FakeResponse, category: str, amount: str) -> None:
    body = response.json()
    breakdown = body.get("incomeBreakdown", [])
    entry = next((item for item in breakdown if item.get("category") == category), None)
    assert entry is not None, f"Category '{category}' not in incomeBreakdown: {breakdown}"
    assert str(entry["total"]) == amount, (
        f"Expected incomeBreakdown[{category}]={amount}, got {entry['total']}"
    )


# @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L breakdown includes category-level amounts for income and expenses
@then(parsers.parse('the expense breakdown should contain "{category}" with amount "{amount}"'))
def check_expense_breakdown(response: FakeResponse, category: str, amount: str) -> None:
    body = response.json()
    breakdown = body.get("expenseBreakdown", [])
    entry = next((item for item in breakdown if item.get("category") == category), None)
    assert entry is not None, f"Category '{category}' not in expenseBreakdown: {breakdown}"
    assert str(entry["total"]) == amount, (
        f"Expected expenseBreakdown[{category}]={amount}, got {entry['total']}"
    )
