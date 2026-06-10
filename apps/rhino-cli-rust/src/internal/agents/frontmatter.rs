//! Frontmatter extraction, YAML normalization, and minimal YAML parsing.
//!
//! The Go code splits content on `\n`, validates the `---` fences, normalizes missing
//! spaces after colons, and parses the YAML with `gopkg.in/yaml.v3`.
//!
//! The parse step uses `serde_norway` (an order-preserving serde_yaml fork) to
//! produce a generic value tree, which we re-wrap into [`YamlValue`] so the
//! field-order validator sees keys in document order.

use anyhow::{Error, anyhow};
use regex::Regex;
use std::sync::OnceLock;

/// An order-preserving YAML value tree, mirroring the subset of
/// `interface{}` shapes the Go agents code relies on.
#[derive(Debug, Clone, PartialEq)]
pub enum YamlValue {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Sequence(Vec<YamlValue>),
    /// Key/value pairs in document order (matches yaml.v3 mapping order).
    Mapping(Vec<(String, YamlValue)>),
}

impl YamlValue {
    /// Returns the string value if this is a `String`, otherwise `None`.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            YamlValue::String(s) => Some(s),
            _ => None,
        }
    }
}

/// Returns the regex used by [`normalize_yaml`], compiled once.
fn normalize_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        //
        Regex::new(r"(?m)^([a-zA-Z0-9_-]+):([^\s])").expect("valid regex")
    })
}

/// Adds a space after colons where missing in key-value lines
/// (e.g. `name:value` → `name: value`). Operates on
/// the byte content as UTF-8 text; non-UTF-8 bytes are left untouched (the Go
/// code operates on bytes but the regex only matches ASCII).
pub fn normalize_yaml(content: &[u8]) -> Vec<u8> {
    let text = String::from_utf8_lossy(content);
    let normalized = normalize_re().replace_all(&text, "$1: $2");
    normalized.into_owned().into_bytes()
}

/// Extracts YAML frontmatter and body from markdown content.
/// finds the closing `---`, normalizes the frontmatter, and returns
/// `(frontmatter, body)`. The frontmatter excludes the fences; the body is
/// everything after the closing fence (joined with `\n`).
pub fn extract_frontmatter(content: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let lines: Vec<&[u8]> = split_lines(content);

    if lines.len() < 3 {
        return Err(anyhow!("file too short to contain frontmatter"));
    }

    if trim_space(lines[0]) != b"---" {
        return Err(anyhow!("frontmatter does not start with ---"));
    }

    let mut end_index: isize = -1;
    for (i, line) in lines.iter().enumerate().skip(1) {
        if trim_space(line) == b"---" {
            end_index = i as isize;
            break;
        }
    }

    if end_index == -1 {
        return Err(anyhow!("frontmatter closing --- not found"));
    }
    let end = end_index.cast_unsigned();

    // Frontmatter = lines[1..end] joined with "\n", then normalized.
    let frontmatter_raw = join_lines(&lines[1..end]);
    let frontmatter = normalize_yaml(&frontmatter_raw);

    // Body = lines[end+1..] joined with "\n", or empty.
    let body = if end + 1 < lines.len() {
        join_lines(&lines[end + 1..])
    } else {
        Vec::new()
    };

    Ok((frontmatter, body))
}

/// Splits a byte slice on `\n` exactly like Go's `bytes.Split(content, "\n")`.
/// A trailing `\n` yields a final empty element (Go semantics).
fn split_lines(content: &[u8]) -> Vec<&[u8]> {
    content.split(|&b| b == b'\n').collect()
}

/// Joins byte-slice lines with `\n` (inverse of [`split_lines`]).
fn join_lines(lines: &[&[u8]]) -> Vec<u8> {
    let mut out = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if i > 0 {
            out.push(b'\n');
        }
        out.extend_from_slice(line);
    }
    out
}

