package docs

import (
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

func TestShouldSkipLink(t *testing.T) {
	tests := []struct {
		name string
		link string
		want bool
	}{
		// Should skip
		{"absolute path", "/docs/path", true},
		{"shortcode angle", "{{< ref >}}", true},
		{"shortcode percent", "{{% ref %}}", true},
		{"Placeholder path.md", "path.md", true},
		{"Placeholder target", "target", true},
		{"Placeholder link", "link", true},
		{"Placeholder ./path/to/", "./path/to/file.md", true},
		{"Placeholder ../path/to/", "../path/to/file.md", true},
		{"Placeholder path/to/convention.md", "path/to/convention.md", true},
		{"Template placeholder", "file[name].md", true},
		{"Example image path", "/images/logo.png", true},
		{"Example ./overview", "./overview", true},
		{"Example by-concept", "by-concept/beginner/intro.md", true},
		{"OpenCode reference", "../../.opencode/agents/test.md", true},

		// Line 179: exact "path" match (not caught by placeholders which check "path.md")
		{"Exact word path", "path", true},
		// Line 184: relative path containing /images/ (not starting with /)
		{"Relative images path", "docs/images/logo.png", true},

		// Should NOT skip
		{"Valid relative link", "../docs/README.md", false},
		{"Valid same dir link", "./file.md", false},
		{"Valid parent link", "../../file.md", false},
		{"Valid nested link", "../repo-governance/conventions/file.md", false},
		{"Valid with anchor", "../docs/README.md#section", false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := ShouldSkipLink(tt.link)
			if got != tt.want {
				t.Errorf("ShouldSkipLink(%q) = %v, want %v", tt.link, got, tt.want)
			}
		})
	}
}

func TestExtractLinks(t *testing.T) {
	// Create temporary test file
	tmpDir := t.TempDir()
	testFile := filepath.Join(tmpDir, "test.md")

	content := `# Test File

This is a [valid link](../docs/README.md) and [another](./file.md).

` + "```go" + `
// This [code link](./should-skip.md) should be skipped
` + "```" + `

External [link](https://example.com) should be skipped.
Internal [anchor](#section) should be extracted for anchor validation.
Email [contact](mailto:test@example.com) should be skipped.

This [placeholder](path.md) should be skipped.
This [real link](../../repo-governance/README.md) should not be skipped.

[path](/docs/page) should be skipped.
`

	if err := os.WriteFile(testFile, []byte(content), 0644); err != nil {
		t.Fatalf("Failed to create test file: %v", err)
	}

	links, err := ExtractLinks(testFile)
	if err != nil {
		t.Fatalf("ExtractLinks() error = %v", err)
	}

	// Expected links (not skipped). Pure-anchor links are extracted so
	// same-file anchors reach validation.
	expected := map[string]int{
		"../docs/README.md":               3,
		"./file.md":                       3,
		"#section":                        10,
		"../../repo-governance/README.md": 14,
	}

	if len(links) != len(expected) {
		t.Errorf("ExtractLinks() found %d links, want %d", len(links), len(expected))
		for _, link := range links {
			t.Logf("  Found: %s at line %d", link.URL, link.LineNumber)
		}
	}

	// Verify each expected link
	for _, link := range links {
		expectedLine, ok := expected[link.URL]
		if !ok {
			t.Errorf("Unexpected link found: %s at line %d", link.URL, link.LineNumber)
			continue
		}
		if link.LineNumber != expectedLine {
			t.Errorf("Link %s at line %d, want line %d", link.URL, link.LineNumber, expectedLine)
		}
		if !link.IsRelative {
			t.Errorf("Link %s should be relative", link.URL)
		}
	}
}

