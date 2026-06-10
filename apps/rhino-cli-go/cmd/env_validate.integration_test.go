//go:build integration

package cmd

import (
	"context"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"testing"

	"github.com/cucumber/godog"
)

var specsIntEnvValidateDir = func() string {
	_, f, _, _ := runtime.Caller(0)
	return filepath.Join(filepath.Dir(f), "../../../specs/apps/rhino/behavior/cli/gherkin")
}()

const (
	stepEnvValidateFixtureAppDeclaredButUnread    = `^a fixture app whose \.env\.example declares FIXTURE_JWT_SECRET but whose source never reads it$`
	stepEnvValidateFixtureAppReadButUndeclared    = `^a fixture app whose source reads FIXTURE_JWT_SECRET but whose \.env\.example does not declare it$`
	stepEnvValidateFixtureAppMatchingDeclaredRead = `^a fixture app whose \.env\.example declares FIXTURE_JWT_SECRET and whose source reads it$`
	stepEnvValidateFixtureAppAllowlisted          = `^a fixture app that reads ENABLE_TEST_API and a framework-injected PORT variant but neither is declared in \.env\.example$`
	stepDeveloperRunsEnvValidate                  = `^the developer runs rhino-cli env validate$`
	stepOutputNamesDeclaredButUnread              = `^the output names FIXTURE_JWT_SECRET as a declared-but-unread key$`
	stepOutputNamesReadButUndeclared              = `^the output names FIXTURE_JWT_SECRET as a read-but-undeclared key$`
	stepOutputReportsValidationPassed             = `^the output reports validation passed$`
)

type envValidateIntSteps struct {
	repoDir   string
	cmdErr    error
	cmdOutput string
}

func (s *envValidateIntSteps) before(_ context.Context, _ *godog.Scenario) (context.Context, error) {
	s.repoDir = ""
	s.cmdErr = nil
	s.cmdOutput = ""
	return context.Background(), nil
}

func (s *envValidateIntSteps) after(_ context.Context, _ *godog.Scenario, _ error) (context.Context, error) {
	if s.repoDir != "" {
		os.RemoveAll(s.repoDir)
	}
	return context.Background(), nil
}

func (s *envValidateIntSteps) makeFixtureRepo(t testing.TB) string {
	dir, err := os.MkdirTemp("", "env-validate-fixture-*")
	if err != nil {
		t.Fatalf("mkdirtemp: %v", err)
	}
	if err := os.MkdirAll(filepath.Join(dir, ".git"), 0o755); err != nil {
		t.Fatalf("mkdir .git: %v", err)
	}
	return dir
}

func writeFixtureFile(dir, rel, content string) error {
	p := filepath.Join(dir, rel)
	if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
		return err
	}
	return os.WriteFile(p, []byte(content), 0o644)
}

func (s *envValidateIntSteps) aFixtureAppDeclaredButNotRead(ctx context.Context) (context.Context, error) {
	dir, err := os.MkdirTemp("", "env-validate-fixture-*")
	if err != nil {
		return ctx, err
	}
	s.repoDir = dir
	if err := os.MkdirAll(filepath.Join(dir, ".git"), 0o755); err != nil {
		return ctx, err
	}
	if err := writeFixtureFile(dir, "infra/dev/crud-be-golang-gin/.env.example", "FIXTURE_JWT_SECRET=change-me\n"); err != nil {
		return ctx, err
	}
	return ctx, nil
}

func (s *envValidateIntSteps) aFixtureAppReadButNotDeclared(ctx context.Context) (context.Context, error) {
	dir, err := os.MkdirTemp("", "env-validate-fixture-*")
	if err != nil {
		return ctx, err
	}
	s.repoDir = dir
	if err := os.MkdirAll(filepath.Join(dir, ".git"), 0o755); err != nil {
		return ctx, err
	}
	if err := writeFixtureFile(dir, "apps/crud-be-golang-gin/main.go",
		`package main
import "os"
func main() { _ = os.Getenv("FIXTURE_JWT_SECRET") }
`); err != nil {
		return ctx, err
	}
	return ctx, nil
}

func (s *envValidateIntSteps) aFixtureAppMatchingDeclaredAndRead(ctx context.Context) (context.Context, error) {
	dir, err := os.MkdirTemp("", "env-validate-fixture-*")
	if err != nil {
		return ctx, err
	}
	s.repoDir = dir
	if err := os.MkdirAll(filepath.Join(dir, ".git"), 0o755); err != nil {
		return ctx, err
	}
	if err := writeFixtureFile(dir, "infra/dev/crud-be-golang-gin/.env.example", "CRUD_BE_GOLANG_GIN_JWT_SECRET=change-me\n"); err != nil {
		return ctx, err
	}
	if err := writeFixtureFile(dir, "apps/crud-be-golang-gin/main.go",
		`package main
import "os"
func main() { _ = os.Getenv("CRUD_BE_GOLANG_GIN_JWT_SECRET") }
`); err != nil {
		return ctx, err
	}
	return ctx, nil
}

