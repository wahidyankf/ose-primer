//! Cucumber-expression to regex conversion, plus Python `parsers.parse` format support.
//!
//! Byte-for-byte port of `apps/rhino-cli/internal/speccoverage/cucumber_expr.go`.

use std::sync::OnceLock;

use regex::Regex;

/// Returns the lazily-compiled regex that matches a single Cucumber parameter
/// placeholder such as `{string}` or `{int}`.
fn cucumber_param_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\{[^}]+\}").expect("valid regex"))
}

/// Returns the lazily-compiled regex for Python `parsers.parse` format
/// placeholders such as `{name}` or `{n:d}`.
fn python_parsers_param_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\{(\w+)(?::([dgw]))?\}").expect("valid regex"))
}

/// Decodes Cucumber expression escape sequences in literal text.
///
/// The sequence `\X` becomes `X` for any character `X`, matching the
/// Cucumber specification that allows escaping `\(`, `\)`, `\{`, `\}`,
/// `\/`, and `\\`.
pub fn unescape_cucumber_expr(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            out.push(chars[i + 1]);
            i += 2;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

/// Maps a Cucumber parameter type name to the corresponding regex fragment.
///
/// Known types: `string`, `int`, `byte`, `short`, `long`, `float`, `double`,
/// `bigdecimal`, `word`. Any unrecognised name maps to `.+`.
pub fn cucumber_param_to_regex(param_name: &str) -> &'static str {
    match param_name {
        "string" => "\"[^\"]*\"",
        "int" | "byte" | "short" | "long" => r"-?\d+",
        "float" | "double" | "bigdecimal" => r"-?\d+\.?\d*",
        "word" => r"\S+",
        _ => ".+",
    }
}

/// Converts a full Cucumber expression string into a regex pattern string
/// (without anchors).
///
/// Each **unescaped** `{type}` placeholder is replaced by the regex from
/// [`cucumber_param_to_regex`]. An escaped brace pair (`\{…\}`) is *not* a
/// placeholder — per the Cucumber Expression spec, `\{` and `\}` denote a
/// literal `{`/`}` character, which is how a step is written when the
/// bound step-definition function takes no parameter for that segment (e.g.
/// a URL path template kept as literal text such as `\{expenseId\}`).
/// Literal text segments (including decoded escape sequences) are
/// regex-escaped before being appended.
pub fn cucumber_expr_to_regex(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut out = String::new();
    let mut literal = String::new();
    let mut i = 0usize;

    while i < n {
        let c = chars[i];
        if c == '\\' && i + 1 < n {
            // Escaped character: keep the escape pair intact in the literal
            // buffer; `unescape_cucumber_expr` decodes it on flush. This is
            // what makes `\{`/`\}` literal rather than a parameter boundary.
            literal.push(c);
            literal.push(chars[i + 1]);
            i += 2;
            continue;
        }
        if c == '{'
            && let Some(rel_close) = chars[i + 1..].iter().position(|&ch| ch == '}')
        {
            let close = i + 1 + rel_close;
            flush_literal(&mut out, &mut literal);
            let inner: String = chars[i + 1..close].iter().collect();
            out.push_str(cucumber_param_to_regex(&inner));
            i = close + 1;
            continue;
        }
        // If c == '{' but no matching '}' was found, fall through and treat '{' as literal.
        literal.push(c);
        i += 1;
    }
    flush_literal(&mut out, &mut literal);
    out
}

/// Decodes and regex-escapes any buffered literal text, appending it to
/// `out`, then clears the buffer.
fn flush_literal(out: &mut String, literal: &mut String) {
    if !literal.is_empty() {
        out.push_str(&regex::escape(&unescape_cucumber_expr(literal)));
        literal.clear();
    }
}

/// Returns `true` if `text` contains at least one Cucumber parameter
/// placeholder (e.g. `{string}`, `{int}`).
pub fn has_cucumber_expressions(text: &str) -> bool {
    cucumber_param_re().is_match(text)
}

/// Converts a Python `parsers.parse` format string into a regex pattern string
/// (without anchors).
///
/// Supported format specifiers: `d` (integer), `g` (float), `w` (word).
/// Plain `{name}` without a specifier maps to `.+`.
///
/// # Panics
///
/// Panics if the regex matches but capture groups are absent (indicates a bug
/// in the compiled regex, which cannot happen in practice).
pub fn convert_python_parsers_expr(text: &str) -> String {
    let re = python_parsers_param_re();
    let mut sb = String::new();
    let mut remaining = text;
    loop {
        match re.find(remaining) {
            None => {
                sb.push_str(&regex::escape(remaining));
                break;
            }
            Some(m) => {
                sb.push_str(&regex::escape(&remaining[..m.start()]));
                let caps = re
                    .captures(&remaining[m.start()..m.end()])
                    .expect("re.find matched so captures always succeeds");
                let format_spec = caps.get(2).map_or("", |x| x.as_str());
                let chunk = match format_spec {
                    "d" => r"-?\d+",
                    "g" => r"-?\d+\.?\d*",
                    "w" => r"\S+",
                    _ => ".+",
                };
                sb.push_str(chunk);
                remaining = &remaining[m.end()..];
            }
        }
    }
    sb
}

