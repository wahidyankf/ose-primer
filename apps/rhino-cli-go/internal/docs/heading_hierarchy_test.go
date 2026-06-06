package docs

// TDD RED tests for the heading-hierarchy validator (Gate C, plan DD-7).
//
// The fixture set mirrors the Rust twin's canonical spec (tests a–k in
// `apps/rhino-cli-rust/src/internal/docs/heading_hierarchy.rs`) exactly, so
// the Go GREEN step converges on identical behavior.

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// writeFixture writes content to root/rel, creating parent directories.
func writeFixture(t *testing.T, root, rel, content string) {
	t.Helper()
	path := filepath.Join(root, rel)
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		t.Fatalf("mkdir %s: %v", rel, err)
	}
	if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
		t.Fatalf("write %s: %v", rel, err)
	}
}

// scanHierarchy runs a full allowlist walk over root and returns findings.
func scanHierarchy(t *testing.T, root string) []HeadingFinding {
	t.Helper()
	findings, err := ValidateHeadingHierarchy(HeadingScanOptions{Root: root})
	if err != nil {
		t.Fatalf("ValidateHeadingHierarchy() error: %v", err)
	}
	return findings
}

// --- (a) duplicate H1 in docs/ ---

func TestHeadingHierarchy_DocsFileWithTwoH1s_ReportsDuplicateH1(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	writeFixture(t, root, "docs/guide.md", "# First Title\n\ntext\n\n# Second Title\n")

	findings := scanHierarchy(t, root)

	if len(findings) != 1 {
		t.Fatalf("expected one finding, got %+v", findings)
	}
	if findings[0].Kind != HeadingKindDuplicateH1 {
		t.Errorf("Kind = %q, want %q", findings[0].Kind, HeadingKindDuplicateH1)
	}
	if findings[0].File != "docs/guide.md" {
		t.Errorf("File = %q, want %q", findings[0].File, "docs/guide.md")
	}
	if findings[0].Line != 5 {
		t.Errorf("Line = %d, want 5 (duplicate-h1 reports the second H1's line)", findings[0].Line)
	}
	if findings[0].Message == "" {
		t.Error("Message must not be empty")
	}
}

// --- (b) missing H1 in docs/ ---

func TestHeadingHierarchy_DocsFileWithZeroH1s_ReportsMissingH1(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	writeFixture(t, root, "docs/notes.md", "## Only A Section\n\ntext\n")

	findings := scanHierarchy(t, root)

	if len(findings) != 1 {
		t.Fatalf("expected one finding, got %+v", findings)
	}
	if findings[0].Kind != HeadingKindMissingH1 {
		t.Errorf("Kind = %q, want %q", findings[0].Kind, HeadingKindMissingH1)
	}
	if findings[0].File != "docs/notes.md" {
		t.Errorf("File = %q, want %q", findings[0].File, "docs/notes.md")
	}
	if findings[0].Line != 1 {
		t.Errorf("Line = %d, want 1", findings[0].Line)
	}
}

// --- (c) skipped level in docs/ ---

func TestHeadingHierarchy_DocsFileJumpingH1ToH3_ReportsSkippedLevel(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	writeFixture(t, root, "docs/jump.md", "# Title\n\n### Jumped Here\n")

	findings := scanHierarchy(t, root)

	if len(findings) != 1 {
		t.Fatalf("expected one finding, got %+v", findings)
	}
	if findings[0].Kind != HeadingKindSkippedLevel {
		t.Errorf("Kind = %q, want %q", findings[0].Kind, HeadingKindSkippedLevel)
	}
	if findings[0].File != "docs/jump.md" {
		t.Errorf("File = %q, want %q", findings[0].File, "docs/jump.md")
	}
	if findings[0].Line != 3 {
		t.Errorf("Line = %d, want 3 (skipped-level reports the jumping heading's line)", findings[0].Line)
	}
}

// --- (d) headings inside code fences are ignored ---

