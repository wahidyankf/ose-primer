//! Output formatting for backup/restore results.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/internal/envbackup/reporter.go`.
//! JSON uses Go `encoding/json` `MarshalIndent` semantics (two-space indent,
//! `omitempty`, HTML escaping via [`crate::internal::cliout::gojson`]).

use std::fmt::Write as _;

use anyhow::Error;
use serde::Serialize;

use crate::internal::cliout::gojson;

use super::types::EnvResult;

/// Returns `s` with the first byte upper-cased. Mirrors Go `capitalize`, which
/// upper-cases `s[:1]` (a single byte). All call sites pass ASCII.
fn capitalize(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap_or_default();
    format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
}

/// Renders a human-readable summary. Mirrors Go `FormatText`.
pub fn format_text(r: &EnvResult, verbose: bool, quiet: bool) -> String {
    let mut sb = String::new();

    // Handle cancelled result.
    if r.cancelled {
        let label = if r.direction.is_empty() {
            "operation"
        } else {
            &r.direction
        };
        let _ = writeln!(sb, "{} cancelled.", capitalize(label));
        return sb;
    }

    if !quiet {
        // Per-file lines.
        for f in &r.files {
            if f.skipped {
                if verbose {
                    let _ = writeln!(sb, "  SKIPPED  {}  ({})", f.rel_path, f.reason);
                }
                continue;
            }
            let tag = if f.source == "config" {
                " [config]"
            } else {
                ""
            };
            let _ = writeln!(
                sb,
                "  {}  {}{}",
                r.direction.to_uppercase(),
                f.rel_path,
                tag
            );
        }

        // Non-fatal warnings.
        for e in &r.errors {
            let _ = writeln!(sb, "  WARNING  {e}");
        }
    }

    // Summary line.
    let label = if r.direction.is_empty() {
        "processed".to_string()
    } else {
        r.direction.clone()
    };
    let _ = write!(
        sb,
        "{} complete: {} file(s) {}d, {} skipped",
        capitalize(&label),
        r.copied,
        label,
        r.skipped
    );

    // Config count in summary when present.
    let config_count = r
        .files
        .iter()
        .filter(|f| f.source == "config" && !f.skipped)
        .count();
    if config_count > 0 {
        let _ = write!(sb, " ({config_count} config)");
    }

    if !r.worktree_name.is_empty() {
        let _ = write!(sb, "  [worktree: {}]", r.worktree_name);
    }
    sb.push('\n');

    sb
}

/// Serialisable file entry. Field order and `omitempty` semantics mirror Go's
/// `jsonFileEntry`; `absPath` is intentionally absent.
#[derive(Serialize)]
struct JsonFileEntry {
    #[serde(rename = "relPath")]
    rel_path: String,
    #[serde(rename = "size", skip_serializing_if = "is_zero_i64")]
    size: i64,
    #[serde(rename = "skipped", skip_serializing_if = "is_false")]
    skipped: bool,
    #[serde(rename = "reason", skip_serializing_if = "String::is_empty")]
    reason: String,
    #[serde(rename = "source", skip_serializing_if = "String::is_empty")]
    source: String,
}

