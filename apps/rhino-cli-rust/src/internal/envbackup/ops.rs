//! Backup and restore engine.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/internal/envbackup/backup.go` and
//! `restore.go`. Discovers `.env*` files (and optional config files), then
//! copies them between the repo root and a backup directory while preserving
//! relative structure and file permissions.

use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context, Error, anyhow};

use super::config::{DEFAULT_CONFIG_PATTERNS, discover_config};
use super::confirm::find_existing;
use super::discover::{DiscoverOptions, discover, is_secret_file};
use super::types::{DEFAULT_MAX_SIZE, DEFAULT_SKIP_DIRS, EnvResult, FileEntry};

/// Callback invoked when destination files already exist. Returns true to
/// proceed with the overwrite, false to cancel. Mirrors Go `Options.ConfirmFn`.
pub type ConfirmFn<'a> = dyn FnMut(&[String]) -> bool + 'a;

/// Configures a backup or restore operation. Mirrors Go `Options`. `skip_dirs`
/// and `max_size` use the `<= 0` / empty defaults from the Go engine.
#[derive(Default)]
pub struct Options<'a> {
    pub repo_root: String,
    pub backup_dir: String,
    pub skip_dirs: Vec<&'static str>,
    pub max_size: i64,
    pub worktree_aware: bool,
    pub worktree_name: String,
    pub force: bool,
    pub include_config: bool,
    pub dry_run: bool,
    pub confirm: Option<&'a mut ConfirmFn<'a>>,
}

/// Returns the default backup directory basename derived from the repo root
/// basename: `<repo-basename>-env-backup`. Mirrors Go `DefaultBackupDirName`.
pub fn default_backup_dir_name(repo_basename: &str) -> String {
    format!("{repo_basename}-env-backup")
}

/// Replaces a leading `~` with the current user's home directory. Mirrors Go
/// `ExpandTilde`.
pub fn expand_tilde(path: &str) -> Result<String, Error> {
    if !path.starts_with('~') {
        return Ok(path.to_string());
    }
    let home = home_dir().context("get home dir")?;
    // Go: filepath.Join(home, path[1:]) — strips the leading "~" then joins.
    let rest = &path[1..];
    let joined = Path::new(&home).join(rest.trim_start_matches('/'));
    // filepath.Join("~"[1:] == "") yields just home; mirror by special-casing "".
    if rest.is_empty() {
        Ok(home)
    } else {
        Ok(joined.to_string_lossy().into_owned())
    }
}

/// Returns the current user's home directory, mirroring Go `os.UserHomeDir`
/// (which reads `$HOME` on unix).
fn home_dir() -> Result<String, Error> {
    std::env::var("HOME")
        .ok()
        .filter(|h| !h.is_empty())
        .ok_or_else(|| anyhow!("$HOME is not defined"))
}

/// Reports whether `backup_dir` is inside (or equal to) `repo_root`. Mirrors Go
/// `isInsideRepo` using `filepath.Rel` semantics.
fn is_inside_repo(backup_dir: &str, repo_root: &str) -> bool {
    match go_rel(repo_root, backup_dir) {
        Ok(rel) => !rel.starts_with(".."),
        Err(_) => false,
    }
}

/// Computes a relative path from `base` to `target`, matching Go `filepath.Rel`
/// closely enough for the inside-repo check (both are absolute paths here).
fn go_rel(base: &str, target: &str) -> Result<String, Error> {
    let base = Path::new(base);
    let target = Path::new(target);
    match target.strip_prefix(base) {
        Ok(rel) => {
            let s = rel.to_string_lossy().into_owned();
            if s.is_empty() {
                Ok(".".to_string())
            } else {
                Ok(s)
            }
        }
        Err(_) => {
            // target is not under base: walk up. We only need to know whether
            // the result starts with "..", which it always does in that case.
            Ok("..".to_string())
        }
    }
}

/// Copies `src` to `dst`, preserving source permissions and truncating `dst`.
/// Mirrors Go `copyFile`.
fn copy_file(src: &str, dst: &str) -> Result<(), Error> {
    let meta = std::fs::symlink_metadata(src).context("lstat src")?;
    std::fs::copy(src, dst).context("copy data")?;
    // Preserve permissions like Go's O_CREATE|O_TRUNC with fi.Mode().Perm().
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let perm = std::fs::Permissions::from_mode(meta.permissions().mode() & 0o777);
        std::fs::set_permissions(dst, perm).context("set dst perms")?;
    }
    #[cfg(not(unix))]
    {
        let _ = meta;
    }
    Ok(())
}

