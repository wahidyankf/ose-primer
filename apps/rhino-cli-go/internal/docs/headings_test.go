package docs

import (
	"reflect"
	"strings"
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

func TestCollectATXHeadings_NestedFences(t *testing.T) {
	defer failOnPanic(t)

	// CommonMark: a fence opened with 4 backticks closes only on a
	// same-char run of >= 4. The inner ``` pair is fence CONTENT, so the
	// naive toggle desyncs and misses the heading after the block.
	content := strings.Join([]string{
		"# Before",
		"",
		"````markdown",
		"# inside outer fence",
		"```json",
		"# inside inner fence",
		"```",
		"# still inside outer fence",
		"````",
		"",
		"## After",
		"",
	}, "\n")

	got := CollectATXHeadings(content)
	want := []ATXHeading{
		{Line: 1, Level: 1, Title: "Before"},
		{Line: 11, Level: 2, Title: "After"},
	}

	if !reflect.DeepEqual(got, want) {
		t.Errorf("CollectATXHeadings() = %+v, want %+v", got, want)
	}
}

func TestCollectATXHeadings_MixedFenceCharsDoNotClose(t *testing.T) {
	defer failOnPanic(t)

	// A ``` line inside a ~~~ fence is content, not a closer — and vice
	// versa. Only a same-char run of sufficient length closes a fence.
	content := strings.Join([]string{
		"# Top",
		"",
		"~~~",
		"```",
		"# inside tilde fence",
		"```",
		"~~~",
		"",
		"```",
		"~~~",
		"# inside backtick fence",
		"~~~",
		"```",
		"",
		"## Bottom",
		"",
	}, "\n")

	got := CollectATXHeadings(content)
	want := []ATXHeading{
		{Line: 1, Level: 1, Title: "Top"},
		{Line: 15, Level: 2, Title: "Bottom"},
	}

	if !reflect.DeepEqual(got, want) {
		t.Errorf("CollectATXHeadings() = %+v, want %+v", got, want)
	}
}

func TestCollectATXHeadings_IndentedFencesInsideListItems(t *testing.T) {
	defer failOnPanic(t)

	// Deliberate CommonMark deviation: Prettier indents fences inside
	// nested list items by 4-7 spaces (legitimate fences relative to the
	// list container), so the flat tracker accepts ANY leading whitespace
	// on opening AND closing fences. A fence opened at a 5-space indent
	// must suppress heading extraction (even for flush-left fence content,
	// e.g. an example markdown snippet), and a 5-space closer must close
	// it.
	content := strings.Join([]string{
		"# Before",
		"",
		"1. List item:",
		"",
		"   - Nested item:",
		"",
		"     ```markdown",
		"# not a heading",
		"     ```",
		"",
		"## After",
		"",
	}, "\n")

	got := CollectATXHeadings(content)
	want := []ATXHeading{
		{Line: 1, Level: 1, Title: "Before"},
		{Line: 11, Level: 2, Title: "After"},
	}

	if !reflect.DeepEqual(got, want) {
		t.Errorf("CollectATXHeadings() = %+v, want %+v", got, want)
	}
}

func TestCollectATXHeadings_CloserMustHaveNoInfoString(t *testing.T) {
	defer failOnPanic(t)

	// A ```-prefixed line WITH an info string inside an open ``` fence is
	// a content line, never a closer (CommonMark closing fences carry no
	// info string). Any leading whitespace is tolerated on a closer.
	content := strings.Join([]string{
		"```",
		"```go",
		"# not a heading",
		"   ```",
		"",
		"# Real",
		"",
	}, "\n")

	got := CollectATXHeadings(content)
	want := []ATXHeading{
		{Line: 6, Level: 1, Title: "Real"},
	}

	if !reflect.DeepEqual(got, want) {
		t.Errorf("CollectATXHeadings() = %+v, want %+v", got, want)
	}
}
