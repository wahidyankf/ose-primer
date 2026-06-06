//! Markdown file discovery and link extraction. Mirrors Go `links_scanner.go`.

use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use anyhow::Error;
use regex::Regex;
use walkdir::WalkDir;

use super::fences::FenceTracker;
use super::types::{LinkInfo, ScanOptions};

/// Matches markdown links: `[text](url)`. Mirrors Go `linkRegex`.
static LINK_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").expect("valid link regex"));

/// Matches template placeholders in square brackets, e.g. `[some-id]`.
static BRACKET_PLACEHOLDER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[[\w-]+\]").expect("valid bracket placeholder regex"));

/// Returns markdown files to scan based on options. Mirrors Go `GetMarkdownFiles`.
pub fn get_markdown_files(opts: &ScanOptions) -> Result<Vec<PathBuf>, Error> {
    let files = if opts.staged_only {
        get_staged_markdown_files(&opts.repo_root)?
    } else {
        get_all_markdown_files(&opts.repo_root)
    };
    Ok(filter_skip_paths(files, &opts.repo_root, &opts.skip_paths))
}

/// Filters out files matching any skip path (relative-prefix match). Mirrors Go `filterSkipPaths`.
/// Shared with the mermaid `--exclude` filter (`commands::docs::filter_mermaid_excluded`)
/// so both gates use one prefix implementation per CLI (plan DD-2).
pub(crate) fn filter_skip_paths(
    files: Vec<PathBuf>,
    repo_root: &Path,
    skip_paths: &[String],
) -> Vec<PathBuf> {
    if skip_paths.is_empty() {
        return files;
    }

    let mut filtered = Vec::new();
    for file in files {
        let Ok(rel_path) = file.strip_prefix(repo_root) else {
            // If we can't get a relative path, keep the file (matches Go).
            filtered.push(file);
            continue;
        };
        let rel = rel_path.to_string_lossy().into_owned();

        let mut skip = false;
        for skip_path in skip_paths {
            // Go compares against both the raw skip path and filepath.Clean(skipPath).
            let cleaned = clean_path(skip_path);
            if rel.starts_with(skip_path.as_str()) || rel.starts_with(cleaned.as_str()) {
                skip = true;
                break;
            }
        }

        if !skip {
            filtered.push(file);
        }
    }
    filtered
}

/// Lexical path cleaning equivalent to Go `filepath.Clean` for the simple
/// trailing-slash inputs used by skip paths (e.g. ".opencode/skill/").
fn clean_path(p: &str) -> String {
    // For the skip-path inputs in practice, Clean only strips trailing slashes.
    let trimmed = p.trim_end_matches('/');
    if trimmed.is_empty() {
        ".".to_string()
    } else {
        trimmed.to_string()
    }
}

/// Returns staged markdown files from git (absolute paths). Mirrors Go `getStagedMarkdownFiles`.
fn get_staged_markdown_files(repo_root: &Path) -> Result<Vec<PathBuf>, Error> {
    let output = std::process::Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--name-only")
        .arg("--diff-filter=ACM")
        .current_dir(repo_root)
        .output()?;
    if !output.status.success() {
        return Err(anyhow::anyhow!("git diff --cached failed"));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut files = Vec::new();
    for line in stdout.trim().lines() {
        if line.is_empty() {
            continue;
        }
        if line.ends_with(".md") {
            files.push(repo_root.join(line));
        }
    }
    Ok(files)
}

/// Standardized cross-repo noise-skip set: directory NAMES dropped from the
/// repo-wide walk wherever they appear, plus `.git`. Identical across the
/// three aligned repos (ose-public / ose-infra / ose-primer). This is the
/// ONE definition per CLI (plan DD-3), consumed only by
/// `get_all_markdown_files` below.
pub(crate) const NOISE_DIRS: &[&str] = &[
    "node_modules",
    "dist",
    "target",
    ".next",
    "coverage",
    "generated-reports",
    "local-temp",
    "archived",
    "apps-labs",
    "worktrees",
    ".terraform",
    "generated-contracts",
    ".nx",
    ".git",
];

/// Returns all `*.md` files under `root` (any directory, or a single file —
/// the walk root at depth 0 is never filtered) via a walk that skips the
/// standardized noise-skip set by directory name. Mirrors Go
/// `getAllMarkdownFiles` (`links_scanner.go`). The ONE walker definition per
/// CLI (plan DD-3), shared by the links gate, the heading-hierarchy validator
/// (`super::heading_hierarchy`), and the mermaid command walkers
/// (`commands::docs::{collect_md_files, collect_md_default_dirs}`). The Go
/// twin still keeps a separate cmd-level `walkMDFiles` with the historical
/// three-dir skip set; convergence on this shared walker is planned there.
pub(crate) fn get_all_markdown_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    // filepath.Walk yields lexical order; WalkDir.sort_by_file_name matches it.
    for entry in WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| {
            // Never filter the walk root itself (depth 0), only descendants.
            e.depth() == 0
                || !(e.file_type().is_dir()
                    && e.file_name()
                        .to_str()
                        .is_some_and(|name| NOISE_DIRS.contains(&name)))
        })
        .filter_map(std::result::Result::ok)
    {
        let path = entry.path();
        if entry.file_type().is_file() && path.to_string_lossy().ends_with(".md") {
            files.push(path.to_path_buf());
        }
    }

    files
}

