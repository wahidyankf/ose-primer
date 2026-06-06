//! Fence-aware ATX heading parsing and GFM anchor slugs.
//!
//! Shared by the link checker's anchor validation (`broken-anchor` findings)
//! and the heading-hierarchy validator. Mirrors the planned Go
//! `internal/docs/headings.go` counterpart.

use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;

/// Characters removed by GFM slugging: everything OUTSIDE Unicode letters,
/// Unicode digits, underscore, hyphen, and space. Mirrors `github-slugger` v2.
static NON_SLUG_CHARS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[^\p{L}\p{N}_\- ]").expect("valid slug character class"));

/// Parses ATX headings (`#` through `######`) from markdown content.
///
/// Returns `(line_number, level, title)` tuples with 1-based line numbers.
/// Lines inside fenced code blocks (` ``` ` or `~~~`) are ignored; trailing
/// closing hashes are stripped; up to three leading spaces are tolerated per
/// CommonMark.
pub fn collect_atx_headings(content: &str) -> Vec<(usize, usize, String)> {
    let mut headings = Vec::new();
    let mut in_code_block = false;

    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim_start();

        // Fence toggle. Unlike `scanner::extract_links` (which mirrors the Go
        // scanner's ```-only check byte-for-byte), heading collection also
        // recognises `~~~` fences so anchors inside tilde blocks are ignored.
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            continue;
        }

        // CommonMark tolerates up to three leading spaces before the hashes.
        let indent = line.len() - trimmed.len();
        if indent > 3 {
            continue;
        }

        let level = trimmed.chars().take_while(|&c| c == '#').count();
        if level == 0 || level > 6 {
            continue;
        }

        // The hash run must be followed by a space, a tab, or end of line
        // (`#5 bolt` is not a heading).
        let rest = &trimmed[level..];
        if !(rest.is_empty() || rest.starts_with(' ') || rest.starts_with('\t')) {
            continue;
        }

        headings.push((idx + 1, level, strip_closing_hashes(rest.trim())));
    }

    headings
}

/// Strips a trailing closing-hash run (`## Title ##` → `Title`). Per
/// CommonMark the closing run only counts when preceded by whitespace (or
/// when the title is nothing but hashes); `# C#` keeps its hash.
fn strip_closing_hashes(title: &str) -> String {
    let stripped = title.trim_end_matches('#');
    if stripped.len() == title.len() {
        return title.to_string();
    }
    if stripped.is_empty() || stripped.ends_with(' ') || stripped.ends_with('\t') {
        stripped.trim_end().to_string()
    } else {
        title.to_string()
    }
}

/// Converts a single heading title to its GitHub (GFM) anchor slug.
///
/// Strips inline markup (backticks), lowercases, removes every character
/// outside `[\p{L}\p{N}_\- ]` (Unicode letters/digits, underscore, hyphen,
/// space), and replaces each space with a hyphen without collapsing runs.
pub fn gfm_slug(title: &str) -> String {
    let text = title.replace('`', "");
    let lowered = text.to_lowercase();
    NON_SLUG_CHARS.replace_all(&lowered, "").replace(' ', "-")
}

/// Returns the anchor slug for every heading in `content` in document order,
/// applying GitHub collision suffixes (`-1`, `-2`, ...) to duplicate slugs.
pub fn collect_heading_anchors(content: &str) -> Vec<String> {
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut anchors = Vec::new();

    for (_, _, title) in collect_atx_headings(content) {
        let base = gfm_slug(&title);
        let count = seen.entry(base.clone()).or_insert(0);
        let slug = if *count == 0 {
            base.clone()
        } else {
            format!("{base}-{count}")
        };
        *count += 1;
        anchors.push(slug);
    }

    anchors
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn gfm_slug_keeps_underscores() {
        assert_eq!(gfm_slug("snake_case naming"), "snake_case-naming");
    }

    #[test]
    fn gfm_slug_keeps_unicode_letters() {
        assert_eq!(gfm_slug("Café Über"), "café-über");
    }

    #[test]
    fn gfm_slug_does_not_collapse_double_spaces() {
        assert_eq!(gfm_slug("a  b"), "a--b");
    }

    #[test]
    fn gfm_slug_strips_backticks() {
        assert_eq!(gfm_slug("`code` block"), "code-block");
    }

    #[test]
    fn collect_heading_anchors_suffixes_duplicates() {
        let content = "## Setup\n\ntext\n\n## Setup\n";
        assert_eq!(
            collect_heading_anchors(content),
            vec!["setup".to_string(), "setup-1".to_string()]
        );
    }

    #[test]
    fn collect_atx_headings_ignores_fenced_code() {
        let content = "# Real\n\n```bash\n# not a heading\n```\n\n## Another\n";
        let headings = collect_atx_headings(content);
        assert_eq!(
            headings,
            vec![(1, 1, "Real".to_string()), (7, 2, "Another".to_string())]
        );
    }
}
