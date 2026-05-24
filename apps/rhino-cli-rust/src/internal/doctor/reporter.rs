//! Output formatting for doctor results.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/internal/doctor/reporter.go`. Text
//! uses Go's `%-10s %-14s` column widths; JSON mirrors `JSONOutput` field order
//! and `omitempty` semantics; markdown mirrors the table layout. JSON
//! timestamps use the same RFC3339-with-offset shape as `timeutil.Timestamp()`.

use std::fmt::Write as _;

use anyhow::Error;
use serde::Serialize;

use crate::internal::cliout::gojson;

use super::types::{DoctorResult, ToolCheck, ToolStatus};

/// Returns the current timestamp in Go's `time.RFC3339` format with a numeric
/// offset, matching `timeutil.Timestamp()` and the existing reporters.
fn timestamp() -> String {
    chrono::Local::now()
        .format("%Y-%m-%dT%H:%M:%S%:z")
        .to_string()
}

fn symbol_for(status: ToolStatus) -> &'static str {
    match status {
        ToolStatus::Ok => "\u{2713}",      // ✓
        ToolStatus::Warning => "\u{26a0}", // ⚠
        ToolStatus::Missing => "\u{2717}", // ✗
    }
}

fn display_version(check: &ToolCheck) -> String {
    if check.status == ToolStatus::Missing {
        return "not found".to_string();
    }
    if check.installed_version.is_empty() {
        return "(unknown)".to_string();
    }
    format!("v{}", check.installed_version)
}

fn overall_status(result: &DoctorResult) -> &'static str {
    if result.missing_count > 0 {
        "missing"
    } else if result.warn_count > 0 {
        "warning"
    } else {
        "ok"
    }
}

/// Left-pads a display string to `width` *runes* with trailing spaces, matching
/// Go's `fmt`'s `%-Ns` (which counts runes, not bytes). The symbol/version
/// columns contain multi-byte runes (✓, ≥, v-prefixed versions), so byte-based
/// padding would diverge.
fn pad_left_runes(s: &str, width: usize) -> String {
    let count = s.chars().count();
    if count >= width {
        s.to_string()
    } else {
        let mut out = String::with_capacity(s.len() + (width - count));
        out.push_str(s);
        for _ in 0..(width - count) {
            out.push(' ');
        }
        out
    }
}

/// Formats the doctor result as human-readable text. Mirrors Go `FormatText`.
/// The verbose `Duration:` line is wall-clock dependent (normalised in the
/// shadow-diff harness); the round-to-ms rendering matches Go's `%v`.
pub fn format_text(result: &DoctorResult, verbose: bool, quiet: bool) -> String {
    let mut sb = String::new();

    if !quiet {
        sb.push_str("Doctor Report\n");
        sb.push_str("=============\n\n");
    }

    for check in &result.checks {
        let sym = symbol_for(check.status);
        let ver = display_version(check);
        let _ = writeln!(
            sb,
            "{} {} {} ({})",
            sym,
            pad_left_runes(&check.name, 10),
            pad_left_runes(&ver, 14),
            check.note
        );
    }

    let total = result.ok_count + result.warn_count + result.missing_count;
    let mut summary = format!(
        "\nSummary: {}/{} tools OK, {} warning, {} missing",
        result.ok_count, total, result.warn_count, result.missing_count
    );
    if result.scope_raw == "minimal" {
        summary.push_str(" (scope: minimal)");
    }
    sb.push_str(&summary);
    sb.push('\n');

    if verbose {
        let _ = writeln!(sb, "Duration: {}", render_duration_ms(result.duration_ms));
    }

    sb
}

/// Renders milliseconds as Go's `time.Duration` `%v` after `Round(Millisecond)`.
/// Only the millisecond-and-up scale ever appears here. This output is masked by
/// the shadow-diff normaliser, so exact sub-forms need not match byte-for-byte.
fn render_duration_ms(ms: i64) -> String {
    if ms == 0 {
        return "0s".to_string();
    }
    if ms < 1000 {
        return format!("{ms}ms");
    }
    let secs = ms as f64 / 1000.0;
    format!("{secs}s")
}

