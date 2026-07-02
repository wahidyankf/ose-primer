//! Git repository root locator — IO adapter for `git rev-parse --show-toplevel`.

use std::path::PathBuf;
use std::process::Command;

use anyhow::{Error, anyhow};

/// Returns the absolute path to the Git repository root from an optional working directory.
///
/// Executes `git rev-parse --show-toplevel` from `cwd` when provided, or from the
/// current working directory when `None`. Using `--show-toplevel` makes this
/// worktree-aware: from a linked worktree, it returns the worktree path, not the
/// main repo path.
///
/// When the `git` binary itself cannot be spawned (e.g. absent from `PATH`),
/// falls back to walking the filesystem tree for a `.git` entry rather than
/// failing outright. This matters for tools — like `rhino-cli doctor` — whose
/// entire purpose is to detect a missing `git` binary and report it, which
/// requires resolving the repo root *without* `git` already being available.
///
/// # Errors
///
/// Returns an error when the command fails (git found `PATH` but reported
/// failure, e.g. not a repository), the output is empty or not valid UTF-8,
/// or `git` cannot be spawned at all *and* no `.git` entry is found by
/// walking up from `cwd`.
pub fn find_root_from(cwd: Option<&std::path::Path>) -> std::result::Result<PathBuf, Error> {
    find_root_from_with(GIT_BINARY, cwd)
}

/// The `git` executable name used by [`find_root_from`]. Extracted as a
/// constant so [`find_root_from_with`] can be exercised with a deliberately
/// unresolvable binary name in tests, without mutating the process-wide
/// `PATH` environment variable (which would race with other tests).
const GIT_BINARY: &str = "git";

/// Implementation of [`find_root_from`] parameterised by the `git` binary
/// name, so tests can simulate "`git` not found in `PATH`" deterministically.
fn find_root_from_with(
    git_bin: &str,
    cwd: Option<&std::path::Path>,
) -> std::result::Result<PathBuf, Error> {
    let mut cmd = Command::new(git_bin);
    cmd.args(["rev-parse", "--show-toplevel"]);
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    match cmd.output() {
        Ok(output) => {
            if !output.status.success() {
                return Err(anyhow!(
                    "git rev-parse failed: {}",
                    String::from_utf8_lossy(&output.stderr).trim()
                ));
            }
            let path = String::from_utf8(output.stdout)?.trim().to_string();
            if path.is_empty() {
                return Err(anyhow!("git rev-parse returned empty path"));
            }
            Ok(PathBuf::from(path))
        }
        Err(spawn_err) => find_root_via_fs_walk(cwd)
            .ok_or_else(|| anyhow!("failed to invoke git rev-parse: {spawn_err}")),
    }
}

/// Walks up from `start` (or the current working directory when `None`)
/// looking for a `.git` entry — a directory for a normal checkout, or a file
/// for a linked worktree/submodule (both satisfy `.exists()`). Returns
/// `None` when the filesystem root is reached without finding one.
fn find_root_via_fs_walk(start: Option<&std::path::Path>) -> Option<PathBuf> {
    let mut dir = match start {
        Some(d) => d.to_path_buf(),
        None => std::env::current_dir().ok()?,
    };
    loop {
        if dir.join(".git").exists() {
            return Some(dir);
        }
        match dir.parent() {
            Some(parent) if parent != dir => dir = parent.to_path_buf(),
            _ => return None,
        }
    }
}

