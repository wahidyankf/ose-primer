package docs

import (
	"bufio"
	"io/fs"
	"os"
	"path/filepath"
	"regexp"
	"strings"

	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/fileutil"
)

var (
	// linkRegex matches markdown links: [text](url).
	linkRegex = regexp.MustCompile(`\[([^\]]+)\]\(([^)]+)\)`)
)

// GetMarkdownFiles returns a list of markdown files to scan based on options.
func GetMarkdownFiles(opts ScanOptions) ([]string, error) {
	var files []string
	var err error

	if opts.StagedOnly {
		files, err = getStagedMarkdownFiles(opts.RepoRoot)
	} else {
		files, err = GetAllMarkdownFiles(opts.RepoRoot)
	}

	if err != nil {
		return nil, err
	}

	// Filter out skip paths
	return FilterSkipPaths(files, opts.RepoRoot, opts.SkipPaths), nil
}

// FilterSkipPaths filters out files whose repo-root-relative path starts with
// any of the skip paths (raw or trailing-slash-cleaned prefix). Exported as
// the single path-shaped prefix implementation per CLI, consumed by the links
// gate (GetMarkdownFiles) and the mermaid cmd (filterMermaidExcluded); the
// heading-hierarchy validator applies the same `--exclude` semantics via its
// deliberately self-contained string predicate isHeadingExcluded (see the
// divergence note in heading_hierarchy.go). Plan DD-2; mirrors Rust
// `scanner::filter_skip_paths`.
func FilterSkipPaths(files []string, repoRoot string, skipPaths []string) []string {
	if len(skipPaths) == 0 {
		return files
	}

	var filtered []string
	for _, file := range files {
		relPath, err := filepath.Rel(repoRoot, file)
		if err != nil {
			// If we can't get relative path, keep the file
			filtered = append(filtered, file)
			continue
		}

		skip := false
		for _, skipPath := range skipPaths {
			// Check if file is under skip path
			if strings.HasPrefix(relPath, skipPath) || strings.HasPrefix(relPath, filepath.Clean(skipPath)) {
				skip = true
				break
			}
		}

		if !skip {
			filtered = append(filtered, file)
		}
	}

	return filtered
}

// getStagedMarkdownFiles returns staged markdown files from git.
func getStagedMarkdownFiles(repoRoot string) ([]string, error) {
	return fileutil.GetStagedFilesFiltered(repoRoot, func(f string) bool {
		return strings.HasSuffix(f, ".md")
	})
}

// noiseDirs is the standardized cross-repo noise-skip set: directory NAMES
// dropped from the repo-wide walk wherever they appear, plus `.git`.
// Identical across the three aligned repos (ose-public / ose-infra /
// ose-primer). Mirrors the Rust `NOISE_DIRS` constant.
var noiseDirs = map[string]bool{
	"node_modules":        true,
	"dist":                true,
	"target":              true,
	".next":               true,
	"coverage":            true,
	"generated-reports":   true,
	"local-temp":          true,
	"archived":            true,
	"apps-labs":           true,
	"worktrees":           true,
	".terraform":          true,
	"generated-contracts": true,
	".nx":                 true,
	".git":                true,
}

// IsNoiseDir reports whether name is in the standardized noise-skip set.
// Exposed for the staged markdown gates (git pre-commit step 6m), which
// filter staged path strings per segment instead of walking the tree.
// Mirrors the Rust `NOISE_DIRS.contains` usage in the git runner.
func IsNoiseDir(name string) bool {
	return noiseDirs[name]
}