/// One tool entry in JSON output. Field order + `omitempty` mirror Go
/// `JSONToolItem`.
#[derive(Serialize)]
struct JsonToolItem {
    name: String,
    binary: String,
    status: String,
    #[serde(rename = "installed_version", skip_serializing_if = "String::is_empty")]
    installed_version: String,
    #[serde(rename = "required_version", skip_serializing_if = "String::is_empty")]
    required_version: String,
    #[serde(rename = "source", skip_serializing_if = "String::is_empty")]
    source: String,
    #[serde(rename = "note", skip_serializing_if = "String::is_empty")]
    note: String,
}

/// JSON output document. Mirrors Go `JSONOutput`.
#[derive(Serialize)]
struct JsonOutput {
    status: String,
    #[serde(rename = "scope", skip_serializing_if = "String::is_empty")]
    scope: String,
    timestamp: String,
    ok_count: i64,
    warn_count: i64,
    missing_count: i64,
    duration_ms: i64,
    tools: Vec<JsonToolItem>,
}

/// Formats the doctor result as JSON. Mirrors Go `FormatJSON`.
pub fn format_json(result: &DoctorResult) -> Result<String, Error> {
    let tools: Vec<JsonToolItem> = result
        .checks
        .iter()
        .map(|check| JsonToolItem {
            name: check.name.clone(),
            binary: check.binary.clone(),
            status: check.status.code().to_string(),
            installed_version: check.installed_version.clone(),
            required_version: check.required_version.clone(),
            source: check.source.clone(),
            note: check.note.clone(),
        })
        .collect();

    let out = JsonOutput {
        status: overall_status(result).to_string(),
        scope: result.scope_raw.clone(),
        timestamp: timestamp(),
        ok_count: result.ok_count,
        warn_count: result.warn_count,
        missing_count: result.missing_count,
        duration_ms: result.duration_ms,
        tools,
    };

    let body = serde_json::to_string_pretty(&out)?;
    Ok(gojson::html_escape(&body))
}

