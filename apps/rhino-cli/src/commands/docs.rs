//! `docs validate-links`, `docs validate-mermaid`, and
//! `docs validate-heading-hierarchy` commands.
//!
//! Output is written with `print!` (no implicit trailing newline).

use std::path::PathBuf;

use anyhow::{Error, anyhow};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::docs::{
    heading_hierarchy, reporter as link_reporter, scanner, types::ScanOptions, validator,
};
use crate::internal::git;
use crate::internal::mermaid::{
    extractor, reporter as mermaid_reporter, validator as mermaid_validator,
};

// ---------------------------------------------------------------------------
// validate-links
// ---------------------------------------------------------------------------

/// Usage block printed to stderr when `validate-links` errors.
pub const VALIDATE_LINKS_USAGE: &str = "Usage:\n  \
rhino-cli docs validate-links [flags]\n\n\
Examples:\n  \
# Validate all markdown files\n  \
rhino-cli docs validate-links\n\n  \
# Validate only staged files (useful in pre-commit hooks)\n  \
rhino-cli docs validate-links --staged-only\n\n  \
# Output as JSON\n  \
rhino-cli docs validate-links -o json\n\n  \
# Output as markdown report\n  \
rhino-cli docs validate-links -o markdown\n\n  \
# Verbose mode with quiet output\n  \
rhino-cli docs validate-links -v -q\n\n\
Flags:\n      \
--exclude stringArray   path prefixes to exclude from validation (repeatable)\n  \
-h, --help                  help for validate-links\n      \
--staged-only           only validate staged files\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

#[derive(Args, Debug)]
pub struct ValidateLinksArgs {
    /// Only validate staged files.
    #[arg(long = "staged-only")]
    pub staged_only: bool,
    /// Path prefixes to exclude from validation (repeatable).
    #[arg(long = "exclude", value_name = "PATH")]
    pub exclude: Vec<String>,
}

