package envvalidate_test

import (
	"testing"

	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/envvalidate"
)

func TestSurfaceResultIsOK(t *testing.T) {
	ok := &envvalidate.SurfaceResult{App: "a", DeclaredNotRead: nil, ReadNotDeclared: nil}
	if !ok.IsOK() {
		t.Error("expected IsOK true for empty sets")
	}
	bad := &envvalidate.SurfaceResult{App: "a", DeclaredNotRead: []string{"X"}}
	if bad.IsOK() {
		t.Error("expected IsOK false when DeclaredNotRead non-empty")
	}
}

func TestValidateResultIsOKAndViolationCount(t *testing.T) {
	clean := &envvalidate.ValidateResult{
		Surfaces: []*envvalidate.SurfaceResult{
			{App: "a"},
			{App: "b"},
		},
	}
	if !clean.IsOK() {
		t.Error("expected IsOK true for clean result")
	}
	if clean.ViolationCount() != 0 {
		t.Errorf("expected 0 violations, got %d", clean.ViolationCount())
	}

	dirty := &envvalidate.ValidateResult{
		Surfaces: []*envvalidate.SurfaceResult{
			{App: "a", DeclaredNotRead: []string{"X"}},
			{App: "b"},
			{App: "c", ReadNotDeclared: []string{"Y"}},
		},
	}
	if dirty.IsOK() {
		t.Error("expected IsOK false for dirty result")
	}
	if dirty.ViolationCount() != 2 {
		t.Errorf("expected 2 violations, got %d", dirty.ViolationCount())
	}
}

func TestFormatTextSingleViolation(t *testing.T) {
	result := &envvalidate.ValidateResult{
		Surfaces: []*envvalidate.SurfaceResult{
			{App: "a", DeclaredNotRead: []string{"ONLY_ONE"}},
		},
	}
	out := envvalidate.FormatText(result)
	// "1 violation" (no "s" suffix)
	if !containsStr(out, "1 violation\n") {
		t.Errorf("expected '1 violation' (no s) in output, got: %s", out)
	}
}

func TestFormatTextReadNotDeclared(t *testing.T) {
	result := &envvalidate.ValidateResult{
		Surfaces: []*envvalidate.SurfaceResult{
			{App: "a", ReadNotDeclared: []string{"EXTRA_KEY"}},
		},
	}
	out := envvalidate.FormatText(result)
	if !containsStr(out, "read-but-undeclared: EXTRA_KEY") {
		t.Errorf("expected read-but-undeclared label, got: %s", out)
	}
}

func containsStr(s, sub string) bool {
	return len(s) >= len(sub) && (s == sub || len(s) > 0 && findSubstr(s, sub))
}

func findSubstr(s, sub string) bool {
	for i := 0; i <= len(s)-len(sub); i++ {
		if s[i:i+len(sub)] == sub {
			return true
		}
	}
	return false
}
