//! Sync orchestration.
//!
//! Skills are NOT copied: OpenCode reads `.claude/skills/<name>/SKILL.md` natively, so the
//! `--skills-only` flag is a no-op that yields an empty result (kept for CLI back-compat).

use std::time::Instant;

use anyhow::Error;

use super::converter::convert_all_agents;
use super::types::{SyncOptions, SyncResult};

/// Performs the complete sync operation.
pub fn sync_all(opts: &SyncOptions) -> Result<SyncResult, Error> {
    let start = Instant::now();
    let mut result = SyncResult {
        failed_files: Vec::new(),
        ..Default::default()
    };

    // Sync agents unless skills-only (which now produces an empty result).
    if !opts.skills_only {
        let (converted, failed, failed_files) = convert_all_agents(&opts.repo_root, opts.dry_run)?;
        result.agents_converted = converted;
        result.agents_failed = failed;
        result.failed_files.extend(failed_files);
    }

    result.duration = start.elapsed();
    Ok(result)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn fixture_repo() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let claude = root.join(".claude/agents");
        std::fs::create_dir_all(&claude).unwrap();
        std::fs::write(
            claude.join("foo-maker.md"),
            "---\nname: foo-maker\ndescription: Makes foo.\ntools: Read, Write\nmodel:\ncolor: blue\n---\n# Body\n",
        )
        .unwrap();
        std::fs::write(claude.join("README.md"), "# readme\n").unwrap();
        dir
    }

    #[test]
    fn dry_run_converts_without_writing() {
        let dir = fixture_repo();
        let opts = SyncOptions {
            repo_root: dir.path().to_path_buf(),
            dry_run: true,
            agents_only: false,
            skills_only: false,
            verbose: false,
            quiet: false,
        };
        let result = sync_all(&opts).unwrap();
        assert_eq!(result.agents_converted, 1);
        assert_eq!(result.agents_failed, 0);
        // README.md is skipped, nothing written.
        assert!(!dir.path().join(".opencode/agents/foo-maker.md").exists());
    }

    #[test]
    fn real_run_writes_opencode_tree() {
        let dir = fixture_repo();
        let opts = SyncOptions {
            repo_root: dir.path().to_path_buf(),
            dry_run: false,
            agents_only: false,
            skills_only: false,
            verbose: false,
            quiet: false,
        };
        let result = sync_all(&opts).unwrap();
        assert_eq!(result.agents_converted, 1);
        let written =
            std::fs::read_to_string(dir.path().join(".opencode/agents/foo-maker.md")).unwrap();
        assert!(written.starts_with("---\ndescription: Makes foo.\nmodel: opencode-go/minimax-m2.7\npermission:\n  read: allow\n  write: allow\ncolor: primary\n---\n"));
        assert!(written.ends_with("# Body\n"));
    }

    #[test]
    fn skills_only_is_noop() {
        let dir = fixture_repo();
        let opts = SyncOptions {
            repo_root: dir.path().to_path_buf(),
            dry_run: false,
            agents_only: false,
            skills_only: true,
            verbose: false,
            quiet: false,
        };
        let result = sync_all(&opts).unwrap();
        assert_eq!(result.agents_converted, 0);
        assert!(result.failed_files.is_empty());
    }
}
