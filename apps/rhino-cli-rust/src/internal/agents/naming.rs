//! Agent naming-convention orchestration.
//!
//! Byte-for-byte port of the agent-relevant orchestration in
//! `cmd/agents_validate_naming.go`. The pure validators (`Violation`,
//! `validate_suffix`, `validate_frontmatter_name`, `validate_mirror`) live in
//! the shared [`crate::internal::naming`] module — this module re-exports them
//! and adds the filesystem walk over `.claude/agents` and `.opencode/agents`.

// Re-export the shared pure validators so existing callers
// (`agents::run_validate_naming`, `agents::reporter`) keep their
// `super::naming::*` paths working without duplicating logic.
pub use crate::internal::naming::{
    Violation, basename_sans_ext, validate_frontmatter_name, validate_mirror, validate_suffix,
};

/// Trailing role tokens permitted by the agent naming convention. Mirrors Go
/// `agentRoles`.
pub const AGENT_ROLES: &[&str] = &["maker", "checker", "fixer", "dev", "deployer", "manager"];

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
        assert_eq!(basename_sans_ext("/x/foo-maker.md"), "foo-maker");
        assert_eq!(basename_sans_ext("plain"), "plain");
    }

    #[test]
    fn frontmatter_name_quoted_value() {
        let content = b"---\nname: \"foo-maker\"\n---\nbody\n";
        assert!(validate_frontmatter_name("/x/foo-maker.md", content).is_none());
    }
}
