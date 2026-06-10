//! Environment-file backup and restore subsystem.
//!
//! Port of `apps/rhino-cli-go/internal/envbackup/`. Provides `.env` discovery,
//! config-file discovery, worktree detection, the backup/restore engine, and
//! the text/JSON/markdown reporters consumed by the `env` command family.

pub mod config;
pub mod confirm;
pub mod discover;
pub mod ops;
pub mod reporter;
pub mod types;
pub mod worktree;

pub use config::{DEFAULT_CONFIG_PATTERNS, discover_config};
pub use confirm::{default_confirm, find_existing};
pub use discover::{DiscoverOptions, discover};
pub use ops::{Options, backup, default_backup_dir_name, expand_tilde, restore};
pub use reporter::{format_json, format_markdown, format_text};
pub use types::{DEFAULT_BACKUP_DIR, DEFAULT_MAX_SIZE, DEFAULT_SKIP_DIRS, EnvResult, FileEntry};
pub use worktree::{WorktreeInfo, detect_worktree};