pub fn run_validate_links(
    args: &ValidateLinksArgs,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    // Exclude auto-generated skill files (matches Go), then append any
    // caller-provided `--exclude` prefixes AFTER the baked-in entry.
    let mut skip_paths = vec![".opencode/skill/".to_string()];
    skip_paths.extend(args.exclude.iter().cloned());

    let opts = ScanOptions {
        repo_root,
        staged_only: args.staged_only,
        skip_paths,
        verbose,
        quiet,
    };

    let result =
        validator::validate_all_links(&opts).map_err(|e| anyhow!("validation failed: {e}"))?;

    let out = match output {
        OutputFormat::Text => link_reporter::format_link_text(&result, verbose, quiet),
        OutputFormat::Json => link_reporter::format_link_json(&result)?,
        OutputFormat::Markdown => link_reporter::format_link_markdown(&result),
    };
    print!("{out}");

    if !result.broken_links.is_empty() {
        if !quiet && matches!(output, OutputFormat::Text) {
            eprintln!("\n❌ Found {} broken links", result.broken_links.len());
        }
        return Err(anyhow!("found {} broken links", result.broken_links.len()));
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// validate-mermaid
// ---------------------------------------------------------------------------

/// Usage block printed to stderr when `validate-mermaid` errors.
pub const VALIDATE_MERMAID_USAGE: &str = "Usage:\n  \
rhino-cli docs validate-mermaid [flags]\n\n\
Examples:\n  \
# Validate all markdown files in default directories\n  \
rhino-cli docs validate-mermaid\n\n  \
# Validate specific files or directories\n  \
rhino-cli docs validate-mermaid docs/ repo-governance/\n\n  \
# Only validate files staged in git (pre-commit use)\n  \
rhino-cli docs validate-mermaid --staged-only\n\n  \
# Only validate files changed since upstream (pre-push use)\n  \
rhino-cli docs validate-mermaid --changed-only\n\n  \
# Output as JSON\n  \
rhino-cli docs validate-mermaid -o json\n\n  \
# Set custom thresholds\n  \
rhino-cli docs validate-mermaid --max-label-len 20 --max-width 4\n\n\
Flags:\n      \
--changed-only             only validate files changed since upstream (pre-push use)\n      \
--exclude stringArray      path prefixes to exclude from validation (repeatable)\n  \
-h, --help                     help for validate-mermaid\n      \
--max-depth int            depth threshold for the both-exceeded warning: when span>max-width AND depth>max-depth, emit warning not error\n      \
--max-label-len int        max characters in a node label (default 30 ~ Mermaid wrappingWidth:200px at 16px font) (default 30)\n      \
--max-subgraph-nodes int   max direct child nodes per subgraph; emits a subgraph_density warning when exceeded (default 6)\n      \
--max-width int            max nodes at the same rank (default 4)\n      \
--staged-only              only validate staged files (pre-commit use)\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

#[derive(Args, Debug)]
pub struct ValidateMermaidArgs {
    /// Optional path arguments (files or directories).
    #[arg(value_name = "PATH")]
    pub paths: Vec<String>,
    /// Only validate staged files (pre-commit use).
    #[arg(long = "staged-only")]
    pub staged_only: bool,
    /// Only validate files changed since upstream (pre-push use).
    #[arg(long = "changed-only")]
    pub changed_only: bool,
    /// Path prefixes to exclude from validation (repeatable).
    #[arg(long = "exclude", value_name = "PATH")]
    pub exclude: Vec<String>,
    /// Max characters in a node label.
    #[arg(long = "max-label-len", default_value_t = 30)]
    pub max_label_len: i64,
    /// Max nodes at the same rank.
    #[arg(long = "max-width", default_value_t = 4)]
    pub max_width: i64,
    /// Depth threshold for the both-exceeded warning.
    #[arg(long = "max-depth", default_value_t = 0)]
    pub max_depth: i64,
    /// Max direct child nodes per subgraph.
    #[arg(long = "max-subgraph-nodes", default_value_t = 6)]
    pub max_subgraph_nodes: i64,
}

pub fn run_validate_mermaid(
    args: &ValidateMermaidArgs,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    // Resolve file list.
    let md_files: Vec<PathBuf> = if args.staged_only {
        get_mermaid_staged_files(&repo_root)
            .map_err(|e| anyhow!("failed to get staged files: {e}"))?
    } else if args.changed_only {
        get_mermaid_changed_files(&repo_root)
            .map_err(|e| anyhow!("failed to get changed files: {e}"))?
    } else if !args.paths.is_empty() {
        collect_md_files(&repo_root, &args.paths)
            .map_err(|e| anyhow!("failed to collect files: {e}"))?
    } else {
        collect_md_default_dirs(&repo_root)
    };

    // Apply `--exclude` prefix filtering to the collected set (plan DD-2).
    let md_files = filter_mermaid_excluded(&repo_root, md_files, &args.exclude);

    // Extract and validate blocks.
    let mut all_blocks = Vec::new();
    let mut file_set = std::collections::HashSet::new();
    for f in &md_files {
        let Ok(content) = std::fs::read_to_string(f) else {
            continue;
        };
        let blocks = extractor::extract_blocks(&f.to_string_lossy(), &content);
        if !blocks.is_empty() {
            file_set.insert(f.clone());
        }
        all_blocks.extend(blocks);
    }

    // Go: if MaxDepth == 0, set to math.MaxInt.
    let max_depth = if args.max_depth == 0 {
        i64::MAX
    } else {
        args.max_depth
    };
    let opts = mermaid_validator::ValidateOptions {
        max_label_len: args.max_label_len,
        max_width: args.max_width,
        max_depth,
        max_subgraph_nodes: args.max_subgraph_nodes,
    };
    let mut result = mermaid_validator::validate_blocks(&all_blocks, opts);
    result.files_scanned = file_set.len();

    let out = match output {
        OutputFormat::Text => mermaid_reporter::format_text(&result, verbose, quiet),
        OutputFormat::Json => mermaid_reporter::format_json(&result)?,
        OutputFormat::Markdown => mermaid_reporter::format_markdown(&result),
    };
    print!("{out}");

    if !result.violations.is_empty() {
        return Err(anyhow!("found {} violation(s)", result.violations.len()));
    }
    Ok(())
}

/// Returns `*.md` files staged in git.
fn get_mermaid_staged_files(repo_root: &std::path::Path) -> Result<Vec<PathBuf>, Error> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("diff")
        .arg("--cached")
        .arg("--name-only")
        .arg("--diff-filter=ACMR")
        .output()?;
    if !output.status.success() {
        return Err(anyhow!("git diff --cached failed"));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let rel: Vec<String> = stdout.trim().lines().map(str::to_string).collect();
    Ok(filter_md_paths(repo_root, &rel))
}

/// Returns `*.md` files changed since upstream.
/// On no-upstream or empty result, falls back to the default repo-wide scan.
fn get_mermaid_changed_files(repo_root: &std::path::Path) -> Result<Vec<PathBuf>, Error> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("diff")
        .arg("--name-only")
        .arg("@{u}..HEAD")
        .output();
    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let rel: Vec<String> = stdout.trim().lines().map(str::to_string).collect();
            let files = filter_md_paths(repo_root, &rel);
            if files.is_empty() {
                Ok(collect_md_default_dirs(repo_root))
            } else {
                Ok(files)
            }
        }
        // No upstream (or git error): fall back to default scan.
        _ => Ok(collect_md_default_dirs(repo_root)),
    }
}