/// Extracts markdown links from a file with line numbers. Mirrors Go `ExtractLinks`.
pub fn extract_links(file_path: &Path) -> Result<Vec<LinkInfo>, Error> {
    let file = std::fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut links = Vec::new();
    let mut fences = FenceTracker::new();

    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        let line_number = idx + 1;

        // Skip fenced code (``` and ~~~) using CommonMark open/close pairing
        // (see `super::fences`). This replaced the historical ```-only naive
        // toggle (shared with the Go scanner, fixed identically in parallel)
        // which desynced on nested example fences and extracted false links.
        if fences.consume_line(&line) {
            continue;
        }

        for caps in LINK_REGEX.captures_iter(&line) {
            let Some(url_match) = caps.get(2) else {
                continue;
            };
            // Strip angle brackets if present (markdown autolink syntax).
            let url = url_match.as_str().trim_matches(|c| c == '<' || c == '>');

            // Skip external URLs and mailto. Pure-anchor links (`#fragment`)
            // ARE extracted so same-file anchors reach validation.
            if url.starts_with("http://")
                || url.starts_with("https://")
                || url.starts_with("mailto:")
            {
                continue;
            }

            // Skip placeholder/example/Hugo paths.
            if should_skip_link(url) {
                continue;
            }

            links.push(LinkInfo {
                line_number,
                url: url.to_string(),
                is_relative: !url.starts_with('/'),
            });
        }
    }

    Ok(links)
}

/// Placeholder substrings that mark a link as a documentation example, not a
/// real target. Mirrors Go `placeholders`.
const PLACEHOLDERS: &[&str] = &[
    "path.md",
    "target",
    "link",
    "./path/to/",
    "../path/to/",
    "path/to/convention.md",
    "path/to/practice.md",
    "path/to/rule.md",
    "./relative/path/to/",
];

/// Example file-name substrings that are clearly illustrative. Mirrors Go `examplePatterns`.
const EXAMPLE_PATTERNS: &[&str] = &[
    "./overview",
    "./guide.md",
    "./examples.md",
    "./reference.md",
    "./diagram.png",
    "./image.png",
    "./screenshots/",
    "./auth-guide.md",
    "by-concept/beginner",
    "./by-example/beginner",
    "swe/prog-lang/",
    "../parent",
    "./ai/",
    "../swe/",
    "../../advanced/",
    "url",
    "./LICENSE",
    "../../features.md",
    "../../.opencode/",
];

