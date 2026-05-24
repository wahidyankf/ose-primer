//! Mermaid validation result formatting. Mirrors Go `reporter.go`.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use anyhow::Error;
use serde::Serialize;

use super::types::{ValidationResult, Violation, ViolationKind, Warning, WarningKind};
use crate::internal::cliout::gojson;

/// Go-style `%q` quoting of a string (matches `strconv.Quote` for the printable
/// inputs the mermaid corpus produces). Rust's `{:?}` on `&str` produces the
/// same escaping for ASCII/printable content.
fn go_quote(s: &str) -> String {
    format!("{s:?}")
}

/// Formats the validation result as human-readable text. Mirrors Go `FormatText`.
/// With quiet=true and no findings, returns empty string.
pub fn format_text(result: &ValidationResult, verbose: bool, quiet: bool) -> String {
    let has_findings = !result.violations.is_empty() || !result.warnings.is_empty();

    if quiet && !has_findings {
        return String::new();
    }

    let mut sb = String::new();

    if verbose || has_findings {
        // Group findings by file.
        let mut file_violations: BTreeMap<String, Vec<&Violation>> = BTreeMap::new();
        let mut file_warnings: BTreeMap<String, Vec<&Warning>> = BTreeMap::new();
        for v in &result.violations {
            file_violations
                .entry(v.file_path.clone())
                .or_default()
                .push(v);
        }
        for w in &result.warnings {
            file_warnings
                .entry(w.file_path.clone())
                .or_default()
                .push(w);
        }

        // Unique file paths from both maps. BTreeSet iteration is sorted, giving
        // deterministic output (Go ranges a map; for single-file findings the
        // result is identical, and the corpus produces zero findings).
        let mut file_set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for fp in file_violations.keys() {
            file_set.insert(fp.clone());
        }
        for fp in file_warnings.keys() {
            file_set.insert(fp.clone());
        }

        for fp in &file_set {
            let vs = file_violations.get(fp);
            let ws = file_warnings.get(fp);
            let has_v = vs.is_some_and(|v| !v.is_empty());
            let has_w = ws.is_some_and(|w| !w.is_empty());
            if has_v {
                let _ = writeln!(sb, "✗ {fp}");
            } else if has_w {
                let _ = writeln!(sb, "⚠ {fp}");
            } else {
                let _ = writeln!(sb, "✓ {fp}");
            }
            if let Some(vs) = vs {
                for v in vs {
                    let _ = writeln!(
                        sb,
                        "  block {} (line {}): {}",
                        v.block_index,
                        v.start_line,
                        violation_detail(v)
                    );
                }
            }
            if let Some(ws) = ws {
                for w in ws {
                    let _ = writeln!(
                        sb,
                        "  block {} (line {}): {}",
                        w.block_index,
                        w.start_line,
                        warning_detail(w)
                    );
                }
            }
        }
    }

    // Summary footer.
    let _ = writeln!(
        sb,
        "Found {} violation(s) and {} warning(s) in {} file(s) ({} block(s) scanned).",
        result.violations.len(),
        result.warnings.len(),
        result.files_scanned,
        result.blocks_scanned,
    );

    sb
}

fn violation_detail(v: &Violation) -> String {
    match v.kind {
        ViolationKind::LabelTooLong => format!(
            "[{}] node {} label {} is {} chars (max {})",
            v.kind.as_str(),
            go_quote(&v.node_id),
            go_quote(&v.label_text),
            v.label_len,
            v.max_label_len
        ),
        ViolationKind::WidthExceeded => format!(
            "[{}] span {} exceeds max-width {}",
            v.kind.as_str(),
            v.actual_width,
            v.max_width
        ),
        ViolationKind::MultipleDiagrams => format!(
            "[{}] block contains multiple flowchart/graph headers",
            v.kind.as_str()
        ),
    }
}

fn warning_detail(w: &Warning) -> String {
    match w.kind {
        WarningKind::SubgraphDense => {
            let label = if w.subgraph_label.is_empty() {
                "(unnamed)".to_string()
            } else {
                w.subgraph_label.clone()
            };
            format!(
                "[{}] subgraph {} has {} children; recommend ≤ {} for mobile rendering",
                w.kind.as_str(),
                go_quote(&label),
                w.subgraph_node_count,
                w.max_subgraph_nodes
            )
        }
        WarningKind::ComplexDiagram => format!(
            "[{}] span {} (max {}) and depth {} (max {}) both exceeded",
            w.kind.as_str(),
            w.actual_width,
            w.max_width,
            w.actual_depth,
            w.max_depth
        ),
    }
}