func TestExtractLinksCodeBlock(t *testing.T) {
	tmpDir := t.TempDir()
	testFile := filepath.Join(tmpDir, "test.md")

	content := `# Test

Before code block [link1](./file1.md)

` + "```" + `
Inside code block [link2](./file2.md)
` + "```" + `

After code block [link3](./file3.md)
`

	if err := os.WriteFile(testFile, []byte(content), 0644); err != nil {
		t.Fatalf("Failed to create test file: %v", err)
	}

	links, err := ExtractLinks(testFile)
	if err != nil {
		t.Fatalf("ExtractLinks() error = %v", err)
	}

	// Should only find links outside code blocks
	if len(links) != 2 {
		t.Errorf("ExtractLinks() found %d links, want 2", len(links))
		for _, link := range links {
			t.Logf("  Found: %s", link.URL)
		}
	}

	// Verify correct links found
	foundURLs := make(map[string]bool)
	for _, link := range links {
		foundURLs[link.URL] = true
	}

	if !foundURLs["./file1.md"] {
		t.Error("Expected to find ./file1.md")
	}
	if !foundURLs["./file3.md"] {
		t.Error("Expected to find ./file3.md")
	}
	if foundURLs["./file2.md"] {
		t.Error("Should not find ./file2.md (inside code block)")
	}
}

func TestExtractLinks_NestedFences(t *testing.T) {
	tmpDir := t.TempDir()
	testFile := filepath.Join(tmpDir, "test.md")

	// CommonMark: the ````markdown fence closes only on a backtick run of
	// >= 4, so the inner ``` pair is fence content. Every link inside the
	// outer block must be skipped; the link after it must be extracted.
	content := strings.Join([]string{
		"# Test",
		"",
		"Before [outside1](./before.md)",
		"",
		"````markdown",
		"[inside outer](./outer.md)",
		"```json",
		"[inside inner](./inner.md)",
		"```",
		"[still inside outer](./still-outer.md)",
		"````",
		"",
		"After [outside2](./after.md)",
		"",
	}, "\n")

	if err := os.WriteFile(testFile, []byte(content), 0644); err != nil {
		t.Fatalf("Failed to create test file: %v", err)
	}

	links, err := ExtractLinks(testFile)
	if err != nil {
		t.Fatalf("ExtractLinks() error = %v", err)
	}

	if len(links) != 2 {
		t.Errorf("ExtractLinks() found %d links, want 2", len(links))
		for _, link := range links {
			t.Logf("  Found: %s at line %d", link.URL, link.LineNumber)
		}
	}

	foundURLs := make(map[string]bool)
	for _, link := range links {
		foundURLs[link.URL] = true
	}

	if !foundURLs["./before.md"] {
		t.Error("Expected to find ./before.md (before nested block)")
	}
	if !foundURLs["./after.md"] {
		t.Error("Expected to find ./after.md (after nested block)")
	}
	for _, inside := range []string{"./outer.md", "./inner.md", "./still-outer.md"} {
		if foundURLs[inside] {
			t.Errorf("Should not find %s (inside nested fenced block)", inside)
		}
	}
}

func TestExtractLinks_IndentedFencesInsideListItems(t *testing.T) {
	tmpDir := t.TempDir()
	testFile := filepath.Join(tmpDir, "test.md")

	// Deliberate CommonMark deviation: Prettier indents fences inside
	// nested list items by 4-7 spaces. The flat tracker accepts ANY
	// leading whitespace on opening AND closing fences so the fence body
	// (e.g. example markdown containing link syntax) never produces false
	// link extractions. Regression case from
	// repo-governance/workflows/plan/plan-execution.md:524.
	content := strings.Join([]string{
		"# Test",
		"",
		"Before [outside1](./before.md)",
		"",
		"1. List item:",
		"",
		"   - Nested item:",
		"",
		"     ```markdown",
		"     [inside indented fence](./inside.md)",
		"     ```",
		"",
		"After [outside2](./after.md)",
		"",
	}, "\n")

	if err := os.WriteFile(testFile, []byte(content), 0644); err != nil {
		t.Fatalf("Failed to create test file: %v", err)
	}

	links, err := ExtractLinks(testFile)
	if err != nil {
		t.Fatalf("ExtractLinks() error = %v", err)
	}

	if len(links) != 2 {
		t.Errorf("ExtractLinks() found %d links, want 2", len(links))
		for _, link := range links {
			t.Logf("  Found: %s at line %d", link.URL, link.LineNumber)
		}
	}

	foundURLs := make(map[string]bool)
	for _, link := range links {
		foundURLs[link.URL] = true
	}

	if !foundURLs["./before.md"] {
		t.Error("Expected to find ./before.md (before indented block)")
	}
	if !foundURLs["./after.md"] {
		t.Error("Expected to find ./after.md (after indented block)")
	}
	if foundURLs["./inside.md"] {
		t.Error("Should not find ./inside.md (inside indented fenced block)")
	}
}