/// Converts relative paths to absolute, keeping only `*.md`.
fn filter_md_paths(repo_root: &std::path::Path, rel_paths: &[String]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for p in rel_paths {
        if p.is_empty() {
            continue;
        }
        if !p.ends_with(".md") {
            continue;
        }
        out.push(repo_root.join(p));
    }
    out
}

/// Walks given paths (files or dirs) collecting `*.md`.
/// Delegates to the links scanner's walker (`scanner::get_all_markdown_files`)
/// — the single noise-skipping walk definition per CLI (plan DD-3). A file
/// path yields itself at depth 0 (never filtered).
fn collect_md_files(repo_root: &std::path::Path, paths: &[String]) -> Result<Vec<PathBuf>, Error> {
    let mut files = Vec::new();
    for p in paths {
        let abs = if std::path::Path::new(p).is_absolute() {
            PathBuf::from(p)
        } else {
            repo_root.join(p)
        };
        files.extend(scanner::get_all_markdown_files(&abs));
    }
    Ok(files)
}

/// Scans the whole repository for `*.md` files (plan DD-3): a repo-wide walk
/// skipping the standardized noise-skip set by directory name, replacing the
/// historical four-dir default (docs/repo-governance/.claude/plans) plus root
/// glob. Delegates to `scanner::get_all_markdown_files`, the one walker per CLI.
fn collect_md_default_dirs(repo_root: &std::path::Path) -> Vec<PathBuf> {
    scanner::get_all_markdown_files(repo_root)
}

/// Applies `--exclude` prefix semantics to the collected mermaid file list
/// (plan DD-2): drops files whose repo-root-relative path starts with any
/// excluded prefix (raw or trailing-slash-cleaned). Delegates to the links
/// walker's `filter_skip_paths` so both gates share one prefix
/// implementation per CLI.
fn filter_mermaid_excluded(
    repo_root: &std::path::Path,
    files: Vec<PathBuf>,
    exclude: &[String],
) -> Vec<PathBuf> {
    scanner::filter_skip_paths(files, repo_root, exclude)
}

// ---------------------------------------------------------------------------
// validate-heading-hierarchy
// ---------------------------------------------------------------------------

/// Usage block printed to stderr when `validate-heading-hierarchy`
/// errors (same flag set and alignment as `validate-links`).
pub const VALIDATE_HEADING_HIERARCHY_USAGE: &str = "Usage:\n  \
rhino-cli docs validate-heading-hierarchy [flags]\n\n\
Examples:\n  \
# Validate heading hierarchy in all prose markdown files\n  \
rhino-cli docs validate-heading-hierarchy\n\n  \
# Validate specific files or directories (prose allowlist still applies)\n  \
rhino-cli docs validate-heading-hierarchy docs/ repo-governance/\n\n  \
# Validate only staged files (useful in pre-commit hooks)\n  \
rhino-cli docs validate-heading-hierarchy --staged-only\n\n  \
# Exclude a tree on top of the prose allowlist\n  \
rhino-cli docs validate-heading-hierarchy --exclude docs\n\n  \
# Output as JSON\n  \
rhino-cli docs validate-heading-hierarchy -o json\n\n\
Flags:\n      \
--exclude stringArray   path prefixes to exclude from validation (repeatable)\n  \
-h, --help                  help for validate-heading-hierarchy\n      \
--staged-only           only validate staged files\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