/// JSON violation shape with camelCase names and Go `omitempty` semantics.
/// Mirrors Go `jsonViolation`.
#[derive(Serialize)]
struct JsonViolation<'a> {
    kind: &'a str,
    #[serde(rename = "filePath")]
    file_path: &'a str,
    #[serde(rename = "blockIndex")]
    block_index: usize,
    #[serde(rename = "startLine")]
    start_line: usize,
    #[serde(rename = "nodeId", skip_serializing_if = "str::is_empty")]
    node_id: &'a str,
    #[serde(rename = "labelText", skip_serializing_if = "str::is_empty")]
    label_text: &'a str,
    #[serde(rename = "labelLen", skip_serializing_if = "is_zero_usize")]
    label_len: usize,
    #[serde(rename = "maxLabelLen", skip_serializing_if = "is_zero_i64")]
    max_label_len: i64,
    #[serde(rename = "actualWidth", skip_serializing_if = "is_zero_i64")]
    actual_width: i64,
    #[serde(rename = "maxWidth", skip_serializing_if = "is_zero_i64")]
    max_width: i64,
}

/// JSON warning shape with camelCase names and Go `omitempty` semantics.
/// Mirrors Go `jsonWarning`.
#[derive(Serialize)]
struct JsonWarning<'a> {
    kind: &'a str,
    #[serde(rename = "filePath")]
    file_path: &'a str,
    #[serde(rename = "blockIndex")]
    block_index: usize,
    #[serde(rename = "startLine")]
    start_line: usize,
    #[serde(rename = "actualWidth", skip_serializing_if = "is_zero_i64")]
    actual_width: i64,
    #[serde(rename = "actualDepth", skip_serializing_if = "is_zero_i64")]
    actual_depth: i64,
    #[serde(rename = "maxWidth", skip_serializing_if = "is_zero_i64")]
    max_width: i64,
    #[serde(rename = "maxDepth", skip_serializing_if = "is_zero_i64")]
    max_depth: i64,
    #[serde(rename = "subgraphLabel", skip_serializing_if = "str::is_empty")]
    subgraph_label: &'a str,
    #[serde(rename = "subgraphNodeCount", skip_serializing_if = "is_zero_usize")]
    subgraph_node_count: usize,
    #[serde(rename = "maxSubgraphNodes", skip_serializing_if = "is_zero_i64")]
    max_subgraph_nodes: i64,
}

#[derive(Serialize)]
struct JsonResult<'a> {
    #[serde(rename = "filesScanned")]
    files_scanned: usize,
    #[serde(rename = "blocksScanned")]
    blocks_scanned: usize,
    violations: Vec<JsonViolation<'a>>,
    warnings: Vec<JsonWarning<'a>>,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero_i64(v: &i64) -> bool {
    *v == 0
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero_usize(v: &usize) -> bool {
    *v == 0
}

/// Formats the validation result as JSON. Mirrors Go `FormatJSON`.
pub fn format_json(result: &ValidationResult) -> Result<String, Error> {
    let violations: Vec<JsonViolation> = result
        .violations
        .iter()
        .map(|v| JsonViolation {
            kind: v.kind.as_str(),
            file_path: &v.file_path,
            block_index: v.block_index,
            start_line: v.start_line,
            node_id: &v.node_id,
            label_text: &v.label_text,
            label_len: v.label_len,
            max_label_len: v.max_label_len,
            actual_width: v.actual_width,
            max_width: v.max_width,
        })
        .collect();

    let warnings: Vec<JsonWarning> = result
        .warnings
        .iter()
        .map(|w| JsonWarning {
            kind: w.kind.as_str(),
            file_path: &w.file_path,
            block_index: w.block_index,
            start_line: w.start_line,
            actual_width: w.actual_width,
            actual_depth: w.actual_depth,
            max_width: w.max_width,
            max_depth: w.max_depth,
            subgraph_label: &w.subgraph_label,
            subgraph_node_count: w.subgraph_node_count,
            max_subgraph_nodes: w.max_subgraph_nodes,
        })
        .collect();

    let out = JsonResult {
        files_scanned: result.files_scanned,
        blocks_scanned: result.blocks_scanned,
        violations,
        warnings,
    };

    // Go's encoding/json HTML-escapes <, >, & in string values; mirror that.
    Ok(gojson::html_escape(&serde_json::to_string_pretty(&out)?))
}

