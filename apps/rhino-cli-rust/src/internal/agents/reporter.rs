//! Output formatting for sync, validation, and naming results.
//!
//! JSON timestamps use the same RFC3339-with-offset shape as Go's `timeutil.Timestamp()`.

use std::fmt::Write as _;
use std::time::Duration;

use anyhow::Error;
use serde::Serialize;

use super::naming::Violation;
use super::types::{SyncResult, ValidationCheck, ValidationResult};

/// Returns the current timestamp in Go's `time.RFC3339` format with a numeric
/// offset, matching `timeutil.Timestamp()` / the existing Phase-4 reporters.
fn timestamp() -> String {
    chrono::Local::now()
        .format("%Y-%m-%dT%H:%M:%S%:z")
        .to_string()
}

/// Formats a `Duration` the way Go's `fmt` renders a `time.Duration` (`%v`):
/// e.g. `4.282625ms`, `1.5s`, `512µs`, `0s`. Used in text/markdown output.
fn go_duration(d: Duration) -> String {
    let nanos = d.as_nanos();
    if nanos == 0 {
        return "0s".to_string();
    }
    if nanos < 1_000 {
        return format!("{nanos}ns");
    }
    if nanos < 1_000_000 {
        return format!("{}µs", trim_frac(nanos as f64 / 1_000.0));
    }
    if nanos < 1_000_000_000 {
        return format!("{}ms", trim_frac(nanos as f64 / 1_000_000.0));
    }
    // Seconds (and above). Go switches to h/m/s composition above 1s; the
    // agent operations complete well under a minute, so seconds suffice.
    format!("{}s", trim_frac(nanos as f64 / 1_000_000_000.0))
}

/// Formats a float with up to 9 fractional digits, trimming trailing zeros and
/// a trailing dot.
fn trim_frac(v: f64) -> String {
    let mut s = format!("{v:.9}");
    while s.contains('.') && s.ends_with('0') {
        s.pop();
    }
    if s.ends_with('.') {
        s.pop();
    }
    s
}

// ---------------------------------------------------------------------------
// Sync formatting
// ---------------------------------------------------------------------------

/// Formats sync results as plain text.
pub fn format_sync_text(result: &SyncResult, _verbose: bool, quiet: bool) -> String {
    let mut sb = String::new();

    if !quiet {
        sb.push_str("Sync Complete\n");
        sb.push_str(&"=".repeat(50));
        sb.push_str("\n\n");
    }

    let _ = write!(sb, "Agents: {} converted", result.agents_converted);
    if result.agents_failed > 0 {
        let _ = write!(sb, ", {} failed", result.agents_failed);
    }
    sb.push('\n');

    let _ = write!(sb, "Skills: {} copied", result.skills_copied);
    if result.skills_failed > 0 {
        let _ = write!(sb, ", {} failed", result.skills_failed);
    }
    sb.push('\n');

    let _ = writeln!(sb, "Duration: {}", go_duration(result.duration));

    if !result.failed_files.is_empty() {
        sb.push_str("\nFailed Files:\n");
        for file in &result.failed_files {
            let _ = writeln!(sb, "  - {file}");
        }
    }

    if !quiet {
        sb.push('\n');
        if result.failed_files.is_empty() {
            sb.push_str("Status: ✓ SUCCESS\n");
        } else {
            sb.push_str("Status: ❌ FAILED\n");
        }
    }

    sb
}

/// JSON output shape for sync results.
#[derive(Serialize)]
struct SyncJsonOutput {
    status: &'static str,
    timestamp: String,
    agents_converted: i64,
    agents_failed: i64,
    skills_copied: i64,
    skills_failed: i64,
    failed_files: Vec<String>,
    duration_ms: i64,
}

/// Formats sync results as JSON.
pub fn format_sync_json(result: &SyncResult) -> Result<String, Error> {
    let status = if result.failed_files.is_empty() {
        "success"
    } else {
        "failure"
    };

    let out = SyncJsonOutput {
        status,
        timestamp: timestamp(),
        agents_converted: result.agents_converted,
        agents_failed: result.agents_failed,
        skills_copied: result.skills_copied,
        skills_failed: result.skills_failed,
        failed_files: result.failed_files.clone(),
        duration_ms: i64::try_from(result.duration.as_millis()).unwrap_or(i64::MAX),
    };

    Ok(crate::internal::cliout::gojson::html_escape(
        &serde_json::to_string_pretty(&out)?,
    ))
}

