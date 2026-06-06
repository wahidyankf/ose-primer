//go:build integration

package cmd

import (
	"bytes"
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/cucumber/godog"
)

// Scenario: A docs file with two H1s is flagged duplicate-h1
// Given a markdown file under docs with two H1 headings
// When the developer runs docs validate-heading-hierarchy
// Then the command exits with a failure code
// And the output reports a duplicate-h1 finding for that file

// Scenario: A docs file with zero H1s is flagged missing-h1
// Given a markdown file under docs with no H1 heading
// When the developer runs docs validate-heading-hierarchy
// Then the command exits with a failure code
// And the output reports a missing-h1 finding for that file

// Scenario: A docs file jumping from H1 to H3 is flagged skipped-level
// Given a markdown file under docs that jumps from H1 directly to H3
// When the developer runs docs validate-heading-hierarchy
// Then the command exits with a failure code
// And the output reports a skipped-level finding for that file

// Scenario: A .claude/agents file with heading violations is exempt
// Given a markdown file under .claude/agents with zero H1 headings
// When the developer runs docs validate-heading-hierarchy
// Then the command exits successfully
// And no missing-h1 finding is reported for that file

// Scenario: A SKILL.md under .claude/skills with many H1s is exempt
// Given a SKILL.md file under .claude/skills with multiple H1 headings
// When the developer runs docs validate-heading-hierarchy
// Then the command exits successfully
// And no duplicate-h1 finding is reported for that file

// Scenario: A file under plans/done with violations is excluded
// Given a markdown file under plans/done with a skipped heading level
// When the developer runs docs validate-heading-hierarchy
// Then the command exits successfully
// And no skipped-level finding is reported for that file

// Scenario: An app README with a violation is flagged
// Given an apps/example/README.md file with a skipped heading level
// When the developer runs docs validate-heading-hierarchy
// Then the command exits with a failure code
// And the output reports a skipped-level finding for that file

// Scenario: A deep app internal path with violations is excluded
// Given a markdown file at apps/example/src/notes.md with zero H1 headings
// When the developer runs docs validate-heading-hierarchy
// Then the command exits successfully
// And no missing-h1 finding is reported for that file

// Scenario: With --exclude docs the docs tree findings are suppressed
// Given a markdown file under docs with a duplicate H1
// And a markdown file under repo-governance with a duplicate H1
// When the developer runs docs validate-heading-hierarchy with --exclude docs
// Then no finding is reported for the docs file
// And the output reports a duplicate-h1 finding for the repo-governance file

type validateHeadingHierarchySteps struct {
	originalWd string
	tmpDir     string
	cmdErr     error
	cmdOutput  string
	// headingFile is the repo-relative path of the heading-hierarchy fixture
	// file, asserted by the heading-finding Then steps.
	headingFile string
}

func (s *validateHeadingHierarchySteps) before(_ context.Context, _ *godog.Scenario) (context.Context, error) {
	s.originalWd, _ = os.Getwd()
	s.tmpDir, _ = os.MkdirTemp("", "validate-heading-hierarchy-*")
	_ = os.MkdirAll(filepath.Join(s.tmpDir, ".git"), 0755)
	verbose = false
	quiet = false
	output = "text"
	validateHeadingHierarchyStagedOnly = false
	validateHeadingHierarchyExclude = nil
	s.cmdErr = nil
	s.cmdOutput = ""
	s.headingFile = ""
	_ = os.Chdir(s.tmpDir)
	return context.Background(), nil
}

func (s *validateHeadingHierarchySteps) after(_ context.Context, _ *godog.Scenario, _ error) (context.Context, error) {
	_ = os.Chdir(s.originalWd)
	_ = os.RemoveAll(s.tmpDir)
	return context.Background(), nil
}

// write writes content to tmpDir/rel, creating parent directories.
func (s *validateHeadingHierarchySteps) write(rel, content string) error {
	path := filepath.Join(s.tmpDir, rel)
	if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
		return err
	}
	return os.WriteFile(path, []byte(content), 0644)
}

// --- Given steps ---

func (s *validateHeadingHierarchySteps) aMarkdownFileUnderDocsWithTwoH1Headings() error {
	s.headingFile = "docs/guide.md"
	return s.write("docs/guide.md", "# First Title\n\ntext\n\n# Second Title\n")
}

func (s *validateHeadingHierarchySteps) aMarkdownFileUnderDocsWithNoH1Heading() error {
	s.headingFile = "docs/notes.md"
	return s.write("docs/notes.md", "## Only A Section\n\ntext\n")
}

func (s *validateHeadingHierarchySteps) aMarkdownFileUnderDocsThatJumpsFromH1DirectlyToH3() error {
	s.headingFile = "docs/jump.md"
	return s.write("docs/jump.md", "# Title\n\n### Jumped Here\n")
}

func (s *validateHeadingHierarchySteps) aMarkdownFileUnderClaudeAgentsWithZeroH1Headings() error {
	s.headingFile = ".claude/agents/swe-rust-dev.md"
	return s.write(".claude/agents/swe-rust-dev.md", "## No H1 In Agent Files\n\nbody\n")
}

func (s *validateHeadingHierarchySteps) aSkillFileUnderClaudeSkillsWithMultipleH1Headings() error {
	s.headingFile = ".claude/skills/example/SKILL.md"
	return s.write(".claude/skills/example/SKILL.md", "# One\n\n# Two\n\n# Three\n")
}