/// Discovers all `.env` files under `opts.repo_root` and copies them to
/// `opts.backup_dir`. Mirrors Go `Backup`.
pub fn backup(mut opts: Options) -> Result<EnvResult, Error> {
    if opts.max_size <= 0 {
        opts.max_size = DEFAULT_MAX_SIZE;
    }
    if opts.skip_dirs.is_empty() {
        opts.skip_dirs = DEFAULT_SKIP_DIRS.to_vec();
    }

    let backup_dir = expand_tilde(&opts.backup_dir).context("expand backup dir")?;
    opts.backup_dir = backup_dir;

    if is_inside_repo(&opts.backup_dir, &opts.repo_root) {
        return Err(anyhow!(
            "backup dir {} is inside repo root {}; choose a directory outside the repo",
            opts.backup_dir,
            opts.repo_root
        ));
    }

    let mut entries = discover(&DiscoverOptions {
        repo_root: &opts.repo_root,
        skip_dirs: &opts.skip_dirs,
        max_size: opts.max_size,
    })
    .context("discover env files")?;

    // Tag discovered entries with "env" when IncludeConfig is active.
    if opts.include_config {
        for e in &mut entries {
            if e.source.is_empty() {
                e.source = "env".to_string();
            }
        }
        let config_entries =
            discover_config(&opts.repo_root, DEFAULT_CONFIG_PATTERNS, opts.max_size)
                .context("discover config files")?;
        entries.extend(config_entries);
        entries.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    }

    // Determine the effective destination root (worktree-aware namespacing).
    let dest_root: PathBuf = if opts.worktree_aware && !opts.worktree_name.is_empty() {
        Path::new(&opts.backup_dir).join(&opts.worktree_name)
    } else {
        PathBuf::from(&opts.backup_dir)
    };
    let dest_root_str = dest_root.to_string_lossy().into_owned();

    // Confirmation check.
    if !opts.force
        && let Some(confirm) = opts.confirm.as_mut()
    {
        let existing = find_existing(&entries, &dest_root_str);
        if !existing.is_empty() && !confirm(&existing) {
            return Ok(EnvResult {
                direction: "backup".to_string(),
                dir: opts.backup_dir.clone(),
                cancelled: true,
                ..Default::default()
            });
        }
    }

    let mut result = EnvResult {
        direction: "backup".to_string(),
        dir: opts.backup_dir.clone(),
        files: entries.clone(),
        worktree_name: opts.worktree_name.clone(),
        ..Default::default()
    };

    // Dry-run: list what would be backed up without writing anything.
    if opts.dry_run {
        for e in &entries {
            if e.skipped {
                result.skipped += 1;
            } else {
                result.copied += 1;
            }
        }
        return Ok(result);
    }

    std::fs::create_dir_all(&dest_root).context("create backup dir")?;

    for e in &entries {
        if e.skipped {
            result.skipped += 1;
            continue;
        }

        let dst = dest_root.join(&e.rel_path);
        if let Some(parent) = dst.parent()
            && let Err(err) = std::fs::create_dir_all(parent)
        {
            result
                .errors
                .push(format!("mkdir for {}: {}", e.rel_path, go_io_err(&err)));
            result.skipped += 1;
            continue;
        }

        if let Err(err) = copy_file(&e.abs_path, &dst.to_string_lossy()) {
            result.errors.push(format!("copy {}: {err:#}", e.rel_path));
            result.skipped += 1;
            continue;
        }
        result.copied += 1;
    }

    Ok(result)
}

