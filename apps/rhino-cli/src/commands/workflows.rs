//! `workflows validate-naming` command.
//!
//! Reuses the shared naming validators and reporter in [`crate::internal::naming`]. Output
//! is written with `print!` (no implicit trailing newline).

use std::fs;
use std::path::Path;

use anyhow::{Context, Error, anyhow};

use crate::internal::cliout::OutputFormat;
use crate::internal::git;
use crate::internal::naming::reporter::{
    format_naming_json, format_naming_markdown, format_naming_text,
};
use crate::internal::naming::{self, Violation};

/// Usage block printed to stderr when `validate-naming` errors.
pub const VALIDATE_NAMING_USAGE: &str = "Usage:\n  \
rhino-cli workflows validate-naming [flags]\n\n\
Examples:\n  \
# Validate workflow naming across the governance tree\n  \
rhino-cli workflows validate-naming\n\n  \
# Output as JSON\n  \
rhino-cli workflows validate-naming -o json\n\n  \
# Markdown output\n  \
rhino-cli workflows validate-naming -o markdown\n\n\
Flags:\n  \
-h, --help   help for validate-naming\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

/// Trailing type tokens permitted by the workflow naming convention.
const WORKFLOW_TYPES: &[&str] = &["quality-gate", "execution", "setup", "planning"];

pub fn run_validate_naming(output: OutputFormat, verbose: bool, quiet: bool) -> Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let violations =
        workflows_validate_naming(&repo_root).map_err(|e| anyhow!("validation failed: {e}"))?;

    let out = match output {
        OutputFormat::Text => format_naming_text("Workflows", &violations, verbose, quiet),
        OutputFormat::Json => format_naming_json("workflows", &violations)?,
        OutputFormat::Markdown => format_naming_markdown("Workflows", &violations),
    };
    print!("{out}");

    if !violations.is_empty() {
        return Err(anyhow!("{} naming violation(s) found", violations.len()));
    }
    Ok(())
}

/// Walks `repo-governance/workflows/` recursively, excluding README.md files and
/// anything under `meta/`, and returns every naming violation sorted by (path,
/// kind).
fn workflows_validate_naming(repo_root: &Path) -> Result<Vec<Violation>, Error> {
    let root = repo_root.join("repo-governance").join("workflows");
    let files = list_workflow_files(&root);

    let mut violations: Vec<Violation> = Vec::new();
    for path in &files {
        if let Some(v) = naming::validate_suffix(path, WORKFLOW_TYPES, "type-suffix") {
            violations.push(v);
        }
        let content = fs::read(path).with_context(|| format!("read {path}"))?;
        if let Some(v) = naming::validate_frontmatter_name(path, &content) {
            violations.push(v);
        }
    }

    // Stable sort by (path, kind).
    violations.sort_by(|a, b| a.path.cmp(&b.path).then(a.kind.cmp(&b.kind)));
    Ok(violations)
}

/// Returns every `.md` file under `root` eligible for validation. Files named
/// `README.md` and anything under a `meta/` directory (at any depth below
/// `root`) are filtered out per convention. A missing root yields an empty
/// slice.
fn list_workflow_files(root: &Path) -> Vec<String> {
    if !root.exists() {
        return Vec::new();
    }
    let mut files = Vec::new();
    let walker = walkdir::WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| {
            // Skip the meta/ reference tree at any depth beneath root.
            if e.file_type().is_dir() && e.path() != root {
                let name = e.file_name().to_string_lossy().to_string();
                if name == "meta" {
                    return false;
                }
            }
            true
        });
    for entry in walker.flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name == "README.md" || !name.ends_with(".md") {
            continue;
        }
        files.push(entry.path().to_string_lossy().to_string());
    }
    files.sort();
    files
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn list_workflow_files_skips_meta_and_readme() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("repo-governance/workflows");
        std::fs::create_dir_all(root.join("meta")).unwrap();
        std::fs::write(root.join("README.md"), "x").unwrap();
        std::fs::write(root.join("meta/foo.md"), "x").unwrap();
        std::fs::write(root.join("plan-quality-gate.md"), "x").unwrap();
        let files = list_workflow_files(&root);
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("plan-quality-gate.md"));
    }

    #[test]
    fn list_workflow_files_skips_nested_meta() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("repo-governance/workflows");
        std::fs::create_dir_all(root.join("plan/meta")).unwrap();
        std::fs::write(root.join("plan/meta/ref.md"), "x").unwrap();
        std::fs::write(root.join("plan/plan-execution.md"), "x").unwrap();
        let files = list_workflow_files(&root);
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("plan-execution.md"));
    }

    #[test]
    fn list_workflow_files_missing_root_empty() {
        let tmp = TempDir::new().unwrap();
        let files = list_workflow_files(&tmp.path().join("nope"));
        assert!(files.is_empty());
    }

    #[test]
    fn validate_naming_clean_returns_empty() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("repo-governance/workflows");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("plan-quality-gate.md"),
            "---\nname: plan-quality-gate\n---\n",
        )
        .unwrap();
        let result = workflows_validate_naming(tmp.path()).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn validate_naming_detects_bad_suffix() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("repo-governance/workflows");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("foo-bar.md"), "---\nname: foo-bar\n---\n").unwrap();
        let result = workflows_validate_naming(tmp.path()).unwrap();
        assert!(result.iter().any(|v| v.kind == "type-suffix"));
    }

    #[test]
    fn validate_naming_accepts_planning_suffix() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("repo-governance/workflows");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("repo-dependency-bump-planning.md"),
            "---\nname: repo-dependency-bump-planning\n---\n",
        )
        .unwrap();
        let result = workflows_validate_naming(tmp.path()).unwrap();
        assert!(
            result.is_empty(),
            "a `-planning` suffix must be accepted, got: {result:?}"
        );
    }

    #[test]
    fn validate_naming_rejects_bogus_suffix_alongside_planning() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("repo-governance/workflows");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("repo-dependency-bump-bogus.md"),
            "---\nname: repo-dependency-bump-bogus\n---\n",
        )
        .unwrap();
        let result = workflows_validate_naming(tmp.path()).unwrap();
        assert!(
            result.iter().any(|v| v.kind == "type-suffix"),
            "a bogus suffix must still be rejected, got: {result:?}"
        );
    }

    #[test]
    fn validate_naming_detects_frontmatter_mismatch() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("repo-governance/workflows");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("plan-setup.md"), "---\nname: wrong-setup\n---\n").unwrap();
        let result = workflows_validate_naming(tmp.path()).unwrap();
        assert!(result.iter().any(|v| v.kind == "frontmatter-mismatch"));
    }
}
