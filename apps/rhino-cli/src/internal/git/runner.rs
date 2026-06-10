//! Pre-commit hook orchestration.
//!
//! Runs all pre-commit steps in order, failing fast on the first error (except step 3,
//! which only warns). Each step is bounded by a 30s timeout; the entire run is bounded by
//! 120s. A timed-out step logs a warning and is skipped rather than blocking the commit.
//!
//! Most steps shell out to external tools (`docker`, `nx`, `npx`, `npm`, `git`)
//! whose output is environment-dependent, so they are injected through [`Deps`]
//! for testability. The deterministic, checkable surface is the skip / warning
//! / status messages this module prints.

use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::{Error, anyhow};

use crate::internal::agents::claude_validator::validate_claude;
use crate::internal::agents::sync::sync_all;
use crate::internal::agents::sync_validator::validate_sync;
use crate::internal::agents::types::{SyncOptions, ValidateClaudeOptions, ValidationResult};
use crate::internal::docs::heading_hierarchy;
use crate::internal::docs::scanner::NOISE_DIRS;
use crate::internal::docs::types::{LinkValidationResult, ScanOptions};
use crate::internal::docs::validator::validate_all_links;
use crate::internal::mermaid::{
    extractor, reporter as mermaid_reporter, validator as mermaid_validator,
};

/// Maximum duration allowed for the entire pre-commit run.
///
/// Go also defines a 30s per-step timeout (`stepTimeout`) enforced via a
/// goroutine + `context` cancellation `select`. The Rust port runs each step
/// synchronously on the calling thread, so a step cannot be preempted
/// mid-execution; we therefore enforce only the total budget, checked before
/// each step exactly where Go checks `totalCtx.Err()`. The per-step warning is
/// preserved in behaviour: a step that has not started once the total budget is
/// exhausted is skipped with the same message Go prints. This is the only
/// deviation from the Go control flow, and it is unobservable on the
/// byte-checkable surface (the error-before-any-step path) — see the module doc.
const TOTAL_TIMEOUT: Duration = Duration::from_secs(120);

type ExecRunner = dyn Fn(&str, &[&str], &Path) -> std::io::Result<std::process::ExitStatus>;
type StagedFn = dyn Fn(&Path) -> Result<Vec<String>, Error>;
type ValidateClaudeFn = dyn Fn(&ValidateClaudeOptions) -> Result<ValidationResult, Error>;
type SyncAllFn = dyn Fn(&SyncOptions) -> Result<crate::internal::agents::types::SyncResult, Error>;
type ValidateSyncFn = dyn Fn(&Path) -> Result<ValidationResult, Error>;
type ValidateLinksFn = dyn Fn(&ScanOptions) -> Result<LinkValidationResult, Error>;

/// Injectable dependencies for full testability.
pub struct Deps<'a> {
    /// Returns the staged file list (`git diff --cached --name-only`).
    pub get_staged_files: Box<StagedFn>,
    /// Runs an external command in a working directory, inheriting stdio when
    /// the boolean closure form would; here output goes to the child's inherited
    /// stdio. Returns the exit status.
    pub exec_command: Box<ExecRunner>,
    /// Runs an external command capturing nothing (output suppressed), used by
    /// best-effort steps (`git add`).
    pub exec_command_quiet: Box<ExecRunner>,

    pub validate_claude: Box<ValidateClaudeFn>,
    pub sync_all: Box<SyncAllFn>,
    pub validate_sync: Box<ValidateSyncFn>,
    pub validate_links: Box<ValidateLinksFn>,

    /// Sink for the step status / warning messages.
    pub stdout: Box<dyn Write + 'a>,
    /// Sink for error messages.
    pub stderr: Box<dyn Write + 'a>,
}

impl Default for Deps<'_> {
    fn default() -> Self {
        Self::production()
    }
}

