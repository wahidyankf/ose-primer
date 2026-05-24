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
	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/agents"
)

var specsDirUnitBindings = func() string {
	_, f, _, _ := runtime.Caller(0)
	return filepath.Join(filepath.Dir(f), "../../../specs/apps/rhino/behavior/cli/gherkin")
}()

// bindingsUnitSteps drives the real runEmitBindings / runValidateBindings
// handlers against a temporary git root. The handlers call findGitRoot() and
// the real agents.EmitBindings / agents.ValidateBindings, so the suite chdirs
// into a temp dir containing a .git marker (mirroring the integration sibling)
// rather than mocking the OS hooks. No writes touch the real repo tree.
type bindingsUnitSteps struct {
	originalWd string
	tmpDir     string
	cmdErr     error
	cmdOutput  string
}

func (s *bindingsUnitSteps) before(_ context.Context, _ *godog.Scenario) (context.Context, error) {
	verbose = false
	quiet = false
	output = "text"
	emitBindingsDryRun = false
	s.cmdErr = nil
	s.cmdOutput = ""

	s.originalWd, _ = os.Getwd()
	s.tmpDir, _ = os.MkdirTemp("", "agents-bindings-*")
	_ = os.MkdirAll(filepath.Join(s.tmpDir, ".git"), 0o755)
	_ = os.Chdir(s.tmpDir)

	return context.Background(), nil
}

func (s *bindingsUnitSteps) after(_ context.Context, _ *godog.Scenario, _ error) (context.Context, error) {
	_ = os.Chdir(s.originalWd)
	_ = os.RemoveAll(s.tmpDir)
	emitBindingsDryRun = false
	return context.Background(), nil
}

// writeExpectedBindings writes every expected binding file into the temp root
// with its canonical content.
func (s *bindingsUnitSteps) writeExpectedBindings() error {
	for _, bf := range agents.ExpectedBindings() {
		target := filepath.Join(s.tmpDir, filepath.FromSlash(bf.Path))
		if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
			return fmt.Errorf("failed to create dir for %s: %w", bf.Path, err)
		}
		if err := os.WriteFile(target, []byte(bf.Content), 0o644); err != nil {
			return fmt.Errorf("failed to write %s: %w", bf.Path, err)
		}
	}
	return nil
}

// writeCatalog writes a platform-bindings catalog containing the provided text.
func (s *bindingsUnitSteps) writeCatalog(text string) error {
	catalogPath := filepath.Join(s.tmpDir, filepath.FromSlash("docs/reference/platform-bindings.md"))
	if err := os.MkdirAll(filepath.Dir(catalogPath), 0o755); err != nil {
		return fmt.Errorf("failed to create catalog dir: %w", err)
	}
	if err := os.WriteFile(catalogPath, []byte(text), 0o644); err != nil {
		return fmt.Errorf("failed to write catalog: %w", err)
	}
	return nil
}

// --- Given steps ---

func (s *bindingsUnitSteps) aRepositoryWithACanonicalAgentsMD() error {
	// The handlers only require a git root; AGENTS.md content is not read by
	// EmitBindings (the bridge files reference it, they do not copy its body).
	return os.WriteFile(filepath.Join(s.tmpDir, "AGENTS.md"), []byte("# AGENTS\n"), 0o644)
}

func (s *bindingsUnitSteps) aRepositoryWhoseCommittedBindingsMatch() error {
	return s.writeExpectedBindings()
}

func (s *bindingsUnitSteps) theCatalogDocumentsEveryBindingDirectory() error {
	var sb strings.Builder
	for _, dir := range agents.BindingDirsForCatalog() {
		if err := os.MkdirAll(filepath.Join(s.tmpDir, dir), 0o755); err != nil {
			return fmt.Errorf("failed to create dir %s: %w", dir, err)
		}
		sb.WriteString("documents ")
		sb.WriteString(dir)
		sb.WriteString("\n")
	}
	return s.writeCatalog(sb.String())
}

func (s *bindingsUnitSteps) aRepositoryWhereACommittedBindingDrifts() error {
	if err := s.writeExpectedBindings(); err != nil {
		return err
	}
	// Mutate the first expected binding so its bytes no longer match.
	first := agents.ExpectedBindings()[0]
	target := filepath.Join(s.tmpDir, filepath.FromSlash(first.Path))
	if err := os.WriteFile(target, []byte("mutated content\n"), 0o644); err != nil {
		return fmt.Errorf("failed to mutate %s: %w", first.Path, err)
	}
	// Document the one binding dir that exists (.amazonq) so the only problem
	// reported is the drift, not a catalog gap.
	return s.writeCatalog("documents .amazonq\n")
}

func (s *bindingsUnitSteps) aRepositoryWithUndocumentedBindingDir() error {
	if err := s.writeExpectedBindings(); err != nil {
		return err
	}
	// Add a second binding dir (.claude) so two dirs exist on disk: the
	// catalog documents only .claude, leaving .amazonq missing.
	if err := os.MkdirAll(filepath.Join(s.tmpDir, ".claude"), 0o755); err != nil {
		return fmt.Errorf("failed to create .claude dir: %w", err)
	}
	return s.writeCatalog("this catalog documents .claude only\n")
}

