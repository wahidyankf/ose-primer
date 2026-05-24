use std::sync::OnceLock;

use regex::Regex;

/// Matches any Cucumber expression parameter like `{string}`, `{int}`, etc.
fn cucumber_param_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\{[^}]+\}").expect("valid regex"))
}

/// Matches Python pytest-bdd `parsers.parse` format like `{name:d}`, `{name}`.
fn python_parsers_param_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\{(\w+)(?::([dgw]))?\}").expect("valid regex"))
}

/// Processes Cucumber expression escape sequences in literal text:
/// `\X` → `X` for every escaped char. Mirrors Go `unescapeCucumberExpr`.
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

/// Converts the inner part of a Cucumber parameter (without braces) to regex.
/// Mirrors Go `cucumberParamToRegex`.
pub fn cucumber_param_to_regex(param_name: &str) -> &'static str {
    match param_name {
        "string" => "\"[^\"]*\"",
        "int" | "byte" | "short" | "long" => r"-?\d+",
        "float" | "double" | "bigdecimal" => r"-?\d+\.?\d*",
        "word" => r"\S+",
        _ => ".+",
    }
}

/// Converts a Cucumber expression to a regex source string, QuoteMeta-escaping
/// literal text. Mirrors Go `cucumberExprToRegex`.
pub fn cucumber_expr_to_regex(text: &str) -> String {
    let re = cucumber_param_re();
    let mut sb = String::new();
    let mut remaining = text;
    loop {
        match re.find(remaining) {
            None => {
                sb.push_str(&regex::escape(&unescape_cucumber_expr(remaining)));
                break;
            }
            Some(m) => {
                sb.push_str(&regex::escape(&unescape_cucumber_expr(
                    &remaining[..m.start()],
                )));
                let param = &remaining[m.start()..m.end()];
                let inner = &param[1..param.len() - 1];
                sb.push_str(cucumber_param_to_regex(inner));
                remaining = &remaining[m.end()..];
            }
        }
    }
    sb
}

/// True when the text contains Cucumber expression parameters.
pub fn has_cucumber_expressions(text: &str) -> bool {
    cucumber_param_re().is_match(text)
}

/// Converts a Python `parsers.parse` format string to a regex source string.
/// Mirrors Go `convertPythonParsersExpr`.
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

/// True when the text looks like a Python `parsers.parse` format string.
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
    fn cucumber_param_maps() {
        assert_eq!(cucumber_param_to_regex("string"), "\"[^\"]*\"");
        assert_eq!(cucumber_param_to_regex("int"), r"-?\d+");
        assert_eq!(cucumber_param_to_regex("long"), r"-?\d+");
        assert_eq!(cucumber_param_to_regex("float"), r"-?\d+\.?\d*");
        assert_eq!(cucumber_param_to_regex("word"), r"\S+");
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
        assert!(r.contains("\\("));
        assert!(r.contains("\\."));
    }

    #[test]
    fn has_cucumber_expressions_detects_braces() {
        assert!(has_cucumber_expressions("user enters {string}"));
        assert!(!has_cucumber_expressions("user enters foo"));
    }

    #[test]
    fn python_parsers_specs() {
        assert!(convert_python_parsers_expr("count is {n:d}").contains(r"-?\d+"));
        assert!(convert_python_parsers_expr("ratio {r:g}").contains(r"-?\d+\.?\d*"));
        assert!(convert_python_parsers_expr("word {w:w}").contains(r"\S+"));
        assert!(convert_python_parsers_expr("plain {x}").contains(".+"));
    }

    #[test]
    fn is_python_parsers_detects_format() {
        assert!(is_python_parsers_expr("{name}"));
        assert!(is_python_parsers_expr("{name:d}"));
        assert!(!is_python_parsers_expr("plain text"));
    }
}
