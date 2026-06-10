use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// A single broken link found during validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrokenLink {
    /// Line number where the link appears (1-based).
    pub line_number: usize,
    /// File containing the broken link (relative to repo root).
    pub source_file: String,
    /// The actual link URL/path.
    pub link_text: String,
    /// The resolved target path that does not exist.
    pub target_path: String,
    /// Category of the broken link (for reporting).
    pub category: String,
}

/// Complete results of a link validation scan.
#[derive(Debug, Clone, Default)]
pub struct LinkValidationResult {
    /// Total number of files scanned.
    pub total_files: usize,
    /// Total number of links checked.
    pub total_links: usize,
    /// All broken links found, in discovery order.
    pub broken_links: Vec<BrokenLink>,
    /// Broken links grouped by category.
    pub broken_by_category: HashMap<String, Vec<BrokenLink>>,
    /// Time taken for the scan.
    pub scan_duration: Duration,
}

/// Configures how the link validation scan runs.
#[derive(Debug, Clone, Default)]
pub struct ScanOptions {
    /// Absolute path to repository root.
    pub repo_root: PathBuf,
    /// Only scan staged files from git.
    pub staged_only: bool,
    /// Paths to skip during scanning (relative to repo root).
    pub skip_paths: Vec<String>,
    /// Enable verbose logging.
    pub verbose: bool,
    /// Quiet mode (errors only).
    pub quiet: bool,
}

/// A link found in a markdown file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkInfo {
    /// Line number where the link appears (1-based).
    pub line_number: usize,
    /// The link URL/path.
    pub url: String,
    /// Whether the link is relative (vs absolute starting with `/`).
    pub is_relative: bool,
}
