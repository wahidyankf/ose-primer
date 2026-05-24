use std::path::PathBuf;

use anyhow::{Error, anyhow};

/// Returns the current working directory, mirroring Go's `os.Getwd()`.
///
/// Go prefers `$PWD` when it is absolute and refers to the same directory as
/// the syscall result (same device + inode), returning the *logical* path
/// (which preserves symlinks such as macOS's `/var` → `/private/var`). Only
/// when `$PWD` is unset/stale does it fall back to the resolved syscall path.
/// Plain `std::env::current_dir()` always returns the resolved physical path,
/// which diverges from the Go CLI on symlinked working directories (e.g.
/// `mktemp -d` under `/var/folders` on macOS). The `env backup` inside-repo
/// check compares this against a user-supplied `--dir`, so the namespaces must
/// match exactly for byte-parity.
fn getwd() -> std::io::Result<PathBuf> {
    let resolved = std::env::current_dir()?;
    if let Some(pwd) = std::env::var_os("PWD") {
        let pwd_path = PathBuf::from(&pwd);
        if pwd_path.is_absolute() {
            if let (Ok(a), Ok(b)) = (std::fs::metadata(&pwd_path), std::fs::metadata(&resolved)) {
                if same_dir(&a, &b) {
                    return Ok(pwd_path);
                }
            }
        }
    }
    Ok(resolved)
}

/// Whether two metadata values refer to the same directory (device + inode on
/// unix). On non-unix targets this conservatively compares nothing and reports
/// `false`, so the resolved path is used.
#[cfg(unix)]
fn same_dir(a: &std::fs::Metadata, b: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt as _;
    a.dev() == b.dev() && a.ino() == b.ino()
}

#[cfg(not(unix))]
fn same_dir(_a: &std::fs::Metadata, _b: &std::fs::Metadata) -> bool {
    false
}

/// Returns the absolute path to the git repository root.
///
/// Mirrors the Go `findGitRoot`: starts at the current working directory and
/// walks up the directory tree looking for a `.git` entry (either a directory
/// for a normal checkout or a file for a worktree/submodule). Returns an error
/// matching Go's `.git directory not found` when the filesystem root is reached
/// without finding one. The starting directory uses [`getwd`] so the logical
/// (`$PWD`-preserving) path is honoured exactly as Go's `os.Getwd()`.
pub fn find_root() -> std::result::Result<PathBuf, Error> {
    let mut dir = getwd()?;
    loop {
        let git_path = dir.join(".git");
        if git_path.exists() {
            return Ok(dir);
        }
        match dir.parent() {
            Some(parent) if parent != dir => dir = parent.to_path_buf(),
            _ => return Err(anyhow!(".git directory not found")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_root_returns_repo_root() {
        let root = find_root().expect("git root resolvable in test");
        assert!(
            root.join("Cargo.toml").exists() || root.join("AGENTS.md").exists(),
            "expected repo root to contain Cargo.toml or AGENTS.md, got {root:?}"
        );
    }
}
