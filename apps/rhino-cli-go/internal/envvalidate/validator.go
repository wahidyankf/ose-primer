package envvalidate

import (
	"bufio"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

// ParseDeclared returns the set of env var keys declared in a .env.example file.
// Blank lines and comment lines are ignored.
func ParseDeclared(content string) map[string]struct{} {
	keys := make(map[string]struct{})
	scanner := bufio.NewScanner(strings.NewReader(content))
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}
		if eq := strings.Index(line, "="); eq > 0 {
			key := strings.TrimSpace(line[:eq])
			if key != "" {
				keys[key] = struct{}{}
			}
		}
	}
	return keys
}

// skipDirs are directories that should not be scanned for env reads.
var skipDirs = []string{
	"node_modules", "dist", ".next", "build", "bin", "obj",
	"_build", "generated-contracts", "test", "tests", "deps",
}

// shouldSkip returns true if the path should be excluded from scanning.
func shouldSkip(path string) bool {
	for _, dir := range skipDirs {
		if strings.Contains(path, "/"+dir+"/") || strings.Contains(path, string(filepath.Separator)+dir+string(filepath.Separator)) {
			return true
		}
	}
	// Skip Go test files and integration test files
	if strings.HasSuffix(path, "_test.go") || strings.Contains(path, ".integration_test.") {
		return true
	}
	return false
}

// ExtractReadKeys walks the source tree for an app surface and returns all env var keys read.
func ExtractReadKeys(sourceRoot string, surface *AppSurface) (map[string]struct{}, error) {
	keys := make(map[string]struct{})

	scanRoot := sourceRoot
	if surface.SourceSubdir != "" {
		scanRoot = filepath.Join(sourceRoot, surface.SourceSubdir)
	}

	if _, err := os.Stat(scanRoot); os.IsNotExist(err) {
		return keys, nil
	}

	extSet := make(map[string]struct{})
	for _, e := range surface.SourceExts {
		extSet[e] = struct{}{}
	}

	err := filepath.WalkDir(scanRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			base := d.Name()
			for _, skip := range skipDirs {
				if base == skip {
					return filepath.SkipDir
				}
			}
			return nil
		}
		if shouldSkip(path) {
			return nil
		}
		ext := strings.TrimPrefix(filepath.Ext(path), ".")
		if _, ok := extSet[ext]; !ok {
			return nil
		}

		data, readErr := os.ReadFile(path)
		if readErr != nil {
			return nil //nolint:nilerr // skip unreadable / binary files
		}
		content := string(data)
		isYAML := ext == "yml" || ext == "yaml"

		var extracted []string
		switch ext {
		case "rs":
			extracted = ExtractRust(content)
		case "go":
			extracted = ExtractGo(content)
		case "ts", "tsx":
			extracted = ExtractTypeScript(content)
		case "clj", "cljs":
			extracted = ExtractClojure(content)
		case "cs":
			extracted = ExtractCSharp(content)
		case "ex", "exs":
			extracted = ExtractElixir(content)
		case "fs", "fsx":
			extracted = ExtractFSharp(content)
		case "java":
			extracted = ExtractJava(content, false)
		case "yml", "yaml":
			extracted = ExtractJava(content, isYAML)
		case "kt":
			extracted = ExtractKotlin(content)
		case "py":
			extracted = ExtractPython(content)
		}
		for _, k := range extracted {
			keys[k] = struct{}{}
		}
		return nil
	})
	return keys, err
}

// ValidateSurface validates a single app surface and returns a SurfaceResult.
func ValidateSurface(repoRoot string, surface *AppSurface) (*SurfaceResult, error) {
	envExamplePath := filepath.Join(repoRoot, "infra", "dev", surface.App, ".env.example")

	var declared map[string]struct{}
	if data, err := os.ReadFile(envExamplePath); err == nil {
		declared = ParseDeclared(string(data))
	} else {
		declared = make(map[string]struct{})
	}

	sourceRoot := filepath.Join(repoRoot, "apps", surface.App)
	read, err := ExtractReadKeys(sourceRoot, surface)
	if err != nil {
		return nil, fmt.Errorf("extract read keys for %s: %w", surface.App, err)
	}

	// Build effective allowlist: global + per-app
	allowlist := make(map[string]struct{})
	for _, k := range GlobalAllowlist {
		allowlist[k] = struct{}{}
	}
	for _, k := range surface.Allowlist {
		allowlist[k] = struct{}{}
	}

	var declaredNotRead []string
	for k := range declared {
		if _, inRead := read[k]; !inRead {
			if _, inAllow := allowlist[k]; !inAllow {
				declaredNotRead = append(declaredNotRead, k)
			}
		}
	}

	var readNotDeclared []string
	for k := range read {
		if _, inDeclared := declared[k]; !inDeclared {
			if _, inAllow := allowlist[k]; !inAllow {
				readNotDeclared = append(readNotDeclared, k)
			}
		}
	}

	sort.Strings(declaredNotRead)
	sort.Strings(readNotDeclared)

	return &SurfaceResult{
		App:             surface.App,
		DeclaredNotRead: declaredNotRead,
		ReadNotDeclared: readNotDeclared,
	}, nil
}
