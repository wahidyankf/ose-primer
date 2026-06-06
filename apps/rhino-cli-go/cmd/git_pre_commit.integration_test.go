//go:build integration

package cmd

import (
	"bytes"
	"context"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"testing"

	"github.com/cucumber/godog"
	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/git"
)

var specsGitPreCommitDir = func() string {
	_, f, _, _ := runtime.Caller(0)
	return filepath.Join(filepath.Dir(f), "../../../specs/apps/rhino/behavior/cli/gherkin")
}()

// stubbedTools are the external tools replaced by always-succeeding PATH
// stubs for the staged-file scenarios (DD-8), so only the deterministic
// in-process steps (staged mermaid, heading-hierarchy, and link validation)
// decide the outcome. `git` is deliberately NOT stubbed — staged-file
// detection needs it. Mirrors the Rust device's STUBBED_TOOLS.
var stubbedTools = []string{"docker", "nx", "npx", "npm"}

// Two fixture shapes are used:
//
//   - Outside a git repository: a temp directory containing NO .git entry,
//     so the orchestrator fails at findGitRoot before any external-tool
//     step runs.
//     Scenario: Running pre-commit outside a git repository fails
//
//   - Staged-file scenarios (DD-8): a freshly git init-ed temp workspace
//     with exactly the scenario's file staged.
//     Scenario: staged-mermaid-blocks - a staged markdown file with a malformed flowchart fails pre-commit
//     Scenario: staged-prose-heading-blocks - a staged docs markdown file with a duplicate H1 fails pre-commit
//     Scenario: staged-skill-file-exempt - a staged SKILL.md under .claude/skills with many H1s passes the heading step
//     Scenario: link-step-honors-exclusions - the link step skips files under plans/done

type gitPreCommitSteps struct {
	originalWd   string
	originalPath string
	tmpDir       string
	stubDir      string
	cmdErr       error
	cmdOutput    string
}

func (s *gitPreCommitSteps) before(_ context.Context, _ *godog.Scenario) (context.Context, error) {
	s.originalWd, _ = os.Getwd()
	s.originalPath = os.Getenv("PATH")
	// No git init here: the outside-a-repo scenario needs findGitRoot to walk
	// up and fail. Staged-file Given steps call initRepo themselves.
	s.tmpDir, _ = os.MkdirTemp("", "git-pre-commit-*")
	s.stubDir, _ = os.MkdirTemp("", "git-pre-commit-stubs-*")
	for _, tool := range stubbedTools {
		if err := writeStubTool(s.stubDir, tool); err != nil {
			return context.Background(), err
		}
	}
	// Prepend the stub bin dir so the orchestrator's external tools succeed
	// deterministically; harmless for the outside-a-repo scenario, which
	// fails before any tool runs.
	_ = os.Setenv("PATH", s.stubDir+string(os.PathListSeparator)+s.originalPath)
	_ = os.Chdir(s.tmpDir)
	return context.Background(), nil
}

func (s *gitPreCommitSteps) after(_ context.Context, _ *godog.Scenario, _ error) (context.Context, error) {
	_ = os.Chdir(s.originalWd)
	_ = os.Setenv("PATH", s.originalPath)
	_ = os.RemoveAll(s.tmpDir)
	_ = os.RemoveAll(s.stubDir)
	return context.Background(), nil
}

// writeStubTool writes an always-succeeding executable stub named name into dir.
func writeStubTool(dir, name string) error {
	return os.WriteFile(filepath.Join(dir, name), []byte("#!/bin/sh\nexit 0\n"), 0o755)
}

// initRepo turns the temp workspace into a fresh git repository.
func (s *gitPreCommitSteps) initRepo() error {
	cmd := exec.Command("git", "init", "-q")
	cmd.Dir = s.tmpDir
	if err := cmd.Run(); err != nil {
		return fmt.Errorf("git init failed: %w", err)
	}
	return nil
}

// writeWorkspaceFile writes content to tmpDir/rel, creating parent directories.
func (s *gitPreCommitSteps) writeWorkspaceFile(rel, content string) error {
	path := filepath.Join(s.tmpDir, filepath.FromSlash(rel))
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	return os.WriteFile(path, []byte(content), 0o644)
}

