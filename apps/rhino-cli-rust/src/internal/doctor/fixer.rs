//! Auto-install ("--fix") engine.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/internal/doctor/fixer.go`. The
//! per-step progress lines, the skip messages, and the one-line summary all
//! match Go exactly.

use std::path::Path;
use std::process::Command;

use super::tools::{InstallStep, build_tool_defs};
use super::types::{DoctorResult, Scope, ToolStatus, is_minimal_tool};

/// Fix behaviour options. Mirrors Go `FixOptions`.
pub struct FixOptions {
    pub dry_run: bool,
}

/// Outcome of a fix attempt. Mirrors Go `FixResult`.
#[derive(Default)]
pub struct FixResult {
    pub fixed: i64,
    pub failed: i64,
    pub already_ok: i64,
    pub skipped: i64,
}

/// Executes an install command. Returns `Ok(())` on success (zero exit).
/// Mirrors Go `FixRunnerFunc` / `realFixRunner` (inherits stdio).
pub type FixRunner<'a> = dyn Fn(&str, &[String]) -> Result<(), String> + 'a;

/// Production runner: spawns the command inheriting the parent's stdio.
pub fn real_fix_runner(command: &str, args: &[String]) -> Result<(), String> {
    let status = Command::new(command).args(args).status();
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(format!("exit status {}", s.code().unwrap_or(-1))),
        Err(e) => Err(e.to_string()),
    }
}

/// Attempts to install missing tools, driven by the per-tool definitions.
/// `printf` receives each progress/skip line (already newline-terminated).
/// Mirrors Go `Fix`.
pub fn fix(
    result: &DoctorResult,
    defs: &[super::tools::ToolDef],
    opts: &FixOptions,
    platform: &str,
    runner: &FixRunner,
    mut printf: impl FnMut(&str),
) -> FixResult {
    let mut fr = FixResult::default();

    for (i, check) in result.checks.iter().enumerate() {
        if check.status == ToolStatus::Ok || check.status == ToolStatus::Warning {
            fr.already_ok += 1;
            continue;
        }
        // StatusMissing.
        let def = defs.get(i);
        let install_cmd = def.and_then(|d| d.install_cmd.as_ref());
        let Some(install_cmd) = install_cmd else {
            printf(&format!(
                "Skip: {} \u{2014} no auto-install available\n",
                check.name
            ));
            fr.skipped += 1;
            continue;
        };

        let steps: Vec<InstallStep> = install_cmd(&check.required_version, platform);
        if steps.is_empty() {
            printf(&format!(
                "Skip: {} \u{2014} no install steps for platform {}\n",
                check.name, platform
            ));
            fr.skipped += 1;
            continue;
        }

        let mut failed = false;
        for step in &steps {
            if opts.dry_run {
                printf(&format!(
                    "Would install: {} via {} {}\n",
                    check.name,
                    step.command,
                    step.args.join(" ")
                ));
                continue;
            }
            printf(&format!(
                "Installing {}: {}\n",
                check.name, step.description
            ));
            if let Err(e) = runner(&step.command, &step.args) {
                printf(&format!("  Failed: {e}\n"));
                fr.failed += 1;
                failed = true;
                break;
            }
        }
        if !opts.dry_run && !failed {
            fr.fixed += 1;
        }
    }

    fr
}

/// Attempts to install missing tools detected by a prior check. Rebuilds the
/// tool defs from the same scope, then runs [`fix`]. Mirrors Go `FixAll`.
pub fn fix_all(
    result: &DoctorResult,
    repo_root: &Path,
    scope: Scope,
    opts: &FixOptions,
    platform: &str,
    runner: &FixRunner,
    printf: impl FnMut(&str),
) -> FixResult {
    let mut defs = build_tool_defs(repo_root);
    if scope == Scope::Minimal {
        defs.retain(|d| is_minimal_tool(d.name));
    }
    fix(result, &defs, opts, platform, runner, printf)
}

