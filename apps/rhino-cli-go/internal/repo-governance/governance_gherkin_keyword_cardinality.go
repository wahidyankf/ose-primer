package repogovernance

// Gherkin step-keyword cardinality audit.
//
// Flags every Scenario (and Scenario Outline body) that uses more than one
// primary Given, When, or Then keyword line. Primary keywords start the
// trimmed line; And/But/* chains are not counted. Background blocks and
// Scenario Outline Examples tables are exempt, and lines inside doc-strings
// (""" or ```) and comment lines (#) are ignored.

import (
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

// CardinalityFinding describes a single scenario violating the one-each
// primary-keyword rule.
type CardinalityFinding struct {
	Path     string
	Line     int
	Scenario string
	Detail   string
}

// cardinalityExcludedDirs are directory names skipped at any depth during the
// feature-file walk (build outputs, dependency trees, worktrees, archives).
var cardinalityExcludedDirs = map[string]bool{
	"bin":          true,
	"build":        true,
	"target":       true,
	"dist":         true,
	"node_modules": true,
	"worktrees":    true,
	"archived":     true,
}

// cardinalityExcludedPathParts are slash-path fragments identifying
// BDD-library self-test fixture trees, excluded wherever they appear (those
// fixtures test the Gherkin parser itself and may deliberately use odd shapes).
var cardinalityExcludedPathParts = []string{
	"libs/elixir-cabbage/test/features/",
	"libs/elixir-gherkin/test/fixtures/",
}

// ScanFeatureFile reads the feature file at path and returns all cardinality
// findings.
func ScanFeatureFile(path string) ([]CardinalityFinding, error) {
	data, err := os.ReadFile(path) //nolint:gosec // trusted repo path
	if err != nil {
		return nil, fmt.Errorf("read %s: %w", path, err)
	}
	return ScanFeatureContent(path, string(data)), nil
}

// WalkFeatures walks all .feature files under root recursively and returns
// all findings sorted by (path, line). Excluded directory names and
// BDD-library fixture trees are skipped. A missing root yields an empty
// slice, not an error.
func WalkFeatures(root string) ([]CardinalityFinding, error) {
	var findings []CardinalityFinding
	err := filepath.WalkDir(root, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			if os.IsNotExist(err) {
				return filepath.SkipAll
			}
			return err
		}
		if d.IsDir() {
			if path != root && cardinalityExcludedDirs[d.Name()] {
				return filepath.SkipDir
			}
			return nil
		}
		if !strings.HasSuffix(d.Name(), ".feature") {
			return nil
		}
		slashed := filepath.ToSlash(path)
		for _, part := range cardinalityExcludedPathParts {
			if strings.Contains(slashed, part) {
				return nil
			}
		}
		ff, err := ScanFeatureFile(path)
		if err != nil {
			return err
		}
		findings = append(findings, ff...)
		return nil
	})
	if err != nil {
		return nil, err
	}
	sortCardinalityFindings(findings)
	return findings, nil
}

// sortCardinalityFindings orders findings by (path, line) ascending.
func sortCardinalityFindings(findings []CardinalityFinding) {
	sort.SliceStable(findings, func(i, j int) bool {
		if findings[i].Path != findings[j].Path {
			return findings[i].Path < findings[j].Path
		}
		return findings[i].Line < findings[j].Line
	})
}

