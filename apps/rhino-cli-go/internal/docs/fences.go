package docs

// CommonMark fenced-code-block tracking shared by the markdown gates.
//
// A fence opens on a run of >= 3 backticks or tildes (any leading
// whitespace tolerated) and closes ONLY on a same-character run of at least
// the opening length with nothing but whitespace after it (closing fences
// carry no info string; any leading whitespace tolerated). A naive toggle
// desyncs on nested constructs (a ````markdown block containing ```json
// inner fences) and on mixed fence characters (``` inside ~~~). Consumed by
// CollectATXHeadings (headings.go) and ExtractLinks (links_scanner.go).
// Mirrors the Rust `internal/docs/fences.rs` counterpart.
//
// Deliberate CommonMark deviation: the spec tolerates at most three leading
// spaces on a fence, but Prettier indents fences inside nested list items
// by 4-7 spaces — legitimate fences relative to the list container, which
// this flat (container-unaware) tracker would otherwise treat as text,
// causing false link/heading extractions from fence bodies (e.g.
// repo-governance/workflows/plan/plan-execution.md:524). We therefore
// accept ANY leading whitespace on opening AND closing fences. Trade-off:
// 4-space-indented code blocks whose content starts with ``` or ~~~ would
// be misread as fences — acceptable because Prettier never emits indented
// code blocks in this repository.

import "strings"

// fenceTracker tracks fenced-code-block state across the lines of one
// markdown document. The zero value is ready to use.
type fenceTracker struct {
	inFence bool
	char    byte // Opening fence character: '`' or '~'.
	length  int  // Opening fence run length (>= 3).
}

// observe consumes one line (without its trailing newline) and reports
// whether the consumer must skip it: fence delimiter lines (opening and
// closing) and every line inside an open fence return true.
func (f *fenceTracker) observe(line string) bool {
	char, length, rest, isFence := parseFenceRun(line)

	if f.inFence {
		// Close only on a same-char run of >= the opening length with no
		// info string (rest of line is whitespace).
		if isFence && char == f.char && length >= f.length && strings.TrimSpace(rest) == "" {
			f.inFence = false
		}
		return true
	}

	if isFence {
		f.inFence = true
		f.char = char
		f.length = length
		return true
	}

	return false
}

// parseFenceRun reports whether line begins (after any leading whitespace;
// see the deliberate CommonMark deviation in the package comment above)
// with a fence run of >= 3 backticks or tildes. On success it returns the
// fence character, the run length, and the remainder of the line after the
// run.
func parseFenceRun(line string) (char byte, length int, rest string, ok bool) {
	start := 0
	for start < len(line) && (line[start] == ' ' || line[start] == '\t') {
		start++
	}
	if start >= len(line) || (line[start] != '`' && line[start] != '~') {
		return 0, 0, "", false
	}

	char = line[start]
	end := start
	for end < len(line) && line[end] == char {
		end++
	}
	length = end - start
	if length < 3 {
		return 0, 0, "", false
	}

	return char, length, line[end:], true
}
