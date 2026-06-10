//! Secret-file discovery walk.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/internal/envbackup/discover.go`.
//! Walks the repo root collecting all secret files: `.env*` files, `secrets.json`,
//! `*.pem`/`*.key`/`*.crt`/`*.pfx` certificates, and any file under `.secrets/`.
//! Records symlinks and oversized files as skipped entries. Results are sorted
//! by `rel_path` for deterministic ordering (matching Go's `sort.Slice` on `RelPath`).

use std::collections::HashSet;
use std::path::Path;

use anyhow::{Context, Error};
use walkdir::WalkDir;

use super::types::{DEFAULT_MAX_SIZE, FileEntry};

/// Options for a discovery walk. Mirrors the subset of Go `Options` that
/// [`discover`] consumes (`RepoRoot`, `SkipDirs`, `MaxSize`).
pub struct DiscoverOptions<'a> {
    pub repo_root: &'a str,
    pub skip_dirs: &'a [&'a str],
    pub max_size: i64,
}

/// Returns true if a file at `rel_path` with `base` filename is a secret file
/// that should be backed up or restored. Mirrors Go `isSecretFile`.
pub fn is_secret_file(base: &str, rel_path: &str) -> bool {
    if base.starts_with(".env") {
        return true;
    }
    if base == "secrets.json" {
        return true;
    }
    // *.pem / *.key / *.crt / *.pfx certificate and key files
    if let Some(ext) = Path::new(base).extension().and_then(|e| e.to_str())
        && matches!(ext, "pem" | "key" | "crt" | "pfx")
    {
        return true;
    }
    // Any file descended into via .secrets/ (carved out of hidden-dir skip)
    if rel_path.starts_with(".secrets/") || rel_path.starts_with(".secrets\\") {
        return true;
    }
    // *.tfvars and inventory files — activate when IaC is added (R3/R11)
    // if base.ends_with(".tfvars") { return true; }
    false
}