#[derive(Args, Debug)]
pub struct ValidateHeadingHierarchyArgs {
    /// Optional path arguments (files or directories); the prose allowlist
    /// still applies to every candidate.
    #[arg(value_name = "PATH")]
    pub paths: Vec<String>,
    /// Only validate staged files.
    #[arg(long = "staged-only")]
    pub staged_only: bool,
    /// Path prefixes to exclude from validation (repeatable).
    #[arg(long = "exclude", value_name = "PATH")]
    pub exclude: Vec<String>,
}

pub fn run_validate_heading_hierarchy(
    args: &ValidateHeadingHierarchyArgs,
    output: OutputFormat,
    _verbose: bool,
    quiet: bool,
) -> Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    // Resolve the candidate paths. The prose allowlist is applied inside the
    // validator's file selection (plan DD-7), so staged and positional inputs
    // can never trip a finding on default-denied trees.
    let paths: Vec<String> = if args.staged_only {
        let staged = get_mermaid_staged_files(&repo_root)
            .map_err(|e| anyhow!("failed to get staged files: {e}"))?;
        let rels: Vec<String> = staged
            .iter()
            .filter_map(|p| p.strip_prefix(&repo_root).ok())
            .map(|rel| rel.to_string_lossy().into_owned())
            .collect();
        if rels.is_empty() {
            // Nothing staged: report success without falling back to a full scan.
            print_heading_report(&[], output, quiet)?;
            return Ok(());
        }
        rels
    } else {
        args.paths.clone()
    };

    let opts = heading_hierarchy::HeadingScanOptions {
        root: repo_root,
        paths,
        exclude: args.exclude.clone(),
    };
    let findings = heading_hierarchy::validate_heading_hierarchy(&opts)
        .map_err(|e| anyhow!("validation failed: {e}"))?;

    print_heading_report(&findings, output, quiet)?;

    if !findings.is_empty() {
        if !quiet && matches!(output, OutputFormat::Text) {
            eprintln!("\n❌ Found {} heading hierarchy finding(s)", findings.len());
        }
        return Err(anyhow!(
            "found {} heading hierarchy finding(s)",
            findings.len()
        ));
    }
    Ok(())
}

