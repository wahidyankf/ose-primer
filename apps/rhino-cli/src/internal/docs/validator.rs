//! Link resolution and validation.

use std::path::{Component, Path, PathBuf};
use std::time::Instant;

use anyhow::Error;

use super::categorizer::categorize_broken_link;
use super::headings;
use super::scanner::{extract_links, get_markdown_files};
use super::types::{BrokenLink, LinkValidationResult, ScanOptions};

/// Resolves a relative link to an absolute, lexically-cleaned path.
/// Resolves a link target relative to the source file (join + clean).
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

/// Checks whether `fragment` matches a heading anchor in `target` (GFM slugs
/// with `-N` collision suffixes, case-sensitive). Unreadable targets are
/// treated as valid so an I/O hiccup never becomes a finding.
fn anchor_exists(target: &Path, fragment: &str) -> bool {
    let Ok(content) = std::fs::read_to_string(target) else {
        return true;
    };
    headings::collect_heading_anchors(&content)
        .iter()
        .any(|anchor| anchor == fragment)
}

/// Validates all links in a single file.
/// Skill files (`.claude/skills/`) are skipped wholesale.
pub fn validate_file(file_path: &Path, opts: &ScanOptions) -> Result<Vec<BrokenLink>, Error> {
    if file_path.to_string_lossy().contains(".claude/skills/") {
        return Ok(Vec::new());
    }

    let links = extract_links(file_path)?;
    let mut broken = Vec::new();

    let rel_path = file_path.strip_prefix(&opts.repo_root).map_or_else(
        |_| file_path.to_string_lossy().into_owned(),
        |r| r.to_string_lossy().into_owned(),
    );

    for link_info in links {
        // Capture the `#fragment` BEFORE resolution strips it.
        let fragment = link_info.url.split_once('#').map(|(_, f)| f);
        let target_path = resolve_link(file_path, &link_info.url);

        if !target_path.exists() {
            let category = categorize_broken_link(&link_info.url);
            broken.push(BrokenLink {
                line_number: link_info.line_number,
                source_file: rel_path.clone(),
                link_text: link_info.url.clone(),
                target_path: target_path.to_string_lossy().into_owned(),
                category,
            });
            continue;
        }

        // Target exists — validate the fragment (if any) against the target
        // file's heading anchor set. A pure anchor (`#fragment`) resolves to
        // the source file, so same-file anchors are covered too.
        let Some(fragment) = fragment else { continue };
        if fragment.is_empty() || !target_path.to_string_lossy().ends_with(".md") {
            continue;
        }
        if !anchor_exists(&target_path, fragment) {
            broken.push(BrokenLink {
                line_number: link_info.line_number,
                source_file: rel_path.clone(),
                link_text: link_info.url.clone(),
                target_path: target_path.to_string_lossy().into_owned(),
                category: "broken-anchor".to_string(),
            });
        }
    }

    Ok(broken)
}

/// Validates all markdown files based on options.
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
    fn validate_all_links_excludes_plans_done_via_skip_paths() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("plans/done")).unwrap();
        std::fs::create_dir_all(root.join("plans/active")).unwrap();
        std::fs::write(
            root.join("plans/done/archived.md"),
            "[bad](./missing-a.md)\n",
        )
        .unwrap();
        std::fs::write(
            root.join("plans/active/current.md"),
            "[bad](./missing-b.md)\n",
        )
        .unwrap();

        // Mirrors `docs validate-links --exclude plans/done`.
        let opts = ScanOptions {
            repo_root: root.to_path_buf(),
            skip_paths: vec!["plans/done".to_string()],
            ..Default::default()
        };
        let result = validate_all_links(&opts).unwrap();

        assert_eq!(
            result.broken_links.len(),
            1,
            "only the non-excluded broken link should be reported: {:?}",
            result.broken_links
        );
        assert_eq!(
            result.broken_links[0].source_file,
            "plans/active/current.md"
        );
    }

    #[test]
    fn validate_file_reports_broken_anchor_for_missing_section() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("docs")).unwrap();
        std::fs::write(
            root.join("docs/chapter.md"),
            "# Chapter\n\n## Real Section\n\ntext\n",
        )
        .unwrap();
        let source = root.join("docs/source.md");
        std::fs::write(&source, "[X](./chapter.md#missing-section)\n").unwrap();

        let opts = ScanOptions {
            repo_root: root.to_path_buf(),
            ..Default::default()
        };
        let broken = validate_file(&source, &opts).unwrap();

        assert_eq!(
            broken.len(),
            1,
            "missing anchor in an existing file must yield a finding: {broken:?}"
        );
        assert_eq!(broken[0].category, "broken-anchor");
        assert_eq!(broken[0].link_text, "./chapter.md#missing-section");
        assert_eq!(broken[0].line_number, 1);
    }

    #[test]
    fn validate_file_accepts_existing_anchor() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("docs")).unwrap();
        std::fs::write(
            root.join("docs/chapter.md"),
            "# Chapter\n\n## Real Section\n\ntext\n",
        )
        .unwrap();
        let source = root.join("docs/source.md");
        std::fs::write(&source, "[X](./chapter.md#real-section)\n").unwrap();

        let opts = ScanOptions {
            repo_root: root.to_path_buf(),
            ..Default::default()
        };
        let broken = validate_file(&source, &opts).unwrap();
        assert!(
            broken.is_empty(),
            "anchor matching an existing heading must not be reported: {broken:?}"
        );
    }

    #[test]
    fn validate_file_reports_broken_anchor_for_same_file_link() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("docs")).unwrap();
        let source = root.join("docs/source.md");
        std::fs::write(&source, "# Title\n\nSee [X](#own-section).\n").unwrap();

        let opts = ScanOptions {
            repo_root: root.to_path_buf(),
            ..Default::default()
        };
        let broken = validate_file(&source, &opts).unwrap();

        assert_eq!(
            broken.len(),
            1,
            "pure-anchor link with no matching heading must yield a finding: {broken:?}"
        );
        assert_eq!(broken[0].category, "broken-anchor");
        assert_eq!(broken[0].link_text, "#own-section");
        assert_eq!(broken[0].line_number, 3);
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