/// Trims ASCII whitespace from both ends (mirrors `bytes.TrimSpace` for the
/// whitespace classes that appear in agent files: space, tab, CR, LF).
fn trim_space(s: &[u8]) -> &[u8] {
    let is_ws =
        |b: u8| b == b' ' || b == b'\t' || b == b'\r' || b == b'\n' || b == 0x0b || b == 0x0c;
    let mut start = 0;
    let mut end = s.len();
    while start < end && is_ws(s[start]) {
        start += 1;
    }
    while end > start && is_ws(s[end - 1]) {
        end -= 1;
    }
    &s[start..end]
}

/// Parses normalized frontmatter bytes into an order-preserving [`YamlValue`].
/// Returns an error. failure path.
pub fn parse_yaml_value(frontmatter: &[u8]) -> Result<YamlValue, Error> {
    // serde_norway parses the document into an order-preserving Value.
    let raw: serde_norway::Value =
        serde_norway::from_slice(frontmatter).map_err(|e| anyhow!("{e}"))?;
    Ok(convert_norway(&raw))
}

/// Re-wraps a `serde_norway::Value` into the local [`YamlValue`], preserving
/// mapping key order (serde_norway's `Mapping` is backed by an `IndexMap`).
fn convert_norway(v: &serde_norway::Value) -> YamlValue {
    match v {
        serde_norway::Value::Null => YamlValue::Null,
        serde_norway::Value::Bool(b) => YamlValue::Bool(*b),
        serde_norway::Value::Number(n) => YamlValue::Number(n.to_string()),
        serde_norway::Value::String(s) => YamlValue::String(s.clone()),
        serde_norway::Value::Sequence(seq) => {
            YamlValue::Sequence(seq.iter().map(convert_norway).collect())
        }
        serde_norway::Value::Mapping(m) => {
            let mut pairs = Vec::with_capacity(m.len());
            for (k, val) in m {
                // Only string keys appear in agent frontmatter.
                let key = match k {
                    serde_norway::Value::String(s) => s.clone(),
                    other => norway_scalar_string(other),
                };
                pairs.push((key, convert_norway(val)));
            }
            YamlValue::Mapping(pairs)
        }
        serde_norway::Value::Tagged(t) => convert_norway(&t.value),
    }
}

/// Best-effort scalar→string for non-string mapping keys (not expected in
/// agent frontmatter, but handled for totality).
fn norway_scalar_string(v: &serde_norway::Value) -> String {
    match v {
        serde_norway::Value::Bool(b) => b.to_string(),
        serde_norway::Value::Number(n) => n.to_string(),
        serde_norway::Value::Null => "null".to_string(),
        _ => String::new(),
    }
}