// --- When steps ---

func (s *bindingsUnitSteps) theDeveloperRunsEmitBindings() error {
	buf := new(bytes.Buffer)
	emitBindingsCmd.SetOut(buf)
	emitBindingsCmd.SetErr(buf)
	s.cmdErr = emitBindingsCmd.RunE(emitBindingsCmd, []string{})
	s.cmdOutput = buf.String()
	return nil
}

func (s *bindingsUnitSteps) theDeveloperRunsEmitBindingsWithDryRun() error {
	emitBindingsDryRun = true
	return s.theDeveloperRunsEmitBindings()
}

func (s *bindingsUnitSteps) theDeveloperRunsValidateBindings() error {
	buf := new(bytes.Buffer)
	validateBindingsCmd.SetOut(buf)
	validateBindingsCmd.SetErr(buf)
	s.cmdErr = validateBindingsCmd.RunE(validateBindingsCmd, []string{})
	s.cmdOutput = buf.String()
	return nil
}

// --- Then steps ---

func (s *bindingsUnitSteps) theCommandExitsSuccessfully() error {
	if s.cmdErr != nil {
		return fmt.Errorf("expected success but got: %w\nOutput: %s", s.cmdErr, s.cmdOutput)
	}
	return nil
}

func (s *bindingsUnitSteps) theCommandExitsWithAFailureCode() error {
	if s.cmdErr == nil {
		return fmt.Errorf("expected failure but succeeded\nOutput: %s", s.cmdOutput)
	}
	return nil
}

func (s *bindingsUnitSteps) theRulesPointerAndAgentJSONAreWritten() error {
	for _, bf := range agents.ExpectedBindings() {
		target := filepath.Join(s.tmpDir, filepath.FromSlash(bf.Path))
		got, err := os.ReadFile(target)
		if err != nil {
			return fmt.Errorf("expected %s to be written: %w", bf.Path, err)
		}
		if string(got) != bf.Content {
			return fmt.Errorf("content mismatch for %s", bf.Path)
		}
	}
	return nil
}

func (s *bindingsUnitSteps) eachGeneratedFileReferencesAgentsMD() error {
	// The Amazon Q rules bridge file points at AGENTS.md; the agent JSON loads
	// it as a resource. Neither copies the body, so both must mention AGENTS.md.
	for _, bf := range agents.ExpectedBindings() {
		target := filepath.Join(s.tmpDir, filepath.FromSlash(bf.Path))
		got, err := os.ReadFile(target)
		if err != nil {
			return fmt.Errorf("failed to read %s: %w", bf.Path, err)
		}
		if !strings.Contains(string(got), "AGENTS.md") {
			return fmt.Errorf("expected %s to reference AGENTS.md", bf.Path)
		}
	}
	return nil
}

func (s *bindingsUnitSteps) theOutputListsTheFilesThatWouldBeWritten() error {
	want := "would write .amazonq/cli-agents/ose-default.json\n" +
		"would write .amazonq/rules/00-agents-md.md\n" +
		"emit-bindings: would write 2 binding file(s) (dry-run)\n"
	if s.cmdOutput != want {
		return fmt.Errorf("dry-run output mismatch:\n got:\n%q\nwant:\n%q", s.cmdOutput, want)
	}
	return nil
}

func (s *bindingsUnitSteps) noBindingFilesAreCreatedOnDisk() error {
	for _, bf := range agents.ExpectedBindings() {
		target := filepath.Join(s.tmpDir, filepath.FromSlash(bf.Path))
		if _, err := os.Stat(target); !os.IsNotExist(err) {
			return fmt.Errorf("dry-run unexpectedly wrote %s (err=%v)", bf.Path, err)
		}
	}
	return nil
}

func (s *bindingsUnitSteps) theOutputReportsZeroDriftAndZeroGaps() error {
	if !strings.Contains(s.cmdOutput, "VALIDATION PASSED") {
		return fmt.Errorf("expected VALIDATION PASSED, output:\n%s", s.cmdOutput)
	}
	if !strings.Contains(s.cmdOutput, "0 drift") {
		return fmt.Errorf("expected zero drift, output:\n%s", s.cmdOutput)
	}
	if !strings.Contains(s.cmdOutput, "0 missing") {
		return fmt.Errorf("expected zero missing, output:\n%s", s.cmdOutput)
	}
	return nil
}

func (s *bindingsUnitSteps) theOutputIdentifiesTheDriftedBindingFile() error {
	first := agents.ExpectedBindings()[0]
	if !strings.Contains(s.cmdOutput, "DRIFT "+first.Path) {
		return fmt.Errorf("expected DRIFT line for %s, output:\n%s", first.Path, s.cmdOutput)
	}
	return nil
}

