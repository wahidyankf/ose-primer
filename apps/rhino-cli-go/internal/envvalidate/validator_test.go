package envvalidate_test

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/envvalidate"
)

func writeFile(t *testing.T, root, rel, content string) {
	t.Helper()
	p := filepath.Join(root, rel)
	if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(p, []byte(content), 0o644); err != nil {
		t.Fatal(err)
	}
}

func fixtureSurface(app string) *envvalidate.AppSurface {
	return &envvalidate.AppSurface{
		App:          app,
		SourceExts:   []string{"rs"},
		SourceSubdir: "src",
		Allowlist:    []string{"FIXTURE_PORT"},
	}
}

func TestParseDeclaredSkipsCommentsAndBlanks(t *testing.T) {
	content := "# comment\nFOO=bar\n\n# BAR=baz\nBAZ=qux\n"
	keys := envvalidate.ParseDeclared(content)
	if _, ok := keys["FOO"]; !ok {
		t.Error("expected FOO")
	}
	if _, ok := keys["BAZ"]; !ok {
		t.Error("expected BAZ")
	}
	if _, ok := keys["BAR"]; ok {
		t.Error("expected BAR to be absent (commented)")
	}
	if len(keys) != 2 {
		t.Errorf("expected 2 keys, got %d", len(keys))
	}
}

func TestDeclaredButUnread(t *testing.T) {
	root := t.TempDir()
	writeFile(t, root, "infra/dev/fixture-app/.env.example", "FIXTURE_JWT_SECRET=change-me\n")
	writeFile(t, root, "apps/fixture-app/src/config.rs", "fn main() { println!(\"no env reads here\"); }\n")

	result, err := envvalidate.ValidateSurface(root, fixtureSurface("fixture-app"))
	if err != nil {
		t.Fatal(err)
	}
	found := false
	for _, k := range result.DeclaredNotRead {
		if k == "FIXTURE_JWT_SECRET" {
			found = true
		}
	}
	if !found {
		t.Errorf("expected FIXTURE_JWT_SECRET in DeclaredNotRead, got %v", result.DeclaredNotRead)
	}
	if result.IsOK() {
		t.Error("expected result to be non-OK")
	}
}

func TestReadButUndeclared(t *testing.T) {
	root := t.TempDir()
	writeFile(t, root, "infra/dev/fixture-app/.env.example", "\n")
	writeFile(t, root, "apps/fixture-app/src/config.rs", `let s = env::var("FIXTURE_JWT_SECRET").unwrap();`)

	result, err := envvalidate.ValidateSurface(root, fixtureSurface("fixture-app"))
	if err != nil {
		t.Fatal(err)
	}
	found := false
	for _, k := range result.ReadNotDeclared {
		if k == "FIXTURE_JWT_SECRET" {
			found = true
		}
	}
	if !found {
		t.Errorf("expected FIXTURE_JWT_SECRET in ReadNotDeclared, got %v", result.ReadNotDeclared)
	}
	if result.IsOK() {
		t.Error("expected result to be non-OK")
	}
}

func TestMatchingDeclaredAndReadExitsOK(t *testing.T) {
	root := t.TempDir()
	writeFile(t, root, "infra/dev/fixture-app/.env.example", "FIXTURE_JWT_SECRET=change-me\n")
	writeFile(t, root, "apps/fixture-app/src/config.rs", `let s = env::var("FIXTURE_JWT_SECRET").context("required")?;`)

	result, err := envvalidate.ValidateSurface(root, fixtureSurface("fixture-app"))
	if err != nil {
		t.Fatal(err)
	}
	if !result.IsOK() {
		t.Errorf("expected result to be OK, got declared_not_read=%v read_not_declared=%v",
			result.DeclaredNotRead, result.ReadNotDeclared)
	}
}

func TestAllowlistedKeysIgnored(t *testing.T) {
	root := t.TempDir()
	writeFile(t, root, "infra/dev/fixture-app/.env.example", "FIXTURE_JWT_SECRET=change-me\n")
	writeFile(t, root, "apps/fixture-app/src/config.rs",
		`let s = env::var("FIXTURE_JWT_SECRET").context("required")?;`+"\n"+
			`let t = env::var("ENABLE_TEST_API").unwrap_or_default();`+"\n"+
			`let p = env::var("FIXTURE_PORT").unwrap_or("8080".into());`,
	)

	result, err := envvalidate.ValidateSurface(root, fixtureSurface("fixture-app"))
	if err != nil {
		t.Fatal(err)
	}
	if !result.IsOK() {
		t.Errorf("expected OK (ENABLE_TEST_API and FIXTURE_PORT allowlisted), got declared_not_read=%v read_not_declared=%v",
			result.DeclaredNotRead, result.ReadNotDeclared)
	}
}

func TestMissingEnvExampleIsEmpty(t *testing.T) {
	root := t.TempDir()
	// No .env.example file; no source files either
	result, err := envvalidate.ValidateSurface(root, fixtureSurface("no-app"))
	if err != nil {
		t.Fatal(err)
	}
	if !result.IsOK() {
		t.Errorf("expected OK when both declared and read are empty, got %+v", result)
	}
}
