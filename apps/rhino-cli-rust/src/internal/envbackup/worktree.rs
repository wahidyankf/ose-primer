//! Git worktree detection for `--worktree-aware`.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/internal/envbackup/worktree.go`.

use std::path::Path;

use anyhow::{Context, Error, anyhow};

/// Information about a git worktree or repository. Mirrors Go `WorktreeInfo`.
#[derive(Debug)]
pub struct WorktreeInfo {
    /// True when `repoRoot/.git` is a file (linked worktree).
    pub is_worktree: bool,
    /// Basename of `repo_root` in both worktree and normal-repo cases.
    pub worktree_name: String,
}

/// Inspects `repo_root` to determine whether it is a linked git worktree or a
/// normal repository. Mirrors Go `DetectWorktree`.
pub fn detect_worktree(repo_root: &str) -> Result<WorktreeInfo, Error> {
    let git_path = Path::new(repo_root).join(".git");

    let meta = match std::fs::symlink_metadata(&git_path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(anyhow!("no .git found at {repo_root}"));
        }
        Err(e) => return Err(e).context("stat .git"),
    };

    let name = Path::new(repo_root)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();

    // Normal repository: .git is a directory.
    if meta.file_type().is_dir() {
        return Ok(WorktreeInfo {
            is_worktree: false,
            worktree_name: name,
        });
    }

    // Linked worktree: .git is a file containing "gitdir: <path>".
    let data = std::fs::read_to_string(&git_path).context("read .git file")?;
    let line = data.trim();
    if !line.starts_with("gitdir:") {
        return Err(anyhow!(
            ".git file does not start with 'gitdir:' (got: {line:?})"
        ));
    }

    Ok(WorktreeInfo {
        is_worktree: true,
        worktree_name: name,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn normal_repo_git_dir() {
        let parent = tempfile::tempdir().unwrap();
        let repo = parent.path().join("myrepo");
        std::fs::create_dir_all(repo.join(".git")).unwrap();
        let info = detect_worktree(&repo.to_string_lossy()).unwrap();
        assert!(!info.is_worktree);
        assert_eq!(info.worktree_name, "myrepo");
    }

    #[test]
    fn linked_worktree_git_file() {
        let parent = tempfile::tempdir().unwrap();
        let repo = parent.path().join("wt-feature");
        std::fs::create_dir_all(&repo).unwrap();
        std::fs::write(repo.join(".git"), "gitdir: /somewhere/.git/worktrees/wt\n").unwrap();
        let info = detect_worktree(&repo.to_string_lossy()).unwrap();
        assert!(info.is_worktree);
        assert_eq!(info.worktree_name, "wt-feature");
    }

    #[test]
    fn missing_git_errors() {
        let tmp = tempfile::tempdir().unwrap();
        let err = detect_worktree(&tmp.path().to_string_lossy()).unwrap_err();
        assert!(err.to_string().starts_with("no .git found at"));
    }

    #[test]
    fn bad_git_file_errors() {
        let parent = tempfile::tempdir().unwrap();
        let repo = parent.path().join("r");
        std::fs::create_dir_all(&repo).unwrap();
        std::fs::write(repo.join(".git"), "not-a-gitdir\n").unwrap();
        let err = detect_worktree(&repo.to_string_lossy()).unwrap_err();
        assert!(err.to_string().contains("does not start with 'gitdir:'"));
    }
}