/// Formats the validation result as a markdown table. Mirrors Go `FormatMarkdown`.
pub fn format_markdown(result: &ValidationResult) -> String {
    if result.violations.is_empty() && result.warnings.is_empty() {
        return format!(
            "All {} block(s) in {} file(s) passed mermaid validation.\n",
            result.blocks_scanned, result.files_scanned
        );
    }

    let mut sb = String::new();
    sb.push_str("| File | Block | Line | Severity | Kind | Detail |\n");
    sb.push_str("|------|-------|------|----------|------|--------|\n");

    for v in &result.violations {
        let _ = writeln!(
            sb,
            "| {} | {} | {} | error | {} | {} |",
            v.file_path,
            v.block_index,
            v.start_line,
            v.kind.as_str(),
            violation_detail(v)
        );
    }
    for w in &result.warnings {
        let _ = writeln!(
            sb,
            "| {} | {} | {} | warning | {} | {} |",
            w.file_path,
            w.block_index,
            w.start_line,
            w.kind.as_str(),
            warning_detail(w)
        );
    }

    sb
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn label_violation() -> Violation {
        Violation {
            kind: ViolationKind::LabelTooLong,
            file_path: "f.md".into(),
            block_index: 0,
            start_line: 2,
            node_id: "A".into(),
            label_text: "long label text".into(),
            label_len: 53,
            max_label_len: 30,
            actual_width: 0,
            max_width: 0,
        }
    }

    fn clean_result() -> ValidationResult {
        ValidationResult {
            files_scanned: 3,
            blocks_scanned: 10,
            violations: Vec::new(),
            warnings: Vec::new(),
        }
    }

    #[test]
    fn text_clean_summary_only() {
        let r = clean_result();
        assert_eq!(
            format_text(&r, false, false),
            "Found 0 violation(s) and 0 warning(s) in 3 file(s) (10 block(s) scanned).\n"
        );
    }

    #[test]
    fn text_quiet_clean_empty() {
        let r = clean_result();
        assert!(format_text(&r, false, true).is_empty());
    }

    #[test]
    fn text_label_violation_detail() {
        let mut r = clean_result();
        r.violations.push(label_violation());
        let s = format_text(&r, false, false);
        assert!(s.contains("✗ f.md\n"));
        assert!(s.contains("block 0 (line 2): [label_too_long] node \"A\" label \"long label text\" is 53 chars (max 30)"));
        assert!(s.ends_with(
            "Found 1 violation(s) and 0 warning(s) in 3 file(s) (10 block(s) scanned).\n"
        ));
    }

    #[test]
    fn json_omits_zero_and_empty_fields() {
        let mut r = clean_result();
        r.violations.push(label_violation());
        let s = format_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        let viol = &v["violations"][0];
        assert_eq!(viol["kind"], "label_too_long");
        assert_eq!(viol["nodeId"], "A");
        assert_eq!(viol["labelLen"], 53);
        // actualWidth/maxWidth are zero → omitted.
        assert!(viol.get("actualWidth").is_none());
        assert!(viol.get("maxWidth").is_none());
    }

    #[test]
    fn json_clean_has_empty_arrays() {
        let r = clean_result();
        let s = format_json(&r).unwrap();
        assert!(s.contains("\"violations\": []"));
        assert!(s.contains("\"warnings\": []"));
    }

    #[test]
    fn markdown_clean_message() {
        let r = clean_result();
        assert_eq!(
            format_markdown(&r),
            "All 10 block(s) in 3 file(s) passed mermaid validation.\n"
        );
    }

    #[test]
    fn markdown_table_for_violation() {
        let mut r = clean_result();
        r.violations.push(label_violation());
        let s = format_markdown(&r);
        assert!(s.contains("| File | Block | Line | Severity | Kind | Detail |"));
        assert!(s.contains("| f.md | 0 | 2 | error | label_too_long |"));
    }
}
