package docs

// Heading-hierarchy validation over prose markdown trees (Gate C).
//
// Greenfield validator (plan DD-7): reports `missing-h1`, `duplicate-h1`,
// and `skipped-level` findings. File selection is allowlist default-deny —
// only prose trees (`docs/`, `repo-governance/`, `specs/`, `plans/` minus
// `plans/done/`, root-level `*.md`, `apps|libs/<name>/README.md`, and
// `apps|libs/<name>/docs/**`) are ever scanned, so prompt/skill artifacts
// (`.claude/**`, `.opencode/**`, deep `apps/`/`libs/` internals) can never
// trip a finding, regardless of caller. Reuses fence-aware
// CollectATXHeadings so headings inside code fences are ignored. Mirrors the
// Rust `internal/docs/heading_hierarchy.rs` counterpart byte-for-byte.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// Heading-hierarchy finding kinds (stable strings used in reports).
const (
	// HeadingKindMissingH1 marks a file with zero H1 headings.
	HeadingKindMissingH1 = "missing-h1"
	// HeadingKindDuplicateH1 marks every H1 after the first in a file.
	HeadingKindDuplicateH1 = "duplicate-h1"
	// HeadingKindSkippedLevel marks a heading level jump of more than one.
	HeadingKindSkippedLevel = "skipped-level"
)

// HeadingFinding is a single heading-hierarchy finding.
type HeadingFinding struct {
	File    string // File containing the finding (relative to repo root).
	Line    int    // 1-based line number (1 for missing-h1).
	Kind    string // One of the HeadingKind* constants.
	Message string // Human-readable description.
}

// HeadingScanOptions configures a heading-hierarchy scan.
type HeadingScanOptions struct {
	// Root is the absolute path to the repository root.
	Root string
	// Paths are explicit repo-relative paths to scan instead of the full
	// allowlist walk (e.g. staged files). The allowlist predicate still
	// applies.
	Paths []string
	// Exclude lists repo-relative path prefixes subtracted on top of the
	// allowlist (`--exclude` semantics).
	Exclude []string
}

// IsProseAllowlisted returns true ONLY for prose-allowlisted repo-relative
// paths:
//
//   - docs/**, repo-governance/**, specs/**
//   - plans/** EXCEPT plans/done/**
//   - root-level *.md (no `/` in the repo-relative path)
//   - apps/<name>/README.md, libs/<name>/README.md
//   - apps/<name>/docs/**, libs/<name>/docs/**
//
// Everything else is default-denied — in particular `.claude/**`,
// `.opencode/**`, and deep `apps/`/`libs/` internals.
func IsProseAllowlisted(repoRel string) bool {
	// Whole prose trees.
	if strings.HasPrefix(repoRel, "docs/") ||
		strings.HasPrefix(repoRel, "repo-governance/") ||
		strings.HasPrefix(repoRel, "specs/") {
		return true
	}

	// plans/ minus the frozen plans/done/ archive.
	if strings.HasPrefix(repoRel, "plans/") {
		return !strings.HasPrefix(repoRel, "plans/done/")
	}

	// Root-level *.md (no `/` in the repo-relative path).
	if !strings.Contains(repoRel, "/") {
		return strings.HasSuffix(repoRel, ".md")
	}

	// apps/<name>/README.md, libs/<name>/README.md,
	// apps/<name>/docs/**, libs/<name>/docs/**.
	parts := strings.Split(repoRel, "/")
	if (parts[0] == "apps" || parts[0] == "libs") && len(parts) >= 3 {
		if len(parts) == 3 && parts[2] == "README.md" {
			return true
		}
		if len(parts) >= 4 && parts[2] == "docs" {
			return true
		}
	}

	return false
}

// isHeadingExcluded returns true when repoRel starts with any `--exclude`
// prefix, raw or trailing-slash-trimmed — the same practical semantics as the
// link scanner's filterSkipPaths. Deliberately a self-contained string
// predicate (mirrors the Rust `is_excluded`): the scanner's filter is
// path-shaped and kept byte-for-byte for the link checker, while this
// validator operates on repo-relative strings.
func isHeadingExcluded(repoRel string, exclude []string) bool {
	for _, prefix := range exclude {
		cleaned := strings.TrimRight(prefix, "/")
		if strings.HasPrefix(repoRel, prefix) ||
			(cleaned != "" && strings.HasPrefix(repoRel, cleaned)) {
			return true
		}
	}
	return false
}

// ValidateHeadingHierarchy scans markdown files under opts.Root (or
// opts.Paths when non-empty), keeping only IsProseAllowlisted survivors
// minus opts.Exclude prefixes, and returns all heading-hierarchy findings
// in discovery order.
func ValidateHeadingHierarchy(opts HeadingScanOptions) ([]HeadingFinding, error) {
	rels, err := collectHeadingCandidateRels(opts)
	if err != nil {
		return nil, err
	}
	var findings []HeadingFinding
	for _, rel := range rels {
		content, readErr := os.ReadFile(filepath.Join(opts.Root, rel))
		if readErr != nil {
			return nil, fmt.Errorf("failed to read %s: %w", rel, readErr)
		}
		findings = append(findings, validateHeadingContent(rel, string(content))...)
	}
	return findings, nil
}

// headingRelUnderRoot converts abs to a root-relative path, reporting false
// when abs is not under root (mirrors Rust `strip_prefix(...).ok()`).
func headingRelUnderRoot(root, abs string) (string, bool) {
	rel, err := filepath.Rel(root, abs)
	if err != nil || rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return "", false
	}
	return rel, true
}

