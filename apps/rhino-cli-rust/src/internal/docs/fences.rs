//! CommonMark fenced-code-block tracking shared by the heading and link
//! parsers. Mirrors the Go `internal/docs/fences.go` counterpart.
//!
//! Per the [CommonMark spec](https://spec.commonmark.org/0.31.2/#fenced-code-blocks)
//! a fence opened with N fence characters (backticks or tildes, N >= 3) is
//! closed ONLY by a fence of the SAME character with a run length >= N and no
//! info string. A naive "any ``` line toggles state" scan desyncs on nested
//! example fences (e.g. a ````markdown block containing ``` fences) and
//! misreads fence content as document content.
//!
//! **Deliberate deviation from strict CommonMark**: the spec caps fence
//! indentation at three leading spaces; this tracker accepts ANY leading
//! whitespace for both opening and closing fences. Prettier formats fences
//! inside nested list items at 4-7 space indents, which are legitimate
//! fences relative to their list container — a flat line tracker that
//! enforced the three-space cap would treat them as text and extract false
//! links/headings from fence content. The trade-off (a 4-space indented code
//! block would be mis-tracked as a fence) is acceptable because
//! Prettier-formatted markdown in this repo never produces indented code
//! blocks.

/// Stateful line-by-line fence tracker.
#[derive(Debug, Default)]
pub struct FenceTracker {
    /// Open fence, if any: `(fence_char, run_length)`.
    open: Option<(char, usize)>,
}

impl FenceTracker {
    /// Creates a tracker with no open fence.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Feeds the next line. Returns `true` when the line belongs to fenced
    /// code (an opening/closing delimiter line or interior fence content) and
    /// should therefore be excluded from markdown parsing.
    pub fn consume_line(&mut self, line: &str) -> bool {
        match self.open {
            Some((fence_char, open_len)) => {
                if is_closing_fence(line, fence_char, open_len) {
                    self.open = None;
                }
                // Delimiter and interior lines are both fence content.
                true
            }
            None => {
                if let Some(opened) = parse_opening_fence(line) {
                    self.open = Some(opened);
                    true
                } else {
                    false
                }
            }
        }
    }
}

/// Splits a fence-candidate line into `(fence_char, run_length, rest)` when
/// it starts with a backtick/tilde run after any leading whitespace (a
/// deliberate relaxation of CommonMark's three-space cap; see module docs).
fn split_fence_run(line: &str) -> Option<(char, usize, &str)> {
    let after_indent = line.trim_start();
    let fence_char = after_indent
        .chars()
        .next()
        .filter(|c| matches!(c, '`' | '~'))?;
    let run = after_indent
        .chars()
        .take_while(|&c| c == fence_char)
        .count();
    Some((fence_char, run, &after_indent[run..]))
}

/// Returns `Some((fence_char, run_length))` when `line` opens a fence: a run
/// of >=3 backticks or tildes after any leading whitespace. Backtick info
/// strings must not contain backticks (CommonMark); tilde info strings are
/// unrestricted.
fn parse_opening_fence(line: &str) -> Option<(char, usize)> {
    let (fence_char, run, rest) = split_fence_run(line)?;
    if run < 3 {
        return None;
    }
    if fence_char == '`' && rest.contains('`') {
        return None;
    }
    Some((fence_char, run))
}

/// Returns `true` when `line` closes a fence opened with `open_len` chars of
/// `fence_char`: same character, run length >= `open_len`, nothing but
/// whitespace after the run, any leading whitespace allowed.
fn is_closing_fence(line: &str, fence_char: char, open_len: usize) -> bool {
    let Some((run_char, run, rest)) = split_fence_run(line) else {
        return false;
    };
    run_char == fence_char && run >= open_len && rest.trim().is_empty()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    /// Runs the tracker over `content` and returns, per line, whether the
    /// line was classified as fence content.
    fn classify(content: &str) -> Vec<bool> {
        let mut tracker = FenceTracker::new();
        content
            .lines()
            .map(|line| tracker.consume_line(line))
            .collect()
    }

    #[test]
    fn simple_backtick_fence_opens_and_closes() {
        assert_eq!(
            classify("text\n```\ncode\n```\nafter\n"),
            vec![false, true, true, true, false]
        );
    }

    #[test]
    fn longer_open_requires_equal_or_longer_close() {
        // ``` cannot close ````; the final ```` does.
        assert_eq!(
            classify("````\n```\ncode\n````\nafter\n"),
            vec![true, true, true, true, false]
        );
    }

    #[test]
    fn closing_fence_must_match_char() {
        assert_eq!(
            classify("~~~\n```\n~~~\nafter\n"),
            vec![true, true, true, false]
        );
        assert_eq!(
            classify("```\n~~~\n```\nafter\n"),
            vec![true, true, true, false]
        );
    }

    #[test]
    fn closing_fence_rejects_info_string() {
        // A line with an info string never closes a fence.
        assert_eq!(
            classify("```\n``` rust\n```\nafter\n"),
            vec![true, true, true, false]
        );
    }

    #[test]
    fn deep_indented_fence_inside_list_item_is_tracked() {
        // Prettier indents fences inside nested list items 4-7 spaces; they
        // are legitimate fences relative to their list container. A 5-space
        // opener must suppress link/heading extraction until its 5-space
        // closer.
        assert_eq!(
            classify("- item\n     ```text\n     [not a link](./x.md)\n     ```\nafter\n"),
            vec![false, true, true, true, false]
        );
    }

    #[test]
    fn four_space_indent_is_treated_as_fence() {
        // Deliberate deviation from strict CommonMark: 4+ space indents open
        // fences here because Prettier-formatted markdown in this repo never
        // produces indented code blocks.
        assert_eq!(
            classify("    ```\ncode\n    ```\nafter\n"),
            vec![true, true, true, false]
        );
    }

    #[test]
    fn up_to_three_leading_spaces_allowed() {
        assert_eq!(
            classify("   ```\ncode\n   ```\nafter\n"),
            vec![true, true, true, false]
        );
    }

    #[test]
    fn backtick_info_string_with_backtick_is_not_a_fence() {
        assert_eq!(classify("``` a`b\ntext\n"), vec![false, false]);
    }

    #[test]
    fn two_chars_do_not_open_a_fence() {
        assert_eq!(classify("``\n~~\n"), vec![false, false]);
    }

    #[test]
    fn unclosed_fence_swallows_rest_of_document() {
        assert_eq!(classify("```\ncode\nmore\n"), vec![true, true, true]);
    }
}