/// Formats sync results as markdown.
pub fn format_sync_markdown(result: &SyncResult) -> String {
    let mut sb = String::new();
    sb.push_str("# Sync Results\n\n");
    sb.push_str("## Summary\n\n");
    let _ = writeln!(sb, "- **Agents Converted**: {}", result.agents_converted);
    if result.agents_failed > 0 {
        let _ = writeln!(sb, "- **Agents Failed**: {}", result.agents_failed);
    }
    let _ = writeln!(sb, "- **Skills Copied**: {}", result.skills_copied);
    if result.skills_failed > 0 {
        let _ = writeln!(sb, "- **Skills Failed**: {}", result.skills_failed);
    }
    let _ = write!(sb, "- **Duration**: {}\n\n", go_duration(result.duration));

    if !result.failed_files.is_empty() {
        sb.push_str("## Failed Files\n\n");
        for file in &result.failed_files {
            let _ = writeln!(sb, "- `{file}`");
        }
        sb.push('\n');
    }

    if result.failed_files.is_empty() {
        sb.push_str("**Status**: ✓ SUCCESS\n");
    } else {
        sb.push_str("**Status**: ❌ FAILED\n");
    }

    sb
}

// ---------------------------------------------------------------------------
// Validation formatting
// ---------------------------------------------------------------------------

/// Formats validation results as plain text.
pub fn format_validation_text(result: &ValidationResult, verbose: bool, quiet: bool) -> String {
    let mut sb = String::new();

    if !quiet {
        sb.push_str("Validation Complete\n");
        sb.push_str(&"=".repeat(50));
        sb.push_str("\n\n");
    }

    let _ = writeln!(sb, "Total Checks: {}", result.total_checks);
    let _ = writeln!(sb, "Passed: {}", result.passed_checks);
    let _ = writeln!(sb, "Failed: {}", result.failed_checks);
    let _ = writeln!(sb, "Duration: {}", go_duration(result.duration));

    if result.failed_checks > 0 {
        sb.push_str("\nFailed Checks:\n");
        for check in &result.checks {
            if check.status == "failed" {
                let _ = write!(sb, "\n  ❌ {}\n", check.name);
                if !check.expected.is_empty() {
                    let _ = writeln!(sb, "     Expected: {}", check.expected);
                }
                if !check.actual.is_empty() {
                    let _ = writeln!(sb, "     Actual: {}", check.actual);
                }
                if !check.message.is_empty() {
                    let _ = writeln!(sb, "     Message: {}", check.message);
                }
            }
        }
    }

    if verbose {
        sb.push_str("\nAll Checks:\n");
        for check in &result.checks {
            if check.status == "passed" {
                let _ = writeln!(sb, "  ✓ {}", check.name);
            } else {
                let _ = writeln!(sb, "  ❌ {}", check.name);
            }
            if verbose && !check.message.is_empty() {
                let _ = writeln!(sb, "     {}", check.message);
            }
        }
    }

    if !quiet {
        sb.push('\n');
        if result.failed_checks > 0 {
            sb.push_str("Status: ❌ VALIDATION FAILED\n");
        } else {
            sb.push_str("Status: ✓ VALIDATION PASSED\n");
        }
    }

    sb
}

/// JSON output shape for validation results.
#[derive(Serialize)]
struct ValidationJsonOutput {
    status: &'static str,
    timestamp: String,
    total_checks: i64,
    passed_checks: i64,
    failed_checks: i64,
    duration_ms: i64,
    checks: Vec<ValidationJsonCheck>,
}

/// A single check in JSON form.: expected,
/// actual, message are `omitempty`.
#[derive(Serialize)]
struct ValidationJsonCheck {
    name: String,
    status: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    expected: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    actual: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    message: String,
}

impl From<&ValidationCheck> for ValidationJsonCheck {
    fn from(c: &ValidationCheck) -> Self {
        Self {
            name: c.name.clone(),
            status: c.status.clone(),
            expected: c.expected.clone(),
            actual: c.actual.clone(),
            message: c.message.clone(),
        }
    }
}

/// Formats validation results as JSON.
pub fn format_validation_json(result: &ValidationResult) -> Result<String, Error> {
    let status = if result.failed_checks > 0 {
        "failure"
    } else {
        "success"
    };

    let checks: Vec<ValidationJsonCheck> = result
        .checks
        .iter()
        .map(ValidationJsonCheck::from)
        .collect();

    let out = ValidationJsonOutput {
        status,
        timestamp: timestamp(),
        total_checks: result.total_checks,
        passed_checks: result.passed_checks,
        failed_checks: result.failed_checks,
        duration_ms: i64::try_from(result.duration.as_millis()).unwrap_or(i64::MAX),
        checks,
    };

    Ok(crate::internal::cliout::gojson::html_escape(
        &serde_json::to_string_pretty(&out)?,
    ))
}