/// Returns the absolute path to the Git repository root.
///
/// Executes `git rev-parse --show-toplevel` from the current working directory.
///
/// # Errors
///
/// Returns an error when `git` is not found, the command fails, or the output
/// is empty or not valid UTF-8.
pub fn find_root() -> std::result::Result<PathBuf, Error> {
    find_root_from(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::CwdLock;
    use std::process::Command as Cmd;
    use tempfile::TempDir;

    #[test]
    fn find_root_returns_repo_root() {
        let _cwd = CwdLock::acquire();
        let root = find_root().expect("git root resolvable in test");
        assert!(
            root.join("Cargo.toml").exists() || root.join("AGENTS.md").exists(),
            "expected repo root to contain Cargo.toml or AGENTS.md, got {root:?}"
        );
    }

    /// Regression lock: `find_root_from` uses `--show-toplevel` which is worktree-aware.
    /// A linked worktree's root resolves to the worktree path, not the main repo.
    ///
    /// Gherkin:
    ///   Given a synthetic linked worktree in the rhino-cli test suite
    ///   When a guardrail command runs inside it
    ///   Then it succeeds, proving repo-root resolution is worktree-aware
    #[test]
    fn find_root_from_worktree_returns_worktree_path() {
        let main_repo = TempDir::new().expect("tempdir");
        let main = main_repo.path();

        // Init a bare-minimum git repo.
        Cmd::new("git")
            .args(["init"])
            .current_dir(main)
            .output()
            .expect("git init");
        Cmd::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(main)
            .output()
            .expect("git config email");
        Cmd::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(main)
            .output()
            .expect("git config name");
        std::fs::write(main.join("README.md"), "test").expect("write README");
        Cmd::new("git")
            .args(["add", "."])
            .current_dir(main)
            .output()
            .expect("git add");
        Cmd::new("git")
            .args(["commit", "-m", "init"])
            .current_dir(main)
            .output()
            .expect("git commit");

        // Create a linked worktree.
        let wt_dir = TempDir::new().expect("tempdir wt");
        let wt_path = wt_dir.path();
        let status = Cmd::new("git")
            .args(["worktree", "add", &wt_path.to_string_lossy(), "HEAD"])
            .current_dir(main)
            .status()
            .expect("git worktree add");
        assert!(status.success(), "git worktree add must succeed");

        // find_root_from the worktree path must return the WORKTREE path, not main.
        let resolved = find_root_from(Some(wt_path))
            .expect("find_root_from must succeed inside a linked worktree");
        let resolved_canonical = std::fs::canonicalize(&resolved).expect("canonicalize resolved");
        let wt_canonical = std::fs::canonicalize(wt_path).expect("canonicalize wt_path");
        assert_eq!(
            resolved_canonical, wt_canonical,
            "find_root_from must return the linked worktree path when invoked from it"
        );
    }

    /// Regression lock: `rhino-cli doctor`'s "tool missing" scenario runs
    /// with an empty `PATH` (no `git` binary either) to prove it can report
    /// `git` itself as missing. Before this fix, `find_root_from` shelled
    /// out to `git rev-parse --show-toplevel` unconditionally, so with no
    /// `git` binary resolvable, repo-root discovery failed *before* `doctor`
    /// could even print its report — the command bailed with an error and
    /// empty stdout instead of listing `git` as `missing` like every other
    /// absent tool. `find_root_from` must now fall back to walking the
    /// filesystem for a `.git` entry when `git` itself cannot be spawned.
    #[test]
    fn find_root_from_falls_back_to_fs_walk_when_git_binary_unresolvable() {
        let tmp = TempDir::new().expect("tempdir");
        std::fs::create_dir_all(tmp.path().join(".git")).expect("mk .git");
        let nested = tmp.path().join("a").join("b");
        std::fs::create_dir_all(&nested).expect("mk nested dir");

        let root = find_root_from_with("definitely-not-a-real-git-binary-xyz", Some(&nested))
            .expect("falls back to a filesystem walk when the git binary is unresolvable");

        let expected = std::fs::canonicalize(tmp.path()).expect("canonicalize tmp");
        let got = std::fs::canonicalize(&root).expect("canonicalize root");
        assert_eq!(got, expected);
    }

    /// The fallback must not mask a *legitimate* git failure (e.g. `git` is
    /// present but the directory truly is not a repository) by silently
    /// walking the filesystem — only an unresolvable binary triggers the
    /// fallback.
    #[test]
    fn find_root_via_fs_walk_returns_none_without_a_git_entry() {
        let tmp = TempDir::new().expect("tempdir");
        assert!(find_root_via_fs_walk(Some(tmp.path())).is_none());
    }
}
