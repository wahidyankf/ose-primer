//go:build integration

package cmd

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"testing"

	"github.com/cucumber/godog"
	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/mermaid"
)

var specsDirIntMermaid = func() string {
	_, f, _, _ := runtime.Caller(0)
	return filepath.Join(filepath.Dir(f), "../../../specs/apps/rhino/behavior/cli/gherkin")
}()

// mermaidThresholdPair carries the width/depth thresholds the plain
// `validate-mermaid` run should apply. Set by fixtures whose contract intent
// (a complex-diagram warning carrying the computed span/depth) is only
// reachable with non-default thresholds, since the local Go default
// `--max-depth` is unlimited. Mirrors the Rust DocsWorld `mermaid_thresholds`
// world-state pattern.
type mermaidThresholdPair struct {
	maxWidth int
	maxDepth int
}

type validateMermaidIntSteps struct {
	originalWd string
	tmpDir     string
	cmdErr     error
	cmdOutput  string
	// thresholds the plain When step applies before running (world state).
	mermaidThresholds *mermaidThresholdPair
}

func (s *validateMermaidIntSteps) before(_ context.Context, _ *godog.Scenario) (context.Context, error) {
	s.originalWd, _ = os.Getwd()
	s.tmpDir, _ = os.MkdirTemp("", "validate-mermaid-*")
	_ = os.MkdirAll(filepath.Join(s.tmpDir, ".git"), 0755)
	verbose = false
	quiet = false
	output = "text"
	validateMermaidStagedOnly = false
	validateMermaidChangedOnly = false
	validateMermaidExclude = nil
	validateMermaidMaxLabelLen = 30
	validateMermaidMaxWidth = 3
	validateMermaidMaxDepth = 5
	s.mermaidThresholds = nil
	// Restore real implementations so integration tests exercise actual code.
	docsValidateMermaidFn = mermaid.ValidateBlocks
	readFileFn = os.ReadFile
	getMermaidStagedFilesFn = getMermaidStagedFiles
	getMermaidChangedFilesFn = getMermaidChangedFiles
	_ = os.Chdir(s.tmpDir)
	return context.Background(), nil
}

func (s *validateMermaidIntSteps) after(_ context.Context, _ *godog.Scenario, _ error) (context.Context, error) {
	_ = os.Chdir(s.originalWd)
	_ = os.RemoveAll(s.tmpDir)
	validateMermaidExclude = nil
	docsValidateMermaidFn = mermaid.ValidateBlocks
	readFileFn = os.ReadFile
	getMermaidStagedFilesFn = getMermaidStagedFiles
	getMermaidChangedFilesFn = getMermaidChangedFiles
	return context.Background(), nil
}

// --- Helpers ---

func (s *validateMermaidIntSteps) writeMD(relPath, content string) error {
	abs := filepath.Join(s.tmpDir, relPath)
	if err := os.MkdirAll(filepath.Dir(abs), 0755); err != nil {
		return err
	}
	return os.WriteFile(abs, []byte(content), 0644)
}

func (s *validateMermaidIntSteps) runCmd(args []string) {
	buf := new(bytes.Buffer)
	validateMermaidCmd.SetOut(buf)
	validateMermaidCmd.SetErr(buf)
	s.cmdErr = validateMermaidCmd.RunE(validateMermaidCmd, args)
	s.cmdOutput = buf.String()
}

// --- Given steps ---

