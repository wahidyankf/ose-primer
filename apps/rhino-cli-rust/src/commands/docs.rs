//! `docs validate-links`, `docs validate-mermaid`, and
//! `docs validate-heading-hierarchy` commands.
//!
//! Byte-for-byte ports of the Go `cmd/docs_validate_links.go` and
//! `cmd/docs_validate_mermaid.go` handlers (the heading-hierarchy command is
//! Rust-canonical; its Go cobra twin mirrors these bytes). Output is written
//! with `print!` (no implicit trailing newline) to mirror Go's `Fprint`.

use std::path::PathBuf;

use anyhow::{Error, anyhow};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::docs::{
    heading_hierarchy, reporter as link_reporter, types::ScanOptions, validator,
};
use crate::internal::git;
use crate::internal::mermaid::{
    extractor, reporter as mermaid_reporter, validator as mermaid_validator,
};

// ---------------------------------------------------------------------------
// validate-links
// ---------------------------------------------------------------------------

/// Cobra-style usage block printed to stderr when `validate-links` errors.
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

/// Cobra-style usage block printed to stderr when `validate-mermaid` errors.
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
--changed-only             only validate files changed since upstream (pre-push use)\n  \
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

/// Returns `*.md` files staged in git. Mirrors Go `getMermaidStagedFiles`.
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

/// Returns `*.md` files changed since upstream. Mirrors Go `getMermaidChangedFiles`.
/// On no-upstream or empty result, falls back to the default directory scan.
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

/// Converts relative paths to absolute, keeping only `*.md`. Mirrors Go `filterMDPaths`.
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

/// Walks given paths (files or dirs) collecting `*.md`. Mirrors Go `collectMDFiles`.
fn collect_md_files(repo_root: &std::path::Path, paths: &[String]) -> Result<Vec<PathBuf>, Error> {
    let mut files = Vec::new();
    for p in paths {
        let abs = if std::path::Path::new(p).is_absolute() {
            PathBuf::from(p)
        } else {
            repo_root.join(p)
        };
        files.extend(walk_md_files(&abs));
    }
    Ok(files)
}

/// Scans the default directories plus root `*.md`. Mirrors Go `collectMDDefaultDirs`.
fn collect_md_default_dirs(repo_root: &std::path::Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for dir in ["docs", "repo-governance", ".claude", "plans"] {
        let dir_path = repo_root.join(dir);
        files.extend(walk_md_files(&dir_path));
    }
    // Root *.md files (filepath.Glob — lexical).
    let mut root_mds: Vec<PathBuf> = std::fs::read_dir(repo_root)
        .into_iter()
        .flatten()
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.to_string_lossy().ends_with(".md"))
        .collect();
    root_mds.sort();
    files.extend(root_mds);
    files
}

/// Returns all `*.md` files under `dir` recursively, skipping build-artifact
/// dirs (`.next`, `node_modules`, `.git`). Mirrors Go `walkMDFiles`.
fn walk_md_files(dir: &std::path::Path) -> Vec<PathBuf> {
    use walkdir::WalkDir;
    let mut files = Vec::new();
    for entry in WalkDir::new(dir)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| {
            !(e.file_type().is_dir()
                && matches!(
                    e.file_name().to_str(),
                    Some(".next" | "node_modules" | ".git")
                ))
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

// ---------------------------------------------------------------------------
// validate-heading-hierarchy
// ---------------------------------------------------------------------------

/// Cobra-style usage block printed to stderr when `validate-heading-hierarchy`
/// errors. Shaped exactly the way cobra generates it so the planned Go twin
/// matches byte-for-byte (same flag set and alignment as `validate-links`).
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
            max_label_len: 30,
            max_width: 4,
            max_depth: 0,
            max_subgraph_nodes: 6,
        };
        assert_eq!(args.max_label_len, 30);
        assert_eq!(args.max_width, 4);
        assert_eq!(args.max_subgraph_nodes, 6);
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
    fn walk_md_files_skips_build_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("node_modules")).unwrap();
        std::fs::write(root.join("node_modules/x.md"), "skip\n").unwrap();
        std::fs::write(root.join("keep.md"), "keep\n").unwrap();
        let files = walk_md_files(root);
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("keep.md"));
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