/// Returns `true` if `text` contains at least one Python `parsers.parse`
/// format placeholder (e.g. `{name}` or `{n:d}`).
pub fn is_python_parsers_expr(text: &str) -> bool {
    python_parsers_param_re().is_match(text)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn unescape_basic_escapes() {
        assert_eq!(unescape_cucumber_expr(r"\(foo\)"), "(foo)");
        assert_eq!(unescape_cucumber_expr(r"a\\b"), "a\\b");
        assert_eq!(unescape_cucumber_expr("no escapes"), "no escapes");
    }

    #[test]
    fn cucumber_param_string_maps_to_quoted() {
        assert_eq!(cucumber_param_to_regex("string"), "\"[^\"]*\"");
    }

    #[test]
    fn cucumber_param_int_maps_to_signed_digits() {
        assert_eq!(cucumber_param_to_regex("int"), r"-?\d+");
        assert_eq!(cucumber_param_to_regex("long"), r"-?\d+");
    }

    #[test]
    fn cucumber_param_float_maps_to_signed_decimal() {
        assert_eq!(cucumber_param_to_regex("float"), r"-?\d+\.?\d*");
    }

    #[test]
    fn cucumber_param_word_maps_to_non_whitespace() {
        assert_eq!(cucumber_param_to_regex("word"), r"\S+");
    }

    #[test]
    fn cucumber_param_unknown_maps_to_any() {
        assert_eq!(cucumber_param_to_regex("custom"), ".+");
    }

    #[test]
    fn cucumber_expr_to_regex_string_param() {
        let r = cucumber_expr_to_regex("user enters {string}");
        assert_eq!(r, "user enters \"[^\"]*\"");
    }

    #[test]
    fn cucumber_expr_to_regex_escapes_literals() {
        let r = cucumber_expr_to_regex("a (1.0) b");
        // ( and . are regex specials → must be escaped.
        assert!(r.contains("\\("));
        assert!(r.contains("\\."));
    }

    #[test]
    fn cucumber_expr_to_regex_handles_escape_then_param() {
        // \(foo\) is literal "(foo)", then {int}
        let r = cucumber_expr_to_regex(r"\(foo\) {int}");
        assert!(r.contains(r"\(foo\)") || r.contains("\\(foo\\)"));
        assert!(r.contains(r"-?\d+"));
    }

    #[test]
    fn has_cucumber_expressions_detects_braces() {
        assert!(has_cucumber_expressions("user enters {string}"));
        assert!(!has_cucumber_expressions("user enters foo"));
    }

    #[test]
    fn cucumber_expr_to_regex_escaped_braces_are_literal_not_a_parameter() {
        // `\{expenseId\}` is an escaped (literal) brace pair per the Cucumber
        // Expression spec — it must compile to a literal-text regex with NO
        // capture group, matching a zero-argument step-definition function.
        let r = cucumber_expr_to_regex(r"alice sends GET /api/v1/expenses/\{expenseId\}");
        let re = Regex::new(&format!("^{r}$")).expect("valid regex");
        assert!(re.is_match("alice sends GET /api/v1/expenses/{expenseId}"));
        // Must not behave as a wildcard capturing arbitrary path segments.
        assert!(!re.is_match("alice sends GET /api/v1/expenses/42"));
    }

    #[test]
    fn cucumber_expr_to_regex_mixes_escaped_braces_and_real_params() {
        // Escaped braces stay literal even when a real `{int}` parameter
        // appears later in the same expression.
        let r =
            cucumber_expr_to_regex(r"POST /api/v1/admin/users/\{alice_id\}/disable/attempt/{int}");
        let re = Regex::new(&format!("^{r}$")).expect("valid regex");
        assert!(re.is_match("POST /api/v1/admin/users/{alice_id}/disable/attempt/3"));
    }

    #[test]
    fn python_parsers_d_maps_to_digit() {
        let r = convert_python_parsers_expr("count is {n:d}");
        assert!(r.contains(r"-?\d+"));
    }

    #[test]
    fn python_parsers_g_maps_to_float() {
        let r = convert_python_parsers_expr("ratio {r:g}");
        assert!(r.contains(r"-?\d+\.?\d*"));
    }

    #[test]
    fn python_parsers_w_maps_to_word() {
        let r = convert_python_parsers_expr("word {w:w}");
        assert!(r.contains(r"\S+"));
    }

    #[test]
    fn python_parsers_plain_name_maps_to_any() {
        let r = convert_python_parsers_expr("plain {x}");
        assert!(r.contains(".+"));
    }

    #[test]
    fn is_python_parsers_detects_format() {
        assert!(is_python_parsers_expr("{name}"));
        assert!(is_python_parsers_expr("{name:d}"));
        assert!(!is_python_parsers_expr("plain text"));
    }
}