/// One-line summary. Mirrors Go `FormatFixSummary`.
pub fn format_fix_summary(fr: &FixResult) -> String {
    format!(
        "\nFix summary: {} fixed, {} failed, {} already OK\n",
        fr.fixed, fr.failed, fr.already_ok
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::internal::doctor::types::{DoctorResult, ToolCheck, ToolStatus};

    fn missing(name: &str, req: &str) -> ToolCheck {
        ToolCheck {
            name: name.to_string(),
            binary: name.to_string(),
            status: ToolStatus::Missing,
            installed_version: String::new(),
            required_version: req.to_string(),
            source: String::new(),
            note: "not found in PATH".to_string(),
        }
    }

    fn ok(name: &str) -> ToolCheck {
        ToolCheck {
            name: name.to_string(),
            binary: name.to_string(),
            status: ToolStatus::Ok,
            installed_version: "1".to_string(),
            required_version: String::new(),
            source: String::new(),
            note: String::new(),
        }
    }

    /// Builds 19 checks in the exact `build_tool_defs` order, marking the named
    /// tools as missing (everything else OK) so that the result indices stay
    /// parallel to the rebuilt defs — the invariant `fix` relies on.
    const DEF_ORDER: &[&str] = &[
        "git",
        "volta",
        "node",
        "npm",
        "java",
        "maven",
        "golang",
        "python",
        "rust",
        "cargo-llvm-cov",
        "elixir",
        "erlang",
        "dotnet",
        "clojure",
        "dart",
        "flutter",
        "docker",
        "jq",
        "playwright",
    ];

    fn aligned_result(missing_names: &[&str]) -> DoctorResult {
        let checks: Vec<ToolCheck> = DEF_ORDER
            .iter()
            .map(|&n| {
                if missing_names.contains(&n) {
                    missing(n, if n == "node" { "24.0.0" } else { "" })
                } else {
                    ok(n)
                }
            })
            .collect();
        DoctorResult {
            checks,
            ok_count: 0,
            warn_count: 0,
            missing_count: missing_names.len() as i64,
            duration_ms: 0,
            scope_raw: "full".to_string(),
        }
    }

    #[test]
    fn dry_run_previews_without_running() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("package.json"),
            "{\"volta\":{\"node\":\"24.0.0\",\"npm\":\"11.0.0\"}}",
        )
        .unwrap();
        let r = aligned_result(&["node"]);
        let mut lines = String::new();
        let never_runs = |_c: &str, _a: &[String]| -> Result<(), String> {
            panic!("runner must not run during dry-run");
        };
        let fr = fix_all(
            &r,
            tmp.path(),
            Scope::Full,
            &FixOptions { dry_run: true },
            "darwin",
            &never_runs,
            |l| lines.push_str(l),
        );
        assert_eq!(fr.already_ok, 18);
        assert_eq!(fr.fixed, 0);
        assert!(lines.contains("Would install: node via volta install node@24.0.0"));
    }

    #[test]
    fn fix_runs_install_and_counts_fixed() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("package.json"),
            "{\"volta\":{\"node\":\"24.0.0\",\"npm\":\"11.0.0\"}}",
        )
        .unwrap();
        let r = aligned_result(&["node"]);
        let mut lines = String::new();
        let succeed = |_c: &str, _a: &[String]| -> Result<(), String> { Ok(()) };
        let fr = fix_all(
            &r,
            tmp.path(),
            Scope::Full,
            &FixOptions { dry_run: false },
            "darwin",
            &succeed,
            |l| lines.push_str(l),
        );
        assert_eq!(fr.fixed, 1);
        assert_eq!(fr.failed, 0);
        assert!(lines.contains("Installing node:"));
    }

    #[test]
    fn fix_records_failure() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("package.json"),
            "{\"volta\":{\"node\":\"24.0.0\",\"npm\":\"11.0.0\"}}",
        )
        .unwrap();
        let r = aligned_result(&["node"]);
        let mut lines = String::new();
        let fail = |_c: &str, _a: &[String]| -> Result<(), String> { Err("boom".to_string()) };
        let fr = fix_all(
            &r,
            tmp.path(),
            Scope::Full,
            &FixOptions { dry_run: false },
            "darwin",
            &fail,
            |l| lines.push_str(l),
        );
        assert_eq!(fr.failed, 1);
        assert_eq!(fr.fixed, 0);
        assert!(lines.contains("  Failed: boom"));
    }

    #[test]
    fn skips_tool_without_install_command() {
        // dart (install_cmd = None) is missing → "no auto-install available".
        let tmp = tempfile::tempdir().unwrap();
        let r = aligned_result(&["dart"]);
        let mut lines = String::new();
        let noop = |_c: &str, _a: &[String]| -> Result<(), String> { Ok(()) };
        let fr = fix_all(
            &r,
            tmp.path(),
            Scope::Full,
            &FixOptions { dry_run: false },
            "darwin",
            &noop,
            |l| lines.push_str(l),
        );
        assert_eq!(fr.skipped, 1);
        assert!(lines.contains("Skip: dart \u{2014} no auto-install available"));
    }

    #[test]
    fn skips_tool_with_no_platform_steps() {
        // docker on darwin yields zero install steps → platform-skip branch.
        let tmp = tempfile::tempdir().unwrap();
        let r = aligned_result(&["docker"]);
        let mut lines = String::new();
        let noop = |_c: &str, _a: &[String]| -> Result<(), String> { Ok(()) };
        let fr = fix_all(
            &r,
            tmp.path(),
            Scope::Full,
            &FixOptions { dry_run: false },
            "darwin",
            &noop,
            |l| lines.push_str(l),
        );
        assert_eq!(fr.skipped, 1);
        assert!(lines.contains("Skip: docker \u{2014} no install steps for platform darwin"));
    }

    #[test]
    fn summary_string_matches_go() {
        let fr = FixResult {
            fixed: 2,
            failed: 1,
            already_ok: 16,
            skipped: 0,
        };
        assert_eq!(
            format_fix_summary(&fr),
            "\nFix summary: 2 fixed, 1 failed, 16 already OK\n"
        );
    }
}
