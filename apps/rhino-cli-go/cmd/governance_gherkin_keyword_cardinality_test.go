package cmd

import (
	"bytes"
	"context"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"testing"

	"github.com/cucumber/godog"
	governance "github.com/wahidyankf/ose-public/apps/rhino-cli/internal/repo-governance"
)

// Step constant patterns for repo-governance gherkin-keyword-cardinality
// scenarios. The Given steps drive the REAL scanner over synthetic feature
// content so the bound behavior is the production parser, not a stub.
const (
	stepFeatureFileWithTwoPrimaryWhens        = `^a feature file containing a scenario with two primary "When" keywords$`
	stepFeatureFileConformingChained          = `^a feature file whose scenarios each use one primary keyword chained with "And"$`
	stepFeatureFileBackgroundRepeatsGiven     = `^a feature file whose Background block repeats the "Given" keyword$`
	stepFeatureFileOutlineWithExamples        = `^a feature file with a Scenario Outline whose Examples table has many rows$`
	stepFeatureFileDocstringsAndComments      = `^a feature file whose doc-strings and comments contain primary keyword words$`
	stepDirectoryOfConformingFeatureFiles     = `^a directory of feature files that all obey the one-each keyword rule$`
	stepDeveloperRunsGherkinCardinalityOnFile = `^the developer runs repo-governance gherkin-keyword-cardinality on the file$`
	stepDeveloperRunsGherkinCardinalityOnDir  = `^the developer runs repo-governance gherkin-keyword-cardinality on the directory$`
	stepOutputNamesOffendingFileAndScenario   = `^the output names the offending file and scenario$`
	stepOutputReportsZeroCardinalityFindings  = `^the output reports zero cardinality findings$`
)

var specsDirUnitGovernanceGherkinCardinality = func() string {
	_, f, _, _ := runtime.Caller(0)
	return filepath.Join(filepath.Dir(f), "../../../specs/apps/rhino/behavior/cli/gherkin")
}()

const gherkinCardinalityViolatingFixture = "Feature: Fixture\n\n" +
	"  Scenario: Double when offender\n" +
	"    Given a start\n" +
	"    When the first action runs\n" +
	"    When the second action runs\n" +
	"    Then the outcome is checked\n"

const gherkinCardinalityConformingFixture = "Feature: Fixture\n\n" +
	"  Scenario: Conforming chained scenario\n" +
	"    Given a start\n" +
	"    And another precondition\n" +
	"    When the action runs\n" +
	"    Then the outcome is checked\n" +
	"    And a second outcome is checked\n" +
	"    But a third outcome is absent\n"

const gherkinCardinalityBackgroundFixture = "Feature: Fixture\n\n" +
	"  Background:\n" +
	"    Given one precondition\n" +
	"    Given another precondition\n\n" +
	"  Scenario: Conforming body\n" +
	"    Given a thing\n" +
	"    When it acts\n" +
	"    Then it is checked\n"

const gherkinCardinalityOutlineFixture = "Feature: Fixture\n\n" +
	"  Scenario Outline: Outline body obeys the rule\n" +
	"    Given a value <v>\n" +
	"    When it is processed\n" +
	"    Then it succeeds\n\n" +
	"    Examples:\n" +
	"      | v |\n" +
	"      | 1 |\n" +
	"      | 2 |\n" +
	"      | 3 |\n"

const gherkinCardinalityDocstringFixture = "Feature: Fixture\n\n" +
	"  Scenario: Docstring and comment heavy\n" +
	"    Given a setup\n" +
	"    When something runs with this payload\n" +
	"      \"\"\"\n" +
	"      When this line is data, not a step\n" +
	"      Then neither is this one\n" +
	"      \"\"\"\n" +
	"    # Then this comment line is ignored\n" +
	"    Then the result is checked\n"

type governanceGherkinCardinalityUnitSteps struct {
	cmdErr    error
	cmdOutput string
	tmpDir    string
}

func (s *governanceGherkinCardinalityUnitSteps) before(_ context.Context, _ *godog.Scenario) (context.Context, error) {
	verbose = false
	quiet = false
	output = "text"
	s.cmdErr = nil
	s.cmdOutput = ""
	s.tmpDir = ""

	osGetwd = func() (string, error) { return "/mock-repo", nil }
	osStat = func(name string) (os.FileInfo, error) {
		if name == "/mock-repo/.git" {
			return &mockFileInfo{name: ".git", isDir: true}, nil
		}
		return nil, os.ErrNotExist
	}
	governanceGherkinCardinalityFn = func(_ string) ([]governance.CardinalityFinding, error) { return nil, nil }
	return context.Background(), nil
}