/// Formats validation results as markdown.
pub fn format_validation_markdown(result: &ValidationResult, verbose: bool) -> String {
    let mut sb = String::new();
    sb.push_str("# Validation Results\n\n");
    sb.push_str("## Summary\n\n");
    let _ = writeln!(sb, "- **Total Checks**: {}", result.total_checks);
    let _ = writeln!(sb, "- **Passed**: {}", result.passed_checks);
    let _ = writeln!(sb, "- **Failed**: {}", result.failed_checks);
    let _ = write!(sb, "- **Duration**: {}\n\n", go_duration(result.duration));

    if result.failed_checks > 0 {
        sb.push_str("## Failed Checks\n\n");
        for check in &result.checks {
            if check.status == "failed" {
                let _ = write!(sb, "### ❌ {}\n\n", check.name);
                if !check.expected.is_empty() {
                    let _ = writeln!(sb, "- **Expected**: {}", check.expected);
                }
                if !check.actual.is_empty() {
                    let _ = writeln!(sb, "- **Actual**: {}", check.actual);
                }
                if !check.message.is_empty() {
                    let _ = writeln!(sb, "- **Message**: {}", check.message);
                }
                sb.push('\n');
            }
        }
    }

    if verbose {
        sb.push_str("## All Checks\n\n");
        for check in &result.checks {
            if check.status == "passed" {
                let _ = write!(sb, "- ✓ {}", check.name);
            } else {
                let _ = write!(sb, "- ❌ {}", check.name);
            }
            if !check.message.is_empty() {
                let _ = write!(sb, " - {}", check.message);
            }
            sb.push('\n');
        }
        sb.push('\n');
    }

    if result.failed_checks > 0 {
        sb.push_str("**Status**: ❌ VALIDATION FAILED\n");
    } else {
        sb.push_str("**Status**: ✓ VALIDATION PASSED\n");
    }

    sb
}

// ---------------------------------------------------------------------------
// Naming formatting
// ---------------------------------------------------------------------------
//
// The naming formatters are shared verbatim with `workflows validate-naming`,
// so they live in `crate::internal::naming::reporter`. These thin wrappers
// preserve the existing `agents::reporter::format_naming_*` call sites.

/// Renders a human-readable summary of naming violations. Delegates to the
/// shared reporter.
pub fn format_naming_text(
    label: &str,
    violations: &[Violation],
    verbose: bool,
    quiet: bool,
) -> String {
    crate::internal::naming::reporter::format_naming_text(label, violations, verbose, quiet)
}

/// JSON report for naming violations. Delegates to the shared reporter. Mirrors
/// Go `formatNamingJSON`.
pub fn format_naming_json(kind: &str, violations: &[Violation]) -> Result<String, Error> {
    crate::internal::naming::reporter::format_naming_json(kind, violations)
}