func TestExtractLinks_TildeFences(t *testing.T) {
	tmpDir := t.TempDir()
	testFile := filepath.Join(tmpDir, "test.md")

	// ~~~ fences are now recognised (aligned with CollectATXHeadings), and
	// a ``` line inside a ~~~ fence is content, not a closer — and vice
	// versa.
	content := strings.Join([]string{
		"# Test",
		"",
		"Before [outside1](./before.md)",
		"",
		"~~~",
		"[inside tilde](./tilde.md)",
		"```",
		"[still inside tilde](./still-tilde.md)",
		"```",
		"~~~",
		"",
		"```",
		"~~~",
		"[inside backtick](./backtick.md)",
		"~~~",
		"```",
		"",
		"After [outside2](./after.md)",
		"",
	}, "\n")

	if err := os.WriteFile(testFile, []byte(content), 0644); err != nil {
		t.Fatalf("Failed to create test file: %v", err)
	}

	links, err := ExtractLinks(testFile)
	if err != nil {
		t.Fatalf("ExtractLinks() error = %v", err)
	}

	if len(links) != 2 {
		t.Errorf("ExtractLinks() found %d links, want 2", len(links))
		for _, link := range links {
			t.Logf("  Found: %s at line %d", link.URL, link.LineNumber)
		}
	}

	foundURLs := make(map[string]bool)
	for _, link := range links {
		foundURLs[link.URL] = true
	}

	if !foundURLs["./before.md"] {
		t.Error("Expected to find ./before.md (before tilde block)")
	}
	if !foundURLs["./after.md"] {
		t.Error("Expected to find ./after.md (after both blocks)")
	}
	for _, inside := range []string{"./tilde.md", "./still-tilde.md", "./backtick.md"} {
		if foundURLs[inside] {
			t.Errorf("Should not find %s (inside fenced block)", inside)
		}
	}
}

func TestExtractLinksAngleBrackets(t *testing.T) {
	tmpDir := t.TempDir()
	testFile := filepath.Join(tmpDir, "test.md")

	content := `# Test

This is a [link](<../docs/README.md>) with angle brackets.
`

	if err := os.WriteFile(testFile, []byte(content), 0644); err != nil {
		t.Fatalf("Failed to create test file: %v", err)
	}

	links, err := ExtractLinks(testFile)
	if err != nil {
		t.Fatalf("ExtractLinks() error = %v", err)
	}

	if len(links) != 1 {
		t.Fatalf("ExtractLinks() found %d links, want 1", len(links))
	}

	if links[0].URL != "../docs/README.md" {
		t.Errorf("Link URL = %q, want %q", links[0].URL, "../docs/README.md")
	}
}

func TestGetMarkdownFiles_NonStaged(t *testing.T) {
	tmpDir := t.TempDir()

	// Create .claude dir with a .md file
	claudeDir := filepath.Join(tmpDir, ".claude")
	if err := os.MkdirAll(claudeDir, 0755); err != nil {
		t.Fatalf("failed to create .claude dir: %v", err)
	}
	if err := os.WriteFile(filepath.Join(claudeDir, "test.md"), []byte("# Test"), 0644); err != nil {
		t.Fatalf("failed to create test.md: %v", err)
	}

	opts := ScanOptions{
		RepoRoot:   tmpDir,
		StagedOnly: false,
		SkipPaths:  []string{},
	}

	files, err := GetMarkdownFiles(opts)
	if err != nil {
		t.Fatalf("GetMarkdownFiles() error: %v", err)
	}

	found := false
	for _, f := range files {
		if filepath.Base(f) == "test.md" {
			found = true
			break
		}
	}
	if !found {
		t.Errorf("expected test.md in results, got %v", files)
	}
}