/// Walks `opts.repo_root` and returns all secret files found, including those
/// skipped (symlinks, oversized). Mirrors Go `Discover`.
pub fn discover(opts: &DiscoverOptions) -> Result<Vec<FileEntry>, Error> {
    let max_size = if opts.max_size <= 0 {
        DEFAULT_MAX_SIZE
    } else {
        opts.max_size
    };

    let skip_set: HashSet<&str> = opts.skip_dirs.iter().copied().collect();
    let root = Path::new(opts.repo_root);

    let mut entries: Vec<FileEntry> = Vec::new();

    // `filter_entry` prunes directories exactly as Go's filepath.SkipDir does:
    // hidden dirs (basename starts with ".") and skip-set members are not descended
    // into. Exception: ".secrets/" is carved out so secret files inside it are found.
    // The root itself is never pruned.
    let walker = WalkDir::new(root).sort_by_file_name().into_iter();
    let it = walker.filter_entry(|e| {
        if e.file_type().is_dir() {
            let path = e.path();
            if path == root {
                return true;
            }
            let base = e.file_name().to_string_lossy();
            // Carve out .secrets/ so its contents are discoverable.
            if base.as_ref() == ".secrets" {
                return true;
            }
            if base.starts_with('.') {
                return false;
            }
            if skip_set.contains(base.as_ref()) {
                return false;
            }
            return true;
        }
        true
    });

    for res in it {
        let entry = res.with_context(|| format!("walk {}", opts.repo_root))?;
        if entry.file_type().is_dir() {
            continue;
        }

        let path = entry.path();
        let base = entry.file_name().to_string_lossy();

        let rel_path = path
            .strip_prefix(root)
            .map(|p| p.to_string_lossy().into_owned())
            .with_context(|| format!("compute relative path for {}", path.to_string_lossy()))?;

        // Apply widened allowlist: .env*, secrets.json, cert files, .secrets/ contents.
        if !is_secret_file(&base, &rel_path) {
            continue;
        }

        // Use symlink_metadata (lstat) to detect symlinks without following them.
        let meta = std::fs::symlink_metadata(path)
            .with_context(|| format!("lstat {}", path.to_string_lossy()))?;

        if meta.file_type().is_symlink() {
            entries.push(FileEntry {
                rel_path,
                abs_path: path.to_string_lossy().into_owned(),
                size: 0,
                skipped: true,
                reason: "symlink".to_string(),
                source: String::new(),
            });
            continue;
        }

        // i64 cast: file sizes for .env files are tiny; u64→i64 cannot wrap.
        let size = meta.len() as i64;
        if size > max_size {
            entries.push(FileEntry {
                rel_path,
                abs_path: path.to_string_lossy().into_owned(),
                size,
                skipped: true,
                reason: "exceeds 1 MB".to_string(),
                source: String::new(),
            });
            continue;
        }

        entries.push(FileEntry::new(
            rel_path,
            path.to_string_lossy().into_owned(),
            size,
        ));
    }

    entries.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    Ok(entries)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::internal::envbackup::types::DEFAULT_SKIP_DIRS;

    fn write(root: &std::path::Path, rel: &str, content: &[u8]) {
        let p = root.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, content).unwrap();
    }

    #[test]
    fn discovers_env_files_sorted_and_skips_autogen() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(root, ".env", b"A=1\n");
        write(root, "apps/web/.env.local", b"B=1\n");
        write(root, "node_modules/.env", b"IGNORED\n");
        write(root, "apps/web/node_modules/.env", b"IGNORED\n");
        write(root, "README.md", b"# not env\n");

        let entries = discover(&DiscoverOptions {
            repo_root: &root.to_string_lossy(),
            skip_dirs: DEFAULT_SKIP_DIRS,
            max_size: 0,
        })
        .unwrap();

        let rels: Vec<&str> = entries.iter().map(|e| e.rel_path.as_str()).collect();
        assert_eq!(rels, vec![".env", "apps/web/.env.local"]);
    }

    #[test]
    fn marks_oversized_as_skipped() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let big = vec![b'X'; 1024 * 1024 + 8];
        write(root, ".env.big", &big);
        write(root, ".env", b"ok\n");

        let entries = discover(&DiscoverOptions {
            repo_root: &root.to_string_lossy(),
            skip_dirs: &[],
            max_size: super::DEFAULT_MAX_SIZE,
        })
        .unwrap();
        let big = entries.iter().find(|e| e.rel_path == ".env.big").unwrap();
        assert!(big.skipped);
        assert_eq!(big.reason, "exceeds 1 MB");
    }

    #[test]
    fn discovers_secrets_json() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(root, "secrets.json", b"{}\n");
        write(root, ".env", b"A=1\n");

        let entries = discover(&DiscoverOptions {
            repo_root: &root.to_string_lossy(),
            skip_dirs: DEFAULT_SKIP_DIRS,
            max_size: 0,
        })
        .unwrap();

        let rels: Vec<&str> = entries.iter().map(|e| e.rel_path.as_str()).collect();
        assert!(
            rels.contains(&"secrets.json"),
            "expected secrets.json in {rels:?}"
        );
    }

    #[test]
    fn discovers_pem_file() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(root, "cert.pem", b"-----BEGIN CERT-----\n");
        write(root, ".env", b"A=1\n");

        let entries = discover(&DiscoverOptions {
            repo_root: &root.to_string_lossy(),
            skip_dirs: DEFAULT_SKIP_DIRS,
            max_size: 0,
        })
        .unwrap();

        let rels: Vec<&str> = entries.iter().map(|e| e.rel_path.as_str()).collect();
        assert!(rels.contains(&"cert.pem"), "expected cert.pem in {rels:?}");
    }

    #[test]
    fn discovers_secrets_dir_files() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(root, ".secrets/notes.md", b"secret notes\n");
        write(root, ".env", b"A=1\n");

        let entries = discover(&DiscoverOptions {
            repo_root: &root.to_string_lossy(),
            skip_dirs: DEFAULT_SKIP_DIRS,
            max_size: 0,
        })
        .unwrap();

        let rels: Vec<&str> = entries.iter().map(|e| e.rel_path.as_str()).collect();
        assert!(
            rels.contains(&".secrets/notes.md"),
            "expected .secrets/notes.md in {rels:?}"
        );
    }

    #[test]
    fn git_dir_still_skipped() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(root, ".git/config", b"[core]\n");
        write(root, ".env", b"A=1\n");

        let entries = discover(&DiscoverOptions {
            repo_root: &root.to_string_lossy(),
            skip_dirs: DEFAULT_SKIP_DIRS,
            max_size: 0,
        })
        .unwrap();

        let rels: Vec<&str> = entries.iter().map(|e| e.rel_path.as_str()).collect();
        assert!(
            !rels.iter().any(|r| r.starts_with(".git/")),
            ".git/ should be skipped"
        );
        assert!(rels.contains(&".env"));
    }

    #[cfg(unix)]
    #[test]
    fn marks_symlink_as_skipped() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(root, ".env", b"target\n");
        std::os::unix::fs::symlink(root.join(".env"), root.join(".env.link")).unwrap();

        let entries = discover(&DiscoverOptions {
            repo_root: &root.to_string_lossy(),
            skip_dirs: &[],
            max_size: 0,
        })
        .unwrap();
        let link = entries.iter().find(|e| e.rel_path == ".env.link").unwrap();
        assert!(link.skipped);
        assert_eq!(link.reason, "symlink");
    }
}
