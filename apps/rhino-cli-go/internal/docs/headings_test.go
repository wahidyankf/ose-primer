package docs

import (
	"reflect"
	"testing"
)

// failOnPanic converts a panic from an unimplemented stub into a test
// failure, so the TDD RED state reports FAIL instead of aborting the whole
// test binary (Go does not isolate panics per test like Rust's harness).
func failOnPanic(t *testing.T) {
	t.Helper()
	if r := recover(); r != nil {
		t.Errorf("panicked: %v", r)
	}
}

func TestGFMSlug(t *testing.T) {
	tests := []struct {
		name  string
		title string
		want  string
	}{
		{"keeps underscores", "snake_case naming", "snake_case-naming"},
		{"keeps unicode letters", "Café Über", "café-über"},
		{"does not collapse double spaces", "a  b", "a--b"},
		{"strips backticks", "`code` block", "code-block"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			defer failOnPanic(t)
			got := GFMSlug(tt.title)
			if got != tt.want {
				t.Errorf("GFMSlug(%q) = %q, want %q", tt.title, got, tt.want)
			}
		})
	}
}

func TestCollectHeadingAnchors_SuffixesDuplicates(t *testing.T) {
	defer failOnPanic(t)

	content := "## Setup\n\ntext\n\n## Setup\n"

	got := CollectHeadingAnchors(content)
	want := []string{"setup", "setup-1"}

	if !reflect.DeepEqual(got, want) {
		t.Errorf("CollectHeadingAnchors() = %v, want %v", got, want)
	}
}

func TestCollectATXHeadings_IgnoresFencedCode(t *testing.T) {
	defer failOnPanic(t)

	content := "# Real\n\n```bash\n# not a heading\n```\n\n## Another\n"

	got := CollectATXHeadings(content)
	want := []ATXHeading{
		{Line: 1, Level: 1, Title: "Real"},
		{Line: 7, Level: 2, Title: "Another"},
	}

	if !reflect.DeepEqual(got, want) {
		t.Errorf("CollectATXHeadings() = %+v, want %+v", got, want)
	}
}