func (s *governanceGherkinCardinalityUnitSteps) after(_ context.Context, _ *godog.Scenario, _ error) (context.Context, error) {
	governanceGherkinCardinalityFn = governanceGherkinKeywordCardinality
	osGetwd = os.Getwd
	osStat = os.Stat
	if s.tmpDir != "" {
		_ = os.RemoveAll(s.tmpDir)
		s.tmpDir = ""
	}
	return context.Background(), nil
}

// mockScanContent points the audit fn at the REAL scanner over the given
// synthetic feature content.
func (s *governanceGherkinCardinalityUnitSteps) mockScanContent(content string) {
	governanceGherkinCardinalityFn = func(_ string) ([]governance.CardinalityFinding, error) {
		return governance.ScanFeatureContent("specs/fixture/violating.feature", content), nil
	}
}

func (s *governanceGherkinCardinalityUnitSteps) fileWithTwoPrimaryWhens() error {
	s.mockScanContent(gherkinCardinalityViolatingFixture)
	return nil
}

func (s *governanceGherkinCardinalityUnitSteps) fileConformingChained() error {
	s.mockScanContent(gherkinCardinalityConformingFixture)
	return nil
}

func (s *governanceGherkinCardinalityUnitSteps) fileBackgroundRepeatsGiven() error {
	s.mockScanContent(gherkinCardinalityBackgroundFixture)
	return nil
}

func (s *governanceGherkinCardinalityUnitSteps) fileOutlineWithExamples() error {
	s.mockScanContent(gherkinCardinalityOutlineFixture)
	return nil
}

func (s *governanceGherkinCardinalityUnitSteps) fileDocstringsAndComments() error {
	s.mockScanContent(gherkinCardinalityDocstringFixture)
	return nil
}

func (s *governanceGherkinCardinalityUnitSteps) directoryOfConformingFeatures() error {
	dir, err := os.MkdirTemp("", "gherkin-cardinality-dir-*")
	if err != nil {
		return err
	}
	s.tmpDir = dir
	for name, content := range map[string]string{
		"a.feature": gherkinCardinalityConformingFixture,
		"b.feature": gherkinCardinalityBackgroundFixture,
	} {
		if err := os.WriteFile(filepath.Join(dir, name), []byte(content), 0o644); err != nil {
			return err
		}
	}
	governanceGherkinCardinalityFn = func(_ string) ([]governance.CardinalityFinding, error) {
		return governance.WalkFeatures(dir)
	}
	return nil
}

func (s *governanceGherkinCardinalityUnitSteps) run() error {
	buf := new(bytes.Buffer)
	governanceGherkinKeywordCardinalityCmd.SetOut(buf)
	governanceGherkinKeywordCardinalityCmd.SetErr(buf)
	s.cmdErr = governanceGherkinKeywordCardinalityCmd.RunE(governanceGherkinKeywordCardinalityCmd, []string{"specs/"})
	s.cmdOutput = buf.String()
	return nil
}

func (s *governanceGherkinCardinalityUnitSteps) exitsSuccessfully() error {
	if s.cmdErr != nil {
		return fmt.Errorf("expected success but got: %w\nOutput: %s", s.cmdErr, s.cmdOutput)
	}
	return nil
}

func (s *governanceGherkinCardinalityUnitSteps) exitsWithFailure() error {
	if s.cmdErr == nil {
		return fmt.Errorf("expected failure but succeeded\nOutput: %s", s.cmdOutput)
	}
	return nil
}

func (s *governanceGherkinCardinalityUnitSteps) outputNamesOffendingFileAndScenario() error {
	if !strings.Contains(s.cmdOutput, "GHERKIN KEYWORD CARDINALITY AUDIT FAILED") {
		return fmt.Errorf("expected FAILED header in output, got: %s", s.cmdOutput)
	}
	if !strings.Contains(s.cmdOutput, "violating.feature") {
		return fmt.Errorf("expected output to name the offending file, got: %s", s.cmdOutput)
	}
	if !strings.Contains(s.cmdOutput, "Double when offender") {
		return fmt.Errorf("expected output to name the offending scenario, got: %s", s.cmdOutput)
	}
	return nil
}

func (s *governanceGherkinCardinalityUnitSteps) outputReportsZeroCardinalityFindings() error {
	if !strings.Contains(s.cmdOutput, "GHERKIN KEYWORD CARDINALITY AUDIT PASSED: no violations found") {
		return fmt.Errorf("expected PASSED output, got: %s", s.cmdOutput)
	}
	return nil
}