/// Copies `.env*` files from `opts.backup_dir` back to `opts.repo_root`.
/// Mirrors Go `Restore`.
pub fn restore(mut opts: Options) -> Result<EnvResult, Error> {
    if opts.max_size <= 0 {
        opts.max_size = DEFAULT_MAX_SIZE;
    }

    let backup_dir = expand_tilde(&opts.backup_dir).context("expand backup dir")?;
    opts.backup_dir = backup_dir;

    // Determine the effective source root (worktree-aware namespacing).
    let src_root: PathBuf = if opts.worktree_aware && !opts.worktree_name.is_empty() {
        Path::new(&opts.backup_dir).join(&opts.worktree_name)
    } else {
        PathBuf::from(&opts.backup_dir)
    };
    let src_root_str = src_root.to_string_lossy().into_owned();

    // Validate source dir exists.
    match std::fs::metadata(&src_root) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(anyhow!("backup dir does not exist: {src_root_str}"));
        }
        Err(e) => return Err(e).context("stat backup dir"),
    }

    // Discover .env* files in the backup dir (skip only ".git").
    let mut entries = discover(&DiscoverOptions {
        repo_root: &src_root_str,
        skip_dirs: &[".git"],
        max_size: opts.max_size,
    })
    .context("discover backup files")?;

    // Config discovery from backup dir.
    if opts.include_config {
        for e in &mut entries {
            if e.source.is_empty() {
                e.source = "env".to_string();
            }
        }
        let config_entries = discover_config(&src_root_str, DEFAULT_CONFIG_PATTERNS, opts.max_size)
            .context("discover config files")?;
        entries.extend(config_entries);
        entries.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    }

    // Confirmation check.
    if !opts.force
        && let Some(confirm) = opts.confirm.as_mut()
    {
        let restore_entries: Vec<FileEntry> = entries
            .iter()
            .filter(|e| {
                if e.source == "config" {
                    return true;
                }
                let base = std::path::Path::new(&e.rel_path)
                    .file_name()
                    .map(|b| b.to_string_lossy().into_owned())
                    .unwrap_or_default();
                is_secret_file(&base, &e.rel_path)
            })
            .cloned()
            .collect();
        let existing = find_existing(&restore_entries, &opts.repo_root);
        if !existing.is_empty() && !confirm(&existing) {
            return Ok(EnvResult {
                direction: "restore".to_string(),
                dir: opts.backup_dir.clone(),
                cancelled: true,
                ..Default::default()
            });
        }
    }

    if opts.dry_run {
        return Ok(restore_dry_run(
            entries,
            opts.backup_dir,
            opts.worktree_name,
        ));
    }

    let mut result = EnvResult {
        direction: "restore".to_string(),
        dir: opts.backup_dir.clone(),
        worktree_name: opts.worktree_name.clone(),
        ..Default::default()
    };
    restore_entries_to_repo(entries, &opts.repo_root, &mut result);
    Ok(result)
}

/// Handles dry-run path for restore: counts and lists what would be restored without
/// writing anything. Extracted to keep `restore` under the line-count limit.
fn restore_dry_run(entries: Vec<FileEntry>, dir: String, worktree_name: String) -> EnvResult {
    let mut result = EnvResult {
        direction: "restore".to_string(),
        dir,
        worktree_name,
        ..Default::default()
    };
    for e in entries {
        let base = Path::new(&e.rel_path)
            .file_name()
            .map(|b| b.to_string_lossy().into_owned())
            .unwrap_or_default();
        if e.source != "config" && !is_secret_file(&base, &e.rel_path) {
            continue;
        }
        result.files.push(e.clone());
        if e.skipped {
            result.skipped += 1;
        } else {
            result.copied += 1;
        }
    }
    result
}

/// Copies entries from the backup dir back to `repo_root`, filtering to secret
/// and config files only. Extracted to keep `restore` under the line-count limit.
fn restore_entries_to_repo(entries: Vec<FileEntry>, repo_root: &str, result: &mut EnvResult) {
    for e in entries {
        let base = Path::new(&e.rel_path)
            .file_name()
            .map(|b| b.to_string_lossy().into_owned())
            .unwrap_or_default();
        if e.source != "config" && !is_secret_file(&base, &e.rel_path) {
            continue;
        }
        result.files.push(e.clone());
        if e.skipped {
            result.skipped += 1;
            continue;
        }
        let dst = Path::new(repo_root).join(&e.rel_path);
        if let Some(parent) = dst.parent()
            && let Err(err) = std::fs::create_dir_all(parent)
        {
            result
                .errors
                .push(format!("mkdir for {}: {}", e.rel_path, go_io_err(&err)));
            result.skipped += 1;
            continue;
        }
        if let Err(err) = copy_file(&e.abs_path, &dst.to_string_lossy()) {
            result.errors.push(format!("copy {}: {err:#}", e.rel_path));
            result.skipped += 1;
            continue;
        }
        result.copied += 1;
    }
}

