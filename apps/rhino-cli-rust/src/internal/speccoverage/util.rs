/// Collapses internal whitespace runs and trims, so matching is
/// whitespace-insensitive. Mirrors Go `normalizeWS` (`strings.Fields` + join).
pub fn normalize_ws(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Returns `a` when non-empty, otherwise `b`. Mirrors Go `firstNonEmpty`.
pub fn first_non_empty<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.is_empty() { b } else { a }
}

/// Rewrites a regex pattern so Rust's `regex` crate accepts the same lenient
/// brace syntax that Go's RE2 (`regexp`) does, then compiles it.
///
/// # Why this exists
///
/// The Go spec-coverage extractors compile raw step-definition patterns with
/// `regexp.Compile`. RE2 treats `{` as a repetition operator **only** when it
/// is immediately followed by a valid count form (`{n}`, `{n,}`, or `{n,m}`);
/// any other `{` is a literal brace, and a lone `}` is always literal. Rust's
/// `regex` crate is stricter and returns a parse error for patterns such as
/// `{alice_id}` or `body { "x": 1 }`.
///
/// Spec-coverage marker patterns (e.g. Kotlin `SpecCoverageMarkers.kt`) embed
/// literal JSON braces inside `^…$`-anchored regexes precisely because RE2
/// tolerates them. Compiling those raw under Rust silently dropped the pattern
/// (`if let Ok(re)`), producing phantom step gaps. This helper escapes every
/// `{` that does not begin a valid RE2 quantifier so the compiled regex matches
/// the same input strings as RE2 — a byte-for-byte parity fix, not a heuristic.
///
/// Character classes (`[...]`) and already-escaped braces (`\{`) are preserved
/// verbatim. Iterates by `char` for UTF-8 safety.
pub fn escape_re2_literal_braces(pattern: &str) -> String {
    let chars: Vec<char> = pattern.chars().collect();
    let mut out = String::with_capacity(pattern.len() + 8);
    let mut i = 0usize;
    let mut in_class = false;
    while i < chars.len() {
        let c = chars[i];
        if c == '\\' && i + 1 < chars.len() {
            // Copy the escape and its target verbatim.
            out.push(c);
            out.push(chars[i + 1]);
            i += 2;
            continue;
        }
        if in_class {
            if c == ']' {
                in_class = false;
            }
            out.push(c);
            i += 1;
            continue;
        }
        if c == '[' {
            in_class = true;
            out.push(c);
            i += 1;
            continue;
        }
        if c == '{' {
            if is_valid_quantifier(&chars, i) {
                out.push(c);
            } else {
                // Literal brace in RE2 — escape so Rust accepts it.
                out.push('\\');
                out.push('{');
            }
            i += 1;
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

/// Returns true when `chars[start]` (which must be `{`) begins a valid RE2
/// repetition operator: `{n}`, `{n,}`, or `{n,m}` with decimal `n`/`m`.
fn is_valid_quantifier(chars: &[char], start: usize) -> bool {
    debug_assert_eq!(chars.get(start), Some(&'{'));
    let mut j = start + 1;
    let digits_start = j;
    while j < chars.len() && chars[j].is_ascii_digit() {
        j += 1;
    }
    if j == digits_start {
        // No leading digits → not a quantifier (e.g. `{,3}`, `{alice}`).
        return false;
    }
    if j < chars.len() && chars[j] == ',' {
        j += 1;
        while j < chars.len() && chars[j].is_ascii_digit() {
            j += 1;
        }
    }
    j < chars.len() && chars[j] == '}'
}

/// JS/TS-style escape sequence handling: `\'`, `\"`, `\\`, `\/`, `\n`, `\t`,
/// `\r`. Unknown escapes keep both the backslash and the following char.
/// Mirrors Go `unescapeString`. UTF-8 safe — iterates by chars, not bytes.
pub fn unescape_string(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            match chars[i + 1] {
                '\'' => out.push('\''),
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                '/' => out.push('/'),
                'n' => out.push('\n'),
                't' => out.push('\t'),
                'r' => out.push('\r'),
                other => {
                    out.push('\\');
                    out.push(other);
                }
            }
            i += 2;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_ws_collapses_runs_and_trims() {
        assert_eq!(normalize_ws("  a  b\t c\n"), "a b c");
    }

    #[test]
    fn first_non_empty_returns_a_when_present() {
        assert_eq!(first_non_empty("a", "b"), "a");
        assert_eq!(first_non_empty("", "b"), "b");
        assert_eq!(first_non_empty("", ""), "");
    }

    #[test]
    fn unescape_string_handles_common_sequences() {
        assert_eq!(unescape_string(r#"a\"b"#), "a\"b");
        assert_eq!(unescape_string(r"a\\b"), "a\\b");
        assert_eq!(unescape_string(r"a\nb"), "a\nb");
        assert_eq!(unescape_string(r"a\tb"), "a\tb");
        assert_eq!(unescape_string(r"a\/b"), "a/b");
    }

    #[test]
    fn unescape_string_unknown_escape_keeps_backslash() {
        assert_eq!(unescape_string(r"a\xb"), r"a\xb");
    }

    #[test]
    fn escape_re2_literal_braces_escapes_literal_open_brace() {
        assert_eq!(escape_re2_literal_braces("{alice_id}"), r"\{alice_id}");
        assert_eq!(escape_re2_literal_braces("a{b}c"), r"a\{b}c");
    }

    #[test]
    fn escape_re2_literal_braces_preserves_valid_quantifiers() {
        assert_eq!(escape_re2_literal_braces("x{2}"), "x{2}");
        assert_eq!(escape_re2_literal_braces("x{2,3}"), "x{2,3}");
        assert_eq!(escape_re2_literal_braces("x{2,}"), "x{2,}");
    }

    #[test]
    fn escape_re2_literal_braces_treats_invalid_quantifiers_as_literal() {
        assert_eq!(escape_re2_literal_braces("x{,3}"), r"x\{,3}");
        assert_eq!(escape_re2_literal_braces("x{}"), r"x\{}");
        assert_eq!(escape_re2_literal_braces("x{2a}"), r"x\{2a}");
    }

    #[test]
    fn escape_re2_literal_braces_skips_char_classes_and_escapes() {
        // Inside a character class, RE2 and Rust both treat `{` literally.
        assert_eq!(escape_re2_literal_braces("[{}]"), "[{}]");
        // Already-escaped braces are left untouched.
        assert_eq!(escape_re2_literal_braces(r"\{alice\}"), r"\{alice\}");
    }

    #[test]
    fn escape_re2_literal_braces_compiles_marker_style_pattern() {
        // The exact shape that appears in Kotlin SpecCoverageMarkers.kt.
        let raw = r#"^the admin sends POST /api/v1/admin/users/{alice_id}/disable with body { "reason": "([^"]+)" }$"#;
        let rewritten = escape_re2_literal_braces(raw);
        let re = regex::Regex::new(&rewritten).expect("rewritten pattern must compile");
        assert!(re.is_match(
            r#"the admin sends POST /api/v1/admin/users/{alice_id}/disable with body { "reason": "Policy violation" }"#
        ));
    }
}