/// Formats the doctor result as a markdown report. Mirrors Go `FormatMarkdown`.
pub fn format_markdown(result: &DoctorResult) -> String {
    let mut sb = String::new();

    sb.push_str("## Doctor Report\n\n");
    let _ = write!(sb, "**Generated**: {}\n\n", timestamp());

    let total = result.ok_count + result.warn_count + result.missing_count;
    sb.push_str("### Summary\n\n");
    sb.push_str("| Metric | Value |\n");
    sb.push_str("|--------|-------|\n");
    let _ = writeln!(sb, "| OK | {} |", result.ok_count);
    let _ = writeln!(sb, "| Warning | {} |", result.warn_count);
    let _ = writeln!(sb, "| Missing | {} |", result.missing_count);
    let _ = writeln!(sb, "| Total | {total} |");
    sb.push('\n');

    sb.push_str("### Tools\n\n");
    sb.push_str("| Tool | Status | Installed | Required | Note |\n");
    sb.push_str("|------|--------|-----------|----------|------|\n");

    for check in &result.checks {
        let sym = symbol_for(check.status);
        let ver = display_version(check);
        let _ = writeln!(
            sb,
            "| {} | {} {} | {} | {} | {} |",
            check.name,
            sym,
            check.status.code(),
            ver,
            check.required_version,
            check.note
        );
    }

    sb
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn check(name: &str, status: ToolStatus, installed: &str, note: &str) -> ToolCheck {
        ToolCheck {
            name: name.to_string(),
            binary: name.to_string(),
            status,
            installed_version: installed.to_string(),
            required_version: String::new(),
            source: "(no config file)".to_string(),
            note: note.to_string(),
        }
    }

    fn result(checks: Vec<ToolCheck>, scope_raw: &str) -> DoctorResult {
        let mut r = DoctorResult {
            checks,
            ok_count: 0,
            warn_count: 0,
            missing_count: 0,
            duration_ms: 5,
            scope_raw: scope_raw.to_string(),
        };
        for c in &r.checks {
            match c.status {
                ToolStatus::Ok => r.ok_count += 1,
                ToolStatus::Warning => r.warn_count += 1,
                ToolStatus::Missing => r.missing_count += 1,
            }
        }
        r
    }

    #[test]
    fn symbols_and_display_version() {
        assert_eq!(symbol_for(ToolStatus::Ok), "\u{2713}");
        assert_eq!(symbol_for(ToolStatus::Warning), "\u{26a0}");
        assert_eq!(symbol_for(ToolStatus::Missing), "\u{2717}");
        assert_eq!(
            display_version(&check("x", ToolStatus::Missing, "", "")),
            "not found"
        );
        assert_eq!(
            display_version(&check("x", ToolStatus::Ok, "", "")),
            "(unknown)"
        );
        assert_eq!(
            display_version(&check("x", ToolStatus::Ok, "1.2.3", "")),
            "v1.2.3"
        );
    }

    #[test]
    fn rune_padding_counts_runes_not_bytes() {
        // "✓" is one rune but 3 bytes; padding must count runes.
        assert_eq!(pad_left_runes("ab", 4), "ab  ");
        assert_eq!(pad_left_runes("abcd", 2), "abcd");
    }

    #[test]
    fn text_format_columns_and_summary() {
        let r = result(
            vec![
                check("git", ToolStatus::Ok, "2.43.0", "no version requirement"),
                check("node", ToolStatus::Missing, "", "not found in PATH"),
            ],
            "full",
        );
        let out = format_text(&r, false, false);
        assert!(out.starts_with("Doctor Report\n=============\n\n"));
        assert!(out.contains("\u{2713} git        v2.43.0        (no version requirement)\n"));
        assert!(out.contains("\u{2717} node       not found      (not found in PATH)\n"));
        assert!(out.contains("\nSummary: 1/2 tools OK, 0 warning, 1 missing\n"));
    }

    #[test]
    fn text_minimal_scope_suffix_and_quiet() {
        let r = result(vec![check("git", ToolStatus::Ok, "2.43.0", "")], "minimal");
        assert!(format_text(&r, false, false).contains("(scope: minimal)"));
        // Quiet omits the header.
        let q = format_text(&r, false, true);
        assert!(!q.contains("Doctor Report"));
        assert!(q.starts_with("\u{2713} git"));
    }

    #[test]
    fn text_verbose_appends_duration_line() {
        let r = result(vec![check("git", ToolStatus::Ok, "2.43.0", "")], "full");
        assert!(format_text(&r, true, false).contains("\nDuration: "));
    }

    #[test]
    fn json_omits_empty_fields_and_lists_tools() {
        let r = result(
            vec![check(
                "git",
                ToolStatus::Ok,
                "2.43.0",
                "no version requirement",
            )],
            "full",
        );
        let out = format_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["status"], "ok");
        assert_eq!(v["scope"], "full");
        assert_eq!(v["tools"].as_array().unwrap().len(), 1);
        assert_eq!(v["tools"][0]["name"], "git");
        // required_version empty → omitted.
        assert!(v["tools"][0].get("required_version").is_none());
    }

    #[test]
    fn json_status_reflects_worst() {
        let missing = result(vec![check("g", ToolStatus::Missing, "", "")], "full");
        assert_eq!(overall_status(&missing), "missing");
        let warn = result(vec![check("g", ToolStatus::Warning, "1", "")], "full");
        assert_eq!(overall_status(&warn), "warning");
        let ok = result(vec![check("g", ToolStatus::Ok, "1", "")], "full");
        assert_eq!(overall_status(&ok), "ok");
    }

    #[test]
    fn markdown_table_and_summary() {
        let r = result(vec![check("git", ToolStatus::Ok, "2.43.0", "ok")], "full");
        let out = format_markdown(&r);
        assert!(out.contains("## Doctor Report"));
        assert!(out.contains("| OK | 1 |"));
        assert!(out.contains("| Tool | Status | Installed | Required | Note |"));
        assert!(out.contains("| git | \u{2713} ok | v2.43.0 |  | ok |"));
    }

    #[test]
    fn duration_render_branches() {
        assert_eq!(render_duration_ms(0), "0s");
        assert_eq!(render_duration_ms(5), "5ms");
        assert_eq!(render_duration_ms(1500), "1.5s");
    }
}
