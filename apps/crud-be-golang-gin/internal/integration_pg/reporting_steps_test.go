//go:build integration_pg

package integration_pg_test

import (
	"fmt"

	"github.com/cucumber/godog"
	"github.com/gin-gonic/gin"
)

func registerReportingSteps(sc *godog.ScenarioContext, ctx *scenarioCtx) {
	sc.Step(`^alice sends GET /api/v1/reports/pl\?from=([^&]+)&to=([^&]+)&currency=([^\s]+)$`, ctx.aliceSendsGetPLReport)
	sc.Step(`^the income breakdown should contain "([^"]*)" with amount "([^"]*)"$`, ctx.theIncomeBreakdownShouldContainCategory)
	sc.Step(`^the expense breakdown should contain "([^"]*)" with amount "([^"]*)"$`, ctx.theExpenseBreakdownShouldContainCategory)
}

// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary returns income total, expense total, and net for a period
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:Income entries are excluded from expense total
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:Expense entries are excluded from income total
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary filters by currency without cross-currency mixing
// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L summary for a period with no entries returns zero totals
// Traced by rhino-cli behavior-coverage validate.
func (ctx *scenarioCtx) aliceSendsGetPLReport(from, to, currency string) error {
	rawQuery := fmt.Sprintf("startDate=%s&endDate=%s&currency=%s", from, to, currency)
	c, w := buildGinContext("GET", "/api/v1/reports/pl?"+rawQuery, nil, ctx.AccessToken, gin.Params{}, ctx.JWTSvc)
	c.Request.URL.RawQuery = rawQuery
	ctx.Handler.PLReport(c)
	ctx.LastStatus = w.Code
	ctx.LastBody = readResponse(w)
	return nil
}

// @covers specs/apps/crud/behavior/crud-be/gherkin/expenses/reporting.feature:P&L breakdown includes category-level amounts for income and expenses
// Traced by rhino-cli behavior-coverage validate.
func (ctx *scenarioCtx) theIncomeBreakdownShouldContainCategory(category, amount string) error {
	breakdown, ok := ctx.LastBody["incomeBreakdown"]
	if !ok {
		return fmt.Errorf("response does not contain 'incomeBreakdown'; body: %v", ctx.LastBody)
	}
	items, ok := breakdown.([]interface{})
	if !ok {
		return fmt.Errorf("'incomeBreakdown' is not an array")
	}
	for _, item := range items {
		m, ok := item.(map[string]interface{})
		if !ok {
			continue
		}
		if fmt.Sprintf("%v", m["category"]) == category {
			if fmt.Sprintf("%v", m["total"]) == amount {
				return nil
			}
			return fmt.Errorf("incomeBreakdown category %q: expected amount %q, got %q", category, amount, fmt.Sprintf("%v", m["total"]))
		}
	}
	return fmt.Errorf("incomeBreakdown does not contain category %q", category)
}

func (ctx *scenarioCtx) theExpenseBreakdownShouldContainCategory(category, amount string) error {
	breakdown, ok := ctx.LastBody["expenseBreakdown"]
	if !ok {
		return fmt.Errorf("response does not contain 'expenseBreakdown'; body: %v", ctx.LastBody)
	}
	items, ok := breakdown.([]interface{})
	if !ok {
		return fmt.Errorf("'expenseBreakdown' is not an array")
	}
	for _, item := range items {
		m, ok := item.(map[string]interface{})
		if !ok {
			continue
		}
		if fmt.Sprintf("%v", m["category"]) == category {
			if fmt.Sprintf("%v", m["total"]) == amount {
				return nil
			}
			return fmt.Errorf("expenseBreakdown category %q: expected amount %q, got %q", category, amount, fmt.Sprintf("%v", m["total"]))
		}
	}
	return fmt.Errorf("expenseBreakdown does not contain category %q", category)
}
