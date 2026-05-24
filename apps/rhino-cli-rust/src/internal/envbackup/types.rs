//! Shared types for the `env backup` / `env restore` subsystem.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/internal/envbackup/types.go`.

/// A single `.env` (or config) file found or processed during a backup/restore.
///
/// Mirrors Go `FileEntry`. The `source` field is `""` for plain `.env` files
/// discovered by the walk and `"config"` for known config patterns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    /// Relative to repo root.
    pub rel_path: String,
    /// Absolute path in source location.
    pub abs_path: String,
    /// File size in bytes.
    pub size: i64,
    /// True if skipped (symlink, too large).
    pub skipped: bool,
    /// Skip reason (empty if not skipped).
    pub reason: String,
    /// `"env"` or `"config"` — empty defaults to `"env"`.
    pub source: String,
}

impl FileEntry {
    /// Constructs a non-skipped env entry.
    pub fn new(rel_path: String, abs_path: String, size: i64) -> Self {
        Self {
            rel_path,
            abs_path,
            size,
            skipped: false,
            reason: String::new(),
            source: String::new(),
        }
    }
}

/// Outcome of a backup or restore operation. Mirrors Go `Result`.
#[derive(Debug, Clone, Default)]
pub struct EnvResult {
    /// `"backup"` or `"restore"`.
    pub direction: String,
    /// Backup directory path.
    pub dir: String,
    /// All discovered files (including skipped).
    pub files: Vec<FileEntry>,
    /// Count of successfully copied files.
    pub copied: i64,
    /// Count of skipped files.
    pub skipped: i64,
    /// Non-fatal warnings.
    pub errors: Vec<String>,
    /// Worktree/repo name when `--worktree-aware` is used.
    pub worktree_name: String,
    /// True if the user declined the confirmation prompt.
    pub cancelled: bool,
}

/// Default skip-directory basenames, in the exact order from Go `DefaultSkipDirs`.
pub const DEFAULT_SKIP_DIRS: &[&str] = &[
    ".git",
    "node_modules",
    "bower_components",
    ".nx",
    ".next",
    ".turbo",
    ".cache",
    ".parcel-cache",
    ".nyc_output",
    "dist",
    "build",
    "coverage",
    "__pycache__",
    ".venv",
    "venv",
    "target",
    ".gradle",
    "vendor",
    "_build",
    "deps",
    ".elixir_ls",
    ".mix",
    ".dart_tool",
    ".cargo",
    "zig-cache",
    ".stack-work",
    "elm-stuff",
    "_deps",
    ".terraform",
    ".pulumi",
    "generated-contracts",
];

/// Maximum backup file size in bytes (1 MB). Mirrors Go `DefaultMaxSize`.
pub const DEFAULT_MAX_SIZE: i64 = 1024 * 1024;

/// Default backup directory name. Mirrors Go `DefaultBackupDir`.
pub const DEFAULT_BACKUP_DIR: &str = "ose-open-env-backup";

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn file_entry_new_defaults() {
        let e = FileEntry::new("a".to_string(), "/a".to_string(), 7);
        assert_eq!(e.rel_path, "a");
        assert_eq!(e.size, 7);
        assert!(!e.skipped);
        assert!(e.reason.is_empty());
        assert!(e.source.is_empty());
    }

    #[test]
    fn constants_match_go() {
        assert_eq!(DEFAULT_MAX_SIZE, 1024 * 1024);
        assert_eq!(DEFAULT_BACKUP_DIR, "ose-open-env-backup");
        assert_eq!(DEFAULT_SKIP_DIRS[0], ".git");
        assert!(DEFAULT_SKIP_DIRS.contains(&"node_modules"));
        assert!(DEFAULT_SKIP_DIRS.contains(&"generated-contracts"));
    }
}
