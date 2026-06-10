package envvalidate_test

import (
	"testing"

	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/envvalidate"
)

func TestExtractReadKeysSkipsTestFiles(t *testing.T) {
	root := t.TempDir()
	// Write a test file that would produce a key — it should be skipped
	writeFile(t, root, "apps/fixture-app/src/config_test.go", `os.Getenv("SHOULD_NOT_APPEAR")`)
	// Write a normal file
	writeFile(t, root, "apps/fixture-app/src/config.go", `os.Getenv("CRUD_BE_GOLANG_GIN_JWT_SECRET")`)

	surface := &envvalidate.AppSurface{
		App:          "fixture-app",
		SourceExts:   []string{"go"},
		SourceSubdir: "src",
		Allowlist:    []string{},
	}
	result, err := envvalidate.ValidateSurface(root, surface)
	if err != nil {
		t.Fatal(err)
	}
	// CRUD_BE_GOLANG_GIN_JWT_SECRET is read but not declared — should appear
	found := false
	for _, k := range result.ReadNotDeclared {
		if k == "CRUD_BE_GOLANG_GIN_JWT_SECRET" {
			found = true
		}
		if k == "SHOULD_NOT_APPEAR" {
			t.Error("test file content should be skipped")
		}
	}
	if !found {
		t.Errorf("expected CRUD_BE_GOLANG_GIN_JWT_SECRET in read_not_declared, got %v", result.ReadNotDeclared)
	}
}

func TestExtractReadKeysSkipsNodeModules(t *testing.T) {
	root := t.TempDir()
	// Write a file inside node_modules — should be skipped
	writeFile(t, root, "apps/fixture-app/src/node_modules/lib/config.ts", `process.env.SHOULD_NOT_APPEAR`)
	// Normal file
	writeFile(t, root, "apps/fixture-app/src/config.ts", `const s = process.env.FIXTURE_KEY;`)

	surface := &envvalidate.AppSurface{
		App:          "fixture-app",
		SourceExts:   []string{"ts"},
		SourceSubdir: "src",
		Allowlist:    []string{},
	}
	result, err := envvalidate.ValidateSurface(root, surface)
	if err != nil {
		t.Fatal(err)
	}
	for _, k := range result.ReadNotDeclared {
		if k == "SHOULD_NOT_APPEAR" {
			t.Error("node_modules content should be skipped")
		}
	}
}

func TestExtractPythonEmptyClass(t *testing.T) {
	// Python class with no BaseSettings should produce nothing from class body
	src := `
class Config:
    debug: bool = True
`
	got := envvalidate.ExtractPython(src)
	for _, k := range got {
		if k == "DEBUG" {
			t.Error("expected non-BaseSettings class to be skipped")
		}
	}
}

func TestExtractGoEmptyOnNoMatches(t *testing.T) {
	got := envvalidate.ExtractGo("package main\nfunc main() {}")
	if len(got) != 0 {
		t.Errorf("expected empty, got %v", got)
	}
}

func TestExtractTypeScriptEmptyOnNoMatches(t *testing.T) {
	got := envvalidate.ExtractTypeScript("const x = 1;")
	if len(got) != 0 {
		t.Errorf("expected empty, got %v", got)
	}
}

func TestExtractReadKeysAllLanguages(t *testing.T) {
	root := t.TempDir()
	writeFile(t, root, "apps/fixture-app/src/config.clj", `(getenv "CLJ_KEY")`)
	writeFile(t, root, "apps/fixture-app/src/Config.cs", `Environment.GetEnvironmentVariable("CS_KEY")`)
	writeFile(t, root, "apps/fixture-app/src/config.ex", `System.get_env("EX_KEY")`)
	writeFile(t, root, "apps/fixture-app/src/Config.fs", `Environment.GetEnvironmentVariable("FS_KEY")`)
	writeFile(t, root, "apps/fixture-app/src/Config.java", `String v = System.getenv("JAVA_KEY");`)
	writeFile(t, root, "apps/fixture-app/src/application.yml", "datasource:\n  url: ${YML_KEY}\n")
	writeFile(t, root, "apps/fixture-app/src/Config.kt", `val v = System.getenv("KT_KEY")`)
	writeFile(t, root, "apps/fixture-app/src/config.py", "import os\nresult = os.getenv(\"PY_KEY\")\n")

	surface := &envvalidate.AppSurface{
		App:          "fixture-app",
		SourceExts:   []string{"clj", "cs", "ex", "fs", "java", "yml", "kt", "py"},
		SourceSubdir: "src",
		Allowlist:    []string{},
	}
	result, err := envvalidate.ValidateSurface(root, surface)
	if err != nil {
		t.Fatal(err)
	}
	want := []string{"CLJ_KEY", "CS_KEY", "EX_KEY", "FS_KEY", "JAVA_KEY", "KT_KEY", "PY_KEY"}
	readSet := map[string]bool{}
	for _, k := range result.ReadNotDeclared {
		readSet[k] = true
	}
	for _, k := range want {
		if !readSet[k] {
			t.Errorf("expected %s in read_not_declared, got %v", k, result.ReadNotDeclared)
		}
	}
}