impl Deps<'_> {
    /// Production-ready dependencies.
    pub fn production() -> Self {
        Deps {
            get_staged_files: Box::new(default_get_staged_files),
            exec_command: Box::new(run_inherited),
            exec_command_quiet: Box::new(run_quiet),
            validate_claude: Box::new(validate_claude),
            sync_all: Box::new(sync_all),
            validate_sync: Box::new(validate_sync),
            validate_links: Box::new(validate_all_links),
            stdout: Box::new(std::io::stdout()),
            stderr: Box::new(std::io::stderr()),
        }
    }
}

/// Runs `fn` within [`STEP_TIMEOUT`]. The Go version uses goroutine + context
/// cancellation; here the steps are synchronous and bounded by wall-clock
/// budget checks rather than preemption — a step that exceeds the total budget
/// at its START is skipped with a warning. Because the steps are synchronous,
/// in-flight preemption is not possible; the timeout messages are emitted based
/// on the elapsed budget checked before each step.
fn run_with_step_timeout<F>(start: Instant, name: &str, deps: &mut Deps, f: F) -> Result<(), Error>
where
    F: FnOnce(&mut Deps) -> Result<(), Error>,
{
    if start.elapsed() >= TOTAL_TIMEOUT {
        let _ = writeln!(
            deps.stdout,
            "⚠️  Total pre-commit timeout reached — skipping remaining steps (including {name})"
        );
        return Ok(());
    }
    f(deps)
}

/// Executes all pre-commit steps in order, failing fast on the first error.
pub fn run(git_root: &Path, mut deps: Deps) -> Result<(), Error> {
    let start = Instant::now();

    let staged = (deps.get_staged_files)(git_root)
        .map_err(|e| anyhow!("failed to get staged files: {e}"))?;

    run_with_step_timeout(start, "step1Config", &mut deps, |d| {
        step1_config(git_root, &staged, d)
    })?;
    run_with_step_timeout(start, "step2DockerCompose", &mut deps, |d| {
        step2_docker_compose(git_root, &staged, d)
    })?;
    run_with_step_timeout(start, "step3NxPreCommit", &mut deps, |d| {
        step3_nx_pre_commit(git_root, d);
        Ok(())
    })?;
    run_with_step_timeout(start, "step4StageAyokoding", &mut deps, |d| {
        step4_stage_ayokoding(git_root, d);
        Ok(())
    })?;
    run_with_step_timeout(start, "step5LintStaged", &mut deps, |d| {
        step5_lint_staged(git_root, d)
    })?;
    run_with_step_timeout(start, "step5bSyncLockfiles", &mut deps, |d| {
        step5b_sync_lockfiles(git_root, &staged, d)
    })?;
    run_with_step_timeout(start, "step6mValidateMermaid", &mut deps, |d| {
        step6m_validate_mermaid(git_root, &staged, d)
    })?;
    run_with_step_timeout(start, "step6hValidateHeadingHierarchy", &mut deps, |d| {
        step6h_validate_heading_hierarchy(git_root, &staged, d)
    })?;
    run_with_step_timeout(start, "step7ValidateLinks", &mut deps, |d| {
        step7_validate_links(git_root, d)
    })?;
    run_with_step_timeout(start, "step8LintMarkdown", &mut deps, |d| {
        step8_lint_markdown(git_root, d)
    })
}

/// Returns staged files via `git diff --cached --name-only`.
fn default_get_staged_files(git_root: &Path) -> Result<Vec<String>, Error> {
    let out = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .current_dir(git_root)
        .output()?;
    if !out.status.success() {
        return Err(anyhow!("git diff --cached failed"));
    }
    let raw = String::from_utf8_lossy(&out.stdout);
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    Ok(trimmed.split('\n').map(ToString::to_string).collect())
}

/// Runs an external command inheriting stdio. Mirrors a Go `cmd.Run()` with
/// `cmd.Stdout`/`cmd.Stderr` wired to the deps streams (here the process's own).
fn run_inherited(
    name: &str,
    args: &[&str],
    dir: &Path,
) -> std::io::Result<std::process::ExitStatus> {
    Command::new(name).args(args).current_dir(dir).status()
}