/// Parses Claude tools from either an array or a comma-separated string.
///: array elements that are strings are kept
/// verbatim; a string is split on commas and each non-empty trimmed part is
/// kept.
pub fn parse_claude_tools(tools_raw: &YamlValue) -> Vec<String> {
    let mut tools = Vec::new();
    match tools_raw {
        YamlValue::Sequence(seq) => {
            for item in seq {
                if let YamlValue::String(s) = item {
                    tools.push(s.clone());
                }
            }
        }
        YamlValue::String(s) => {
            for part in s.split(',') {
                let trimmed = part.trim();
                if !trimmed.is_empty() {
                    tools.push(trimmed.to_string());
                }
            }
        }
        _ => {}
    }
    tools
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn normalize_adds_space_after_colon() {
        let out = normalize_yaml(b"name:value\ndescription: ok\n");
        assert_eq!(out, b"name: value\ndescription: ok\n");
    }

    #[test]
    fn normalize_leaves_list_items() {
        let out = normalize_yaml(b"skills:\n  - a\n  - b\n");
        assert_eq!(out, b"skills:\n  - a\n  - b\n");
    }

    #[test]
    fn extract_frontmatter_basic() {
        let content = b"---\nname: foo\n---\n# Body\ntext\n";
        let (fm, body) = extract_frontmatter(content).unwrap();
        assert_eq!(fm, b"name: foo");
        assert_eq!(body, b"# Body\ntext\n");
    }

    #[test]
    fn extract_frontmatter_no_start_fence() {
        let err = extract_frontmatter(b"name: foo\nbar: baz\nqux: x\n").unwrap_err();
        assert_eq!(err.to_string(), "frontmatter does not start with ---");
    }

    #[test]
    fn extract_frontmatter_too_short() {
        let err = extract_frontmatter(b"---\n").unwrap_err();
        assert_eq!(err.to_string(), "file too short to contain frontmatter");
    }

    #[test]
    fn extract_frontmatter_no_close() {
        let err = extract_frontmatter(b"---\nname: foo\nbar: baz\n").unwrap_err();
        assert_eq!(err.to_string(), "frontmatter closing --- not found");
    }

    #[test]
    fn extract_frontmatter_empty_body() {
        // Closing fence is the last line → empty body.
        let content = b"---\nname: foo\n---";
        let (fm, body) = extract_frontmatter(content).unwrap();
        assert_eq!(fm, b"name: foo");
        assert_eq!(body, b"");
    }

    #[test]
    fn parse_tools_from_array() {
        let v = YamlValue::Sequence(vec![
            YamlValue::String("Read".to_string()),
            YamlValue::String("Write".to_string()),
        ]);
        assert_eq!(parse_claude_tools(&v), vec!["Read", "Write"]);
    }

    #[test]
    fn parse_tools_from_string() {
        let v = YamlValue::String("Read, Write , Bash".to_string());
        assert_eq!(parse_claude_tools(&v), vec!["Read", "Write", "Bash"]);
    }

    #[test]
    fn parse_yaml_preserves_order() {
        let v = parse_yaml_value(b"name: a\ndescription: b\ntools: c\n").unwrap();
        match v {
            YamlValue::Mapping(pairs) => {
                let keys: Vec<&str> = pairs.iter().map(|(k, _)| k.as_str()).collect();
                assert_eq!(keys, vec!["name", "description", "tools"]);
            }
            _ => panic!("expected mapping"),
        }
    }

    #[test]
    fn parse_tools_non_collection_is_empty() {
        // A bool/number value yields no tools.
        assert!(parse_claude_tools(&YamlValue::Bool(true)).is_empty());
        assert!(parse_claude_tools(&YamlValue::Number("3".to_string())).is_empty());
        assert!(parse_claude_tools(&YamlValue::Null).is_empty());
    }

    #[test]
    fn parse_yaml_handles_scalars_and_bools() {
        let v = parse_yaml_value(b"a: true\nb: 42\nc: ~\nd: text\n").unwrap();
        match v {
            YamlValue::Mapping(pairs) => {
                assert!(matches!(pairs[0].1, YamlValue::Bool(true)));
                assert!(matches!(&pairs[1].1, YamlValue::Number(n) if n == "42"));
                assert!(matches!(pairs[2].1, YamlValue::Null));
                assert!(matches!(&pairs[3].1, YamlValue::String(s) if s == "text"));
            }
            _ => panic!("expected mapping"),
        }
    }

    #[test]
    fn parse_yaml_handles_nested_sequence() {
        let v = parse_yaml_value(b"skills:\n  - one\n  - two\n").unwrap();
        match v {
            YamlValue::Mapping(pairs) => match &pairs[0].1 {
                YamlValue::Sequence(seq) => assert_eq!(seq.len(), 2),
                other => panic!("expected sequence, got {other:?}"),
            },
            _ => panic!("expected mapping"),
        }
    }

    #[test]
    fn as_str_only_for_strings() {
        assert_eq!(YamlValue::String("x".to_string()).as_str(), Some("x"));
        assert_eq!(YamlValue::Bool(true).as_str(), None);
    }
}