// GetAllMarkdownFiles returns all markdown files via a repo-wide walk that
// skips the standardized noise-skip set by directory name. The walk root
// itself is never skipped, only descendants — a `.md` file passed as the root
// yields itself. filepath.WalkDir yields deterministic lexical order (mirrors
// Rust WalkDir.sort_by_file_name). Exported as the single noise-skipping walk
// definition per CLI, shared by all three markdown gates: the links gate
// (GetMarkdownFiles), the heading-hierarchy validator
// (collectHeadingCandidateRels), and the mermaid cmd (collectMDFiles /
// collectMDDefaultDirs). Plan DD-3; mirrors Rust
// `scanner::get_all_markdown_files`.
func GetAllMarkdownFiles(repoRoot string) ([]string, error) {
	var files []string
	err := filepath.WalkDir(repoRoot, func(path string, d fs.DirEntry, walkErr error) error {
		if walkErr != nil {
			// Skip unreadable entries (mirrors Rust filter_map(Result::ok)).
			return nil //nolint:nilerr // intentional: unreadable entries are skipped, not fatal
		}
		if d.IsDir() {
			if path != repoRoot && noiseDirs[d.Name()] {
				return fs.SkipDir
			}
			return nil
		}
		if strings.HasSuffix(path, ".md") {
			files = append(files, path)
		}
		return nil
	})
	if err != nil {
		return nil, err
	}
	return files, nil
}

// ExtractLinks extracts markdown links from a file with line numbers.
func ExtractLinks(filePath string) ([]LinkInfo, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}
	defer func() { _ = file.Close() }()

	var links []LinkInfo
	scanner := bufio.NewScanner(file)
	lineNumber := 0
	var fences fenceTracker

	for scanner.Scan() {
		lineNumber++
		line := scanner.Text()

		// Skip fence delimiter lines and fence content. Fence state uses
		// CommonMark close semantics (same char, >= opening length, no
		// info string) and recognises both ``` and ~~~ fences — aligned
		// with CollectATXHeadings (headings.go) via fenceTracker
		// (fences.go). This deliberately replaces the historical ```-only
		// naive toggle; the Rust twin carries the identical change.
		if fences.observe(line) {
			continue
		}

		// Find all markdown links in the line
		matches := linkRegex.FindAllStringSubmatch(line, -1)
		for _, match := range matches {
			if len(match) < 3 {
				continue
			}
			url := match[2]

			// Strip angle brackets if present (markdown autolink syntax)
			url = strings.Trim(url, "<>")

			// Skip external URLs and mailto. Pure-anchor links (`#fragment`)
			// ARE extracted so same-file anchors reach validation.
			if strings.HasPrefix(url, "http://") ||
				strings.HasPrefix(url, "https://") ||
				strings.HasPrefix(url, "mailto:") {
				continue
			}

			// Skip placeholder/example/absolute paths
			if ShouldSkipLink(url) {
				continue
			}

			links = append(links, LinkInfo{
				LineNumber: lineNumber,
				URL:        url,
				IsRelative: !strings.HasPrefix(url, "/"),
			})
		}
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return links, nil
}

// ShouldSkipLink determines if a link should be skipped during validation.
func ShouldSkipLink(link string) bool {
	// Skip absolute paths
	if strings.HasPrefix(link, "/") {
		return true
	}

	// Skip shortcodes
	if strings.Contains(link, "{{<") || strings.Contains(link, "{{%") {
		return true
	}

	// Skip obvious placeholder patterns
	placeholders := []string{
		"path.md", "target", "link",
		"./path/to/", "../path/to/",
		"path/to/convention.md", "path/to/practice.md",
		"path/to/rule.md", "./relative/path/to/",
	}
	for _, placeholder := range placeholders {
		if strings.Contains(link, placeholder) {
			return true
		}
	}

	// Skip links with template placeholders in square brackets
	if regexp.MustCompile(`\[[\w-]+\]`).MatchString(link) {
		return true
	}

	// Skip links that are just "path", "target", or "link"
	if link == "path" || link == "target" || link == "link" {
		return true
	}

	// Skip example image paths
	if strings.Contains(link, "/images/") && !strings.HasPrefix(link, "../") {
		return true
	}

	// Skip example file names (clearly examples, not real links)
	examplePatterns := []string{
		"./overview", "./guide.md", "./examples.md", "./reference.md",
		"./diagram.png", "./image.png", "./screenshots/",
		"./auth-guide.md", "by-concept/beginner", "./by-example/beginner",
		"swe/prog-lang/", "../parent", "./ai/", "../swe/", "../../advanced/",
		"url", "./LICENSE", "../../features.md",
		"../../.opencode/", // OpenCode references (not part of this repo)
	}
	for _, pattern := range examplePatterns {
		if strings.Contains(link, pattern) {
			return true
		}
	}

	return false
}
