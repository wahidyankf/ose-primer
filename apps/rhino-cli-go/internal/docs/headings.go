package docs

// Fence-aware ATX heading parsing and GFM anchor slugs.
//
// Shared by the link checker's anchor validation (`broken-anchor` findings)
// and the heading-hierarchy validator. Mirrors the Rust
// `internal/docs/headings.rs` counterpart.

import (
	"fmt"
	"regexp"
	"strings"
	"unicode"
)

// nonSlugChars matches characters removed by GFM slugging: everything OUTSIDE
// Unicode letters, Unicode digits, underscore, hyphen, and space. Mirrors
// `github-slugger` v2.
var nonSlugChars = regexp.MustCompile(`[^\p{L}\p{N}_\- ]`)

// ATXHeading represents a single ATX heading (`#` through `######`) found in
// markdown content.
type ATXHeading struct {
	Line  int    // 1-based line number where the heading appears.
	Level int    // Heading level (1-6).
	Title string // Heading title with trailing closing hashes stripped.
}

// CollectATXHeadings parses ATX headings from markdown content, ignoring
// lines inside fenced code blocks. Mirrors the Rust
// `internal/docs/headings.rs` counterpart.
//
// Lines inside fenced code blocks (``` or ~~~) are ignored; trailing closing
// hashes are stripped; up to three leading spaces are tolerated per
// CommonMark. Fence state is tracked with CommonMark close semantics (same
// char, >= opening length, no info string) via fenceTracker — see fences.go.
func CollectATXHeadings(content string) []ATXHeading {
	var headings []ATXHeading
	var fences fenceTracker

	for idx, line := range strings.Split(content, "\n") {
		line = strings.TrimSuffix(line, "\r")

		// Skip fence delimiter lines and fence content.
		if fences.observe(line) {
			continue
		}

		trimmed := strings.TrimLeftFunc(line, unicode.IsSpace)

		// CommonMark tolerates up to three leading spaces before the hashes.
		indent := len(line) - len(trimmed)
		if indent > 3 {
			continue
		}

		level := 0
		for level < len(trimmed) && trimmed[level] == '#' {
			level++
		}
		if level == 0 || level > 6 {
			continue
		}

		// The hash run must be followed by a space, a tab, or end of line
		// (`#5 bolt` is not a heading).
		rest := trimmed[level:]
		if rest != "" && !strings.HasPrefix(rest, " ") && !strings.HasPrefix(rest, "\t") {
			continue
		}

		headings = append(headings, ATXHeading{
			Line:  idx + 1,
			Level: level,
			Title: stripClosingHashes(strings.TrimSpace(rest)),
		})
	}

	return headings
}

// stripClosingHashes strips a trailing closing-hash run (`## Title ##` →
// `Title`). Per CommonMark the closing run only counts when preceded by
// whitespace (or when the title is nothing but hashes); `# C#` keeps its hash.
func stripClosingHashes(title string) string {
	stripped := strings.TrimRight(title, "#")
	if len(stripped) == len(title) {
		return title
	}
	if stripped == "" || strings.HasSuffix(stripped, " ") || strings.HasSuffix(stripped, "\t") {
		return strings.TrimRightFunc(stripped, unicode.IsSpace)
	}
	return title
}

// GFMSlug converts a single heading title to its GitHub (GFM) anchor slug.
//
// Strips inline markup (backticks), lowercases, removes every character
// outside `[\p{L}\p{N}_\- ]` (Unicode letters/digits, underscore, hyphen,
// space), and replaces each space with a hyphen without collapsing runs.
func GFMSlug(title string) string {
	text := strings.ReplaceAll(title, "`", "")
	lowered := strings.ToLower(text)
	return strings.ReplaceAll(nonSlugChars.ReplaceAllString(lowered, ""), " ", "-")
}

// CollectHeadingAnchors returns the anchor slug for every heading in content
// in document order, applying GitHub collision suffixes (-1, -2, ...) to
// duplicate slugs.
func CollectHeadingAnchors(content string) []string {
	seen := make(map[string]int)
	var anchors []string

	for _, heading := range CollectATXHeadings(content) {
		base := GFMSlug(heading.Title)
		count := seen[base]
		slug := base
		if count > 0 {
			slug = fmt.Sprintf("%s-%d", base, count)
		}
		seen[base] = count + 1
		anchors = append(anchors, slug)
	}

	return anchors
}
