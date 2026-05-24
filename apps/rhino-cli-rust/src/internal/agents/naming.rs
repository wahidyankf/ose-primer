//! Agent naming-convention validators.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/internal/naming/naming.go` (the
//! agent-relevant subset) plus the orchestration in
//! `cmd/agents_validate_naming.go`. The validators are filesystem-agnostic:
//! callers collect the file lists (and content bytes for frontmatter checks)
//! and pass them in.

use std::path::Path;

use serde::Serialize;

/// Trailing role tokens permitted by the agent naming convention. Mirrors Go
/// `agentRoles`.
pub const AGENT_ROLES: &[&str] = &["maker", "checker", "fixer", "dev", "deployer", "manager"];

/// A single naming-rule failure. Mirrors Go `naming.Violation`. Serialized to
/// JSON with `path`, `kind`, `message` keys (Go uses exported field names with
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

/// Returns the filename of `path` with the `.md` extension stripped.
/// Mirrors Go `basenameSansExt`.
fn basename_sans_ext(path: &str) -> String {
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
/// Mirrors Go `ValidateSuffix`.
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
/// of `content`, or empty if absent. Mirrors Go `extractFrontmatterName`: a
/// minimal parser reading the first `name:` line in the delimited block.
fn extract_frontmatter_name(content: &[u8]) -> String {
    let text = String::from_utf8_lossy(content);
    if !text.starts_with("---\n") && !text.starts_with("---\r\n") {
        return String::new();
    }
    let rest = &text[4..];
    let end = match rest.find("\n---") {
        Some(e) => e,
        None => return String::new(),
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
/// `basename(path)`. No frontmatter / no `name:` → no violation. Mirrors Go
/// `ValidateFrontmatterName`.
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
/// and `opencode_files` (compared by basename). Mirrors Go `ValidateMirror`,
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

/// Walks the agent tree under `repo_root` and returns every naming violation,
/// sorted by path then kind. Mirrors Go `agentsValidateNaming`.
pub fn validate_naming(repo_root: &std::path::Path) -> Result<Vec<Violation>, anyhow::Error> {
    let claude_dir = repo_root.join(".claude").join("agents");
    let opencode_dir = repo_root.join(".opencode").join("agents");

    let claude_files = list_agent_files(&claude_dir)?;
    let opencode_files = list_agent_files(&opencode_dir)?;

    let mut violations: Vec<Violation> = Vec::new();

    // Suffix + frontmatter checks for .claude/agents/*.md.
    for path in &claude_files {
        if let Some(v) = validate_suffix(path, AGENT_ROLES, "role-suffix") {
            violations.push(v);
        }
        let content = std::fs::read(path).map_err(|e| anyhow::anyhow!("read {path}: {e}"))?;
        if let Some(v) = validate_frontmatter_name(path, &content) {
            violations.push(v);
        }
    }

    // Suffix check for .opencode/agents/*.md (frontmatter omits `name:`).
    for path in &opencode_files {
        if let Some(v) = validate_suffix(path, AGENT_ROLES, "role-suffix") {
            violations.push(v);
        }
    }

    // Mirror-drift check.
    violations.extend(validate_mirror(&claude_files, &opencode_files));

    // Stable sort by (path, kind) — mirrors Go's sort.SliceStable.
    violations.sort_by(|a, b| {
        if a.path == b.path {
            a.kind.cmp(&b.kind)
        } else {
            a.path.cmp(&b.path)
        }
    });

    Ok(violations)
}

/// Returns absolute paths for `*.md` files directly under `dir`, excluding
/// `README.md` and `ci-monitor-subagent.md`. A missing dir yields an empty
/// list. Mirrors Go `listAgentFiles`.
fn list_agent_files(dir: &std::path::Path) -> Result<Vec<String>, anyhow::Error> {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(anyhow::anyhow!("read {}: {e}", dir.display())),
    };
    let mut files: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        if entry.file_type().is_ok_and(|t| t.is_dir()) {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == "README.md" || name == "ci-monitor-subagent.md" {
            continue;
        }
        if !name.ends_with(".md") {
            continue;
        }
        files.push(dir.join(&name).to_string_lossy().into_owned());
    }
    files.sort();
    Ok(files)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn suffix_ok() {
        assert!(validate_suffix("/x/foo-maker.md", AGENT_ROLES, "role-suffix").is_none());
        assert!(validate_suffix("/x/swe-rust-dev.md", AGENT_ROLES, "role-suffix").is_none());
    }

    #[test]
    fn suffix_unknown_fails() {
        let v = validate_suffix("/x/foo-bar.md", AGENT_ROLES, "role-suffix").unwrap();
        assert_eq!(v.kind, "role-suffix");
        assert!(v.message.contains("\"foo-bar\""));
    }

    #[test]
    fn bare_suffix_fails() {
        let v = validate_suffix("/x/maker.md", AGENT_ROLES, "role-suffix").unwrap();
        assert_eq!(v.kind, "role-suffix");
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
    fn validate_naming_clean_tree() {
        let dir = tempfile::tempdir().unwrap();
        let claude = dir.path().join(".claude/agents");
        let opencode = dir.path().join(".opencode/agents");
        std::fs::create_dir_all(&claude).unwrap();
        std::fs::create_dir_all(&opencode).unwrap();
        std::fs::write(
            claude.join("foo-maker.md"),
            "---\nname: foo-maker\n---\nx\n",
        )
        .unwrap();
        std::fs::write(
            opencode.join("foo-maker.md"),
            "---\ndescription: d\n---\nx\n",
        )
        .unwrap();
        let v = validate_naming(dir.path()).unwrap();
        assert!(v.is_empty(), "{v:?}");
    }

    #[test]
    fn validate_naming_reports_violations_sorted() {
        let dir = tempfile::tempdir().unwrap();
        let claude = dir.path().join(".claude/agents");
        std::fs::create_dir_all(&claude).unwrap();
        // Bad suffix + frontmatter mismatch + no opencode mirror.
        std::fs::write(claude.join("foo-widget.md"), "---\nname: nope\n---\nx\n").unwrap();
        let v = validate_naming(dir.path()).unwrap();
        // role-suffix, frontmatter-mismatch, and mirror-drift all fire.
        let kinds: Vec<&str> = v.iter().map(|x| x.kind.as_str()).collect();
        assert!(kinds.contains(&"role-suffix"));
        assert!(kinds.contains(&"frontmatter-mismatch"));
        assert!(kinds.contains(&"mirror-drift"));
        // Same path → sorted by kind: frontmatter-mismatch < mirror-drift < role-suffix.
        assert_eq!(v[0].kind, "frontmatter-mismatch");
    }

    #[test]
    fn list_agent_files_excludes_readme_and_subagent() {
        let dir = tempfile::tempdir().unwrap();
        let claude = dir.path().join(".claude/agents");
        let opencode = dir.path().join(".opencode/agents");
        std::fs::create_dir_all(&claude).unwrap();
        std::fs::create_dir_all(&opencode).unwrap();
        std::fs::write(
            claude.join("foo-maker.md"),
            "---\nname: foo-maker\n---\nx\n",
        )
        .unwrap();
        std::fs::write(claude.join("README.md"), "# readme\n").unwrap();
        std::fs::write(
            opencode.join("foo-maker.md"),
            "---\ndescription: d\n---\nx\n",
        )
        .unwrap();
        // ci-monitor-subagent.md exists only in opencode and is exempt.
        std::fs::write(
            opencode.join("ci-monitor-subagent.md"),
            "---\ndescription: d\n---\nx\n",
        )
        .unwrap();
        let v = validate_naming(dir.path()).unwrap();
        assert!(v.is_empty(), "{v:?}");
    }

    #[test]
    fn basename_without_extension() {
        assert_eq!(super::basename_sans_ext("/x/foo-maker.md"), "foo-maker");
        assert_eq!(super::basename_sans_ext("plain"), "plain");
    }

    #[test]
    fn frontmatter_name_quoted_value() {
        let content = b"---\nname: \"foo-maker\"\n---\nbody\n";
        assert!(validate_frontmatter_name("/x/foo-maker.md", content).is_none());
    }
}