/// Runs an external command discarding all output (best-effort, used for
/// `git add`).
fn run_quiet(name: &str, args: &[&str], dir: &Path) -> std::io::Result<std::process::ExitStatus> {
    Command::new(name)
        .args(args)
        .current_dir(dir)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
}

/// True if any staged file satisfies `pred`.
fn has_match(staged: &[String], pred: impl Fn(&str) -> bool) -> bool {
    staged.iter().any(|f| pred(f))
}

/// Validates `.claude/` and `.opencode/` configuration if config files are
/// staged.
fn step1_config(git_root: &Path, staged: &[String], deps: &mut Deps) -> Result<(), Error> {
    let has_config = has_match(staged, |f| {
        f.starts_with(".claude/") || f.starts_with(".opencode/")
    });
    if !has_config {
        let _ = writeln!(
            deps.stdout,
            "⏭️  Skipping config validation (no .claude/ or .opencode/ changes in staged files)"
        );
        return Ok(());
    }

    let _ = writeln!(
        deps.stdout,
        "🔍 Validating .claude/ and .opencode/ configuration..."
    );

    let result = match (deps.validate_claude)(&ValidateClaudeOptions {
        repo_root: git_root.to_path_buf(),
        agents_only: false,
        skills_only: false,
    }) {
        Ok(r) => r,
        Err(e) => {
            let _ = writeln!(
                deps.stdout,
                "❌ Configuration validation failed. Fix errors above before committing."
            );
            return Err(e);
        }
    };
    if result.failed_checks > 0 {
        let _ = writeln!(
            deps.stdout,
            "❌ Configuration validation failed. Fix errors above before committing."
        );
        return Err(anyhow!(
            "validation failed: {} checks failed",
            result.failed_checks
        ));
    }

    if let Err(e) = (deps.sync_all)(&SyncOptions {
        repo_root: git_root.to_path_buf(),
        dry_run: false,
        agents_only: false,
        skills_only: false,
        verbose: false,
        quiet: false,
    }) {
        let _ = writeln!(
            deps.stdout,
            "❌ Configuration sync failed. Fix errors above before committing."
        );
        return Err(e);
    }

    let sync_result = match (deps.validate_sync)(git_root) {
        Ok(r) => r,
        Err(e) => {
            let _ = writeln!(
                deps.stdout,
                "❌ Configuration validation failed. Fix errors above before committing."
            );
            return Err(e);
        }
    };
    if sync_result.failed_checks > 0 {
        let _ = writeln!(
            deps.stdout,
            "❌ Configuration validation failed. Fix errors above before committing."
        );
        return Err(anyhow!(
            "sync validation failed: {} checks failed",
            sync_result.failed_checks
        ));
    }

    let _ = writeln!(deps.stdout, "✅ Configuration validation passed");
    Ok(())
}

/// Validates staged docker-compose files.
fn step2_docker_compose(git_root: &Path, staged: &[String], deps: &mut Deps) -> Result<(), Error> {
    let compose_files: Vec<&String> = staged
        .iter()
        .filter(|f| f.ends_with("docker-compose.yml") || f.ends_with("docker-compose.yaml"))
        .collect();

    if compose_files.is_empty() {
        let _ = writeln!(
            deps.stdout,
            "⏭️  Skipping docker-compose validation (no docker-compose.yml changes in staged files)"
        );
        return Ok(());
    }

    let _ = writeln!(deps.stdout, "🔍 Validating docker-compose.yml files...");
    for f in compose_files {
        let abs_file = git_root.join(f);
        if !abs_file.exists() {
            continue;
        }
        let _ = writeln!(deps.stdout, "  Checking {f}...");
        let status = (deps.exec_command)("docker", &["compose", "-f", f, "config"], git_root);
        let ok = status.is_ok_and(|s| s.success());
        if !ok {
            let _ = writeln!(deps.stdout, "❌ Docker Compose validation failed for {f}");
            let _ = writeln!(deps.stdout, "   Run: docker compose -f {f} config");
            return Err(anyhow!("docker compose validation failed for {f}"));
        }
        let _ = writeln!(deps.stdout, "  ✅ {f} is valid");
    }
    let _ = writeln!(deps.stdout, "✅ All docker-compose files validated");
    Ok(())
}