// collectHeadingCandidateRels resolves the candidate repo-relative markdown
// paths: the full repo-wide walk (shared with the link scanner, noise dirs
// skipped) when opts.Paths is empty, otherwise the explicit paths
// (directories are walked). The allowlist and `--exclude` prefixes apply to
// BOTH modes.
func collectHeadingCandidateRels(opts HeadingScanOptions) ([]string, error) {
	var rels []string
	if len(opts.Paths) == 0 {
		files, err := getAllMarkdownFiles(opts.Root)
		if err != nil {
			return nil, err
		}
		for _, abs := range files {
			if rel, ok := headingRelUnderRoot(opts.Root, abs); ok {
				rels = append(rels, rel)
			}
		}
	} else {
		for _, p := range opts.Paths {
			abs := p
			if !filepath.IsAbs(p) {
				abs = filepath.Join(opts.Root, p)
			}
			if info, statErr := os.Stat(abs); statErr == nil && info.IsDir() {
				files, err := getAllMarkdownFiles(abs)
				if err != nil {
					return nil, err
				}
				for _, f := range files {
					if rel, ok := headingRelUnderRoot(opts.Root, f); ok {
						rels = append(rels, rel)
					}
				}
			} else if rel, ok := headingRelUnderRoot(opts.Root, abs); ok {
				rels = append(rels, rel)
			} else {
				rels = append(rels, p)
			}
		}
	}

	filtered := make([]string, 0, len(rels))
	for _, rel := range rels {
		if strings.HasSuffix(rel, ".md") &&
			IsProseAllowlisted(rel) &&
			!isHeadingExcluded(rel, opts.Exclude) {
			filtered = append(filtered, rel)
		}
	}
	return filtered, nil
}

// validateHeadingContent validates one file's heading sequence: zero H1s →
// missing-h1 (line 1), every H1 after the first → duplicate-h1, and any
// heading whose level increases by more than one over the previous heading →
// skipped-level. The first heading in a file never produces skipped-level on
// its own.
func validateHeadingContent(file, content string) []HeadingFinding {
	firstH1 := 0
	prevLevel := 0
	var perHeading []HeadingFinding

	for _, h := range CollectATXHeadings(content) {
		if h.Level == 1 {
			if firstH1 != 0 {
				perHeading = append(perHeading, HeadingFinding{
					File: file,
					Line: h.Line,
					Kind: HeadingKindDuplicateH1,
					Message: fmt.Sprintf("duplicate H1 \"%s\" (first H1 at line %d)",
						h.Title, firstH1),
				})
			} else {
				firstH1 = h.Line
			}
		}
		if prevLevel != 0 && h.Level > prevLevel+1 {
			perHeading = append(perHeading, HeadingFinding{
				File: file,
				Line: h.Line,
				Kind: HeadingKindSkippedLevel,
				Message: fmt.Sprintf("heading level jumps from H%d to H%d at \"%s\"",
					prevLevel, h.Level, h.Title),
			})
		}
		prevLevel = h.Level
	}

	var findings []HeadingFinding
	if firstH1 == 0 {
		findings = append(findings, HeadingFinding{
			File:    file,
			Line:    1,
			Kind:    HeadingKindMissingH1,
			Message: "file has no H1 heading",
		})
	}
	findings = append(findings, perHeading...)
	return findings
}

// ---------------------------------------------------------------------------
// Report formatting (mirrors the link reporter's text/json/markdown trio)
// ---------------------------------------------------------------------------

// FormatHeadingText formats findings as human-readable text. Findings are
// already in discovery order; consecutive findings for the same file share a
// section.
func FormatHeadingText(findings []HeadingFinding, quiet bool) string {
	var out strings.Builder

	if len(findings) == 0 {
		if !quiet {
			out.WriteString("✓ All heading hierarchies valid! No findings found.\n")
		}
		return out.String()
	}

	out.WriteString("# Heading Hierarchy Report\n\n")
	_, _ = fmt.Fprintf(&out, "**Total findings**: %d\n", len(findings))

	first := true
	currentFile := ""
	for _, finding := range findings {
		if first || currentFile != finding.File {
			_, _ = fmt.Fprintf(&out, "\n## %s\n\n", finding.File)
			currentFile = finding.File
			first = false
		}
		_, _ = fmt.Fprintf(&out, "- Line %d: %s: %s\n", finding.Line, finding.Kind, finding.Message)
	}

	return out.String()
}

// headingJSONOutput is the JSON output shape for `-o json`.
type headingJSONOutput struct {
	Status        string               `json:"status"`
	TotalFindings int                  `json:"total_findings"`
	Findings      []headingJSONFinding `json:"findings"`
}

// headingJSONFinding is the JSON finding shape.
type headingJSONFinding struct {
	File    string `json:"file"`
	Line    int    `json:"line"`
	Kind    string `json:"kind"`
	Message string `json:"message"`
}

// FormatHeadingJSON formats findings as JSON (Go-default HTML escaping, no
// timestamp — the Rust twin mirrors these bytes via its gojson helper).
func FormatHeadingJSON(findings []HeadingFinding) (string, error) {
	status := "success"
	if len(findings) > 0 {
		status = "failure"
	}
	jsonFindings := make([]headingJSONFinding, 0, len(findings))
	for _, f := range findings {
		jsonFindings = append(jsonFindings, headingJSONFinding(f))
	}
	out := headingJSONOutput{
		Status:        status,
		TotalFindings: len(findings),
		Findings:      jsonFindings,
	}
	bytes, err := json.MarshalIndent(out, "", "  ")
	if err != nil {
		return "", err
	}
	return string(bytes), nil
}

// FormatHeadingMarkdown formats findings as markdown. Intentionally delegates
// to FormatHeadingText — the text format is already markdown-compatible.
func FormatHeadingMarkdown(findings []HeadingFinding) string {
	return FormatHeadingText(findings, false)
}
