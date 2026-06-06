package docs

import (
	"os"
	"path/filepath"
	"strings"
	"time"
)

// ResolveLink resolves a relative link to an absolute path.
func ResolveLink(sourceFile, link, repoRoot string) string {
	// Remove anchor if present
	linkWithoutAnchor := strings.Split(link, "#")[0]

	// If link is empty after removing anchor (pure anchor link), return source file
	if linkWithoutAnchor == "" {
		return sourceFile
	}

	// Resolve relative to source file's directory
	sourceDir := filepath.Dir(sourceFile)
	targetPath := filepath.Join(sourceDir, linkWithoutAnchor)
	resolved := filepath.Clean(targetPath)

	return resolved
}

// ValidateLink checks if a link's target exists.
func ValidateLink(sourceFile, link, repoRoot string) (bool, error) {
	targetPath := ResolveLink(sourceFile, link, repoRoot)

	// Check if target exists
	_, err := os.Stat(targetPath)
	if os.IsNotExist(err) {
		return false, nil
	}
	if err != nil {
		return false, err
	}

	return true, nil
}

// anchorExists checks whether fragment matches a heading anchor in the
// target file (GFM slugs with `-N` collision suffixes, case-sensitive).
// Unreadable targets are treated as valid so an I/O hiccup never becomes a
// finding. Mirrors Rust `anchor_exists`.
func anchorExists(targetPath, fragment string) bool {
	content, err := os.ReadFile(targetPath)
	if err != nil {
		return true
	}
	for _, anchor := range CollectHeadingAnchors(string(content)) {
		if anchor == fragment {
			return true
		}
	}
	return false
}

// ValidateFile validates all links in a single file.
// Skill files (`.claude/skills/`) are skipped wholesale.
func ValidateFile(filePath string, opts ScanOptions) ([]BrokenLink, error) {
	// Skip validation for skill files as they contain many examples
	if strings.Contains(filePath, ".claude/skills/") {
		return nil, nil
	}

	links, err := ExtractLinks(filePath)
	if err != nil {
		return nil, err
	}

	var brokenLinks []BrokenLink

	// Get relative path for reporting
	relPath, err := filepath.Rel(opts.RepoRoot, filePath)
	if err != nil {
		relPath = filePath
	}

	for _, linkInfo := range links {
		// Capture the `#fragment` BEFORE resolution strips it.
		fragment := ""
		hasFragment := false
		if idx := strings.Index(linkInfo.URL, "#"); idx >= 0 {
			hasFragment = true
			fragment = linkInfo.URL[idx+1:]
		}

		valid, err := ValidateLink(filePath, linkInfo.URL, opts.RepoRoot)
		if err != nil {
			return nil, err
		}
		targetPath := ResolveLink(filePath, linkInfo.URL, opts.RepoRoot)

		if !valid {
			category := CategorizeBrokenLink(linkInfo.URL)

			brokenLinks = append(brokenLinks, BrokenLink{
				LineNumber: linkInfo.LineNumber,
				SourceFile: relPath,
				LinkText:   linkInfo.URL,
				TargetPath: targetPath,
				Category:   category,
			})
			continue
		}

		// Target exists — validate the fragment (if any) against the target
		// file's heading anchor set. A pure anchor (`#fragment`) resolves to
		// the source file, so same-file anchors are covered too.
		if !hasFragment || fragment == "" || !strings.HasSuffix(targetPath, ".md") {
			continue
		}
		if !anchorExists(targetPath, fragment) {
			brokenLinks = append(brokenLinks, BrokenLink{
				LineNumber: linkInfo.LineNumber,
				SourceFile: relPath,
				LinkText:   linkInfo.URL,
				TargetPath: targetPath,
				Category:   "broken-anchor",
			})
		}
	}

	return brokenLinks, nil
}

// ValidateAllLinks validates all markdown files based on options.
func ValidateAllLinks(opts ScanOptions) (*LinkValidationResult, error) {
	startTime := time.Now()

	files, err := GetMarkdownFiles(opts)
	if err != nil {
		return nil, err
	}

	result := &LinkValidationResult{
		TotalFiles:       len(files),
		TotalLinks:       0,
		BrokenLinks:      []BrokenLink{},
		BrokenByCategory: make(map[string][]BrokenLink),
	}

	for _, filePath := range files {
		links, err := ExtractLinks(filePath)
		if err != nil {
			// Skip files we can't read
			continue
		}
		result.TotalLinks += len(links)

		brokenLinks, err := ValidateFile(filePath, opts)
		if err != nil {
			// Skip files with validation errors
			continue
		}

		for _, broken := range brokenLinks {
			result.BrokenLinks = append(result.BrokenLinks, broken)
			result.BrokenByCategory[broken.Category] = append(
				result.BrokenByCategory[broken.Category],
				broken,
			)
		}
	}

	result.ScanDuration = time.Since(startTime)
	return result, nil
}