func TestUnitGovernanceGherkinKeywordCardinality(t *testing.T) {
	s := &governanceGherkinCardinalityUnitSteps{}
	suite := godog.TestSuite{
		ScenarioInitializer: func(sc *godog.ScenarioContext) {
			sc.Before(s.before)
			sc.After(s.after)
			sc.Step(stepFeatureFileWithTwoPrimaryWhens, s.fileWithTwoPrimaryWhens)
			sc.Step(stepFeatureFileConformingChained, s.fileConformingChained)
			sc.Step(stepFeatureFileBackgroundRepeatsGiven, s.fileBackgroundRepeatsGiven)
			sc.Step(stepFeatureFileOutlineWithExamples, s.fileOutlineWithExamples)
			sc.Step(stepFeatureFileDocstringsAndComments, s.fileDocstringsAndComments)
			sc.Step(stepDirectoryOfConformingFeatureFiles, s.directoryOfConformingFeatures)
			sc.Step(stepDeveloperRunsGherkinCardinalityOnFile, s.run)
			sc.Step(stepDeveloperRunsGherkinCardinalityOnDir, s.run)
			sc.Step(stepOutputNamesOffendingFileAndScenario, s.outputNamesOffendingFileAndScenario)
			sc.Step(stepOutputReportsZeroCardinalityFindings, s.outputReportsZeroCardinalityFindings)
			sc.Step(stepExitsSuccessfully, s.exitsSuccessfully)
			sc.Step(stepExitsWithFailure, s.exitsWithFailure)
		},
		Options: &godog.Options{
			Format:   "pretty",
			Paths:    []string{specsDirUnitGovernanceGherkinCardinality},
			TestingT: t,
			Tags:     "repo-governance-gherkin-keyword-cardinality",
		},
	}
	if suite.Run() != 0 {
		t.Fatal("non-zero status returned, failed to run unit feature tests")
	}
}

// TestGovernanceGherkinCardinality_MissingGitRoot verifies the command fails
// gracefully when not inside a git repository.
func TestGovernanceGherkinCardinality_MissingGitRoot(t *testing.T) {
	origGetwd := osGetwd
	origStat := osStat
	defer func() {
		osGetwd = origGetwd
		osStat = origStat
	}()

	osGetwd = func() (string, error) { return "/no-git-here", nil }
	osStat = func(_ string) (os.FileInfo, error) { return nil, os.ErrNotExist }

	buf := new(bytes.Buffer)
	governanceGherkinKeywordCardinalityCmd.SetOut(buf)
	governanceGherkinKeywordCardinalityCmd.SetErr(buf)

	err := governanceGherkinKeywordCardinalityCmd.RunE(governanceGherkinKeywordCardinalityCmd, []string{})
	if err == nil || !strings.Contains(err.Error(), "git") {
		t.Fatalf("expected git-root error, got: %v", err)
	}
}

// TestGovernanceGherkinCardinality_RealTree exercises the real filesystem
// walker against a small tmp fixture to verify the walk + exclusion logic.
func TestGovernanceGherkinCardinality_RealTree(t *testing.T) {
	tmp := t.TempDir()

	writeFile := func(path, content string) {
		if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
			t.Fatal(err)
		}
	}

	writeFile(filepath.Join(tmp, "specs", "bad.feature"), gherkinCardinalityViolatingFixture)
	writeFile(filepath.Join(tmp, "specs", "good.feature"), gherkinCardinalityConformingFixture)
	// Excluded directory — violations here must not be reported.
	writeFile(filepath.Join(tmp, "node_modules", "dep.feature"), gherkinCardinalityViolatingFixture)

	findings, err := governanceGherkinKeywordCardinality(tmp)
	if err != nil {
		t.Fatalf("governanceGherkinKeywordCardinality: %v", err)
	}

	if len(findings) != 1 {
		t.Fatalf("expected exactly 1 finding, got %d: %+v", len(findings), findings)
	}
	if !strings.HasSuffix(filepath.ToSlash(findings[0].Path), "specs/bad.feature") {
		t.Errorf("expected finding in specs/bad.feature, got %q", findings[0].Path)
	}
}

