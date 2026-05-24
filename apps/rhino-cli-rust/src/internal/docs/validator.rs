//! Link resolution and validation. Mirrors Go `links_validator.go`.

use std::path::{Component, Path, PathBuf};
use std::time::Instant;

use anyhow::Error;

use super::categorizer::categorize_broken_link;
use super::scanner::{extract_links, get_markdown_files};
use super::types::{BrokenLink, LinkValidationResult, ScanOptions};

/// Resolves a relative link to an absolute, lexically-cleaned path.
/// Mirrors Go `ResolveLink` (`filepath.Join` + `filepath.Clean`).
pub fn resolve_link(source_file: &Path, link: &str) -> PathBuf {
    // Remove anchor if present.
    let link_without_anchor = link.split('#').next().unwrap_or("");

    // Pure anchor link → source file.
    if link_without_anchor.is_empty() {
        return source_file.to_path_buf();
    }

    let source_dir = source_file.parent().unwrap_or_else(|| Path::new("."));
    let joined = source_dir.join(link_without_anchor);
    clean(&joined)
}

/// Lexically cleans a path the way Go's `filepath.Clean` does: collapses `.`,
/// resolves `..` against preceding non-`..` components, and removes redundant
/// separators, without touching the filesystem.
fn clean(path: &Path) -> PathBuf {
    let is_absolute = path.is_absolute();
    let mut stack: Vec<std::ffi::OsString> = Vec::new();

    for comp in path.components() {
        match comp {
            Component::Prefix(p) => stack.push(p.as_os_str().to_os_string()),
            // RootDir is handled by is_absolute reconstruction; CurDir ("." ) is dropped.
            Component::RootDir | Component::CurDir => {}
            Component::ParentDir => {
                match stack.last() {
                    Some(last) if last != ".." => {
                        stack.pop();
                    }
                    _ => {
                        // Keep ".." for relative paths that ascend past the start.
                        if !is_absolute {
                            stack.push("..".into());
                        }
                    }
                }
            }
            Component::Normal(c) => stack.push(c.to_os_string()),
        }
    }

    let mut out = PathBuf::new();
    if is_absolute {
        out.push(std::path::MAIN_SEPARATOR_STR);
    }
    for c in stack {
        out.push(c);
    }
    // Go Clean returns "." for an empty relative result.
    if out.as_os_str().is_empty() {
        out.push(".");
    }
    out
}

/// Checks if a link's target exists. Mirrors Go `ValidateLink`.
fn validate_link(source_file: &Path, link: &str) -> bool {
    resolve_link(source_file, link).exists()
}

/// Validates all links in a single file. Mirrors Go `ValidateFile`.
/// Skill files (`.claude/skills/`) are skipped wholesale.
pub fn validate_file(file_path: &Path, opts: &ScanOptions) -> Result<Vec<BrokenLink>, Error> {
    if file_path.to_string_lossy().contains(".claude/skills/") {
        return Ok(Vec::new());
    }

    let links = extract_links(file_path)?;
    let mut broken = Vec::new();

    for link_info in links {
        if !validate_link(file_path, &link_info.url) {
            let target_path = resolve_link(file_path, &link_info.url);
            let category = categorize_broken_link(&link_info.url);

            let rel_path = file_path.strip_prefix(&opts.repo_root).map_or_else(
                |_| file_path.to_string_lossy().into_owned(),
                |r| r.to_string_lossy().into_owned(),
            );

            broken.push(BrokenLink {
                line_number: link_info.line_number,
                source_file: rel_path,
                link_text: link_info.url.clone(),
                target_path: target_path.to_string_lossy().into_owned(),
                category,
            });
        }
    }

    Ok(broken)
}

/// Validates all markdown files based on options. Mirrors Go `ValidateAllLinks`.
pub fn validate_all_links(opts: &ScanOptions) -> Result<LinkValidationResult, Error> {
    let start = Instant::now();

    let files = get_markdown_files(opts)?;

    let mut result = LinkValidationResult {
        total_files: files.len(),
        total_links: 0,
        broken_links: Vec::new(),
        broken_by_category: std::collections::HashMap::new(),
        scan_duration: std::time::Duration::ZERO,
    };

    for file_path in &files {
        // Count links (skip unreadable files, matching Go's continue).
        let Ok(links) = extract_links(file_path) else {
            continue;
        };
        result.total_links += links.len();

        let Ok(broken) = validate_file(file_path, opts) else {
            continue;
        };

        for b in broken {
            result
                .broken_by_category
                .entry(b.category.clone())
                .or_default()
                .push(b.clone());
            result.broken_links.push(b);
        }
    }

    result.scan_duration = start.elapsed();
    Ok(result)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn resolve_link_strips_anchor() {
        let src = Path::new("/repo/docs/a.md");
        assert_eq!(
            resolve_link(src, "./b.md#section"),
            PathBuf::from("/repo/docs/b.md")
        );
    }

    #[test]
    fn resolve_link_pure_anchor_returns_source() {
        let src = Path::new("/repo/docs/a.md");
        assert_eq!(resolve_link(src, "#top"), PathBuf::from("/repo/docs/a.md"));
    }

    #[test]
    fn resolve_link_parent_dir() {
        let src = Path::new("/repo/docs/sub/a.md");
        assert_eq!(
            resolve_link(src, "../b.md"),
            PathBuf::from("/repo/docs/b.md")
        );
    }

    #[test]
    fn clean_collapses_dot_and_dotdot() {
        assert_eq!(clean(Path::new("/a/b/../c")), PathBuf::from("/a/c"));
        assert_eq!(clean(Path::new("/a/./b")), PathBuf::from("/a/b"));
        assert_eq!(clean(Path::new("a/b/../../c")), PathBuf::from("c"));
        assert_eq!(clean(Path::new("../a")), PathBuf::from("../a"));
    }

    #[test]
    fn validate_all_links_counts_and_categorizes() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("docs")).unwrap();
        std::fs::write(
            root.join("docs/a.md"),
            "[ok](./real.md)\n[bad](./missing.md)\n[wf](workflows/x.md)\n",
        )
        .unwrap();
        std::fs::write(root.join("docs/real.md"), "ok\n").unwrap();

        let opts = ScanOptions {
            repo_root: root.to_path_buf(),
            ..Default::default()
        };
        let result = validate_all_links(&opts).unwrap();
        assert_eq!(result.total_links, 3);
        assert_eq!(result.broken_links.len(), 2);
        assert!(
            result
                .broken_by_category
                .contains_key("General/other paths")
        );
        assert!(result.broken_by_category.contains_key("workflows/ paths"));
    }

    #[test]
    fn validate_file_skips_skills() {
        let dir = tempfile::tempdir().unwrap();
        let skill = dir.path().join(".claude/skills/x/SKILL.md");
        std::fs::create_dir_all(skill.parent().unwrap()).unwrap();
        std::fs::write(&skill, "[bad](./nope.md)\n").unwrap();
        let opts = ScanOptions {
            repo_root: dir.path().to_path_buf(),
            ..Default::default()
        };
        let broken = validate_file(&skill, &opts).unwrap();
        assert!(broken.is_empty());
    }
}