/// Determines if a link should be skipped during validation. Mirrors Go `ShouldSkipLink`.
pub fn should_skip_link(link: &str) -> bool {
    // Skip Hugo absolute paths.
    if link.starts_with('/') {
        return true;
    }

    // Skip Hugo shortcodes.
    if link.contains("{{<") || link.contains("{{%") {
        return true;
    }

    // Skip obvious placeholder patterns.
    for placeholder in PLACEHOLDERS {
        if link.contains(placeholder) {
            return true;
        }
    }

    // Skip links with template placeholders in square brackets.
    if BRACKET_PLACEHOLDER_RE.is_match(link) {
        return true;
    }

    // Skip links that are just "path", "target", or "link".
    if link == "path" || link == "target" || link == "link" {
        return true;
    }

    // Skip example image paths.
    if link.contains("/images/") && !link.starts_with("../") {
        return true;
    }

    // Skip example file names.
    for pattern in EXAMPLE_PATTERNS {
        if link.contains(pattern) {
            return true;
        }
    }

    false
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn should_skip_absolute_and_shortcodes() {
        assert!(should_skip_link("/hugo/path"));
        assert!(should_skip_link("{{< ref foo >}}"));
        assert!(should_skip_link("{{% something %}}"));
    }

    #[test]
    fn should_skip_placeholders() {
        assert!(should_skip_link("path.md"));
        assert!(should_skip_link("./path/to/x.md"));
        assert!(should_skip_link("[some-id]"));
        assert!(should_skip_link("path"));
        assert!(should_skip_link("target"));
        assert!(should_skip_link("link"));
    }

    #[test]
    fn should_skip_images_unless_parent_relative() {
        assert!(should_skip_link("docs/images/x.png"));
        assert!(!should_skip_link("../images/x.png"));
    }

    #[test]
    fn should_skip_example_patterns() {
        assert!(should_skip_link("./guide.md"));
        assert!(should_skip_link("../../features.md"));
        assert!(should_skip_link("../../.opencode/foo.md"));
    }

    #[test]
    fn should_not_skip_real_link() {
        assert!(!should_skip_link("./real-doc.md"));
        assert!(!should_skip_link("../sibling/real.md"));
    }

    #[test]
    fn extract_links_skips_code_blocks_and_external() {
        let dir = tempfile::tempdir().unwrap();
        let f = dir.path().join("a.md");
        let mut file = std::fs::File::create(&f).unwrap();
        writeln!(file, "# Title").unwrap();
        writeln!(file, "See [doc](./real.md) and [ext](https://example.com).").unwrap();
        writeln!(file, "```").unwrap();
        writeln!(file, "[incode](./skip.md)").unwrap();
        writeln!(file, "```").unwrap();
        writeln!(file, "[anchor](#section)").unwrap();
        let links = extract_links(&f).unwrap();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].url, "./real.md");
        assert_eq!(links[0].line_number, 2);
        assert!(links[0].is_relative);
        // Pure-anchor links are extracted so same-file anchors get validated.
        assert_eq!(links[1].url, "#section");
        assert_eq!(links[1].line_number, 6);
        assert!(links[1].is_relative);
    }

    #[test]
    fn extract_links_skips_links_inside_nested_fences() {
        // A ````markdown example fence containing inner ``` fences: nothing
        // inside the outer fence is a real link. The naive toggle desyncs on
        // the inner ``` lines and would extract the link on line 5.
        let dir = tempfile::tempdir().unwrap();
        let f = dir.path().join("nested.md");
        let content = "[before](./real.md)\n\n````markdown\n```json\n[inside](./missing.md)\n```\n````\n\n[after](./other.md)\n";
        std::fs::write(&f, content).unwrap();
        let links = extract_links(&f).unwrap();
        let urls: Vec<&str> = links.iter().map(|l| l.url.as_str()).collect();
        assert_eq!(urls, vec!["./real.md", "./other.md"]);
        assert_eq!(links[1].line_number, 9);
    }

    #[test]
    fn extract_links_recognizes_tilde_fences() {
        // ~~~ opens a fence; a ``` line inside it is content, not a closer.
        // The naive ```-only toggle would extract both inner links.
        let dir = tempfile::tempdir().unwrap();
        let f = dir.path().join("tilde.md");
        let content = "~~~text\n[inside](./missing.md)\n```\n[also inside](./missing-too.md)\n```\n~~~\n\n[after](./real.md)\n";
        std::fs::write(&f, content).unwrap();
        let links = extract_links(&f).unwrap();
        let urls: Vec<&str> = links.iter().map(|l| l.url.as_str()).collect();
        assert_eq!(urls, vec!["./real.md"]);
    }

    #[test]
    fn extract_links_strips_angle_brackets() {
        let dir = tempfile::tempdir().unwrap();
        let f = dir.path().join("b.md");
        std::fs::write(&f, "[x](<./real.md>)\n").unwrap();
        let links = extract_links(&f).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].url, "./real.md");
    }

    #[test]
    fn filter_skip_paths_removes_matching() {
        let root = PathBuf::from("/repo");
        let files = vec![
            PathBuf::from("/repo/.opencode/skill/x.md"),
            PathBuf::from("/repo/docs/y.md"),
        ];
        let out = filter_skip_paths(files, &root, &[".opencode/skill/".to_string()]);
        assert_eq!(out, vec![PathBuf::from("/repo/docs/y.md")]);
    }

    #[test]
    fn filter_skip_paths_empty_returns_all() {
        let root = PathBuf::from("/repo");
        let files = vec![PathBuf::from("/repo/docs/y.md")];
        let out = filter_skip_paths(files.clone(), &root, &[]);
        assert_eq!(out, files);
    }

    #[test]
    fn get_markdown_files_walks_repo_wide_and_skips_noise_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        for sub in [
            "libs/my-lib",
            "docs",
            "node_modules/some-pkg",
            "generated-reports",
            "worktrees/copy/docs",
        ] {
            std::fs::create_dir_all(root.join(sub)).unwrap();
        }
        std::fs::write(root.join("libs/my-lib/README.md"), "[bad](./missing.md)\n").unwrap();
        std::fs::write(root.join("docs/a.md"), "ok\n").unwrap();
        std::fs::write(root.join("node_modules/some-pkg/README.md"), "skip\n").unwrap();
        std::fs::write(root.join("generated-reports/report.md"), "skip\n").unwrap();
        std::fs::write(root.join("worktrees/copy/docs/a.md"), "skip\n").unwrap();

        let opts = ScanOptions {
            repo_root: root.to_path_buf(),
            ..Default::default()
        };
        let files = get_markdown_files(&opts).unwrap();
        let rels: Vec<String> = files
            .iter()
            .map(|f| f.strip_prefix(root).unwrap().to_string_lossy().into_owned())
            .collect();

        // Repo-wide walk must reach beyond the historical 3-dir set.
        assert!(
            rels.contains(&"libs/my-lib/README.md".to_string()),
            "expected libs/ file in scan set, got {rels:?}"
        );
        assert!(rels.contains(&"docs/a.md".to_string()));

        // Standardized noise dirs must be skipped by name.
        for noise in ["node_modules/", "generated-reports/", "worktrees/"] {
            assert!(
                !rels.iter().any(|r| r.starts_with(noise)),
                "noise dir {noise} leaked into scan set: {rels:?}"
            );
        }
    }

    #[test]
    fn clean_path_strips_trailing_slash() {
        assert_eq!(clean_path(".opencode/skill/"), ".opencode/skill");
        assert_eq!(clean_path("/"), ".");
    }
}