// stage stages rel in the workspace repository.
func (s *gitPreCommitSteps) stage(rel string) error {
	cmd := exec.Command("git", "add", rel)
	cmd.Dir = s.tmpDir
	if err := cmd.Run(); err != nil {
		return fmt.Errorf("git add %s failed: %w", rel, err)
	}
	return nil
}

func (s *gitPreCommitSteps) theDeveloperIsOutsideAGitRepository() error {
	// tmpDir has no .git directory — already set up in before().
	return nil
}

func (s *gitPreCommitSteps) aStagedMalformedFlowchartFile() error {
	if err := s.initRepo(); err != nil {
		return err
	}
	// Node label far exceeds the 30-character limit — a blocking violation.
	if err := s.writeWorkspaceFile("docs/diagram.md",
		"# Doc\n\n```mermaid\nflowchart TD\n  A[This label is far longer than the thirty character limit] --> B[Ok]\n```\n"); err != nil {
		return err
	}
	return s.stage("docs/diagram.md")
}

func (s *gitPreCommitSteps) aStagedDocsFileWithDuplicateH1() error {
	if err := s.initRepo(); err != nil {
		return err
	}
	if err := s.writeWorkspaceFile("docs/guide.md",
		"# First Title\n\ntext\n\n# Second Title\n"); err != nil {
		return err
	}
	return s.stage("docs/guide.md")
}

func (s *gitPreCommitSteps) aStagedSkillFileWithMultipleH1s() error {
	if err := s.initRepo(); err != nil {
		return err
	}
	// A staged `.claude/` path also triggers the config-validation step
	// (step 1), so the fixture must be a minimally VALID `.claude/` tree:
	// an empty agents dir plus a skill whose frontmatter passes
	// validate-claude. The body's multiple H1s are the heading-gate
	// exemption under test.
	if err := os.MkdirAll(filepath.Join(s.tmpDir, ".claude", "agents"), 0o755); err != nil {
		return err
	}
	if err := s.writeWorkspaceFile(".claude/skills/example-skill/SKILL.md",
		"---\nname: example-skill\ndescription: Fixture skill for the pre-commit heading-gate exemption test.\n---\n\n# First H1\n\ntext\n\n# Second H1\n\ntext\n\n# Third H1\n"); err != nil {
		return err
	}
	return s.stage(".claude/skills/example-skill/SKILL.md")
}

func (s *gitPreCommitSteps) aStagedPlansDoneFileWithBrokenLink() error {
	if err := s.initRepo(); err != nil {
		return err
	}
	if err := s.writeWorkspaceFile("plans/done/2026-01-01__archived-plan/notes.md",
		"# Archived Notes\n\nSee the [missing target](./missing.md) for details.\n"); err != nil {
		return err
	}
	return s.stage("plans/done/2026-01-01__archived-plan/notes.md")
}

func (s *gitPreCommitSteps) runGitPreCommit() error {
	buf := new(bytes.Buffer)
	gitPreCommitCmd.SetOut(buf)
	gitPreCommitCmd.SetErr(buf)
	// Capture the runner's deterministic step output (the runner writes to
	// its injected Deps streams, not to the cobra writers).
	origDeps := gitDefaultDepsFn
	gitDefaultDepsFn = func() git.Deps {
		d := git.DefaultDeps()
		d.Stdout = buf
		d.Stderr = buf
		return d
	}
	defer func() { gitDefaultDepsFn = origDeps }()
	s.cmdErr = gitPreCommitCmd.RunE(gitPreCommitCmd, []string{})
	s.cmdOutput = buf.String()
	return nil
}

// combined returns the captured step output plus the returned error text.
func (s *gitPreCommitSteps) combined() string {
	combined := s.cmdOutput
	if s.cmdErr != nil {
		combined += s.cmdErr.Error()
	}
	return combined
}

func (s *gitPreCommitSteps) commandExitsWithFailureCode() error {
	if s.cmdErr == nil {
		return fmt.Errorf("expected command to fail, but it succeeded\noutput: %s", s.cmdOutput)
	}
	return nil
}

