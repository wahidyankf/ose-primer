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
	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/docs"
)

var specsDirUnitHeadingHierarchy = func() string {
	_, f, _, _ := runtime.Caller(0)
	return filepath.Join(filepath.Dir(f), "../../../specs/apps/rhino/behavior/cli/gherkin")
}()

type validateHeadingUnitSteps struct {
	cmdErr    error
	cmdOutput string
	// headingFile is the repo-relative path of the fixture file, asserted by
	// the heading-finding Then steps.
	headingFile string
	// mockFindings is what the mocked engine returns BEFORE applying the
	// scenario's --exclude prefixes (denied-tree fixtures contribute nothing,
	// mirroring the real allowlist).
	mockFindings []docs.HeadingFinding
}

func (s *validateHeadingUnitSteps) before(_ context.Context, _ *godog.Scenario) (context.Context, error) {
	verbose = false
	quiet = false
	output = "text"
	validateHeadingHierarchyStagedOnly = false
	validateHeadingHierarchyExclude = nil
	s.cmdErr = nil
	s.cmdOutput = ""
	s.headingFile = ""
	s.mockFindings = nil

	// Mock findGitRoot
	osGetwd = func() (string, error) { return "/mock-repo", nil }
	osStat = func(name string) (os.FileInfo, error) {
		if name == "/mock-repo/.git" {
			return &mockFileInfo{name: ".git", isDir: true}, nil
		}
		return nil, os.ErrNotExist
	}

	// Mock engine: return the fixture findings minus --exclude prefixes.
	docsValidateHeadingHierarchyFn = func(opts docs.HeadingScanOptions) ([]docs.HeadingFinding, error) {
		var out []docs.HeadingFinding
		for _, f := range s.mockFindings {
			excluded := false
			for _, prefix := range opts.Exclude {
				if strings.HasPrefix(f.File, prefix) {
					excluded = true
					break
				}
			}
			if !excluded {
				out = append(out, f)
			}
		}
		return out, nil
	}

	return context.Background(), nil
}

func (s *validateHeadingUnitSteps) after(_ context.Context, _ *godog.Scenario, _ error) (context.Context, error) {
	docsValidateHeadingHierarchyFn = docs.ValidateHeadingHierarchy
	osGetwd = os.Getwd
	osStat = os.Stat
	return context.Background(), nil
}

// --- Given steps ---

func (s *validateHeadingUnitSteps) aMarkdownFileUnderDocsWithTwoH1Headings() error {
	s.headingFile = "docs/guide.md"
	s.mockFindings = append(s.mockFindings, docs.HeadingFinding{
		File: "docs/guide.md", Line: 5, Kind: docs.HeadingKindDuplicateH1,
		Message: `duplicate H1 "Second Title" (first H1 at line 1)`,
	})
	return nil
}

func (s *validateHeadingUnitSteps) aMarkdownFileUnderDocsWithNoH1Heading() error {
	s.headingFile = "docs/notes.md"
	s.mockFindings = append(s.mockFindings, docs.HeadingFinding{
		File: "docs/notes.md", Line: 1, Kind: docs.HeadingKindMissingH1,
		Message: "file has no H1 heading",
	})
	return nil
}

func (s *validateHeadingUnitSteps) aMarkdownFileUnderDocsThatJumpsFromH1DirectlyToH3() error {
	s.headingFile = "docs/jump.md"
	s.mockFindings = append(s.mockFindings, docs.HeadingFinding{
		File: "docs/jump.md", Line: 3, Kind: docs.HeadingKindSkippedLevel,
		Message: `heading level jumps from H1 to H3 at "Jumped Here"`,
	})
	return nil
}

// Denied-tree fixtures contribute NO mock findings: the real engine's
// allowlist default-denies these paths.

func (s *validateHeadingUnitSteps) aMarkdownFileUnderClaudeAgentsWithZeroH1Headings() error {
	s.headingFile = ".claude/agents/swe-rust-dev.md"
	return nil
}