func TestHeadingHierarchy_HeadingsInsideCodeFences_ProduceNoFindings(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	writeFixture(t, root, "docs/fenced.md",
		"# Title\n\n```bash\n# not a duplicate h1\n### not a skipped level\n```\n\n## Real Section\n")

	findings := scanHierarchy(t, root)

	if len(findings) != 0 {
		t.Errorf("fenced pseudo-headings must not be findings, got %+v", findings)
	}
}

// --- (e) .claude/agents/ default-denied ---

func TestHeadingHierarchy_ClaudeAgentsFileWithZeroH1s_IsNotScanned(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	writeFixture(t, root, ".claude/agents/swe-rust-dev.md", "## No H1 In Agent Files\n\nbody\n")

	findings := scanHierarchy(t, root)

	if len(findings) != 0 {
		t.Errorf(".claude/agents/ is default-denied, got %+v", findings)
	}
}

// --- (f) SKILL.md under .claude/skills/ default-denied ---

func TestHeadingHierarchy_SkillMdWithManyH1s_IsNotScanned(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	writeFixture(t, root, ".claude/skills/example/SKILL.md", "# One\n\n# Two\n\n# Three\n")

	findings := scanHierarchy(t, root)

	if len(findings) != 0 {
		t.Errorf(".claude/skills/ is default-denied, got %+v", findings)
	}
}

// --- (g) plans/done/ excluded ---

func TestHeadingHierarchy_PlansDoneFileWithSkippedLevel_IsNotScanned(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	writeFixture(t, root, "plans/done/2026-01-01__archived/delivery.md",
		"# Title\n\n### Skipped In Archive\n")

	findings := scanHierarchy(t, root)

	if len(findings) != 0 {
		t.Errorf("plans/done/ is excluded from the allowlist, got %+v", findings)
	}
}

// --- (h) plans/in-progress/ scanned ---

func TestHeadingHierarchy_PlansInProgressFileWithDuplicateH1_ReportsFinding(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	writeFixture(t, root, "plans/in-progress/some-plan/prd.md", "# Plan\n\ntext\n\n# Plan Again\n")

	findings := scanHierarchy(t, root)

	if len(findings) != 1 {
		t.Fatalf("expected one finding, got %+v", findings)
	}
	if findings[0].Kind != HeadingKindDuplicateH1 {
		t.Errorf("Kind = %q, want %q", findings[0].Kind, HeadingKindDuplicateH1)
	}
	if findings[0].File != "plans/in-progress/some-plan/prd.md" {
		t.Errorf("File = %q, want %q", findings[0].File, "plans/in-progress/some-plan/prd.md")
	}
}

// --- (i) apps/<name>/README.md scanned, apps/<name>/src/** denied ---

func TestHeadingHierarchy_AppsReadmeIsScannedWhileAppsSrcIsNot(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	writeFixture(t, root, "apps/example/README.md", "# Example\n\n### Skipped In Readme\n")
	writeFixture(t, root, "apps/example/src/notes.md", "## Zero H1s Here But Default-Denied\n")

	findings := scanHierarchy(t, root)

	if len(findings) != 1 {
		t.Fatalf("only the README finding is expected, got %+v", findings)
	}
	if findings[0].Kind != HeadingKindSkippedLevel {
		t.Errorf("Kind = %q, want %q", findings[0].Kind, HeadingKindSkippedLevel)
	}
	if findings[0].File != "apps/example/README.md" {
		t.Errorf("File = %q, want %q", findings[0].File, "apps/example/README.md")
	}
}

// --- (j) specs/ scanned ---

func TestHeadingHierarchy_SpecsFileWithDuplicateH1_ReportsFinding(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	writeFixture(t, root, "specs/apps/rhino/overview.md", "# Spec\n\n# Spec Duplicate\n")

	findings := scanHierarchy(t, root)

	if len(findings) != 1 {
		t.Fatalf("expected one finding, got %+v", findings)
	}
	if findings[0].Kind != HeadingKindDuplicateH1 {
		t.Errorf("Kind = %q, want %q", findings[0].Kind, HeadingKindDuplicateH1)
	}
	if findings[0].File != "specs/apps/rhino/overview.md" {
		t.Errorf("File = %q, want %q", findings[0].File, "specs/apps/rhino/overview.md")
	}
}

