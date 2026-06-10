package cmd

import (
	"bytes"
	"os"
	"path/filepath"
	"testing"

	"github.com/spf13/cobra"
)

// mockEnvValidateRoot sets up osGetwd/osStat to point at a synthetic git repo
// and returns a cleanup func.
func mockEnvValidateRoot(t *testing.T, root string) func() {
	t.Helper()
	origGetwd := osGetwd
	origStat := osStat
	osGetwd = func() (string, error) { return root, nil }
	osStat = func(name string) (os.FileInfo, error) {
		if name == filepath.Join(root, ".git") {
			// return a minimal FileInfo for the .git dir
			return os.Stat(filepath.Join(root, ".git"))
		}
		return origStat(name)
	}
	return func() {
		osGetwd = origGetwd
		osStat = origStat
	}
}

func setupEnvValidateRepo(t *testing.T) (root string, cleanup func()) {
	t.Helper()
	tmp := t.TempDir()
	if err := os.MkdirAll(filepath.Join(tmp, ".git"), 0o755); err != nil {
		t.Fatal(err)
	}
	restore := mockEnvValidateRoot(t, tmp)
	return tmp, restore
}

func writeEnvValidateFile(t *testing.T, root, rel, content string) {
	t.Helper()
	p := filepath.Join(root, rel)
	if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(p, []byte(content), 0o644); err != nil {
		t.Fatal(err)
	}
}

func TestEnvValidateCmdClean(t *testing.T) {
	root, cleanup := setupEnvValidateRepo(t)
	defer cleanup()

	// No app directories → all surfaces have empty declared + empty read → clean
	_ = root

	cmd := &cobra.Command{Use: "test"}
	cmd.Flags().String("output", "text", "output format")
	var buf bytes.Buffer
	cmd.SetOut(&buf)

	err := runEnvValidate(cmd, nil)
	if err != nil {
		t.Errorf("expected no error for empty repo, got: %v", err)
	}
}

func TestEnvValidateCmdCleanJSON(t *testing.T) {
	root, cleanup := setupEnvValidateRepo(t)
	defer cleanup()
	_ = root

	cmd := &cobra.Command{Use: "test"}
	cmd.Flags().String("output", "json", "output format")
	if err := cmd.Flags().Set("output", "json"); err != nil {
		t.Fatal(err)
	}
	var buf bytes.Buffer
	cmd.SetOut(&buf)

	err := runEnvValidate(cmd, nil)
	if err != nil {
		t.Errorf("expected no error, got: %v", err)
	}
}

func TestEnvValidateCmdNoGit(t *testing.T) {
	origGetwd := osGetwd
	osGetwd = func() (string, error) { return "/no-git-here", nil }
	origStat := osStat
	osStat = func(name string) (os.FileInfo, error) { return nil, os.ErrNotExist }
	defer func() {
		osGetwd = origGetwd
		osStat = origStat
	}()

	cmd := &cobra.Command{Use: "test"}
	cmd.Flags().String("output", "text", "output format")
	err := runEnvValidate(cmd, nil)
	if err == nil {
		t.Error("expected error when no git root found")
	}
}

func TestEnvValidateCmdViolations(t *testing.T) {
	root, cleanup := setupEnvValidateRepo(t)
	defer cleanup()

	// Declare a key for crud-be-clojure-pedestal but write no source files
	// → declared-but-unread violation
	writeEnvValidateFile(t, root, "infra/dev/crud-be-clojure-pedestal/.env.example",
		"CRUD_BE_CLOJURE_PEDESTAL_JWT_SECRET=dev-secret\n")

	cmd := &cobra.Command{Use: "test"}
	cmd.Flags().String("output", "text", "output format")
	var buf bytes.Buffer
	cmd.SetOut(&buf)

	err := runEnvValidate(cmd, nil)
	if err == nil {
		t.Error("expected error when violations detected")
	}
	if err != nil && err.Error() != "env validate found violations" {
		t.Errorf("unexpected error: %v", err)
	}
}