func (s *validateHeadingUnitSteps) aSkillFileUnderClaudeSkillsWithMultipleH1Headings() error {
	s.headingFile = ".claude/skills/example/SKILL.md"
	return nil
}

func (s *validateHeadingUnitSteps) aMarkdownFileUnderPlansDoneWithASkippedHeadingLevel() error {
	s.headingFile = "plans/done/2026-01-01__archived/delivery.md"
	return nil
}

func (s *validateHeadingUnitSteps) anAppsExampleReadmeFileWithASkippedHeadingLevel() error {
	s.headingFile = "apps/example/README.md"
	s.mockFindings = append(s.mockFindings, docs.HeadingFinding{
		File: "apps/example/README.md", Line: 3, Kind: docs.HeadingKindSkippedLevel,
		Message: `heading level jumps from H1 to H3 at "Skipped In Readme"`,
	})
	return nil
}

func (s *validateHeadingUnitSteps) aMarkdownFileAtAppsExampleSrcNotesWithZeroH1Headings() error {
	s.headingFile = "apps/example/src/notes.md"
	return nil
}

func (s *validateHeadingUnitSteps) aMarkdownFileUnderDocsWithADuplicateH1() error {
	s.mockFindings = append(s.mockFindings, docs.HeadingFinding{
		File: "docs/excluded.md", Line: 3, Kind: docs.HeadingKindDuplicateH1,
		Message: `duplicate H1 "Doc Again" (first H1 at line 1)`,
	})
	return nil
}

func (s *validateHeadingUnitSteps) aMarkdownFileUnderRepoGovernanceWithADuplicateH1() error {
	s.mockFindings = append(s.mockFindings, docs.HeadingFinding{
		File: "repo-governance/rule.md", Line: 3, Kind: docs.HeadingKindDuplicateH1,
		Message: `duplicate H1 "Rule Again" (first H1 at line 1)`,
	})
	return nil
}

// --- When steps ---

func (s *validateHeadingUnitSteps) run() error {
	buf := new(bytes.Buffer)
	validateHeadingHierarchyCmd.SetOut(buf)
	validateHeadingHierarchyCmd.SetErr(buf)
	s.cmdErr = validateHeadingHierarchyCmd.RunE(validateHeadingHierarchyCmd, []string{})
	s.cmdOutput = buf.String()
	return nil
}

func (s *validateHeadingUnitSteps) theDeveloperRunsValidateHeadingHierarchy() error {
	validateHeadingHierarchyStagedOnly = false
	validateHeadingHierarchyExclude = nil
	return s.run()
}

func (s *validateHeadingUnitSteps) theDeveloperRunsValidateHeadingHierarchyWithExcludeDocs() error {
	validateHeadingHierarchyStagedOnly = false
	validateHeadingHierarchyExclude = []string{"docs"}
	return s.run()
}

// --- Then steps ---

func (s *validateHeadingUnitSteps) theCommandExitsSuccessfully() error {
	if s.cmdErr != nil {
		return fmt.Errorf("expected success but got: %w\nOutput: %s", s.cmdErr, s.cmdOutput)
	}
	return nil
}

func (s *validateHeadingUnitSteps) theCommandExitsWithAFailureCode() error {
	if s.cmdErr == nil {
		return fmt.Errorf("expected failure but succeeded\nOutput: %s", s.cmdOutput)
	}
	return nil
}

func (s *validateHeadingUnitSteps) assertHeadingFinding(kind string) error {
	if s.headingFile == "" {
		return fmt.Errorf("fixture did not set headingFile")
	}
	if !strings.Contains(s.cmdOutput, "Heading Hierarchy Report") {
		return fmt.Errorf("expected Heading Hierarchy Report in output, got: %s", s.cmdOutput)
	}
	if !strings.Contains(s.cmdOutput, kind) {
		return fmt.Errorf("expected %s finding in output, got: %s", kind, s.cmdOutput)
	}
	if !strings.Contains(s.cmdOutput, s.headingFile) {
		return fmt.Errorf("expected output to identify %s, got: %s", s.headingFile, s.cmdOutput)
	}
	return nil
}