/// Formats and prints the heading-hierarchy report in the requested format.
fn print_heading_report(
    findings: &[heading_hierarchy::HeadingFinding],
    output: OutputFormat,
    quiet: bool,
) -> Result<(), Error> {
    let out = match output {
        OutputFormat::Text => heading_hierarchy::format_heading_text(findings, quiet),
        OutputFormat::Json => heading_hierarchy::format_heading_json(findings)?,
        OutputFormat::Markdown => heading_hierarchy::format_heading_markdown(findings),
    };
    print!("{out}");
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn links_args_default() {
        let args = ValidateLinksArgs {
            staged_only: false,
            exclude: vec![],
        };
        assert!(!args.staged_only);
        assert!(args.exclude.is_empty());
    }

    #[test]
    fn heading_hierarchy_args_default() {
        let args = ValidateHeadingHierarchyArgs {
            paths: vec![],
            staged_only: false,
            exclude: vec![],
        };
        assert!(args.paths.is_empty());
        assert!(!args.staged_only);
        assert!(args.exclude.is_empty());
    }

    #[test]
    fn mermaid_args_defaults() {
        let args = ValidateMermaidArgs {
            paths: vec![],
            staged_only: false,
            changed_only: false,
            exclude: vec![],
            max_label_len: 30,
            max_width: 4,
            max_depth: 0,
            max_subgraph_nodes: 6,
        };
        assert_eq!(args.max_label_len, 30);
        assert_eq!(args.max_width, 4);
        assert_eq!(args.max_subgraph_nodes, 6);
        assert!(args.exclude.is_empty());
    }

    #[test]
    fn filter_md_paths_keeps_only_md() {
        let root = std::path::Path::new("/repo");
        let out = filter_md_paths(
            root,
            &[
                "docs/a.md".to_string(),
                "src/b.rs".to_string(),
                String::new(),
                "c.md".to_string(),
            ],
        );
        assert_eq!(
            out,
            vec![
                PathBuf::from("/repo/docs/a.md"),
                PathBuf::from("/repo/c.md")
            ]
        );
    }

    #[test]
    fn default_scan_skips_build_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("node_modules")).unwrap();
        std::fs::write(root.join("node_modules/x.md"), "skip\n").unwrap();
        std::fs::write(root.join("keep.md"), "keep\n").unwrap();
        let files = collect_md_default_dirs(root);
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("keep.md"));
    }

    #[test]
    fn collect_md_default_dirs_walks_repo_wide() {
        // Plan DD-3: the default mermaid scan must be a repo-wide walk, not
        // the historical four-dir set (docs/repo-governance/.claude/plans).
        // Files under trees OUTSIDE that set must be collected.
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("specs/apps")).unwrap();
        std::fs::write(root.join("specs/apps/spec.md"), "x\n").unwrap();
        std::fs::create_dir_all(root.join("libs/my-lib")).unwrap();
        std::fs::write(root.join("libs/my-lib/README.md"), "y\n").unwrap();

        let files = collect_md_default_dirs(root);
        assert!(
            files.iter().any(|p| p.ends_with("specs/apps/spec.md")),
            "repo-wide scan must collect specs/ markdown, got {files:?}"
        );
        assert!(
            files.iter().any(|p| p.ends_with("libs/my-lib/README.md")),
            "repo-wide scan must collect libs/ markdown, got {files:?}"
        );
    }

    #[test]
    fn default_scan_skips_standardized_noise_set() {
        // Plan DD-3: the walk must skip the full standardized cross-repo
        // noise-skip set by directory name (same set as scanner NOISE_DIRS),
        // not just the historical .next/node_modules/.git trio. The
        // `worktrees` skip is non-negotiable: without it a repo-wide walk
        // re-scans entire repo copies.
        let noise_dirs = [
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
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        for noise in noise_dirs {
            std::fs::create_dir_all(root.join(noise)).unwrap();
            std::fs::write(root.join(noise).join("skip.md"), "skip\n").unwrap();
        }
        std::fs::create_dir_all(root.join("docs")).unwrap();
        std::fs::write(root.join("docs/keep.md"), "keep\n").unwrap();

        let files = collect_md_default_dirs(root);
        for noise in noise_dirs {
            assert!(
                !files
                    .iter()
                    .any(|p| p.components().any(|c| c.as_os_str() == noise)),
                "noise dir {noise} leaked into the walk: {files:?}"
            );
        }
        assert_eq!(files.len(), 1, "only docs/keep.md expected, got {files:?}");
        assert!(files[0].ends_with("docs/keep.md"));
    }

    #[test]
    fn filter_mermaid_excluded_drops_excluded_prefixes() {
        // Plan DD-2: `--exclude plans/done` semantics — files whose
        // repo-relative path starts with an excluded prefix (raw or
        // trailing-slash form) are dropped from the collected set; all
        // other files survive in order.
        let root = std::path::Path::new("/repo");
        let files = vec![
            PathBuf::from("/repo/plans/done/old.md"),
            PathBuf::from("/repo/plans/in-progress/cur.md"),
            PathBuf::from("/repo/docs/a.md"),
        ];
        let out = filter_mermaid_excluded(root, files, &["plans/done".to_string()]);
        assert_eq!(
            out,
            vec![
                PathBuf::from("/repo/plans/in-progress/cur.md"),
                PathBuf::from("/repo/docs/a.md"),
            ]
        );

        // Trailing-slash exclude form behaves identically (matching the
        // links filter_skip_paths clean-path handling).
        let files = vec![
            PathBuf::from("/repo/plans/done/old.md"),
            PathBuf::from("/repo/docs/a.md"),
        ];
        let out = filter_mermaid_excluded(root, files, &["plans/done/".to_string()]);
        assert_eq!(out, vec![PathBuf::from("/repo/docs/a.md")]);

        // Empty exclude list keeps everything.
        let files = vec![PathBuf::from("/repo/docs/a.md")];
        let out = filter_mermaid_excluded(root, files.clone(), &[]);
        assert_eq!(out, files);
    }

    #[test]
    fn collect_md_default_dirs_includes_root_md() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("README.md"), "x\n").unwrap();
        std::fs::create_dir_all(root.join("docs")).unwrap();
        std::fs::write(root.join("docs/d.md"), "y\n").unwrap();
        let files = collect_md_default_dirs(root);
        assert!(files.iter().any(|p| p.ends_with("README.md")));
        assert!(files.iter().any(|p| p.ends_with("docs/d.md")));
    }
}
