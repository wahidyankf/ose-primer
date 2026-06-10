//! `env` command family: `init`, `backup`, `restore`.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/cmd/env.go`, `env_init.go`,
//! `env_backup.go`, and `env_restore.go`. The `init` walk lives here (it lived
//! in the Go cmd layer); `backup`/`restore` delegate to
//! [`crate::internal::envbackup`]. On error the dispatcher prints the matching
//! cobra-style usage block (the `*_USAGE` constants) to stderr.

use std::io::Write as _;
use std::path::Path;

use anyhow::{Context, Error, anyhow};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::envbackup::{
    self, DEFAULT_BACKUP_DIR, DEFAULT_MAX_SIZE, DEFAULT_SKIP_DIRS, Options, default_confirm,
    detect_worktree, expand_tilde,
};
use crate::internal::git::root::find_root;

// ===========================================================================
// Usage blocks (cobra-style, printed to stderr on error)
// ===========================================================================

/// Usage block for `env init` (cobra `UsageString`).
pub const ENV_INIT_USAGE: &str = "Usage:\n  \
rhino-cli env init [flags]\n\n\
Flags:\n      \
--force   Overwrite existing .env files\n  \
-h, --help    help for init\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

/// Usage block for `env backup`.
pub const ENV_BACKUP_USAGE: &str = "Usage:\n  \
rhino-cli env backup [flags]\n\n\
Examples:\n  \
# Back up to default directory ~/<repo-name>-env-backup\n  \
rhino-cli env backup\n\n  \
# Back up to a custom directory\n  \
rhino-cli env backup --dir /tmp/my-env-backup\n\n  \
# Namespace backup by worktree/repo name\n  \
rhino-cli env backup --worktree-aware\n\n  \
# Skip overwrite confirmation\n  \
rhino-cli env backup --force\n\n  \
# Include uncommitted config files\n  \
rhino-cli env backup --include-config\n\n  \
# Preview what would be backed up without writing\n  \
rhino-cli env backup --dry-run\n\n  \
# JSON output (implies --force)\n  \
rhino-cli env backup -o json\n\n\
Flags:\n      \
--dir string       backup directory (default: ~/<repo-name>-env-backup)\n      \
--dry-run          preview what would be backed up without writing\n  \
-f, --force            skip overwrite confirmation\n  \
-h, --help             help for backup\n      \
--include-config   also back up known uncommitted config files\n      \
--worktree-aware   namespace backup by worktree/repo directory name\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

/// Usage block for `env restore`.
pub const ENV_RESTORE_USAGE: &str = "Usage:\n  \
rhino-cli env restore [flags]\n\n\
Examples:\n  \
# Restore from default directory ~/<repo-name>-env-backup\n  \
rhino-cli env restore\n\n  \
# Restore from a custom directory\n  \
rhino-cli env restore --dir /tmp/my-env-backup\n\n  \
# Restore from worktree-namespaced backup\n  \
rhino-cli env restore --worktree-aware\n\n  \
# Skip overwrite confirmation\n  \
rhino-cli env restore --force\n\n  \
# Include config files\n  \
rhino-cli env restore --include-config\n\n  \
# Preview what would be restored without writing\n  \
rhino-cli env restore --dry-run\n\n  \
# JSON output (implies --force)\n  \
rhino-cli env restore -o json\n\n\
Flags:\n      \
--dir string       backup source directory (default: ~/<repo-name>-env-backup)\n      \
--dry-run          preview what would be restored without writing\n  \
-f, --force            skip overwrite confirmation\n  \
-h, --help             help for restore\n      \
--include-config   also restore known uncommitted config files\n      \
--worktree-aware   read from worktree-namespaced backup\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

// ===========================================================================
// env init
// ===========================================================================

#[derive(Args, Debug)]
pub struct EnvInitArgs {
    /// Overwrite existing .env files.
    #[arg(long)]
    pub force: bool,
}