// --- (k) --exclude subtracts on top of the allowlist ---

func TestHeadingHierarchy_ExcludePrefixSuppressesDocs_OtherAllowlistTreesStillReport(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	writeFixture(t, root, "docs/excluded.md", "## Missing H1 In Docs\n")
	writeFixture(t, root, "repo-governance/rule.md", "## Missing H1 In Governance\n")

	findings, err := ValidateHeadingHierarchy(HeadingScanOptions{
		Root:    root,
		Exclude: []string{"docs"},
	})
	if err != nil {
		t.Fatalf("ValidateHeadingHierarchy() error: %v", err)
	}

	if len(findings) != 1 {
		t.Fatalf("only the non-excluded tree should report, got %+v", findings)
	}
	if findings[0].Kind != HeadingKindMissingH1 {
		t.Errorf("Kind = %q, want %q", findings[0].Kind, HeadingKindMissingH1)
	}
	if findings[0].File != "repo-governance/rule.md" {
		t.Errorf("File = %q, want %q", findings[0].File, "repo-governance/rule.md")
	}
}

// --- Allowlist predicate unit tests ---

func TestIsProseAllowlisted(t *testing.T) {
	tests := []struct {
		name    string
		repoRel string
		want    bool
	}{
		// docs/ tree.
		{"accepts docs file", "docs/guide.md", true},
		{"accepts deep docs file", "docs/explanation/deep/file.md", true},
		// repo-governance/ tree.
		{"accepts repo-governance file", "repo-governance/principles/README.md", true},
		// specs/ tree.
		{"accepts specs file", "specs/apps/rhino/overview.md", true},
		// plans/ minus plans/done/.
		{"accepts plans root file", "plans/README.md", true},
		{"accepts plans in-progress file", "plans/in-progress/x/prd.md", true},
		{"accepts plans backlog file", "plans/backlog/idea.md", true},
		{"denies plans done file", "plans/done/2026-01-01__x/prd.md", false},
		// Root-level *.md only.
		{"accepts root README", "README.md", true},
		{"accepts root AGENTS", "AGENTS.md", true},
		{"denies root non-markdown", "README.txt", false},
		// apps|libs README.
		{"accepts apps README", "apps/example/README.md", true},
		{"accepts libs README", "libs/ts-utils/README.md", true},
		// apps|libs docs subtree.
		{"accepts apps docs file", "apps/example/docs/design.md", true},
		{"accepts libs deep docs file", "libs/ts-utils/docs/api/usage.md", true},
		// Default-deny everything else.
		{"denies claude agents file", ".claude/agents/swe-rust-dev.md", false},
		{"denies claude skill file", ".claude/skills/example/SKILL.md", false},
		{"denies opencode agents file", ".opencode/agents/docs-maker.md", false},
		{"denies apps src file", "apps/example/src/notes.md", false},
		{"denies libs src README", "libs/ts-utils/src/README.md", false},
		{"denies node_modules README", "node_modules/pkg/README.md", false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			defer failOnPanic(t)
			if got := IsProseAllowlisted(tt.repoRel); got != tt.want {
				t.Errorf("IsProseAllowlisted(%q) = %v, want %v", tt.repoRel, got, tt.want)
			}
		})
	}
}

// --- Explicit-paths mode (positional / staged inputs) ---
// Mirrors the Rust canonical tests `explicit_paths_still_apply_the_allowlist`,
// `explicit_directory_path_is_walked`, and `explicit_missing_path_errors`.

