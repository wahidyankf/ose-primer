//! Shared naming-violation reporter used by `agents validate-naming` and
//! `workflows validate-naming`.
//!
//! JSON is HTML-escaped to match Go's `json.MarshalIndent` default; markdown wraps the path
//! column in backticks (the primer's Go reference uses `` `%s` `` for the path cell).

// Re-export the canonical names that ose-public command files expect.
pub use crate::application::naming::reporter::{format_json, format_markdown, format_text};

use std::fmt::Write as _;

use anyhow::Error;
use serde::Serialize;

use super::Violation;

/// Renders a human-readable summary of naming violations.
pub fn format_naming_text(
    label: &str,
    violations: &[Violation],
    verbose: bool,
    quiet: bool,
) -> String {
    let mut b = String::new();
    if violations.is_empty() {
        if !quiet {
            let _ = writeln!(
                b,
                "{label} naming validation: VALIDATION PASSED (0 violations)"
            );
        }
        return b;
    }
    let _ = writeln!(
        b,
        "{label} naming validation: {} violation(s)",
        violations.len()
    );
    for v in violations {
        let _ = writeln!(b, "  [{}] {} — {}", v.kind, v.path, v.message);
    }
    if verbose {
        b.push_str("\nSee repo-governance/conventions/structure/agent-naming.md (or workflow-naming.md) for the normative rule.\n");
    }
    b
}

/// JSON report for naming violations. Emits a
/// trailing newline (Go appends `"\n"`).
pub fn format_naming_json(kind: &str, violations: &[Violation]) -> Result<String, Error> {
    #[derive(Serialize)]
    struct Out<'a> {
        kind: &'a str,
        violations: &'a [Violation],
        count: usize,
    }
    let out = Out {
        kind,
        violations,
        count: violations.len(),
    };
    let json = crate::internal::cliout::gojson::html_escape(&serde_json::to_string_pretty(&out)?);
    Ok(format!("{json}\n"))
}

/// PR-friendly markdown table for naming violations.
pub fn format_naming_markdown(label: &str, violations: &[Violation]) -> String {
    let mut b = String::new();
    let _ = write!(b, "## {label} naming validation\n\n");
    if violations.is_empty() {
        b.push_str("All files conform to the naming convention.\n");
        return b;
    }
    b.push_str("| Kind | Path | Message |\n");
    b.push_str("| --- | --- | --- |\n");
    for v in violations {
        let _ = writeln!(b, "| {} | `{}` | {} |", v.kind, v.path, v.message);
    }
    b
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn sample() -> Violation {
        Violation {
            path: ".claude/agents/foo.md".to_string(),
            kind: "role-suffix".to_string(),
            message: "bad".to_string(),
        }
    }

    #[test]
    fn text_passed_non_quiet() {
        let s = format_naming_text("Agents", &[], false, false);
        assert!(s.contains("VALIDATION PASSED"));
    }

    #[test]
    fn text_passed_quiet_empty() {
        let s = format_naming_text("Agents", &[], false, true);
        assert!(s.is_empty());
    }

    #[test]
    fn text_failed_lists() {
        let s = format_naming_text("Workflows", std::slice::from_ref(&sample()), false, false);
        assert!(s.contains("Workflows naming validation: 1 violation"));
        assert!(s.contains("role-suffix"));
    }

    #[test]
    fn text_verbose_adds_note() {
        let s = format_naming_text("Workflows", std::slice::from_ref(&sample()), true, false);
        assert!(s.contains("normative rule"));
    }

    #[test]
    fn json_carries_kind_and_count() {
        let s = format_naming_json("workflows", std::slice::from_ref(&sample())).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["kind"], "workflows");
        assert_eq!(v["count"], 1);
        assert_eq!(v["violations"][0]["Path"], ".claude/agents/foo.md");
    }

    #[test]
    fn json_empty_violations_array() {
        let s = format_naming_json("workflows", &[]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["count"], 0);
        assert!(v["violations"].as_array().unwrap().is_empty());
    }

    #[test]
    fn markdown_clean_passes() {
        let s = format_naming_markdown("Workflows", &[]);
        assert!(s.contains("All files conform"));
    }

    #[test]
    fn markdown_violations_emit_backticked_path() {
        let s = format_naming_markdown("Agents", std::slice::from_ref(&sample()));
        assert!(s.contains("| Kind | Path | Message |"));
        assert!(s.contains("| role-suffix | `.claude/agents/foo.md` | bad |"));
    }
}
