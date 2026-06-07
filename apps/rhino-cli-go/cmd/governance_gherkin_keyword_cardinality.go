package cmd

import (
	"encoding/json"
	"fmt"
	"path/filepath"
	"strings"

	"github.com/spf13/cobra"
	governance "github.com/wahidyankf/ose-public/apps/rhino-cli/internal/repo-governance"
)

// governanceGherkinCardinalityFn is the test-mockable entrypoint for the
// gherkin keyword-cardinality audit.
var governanceGherkinCardinalityFn = governanceGherkinKeywordCardinality

var governanceGherkinKeywordCardinalityCmd = &cobra.Command{
	Use:   "gherkin-keyword-cardinality [path]",
	Short: "Audit feature files for scenarios with repeated primary Gherkin keywords",
	Long: `Scan all .feature files under [path] (default: the repository root) and flag
every Scenario whose body uses more than one primary Given, When, or Then
keyword line. Extra steps must chain with And/But.

The audit respects several exemptions:
  - Background blocks (a Background may repeat primary keywords)
  - Scenario Outline Examples tables
  - Lines inside doc-strings (""" or fenced) and comment lines (#)

The walk skips build outputs (bin/, build/, target/, dist/, node_modules/),
worktrees/, archived/, and the BDD-library self-test fixture trees
(libs/elixir-cabbage/test/features/, libs/elixir-gherkin/test/fixtures/).

Exits with code 1 if any violations are found, 0 if clean.`,
	Example: `  # Audit the entire repository
  rhino-cli repo-governance gherkin-keyword-cardinality

  # Audit a specific path
  rhino-cli repo-governance gherkin-keyword-cardinality specs/apps/crud

  # Output as JSON
  rhino-cli repo-governance gherkin-keyword-cardinality -o json`,
	SilenceErrors: true,
	RunE:          runGovernanceGherkinKeywordCardinality,
}

func init() {
	repoGovernanceCmd.AddCommand(governanceGherkinKeywordCardinalityCmd)
}

func runGovernanceGherkinKeywordCardinality(cmd *cobra.Command, args []string) error {
	repoRoot, err := findGitRoot()
	if err != nil {
		return fmt.Errorf("failed to find git repository root: %w", err)
	}

	scanPath := "."
	if len(args) > 0 {
		scanPath = args[0]
	}
	fullPath := filepath.Join(repoRoot, scanPath)

	findings, err := governanceGherkinCardinalityFn(fullPath)
	if err != nil {
		return fmt.Errorf("gherkin keyword cardinality audit failed: %w", err)
	}

	if err := writeFormatted(cmd, output, verbose, quiet, outputFuncs{
		text:     func(v, q bool) string { return formatGherkinCardinalityText(findings) },
		json:     func() (string, error) { return formatGherkinCardinalityJSON(findings) },
		markdown: func() string { return formatGherkinCardinalityMarkdown(findings) },
	}); err != nil {
		return err
	}

	if len(findings) > 0 {
		return fmt.Errorf("%d violation(s) found", len(findings))
	}
	return nil
}

// governanceGherkinKeywordCardinality is the real implementation that
// delegates to the internal governance package.
func governanceGherkinKeywordCardinality(root string) ([]governance.CardinalityFinding, error) {
	return governance.WalkFeatures(root)
}

// formatGherkinCardinalityText formats the findings as human-readable text.
func formatGherkinCardinalityText(findings []governance.CardinalityFinding) string {
	if len(findings) == 0 {
		return "GHERKIN KEYWORD CARDINALITY AUDIT PASSED: no violations found\n"
	}
	var sb strings.Builder
	fmt.Fprintf(&sb, "GHERKIN KEYWORD CARDINALITY AUDIT FAILED: %d violation(s) found\n", len(findings))
	for _, f := range findings {
		fmt.Fprintf(&sb, "  %s:%d  %s  →  %s\n", f.Path, f.Line, f.Scenario, f.Detail)
	}
	return sb.String()
}

// formatGherkinCardinalityJSON formats the findings as JSON.
func formatGherkinCardinalityJSON(findings []governance.CardinalityFinding) (string, error) {
	type jsonFinding struct {
		Path     string `json:"path"`
		Line     int    `json:"line"`
		Scenario string `json:"scenario"`
		Detail   string `json:"detail"`
	}
	type jsonResult struct {
		Status   string        `json:"status"`
		Count    int           `json:"count"`
		Findings []jsonFinding `json:"findings"`
	}

	status := "passed"
	if len(findings) > 0 {
		status = "failed"
	}

	jf := make([]jsonFinding, 0, len(findings))
	for _, f := range findings {
		jf = append(jf, jsonFinding{
			Path:     f.Path,
			Line:     f.Line,
			Scenario: f.Scenario,
			Detail:   f.Detail,
		})
	}

	result := jsonResult{
		Status:   status,
		Count:    len(findings),
		Findings: jf,
	}

	data, err := json.MarshalIndent(result, "", "  ")
	if err != nil {
		return "", err
	}
	return string(data) + "\n", nil
}

// formatGherkinCardinalityMarkdown formats the findings as a markdown table.
func formatGherkinCardinalityMarkdown(findings []governance.CardinalityFinding) string {
	if len(findings) == 0 {
		return "## Gherkin Keyword Cardinality Audit\n\n**PASSED**: no violations found\n"
	}
	var sb strings.Builder
	fmt.Fprintf(&sb, "## Gherkin Keyword Cardinality Audit\n\n**FAILED**: %d violation(s) found\n\n", len(findings))
	sb.WriteString("| File | Line | Scenario | Violation |\n")
	sb.WriteString("|------|------|----------|-----------|\n")
	for _, f := range findings {
		fmt.Fprintf(&sb, "| %s | %d | %s | %s |\n", f.Path, f.Line, f.Scenario, f.Detail)
	}
	return sb.String()
}