func TestHeadingHierarchy_ExplicitPathsStillApplyTheAllowlist(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	writeFixture(t, root, ".claude/skills/example/SKILL.md", "# One\n\n# Two\n")
	writeFixture(t, root, "docs/dup.md", "# A\n\n# B\n")

	findings, err := ValidateHeadingHierarchy(HeadingScanOptions{
		Root:  root,
		Paths: []string{".claude/skills/example/SKILL.md", "docs/dup.md"},
	})
	if err != nil {
		t.Fatalf("ValidateHeadingHierarchy() error: %v", err)
	}

	if len(findings) != 1 {
		t.Fatalf("skill file must be denied, got %+v", findings)
	}
	if findings[0].File != "docs/dup.md" {
		t.Errorf("File = %q, want %q", findings[0].File, "docs/dup.md")
	}
	if findings[0].Kind != HeadingKindDuplicateH1 {
		t.Errorf("Kind = %q, want %q", findings[0].Kind, HeadingKindDuplicateH1)
	}
}

func TestHeadingHierarchy_ExplicitDirectoryPathIsWalked(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	writeFixture(t, root, "docs/sub/no-h1.md", "## Section Only\n")
	writeFixture(t, root, "repo-governance/no-h1.md", "## Also Missing\n")

	findings, err := ValidateHeadingHierarchy(HeadingScanOptions{
		Root:  root,
		Paths: []string{"docs"},
	})
	if err != nil {
		t.Fatalf("ValidateHeadingHierarchy() error: %v", err)
	}

	if len(findings) != 1 {
		t.Fatalf("only the docs tree was requested, got %+v", findings)
	}
	if findings[0].File != "docs/sub/no-h1.md" {
		t.Errorf("File = %q, want %q", findings[0].File, "docs/sub/no-h1.md")
	}
	if findings[0].Kind != HeadingKindMissingH1 {
		t.Errorf("Kind = %q, want %q", findings[0].Kind, HeadingKindMissingH1)
	}
}

func TestHeadingHierarchy_ExplicitMissingPathErrors(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()

	_, err := ValidateHeadingHierarchy(HeadingScanOptions{
		Root:  root,
		Paths: []string{"docs/does-not-exist.md"},
	})
	if err == nil {
		t.Fatal("expected error for missing explicit path")
	}
}

func TestHeadingHierarchy_ExplicitAbsolutePathInsideRootIsScanned(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	writeFixture(t, root, "docs/dup.md", "# A\n\n# B\n")

	findings, err := ValidateHeadingHierarchy(HeadingScanOptions{
		Root:  root,
		Paths: []string{filepath.Join(root, "docs/dup.md")},
	})
	if err != nil {
		t.Fatalf("ValidateHeadingHierarchy() error: %v", err)
	}

	if len(findings) != 1 || findings[0].File != "docs/dup.md" {
		t.Fatalf("expected the docs finding, got %+v", findings)
	}
}

func TestHeadingHierarchy_ExplicitAbsolutePathOutsideRootIsDenied(t *testing.T) {
	defer failOnPanic(t)
	root := t.TempDir()
	outside := t.TempDir()
	writeFixture(t, outside, "docs/dup.md", "# A\n\n# B\n")

	findings, err := ValidateHeadingHierarchy(HeadingScanOptions{
		Root:  root,
		Paths: []string{filepath.Join(outside, "docs/dup.md")},
	})
	if err != nil {
		t.Fatalf("ValidateHeadingHierarchy() error: %v", err)
	}

	if len(findings) != 0 {
		t.Errorf("paths outside root fail the allowlist, got %+v", findings)
	}
}

// --- Report formatting ---
// Mirrors the Rust canonical reporter tests byte-for-byte.

func sampleHeadingFindings() []HeadingFinding {
	return []HeadingFinding{
		{
			File:    "docs/a.md",
			Line:    1,
			Kind:    HeadingKindMissingH1,
			Message: "file has no H1 heading",
		},
		{
			File:    "docs/b.md",
			Line:    5,
			Kind:    HeadingKindDuplicateH1,
			Message: `duplicate H1 "Two" (first H1 at line 1)`,
		},
	}
}