func (s *envValidateIntSteps) aFixtureAppAllowlisted(ctx context.Context) (context.Context, error) {
	dir, err := os.MkdirTemp("", "env-validate-fixture-*")
	if err != nil {
		return ctx, err
	}
	s.repoDir = dir
	if err := os.MkdirAll(filepath.Join(dir, ".git"), 0o755); err != nil {
		return ctx, err
	}
	if err := writeFixtureFile(dir, "apps/crud-be-golang-gin/main.go",
		`package main
import "os"
func main() {
	_ = os.Getenv("ENABLE_TEST_API")
	_ = os.Getenv("CRUD_BE_GOLANG_GIN_PORT")
}
`); err != nil {
		return ctx, err
	}
	return ctx, nil
}

func (s *envValidateIntSteps) theDeveloperRunsEnvValidate(ctx context.Context) (context.Context, error) {
	origGetwd := osGetwd
	origStat := osStat
	repoDir := s.repoDir
	osGetwd = func() (string, error) { return repoDir, nil }
	osStat = func(name string) (os.FileInfo, error) {
		if name == filepath.Join(repoDir, ".git") {
			return os.Stat(name)
		}
		return origStat(name)
	}
	defer func() {
		osGetwd = origGetwd
		osStat = origStat
	}()

	r, w, _ := os.Pipe()
	oldStdout := os.Stdout
	os.Stdout = w

	var buf strings.Builder
	done := make(chan struct{})
	go func() {
		tmp := make([]byte, 4096)
		for {
			n, err := r.Read(tmp)
			if n > 0 {
				buf.Write(tmp[:n])
			}
			if err != nil {
				break
			}
		}
		close(done)
	}()

	rootCmd := newRootCmd()
	rootCmd.SetArgs([]string{"env", "validate", "--no-color"})
	s.cmdErr = rootCmd.Execute()

	w.Close()
	os.Stdout = oldStdout
	<-done
	r.Close()
	s.cmdOutput = buf.String()

	return ctx, nil
}

func (s *envValidateIntSteps) theCommandExitsSuccessfully(_ context.Context) error {
	if s.cmdErr != nil {
		return s.cmdErr
	}
	return nil
}

func (s *envValidateIntSteps) theCommandExitsWithFailure(_ context.Context) error {
	if s.cmdErr == nil {
		return godog.ErrPending
	}
	return nil
}

func (s *envValidateIntSteps) theOutputNamesDeclaredButUnread(_ context.Context) error {
	if !strings.Contains(s.cmdOutput, "declared-but-unread") && !strings.Contains(s.cmdOutput, "FIXTURE_JWT_SECRET") {
		return godog.ErrPending
	}
	return nil
}

func (s *envValidateIntSteps) theOutputNamesReadButUndeclared(_ context.Context) error {
	if !strings.Contains(s.cmdOutput, "read-but-undeclared") && !strings.Contains(s.cmdOutput, "FIXTURE_JWT_SECRET") {
		return godog.ErrPending
	}
	return nil
}

func (s *envValidateIntSteps) theOutputReportsValidationPassed(_ context.Context) error {
	if !strings.Contains(s.cmdOutput, "passed") && s.cmdErr != nil {
		return s.cmdErr
	}
	return nil
}

func TestEnvValidateIntegration(t *testing.T) {
	s := &envValidateIntSteps{}
	suite := godog.TestSuite{
		Name:                 "env-validate",
		TestSuiteInitializer: func(sc *godog.TestSuiteContext) {},
		ScenarioInitializer: func(sc *godog.ScenarioContext) {
			sc.Before(s.before)
			sc.After(s.after)

			sc.Step(stepEnvValidateFixtureAppDeclaredButUnread, s.aFixtureAppDeclaredButNotRead)
			sc.Step(stepEnvValidateFixtureAppReadButUndeclared, s.aFixtureAppReadButNotDeclared)
			sc.Step(stepEnvValidateFixtureAppMatchingDeclaredRead, s.aFixtureAppMatchingDeclaredAndRead)
			sc.Step(stepEnvValidateFixtureAppAllowlisted, s.aFixtureAppAllowlisted)

			sc.Step(stepDeveloperRunsEnvValidate, s.theDeveloperRunsEnvValidate)

			sc.Step(stepExitsSuccessfully, s.theCommandExitsSuccessfully)
			sc.Step(stepExitsWithFailure, s.theCommandExitsWithFailure)
			sc.Step(stepOutputNamesDeclaredButUnread, s.theOutputNamesDeclaredButUnread)
			sc.Step(stepOutputNamesReadButUndeclared, s.theOutputNamesReadButUndeclared)
			sc.Step(stepOutputReportsValidationPassed, s.theOutputReportsValidationPassed)
		},
		Options: &godog.Options{
			Format: "pretty",
			Paths:  []string{specsIntEnvValidateDir + "/env/env-validate.feature"},
			Tags:   "env-validate",
		},
	}
	if suite.Run() != 0 {
		t.Fatal("integration scenarios failed")
	}
}