func (s *validateHeadingUnitSteps) assertNoHeadingFinding(kind string) error {
	if s.headingFile == "" {
		return fmt.Errorf("fixture did not set headingFile")
	}
	if strings.Contains(s.cmdOutput, kind) {
		return fmt.Errorf("expected no %s finding in output, got: %s", kind, s.cmdOutput)
	}
	if strings.Contains(s.cmdOutput, s.headingFile) {
		return fmt.Errorf("expected output not to mention %s, got: %s", s.headingFile, s.cmdOutput)
	}
	return nil
}

func (s *validateHeadingUnitSteps) theOutputReportsADuplicateH1FindingForThatFile() error {
	return s.assertHeadingFinding("duplicate-h1")
}

func (s *validateHeadingUnitSteps) theOutputReportsAMissingH1FindingForThatFile() error {
	return s.assertHeadingFinding("missing-h1")
}

func (s *validateHeadingUnitSteps) theOutputReportsASkippedLevelFindingForThatFile() error {
	return s.assertHeadingFinding("skipped-level")
}

func (s *validateHeadingUnitSteps) noMissingH1FindingIsReportedForThatFile() error {
	return s.assertNoHeadingFinding("missing-h1")
}

func (s *validateHeadingUnitSteps) noDuplicateH1FindingIsReportedForThatFile() error {
	return s.assertNoHeadingFinding("duplicate-h1")
}

func (s *validateHeadingUnitSteps) noSkippedLevelFindingIsReportedForThatFile() error {
	return s.assertNoHeadingFinding("skipped-level")
}

func (s *validateHeadingUnitSteps) noFindingIsReportedForTheDocsFile() error {
	if strings.Contains(s.cmdOutput, "docs/excluded.md") {
		return fmt.Errorf("expected no finding for docs/excluded.md, got: %s", s.cmdOutput)
	}
	return nil
}

func (s *validateHeadingUnitSteps) theOutputReportsADuplicateH1FindingForTheRepoGovernanceFile() error {
	if !strings.Contains(s.cmdOutput, "duplicate-h1") {
		return fmt.Errorf("expected duplicate-h1 finding in output, got: %s", s.cmdOutput)
	}
	if !strings.Contains(s.cmdOutput, "repo-governance/rule.md") {
		return fmt.Errorf("expected output to identify repo-governance/rule.md, got: %s", s.cmdOutput)
	}
	return nil
}