func TestFormatHeadingText_NoFindingsShowsSuccess(t *testing.T) {
	defer failOnPanic(t)
	got := FormatHeadingText(nil, false)
	want := "✓ All heading hierarchies valid! No findings found.\n"
	if got != want {
		t.Errorf("FormatHeadingText(nil, false) = %q, want %q", got, want)
	}
}

func TestFormatHeadingText_NoFindingsQuietIsEmpty(t *testing.T) {
	defer failOnPanic(t)
	if got := FormatHeadingText(nil, true); got != "" {
		t.Errorf("FormatHeadingText(nil, true) = %q, want empty", got)
	}
}

func TestFormatHeadingText_ReportGroupsFindingsByFile(t *testing.T) {
	defer failOnPanic(t)
	s := FormatHeadingText(sampleHeadingFindings(), false)

	if !strings.HasPrefix(s, "# Heading Hierarchy Report\n\n") {
		t.Errorf("missing report header, got: %s", s)
	}
	for _, want := range []string{
		"**Total findings**: 2\n",
		"\n## docs/a.md\n\n",
		"- Line 1: missing-h1: file has no H1 heading\n",
		"\n## docs/b.md\n\n",
		"- Line 5: duplicate-h1: duplicate H1 \"Two\" (first H1 at line 1)\n",
	} {
		if !strings.Contains(s, want) {
			t.Errorf("missing %q, got: %s", want, s)
		}
	}
}

func TestFormatHeadingJSON_SuccessEmptyFindings(t *testing.T) {
	defer failOnPanic(t)
	s, err := FormatHeadingJSON(nil)
	if err != nil {
		t.Fatalf("FormatHeadingJSON() error: %v", err)
	}

	var v map[string]any
	if err := json.Unmarshal([]byte(s), &v); err != nil {
		t.Fatalf("output is not valid JSON: %v\n%s", err, s)
	}
	if v["status"] != "success" {
		t.Errorf("status = %v, want success", v["status"])
	}
	if v["total_findings"] != float64(0) {
		t.Errorf("total_findings = %v, want 0", v["total_findings"])
	}
	findings, ok := v["findings"].([]any)
	if !ok || len(findings) != 0 {
		t.Errorf("findings must be an empty array (not null), got: %s", s)
	}
}

func TestFormatHeadingJSON_FailureWithFindings(t *testing.T) {
	defer failOnPanic(t)
	s, err := FormatHeadingJSON(sampleHeadingFindings())
	if err != nil {
		t.Fatalf("FormatHeadingJSON() error: %v", err)
	}

	var v map[string]any
	if err := json.Unmarshal([]byte(s), &v); err != nil {
		t.Fatalf("output is not valid JSON: %v\n%s", err, s)
	}
	if v["status"] != "failure" {
		t.Errorf("status = %v, want failure", v["status"])
	}
	if v["total_findings"] != float64(2) {
		t.Errorf("total_findings = %v, want 2", v["total_findings"])
	}
	findings, ok := v["findings"].([]any)
	if !ok || len(findings) != 2 {
		t.Fatalf("findings must be an array of 2, got: %s", s)
	}
	first, ok := findings[0].(map[string]any)
	if !ok {
		t.Fatalf("findings[0] must be an object, got: %s", s)
	}
	if first["file"] != "docs/a.md" {
		t.Errorf("findings[0].file = %v, want docs/a.md", first["file"])
	}
	if first["kind"] != "missing-h1" {
		t.Errorf("findings[0].kind = %v, want missing-h1", first["kind"])
	}
	second, ok := findings[1].(map[string]any)
	if !ok {
		t.Fatalf("findings[1] must be an object, got: %s", s)
	}
	if second["line"] != float64(5) {
		t.Errorf("findings[1].line = %v, want 5", second["line"])
	}
}

func TestFormatHeadingMarkdown_DelegatesToText(t *testing.T) {
	defer failOnPanic(t)
	f := sampleHeadingFindings()
	if got, want := FormatHeadingMarkdown(f), FormatHeadingText(f, false); got != want {
		t.Errorf("FormatHeadingMarkdown() = %q, want %q", got, want)
	}
}
