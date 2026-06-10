//! Pure naming-convention validators shared by `agents validate-naming` and
//! `workflows validate-naming`.
//!
//! The validators are filesystem-agnostic: callers collect the file lists (and content
//! bytes for frontmatter checks) and pass them in. The orchestration that walks the real
//! tree lives in the per-command modules (`internal::agents::naming`,
//! `commands::workflows`).

pub mod reporter;

use std::path::Path;

use serde::Serialize;

/// A single naming-rule failure. Serialized to
/// JSON with `Path`, `Kind`, `Message` keys (Go uses exported field names with
/// default json tags → PascalCase keys).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Violation {
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Kind")]
    pub kind: String,
    #[serde(rename = "Message")]
    pub message: String,
}

/// Returns the filename of `path` with the final extension stripped.
pub fn basename_sans_ext(path: &str) -> String {
    let base = Path::new(path)
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    // Strip the final extension (Go filepath.Ext returns the last dot segment).
    if let Some(idx) = base.rfind('.') {
        base[..idx].to_string()
    } else {
        base
    }
}

/// Returns a [`Violation`] when `basename(path)` (without `.md`) does not end
/// with any of `allowed_suffixes`. A bare suffix (e.g. `maker.md`) is invalid.
pub fn validate_suffix(path: &str, allowed_suffixes: &[&str], kind: &str) -> Option<Violation> {
    let name = basename_sans_ext(path);
    for suffix in allowed_suffixes {
        if name == *suffix {
            // Bare suffix has no scope and is invalid.
            continue;
        }
        if name.ends_with(&format!("-{suffix}")) {
            return None;
        }
    }
    Some(Violation {
        path: path.to_string(),
        kind: kind.to_string(),
        message: format!(
            "filename {:?} does not end with any allowed suffix ({})",
            name,
            allowed_suffixes.join(", ")
        ),
    })
}

/// Returns the value of the top-level `name:` field from the YAML frontmatter
/// of `content`, or empty if absent.: a
/// minimal parser reading the first `name:` line in the delimited block.
fn extract_frontmatter_name(content: &[u8]) -> String {
    let text = String::from_utf8_lossy(content);
    if !text.starts_with("---\n") && !text.starts_with("---\r\n") {
        return String::new();
    }
    let rest = &text[4..];
    let Some(end) = rest.find("\n---") else {
        return String::new();
    };
    let frontmatter = &rest[..end];
    for line in frontmatter.split('\n') {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("name:") {
            let value = value.trim();
            // Strip optional surrounding quotes.
            return value.trim_matches(|c| c == '"' || c == '\'').to_string();
        }
    }
    String::new()
}

/// Returns a [`Violation`] when the frontmatter `name:` field does not equal
/// `basename(path)`. No frontmatter / no `name:` → no violation.
pub fn validate_frontmatter_name(path: &str, content: &[u8]) -> Option<Violation> {
    let name = extract_frontmatter_name(content);
    if name.is_empty() {
        return None;
    }
    let expected = basename_sans_ext(path);
    if name == expected {
        return None;
    }
    Some(Violation {
        path: path.to_string(),
        kind: "frontmatter-mismatch".to_string(),
        message: format!("frontmatter name {name:?} does not match filename {expected:?}"),
    })
}

/// Returns violations for every file present in exactly one of `claude_files`
/// and `opencode_files` (compared by basename).
/// including its `.opencode/agent/` (singular) wording in the messages.
pub fn validate_mirror(claude_files: &[String], opencode_files: &[String]) -> Vec<Violation> {
    use std::collections::BTreeMap;

    let mut claude_set: BTreeMap<String, String> = BTreeMap::new();
    for p in claude_files {
        claude_set.insert(basename_sans_ext(p), p.clone());
    }
    let mut opencode_set: BTreeMap<String, String> = BTreeMap::new();
    for p in opencode_files {
        opencode_set.insert(basename_sans_ext(p), p.clone());
    }

    let mut violations: Vec<Violation> = Vec::new();
    for (name, path) in &claude_set {
        if !opencode_set.contains_key(name) {
            violations.push(Violation {
                path: path.clone(),
                kind: "mirror-drift".to_string(),
                message: format!("{name}.md exists in .claude/agents/ but not in .opencode/agent/"),
            });
        }
    }
    for (name, path) in &opencode_set {
        if !claude_set.contains_key(name) {
            violations.push(Violation {
                path: path.clone(),
                kind: "mirror-drift".to_string(),
                message: format!("{name}.md exists in .opencode/agent/ but not in .claude/agents/"),
            });
        }
    }
    violations.sort_by(|a, b| a.path.cmp(&b.path));
    violations
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn suffix_ok() {
        assert!(validate_suffix("/x/foo-maker.md", &["maker"], "role-suffix").is_none());
        assert!(
            validate_suffix("/x/foo-quality-gate.md", &["quality-gate"], "type-suffix").is_none()
        );
    }

    #[test]
    fn suffix_unknown_fails() {
        let v = validate_suffix("/x/foo-bar.md", &["maker"], "role-suffix").unwrap();
        assert_eq!(v.kind, "role-suffix");
        assert!(v.message.contains("\"foo-bar\""));
    }

    #[test]
    fn bare_suffix_fails() {
        let v = validate_suffix("/x/setup.md", &["setup"], "type-suffix").unwrap();
        assert_eq!(v.kind, "type-suffix");
    }

    #[test]
    fn frontmatter_name_match() {
        let content = b"---\nname: foo-maker\n---\nbody\n";
        assert!(validate_frontmatter_name("/x/foo-maker.md", content).is_none());
    }

    #[test]
    fn frontmatter_name_mismatch() {
        let content = b"---\nname: other\n---\nbody\n";
        let v = validate_frontmatter_name("/x/foo-maker.md", content).unwrap();
        assert_eq!(v.kind, "frontmatter-mismatch");
        assert!(v.message.contains("\"other\""));
        assert!(v.message.contains("\"foo-maker\""));
    }

    #[test]
    fn frontmatter_no_name_no_violation() {
        let content = b"---\ndescription: d\n---\nbody\n";
        assert!(validate_frontmatter_name("/x/foo-maker.md", content).is_none());
    }

    #[test]
    fn frontmatter_quoted_value() {
        let content = b"---\nname: \"foo-maker\"\n---\nbody\n";
        assert!(validate_frontmatter_name("/x/foo-maker.md", content).is_none());
    }

    #[test]
    fn mirror_drift_claude_only() {
        let claude = vec!["/c/foo-maker.md".to_string(), "/c/bar-maker.md".to_string()];
        let opencode = vec!["/o/foo-maker.md".to_string()];
        let v = validate_mirror(&claude, &opencode);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].kind, "mirror-drift");
        assert!(
            v[0].message
                .contains("exists in .claude/agents/ but not in .opencode/agent/")
        );
    }

    #[test]
    fn mirror_drift_opencode_only() {
        let claude = vec!["/c/foo-maker.md".to_string()];
        let opencode = vec![
            "/o/foo-maker.md".to_string(),
            "/o/extra-maker.md".to_string(),
        ];
        let v = validate_mirror(&claude, &opencode);
        assert_eq!(v.len(), 1);
        assert!(
            v[0].message
                .contains("exists in .opencode/agent/ but not in .claude/agents/")
        );
    }

    #[test]
    fn basename_without_extension() {
        assert_eq!(basename_sans_ext("/x/foo-maker.md"), "foo-maker");
        assert_eq!(basename_sans_ext("plain"), "plain");
    }
}