/// Runs `env init`. Mirrors Go `runEnvInit`: walks `infra/dev/` for
/// `.env.example` files and copies each to `.env` in the same directory.
pub fn run_env_init(args: &EnvInitArgs) -> Result<(), Error> {
    let repo_root = find_root().context("failed to find git repository root")?;
    let infra_dev_dir = repo_root.join("infra").join("dev");

    let mut created: i64 = 0;
    let mut skipped: i64 = 0;
    let mut errs: Vec<String> = Vec::new();

    // Walk infra/dev/ for .env.example files. Go's filepath.WalkDir errors out
    // entirely if the root cannot be walked; a missing infra/dev simply yields
    // zero matches in walkdir terms, but Go's WalkDir surfaces the stat error of
    // the root. We mirror Go: a missing root produces a walk error.
    if !infra_dev_dir.exists() {
        // Go's WalkDir invokes the callback with the lstat error for the root,
        // and runEnvInit returns it wrapped as "failed to walk infra/dev/".
        return Err(anyhow!(
            "failed to walk infra/dev/: lstat {}: no such file or directory",
            infra_dev_dir.to_string_lossy()
        ));
    }

    let walker = walkdir::WalkDir::new(&infra_dev_dir).sort_by_file_name();
    for entry in walker {
        let entry = entry.with_context(|| "failed to walk infra/dev/")?;
        if entry.file_type().is_dir() {
            continue;
        }
        if entry.file_name() != std::ffi::OsStr::new(".env.example") {
            continue;
        }

        let path = entry.path();
        let dir = path.parent().unwrap_or(Path::new(""));
        let env_path = dir.join(".env");
        let rel_path = env_path.strip_prefix(&repo_root).map_or_else(
            |_| env_path.to_string_lossy().into_owned(),
            |p| p.to_string_lossy().into_owned(),
        );

        if !args.force && std::fs::metadata(&env_path).is_ok() {
            println!("Skipped: {rel_path} (already exists, use --force to overwrite)");
            skipped += 1;
            continue;
        }

        let data = match std::fs::read(path) {
            Ok(d) => d,
            Err(e) => {
                errs.push(format!("failed to read {}: {}", path.to_string_lossy(), e));
                continue;
            }
        };

        if let Err(e) = std::fs::write(&env_path, &data) {
            errs.push(format!(
                "failed to write {}: {}",
                env_path.to_string_lossy(),
                e
            ));
            continue;
        }

        let base = path
            .file_name()
            .map(|b| b.to_string_lossy().into_owned())
            .unwrap_or_default();
        println!("Created: {rel_path} (from {base})");
        created += 1;
    }

    println!("\nSummary: {created} created, {skipped} skipped");

    for e in &errs {
        eprintln!("Error: {e}");
    }

    Ok(())
}

// ===========================================================================
// env backup / restore shared flag struct
// ===========================================================================

#[derive(Args, Debug)]
pub struct EnvBackupArgs {
    /// Backup directory (default: ~/<repo-name>-env-backup).
    #[arg(long, default_value = "")]
    pub dir: String,
    /// Namespace backup by worktree/repo directory name.
    #[arg(long = "worktree-aware")]
    pub worktree_aware: bool,
    /// Skip overwrite confirmation.
    #[arg(long, short = 'f')]
    pub force: bool,
    /// Also back up known uncommitted config files.
    #[arg(long = "include-config")]
    pub include_config: bool,
    /// Preview what would be backed up without writing anything.
    #[arg(long = "dry-run")]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct EnvRestoreArgs {
    /// Backup source directory (default: ~/<repo-name>-env-backup).
    #[arg(long, default_value = "")]
    pub dir: String,
    /// Read from worktree-namespaced backup.
    #[arg(long = "worktree-aware")]
    pub worktree_aware: bool,
    /// Skip overwrite confirmation.
    #[arg(long, short = 'f')]
    pub force: bool,
    /// Also restore known uncommitted config files.
    #[arg(long = "include-config")]
    pub include_config: bool,
    /// Preview what would be restored without writing anything.
    #[arg(long = "dry-run")]
    pub dry_run: bool,
}

/// Resolves the effective backup directory (R11b): when `dir` is empty, derive
/// `~/<repo-basename>-env-backup` from the repo root; otherwise expand `~` then
/// make absolute. Mirrors Go cmd `--dir` handling.
fn resolve_backup_dir(dir: &str, repo_root: &str) -> Result<String, Error> {
    if dir.is_empty() {
        let home = expand_tilde("~").context("cannot determine home directory")?;
        let repo_basename = Path::new(repo_root).file_name().map_or_else(
            || DEFAULT_BACKUP_DIR.to_string(),
            |n| n.to_string_lossy().into_owned(),
        );
        let dir_name = envbackup::default_backup_dir_name(&repo_basename);
        Ok(Path::new(&home)
            .join(dir_name)
            .to_string_lossy()
            .into_owned())
    } else {
        let expanded = expand_tilde(dir).context("invalid backup directory")?;
        let abs = go_abs(&expanded).context("cannot resolve backup directory")?;
        Ok(abs)
    }
}

/// Mirrors Go `filepath.Abs`: joins with the cwd when relative, then cleans.
fn go_abs(path: &str) -> Result<String, Error> {
    let p = Path::new(path);
    let abs = if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir()?.join(p)
    };
    Ok(clean_path(&abs.to_string_lossy()))
}

/// Lexical path cleaning (Go `filepath.Clean` subset sufficient for abs paths).
fn clean_path(path: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    let is_abs = path.starts_with('/');
    for comp in path.split('/') {
        match comp {
            "" | "." => {}
            ".." => {
                if matches!(parts.last(), Some(&p) if p != "..") {
                    parts.pop();
                } else if !is_abs {
                    parts.push("..");
                }
            }
            other => parts.push(other),
        }
    }
    let joined = parts.join("/");
    if is_abs {
        format!("/{joined}")
    } else if joined.is_empty() {
        ".".to_string()
    } else {
        joined
    }
}

