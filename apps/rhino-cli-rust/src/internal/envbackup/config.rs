//! Known uncommitted config-file discovery for `--include-config`.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/internal/envbackup/config.go`.

use std::path::Path;

use anyhow::{Context, Error};

use super::types::{DEFAULT_MAX_SIZE, FileEntry};

/// A known uncommitted config file. Mirrors Go `ConfigPattern`.
pub struct ConfigPattern {
    /// Exact relative path from repo root.
    pub rel_path: &'static str,
}

/// Known uncommitted local config files, in Go `DefaultConfigPatterns` order.
/// Only `RelPath` is consumed by discovery; the Go `Description`/`Category`
/// fields are documentation-only and never reach output, so they are omitted.
pub const DEFAULT_CONFIG_PATTERNS: &[ConfigPattern] = &[
    // AI Tools
    ConfigPattern {
        rel_path: ".claude/settings.local.json",
    },
    ConfigPattern {
        rel_path: ".claude/settings.local.json.bkup",
    },
    ConfigPattern {
        rel_path: ".cursor/mcp.json",
    },
    ConfigPattern {
        rel_path: ".windsurfrules",
    },
    ConfigPattern {
        rel_path: ".clinerules",
    },
    ConfigPattern {
        rel_path: ".aider.conf.yml",
    },
    ConfigPattern {
        rel_path: ".aiderignore",
    },
    ConfigPattern {
        rel_path: ".continue/config.json",
    },
    ConfigPattern {
        rel_path: ".gemini/settings.json",
    },
    ConfigPattern {
        rel_path: ".amazonq/mcp.json",
    },
    ConfigPattern {
        rel_path: ".roomodes",
    },
    // Docker
    ConfigPattern {
        rel_path: "docker-compose.override.yml",
    },
    // Version Managers
    ConfigPattern {
        rel_path: "mise.local.toml",
    },
    // Environment
    ConfigPattern { rel_path: ".envrc" },
];

/// Checks each pattern against `repo_root` and returns `FileEntry` items for
/// files that exist, applying the same symlink/size checks as [`super::discover`].
/// Every returned entry has `source: "config"`. Mirrors Go `DiscoverConfig`.
pub fn discover_config(
    repo_root: &str,
    patterns: &[ConfigPattern],
    max_size: i64,
) -> Result<Vec<FileEntry>, Error> {
    let max_size = if max_size <= 0 {
        DEFAULT_MAX_SIZE
    } else {
        max_size
    };

    let mut entries: Vec<FileEntry> = Vec::new();

    for p in patterns {
        let abs_path = Path::new(repo_root).join(p.rel_path);

        let meta = match std::fs::symlink_metadata(&abs_path) {
            Ok(m) => m,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue, // silently skip
            Err(e) => return Err(e).with_context(|| format!("lstat {}", p.rel_path)),
        };

        // Skip directories.
        if meta.file_type().is_dir() {
            continue;
        }

        let abs_str = abs_path.to_string_lossy().into_owned();

        // Skip symlinks.
        if meta.file_type().is_symlink() {
            entries.push(FileEntry {
                rel_path: p.rel_path.to_string(),
                abs_path: abs_str,
                size: 0,
                skipped: true,
                reason: "symlink".to_string(),
                source: "config".to_string(),
            });
            continue;
        }

        let size = meta.len() as i64;

        // Skip oversized files.
        if size > max_size {
            entries.push(FileEntry {
                rel_path: p.rel_path.to_string(),
                abs_path: abs_str,
                size,
                skipped: true,
                reason: format!("file too large ({size} bytes > {max_size})"),
                source: "config".to_string(),
            });
            continue;
        }

        entries.push(FileEntry {
            rel_path: p.rel_path.to_string(),
            abs_path: abs_str,
            size,
            skipped: false,
            reason: String::new(),
            source: "config".to_string(),
        });
    }

    entries.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    Ok(entries)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn write(root: &std::path::Path, rel: &str, content: &[u8]) {
        let p = root.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, content).unwrap();
    }

    #[test]
    fn finds_present_config_files_only() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(root, ".claude/settings.local.json", b"{}\n");
        write(root, ".envrc", b"export X=1\n");
        // .windsurfrules absent → skipped silently.

        let entries = discover_config(&root.to_string_lossy(), DEFAULT_CONFIG_PATTERNS, 0).unwrap();
        let rels: Vec<&str> = entries.iter().map(|e| e.rel_path.as_str()).collect();
        assert_eq!(rels, vec![".claude/settings.local.json", ".envrc"]);
        assert!(entries.iter().all(|e| e.source == "config"));
    }

    #[test]
    fn marks_oversized_config_skipped_with_reason() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(root, ".envrc", &[b'Z'; 20]);
        let entries = discover_config(&root.to_string_lossy(), DEFAULT_CONFIG_PATTERNS, 8).unwrap();
        let e = entries.iter().find(|e| e.rel_path == ".envrc").unwrap();
        assert!(e.skipped);
        assert_eq!(e.reason, "file too large (20 bytes > 8)");
    }

    #[test]
    fn empty_when_no_config_files() {
        let tmp = tempfile::tempdir().unwrap();
        let entries =
            discover_config(&tmp.path().to_string_lossy(), DEFAULT_CONFIG_PATTERNS, 0).unwrap();
        assert!(entries.is_empty());
    }
}
