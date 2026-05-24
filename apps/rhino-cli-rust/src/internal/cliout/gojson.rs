//! Go-compatible JSON encoding helpers.
//!
//! Go's `encoding/json` HTML-escapes `<`, `>`, `&`, U+2028, and U+2029 in
//! string values by default (both `Marshal` and `MarshalIndent`). serde_json
//! does not, so to reach byte-identical parity with the Go CLI's JSON output we
//! post-process serde's output, applying the same escaping inside string
//! literals only.

/// Applies Go's default HTML escaping to a serde_json-produced document.
///
/// serde_json already escapes `"` and `\`, so every remaining unescaped `<`,
/// `>`, and `&` that survives into the output sits inside a string literal
/// (structural JSON never contains those bytes). U+2028/U+2029 are likewise
/// only present inside strings. We therefore replace them globally, matching
/// Go's `<` / `>` / `&` / ` ` / ` ` forms.
pub fn html_escape(json: &str) -> String {
    let mut out = String::with_capacity(json.len());
    for ch in json.chars() {
        match ch {
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn escapes_angle_brackets_and_amp() {
        // Input has literal <, >, &; output replaces them with Go's \uXXXX forms.
        let input = "{\"k\": \"a<b>c&d\"}";
        let expected = "{\"k\": \"a\\u003cb\\u003ec\\u0026d\"}";
        assert_eq!(html_escape(input), expected);
    }

    #[test]
    fn escapes_line_separators() {
        let input = "\"x\u{2028}y\u{2029}z\"";
        assert_eq!(html_escape(input), "\"x\\u2028y\\u2029z\"");
    }

    #[test]
    fn leaves_plain_json_untouched() {
        let s = "{\n  \"a\": 1,\n  \"b\": []\n}";
        assert_eq!(html_escape(s), s);
    }
}