/// Whether force mode applies: explicit flag, non-text output, or non-TTY stdin.
fn effective_force(flag_force: bool, output: OutputFormat) -> bool {
    if flag_force || output != OutputFormat::Text {
        return true;
    }
    // Non-TTY stdin implies force.
    !stdin_is_terminal()
}

/// Reports whether stdin is a terminal, mirroring Go's
/// `fi.Mode()&os.ModeCharDevice != 0` check on `os.Stdin.Stat()`. We avoid an
/// `unsafe` libc `isatty` call (the crate forbids unsafe) by stat-ing the fd 0
/// device and testing for a character device. In the shadow-diff harness and
/// integration tests stdin is always piped (non-char-device), so this returns
/// false there, matching the Go binary's behaviour under the same conditions.
#[cfg(unix)]
fn stdin_is_terminal() -> bool {
    use std::os::unix::fs::FileTypeExt as _;
    match std::fs::metadata("/dev/stdin") {
        Ok(m) => m.file_type().is_char_device(),
        Err(_) => false,
    }
}

#[cfg(not(unix))]
fn stdin_is_terminal() -> bool {
    true
}

/// Runs `env backup`. Mirrors Go `runEnvBackup`.
pub fn run_env_backup(
    args: &EnvBackupArgs,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<(), Error> {
    let repo_root = find_root().context("failed to find git repository root")?;
    let repo_root_str = repo_root.to_string_lossy().into_owned();

    let backup_dir = resolve_backup_dir(&args.dir, &repo_root_str)?;
    let force = effective_force(args.force, output);

    let worktree_name = if args.worktree_aware {
        let info = detect_worktree(&repo_root_str).context("worktree detection failed")?;
        info.worktree_name
    } else {
        String::new()
    };

    // Build the confirm closure (prompt → stderr; answer ← stdin). Only used
    // when not forced; mirrors `opts.ConfirmFn = confirmFn(os.Stdin, stderr)`.
    let mut stdin = std::io::stdin().lock();
    let mut confirm_closure = move |existing: &[String]| {
        let mut stderr = std::io::stderr();
        let ok = default_confirm(&mut stdin, &mut stderr, existing);
        let _ = stderr.flush();
        ok
    };

    let opts = Options {
        repo_root: repo_root_str,
        backup_dir,
        skip_dirs: DEFAULT_SKIP_DIRS.to_vec(),
        max_size: DEFAULT_MAX_SIZE,
        worktree_aware: args.worktree_aware,
        worktree_name,
        force,
        include_config: args.include_config,
        dry_run: args.dry_run,
        confirm: if force {
            None
        } else {
            Some(&mut confirm_closure)
        },
    };

    let result = envbackup::backup(opts).context("env backup failed")?;
    write_formatted(&result, output, verbose, quiet)
}

/// Runs `env restore`. Mirrors Go `runEnvRestore`.
pub fn run_env_restore(
    args: &EnvRestoreArgs,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<(), Error> {
    let repo_root = find_root().context("failed to find git repository root")?;
    let repo_root_str = repo_root.to_string_lossy().into_owned();

    let backup_dir = resolve_backup_dir(&args.dir, &repo_root_str)?;
    let force = effective_force(args.force, output);

    let worktree_name = if args.worktree_aware {
        let info = detect_worktree(&repo_root_str).context("worktree detection failed")?;
        info.worktree_name
    } else {
        String::new()
    };

    let mut stdin = std::io::stdin().lock();
    let mut confirm_closure = move |existing: &[String]| {
        let mut stderr = std::io::stderr();
        let ok = default_confirm(&mut stdin, &mut stderr, existing);
        let _ = stderr.flush();
        ok
    };

    let opts = Options {
        repo_root: repo_root_str,
        backup_dir,
        skip_dirs: Vec::new(),
        max_size: DEFAULT_MAX_SIZE,
        worktree_aware: args.worktree_aware,
        worktree_name,
        force,
        include_config: args.include_config,
        dry_run: args.dry_run,
        confirm: if force {
            None
        } else {
            Some(&mut confirm_closure)
        },
    };

    let result = envbackup::restore(opts).context("env restore failed")?;
    write_formatted(&result, output, verbose, quiet)
}

/// Selects the formatter and prints to stdout. Mirrors Go `writeFormatted`.
fn write_formatted(
    result: &envbackup::EnvResult,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<(), Error> {
    let out = match output {
        OutputFormat::Text => envbackup::format_text(result, verbose, quiet),
        OutputFormat::Json => envbackup::format_json(result).context("failed to format JSON")?,
        OutputFormat::Markdown => envbackup::format_markdown(result),
    };
    print!("{out}");
    Ok(())
}