func TestUnitDocsValidateHeadingHierarchy(t *testing.T) {
	s := &validateHeadingUnitSteps{}
	suite := godog.TestSuite{
		ScenarioInitializer: func(sc *godog.ScenarioContext) {
			sc.Before(s.before)
			sc.After(s.after)
			sc.Step(stepHeadingDocsFileWithTwoH1s, s.aMarkdownFileUnderDocsWithTwoH1Headings)
			sc.Step(stepHeadingDocsFileWithNoH1, s.aMarkdownFileUnderDocsWithNoH1Heading)
			sc.Step(stepHeadingDocsFileJumpsH1ToH3, s.aMarkdownFileUnderDocsThatJumpsFromH1DirectlyToH3)
			sc.Step(stepHeadingClaudeAgentsFileWithZeroH1s, s.aMarkdownFileUnderClaudeAgentsWithZeroH1Headings)
			sc.Step(stepHeadingSkillFileWithMultipleH1s, s.aSkillFileUnderClaudeSkillsWithMultipleH1Headings)
			sc.Step(stepHeadingPlansDoneFileWithSkippedLevel, s.aMarkdownFileUnderPlansDoneWithASkippedHeadingLevel)
			sc.Step(stepHeadingAppsReadmeWithSkippedLevel, s.anAppsExampleReadmeFileWithASkippedHeadingLevel)
			sc.Step(stepHeadingAppsInternalFileWithZeroH1s, s.aMarkdownFileAtAppsExampleSrcNotesWithZeroH1Headings)
			sc.Step(stepHeadingDocsFileWithDuplicateH1, s.aMarkdownFileUnderDocsWithADuplicateH1)
			sc.Step(stepHeadingGovernanceFileWithDuplicateH1, s.aMarkdownFileUnderRepoGovernanceWithADuplicateH1)
			sc.Step(stepDeveloperRunsValidateHeading, s.theDeveloperRunsValidateHeadingHierarchy)
			sc.Step(stepDeveloperRunsValidateHeadingExclude, s.theDeveloperRunsValidateHeadingHierarchyWithExcludeDocs)
			sc.Step(stepExitsSuccessfully, s.theCommandExitsSuccessfully)
			sc.Step(stepExitsWithFailure, s.theCommandExitsWithAFailureCode)
			sc.Step(stepOutputReportsDuplicateH1Finding, s.theOutputReportsADuplicateH1FindingForThatFile)
			sc.Step(stepOutputReportsMissingH1Finding, s.theOutputReportsAMissingH1FindingForThatFile)
			sc.Step(stepOutputReportsSkippedLevelFinding, s.theOutputReportsASkippedLevelFindingForThatFile)
			sc.Step(stepNoMissingH1FindingReported, s.noMissingH1FindingIsReportedForThatFile)
			sc.Step(stepNoDuplicateH1FindingReported, s.noDuplicateH1FindingIsReportedForThatFile)
			sc.Step(stepNoSkippedLevelFindingReported, s.noSkippedLevelFindingIsReportedForThatFile)
			sc.Step(stepNoFindingReportedForDocsFile, s.noFindingIsReportedForTheDocsFile)
			sc.Step(stepOutputReportsGovernanceDuplicateH1, s.theOutputReportsADuplicateH1FindingForTheRepoGovernanceFile)
		},
		Options: &godog.Options{
			Format:   "pretty",
			Paths:    []string{specsDirUnitHeadingHierarchy},
			TestingT: t,
			Tags:     "@docs-validate-heading-hierarchy",
		},
	}
	if suite.Run() != 0 {
		t.Fatal("non-zero status returned, failed to run unit feature tests")
	}
}

// mockHeadingGitRoot installs the /mock-repo git-root mocks and returns a
// restore function.
func mockHeadingGitRoot() func() {
	origGetwd := osGetwd
	origStat := osStat
	origFn := docsValidateHeadingHierarchyFn
	origStaged := getMermaidStagedFilesFn
	osGetwd = func() (string, error) { return "/mock-repo", nil }
	osStat = func(name string) (os.FileInfo, error) {
		if name == "/mock-repo/.git" {
			return &mockFileInfo{name: ".git", isDir: true}, nil
		}
		return nil, os.ErrNotExist
	}
	return func() {
		osGetwd = origGetwd
		osStat = origStat
		docsValidateHeadingHierarchyFn = origFn
		getMermaidStagedFilesFn = origStaged
	}
}

// resetHeadingFlags resets the command's globals to defaults.
func resetHeadingFlags() {
	validateHeadingHierarchyStagedOnly = false
	validateHeadingHierarchyExclude = nil
	output = "text"
	verbose = false
	quiet = false
}

// runHeadingCmd runs the command with args and returns (combined output, err).
func runHeadingCmd(args []string) (string, error) {
	buf := new(bytes.Buffer)
	validateHeadingHierarchyCmd.SetOut(buf)
	validateHeadingHierarchyCmd.SetErr(buf)
	err := validateHeadingHierarchyCmd.RunE(validateHeadingHierarchyCmd, args)
	return buf.String(), err
}

// TestValidateHeadingHierarchyCommand_MissingGitRoot verifies git root detection — not in Gherkin specs.
func TestValidateHeadingHierarchyCommand_MissingGitRoot(t *testing.T) {
	restore := mockHeadingGitRoot()
	defer restore()
	osGetwd = func() (string, error) { return "/no-git-here", nil }
	osStat = func(_ string) (os.FileInfo, error) { return nil, os.ErrNotExist }
	resetHeadingFlags()

	_, err := runHeadingCmd([]string{})
	if err == nil {
		t.Error("expected error when no .git directory found")
	}
	if !strings.Contains(err.Error(), "git") {
		t.Errorf("expected error mentioning 'git', got: %v", err)
	}
}

