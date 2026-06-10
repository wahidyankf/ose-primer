package envvalidate_test

import (
	"strings"
	"testing"

	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/envvalidate"
)

func cleanResult() *envvalidate.ValidateResult {
	return &envvalidate.ValidateResult{
		Surfaces: []*envvalidate.SurfaceResult{
			{App: "fixture-app", DeclaredNotRead: nil, ReadNotDeclared: nil},
		},
	}
}

func failedResult() *envvalidate.ValidateResult {
	return &envvalidate.ValidateResult{
		Surfaces: []*envvalidate.SurfaceResult{
			{App: "fixture-app", DeclaredNotRead: []string{"FIXTURE_JWT_SECRET"}, ReadNotDeclared: nil},
		},
	}
}

func TestFormatTextCleanReportsPassed(t *testing.T) {
	out := envvalidate.FormatText(cleanResult())
	if !strings.Contains(out, "passed") {
		t.Errorf("expected 'passed' in output, got: %s", out)
	}
}

func TestFormatTextFailureNamesKey(t *testing.T) {
	out := envvalidate.FormatText(failedResult())
	if !strings.Contains(out, "FIXTURE_JWT_SECRET") {
		t.Errorf("expected FIXTURE_JWT_SECRET in output, got: %s", out)
	}
	if !strings.Contains(out, "declared-but-unread") {
		t.Errorf("expected 'declared-but-unread' in output, got: %s", out)
	}
}

func TestFormatJSONCleanOkTrue(t *testing.T) {
	out, err := envvalidate.FormatJSON(cleanResult())
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(out, `"ok":true`) {
		t.Errorf("expected \"ok\":true in JSON, got: %s", out)
	}
}

func TestFormatJSONFailureNamesKey(t *testing.T) {
	out, err := envvalidate.FormatJSON(failedResult())
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(out, "FIXTURE_JWT_SECRET") {
		t.Errorf("expected FIXTURE_JWT_SECRET in JSON, got: %s", out)
	}
	if !strings.Contains(out, `"ok":false`) {
		t.Errorf("expected \"ok\":false in JSON, got: %s", out)
	}
}