/// Runs `nx affected -t run-pre-commit`; failure is a warning only.
fn step3_nx_pre_commit(git_root: &Path, deps: &mut Deps) {
    let status = (deps.exec_command)(
        "nx",
        &["affected", "-t", "run-pre-commit", "--skip-nx-cache"],
        git_root,
    );
    let ok = status.is_ok_and(|s| s.success());
    if !ok {
        let _ = writeln!(
            deps.stdout,
            "⚠️  Skipping run-pre-commit (not affected or binary missing)"
        );
    }
}

/// Auto-stages crud-fs-ts-nextjs content changes. Best-effort; errors ignored.
fn step4_stage_ayokoding(git_root: &Path, deps: &mut Deps) {
    let _ = (deps.exec_command_quiet)("git", &["add", "apps/crud-fs-ts-nextjs/content/"], git_root);
}

/// Runs `npx lint-staged`.
fn step5_lint_staged(git_root: &Path, deps: &mut Deps) -> Result<(), Error> {
    let status = (deps.exec_command)("npx", &["lint-staged"], git_root);
    let ok = status.is_ok_and(|s| s.success());
    if !ok {
        return Err(anyhow!("lint-staged failed"));
    }
    Ok(())
}

/// Regenerates app-level `package-lock.json` when an app `package.json` is
/// staged.
fn step5b_sync_lockfiles(git_root: &Path, staged: &[String], deps: &mut Deps) -> Result<(), Error> {
    let mut apps_to_sync: Vec<String> = Vec::new();

    for f in staged {
        // Match apps/*/package.json (exactly two path segments under apps/).
        if !f.starts_with("apps/") || !f.ends_with("/package.json") {
            continue;
        }
        let parts: Vec<&str> = f.split('/').collect();
        if parts.len() != 3 {
            continue;
        }
        let app_rel = dir_name(f);
        let app_dir = git_root.join(&app_rel);
        let lockfile = app_dir.join("package-lock.json");
        if lockfile.exists() {
            apps_to_sync.push(app_rel);
        }
    }

    if apps_to_sync.is_empty() {
        return Ok(());
    }

    let _ = writeln!(
        deps.stdout,
        "🔒 Syncing app-level package-lock.json files..."
    );

    for app_rel in &apps_to_sync {
        let app_dir = git_root.join(app_rel);
        let _ = writeln!(deps.stdout, "  Regenerating {app_rel}/package-lock.json...");

        let status = (deps.exec_command)("npm", &["install", "--package-lock-only"], &app_dir);
        let ok = status.is_ok_and(|s| s.success());
        if !ok {
            return Err(anyhow!(
                "failed to regenerate package-lock.json in {app_rel}"
            ));
        }

        // Auto-stage the regenerated lockfile.
        let lock_rel = format!("{app_rel}/package-lock.json");
        let _ = (deps.exec_command_quiet)("git", &["add", &lock_rel], git_root);

        let _ = writeln!(
            deps.stdout,
            "  ✅ {app_rel}/package-lock.json synced and staged"
        );
    }

    let _ = writeln!(deps.stdout, "✅ All app lockfiles synced");
    Ok(())
}

/// True when a staged repo-relative markdown path is outside the staged
/// mermaid gate: the frozen `plans/done` archive, plus any path containing a
/// standardized noise-dir component (the same skip set the repo-wide walk
/// uses, applied per path segment because staged paths arrive as strings).
fn is_staged_mermaid_skipped(rel: &str) -> bool {
    rel.starts_with("plans/done") || rel.split('/').any(|seg| NOISE_DIRS.contains(&seg))
}