func (s *bindingsUnitSteps) theOutputIdentifiesTheMissingBindingDir() error {
	if !strings.Contains(s.cmdOutput, "MISSING-CATALOG .amazonq") {
		return fmt.Errorf("expected MISSING-CATALOG line, output:\n%s", s.cmdOutput)
	}
	return nil
}

func TestUnitAgentsBindings(t *testing.T) {
	s := &bindingsUnitSteps{}
	suite := godog.TestSuite{
		ScenarioInitializer: func(sc *godog.ScenarioContext) {
			sc.Before(s.before)
			sc.After(s.after)

			sc.Step(`^a repository with a canonical AGENTS.md at the root$`, s.aRepositoryWithACanonicalAgentsMD)
			sc.Step(`^a repository whose committed binding files match a fresh regenerate$`, s.aRepositoryWhoseCommittedBindingsMatch)
			sc.Step(`^the platform-bindings catalog documents every binding directory present on disk$`, s.theCatalogDocumentsEveryBindingDirectory)
			sc.Step(`^a repository where a committed binding file no longer matches a regenerate from AGENTS.md$`, s.aRepositoryWhereACommittedBindingDrifts)
			sc.Step(`^a repository with a binding directory that the platform-bindings catalog does not document$`, s.aRepositoryWithUndocumentedBindingDir)

			sc.Step(`^the developer runs agents emit-bindings$`, s.theDeveloperRunsEmitBindings)
			sc.Step(`^the developer runs agents emit-bindings with the --dry-run flag$`, s.theDeveloperRunsEmitBindingsWithDryRun)
			sc.Step(`^the developer runs agents validate-bindings$`, s.theDeveloperRunsValidateBindings)

			sc.Step(stepExitsSuccessfully, s.theCommandExitsSuccessfully)
			sc.Step(stepExitsWithFailure, s.theCommandExitsWithAFailureCode)
			sc.Step(`^the Amazon Q rules pointer and default agent JSON are written$`, s.theRulesPointerAndAgentJSONAreWritten)
			sc.Step(`^each generated file references AGENTS.md without duplicating its body$`, s.eachGeneratedFileReferencesAgentsMD)
			sc.Step(`^the output lists the files that would be written$`, s.theOutputListsTheFilesThatWouldBeWritten)
			sc.Step(`^no binding files are created on disk$`, s.noBindingFilesAreCreatedOnDisk)
			sc.Step(`^the output reports zero binding drift and zero catalog gaps$`, s.theOutputReportsZeroDriftAndZeroGaps)
			sc.Step(`^the output identifies the drifted binding file$`, s.theOutputIdentifiesTheDriftedBindingFile)
			sc.Step(`^the output identifies the binding directory missing from the catalog$`, s.theOutputIdentifiesTheMissingBindingDir)
		},
		Options: &godog.Options{
			Format:   "pretty",
			Paths:    []string{specsDirUnitBindings},
			TestingT: t,
			Tags:     "agents-bindings",
		},
	}
	if suite.Run() != 0 {
		t.Fatal("non-zero status returned, failed to run unit feature tests")
	}
}

// TestEmitBindingsCommand_MissingGitRoot verifies git root detection in the
// emit-bindings handler — not in Gherkin specs.
func TestEmitBindingsCommand_MissingGitRoot(t *testing.T) {
	origGetwd := osGetwd
	origStat := osStat
	defer func() {
		osGetwd = origGetwd
		osStat = origStat
	}()

	osGetwd = func() (string, error) { return "/no-git-here", nil }
	osStat = func(_ string) (os.FileInfo, error) { return nil, os.ErrNotExist }

	buf := new(bytes.Buffer)
	emitBindingsCmd.SetOut(buf)
	emitBindingsCmd.SetErr(buf)

	output = "text"
	verbose = false
	quiet = false
	emitBindingsDryRun = true

	err := emitBindingsCmd.RunE(emitBindingsCmd, []string{})
	if err == nil {
		t.Error("expected error when no .git directory found")
	}
	if !strings.Contains(err.Error(), "git") {
		t.Errorf("expected error mentioning 'git', got: %v", err)
	}
	emitBindingsDryRun = false
}

// TestValidateBindingsCommand_MissingGitRoot verifies git root detection in the
// validate-bindings handler — not in Gherkin specs.
func TestValidateBindingsCommand_MissingGitRoot(t *testing.T) {
	origGetwd := osGetwd
	origStat := osStat
	defer func() {
		osGetwd = origGetwd
		osStat = origStat
	}()

	osGetwd = func() (string, error) { return "/no-git-here", nil }
	osStat = func(_ string) (os.FileInfo, error) { return nil, os.ErrNotExist }

	buf := new(bytes.Buffer)
	validateBindingsCmd.SetOut(buf)
	validateBindingsCmd.SetErr(buf)

	output = "text"
	verbose = false
	quiet = false

	err := validateBindingsCmd.RunE(validateBindingsCmd, []string{})
	if err == nil {
		t.Error("expected error when no .git directory found")
	}
	if !strings.Contains(err.Error(), "git") {
		t.Errorf("expected error mentioning 'git', got: %v", err)
	}
}
