/// Collapses internal whitespace runs and trims, so matching is
/// whitespace-insensitive. Mirrors Go `normalizeWS` (`strings.Fields` + join).
pub fn normalize_ws(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Returns `a` when non-empty, otherwise `b`. Mirrors Go `firstNonEmpty`.
pub fn first_non_empty<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.is_empty() { b } else { a }
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
}
