//! `repo-governance vendor-audit` command.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/cmd/governance_vendor_audit.go`.
//! Output is written with `print!` (no implicit trailing newline) to mirror
//! Go's `Fprint`. The cobra-style usage block (printed to stderr on error by
//! the dispatcher) reproduces the Go binary's help text.

use std::fmt::Write as _;
use std::path::PathBuf;

use anyhow::{Context, Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::internal::cliout::OutputFormat;
use crate::internal::git;
use crate::internal::repo_governance::vendor_audit::{Finding, walk};

/// Cobra-style usage block printed to stderr when `vendor-audit` errors.
pub const VENDOR_AUDIT_USAGE: &str = "Usage:\n  \
rhino-cli repo-governance vendor-audit [path] [flags]\n\n\
Examples:\n  \
# Audit the default repo-governance/ directory\n  \
rhino-cli repo-governance vendor-audit\n\n  \
# Audit a specific path\n  \
rhino-cli repo-governance vendor-audit docs/\n\n  \
# Output as JSON\n  \
rhino-cli repo-governance vendor-audit -o json\n\n\
Flags:\n  \
-h, --help   help for vendor-audit\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

#[derive(Args, Debug)]
pub struct VendorAuditArgs {
    /// Optional scan path (defaults to `repo-governance/`).
    pub path: Option<String>,
}

#[derive(Serialize)]
struct JsonFinding<'a> {
    path: &'a str,
    line: usize,
    r#match: &'a str,
    replacement: &'a str,
}

#[derive(Serialize)]
struct JsonResult<'a> {
    status: &'a str,
    count: usize,
    findings: Vec<JsonFinding<'a>>,
}

pub fn run_vendor_audit(args: &VendorAuditArgs, output: OutputFormat) -> Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let scan_path = args.path.as_deref().unwrap_or("repo-governance");
    // Mirror Go's `filepath.Join(repoRoot, scanPath)`: an absolute scanPath is
    // treated as relative (joined under repo root), matching the Go binary's
    // path resolution exactly.
    let full_path = go_join(&repo_root, scan_path);

    let findings = walk(&full_path).context("vendor audit failed")?;

    let out = match output {
        OutputFormat::Text => format_vendor_audit_text(&findings),
        OutputFormat::Json => format_vendor_audit_json(&findings)?,
        OutputFormat::Markdown => format_vendor_audit_markdown(&findings),
    };
    print!("{out}");

    if !findings.is_empty() {
        return Err(anyhow!("{} violation(s) found", findings.len()));
    }
    Ok(())
}

/// Joins `base` and `elem` the way Go's `filepath.Join` does: all elements are
/// concatenated with a separator and lexically cleaned. Unlike Rust's
/// `PathBuf::join`, an absolute `elem` does NOT replace `base` — it is appended
/// under it (Go semantics), so `Join("/repo", "/tmp/x") == "/repo/tmp/x"`.
fn go_join(base: &std::path::Path, elem: &str) -> PathBuf {
    if elem.is_empty() {
        return clean(base);
    }
    let mut joined = base.to_string_lossy().into_owned();
    joined.push('/');
    joined.push_str(elem);
    clean(std::path::Path::new(&joined))
}

/// Lexical `filepath.Clean` equivalent: collapses `.`, resolves `..` against
/// preceding components, removes redundant separators, without touching disk.
fn clean(path: &std::path::Path) -> PathBuf {
    use std::ffi::OsString;
    use std::path::Component;

    let is_absolute = path.is_absolute();
    let mut stack: Vec<OsString> = Vec::new();

    for comp in path.components() {
        match comp {
            Component::Prefix(p) => stack.push(p.as_os_str().to_os_string()),
            Component::RootDir | Component::CurDir => {}
            Component::ParentDir => match stack.last() {
                Some(last) if last != ".." => {
                    stack.pop();
                }
                _ => {
                    if !is_absolute {
                        stack.push("..".into());
                    }
                }
            },
            Component::Normal(c) => stack.push(c.to_os_string()),
        }
    }

    let mut out = PathBuf::new();
    if is_absolute {
        out.push(std::path::MAIN_SEPARATOR_STR);
    }
    for c in stack {
        out.push(c);
    }
    if out.as_os_str().is_empty() {
        out.push(".");
    }
    out
}