func (s *gitPreCommitSteps) commandExitsSuccessfully() error {
	if s.cmdErr != nil {
		return fmt.Errorf("expected command to succeed, got: %v\noutput: %s", s.cmdErr, s.cmdOutput)
	}
	return nil
}

func (s *gitPreCommitSteps) outputMentionsGitRepositoryNotFound() error {
	combined := s.combined()
	if !strings.Contains(combined, "git") {
		return fmt.Errorf("expected output or error to mention 'git', got output=%q err=%q",
			s.cmdOutput, s.cmdErr)
	}
	return nil
}

func (s *gitPreCommitSteps) outputReportsMermaidViolationForStagedFile() error {
	combined := s.combined()
	if !strings.Contains(combined, "docs/diagram.md") || !strings.Contains(combined, "mermaid violation") {
		return fmt.Errorf("expected a mermaid violation naming docs/diagram.md, got: %s", combined)
	}
	return nil
}

func (s *gitPreCommitSteps) outputReportsHeadingViolationForStagedFile() error {
	combined := s.combined()
	if !strings.Contains(combined, "docs/guide.md") || !strings.Contains(combined, "heading hierarchy") {
		return fmt.Errorf("expected a heading hierarchy finding naming docs/guide.md, got: %s", combined)
	}
	return nil
}

func (s *gitPreCommitSteps) noHeadingViolationReportedForSkillFile() error {
	combined := s.combined()
	if strings.Contains(combined, "duplicate-h1") || strings.Contains(combined, "heading hierarchy") {
		return fmt.Errorf("expected no heading finding for the exempt SKILL.md, got: %s", combined)
	}
	return nil
}

func (s *gitPreCommitSteps) noBrokenLinkViolationReportedForPlansDoneFile() error {
	combined := s.combined()
	if strings.Contains(combined, "broken links") || strings.Contains(combined, "missing.md") {
		return fmt.Errorf("expected the plans/done broken link to be skipped, got: %s", combined)
	}
	return nil
}

func InitializeGitPreCommitScenario(sc *godog.ScenarioContext) {
	s := &gitPreCommitSteps{}
	sc.Before(s.before)
	sc.After(s.after)

	sc.Step(stepDeveloperIsOutsideGitRepository, s.theDeveloperIsOutsideAGitRepository)
	sc.Step(stepStagedMalformedFlowchartFile, s.aStagedMalformedFlowchartFile)
	sc.Step(stepStagedDocsFileWithDuplicateH1, s.aStagedDocsFileWithDuplicateH1)
	sc.Step(stepStagedSkillFileWithMultipleH1s, s.aStagedSkillFileWithMultipleH1s)
	sc.Step(stepStagedPlansDoneFileWithBrokenLink, s.aStagedPlansDoneFileWithBrokenLink)
	sc.Step(stepDeveloperRunsGitPreCommit, s.runGitPreCommit)
	sc.Step(stepExitsWithFailure, s.commandExitsWithFailureCode)
	sc.Step(stepExitsSuccessfully, s.commandExitsSuccessfully)
	sc.Step(stepOutputMentionsGitRepositoryNotFound, s.outputMentionsGitRepositoryNotFound)
	sc.Step(stepOutputReportsMermaidViolationStaged, s.outputReportsMermaidViolationForStagedFile)
	sc.Step(stepOutputReportsHeadingViolationStaged, s.outputReportsHeadingViolationForStagedFile)
	sc.Step(stepNoHeadingViolationForSkillFile, s.noHeadingViolationReportedForSkillFile)
	sc.Step(stepNoBrokenLinkViolationForPlansDone, s.noBrokenLinkViolationReportedForPlansDoneFile)
}

func TestIntegrationGitPreCommit(t *testing.T) {
	suite := godog.TestSuite{
		ScenarioInitializer: InitializeGitPreCommitScenario,
		Options: &godog.Options{
			Format:   "pretty",
			Paths:    []string{specsGitPreCommitDir},
			Tags:     "git-pre-commit",
			TestingT: t,
		},
	}
	if suite.Run() != 0 {
		t.Fatal("non-zero status returned, failed to run feature tests")
	}
}