func TestGetMarkdownFiles_WithSkipPaths(t *testing.T) {
	tmpDir := t.TempDir()

	for _, dir := range []string{"docs", ".claude"} {
		d := filepath.Join(tmpDir, dir)
		if err := os.MkdirAll(d, 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		if err := os.WriteFile(filepath.Join(d, "file.md"), []byte("# Content"), 0644); err != nil {
			t.Fatalf("failed to create file: %v", err)
		}
	}

	opts := ScanOptions{
		RepoRoot:   tmpDir,
		StagedOnly: false,
		SkipPaths:  []string{"docs"},
	}

	files, err := GetMarkdownFiles(opts)
	if err != nil {
		t.Fatalf("GetMarkdownFiles() error: %v", err)
	}

	for _, f := range files {
		rel, _ := filepath.Rel(tmpDir, f)
		if len(rel) > 4 && rel[:4] == "docs" {
			t.Errorf("expected docs to be skipped, got %v in results", f)
		}
	}
}

func TestFilterSkipPaths_Empty(t *testing.T) {
	files := []string{"/repo/docs/file.md", "/repo/repo-governance/other.md"}
	result := FilterSkipPaths(files, "/repo", []string{})
	if len(result) != len(files) {
		t.Errorf("expected all files with empty skip paths, got %d files", len(result))
	}
}

func TestFilterSkipPaths_WithSkipPath(t *testing.T) {
	tmpDir := t.TempDir()

	docsDir := filepath.Join(tmpDir, "docs")
	govDir := filepath.Join(tmpDir, "repo-governance")
	if err := os.MkdirAll(docsDir, 0755); err != nil {
		t.Fatalf("failed to create docs: %v", err)
	}
	if err := os.MkdirAll(govDir, 0755); err != nil {
		t.Fatalf("failed to create governance: %v", err)
	}

	files := []string{
		filepath.Join(docsDir, "file.md"),
		filepath.Join(govDir, "other.md"),
		filepath.Join(docsDir, "nested", "deep.md"),
	}
	result := FilterSkipPaths(files, tmpDir, []string{"repo-governance"})

	for _, f := range result {
		rel, _ := filepath.Rel(tmpDir, f)
		if len(rel) > 10 && rel[:10] == "repo-governance" {
			t.Errorf("expected governance files to be filtered out, got %v", result)
		}
	}
}

func TestGetMarkdownFiles_RepoWideWalkSkipsNoiseDirs(t *testing.T) {
	tmpDir := t.TempDir()

	for _, sub := range []string{
		"libs/my-lib",
		"docs",
		"node_modules/some-pkg",
		"generated-reports",
		"worktrees/copy/docs",
	} {
		if err := os.MkdirAll(filepath.Join(tmpDir, sub), 0755); err != nil {
			t.Fatalf("failed to create %s: %v", sub, err)
		}
	}

	writes := map[string]string{
		"libs/my-lib/README.md":           "[bad](./missing.md)\n",
		"docs/a.md":                       "ok\n",
		"node_modules/some-pkg/README.md": "skip\n",
		"generated-reports/report.md":     "skip\n",
		"worktrees/copy/docs/a.md":        "skip\n",
	}
	for rel, content := range writes {
		if err := os.WriteFile(filepath.Join(tmpDir, rel), []byte(content), 0644); err != nil {
			t.Fatalf("failed to write %s: %v", rel, err)
		}
	}

	opts := ScanOptions{
		RepoRoot:   tmpDir,
		StagedOnly: false,
	}

	files, err := GetMarkdownFiles(opts)
	if err != nil {
		t.Fatalf("GetMarkdownFiles() error: %v", err)
	}

	rels := make(map[string]bool, len(files))
	var relList []string
	for _, f := range files {
		rel, relErr := filepath.Rel(tmpDir, f)
		if relErr != nil {
			t.Fatalf("failed to get relative path for %s: %v", f, relErr)
		}
		rel = filepath.ToSlash(rel)
		rels[rel] = true
		relList = append(relList, rel)
	}

	// Repo-wide walk must reach beyond the historical 3-dir set.
	if !rels["libs/my-lib/README.md"] {
		t.Errorf("expected libs/my-lib/README.md in scan set, got %v", relList)
	}
	if !rels["docs/a.md"] {
		t.Errorf("expected docs/a.md in scan set, got %v", relList)
	}

	// Standardized noise dirs must be skipped by name.
	for _, noise := range []string{"node_modules/", "generated-reports/", "worktrees/"} {
		for _, rel := range relList {
			if strings.HasPrefix(rel, noise) {
				t.Errorf("noise dir %s leaked into scan set: %v", noise, relList)
			}
		}
	}
}

func TestExtractLinks_PureAnchorExtracted(t *testing.T) {
	tmpDir := t.TempDir()
	testFile := filepath.Join(tmpDir, "test.md")

	content := `# Title

See [doc](./real.md) and [ext](https://example.com).

[anchor](#section)
`

	if err := os.WriteFile(testFile, []byte(content), 0644); err != nil {
		t.Fatalf("Failed to create test file: %v", err)
	}

	links, err := ExtractLinks(testFile)
	if err != nil {
		t.Fatalf("ExtractLinks() error = %v", err)
	}

	// Pure-anchor links must be extracted so same-file anchors reach
	// validation; external URLs are still skipped.
	if len(links) != 2 {
		t.Fatalf("ExtractLinks() found %d links, want 2 (./real.md and #section): %+v", len(links), links)
	}

	if links[0].URL != "./real.md" || links[0].LineNumber != 3 {
		t.Errorf("links[0] = %+v, want URL ./real.md at line 3", links[0])
	}
	if links[1].URL != "#section" || links[1].LineNumber != 5 {
		t.Errorf("links[1] = %+v, want URL #section at line 5", links[1])
	}
	if !links[1].IsRelative {
		t.Errorf("pure-anchor link should be relative: %+v", links[1])
	}
}

func TestGetMarkdownFiles_Staged(t *testing.T) {
	tmpDir := t.TempDir()

	// Init git repo
	initCmds := [][]string{
		{"git", "-C", tmpDir, "init", "-q"},
		{"git", "-C", tmpDir, "config", "user.email", "test@test.com"},
		{"git", "-C", tmpDir, "config", "user.name", "Test"},
	}
	for _, args := range initCmds {
		if err := exec.Command(args[0], args[1:]...).Run(); err != nil {
			t.Skipf("git not available: %v", err)
		}
	}

	// Create and stage a markdown file
	docsDir := filepath.Join(tmpDir, "docs")
	if err := os.MkdirAll(docsDir, 0755); err != nil {
		t.Fatal(err)
	}
	mdFile := filepath.Join(docsDir, "test.md")
	if err := os.WriteFile(mdFile, []byte("# Test"), 0644); err != nil {
		t.Fatal(err)
	}
	if err := exec.Command("git", "-C", tmpDir, "add", "docs/test.md").Run(); err != nil {
		t.Skipf("git add failed: %v", err)
	}

	opts := ScanOptions{
		RepoRoot:   tmpDir,
		StagedOnly: true,
	}

	files, err := GetMarkdownFiles(opts)
	if err != nil {
		t.Fatalf("GetMarkdownFiles(staged=true) error: %v", err)
	}
	if len(files) != 1 {
		t.Errorf("expected 1 staged markdown file, got %d: %v", len(files), files)
	}
}