/// Formats findings as human-readable text. Mirrors Go `formatVendorAuditText`.
fn format_vendor_audit_text(findings: &[Finding]) -> String {
    if findings.is_empty() {
        return "GOVERNANCE VENDOR AUDIT PASSED: no violations found\n".to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "GOVERNANCE VENDOR AUDIT FAILED: {} violation(s) found",
        findings.len()
    );
    for f in findings {
        let _ = writeln!(
            sb,
            "  {}:{}  {}  →  {}",
            f.path, f.line, f.r#match, f.replacement
        );
    }
    sb
}

/// Formats findings as JSON. Mirrors Go `formatVendorAuditJSON`
/// (`json.MarshalIndent` → HTML-escaped, with trailing newline).
fn format_vendor_audit_json(findings: &[Finding]) -> Result<String, Error> {
    let status = if findings.is_empty() {
        "passed"
    } else {
        "failed"
    };
    let jf: Vec<JsonFinding> = findings
        .iter()
        .map(|f| JsonFinding {
            path: &f.path,
            line: f.line,
            r#match: &f.r#match,
            replacement: &f.replacement,
        })
        .collect();
    let result = JsonResult {
        status,
        count: findings.len(),
        findings: jf,
    };
    let json =
        crate::internal::cliout::gojson::html_escape(&serde_json::to_string_pretty(&result)?);
    Ok(format!("{json}\n"))
}

/// Formats findings as a markdown table. Mirrors Go `formatVendorAuditMarkdown`.
fn format_vendor_audit_markdown(findings: &[Finding]) -> String {
    if findings.is_empty() {
        return "## Governance Vendor Audit\n\n**PASSED**: no violations found\n".to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "## Governance Vendor Audit\n\n**FAILED**: {} violation(s) found\n",
        findings.len()
    );
    sb.push_str("| File | Line | Term | Replacement |\n");
    sb.push_str("|------|------|------|-------------|\n");
    for f in findings {
        let _ = writeln!(
            sb,
            "| {} | {} | `{}` | {} |",
            f.path, f.line, f.r#match, f.replacement
        );
    }
    sb
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn sample() -> Finding {
        Finding {
            path: "x.md".to_string(),
            line: 3,
            r#match: "Claude Code".to_string(),
            replacement: "\"the coding agent\"".to_string(),
        }
    }

    #[test]
    fn text_passed() {
        assert_eq!(
            format_vendor_audit_text(&[]),
            "GOVERNANCE VENDOR AUDIT PASSED: no violations found\n"
        );
    }

    #[test]
    fn text_failed() {
        let s = format_vendor_audit_text(&[sample()]);
        assert!(s.starts_with("GOVERNANCE VENDOR AUDIT FAILED: 1 violation(s) found\n"));
        assert!(s.contains("  x.md:3  Claude Code  →  \"the coding agent\"\n"));
    }

    #[test]
    fn json_passed() {
        let s = format_vendor_audit_json(&[]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "passed");
        assert_eq!(v["count"], 0);
        assert!(v["findings"].as_array().unwrap().is_empty());
        assert!(s.ends_with('\n'));
    }

    #[test]
    fn json_failed() {
        let s = format_vendor_audit_json(&[sample()]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failed");
        assert_eq!(v["count"], 1);
        assert_eq!(v["findings"][0]["match"], "Claude Code");
        assert_eq!(v["findings"][0]["line"], 3);
    }

    #[test]
    fn markdown_passed() {
        assert_eq!(
            format_vendor_audit_markdown(&[]),
            "## Governance Vendor Audit\n\n**PASSED**: no violations found\n"
        );
    }

    #[test]
    fn markdown_failed() {
        let s = format_vendor_audit_markdown(&[sample()]);
        assert!(s.contains("**FAILED**: 1 violation(s) found"));
        assert!(s.contains("| x.md | 3 | `Claude Code` | \"the coding agent\" |"));
    }

    #[test]
    fn go_join_relative() {
        let p = go_join(std::path::Path::new("/repo"), "repo-governance");
        assert_eq!(p, std::path::Path::new("/repo/repo-governance"));
    }

    #[test]
    fn go_join_absolute_arg_appended_under_base() {
        // Go semantics: absolute elem is joined under base (NOT replacing it).
        let p = go_join(std::path::Path::new("/repo"), "/tmp/x");
        assert_eq!(p, std::path::Path::new("/repo/tmp/x"));
    }

    #[test]
    fn go_join_cleans_dotdot() {
        let p = go_join(std::path::Path::new("/repo"), "docs/../AGENTS.md");
        assert_eq!(p, std::path::Path::new("/repo/AGENTS.md"));
    }
}
