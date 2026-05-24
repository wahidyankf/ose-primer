use std::path::PathBuf;

use anyhow::{Error, anyhow};

/// Returns the absolute path to the git repository root.
///
/// Mirrors the Go `findGitRoot`: starts at the current working directory and
/// walks up the directory tree looking for a `.git` entry (either a directory
/// for a normal checkout or a file for a worktree/submodule). Returns an error
/// matching Go's `.git directory not found` when the filesystem root is reached
/// without finding one.
pub fn find_root() -> std::result::Result<PathBuf, Error> {
    let mut dir = std::env::current_dir()?;
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