func (s *validateHeadingHierarchySteps) aMarkdownFileUnderPlansDoneWithASkippedHeadingLevel() error {
	s.headingFile = "plans/done/2026-01-01__archived/delivery.md"
	return s.write("plans/done/2026-01-01__archived/delivery.md", "# Title\n\n### Skipped In Archive\n")
}

func (s *validateHeadingHierarchySteps) anAppsExampleReadmeFileWithASkippedHeadingLevel() error {
	s.headingFile = "apps/example/README.md"
	return s.write("apps/example/README.md", "# Example\n\n### Skipped In Readme\n")
}

func (s *validateHeadingHierarchySteps) aMarkdownFileAtAppsExampleSrcNotesWithZeroH1Headings() error {
	s.headingFile = "apps/example/src/notes.md"
	return s.write("apps/example/src/notes.md", "## Zero H1s Here But Default-Denied\n")
}

func (s *validateHeadingHierarchySteps) aMarkdownFileUnderDocsWithADuplicateH1() error {
	return s.write("docs/excluded.md", "# Doc\n\n# Doc Again\n")
}

func (s *validateHeadingHierarchySteps) aMarkdownFileUnderRepoGovernanceWithADuplicateH1() error {
	return s.write("repo-governance/rule.md", "# Rule\n\n# Rule Again\n")
}

// --- When steps ---

func (s *validateHeadingHierarchySteps) run(args []string) error {
	buf := new(bytes.Buffer)
	validateHeadingHierarchyCmd.SetOut(buf)
	validateHeadingHierarchyCmd.SetErr(buf)
	s.cmdErr = validateHeadingHierarchyCmd.RunE(validateHeadingHierarchyCmd, args)
	s.cmdOutput = buf.String()
	return nil
}

func (s *validateHeadingHierarchySteps) theDeveloperRunsValidateHeadingHierarchy() error {
	validateHeadingHierarchyStagedOnly = false
	validateHeadingHierarchyExclude = nil
	return s.run([]string{})
}

func (s *validateHeadingHierarchySteps) theDeveloperRunsValidateHeadingHierarchyWithExcludeDocs() error {
	validateHeadingHierarchyStagedOnly = false
	validateHeadingHierarchyExclude = []string{"docs"}
	return s.run([]string{})
}

// --- Then steps ---

func (s *validateHeadingHierarchySteps) theCommandExitsSuccessfully() error {
	if s.cmdErr != nil {
		return fmt.Errorf("expected command to exit successfully, got error: %w (output: %s)", s.cmdErr, s.cmdOutput)
	}
	return nil
}

func (s *validateHeadingHierarchySteps) theCommandExitsWithAFailureCode() error {
	if s.cmdErr == nil {
		return fmt.Errorf("expected command to exit with failure, but it succeeded (output: %s)", s.cmdOutput)
	}
	return nil
}

// assertHeadingFinding asserts the report contains a finding of kind for the
// fixture file.
func (s *validateHeadingHierarchySteps) assertHeadingFinding(kind string) error {
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

// assertNoHeadingFinding asserts the report contains NO finding of kind for
// the fixture file.
func (s *validateHeadingHierarchySteps) assertNoHeadingFinding(kind string) error {
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

func (s *validateHeadingHierarchySteps) theOutputReportsADuplicateH1FindingForThatFile() error {
	return s.assertHeadingFinding("duplicate-h1")
}

func (s *validateHeadingHierarchySteps) theOutputReportsAMissingH1FindingForThatFile() error {
	return s.assertHeadingFinding("missing-h1")
}

func (s *validateHeadingHierarchySteps) theOutputReportsASkippedLevelFindingForThatFile() error {
	return s.assertHeadingFinding("skipped-level")
}

func (s *validateHeadingHierarchySteps) noMissingH1FindingIsReportedForThatFile() error {
	return s.assertNoHeadingFinding("missing-h1")
}

func (s *validateHeadingHierarchySteps) noDuplicateH1FindingIsReportedForThatFile() error {
	return s.assertNoHeadingFinding("duplicate-h1")
}

func (s *validateHeadingHierarchySteps) noSkippedLevelFindingIsReportedForThatFile() error {
	return s.assertNoHeadingFinding("skipped-level")
}

func (s *validateHeadingHierarchySteps) noFindingIsReportedForTheDocsFile() error {
	if strings.Contains(s.cmdOutput, "docs/excluded.md") {
		return fmt.Errorf("expected no finding for docs/excluded.md, got: %s", s.cmdOutput)
	}
	return nil
}

func (s *validateHeadingHierarchySteps) theOutputReportsADuplicateH1FindingForTheRepoGovernanceFile() error {
	if !strings.Contains(s.cmdOutput, "duplicate-h1") {
		return fmt.Errorf("expected duplicate-h1 finding in output, got: %s", s.cmdOutput)
	}
	if !strings.Contains(s.cmdOutput, "repo-governance/rule.md") {
		return fmt.Errorf("expected output to identify repo-governance/rule.md, got: %s", s.cmdOutput)
	}
	return nil
}

func InitializeValidateHeadingHierarchyScenario(sc *godog.ScenarioContext) {
	s := &validateHeadingHierarchySteps{}
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
}

func TestIntegrationValidateHeadingHierarchy(t *testing.T) {
	suite := godog.TestSuite{
		ScenarioInitializer: InitializeValidateHeadingHierarchyScenario,
		Options: &godog.Options{
			Format:   "pretty",
			Paths:    []string{specsDocsDir},
			Tags:     "docs-validate-heading-hierarchy",
			TestingT: t,
		},
	}
	if suite.Run() != 0 {
		t.Fatal("non-zero status returned, failed to run feature tests")
	}
}