// TestGovernanceGherkinCardinality_OutputFormats checks that all three output
// formats produce non-empty output.
func TestGovernanceGherkinCardinality_OutputFormats(t *testing.T) {
	origGetwd := osGetwd
	origStat := osStat
	origFn := governanceGherkinCardinalityFn
	defer func() {
		osGetwd = origGetwd
		osStat = origStat
		governanceGherkinCardinalityFn = origFn
	}()

	osGetwd = func() (string, error) { return "/mock-repo", nil }
	osStat = func(name string) (os.FileInfo, error) {
		if name == "/mock-repo/.git" {
			return &mockFileInfo{name: ".git", isDir: true}, nil
		}
		return nil, os.ErrNotExist
	}
	governanceGherkinCardinalityFn = func(_ string) ([]governance.CardinalityFinding, error) {
		return []governance.CardinalityFinding{{
			Path:     "/mock-repo/specs/foo.feature",
			Line:     12,
			Scenario: "Double when offender",
			Detail:   "2 When",
		}}, nil
	}

	for _, format := range []string{"json", "markdown", "text"} {
		t.Run(format, func(t *testing.T) {
			buf := new(bytes.Buffer)
			governanceGherkinKeywordCardinalityCmd.SetOut(buf)
			governanceGherkinKeywordCardinalityCmd.SetErr(buf)
			output = format
			verbose = false
			quiet = false
			_ = governanceGherkinKeywordCardinalityCmd.RunE(governanceGherkinKeywordCardinalityCmd, []string{})
			if buf.Len() == 0 {
				t.Errorf("format %s produced no output", format)
			}
		})
	}
	output = "text"
}

// TestGovernanceGherkinCardinality_Formatters covers the PASSED and FAILED
// branches of every formatter directly.
func TestGovernanceGherkinCardinality_Formatters(t *testing.T) {
	sample := []governance.CardinalityFinding{{
		Path:     "specs/x.feature",
		Line:     12,
		Scenario: "Double when offender",
		Detail:   "2 When",
	}}

	if got := formatGherkinCardinalityText(nil); got != "GHERKIN KEYWORD CARDINALITY AUDIT PASSED: no violations found\n" {
		t.Errorf("text passed: got %q", got)
	}
	if got := formatGherkinCardinalityText(sample); !strings.Contains(got, "  specs/x.feature:12  Double when offender  →  2 When\n") {
		t.Errorf("text failed: got %q", got)
	}

	if got := formatGherkinCardinalityMarkdown(nil); got != "## Gherkin Keyword Cardinality Audit\n\n**PASSED**: no violations found\n" {
		t.Errorf("markdown passed: got %q", got)
	}
	if got := formatGherkinCardinalityMarkdown(sample); !strings.Contains(got, "| specs/x.feature | 12 | Double when offender | 2 When |") {
		t.Errorf("markdown failed: got %q", got)
	}

	passedJSON, err := formatGherkinCardinalityJSON(nil)
	if err != nil {
		t.Fatalf("json passed: %v", err)
	}
	if !strings.Contains(passedJSON, `"status": "passed"`) || !strings.Contains(passedJSON, `"count": 0`) {
		t.Errorf("json passed: got %q", passedJSON)
	}
	failedJSON, err := formatGherkinCardinalityJSON(sample)
	if err != nil {
		t.Fatalf("json failed: %v", err)
	}
	if !strings.Contains(failedJSON, `"status": "failed"`) || !strings.Contains(failedJSON, `"scenario": "Double when offender"`) {
		t.Errorf("json failed: got %q", failedJSON)
	}
}

// TestGovernanceGherkinCardinality_DefaultPathUsesRepoRoot verifies that when
// no path argument is provided, the repository root is scanned.
func TestGovernanceGherkinCardinality_DefaultPathUsesRepoRoot(t *testing.T) {
	origGetwd := osGetwd
	origStat := osStat
	origFn := governanceGherkinCardinalityFn
	defer func() {
		osGetwd = origGetwd
		osStat = origStat
		governanceGherkinCardinalityFn = origFn
	}()

	osGetwd = func() (string, error) { return "/mock-repo", nil }
	osStat = func(name string) (os.FileInfo, error) {
		if name == "/mock-repo/.git" {
			return &mockFileInfo{name: ".git", isDir: true}, nil
		}
		return nil, os.ErrNotExist
	}

	var capturedPath string
	governanceGherkinCardinalityFn = func(path string) ([]governance.CardinalityFinding, error) {
		capturedPath = path
		return nil, nil
	}

	buf := new(bytes.Buffer)
	governanceGherkinKeywordCardinalityCmd.SetOut(buf)
	governanceGherkinKeywordCardinalityCmd.SetErr(buf)
	_ = governanceGherkinKeywordCardinalityCmd.RunE(governanceGherkinKeywordCardinalityCmd, []string{})

	expected := "/mock-repo"
	if capturedPath != expected {
		t.Errorf("expected path %q, got %q", expected, capturedPath)
	}
}