// TestValidateHeadingHierarchyCommand_JSONOutput verifies JSON success output — not in Gherkin specs.
func TestValidateHeadingHierarchyCommand_JSONOutput(t *testing.T) {
	restore := mockHeadingGitRoot()
	defer restore()
	docsValidateHeadingHierarchyFn = func(_ docs.HeadingScanOptions) ([]docs.HeadingFinding, error) {
		return nil, nil
	}
	resetHeadingFlags()
	output = "json"

	out, err := runHeadingCmd([]string{})
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if !strings.Contains(out, `"status": "success"`) {
		t.Errorf("expected JSON output with success status, got: %s", out)
	}
	if !strings.Contains(out, `"total_findings": 0`) {
		t.Errorf("expected JSON output with total_findings, got: %s", out)
	}
	if !strings.Contains(out, `"findings": []`) {
		t.Errorf("expected JSON output with empty findings array (not null), got: %s", out)
	}
}

// TestValidateHeadingHierarchyCommand_FindingsJSON verifies error returned for findings in JSON mode.
func TestValidateHeadingHierarchyCommand_FindingsJSON(t *testing.T) {
	restore := mockHeadingGitRoot()
	defer restore()
	docsValidateHeadingHierarchyFn = func(_ docs.HeadingScanOptions) ([]docs.HeadingFinding, error) {
		return []docs.HeadingFinding{
			{File: "docs/a.md", Line: 1, Kind: docs.HeadingKindMissingH1, Message: "file has no H1 heading"},
		}, nil
	}
	resetHeadingFlags()
	output = "json"

	out, err := runHeadingCmd([]string{})
	if err == nil {
		t.Error("expected error for findings in JSON output")
	}
	if !strings.Contains(err.Error(), "heading hierarchy finding") {
		t.Errorf("expected error mentioning findings, got: %v", err)
	}
	if !strings.Contains(out, `"status": "failure"`) {
		t.Errorf("expected JSON failure status, got: %s", out)
	}
	if strings.Contains(out, "❌") {
		t.Errorf("expected no text footer in JSON mode, got: %s", out)
	}
}

// TestValidateHeadingHierarchyCommand_TextFooter verifies the stderr footer in non-quiet text mode.
func TestValidateHeadingHierarchyCommand_TextFooter(t *testing.T) {
	restore := mockHeadingGitRoot()
	defer restore()
	docsValidateHeadingHierarchyFn = func(_ docs.HeadingScanOptions) ([]docs.HeadingFinding, error) {
		return []docs.HeadingFinding{
			{File: "docs/a.md", Line: 1, Kind: docs.HeadingKindMissingH1, Message: "file has no H1 heading"},
		}, nil
	}
	resetHeadingFlags()

	out, err := runHeadingCmd([]string{})
	if err == nil {
		t.Error("expected error for findings")
	}
	if !strings.Contains(out, "❌ Found 1 heading hierarchy finding(s)") {
		t.Errorf("expected footer in text mode, got: %s", out)
	}
}

// TestValidateHeadingHierarchyCommand_QuietSuppressesFooter verifies quiet mode hides the footer.
func TestValidateHeadingHierarchyCommand_QuietSuppressesFooter(t *testing.T) {
	restore := mockHeadingGitRoot()
	defer restore()
	docsValidateHeadingHierarchyFn = func(_ docs.HeadingScanOptions) ([]docs.HeadingFinding, error) {
		return []docs.HeadingFinding{
			{File: "docs/a.md", Line: 1, Kind: docs.HeadingKindMissingH1, Message: "file has no H1 heading"},
		}, nil
	}
	resetHeadingFlags()
	quiet = true

	out, err := runHeadingCmd([]string{})
	if err == nil {
		t.Error("expected error for findings")
	}
	if strings.Contains(out, "❌") {
		t.Errorf("expected no footer in quiet mode, got: %s", out)
	}
}