// ScanFeatureContent performs the core line-by-line scan of a feature file's
// content, tracking doc-string state and the current scenario block.
func ScanFeatureContent(path, content string) []CardinalityFinding {
	lines := strings.Split(content, "\n")

	var findings []CardinalityFinding

	inDocString := false
	docStringDelim := ""

	inScenario := false
	scenarioName := ""
	scenarioLine := 0
	givenCount, whenCount, thenCount := 0, 0, 0

	flush := func() {
		if !inScenario {
			return
		}
		if detail := cardinalityDetail(givenCount, whenCount, thenCount); detail != "" {
			findings = append(findings, CardinalityFinding{
				Path:     path,
				Line:     scenarioLine,
				Scenario: scenarioName,
				Detail:   detail,
			})
		}
		inScenario = false
	}

	startScenario := func(name string, line int) {
		flush()
		inScenario = true
		scenarioName = name
		scenarioLine = line
		givenCount, whenCount, thenCount = 0, 0, 0
	}

	for i, line := range lines {
		lineNum := i + 1
		trimmed := strings.TrimSpace(line)

		// ── Doc-strings (""" or ```) ──────────────────────────────────────────
		if inDocString {
			if strings.HasPrefix(trimmed, docStringDelim) {
				inDocString = false
			}
			continue
		}
		if delim := docStringDelimiter(trimmed); delim != "" {
			inDocString = true
			docStringDelim = delim
			continue
		}

		// ── Comments ──────────────────────────────────────────────────────────
		if strings.HasPrefix(trimmed, "#") {
			continue
		}

		// ── Block headers ─────────────────────────────────────────────────────
		switch {
		case strings.HasPrefix(trimmed, "Scenario Outline:"):
			startScenario(headerName(trimmed, "Scenario Outline:"), lineNum)
			continue
		case strings.HasPrefix(trimmed, "Scenario Template:"):
			startScenario(headerName(trimmed, "Scenario Template:"), lineNum)
			continue
		case strings.HasPrefix(trimmed, "Scenario:"):
			startScenario(headerName(trimmed, "Scenario:"), lineNum)
			continue
		case strings.HasPrefix(trimmed, "Example:"):
			startScenario(headerName(trimmed, "Example:"), lineNum)
			continue
		case strings.HasPrefix(trimmed, "Background:"),
			strings.HasPrefix(trimmed, "Examples:"),
			strings.HasPrefix(trimmed, "Scenarios:"),
			strings.HasPrefix(trimmed, "Rule:"),
			strings.HasPrefix(trimmed, "Feature:"):
			// Exempt regions (Background, Examples tables) and structural
			// headers end the current scenario's counted body.
			flush()
			continue
		}

		// ── Primary keyword counting ──────────────────────────────────────────
		if !inScenario {
			continue
		}
		switch primaryKeyword(trimmed) {
		case "Given":
			givenCount++
		case "When":
			whenCount++
		case "Then":
			thenCount++
		}
	}
	flush()
	return findings
}

// primaryKeyword classifies a trimmed step line, returning the primary
// keyword starting it ("Given", "When", "Then") or "" when the line is a
// chain step (And/But/*) or not a step at all.
func primaryKeyword(trimmed string) string {
	for _, kw := range []string{"Given", "When", "Then"} {
		if strings.HasPrefix(trimmed, kw+" ") {
			return kw
		}
	}
	return ""
}

// headerName extracts the trimmed name following a block-header keyword.
func headerName(trimmed, header string) string {
	return strings.TrimSpace(strings.TrimPrefix(trimmed, header))
}

// docStringDelimiter returns the doc-string delimiter opening on this trimmed
// line (`"""` or "```", possibly followed by a content type), or "" if the
// line does not open a doc-string.
func docStringDelimiter(trimmed string) string {
	if strings.HasPrefix(trimmed, `"""`) {
		return `"""`
	}
	if strings.HasPrefix(trimmed, "```") {
		return "```"
	}
	return ""
}

// cardinalityDetail renders the repeated-keyword summary (e.g. "2 When, 2
// Then") for counts above one, in Given/When/Then order. Returns "" when the
// scenario conforms.
func cardinalityDetail(given, when, then int) string {
	var parts []string
	if given > 1 {
		parts = append(parts, fmt.Sprintf("%d Given", given))
	}
	if when > 1 {
		parts = append(parts, fmt.Sprintf("%d When", when))
	}
	if then > 1 {
		parts = append(parts, fmt.Sprintf("%d Then", then))
	}
	return strings.Join(parts, ", ")
}