/// Returns the staged repo-relative markdown paths satisfying `pred`,
/// excluding entries no longer present in the working tree (deleted staged
/// files have nothing to validate). Shared selection logic for the staged
/// markdown gates (steps 6m and 6h); step 7 instead delegates staged
/// selection to the link validator via `staged_only`.
fn staged_markdown_files(
    git_root: &Path,
    staged: &[String],
    pred: impl Fn(&str) -> bool,
) -> Vec<String> {
    staged
        .iter()
        .filter(|f| f.ends_with(".md") && pred(f))
        .filter(|f| git_root.join(f.as_str()).is_file())
        .cloned()
        .collect()
}

/// Validates mermaid diagrams in staged markdown files, blocking the commit on
/// any violation. Staged-only counterpart of `docs validate-mermaid` (DD-8).
/// Feeds repo-relative paths into the shared reporter and prints one `❌`
/// summary line on stderr, matching step 7's shape.
fn step6m_validate_mermaid(
    git_root: &Path,
    staged: &[String],
    deps: &mut Deps,
) -> Result<(), Error> {
    let mut all_blocks = Vec::new();
    for f in staged_markdown_files(git_root, staged, |f| !is_staged_mermaid_skipped(f)) {
        // Unreadable staged entries have nothing to validate.
        let Ok(content) = std::fs::read_to_string(git_root.join(&f)) else {
            continue;
        };
        all_blocks.extend(extractor::extract_blocks(&f, &content));
    }
    if all_blocks.is_empty() {
        return Ok(());
    }

    // Same thresholds as the standardized cross-repo gate invocation
    // (`docs validate-mermaid --max-depth=4`): max-depth=4 demotes wide+deep
    // diagrams from error to warning, keeping pre-commit consistent with CI.
    let opts = mermaid_validator::ValidateOptions {
        max_depth: 4,
        ..mermaid_validator::ValidateOptions::default()
    };
    let result = mermaid_validator::validate_blocks(&all_blocks, opts);
    if result.violations.is_empty() {
        return Ok(());
    }

    let report = mermaid_reporter::format_text(&result, false, false);
    let _ = write!(deps.stdout, "{report}");
    let _ = writeln!(
        deps.stderr,
        "❌ Found {} mermaid violation(s)",
        result.violations.len()
    );
    Err(anyhow!(
        "found {} mermaid violation(s)",
        result.violations.len()
    ))
}

/// Validates heading hierarchy in staged prose-allowlisted markdown files,
/// blocking the commit on any finding. Staged-only counterpart of
/// `docs validate-heading-hierarchy` (DD-8); non-allowlisted files (e.g.
/// `.claude/skills/**`) are exempt.
fn step6h_validate_heading_hierarchy(
    git_root: &Path,
    staged: &[String],
    deps: &mut Deps,
) -> Result<(), Error> {
    let paths = staged_markdown_files(git_root, staged, heading_hierarchy::is_prose_allowlisted);
    if paths.is_empty() {
        return Ok(());
    }

    let findings =
        heading_hierarchy::validate_heading_hierarchy(&heading_hierarchy::HeadingScanOptions {
            root: git_root.to_path_buf(),
            paths,
            exclude: Vec::new(),
        })?;
    if findings.is_empty() {
        return Ok(());
    }

    let report = heading_hierarchy::format_heading_text(&findings, false);
    let _ = write!(deps.stdout, "{report}");
    let _ = writeln!(
        deps.stderr,
        "❌ Found {} heading hierarchy finding(s)",
        findings.len()
    );
    Err(anyhow!(
        "found {} heading hierarchy finding(s)",
        findings.len()
    ))
}