/// PR-friendly markdown table for naming violations. Delegates to the shared
/// reporter.
pub fn format_naming_markdown(label: &str, violations: &[Violation]) -> String {
    crate::internal::naming::reporter::format_naming_markdown(label, violations)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn go_duration_units() {
        assert_eq!(go_duration(Duration::from_secs(0)), "0s");
        assert_eq!(go_duration(Duration::from_nanos(500)), "500ns");
        assert_eq!(go_duration(Duration::from_micros(512)), "512µs");
        assert_eq!(go_duration(Duration::from_millis(4)), "4ms");
        assert_eq!(go_duration(Duration::from_millis(1500)), "1.5s");
    }

    #[test]
    fn sync_text_success() {
        let r = SyncResult {
            agents_converted: 49,
            duration: Duration::from_millis(4),
            ..Default::default()
        };
        let out = format_sync_text(&r, false, false);
        assert!(out.starts_with("Sync Complete\n"));
        assert!(out.contains("Agents: 49 converted\n"));
        assert!(out.contains("Skills: 0 copied\n"));
        assert!(out.ends_with("Status: ✓ SUCCESS\n"));
    }

    #[test]
    fn sync_text_quiet_omits_header_and_footer() {
        let r = SyncResult {
            agents_converted: 1,
            ..Default::default()
        };
        let out = format_sync_text(&r, false, true);
        assert!(!out.contains("Sync Complete"));
        assert!(!out.contains("Status:"));
        assert!(out.contains("Agents: 1 converted"));
    }

    #[test]
    fn sync_json_shape() {
        let r = SyncResult {
            agents_converted: 49,
            duration: Duration::from_millis(4),
            ..Default::default()
        };
        let json = format_sync_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["status"], "success");
        assert_eq!(v["agents_converted"], 49);
        assert_eq!(v["skills_copied"], 0);
        assert_eq!(v["failed_files"], serde_json::json!([]));
        assert_eq!(v["duration_ms"], 4);
    }

    #[test]
    fn validation_text_passed() {
        let r = ValidationResult {
            total_checks: 52,
            passed_checks: 52,
            failed_checks: 0,
            checks: vec![],
            duration: Duration::from_millis(12),
        };
        let out = format_validation_text(&r, false, false);
        assert!(out.contains("Total Checks: 52\n"));
        assert!(out.ends_with("Status: ✓ VALIDATION PASSED\n"));
    }

    #[test]
    fn validation_json_omits_empty_fields() {
        let r = ValidationResult {
            total_checks: 1,
            passed_checks: 1,
            failed_checks: 0,
            checks: vec![ValidationCheck::passed("Agent: x", "ok")],
            duration: Duration::from_millis(1),
        };
        let json = format_validation_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let check = &v["checks"][0];
        assert_eq!(check["name"], "Agent: x");
        assert_eq!(check["status"], "passed");
        // expected/actual omitted (empty), message present.
        assert!(check.get("expected").is_none());
        assert_eq!(check["message"], "ok");
    }

    #[test]
    fn naming_text_pass() {
        let out = format_naming_text("Agents", &[], false, false);
        assert_eq!(
            out,
            "Agents naming validation: VALIDATION PASSED (0 violations)\n"
        );
    }

    #[test]
    fn naming_json_trailing_newline() {
        let json = format_naming_json("agents", &[]).unwrap();
        assert!(json.ends_with('\n'));
        let trimmed = json.trim_end();
        let v: serde_json::Value = serde_json::from_str(trimmed).unwrap();
        assert_eq!(v["kind"], "agents");
        assert_eq!(v["count"], 0);
        assert_eq!(v["violations"], serde_json::json!([]));
    }

    #[test]
    fn naming_markdown_with_violations() {
        let viols = vec![Violation {
            path: "/x/foo-bar.md".to_string(),
            kind: "role-suffix".to_string(),
            message: "msg".to_string(),
        }];
        let out = format_naming_markdown("Agents", &viols);
        assert!(out.contains("| Kind | Path | Message |"));
        assert!(out.contains("| role-suffix | `/x/foo-bar.md` | msg |"));
    }

    // --- additional coverage of the failure / verbose / markdown branches ---

    fn failed_check() -> ValidationCheck {
        ValidationCheck {
            name: "Agent: x.md".to_string(),
            status: "failed".to_string(),
            expected: "exp".to_string(),
            actual: "act".to_string(),
            message: "msg".to_string(),
        }
    }

    fn failing_result() -> ValidationResult {
        ValidationResult {
            total_checks: 2,
            passed_checks: 1,
            failed_checks: 1,
            checks: vec![
                ValidationCheck::passed("Agent: ok.md", "fine"),
                failed_check(),
            ],
            duration: Duration::from_millis(3),
        }
    }

    #[test]
    fn validation_text_failed_lists_details() {
        let out = format_validation_text(&failing_result(), false, false);
        assert!(out.contains("Failed Checks:"));
        assert!(out.contains("❌ Agent: x.md"));
        assert!(out.contains("Expected: exp"));
        assert!(out.contains("Actual: act"));
        assert!(out.contains("Message: msg"));
        assert!(out.ends_with("Status: ❌ VALIDATION FAILED\n"));
    }

    #[test]
    fn validation_text_verbose_lists_all() {
        let out = format_validation_text(&failing_result(), true, false);
        assert!(out.contains("All Checks:"));
        assert!(out.contains("✓ Agent: ok.md"));
        assert!(out.contains("❌ Agent: x.md"));
    }

    #[test]
    fn validation_markdown_failed_and_verbose() {
        let out = format_validation_markdown(&failing_result(), true);
        assert!(out.contains("## Failed Checks"));
        assert!(out.contains("### ❌ Agent: x.md"));
        assert!(out.contains("- **Expected**: exp"));
        assert!(out.contains("- **Actual**: act"));
        assert!(out.contains("- **Message**: msg"));
        assert!(out.contains("## All Checks"));
        assert!(out.contains("- ✓ Agent: ok.md"));
        assert!(out.ends_with("**Status**: ❌ VALIDATION FAILED\n"));
    }

    #[test]
    fn validation_markdown_passed() {
        let r = ValidationResult {
            total_checks: 1,
            passed_checks: 1,
            failed_checks: 0,
            checks: vec![ValidationCheck::passed("Agent: ok.md", "fine")],
            duration: Duration::from_millis(1),
        };
        let out = format_validation_markdown(&r, false);
        assert!(out.starts_with("# Validation Results\n"));
        assert!(out.ends_with("**Status**: ✓ VALIDATION PASSED\n"));
    }

    #[test]
    fn sync_text_with_failures() {
        let r = SyncResult {
            agents_converted: 5,
            agents_failed: 2,
            skills_failed: 1,
            failed_files: vec!["a.md".to_string(), "b.md".to_string()],
            duration: Duration::from_millis(7),
            ..Default::default()
        };
        let out = format_sync_text(&r, false, false);
        assert!(out.contains("Agents: 5 converted, 2 failed\n"));
        assert!(out.contains("Skills: 0 copied, 1 failed\n"));
        assert!(out.contains("Failed Files:\n  - a.md\n  - b.md\n"));
        assert!(out.ends_with("Status: ❌ FAILED\n"));
    }

    #[test]
    fn sync_markdown_success_and_failure() {
        let ok = SyncResult {
            agents_converted: 49,
            duration: Duration::from_millis(4),
            ..Default::default()
        };
        let out = format_sync_markdown(&ok);
        assert!(out.starts_with("# Sync Results\n"));
        assert!(out.contains("- **Agents Converted**: 49\n"));
        assert!(out.ends_with("**Status**: ✓ SUCCESS\n"));

        let bad = SyncResult {
            agents_converted: 1,
            agents_failed: 1,
            skills_failed: 1,
            failed_files: vec!["x.md".to_string()],
            duration: Duration::from_millis(2),
            ..Default::default()
        };
        let out = format_sync_markdown(&bad);
        assert!(out.contains("- **Agents Failed**: 1\n"));
        assert!(out.contains("- **Skills Failed**: 1\n"));
        assert!(out.contains("## Failed Files\n\n- `x.md`\n"));
        assert!(out.ends_with("**Status**: ❌ FAILED\n"));
    }

    #[test]
    fn sync_json_failure_status() {
        let r = SyncResult {
            agents_converted: 1,
            agents_failed: 1,
            failed_files: vec!["x.md".to_string()],
            duration: Duration::from_millis(2),
            ..Default::default()
        };
        let json = format_sync_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["status"], "failure");
        assert_eq!(v["failed_files"], serde_json::json!(["x.md"]));
    }

    #[test]
    fn naming_text_with_violations_and_verbose() {
        let viols = vec![Violation {
            path: "/x/foo-bar.md".to_string(),
            kind: "role-suffix".to_string(),
            message: "msg".to_string(),
        }];
        let out = format_naming_text("Agents", &viols, true, false);
        assert!(out.contains("Agents naming validation: 1 violation(s)\n"));
        assert!(out.contains("[role-suffix] /x/foo-bar.md — msg\n"));
        assert!(out.contains("agent-naming.md"));
    }

    #[test]
    fn naming_text_quiet_pass_is_empty() {
        let out = format_naming_text("Agents", &[], false, true);
        assert_eq!(out, "");
    }

    #[test]
    fn naming_markdown_pass() {
        let out = format_naming_markdown("Agents", &[]);
        assert!(out.contains("## Agents naming validation"));
        assert!(out.contains("All files conform to the naming convention.\n"));
    }

    #[test]
    fn naming_json_with_violations() {
        let viols = vec![Violation {
            path: "/x/foo-bar.md".to_string(),
            kind: "role-suffix".to_string(),
            message: "msg".to_string(),
        }];
        let json = format_naming_json("agents", &viols).unwrap();
        let v: serde_json::Value = serde_json::from_str(json.trim_end()).unwrap();
        assert_eq!(v["count"], 1);
        assert_eq!(v["violations"][0]["Path"], "/x/foo-bar.md");
        assert_eq!(v["violations"][0]["Kind"], "role-suffix");
    }
}