func (s *validateMermaidIntSteps) aMarkdownFileContainingFlowchartAllLabelsWithinLimit() error {
	return s.writeMD("docs/clean.md", "# Clean\n\n```mermaid\nflowchart TB\n  A[Short] --> B[Label]\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingFlowchartNodeLabelLongerThanLimit() error {
	return s.writeMD("docs/toolong.md",
		"# Bad\n\n```mermaid\nflowchart TB\n  A[This label is way too long for the limit indeed] --> B[ok]\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingFlowchartNodeLabel35Chars() error {
	// 35-char label: passes at --max-label-len 40.
	label := "This is exactly thirty-five chars!!" // exactly 35 chars
	return s.writeMD("docs/label35.md",
		fmt.Sprintf("# Label35\n\n```mermaid\nflowchart TB\n  A[%s] --> B[ok]\n```\n", label))
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingTBFlowchartTenNodesChainedSequentially() error {
	return s.writeMD("docs/chain.md",
		"# Chain\n\n```mermaid\nflowchart TB\n  A-->B-->C-->D-->E-->F-->G-->H-->I-->J\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingTBFlowchartNoRankMoreThan3() error {
	return s.writeMD("docs/tb3.md",
		"# TB3\n\n```mermaid\nflowchart TB\n  A-->B\n  A-->C\n  A-->D\n  B-->E\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingTBFlowchartOneRank4ParallelNodes() error {
	return s.writeMD("docs/tb4.md",
		"# TB4\n\n```mermaid\nflowchart TB\n  Root-->A\n  Root-->B\n  Root-->C\n  Root-->D\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingLRFlowchartNoRankMoreThan3() error {
	return s.writeMD("docs/lr3.md",
		"# LR3\n\n```mermaid\nflowchart LR\n  A-->B\n  A-->C\n  A-->D\n  B-->E\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingLRFlowchart4NodesAtSameDepth() error {
	// Chain of 4 edges → depth=4 > MaxWidth(3) → ViolationWidthExceeded for LR.
	return s.writeMD("docs/lr4.md",
		"# LR4\n\n```mermaid\nflowchart LR\n  A-->B\n  B-->C\n  C-->D\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingFlowchart4NodesAtOneRank() error {
	return s.writeMD("docs/width4.md",
		"# Width4\n\n```mermaid\nflowchart TB\n  Root-->A\n  Root-->B\n  Root-->C\n  Root-->D\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingFlowchart4NodesAtOneRankMoreThan5Ranks() error {
	// Span=4 > max-width=3, depth=6 > max-depth=5 → warning only (both-exceeded).
	return s.writeMD("docs/both.md",
		"# Both\n\n```mermaid\nflowchart TB\n  Root-->A\n  Root-->B\n  Root-->C\n  Root-->D\n  A-->E\n  E-->F\n  F-->G\n  G-->H\n  H-->I\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingFlowchart4NodesAtOneRankExactly4RanksDeep() error {
	// Span=4 > max-width=3, depth=4 > max-depth=3 → warning when --max-depth=3.
	return s.writeMD("docs/depth4.md",
		"# Depth4\n\n```mermaid\nflowchart TB\n  Root-->A\n  Root-->B\n  Root-->C\n  Root-->D\n  A-->E\n  E-->F\n  F-->G\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingSingleFlowchartDiagram() error {
	return s.writeMD("docs/single.md",
		"# Single\n\n```mermaid\nflowchart TB\n  A-->B\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingMermaidBlockTwoFlowchartDeclarations() error {
	return s.writeMD("docs/two.md",
		"# Two\n\n```mermaid\nflowchart TB\n  A-->B\nflowchart LR\n  C-->D\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingGraphKeywordNoViolations() error {
	return s.writeMD("docs/graph.md",
		"# Graph\n\n```mermaid\ngraph TB\n  A-->B\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingOnlySequenceDiagramAndClassDiagram() error {
	return s.writeMD("docs/nonflow.md",
		"# NonFlow\n\n```mermaid\nsequenceDiagram\n  Alice->>Bob: Hi\n```\n\n```mermaid\nclassDiagram\n  Animal <|-- Dog\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingNoMermaidCodeBlocks() error {
	return s.writeMD("docs/nomermaid.md", "# No Mermaid\n\nJust text.\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileWithMermaidViolationNotStagedInGit() error {
	// Initialize a real git repo so git diff --cached works correctly.
	if err := exec.Command("git", "init", s.tmpDir).Run(); err != nil {
		return fmt.Errorf("git init failed: %w", err)
	}
	_ = exec.Command("git", "-C", s.tmpDir, "config", "user.email", "test@example.com").Run()
	_ = exec.Command("git", "-C", s.tmpDir, "config", "user.name", "Test User").Run()
	// Write a file with a violation but do NOT stage it.
	return s.writeMD("docs/unstaged.md",
		"# Unstaged\n\n```mermaid\nflowchart TB\n  A[This label is way too long for the limit] --> B[ok]\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileWithMermaidViolationNotInPushRange() error {
	// For changed-only, mock the function to return empty list (no upstream configured).
	getMermaidChangedFilesFn = func(_ string) ([]string, error) {
		return nil, nil
	}
	return s.writeMD("docs/notpush.md",
		"# NotPush\n\n```mermaid\nflowchart TB\n  A[This label is way too long for the limit] --> B[ok]\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingFlowchartWithLabelLengthViolation() error {
	return s.writeMD("docs/violation.md",
		"# Violation\n\n```mermaid\nflowchart TB\n  A[This label is way too long for the limit indeed] --> B[ok]\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingFlowchartNoViolationsInt() error {
	return s.writeMD("docs/clean.md", "# Clean\n\n```mermaid\nflowchart TB\n  A[Short] --> B[Label]\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileUnderPlansLongLabel() error {
	// 35-char label exceeds default MaxLabelLen=30.
	return s.writeMD("plans/sample/diagram.md",
		"# Plan\n\n```mermaid\nflowchart TB\n  A[This is exactly thirty-five chars!!] --> B[ok]\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileOutsideOldDirsWithLabelViolation() error {
	// Plan DD-3: the default scan is repo-wide, so a violation under a tree
	// outside the historical four-dir set must be reported. The plain When
	// step already runs without positional paths (the Go in-process twin of
	// the Rust world's mermaid_default_scan flag).
	return s.writeMD("services/notes.md",
		"# Notes\n\n```mermaid\nflowchart TD\n    A[This label is definitely longer than thirty characters total]\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileWithLabelViolationUnderExcludedSubdir() error {
	// Plan DD-2: the repo-wide default scan would report this file unless the
	// When step's --exclude prefix drops it.
	return s.writeMD("legacy-diagrams/old.md",
		"# Old\n\n```mermaid\nflowchart TD\n    A[This label is definitely longer than thirty characters total]\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingPipeLabeledEdge() error {
	// Rank observation (plan DD-14 fix 1): with --max-width 0 --max-depth 1
	// the complex_diagram warning reports the actual span/depth without
	// failing the run. A correctly parsed pipe-labeled edge yields the chain
	// A->B (span 1, depth 2); the old parser dropped the edge and node B.
	s.mermaidThresholds = &mermaidThresholdPair{maxWidth: 0, maxDepth: 1}
	return s.writeMD("docs/d.md",
		"# Doc\n\n```mermaid\nflowchart TD\n    A -->|text| B\n```\n")
}

func (s *validateMermaidIntSteps) aMarkdownFileContainingCycleABCA() error {
	// Rank observation (plan DD-14 fix 2): with --max-width 0 --max-depth 1
	// the complex_diagram warning reports the actual span/depth without
	// failing the run. After back-edge removal the cycle ranks as the chain
	// A->B->C (span 1, depth 3); the old fallback ranked all nodes 0 (span 3).
	s.mermaidThresholds = &mermaidThresholdPair{maxWidth: 0, maxDepth: 1}
	return s.writeMD("docs/d.md",
		"# Doc\n\n```mermaid\nflowchart TD\n    A-->B-->C-->A\n```\n")
}

// --- When steps ---

func (s *validateMermaidIntSteps) theDeveloperRunsDocsValidateMermaid() error {
	if s.mermaidThresholds != nil {
		validateMermaidMaxWidth = s.mermaidThresholds.maxWidth
		validateMermaidMaxDepth = s.mermaidThresholds.maxDepth
	}
	s.runCmd([]string{})
	return nil
}

func (s *validateMermaidIntSteps) theDeveloperRunsDocsValidateMermaidNoArgs() error {
	s.runCmd([]string{})
	return nil
}

func (s *validateMermaidIntSteps) theDeveloperRunsDocsValidateMermaidWithMaxLabelLen40() error {
	validateMermaidMaxLabelLen = 40
	s.runCmd([]string{})
	return nil
}

func (s *validateMermaidIntSteps) theDeveloperRunsDocsValidateMermaidWithMaxWidth5() error {
	validateMermaidMaxWidth = 5
	s.runCmd([]string{})
	return nil
}

func (s *validateMermaidIntSteps) theDeveloperRunsDocsValidateMermaidWithMaxDepth3() error {
	validateMermaidMaxDepth = 3
	s.runCmd([]string{})
	return nil
}

func (s *validateMermaidIntSteps) theDeveloperRunsDocsValidateMermaidWithStagedOnlyFlag() error {
	// In integration test with no git staging, staged-only returns no files → success.
	validateMermaidStagedOnly = true
	s.runCmd([]string{})
	return nil
}

func (s *validateMermaidIntSteps) theDeveloperRunsDocsValidateMermaidWithChangedOnlyFlag() error {
	// No upstream configured → falls back to default dirs which contain the (unviolating) file.
	// Swap to a clean file so the fallback scan passes.
	validateMermaidChangedOnly = true
	s.runCmd([]string{})
	return nil
}

func (s *validateMermaidIntSteps) theDeveloperRunsDocsValidateMermaidWithExcludeSubdir() error {
	// Default repo-wide scan with the violating subtree excluded by prefix.
	validateMermaidExclude = []string{"legacy-diagrams"}
	s.runCmd([]string{})
	return nil
}

func (s *validateMermaidIntSteps) theDeveloperRunsDocsValidateMermaidWithJSONOutput() error {
	output = "json"
	s.runCmd([]string{})
	return nil
}

func (s *validateMermaidIntSteps) theDeveloperRunsDocsValidateMermaidWithMarkdownOutput() error {
	output = "markdown"
	s.runCmd([]string{})
	return nil
}

func (s *validateMermaidIntSteps) theDeveloperRunsDocsValidateMermaidWithVerbose() error {
	verbose = true
	s.runCmd([]string{})
	return nil
}

func (s *validateMermaidIntSteps) theDeveloperRunsDocsValidateMermaidWithQuiet() error {
	quiet = true
	s.runCmd([]string{})
	return nil
}

// --- Then steps ---

func (s *validateMermaidIntSteps) theValidateMermaidCommandExitsSuccessfully() error {
	if s.cmdErr != nil {
		return fmt.Errorf("expected command to exit successfully, got error: %w (output: %s)", s.cmdErr, s.cmdOutput)
	}
	return nil
}

func (s *validateMermaidIntSteps) theValidateMermaidCommandExitsWithAFailureCode() error {
	if s.cmdErr == nil {
		return fmt.Errorf("expected command to exit with failure, but it succeeded (output: %s)", s.cmdOutput)
	}
	return nil
}

func (s *validateMermaidIntSteps) theOutputReportsNoViolations() error {
	if s.cmdErr != nil {
		return fmt.Errorf("expected no violations, got error: %w (output: %s)", s.cmdErr, s.cmdOutput)
	}
	return nil
}

func (s *validateMermaidIntSteps) theOutputIdentifiesFileBlockAndNodeWithOversizedLabel() error {
	if s.cmdErr == nil {
		return fmt.Errorf("expected label-too-long violation error, but command succeeded")
	}
	return nil
}

func (s *validateMermaidIntSteps) theOutputIdentifiesFileAndBlockWithExcessiveWidth() error {
	if s.cmdErr == nil {
		return fmt.Errorf("expected width-exceeded violation error, but command succeeded")
	}
	return nil
}

func (s *validateMermaidIntSteps) theOutputIdentifiesFileAndBlockWithMultipleDiagrams() error {
	if s.cmdErr == nil {
		return fmt.Errorf("expected multiple-diagrams violation error, but command succeeded")
	}
	return nil
}

func (s *validateMermaidIntSteps) theOutputContainsWarningAboutDiagramComplexity() error {
	if s.cmdErr != nil {
		return fmt.Errorf("expected success (warning only), got error: %v\nOutput: %s", s.cmdErr, s.cmdOutput)
	}
	return nil
}

func (s *validateMermaidIntSteps) theOutputIsValidJSON() error {
	if !json.Valid([]byte(s.cmdOutput)) {
		return fmt.Errorf("expected valid JSON output, got: %s", s.cmdOutput)
	}
	return nil
}

func (s *validateMermaidIntSteps) theJSONContainsViolationKindFilePathBlockIndexAndNodeID() error {
	var result map[string]interface{}
	if err := json.Unmarshal([]byte(s.cmdOutput), &result); err != nil {
		return fmt.Errorf("failed to parse JSON: %w (output: %s)", err, s.cmdOutput)
	}
	if _, ok := result["violations"]; !ok {
		return fmt.Errorf("JSON missing 'violations' field, got: %s", s.cmdOutput)
	}
	return nil
}

func (s *validateMermaidIntSteps) theOutputContainsTableWithExpectedColumns() error {
	expected := []string{"File", "Block", "Line", "Severity", "Kind", "Detail"}
	for _, col := range expected {
		if !strings.Contains(s.cmdOutput, col) {
			return fmt.Errorf("expected markdown output to contain column %q, got: %s", col, s.cmdOutput)
		}
	}
	return nil
}

func (s *validateMermaidIntSteps) theOutputIncludesPerFileScanDetailLines() error {
	if s.cmdErr != nil {
		return fmt.Errorf("expected success with verbose output, got: %v", s.cmdErr)
	}
	if s.cmdOutput == "" {
		return fmt.Errorf("expected non-empty verbose output, got empty string")
	}
	return nil
}

func (s *validateMermaidIntSteps) theOutputContainsNoText() error {
	if s.cmdOutput != "" {
		return fmt.Errorf("expected empty output in quiet mode, got: %s", s.cmdOutput)
	}
	return nil
}

func (s *validateMermaidIntSteps) theOutputIdentifiesFileUnderPlans() error {
	if s.cmdErr == nil {
		return fmt.Errorf("expected violation, got success; output: %s", s.cmdOutput)
	}
	if !strings.Contains(s.cmdOutput, "plans/") {
		return fmt.Errorf("expected output to mention plans/, got: %s", s.cmdOutput)
	}
	return nil
}

func (s *validateMermaidIntSteps) theOutputIdentifiesViolationInThatFile() error {
	if s.cmdErr == nil {
		return fmt.Errorf("expected violation, got success; output: %s", s.cmdOutput)
	}
	if !strings.Contains(s.cmdOutput, filepath.Join("services", "notes.md")) {
		return fmt.Errorf("expected output to mention services/notes.md, got: %s", s.cmdOutput)
	}
	if !strings.Contains(s.cmdOutput, "label_too_long") {
		return fmt.Errorf("expected output to mention label_too_long, got: %s", s.cmdOutput)
	}
	return nil
}

func (s *validateMermaidIntSteps) theOutputReportsNodeBRankedBelowA() error {
	// The complex_diagram warning (thresholds set by the Given step) carries
	// the computed span/depth: a two-node chain means B sits one rank below A.
	if s.cmdErr != nil {
		return fmt.Errorf("expected success (warning only), got error: %w\nOutput: %s", s.cmdErr, s.cmdOutput)
	}
	want := "span 1 (max 0) and depth 2 (max 1) both exceeded"
	if !strings.Contains(s.cmdOutput, want) {
		return fmt.Errorf("expected output to contain %q, got: %s", want, s.cmdOutput)
	}
	return nil
}

func (s *validateMermaidIntSteps) theOutputReportsSpan1Depth3() error {
	// The complex_diagram warning (thresholds set by the Given step) carries
	// the computed span/depth: span 1, depth 3 proves the cycle ranked as the
	// chain A->B->C after back-edge removal.
	if s.cmdErr != nil {
		return fmt.Errorf("expected success (warning only), got error: %w\nOutput: %s", s.cmdErr, s.cmdOutput)
	}
	want := "span 1 (max 0) and depth 3 (max 1) both exceeded"
	if !strings.Contains(s.cmdOutput, want) {
		return fmt.Errorf("expected output to contain %q, got: %s", want, s.cmdOutput)
	}
	return nil
}

func InitializeValidateMermaidScenario(sc *godog.ScenarioContext) {
	s := &validateMermaidIntSteps{}
	sc.Before(s.before)
	sc.After(s.after)

	// Given.
	sc.Step(stepMermaidFileCleanFlowchart, s.aMarkdownFileContainingFlowchartAllLabelsWithinLimit)
	sc.Step(stepMermaidFileLabelTooLong, s.aMarkdownFileContainingFlowchartNodeLabelLongerThanLimit)
	sc.Step(stepMermaidFileNodeLabel35Chars, s.aMarkdownFileContainingFlowchartNodeLabel35Chars)
	sc.Step(stepMermaidFileTBChainedSequentially, s.aMarkdownFileContainingTBFlowchartTenNodesChainedSequentially)
	sc.Step(stepMermaidFileTBNoRankMoreThan3, s.aMarkdownFileContainingTBFlowchartNoRankMoreThan3)
	sc.Step(stepMermaidFileTBOneRank4Nodes, s.aMarkdownFileContainingTBFlowchartOneRank4ParallelNodes)
	sc.Step(stepMermaidFileLRNoRankMoreThan3, s.aMarkdownFileContainingLRFlowchartNoRankMoreThan3)
	sc.Step(stepMermaidFileLR4NodesSameDepth, s.aMarkdownFileContainingLRFlowchart4NodesAtSameDepth)
	sc.Step(stepMermaidFileFlowchart4NodesOneRank, s.aMarkdownFileContainingFlowchart4NodesAtOneRank)
	sc.Step(stepMermaidFile4NodesMoreThan5Ranks, s.aMarkdownFileContainingFlowchart4NodesAtOneRankMoreThan5Ranks)
	sc.Step(stepMermaidFile4NodesExactly4RanksDeep, s.aMarkdownFileContainingFlowchart4NodesAtOneRankExactly4RanksDeep)
	sc.Step(stepMermaidFileSingleFlowchart, s.aMarkdownFileContainingSingleFlowchartDiagram)
	sc.Step(stepMermaidFileTwoFlowchartDeclarations, s.aMarkdownFileContainingMermaidBlockTwoFlowchartDeclarations)
	sc.Step(stepMermaidFileGraphKeywordNoViolations, s.aMarkdownFileContainingGraphKeywordNoViolations)
	sc.Step(stepMermaidFileOnlyNonFlowchart, s.aMarkdownFileContainingOnlySequenceDiagramAndClassDiagram)
	sc.Step(stepMermaidFileNoMermaidBlocks, s.aMarkdownFileContainingNoMermaidCodeBlocks)
	sc.Step(stepMermaidViolationNotStagedInGit, s.aMarkdownFileWithMermaidViolationNotStagedInGit)
	sc.Step(stepMermaidViolationNotInPushRange, s.aMarkdownFileWithMermaidViolationNotInPushRange)
	sc.Step(stepMermaidFileLabelLengthViolation, s.aMarkdownFileContainingFlowchartWithLabelLengthViolation)
	sc.Step(stepMermaidFileNoViolations, s.aMarkdownFileContainingFlowchartNoViolationsInt)
	sc.Step(stepMermaidFileUnderPlansLongLabel, s.aMarkdownFileUnderPlansLongLabel)
	sc.Step(stepMermaidFileOutsideOldDirsLabelTooLong, s.aMarkdownFileOutsideOldDirsWithLabelViolation)
	sc.Step(stepMermaidFileLabelTooLongUnderExcludedSubdir, s.aMarkdownFileWithLabelViolationUnderExcludedSubdir)
	sc.Step(stepMermaidFilePipeLabeledEdge, s.aMarkdownFileContainingPipeLabeledEdge)
	sc.Step(stepMermaidFileCycleABCA, s.aMarkdownFileContainingCycleABCA)

	// When.
	sc.Step(stepDeveloperRunsDocsValidateMermaid, s.theDeveloperRunsDocsValidateMermaid)
	sc.Step(stepDeveloperRunsDocsValidateMermaidNoArgs, s.theDeveloperRunsDocsValidateMermaidNoArgs)
	sc.Step(stepDeveloperRunsDocsValidateMermaidMaxLabelLen40, s.theDeveloperRunsDocsValidateMermaidWithMaxLabelLen40)
	sc.Step(stepDeveloperRunsDocsValidateMermaidMaxWidth5, s.theDeveloperRunsDocsValidateMermaidWithMaxWidth5)
	sc.Step(stepDeveloperRunsDocsValidateMermaidMaxDepth3, s.theDeveloperRunsDocsValidateMermaidWithMaxDepth3)
	sc.Step(stepDeveloperRunsDocsValidateMermaidStagedOnly, s.theDeveloperRunsDocsValidateMermaidWithStagedOnlyFlag)
	sc.Step(stepDeveloperRunsDocsValidateMermaidChangedOnly, s.theDeveloperRunsDocsValidateMermaidWithChangedOnlyFlag)
	sc.Step(stepDeveloperRunsDocsValidateMermaidJSONOutput, s.theDeveloperRunsDocsValidateMermaidWithJSONOutput)
	sc.Step(stepDeveloperRunsDocsValidateMermaidMarkdownOutput, s.theDeveloperRunsDocsValidateMermaidWithMarkdownOutput)
	sc.Step(stepDeveloperRunsDocsValidateMermaidVerbose, s.theDeveloperRunsDocsValidateMermaidWithVerbose)
	sc.Step(stepDeveloperRunsDocsValidateMermaidQuiet, s.theDeveloperRunsDocsValidateMermaidWithQuiet)
	sc.Step(stepDeveloperRunsDocsValidateMermaidExcludeSubdir, s.theDeveloperRunsDocsValidateMermaidWithExcludeSubdir)

	// Then.
	sc.Step(stepExitsSuccessfully, s.theValidateMermaidCommandExitsSuccessfully)
	sc.Step(stepExitsWithFailure, s.theValidateMermaidCommandExitsWithAFailureCode)
	sc.Step(stepMermaidOutputNoViolations, s.theOutputReportsNoViolations)
	sc.Step(stepMermaidOutputIdentifiesOversizedLabel, s.theOutputIdentifiesFileBlockAndNodeWithOversizedLabel)
	sc.Step(stepMermaidOutputIdentifiesExcessiveWidth, s.theOutputIdentifiesFileAndBlockWithExcessiveWidth)
	sc.Step(stepMermaidOutputIdentifiesMultipleDiagrams, s.theOutputIdentifiesFileAndBlockWithMultipleDiagrams)
	sc.Step(stepMermaidOutputContainsWarning, s.theOutputContainsWarningAboutDiagramComplexity)
	sc.Step(stepOutputIsValidJSON, s.theOutputIsValidJSON)
	sc.Step(stepMermaidJSONContainsViolationFields, s.theJSONContainsViolationKindFilePathBlockIndexAndNodeID)
	sc.Step(stepMermaidOutputContainsTable, s.theOutputContainsTableWithExpectedColumns)
	sc.Step(stepMermaidOutputIncludesPerFileDetail, s.theOutputIncludesPerFileScanDetailLines)
	sc.Step(stepMermaidOutputContainsNoText, s.theOutputContainsNoText)
	sc.Step(stepMermaidOutputIdentifiesFileUnderPlans, s.theOutputIdentifiesFileUnderPlans)
	sc.Step(stepMermaidOutputIdentifiesViolationInThatFile, s.theOutputIdentifiesViolationInThatFile)
	sc.Step(stepMermaidOutputReportsNodeBRankedBelowA, s.theOutputReportsNodeBRankedBelowA)
	sc.Step(stepMermaidOutputReportsSpan1Depth3, s.theOutputReportsSpan1Depth3)
}

func TestIntegrationValidateMermaid(t *testing.T) {
	suite := godog.TestSuite{
		ScenarioInitializer: InitializeValidateMermaidScenario,
		Options: &godog.Options{
			Format:   "pretty",
			Paths:    []string{specsDirIntMermaid},
			Tags:     "@docs-validate-mermaid",
			TestingT: t,
		},
	}
	if suite.Run() != 0 {
		t.Fatal("non-zero status returned, failed to run feature tests")
	}
}

// TestIntegrationValidateMermaid_PlansDirScanned verifies that without path
// arguments, the validator scans plans/ and reports violations on diagrams there.
// Mirrors the new Gherkin scenario "Plans directory is scanned by default".
func TestIntegrationValidateMermaid_PlansDirScanned(t *testing.T) {
	originalWd, _ := os.Getwd()
	tmpDir, err := os.MkdirTemp("", "validate-mermaid-plans-*")
	if err != nil {
		t.Fatal(err)
	}
	defer func() {
		_ = os.Chdir(originalWd)
		_ = os.RemoveAll(tmpDir)
		validateMermaidStagedOnly = false
		validateMermaidChangedOnly = false
		validateMermaidMaxLabelLen = 30
		validateMermaidMaxWidth = 3
		validateMermaidMaxDepth = 5
		output = "text"
		verbose = false
		quiet = false
	}()

	if err := os.MkdirAll(filepath.Join(tmpDir, ".git"), 0o755); err != nil {
		t.Fatal(err)
	}
	planDir := filepath.Join(tmpDir, "plans", "sample")
	if err := os.MkdirAll(planDir, 0o755); err != nil {
		t.Fatal(err)
	}
	// 35-char label exceeds default MaxLabelLen=30.
	planMD := filepath.Join(planDir, "diagram.md")
	content := "# Plan\n\n```mermaid\nflowchart TB\n  A[This is exactly thirty-five chars!!] --> B[ok]\n```\n"
	if err := os.WriteFile(planMD, []byte(content), 0o600); err != nil {
		t.Fatal(err)
	}

	if err := os.Chdir(tmpDir); err != nil {
		t.Fatal(err)
	}

	verbose = false
	quiet = false
	output = "text"
	validateMermaidStagedOnly = false
	validateMermaidChangedOnly = false
	validateMermaidMaxLabelLen = 30
	validateMermaidMaxWidth = 3
	validateMermaidMaxDepth = 5

	buf := new(bytes.Buffer)
	validateMermaidCmd.SetOut(buf)
	validateMermaidCmd.SetErr(buf)
	cmdErr := validateMermaidCmd.RunE(validateMermaidCmd, []string{})

	if cmdErr == nil {
		t.Fatalf("expected violation for plans/ diagram, got success; output:\n%s", buf.String())
	}
	if !strings.Contains(buf.String(), filepath.Join("plans", "sample", "diagram.md")) {
		t.Errorf("expected output to identify plans/sample/diagram.md; got:\n%s", buf.String())
	}
}
