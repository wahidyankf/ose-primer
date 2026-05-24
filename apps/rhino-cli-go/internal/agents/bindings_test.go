package agents

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// writeExpectedBindings writes every ExpectedBindings entry into root with
// its canonical content, creating parent directories as needed.
func writeExpectedBindings(t *testing.T, root string) {
	t.Helper()
	for _, bf := range ExpectedBindings() {
		target := filepath.Join(root, filepath.FromSlash(bf.Path))
		if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
			t.Fatalf("failed to create dir for %s: %v", bf.Path, err)
		}
		if err := os.WriteFile(target, []byte(bf.Content), 0o644); err != nil {
			t.Fatalf("failed to write %s: %v", bf.Path, err)
		}
	}
}

// writeCatalog writes a platform-bindings.md catalog file containing the
// provided text at docs/reference/platform-bindings.md under root.
func writeCatalog(t *testing.T, root, text string) {
	t.Helper()
	catalogPath := filepath.Join(root, filepath.FromSlash("docs/reference/platform-bindings.md"))
	if err := os.MkdirAll(filepath.Dir(catalogPath), 0o755); err != nil {
		t.Fatalf("failed to create catalog dir: %v", err)
	}
	if err := os.WriteFile(catalogPath, []byte(text), 0o644); err != nil {
		t.Fatalf("failed to write catalog: %v", err)
	}
}

func TestEmitBindings_DryRunFormatting(t *testing.T) {
	root := t.TempDir()

	out, err := EmitBindings(root, true)
	if err != nil {
		t.Fatalf("EmitBindings dry-run returned error: %v", err)
	}

	want := "would write .amazonq/cli-agents/ose-default.json\n" +
		"would write .amazonq/rules/00-agents-md.md\n" +
		"emit-bindings: would write 2 binding file(s) (dry-run)\n"

	if out != want {
		t.Errorf("dry-run output mismatch:\n got:\n%q\nwant:\n%q", out, want)
	}

	// Dry-run must not write any files.
	for _, bf := range ExpectedBindings() {
		target := filepath.Join(root, filepath.FromSlash(bf.Path))
		if _, err := os.Stat(target); !os.IsNotExist(err) {
			t.Errorf("dry-run unexpectedly wrote %s (err=%v)", bf.Path, err)
		}
	}
}

func TestEmitBindings_WritesFiles(t *testing.T) {
	root := t.TempDir()

	out, err := EmitBindings(root, false)
	if err != nil {
		t.Fatalf("EmitBindings returned error: %v", err)
	}

	want := "wrote .amazonq/cli-agents/ose-default.json\n" +
		"wrote .amazonq/rules/00-agents-md.md\n" +
		"emit-bindings: wrote 2 binding file(s)\n"

	if out != want {
		t.Errorf("write output mismatch:\n got:\n%q\nwant:\n%q", out, want)
	}

	for _, bf := range ExpectedBindings() {
		target := filepath.Join(root, filepath.FromSlash(bf.Path))
		got, readErr := os.ReadFile(target)
		if readErr != nil {
			t.Fatalf("expected file %s to exist: %v", bf.Path, readErr)
		}
		if string(got) != bf.Content {
			t.Errorf("content mismatch for %s:\n got:\n%q\nwant:\n%q", bf.Path, string(got), bf.Content)
		}
	}
}

func TestEmitBindings_MkdirError(t *testing.T) {
	root := t.TempDir()

	// Place a regular file where EmitBindings must create the .amazonq
	// directory. os.MkdirAll then fails because a path component is not a
	// directory, exercising the MkdirAll error branch.
	if err := os.WriteFile(filepath.Join(root, ".amazonq"), []byte("not a dir\n"), 0o644); err != nil {
		t.Fatalf("failed to seed blocking file: %v", err)
	}

	out, err := EmitBindings(root, false)
	if err == nil {
		t.Fatalf("expected error when target directory cannot be created, got nil; output:\n%s", out)
	}
	if out != "" {
		t.Errorf("expected empty output on error, got:\n%q", out)
	}
	if !strings.Contains(err.Error(), "failed to create directory") {
		t.Errorf("expected MkdirAll error, got: %v", err)
	}
}