/// Serialisable result. Mirrors Go `jsonResult`.
#[derive(Serialize)]
struct JsonResult {
    direction: String,
    dir: String,
    files: Vec<JsonFileEntry>,
    copied: i64,
    skipped: i64,
    #[serde(rename = "errors", skip_serializing_if = "Vec::is_empty")]
    errors: Vec<String>,
    #[serde(rename = "worktreeName", skip_serializing_if = "String::is_empty")]
    worktree_name: String,
    #[serde(rename = "cancelled", skip_serializing_if = "is_false")]
    cancelled: bool,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero_i64(v: &i64) -> bool {
    *v == 0
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(v: &bool) -> bool {
    !*v
}

/// Serialises the result to a JSON string. Mirrors Go `FormatJSON`.
pub fn format_json(r: &EnvResult) -> Result<String, Error> {
    let files: Vec<JsonFileEntry> = r
        .files
        .iter()
        .map(|f| JsonFileEntry {
            rel_path: f.rel_path.clone(),
            size: f.size,
            skipped: f.skipped,
            reason: f.reason.clone(),
            source: f.source.clone(),
        })
        .collect();

    let out = JsonResult {
        direction: r.direction.clone(),
        dir: r.dir.clone(),
        files,
        copied: r.copied,
        skipped: r.skipped,
        errors: r.errors.clone(),
        worktree_name: r.worktree_name.clone(),
        cancelled: r.cancelled,
    };

    let body = serde_json::to_string_pretty(&out)?;
    Ok(gojson::html_escape(&body))
}

/// Renders the result as a Markdown table. Mirrors Go `FormatMarkdown`.
pub fn format_markdown(r: &EnvResult) -> String {
    let mut sb = String::new();

    let action = capitalize(&r.direction);
    let _ = write!(sb, "## {action} Report\n\n");
    let _ = write!(sb, "**Directory**: `{}`\n\n", r.dir);
    let _ = write!(
        sb,
        "**Copied**: {} | **Skipped**: {}\n\n",
        r.copied, r.skipped
    );

    if !r.worktree_name.is_empty() {
        let _ = write!(sb, "**Worktree**: `{}`\n\n", r.worktree_name);
    }

    // Handle cancelled result.
    if r.cancelled {
        let label = if r.direction.is_empty() {
            "operation"
        } else {
            &r.direction
        };
        let _ = writeln!(sb, "_{} cancelled._", capitalize(label));
        return sb;
    }

    if r.files.is_empty() {
        let _ = writeln!(sb, "_No .env files found._");
        return sb;
    }

    // Detect if any config files are present for an extra source column.
    let has_config = r.files.iter().any(|f| f.source == "config");

    if has_config {
        let _ = writeln!(sb, "| File | Size (bytes) | Source | Status | Reason |");
        let _ = writeln!(sb, "|------|-------------|--------|--------|--------|");
    } else {
        let _ = writeln!(sb, "| File | Size (bytes) | Status | Reason |");
        let _ = writeln!(sb, "|------|-------------|--------|--------|");
    }

    for f in &r.files {
        let (status, reason) = if f.skipped {
            ("skipped", f.reason.as_str())
        } else {
            ("copied", "")
        };
        // Normalise path separators for display (Go filepath.ToSlash). On unix
        // the separator is already "/", so this is a no-op, but we mirror it.
        let display_path = to_slash(&f.rel_path);
        if has_config {
            let source = if f.source.is_empty() {
                "env"
            } else {
                &f.source
            };
            let _ = writeln!(
                sb,
                "| `{}` | {} | {} | {} | {} |",
                display_path, f.size, source, status, reason
            );
        } else {
            let _ = writeln!(
                sb,
                "| `{}` | {} | {} | {} |",
                display_path, f.size, status, reason
            );
        }
    }

    if !r.errors.is_empty() {
        let _ = writeln!(sb, "\n### Warnings");
        for e in &r.errors {
            let _ = writeln!(sb, "- {e}");
        }
    }

    sb
}

/// Mirrors Go `filepath.ToSlash` on unix (already `/`-separated).
fn to_slash(p: &str) -> String {
    p.replace('\\', "/")
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::internal::envbackup::types::FileEntry;

    fn sample(direction: &str) -> EnvResult {
        EnvResult {
            direction: direction.to_string(),
            dir: "/bk".to_string(),
            files: vec![
                FileEntry::new(".env".to_string(), "/r/.env".to_string(), 5),
                FileEntry {
                    rel_path: "big".to_string(),
                    abs_path: "/r/big".to_string(),
                    size: 9,
                    skipped: true,
                    reason: "exceeds 1 MB".to_string(),
                    source: String::new(),
                },
            ],
            copied: 1,
            skipped: 1,
            errors: vec![],
            worktree_name: String::new(),
            cancelled: false,
        }
    }

    #[test]
    fn capitalize_first_byte() {
        assert_eq!(capitalize("backup"), "Backup");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("x"), "X");
    }

    #[test]
    fn text_lists_copied_and_summary() {
        let out = format_text(&sample("backup"), false, false);
        assert!(out.contains("  BACKUP  .env"));
        assert!(out.ends_with("Backup complete: 1 file(s) backupd, 1 skipped\n"));
        assert!(!out.contains("SKIPPED"));
    }

    #[test]
    fn text_verbose_shows_skipped() {
        let out = format_text(&sample("backup"), true, false);
        assert!(out.contains("  SKIPPED  big  (exceeds 1 MB)"));
    }

    #[test]
    fn text_quiet_summary_only() {
        let out = format_text(&sample("restore"), false, true);
        assert_eq!(out, "Restore complete: 1 file(s) restored, 1 skipped\n");
    }

    #[test]
    fn text_cancelled() {
        let mut r = sample("backup");
        r.cancelled = true;
        assert_eq!(format_text(&r, false, false), "Backup cancelled.\n");
    }

    #[test]
    fn text_config_count_and_worktree() {
        let mut r = sample("backup");
        r.files.push(FileEntry {
            rel_path: ".claude/x.json".to_string(),
            abs_path: "/r/.claude/x.json".to_string(),
            size: 2,
            skipped: false,
            reason: String::new(),
            source: "config".to_string(),
        });
        r.copied = 2;
        r.worktree_name = "wt".to_string();
        let out = format_text(&r, false, false);
        assert!(out.contains("  BACKUP  .claude/x.json [config]"));
        assert!(out.contains("(1 config)"));
        assert!(out.contains("[worktree: wt]"));
    }

    #[test]
    fn json_round_trips_and_omits_empty() {
        let out = format_json(&sample("backup")).unwrap();
        assert!(out.contains("\"direction\": \"backup\""));
        assert!(out.contains("\"copied\": 1"));
        // absPath is never serialised.
        assert!(!out.contains("absPath"));
        // errors omitted when empty.
        assert!(!out.contains("\"errors\""));
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["files"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn markdown_with_and_without_config_columns() {
        let plain = format_markdown(&sample("backup"));
        assert!(plain.contains("## Backup Report"));
        assert!(plain.contains("| File | Size (bytes) | Status | Reason |"));

        let mut r = sample("backup");
        r.files.push(FileEntry {
            rel_path: "cfg".to_string(),
            abs_path: "/r/cfg".to_string(),
            size: 1,
            skipped: false,
            reason: String::new(),
            source: "config".to_string(),
        });
        let withcfg = format_markdown(&r);
        assert!(withcfg.contains("| File | Size (bytes) | Source | Status | Reason |"));
        assert!(withcfg.contains("| `cfg` | 1 | config | copied |  |"));
    }

    #[test]
    fn markdown_empty_and_cancelled() {
        let mut empty = sample("backup");
        empty.files.clear();
        assert!(format_markdown(&empty).contains("_No .env files found._"));

        let mut cancelled = sample("backup");
        cancelled.cancelled = true;
        assert!(format_markdown(&cancelled).contains("_Backup cancelled._"));
    }

    #[test]
    fn markdown_warnings_section() {
        let mut r = sample("backup");
        r.errors.push("copy x: boom".to_string());
        let out = format_markdown(&r);
        assert!(out.contains("### Warnings"));
        assert!(out.contains("- copy x: boom"));
    }

    #[test]
    fn to_slash_is_noop_on_unix_paths() {
        assert_eq!(to_slash("a/b/c"), "a/b/c");
    }
}