/// Renders an io error message close to Go's `%v`. Used only inside non-fatal
/// warnings, which are exercised by injected error paths.
fn go_io_err(err: &io::Error) -> String {
    err.to_string()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn write(root: &Path, rel: &str, content: &[u8]) {
        let p = root.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, content).unwrap();
    }

    fn opts<'a>(repo: &Path, backup: &Path) -> Options<'a> {
        Options {
            repo_root: repo.to_string_lossy().into_owned(),
            backup_dir: backup.to_string_lossy().into_owned(),
            skip_dirs: DEFAULT_SKIP_DIRS.to_vec(),
            max_size: DEFAULT_MAX_SIZE,
            worktree_aware: false,
            worktree_name: String::new(),
            force: true,
            include_config: false,
            dry_run: false,
            confirm: None,
        }
    }

    #[test]
    fn expand_tilde_replaces_home() {
        let home = std::env::var("HOME").unwrap();
        assert_eq!(expand_tilde("~").unwrap(), home);
        assert_eq!(expand_tilde("~/sub").unwrap(), format!("{home}/sub"));
        assert_eq!(expand_tilde("/abs").unwrap(), "/abs");
    }

    #[test]
    fn is_inside_repo_detects_nested() {
        assert!(is_inside_repo("/r/inside", "/r"));
        assert!(is_inside_repo("/r", "/r"));
        assert!(!is_inside_repo("/other", "/r"));
    }

    #[test]
    fn backup_rejects_dir_inside_repo() {
        let repo = tempfile::tempdir().unwrap();
        write(repo.path(), ".env", b"A=1\n");
        let mut o = opts(repo.path(), &repo.path().join("inside-bk"));
        o.repo_root = repo.path().to_string_lossy().into_owned();
        let err = backup(o).unwrap_err();
        assert!(err.to_string().contains("is inside repo root"));
    }

    #[test]
    fn backup_copies_env_files_preserving_structure() {
        let repo = tempfile::tempdir().unwrap();
        let bk = tempfile::tempdir().unwrap();
        write(repo.path(), ".env", b"ROOT=1\n");
        write(repo.path(), "apps/web/.env.local", b"WEB=1\n");
        write(repo.path(), "node_modules/.env", b"IGNORED\n");

        let result = backup(opts(repo.path(), bk.path())).unwrap();
        assert_eq!(result.direction, "backup");
        assert_eq!(result.copied, 2);
        assert!(bk.path().join(".env").exists());
        assert!(bk.path().join("apps/web/.env.local").exists());
        assert!(!bk.path().join("node_modules/.env").exists());
    }

    #[test]
    fn backup_worktree_aware_namespaces() {
        let repo = tempfile::tempdir().unwrap();
        let bk = tempfile::tempdir().unwrap();
        write(repo.path(), ".env", b"ROOT=1\n");
        let mut o = opts(repo.path(), bk.path());
        o.worktree_aware = true;
        o.worktree_name = "feature-x".to_string();
        let result = backup(o).unwrap();
        assert_eq!(result.worktree_name, "feature-x");
        assert!(bk.path().join("feature-x/.env").exists());
    }

    #[test]
    fn backup_include_config_picks_up_claude_settings() {
        let repo = tempfile::tempdir().unwrap();
        let bk = tempfile::tempdir().unwrap();
        write(repo.path(), ".env", b"ROOT=1\n");
        write(repo.path(), ".claude/settings.local.json", b"{}\n");
        let mut o = opts(repo.path(), bk.path());
        o.include_config = true;
        let result = backup(o).unwrap();
        assert!(bk.path().join(".claude/settings.local.json").exists());
        assert!(result.files.iter().any(|f| f.source == "config"));
    }

    #[test]
    fn backup_confirm_decline_cancels() {
        let repo = tempfile::tempdir().unwrap();
        let bk = tempfile::tempdir().unwrap();
        write(repo.path(), ".env", b"ROOT=1\n");
        write(bk.path(), ".env", b"OLD=1\n"); // existing conflict
        let mut declined = false;
        let mut confirm = |_existing: &[String]| {
            declined = true;
            false
        };
        let mut o = opts(repo.path(), bk.path());
        o.force = false;
        o.confirm = Some(&mut confirm);
        let result = backup(o).unwrap();
        assert!(result.cancelled);
        assert!(declined);
        // Existing backup file untouched.
        assert_eq!(
            std::fs::read_to_string(bk.path().join(".env")).unwrap(),
            "OLD=1\n"
        );
    }

    #[test]
    fn restore_copies_env_back_and_ignores_non_env() {
        let repo = tempfile::tempdir().unwrap();
        let bk = tempfile::tempdir().unwrap();
        write(bk.path(), ".env", b"ROOT=1\n");
        write(bk.path(), "apps/web/.env", b"WEB=1\n");
        write(bk.path(), "README.md", b"# doc\n");
        let result = restore(opts(repo.path(), bk.path())).unwrap();
        assert_eq!(result.direction, "restore");
        assert_eq!(result.copied, 2);
        assert!(repo.path().join(".env").exists());
        assert!(repo.path().join("apps/web/.env").exists());
        assert!(!repo.path().join("README.md").exists());
    }

    #[test]
    fn restore_missing_dir_errors() {
        let repo = tempfile::tempdir().unwrap();
        let mut o = opts(repo.path(), Path::new("/nonexistent-rhino-xyz-123"));
        o.backup_dir = "/nonexistent-rhino-xyz-123".to_string();
        let err = restore(o).unwrap_err();
        assert!(err.to_string().contains("does not exist"));
    }

    #[test]
    fn restore_include_config_restores_claude() {
        let repo = tempfile::tempdir().unwrap();
        let bk = tempfile::tempdir().unwrap();
        write(bk.path(), ".env", b"ROOT=1\n");
        write(bk.path(), ".claude/settings.local.json", b"{}\n");
        let mut o = opts(repo.path(), bk.path());
        o.include_config = true;
        let _ = restore(o).unwrap();
        assert!(repo.path().join(".claude/settings.local.json").exists());
    }

    #[test]
    fn backup_dry_run_writes_nothing() {
        let repo = tempfile::tempdir().unwrap();
        let bk = tempfile::tempdir().unwrap();
        write(repo.path(), ".env", b"A=1\n");
        write(repo.path(), "secrets.json", b"{}\n");
        let mut o = opts(repo.path(), bk.path());
        o.dry_run = true;
        let result = backup(o).unwrap();
        assert_eq!(result.direction, "backup");
        // dry-run: no files actually written
        assert!(
            !bk.path().join(".env").exists(),
            "dry-run must not write .env"
        );
        assert!(
            !bk.path().join("secrets.json").exists(),
            "dry-run must not write secrets.json"
        );
        // files still listed in result
        assert!(
            result
                .files
                .iter()
                .any(|f| f.rel_path == ".env" || f.rel_path == "secrets.json")
        );
    }

    #[test]
    fn restore_secret_kinds_roundtrip() {
        let repo = tempfile::tempdir().unwrap();
        let bk = tempfile::tempdir().unwrap();
        write(bk.path(), "secrets.json", b"{}\n");
        write(bk.path(), "cert.pem", b"PEM\n");
        write(bk.path(), ".secrets/notes.md", b"secret\n");
        let result = restore(opts(repo.path(), bk.path())).unwrap();
        assert_eq!(result.direction, "restore");
        assert!(
            repo.path().join("secrets.json").exists(),
            "secrets.json should be restored"
        );
        assert!(
            repo.path().join("cert.pem").exists(),
            "cert.pem should be restored"
        );
        assert!(
            repo.path().join(".secrets/notes.md").exists(),
            ".secrets/notes.md should be restored"
        );
    }

    #[cfg(unix)]
    #[test]
    fn copy_file_preserves_permissions() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        let dst = tmp.path().join("dst");
        std::fs::write(&src, b"x").unwrap();
        std::fs::set_permissions(&src, std::fs::Permissions::from_mode(0o600)).unwrap();
        copy_file(&src.to_string_lossy(), &dst.to_string_lossy()).unwrap();
        let mode = std::fs::metadata(&dst).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
    }
}