// TestValidateHeadingHierarchyCommand_MarkdownOutput verifies markdown output delegates to text.
func TestValidateHeadingHierarchyCommand_MarkdownOutput(t *testing.T) {
	restore := mockHeadingGitRoot()
	defer restore()
	docsValidateHeadingHierarchyFn = func(_ docs.HeadingScanOptions) ([]docs.HeadingFinding, error) {
		return nil, nil
	}
	resetHeadingFlags()
	output = "markdown"

	out, err := runHeadingCmd([]string{})
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if !strings.Contains(out, "All heading hierarchies valid") {
		t.Errorf("expected markdown success message, got: %s", out)
	}
}

// TestValidateHeadingHierarchyCommand_ValidationError verifies engine errors are wrapped.
func TestValidateHeadingHierarchyCommand_ValidationError(t *testing.T) {
	restore := mockHeadingGitRoot()
	defer restore()
	docsValidateHeadingHierarchyFn = func(_ docs.HeadingScanOptions) ([]docs.HeadingFinding, error) {
		return nil, fmt.Errorf("boom")
	}
	resetHeadingFlags()

	_, err := runHeadingCmd([]string{})
	if err == nil || !strings.Contains(err.Error(), "validation failed") {
		t.Errorf("expected wrapped validation error, got: %v", err)
	}
}

// TestValidateHeadingHierarchyCommand_StagedOnlyEmptyShortCircuits verifies an
// empty staged set reports success WITHOUT falling back to a full scan.
func TestValidateHeadingHierarchyCommand_StagedOnlyEmptyShortCircuits(t *testing.T) {
	restore := mockHeadingGitRoot()
	defer restore()
	getMermaidStagedFilesFn = func(_ string) ([]string, error) { return nil, nil }
	engineCalled := false
	docsValidateHeadingHierarchyFn = func(_ docs.HeadingScanOptions) ([]docs.HeadingFinding, error) {
		engineCalled = true
		return nil, nil
	}
	resetHeadingFlags()
	validateHeadingHierarchyStagedOnly = true

	out, err := runHeadingCmd([]string{})
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if engineCalled {
		t.Error("expected NO full scan when staged set is empty")
	}
	if !strings.Contains(out, "All heading hierarchies valid") {
		t.Errorf("expected success message, got: %s", out)
	}
}

// TestValidateHeadingHierarchyCommand_StagedOnlyPassesRelPaths verifies staged
// absolute paths are converted to repo-relative engine paths.
func TestValidateHeadingHierarchyCommand_StagedOnlyPassesRelPaths(t *testing.T) {
	restore := mockHeadingGitRoot()
	defer restore()
	getMermaidStagedFilesFn = func(_ string) ([]string, error) {
		return []string{"/mock-repo/docs/a.md", "/elsewhere/x.md"}, nil
	}
	var gotPaths []string
	docsValidateHeadingHierarchyFn = func(opts docs.HeadingScanOptions) ([]docs.HeadingFinding, error) {
		gotPaths = opts.Paths
		return nil, nil
	}
	resetHeadingFlags()
	validateHeadingHierarchyStagedOnly = true

	_, err := runHeadingCmd([]string{})
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if len(gotPaths) != 1 || gotPaths[0] != "docs/a.md" {
		t.Errorf("expected engine paths [docs/a.md], got: %v", gotPaths)
	}
}

// TestValidateHeadingHierarchyCommand_StagedFilesError verifies git failures are wrapped.
func TestValidateHeadingHierarchyCommand_StagedFilesError(t *testing.T) {
	restore := mockHeadingGitRoot()
	defer restore()
	getMermaidStagedFilesFn = func(_ string) ([]string, error) {
		return nil, fmt.Errorf("git unavailable")
	}
	resetHeadingFlags()
	validateHeadingHierarchyStagedOnly = true

	_, err := runHeadingCmd([]string{})
	if err == nil || !strings.Contains(err.Error(), "failed to get staged files") {
		t.Errorf("expected staged-files error, got: %v", err)
	}
}