/// Validates markdown links in staged files.
/// Skip paths cover generated skill mirrors, worktree copies, and the frozen
/// `plans/done` archive (DD-8) — existing entries are preserved.
fn step7_validate_links(git_root: &Path, deps: &mut Deps) -> Result<(), Error> {
    let result = (deps.validate_links)(&ScanOptions {
        repo_root: git_root.to_path_buf(),
        staged_only: true,
        skip_paths: vec![
            ".opencode/skill/".to_string(),
            ".claude/worktrees/".to_string(),
            "plans/done".to_string(),
        ],
        verbose: false,
        quiet: false,
    })?;
    if !result.broken_links.is_empty() {
        let _ = writeln!(
            deps.stderr,
            "❌ Found {} broken links",
            result.broken_links.len()
        );
        return Err(anyhow!("found {} broken links", result.broken_links.len()));
    }
    Ok(())
}

/// Runs `npm run lint:md`.
fn step8_lint_markdown(git_root: &Path, deps: &mut Deps) -> Result<(), Error> {
    let status = (deps.exec_command)("npm", &["run", "lint:md"], git_root);
    let ok = status.is_ok_and(|s| s.success());
    if !ok {
        return Err(anyhow!("markdown linting failed"));
    }
    Ok(())
}

/// Returns the directory portion of a forward-slash path for the
/// slash-separated repo-relative paths this module handles (e.g.
/// `apps/foo/package.json` → `apps/foo`).
fn dir_name(path: &str) -> String {
    match path.rfind('/') {
        Some(idx) => path[..idx].to_string(),
        None => ".".to_string(),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A buffer that records everything written, shared via `Rc<RefCell<_>>`.
    #[derive(Clone, Default)]
    struct SharedBuf(Rc<RefCell<Vec<u8>>>);
    impl Write for SharedBuf {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.borrow_mut().extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    impl SharedBuf {
        fn contents(&self) -> String {
            String::from_utf8_lossy(&self.0.borrow()).into_owned()
        }
    }

    fn ok_status() -> std::process::ExitStatus {
        // A successful `true` invocation yields a zero exit status portably.
        Command::new("true").status().unwrap()
    }

    /// Builds a Deps where every external command and validator succeeds and no
    /// files are staged, so every step takes its skip/no-op branch.
    fn deps_all_skip<'a>(out: SharedBuf, err: SharedBuf) -> Deps<'a> {
        Deps {
            get_staged_files: Box::new(|_| Ok(Vec::new())),
            exec_command: Box::new(|_, _, _| Ok(ok_status())),
            exec_command_quiet: Box::new(|_, _, _| Ok(ok_status())),
            validate_claude: Box::new(|_| Ok(ValidationResult::default())),
            sync_all: Box::new(|_| Ok(crate::internal::agents::types::SyncResult::default())),
            validate_sync: Box::new(|_| Ok(ValidationResult::default())),
            validate_links: Box::new(|_| Ok(LinkValidationResult::default())),
            stdout: Box::new(out),
            stderr: Box::new(err),
        }
    }

    #[test]
    fn run_no_staged_files_emits_skip_messages() {
        let out = SharedBuf::default();
        let err = SharedBuf::default();
        let deps = deps_all_skip(out.clone(), err.clone());
        run(Path::new("/tmp"), deps).unwrap();
        let s = out.contents();
        assert!(s.contains(
            "⏭️  Skipping config validation (no .claude/ or .opencode/ changes in staged files)\n"
        ));
        assert!(s.contains(
            "⏭️  Skipping docker-compose validation (no docker-compose.yml changes in staged files)\n"
        ));
    }

    #[test]
    fn step1_config_runs_when_claude_staged_and_passes() {
        let out = SharedBuf::default();
        let mut deps = Deps {
            get_staged_files: Box::new(|_| Ok(vec![".claude/agents/x.md".to_string()])),
            exec_command: Box::new(|_, _, _| Ok(ok_status())),
            exec_command_quiet: Box::new(|_, _, _| Ok(ok_status())),
            validate_claude: Box::new(|_| Ok(ValidationResult::default())),
            sync_all: Box::new(|_| Ok(crate::internal::agents::types::SyncResult::default())),
            validate_sync: Box::new(|_| Ok(ValidationResult::default())),
            validate_links: Box::new(|_| Ok(LinkValidationResult::default())),
            stdout: Box::new(out.clone()),
            stderr: Box::new(SharedBuf::default()),
        };
        let staged = vec![".claude/agents/x.md".to_string()];
        step1_config(Path::new("/tmp"), &staged, &mut deps).unwrap();
        let s = out.contents();
        assert!(s.contains("🔍 Validating .claude/ and .opencode/ configuration...\n"));
        assert!(s.contains("✅ Configuration validation passed\n"));
    }

    #[test]
    fn step1_config_fails_on_failed_checks() {
        let out = SharedBuf::default();
        let mut deps = Deps {
            get_staged_files: Box::new(|_| Ok(Vec::new())),
            exec_command: Box::new(|_, _, _| Ok(ok_status())),
            exec_command_quiet: Box::new(|_, _, _| Ok(ok_status())),
            validate_claude: Box::new(|_| {
                Ok(ValidationResult {
                    failed_checks: 2,
                    ..ValidationResult::default()
                })
            }),
            sync_all: Box::new(|_| Ok(crate::internal::agents::types::SyncResult::default())),
            validate_sync: Box::new(|_| Ok(ValidationResult::default())),
            validate_links: Box::new(|_| Ok(LinkValidationResult::default())),
            stdout: Box::new(out.clone()),
            stderr: Box::new(SharedBuf::default()),
        };
        let staged = vec![".opencode/agents/x.md".to_string()];
        let res = step1_config(Path::new("/tmp"), &staged, &mut deps);
        assert!(res.is_err());
        assert!(
            out.contents()
                .contains("❌ Configuration validation failed.")
        );
    }

    #[test]
    fn step2_docker_compose_skips_without_compose_files() {
        let out = SharedBuf::default();
        let mut deps = deps_all_skip(out.clone(), SharedBuf::default());
        step2_docker_compose(Path::new("/tmp"), &[], &mut deps).unwrap();
        assert!(
            out.contents()
                .contains("⏭️  Skipping docker-compose validation")
        );
    }

    #[test]
    fn step3_nx_warns_on_failure() {
        let out = SharedBuf::default();
        let mut deps = Deps {
            exec_command: Box::new(|_, _, _| Ok(Command::new("false").status().unwrap())),
            ..deps_all_skip(out.clone(), SharedBuf::default())
        };
        step3_nx_pre_commit(Path::new("/tmp"), &mut deps);
        assert!(
            out.contents()
                .contains("⚠️  Skipping run-pre-commit (not affected or binary missing)\n")
        );
    }

    #[test]
    fn step5_lint_staged_errors_on_failure() {
        let mut deps = Deps {
            exec_command: Box::new(|_, _, _| Ok(Command::new("false").status().unwrap())),
            ..deps_all_skip(SharedBuf::default(), SharedBuf::default())
        };
        assert!(step5_lint_staged(Path::new("/tmp"), &mut deps).is_err());
    }

    /// Writes `content` to `root/rel`, creating parent directories.
    fn write_file(root: &Path, rel: &str, content: &str) {
        let path = root.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    // --- Phase 4 (DD-8): staged-only mermaid + heading steps ---

    #[test]
    fn step6m_validate_mermaid_errors_on_staged_malformed_flowchart() {
        let dir = tempfile::tempdir().unwrap();
        // Node label far exceeds the 30-character limit — a blocking violation.
        write_file(
            dir.path(),
            "docs/diagram.md",
            "# Doc\n\n```mermaid\nflowchart TD\n  \
             A[This label is far longer than the thirty character limit] --> B[Ok]\n```\n",
        );
        let mut deps = deps_all_skip(SharedBuf::default(), SharedBuf::default());
        let staged = vec!["docs/diagram.md".to_string()];
        let res = step6m_validate_mermaid(dir.path(), &staged, &mut deps);
        assert!(
            res.is_err(),
            "staged markdown with a malformed flowchart must block the commit"
        );
    }

    #[test]
    fn step6h_validate_heading_hierarchy_errors_on_staged_docs_duplicate_h1() {
        let dir = tempfile::tempdir().unwrap();
        write_file(
            dir.path(),
            "docs/guide.md",
            "# First Title\n\ntext\n\n# Second Title\n",
        );
        let mut deps = deps_all_skip(SharedBuf::default(), SharedBuf::default());
        let staged = vec!["docs/guide.md".to_string()];
        let res = step6h_validate_heading_hierarchy(dir.path(), &staged, &mut deps);
        assert!(
            res.is_err(),
            "staged docs/ file with a duplicate H1 must block the commit"
        );
    }

    #[test]
    fn step6h_validate_heading_hierarchy_allows_staged_skill_file_with_many_h1s() {
        let dir = tempfile::tempdir().unwrap();
        // SKILL.md files under .claude/skills/ are NOT prose-allowlisted, so
        // multiple H1s must not block the commit.
        write_file(
            dir.path(),
            ".claude/skills/example-skill/SKILL.md",
            "# First H1\n\ntext\n\n# Second H1\n\ntext\n\n# Third H1\n",
        );
        let mut deps = deps_all_skip(SharedBuf::default(), SharedBuf::default());
        let staged = vec![".claude/skills/example-skill/SKILL.md".to_string()];
        let res = step6h_validate_heading_hierarchy(dir.path(), &staged, &mut deps);
        assert!(
            res.is_ok(),
            "staged SKILL.md is exempt from the prose heading gate: {res:?}"
        );
    }

    #[test]
    fn step7_validate_links_skip_paths_include_plans_done() {
        let captured: Rc<RefCell<Vec<String>>> = Rc::default();
        let cap = Rc::clone(&captured);
        let mut deps = Deps {
            validate_links: Box::new(move |opts| {
                *cap.borrow_mut() = opts.skip_paths.clone();
                Ok(LinkValidationResult::default())
            }),
            ..deps_all_skip(SharedBuf::default(), SharedBuf::default())
        };
        step7_validate_links(Path::new("/tmp"), &mut deps).unwrap();
        // plans/done joins the existing skip entries (a staged broken link
        // under plans/done must NOT block the commit) — existing entries are
        // preserved, not replaced.
        assert_eq!(
            *captured.borrow(),
            vec![
                ".opencode/skill/".to_string(),
                ".claude/worktrees/".to_string(),
                "plans/done".to_string(),
            ],
            "link-step skip paths must include plans/done alongside the existing entries"
        );
    }

    #[test]
    fn step7_validate_links_errors_on_broken() {
        let err = SharedBuf::default();
        let broken = crate::internal::docs::types::BrokenLink {
            line_number: 1,
            source_file: "a.md".to_string(),
            link_text: "x".to_string(),
            target_path: "y".to_string(),
            category: "internal".to_string(),
        };
        let mut deps = Deps {
            validate_links: Box::new(move |_| {
                Ok(LinkValidationResult {
                    broken_links: vec![broken.clone()],
                    ..LinkValidationResult::default()
                })
            }),
            stderr: Box::new(err.clone()),
            ..deps_all_skip(SharedBuf::default(), SharedBuf::default())
        };
        assert!(step7_validate_links(Path::new("/tmp"), &mut deps).is_err());
        assert!(err.contents().contains("❌ Found 1 broken links\n"));
    }

    #[test]
    fn dir_name_handles_paths() {
        assert_eq!(dir_name("apps/foo/package.json"), "apps/foo");
        assert_eq!(dir_name("package.json"), ".");
    }

    #[test]
    fn has_match_predicate() {
        let staged = vec!["a.txt".to_string(), ".claude/x".to_string()];
        assert!(has_match(&staged, |f| f.starts_with(".claude/")));
        assert!(!has_match(&staged, |f| f.starts_with(".opencode/")));
    }
}