func TestValidateBindings_ReportsDrift(t *testing.T) {
	root := t.TempDir()
	writeExpectedBindings(t, root)

	// Mutate the first expected file so its bytes no longer match.
	first := ExpectedBindings()[0]
	target := filepath.Join(root, filepath.FromSlash(first.Path))
	if err := os.WriteFile(target, []byte("mutated content\n"), 0o644); err != nil {
		t.Fatalf("failed to mutate %s: %v", first.Path, err)
	}

	// Catalog covers every existing dir; only .amazonq exists here (from the
	// written bindings), so document it to isolate the drift signal.
	writeCatalog(t, root, "covers .amazonq\n")

	out, err := ValidateBindings(root)
	if err == nil {
		t.Fatalf("expected error for drift, got nil; output:\n%s", out)
	}

	if !strings.Contains(out, "DRIFT "+first.Path+"\n") {
		t.Errorf("expected DRIFT line for %s, output:\n%s", first.Path, out)
	}
	if !strings.Contains(out, "binding-parity: 2 file(s) checked, 1 drift\n") {
		t.Errorf("expected binding-parity summary with 1 drift, output:\n%s", out)
	}
	if !strings.Contains(out, "VALIDATION FAILED: 1 problem(s)\n") {
		t.Errorf("expected VALIDATION FAILED with 1 problem, output:\n%s", out)
	}
}

func TestValidateBindings_ReportsMissingCatalog(t *testing.T) {
	root := t.TempDir()
	writeExpectedBindings(t, root)

	// Create an additional binding dir so two dirs exist on disk: .amazonq
	// (from the written bindings) and .claude.
	if err := os.MkdirAll(filepath.Join(root, ".claude"), 0o755); err != nil {
		t.Fatalf("failed to create .claude dir: %v", err)
	}

	// Catalog mentions .claude but NOT .amazonq, so .amazonq is missing.
	writeCatalog(t, root, "this catalog documents .claude only\n")

	out, err := ValidateBindings(root)
	if err == nil {
		t.Fatalf("expected error for missing catalog, got nil; output:\n%s", out)
	}

	if !strings.Contains(out, "MISSING-CATALOG .amazonq\n") {
		t.Errorf("expected MISSING-CATALOG .amazonq line, output:\n%s", out)
	}
	if !strings.Contains(out, "catalog-coverage: 2 dir(s) checked, 1 missing\n") {
		t.Errorf("expected catalog-coverage summary with 1 missing, output:\n%s", out)
	}
	if !strings.Contains(out, "VALIDATION FAILED: 1 problem(s)\n") {
		t.Errorf("expected VALIDATION FAILED with 1 problem, output:\n%s", out)
	}
}

func TestValidateBindings_Passes(t *testing.T) {
	root := t.TempDir()
	writeExpectedBindings(t, root)

	// Create every binding dir so all are checked, and a catalog that
	// documents each one.
	var sb strings.Builder
	for _, dir := range BindingDirsForCatalog() {
		if err := os.MkdirAll(filepath.Join(root, dir), 0o755); err != nil {
			t.Fatalf("failed to create dir %s: %v", dir, err)
		}
		sb.WriteString("documents ")
		sb.WriteString(dir)
		sb.WriteString("\n")
	}
	writeCatalog(t, root, sb.String())

	out, err := ValidateBindings(root)
	if err != nil {
		t.Fatalf("expected no error, got %v; output:\n%s", err, out)
	}

	want := "binding-parity: 2 file(s) checked, 0 drift\n" +
		"catalog-coverage: 5 dir(s) checked, 0 missing\n" +
		"VALIDATION PASSED\n"

	if out != want {
		t.Errorf("clean validate output mismatch:\n got:\n%q\nwant:\n%q", out, want)
	}
}
