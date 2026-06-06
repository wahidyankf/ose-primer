//! Link validation result formatting. Mirrors Go `links_reporter.go`.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use anyhow::Error;
use serde::Serialize;

use super::types::{BrokenLink, LinkValidationResult};

/// Category display order for the text/markdown report. Mirrors Go `categoryOrder`.
const CATEGORY_ORDER: &[&str] = &[
    "Legacy prefixed paths",
    "Missing files",
    "General/other paths",
    "workflows/ paths",
    "vision/ paths",
    "conventions README",
    "broken-anchor",
];

/// Formats the validation result as human-readable text. Mirrors Go `FormatLinkText`.
pub fn format_link_text(result: &LinkValidationResult, _verbose: bool, quiet: bool) -> String {
    let mut out = String::new();

    if result.broken_links.is_empty() {
        if !quiet {
            out.push_str("✓ All links valid! No broken links found.\n");
        }
        return out;
    }

    out.push_str("# Broken Links Report\n\n");
    let _ = writeln!(out, "**Total broken links**: {}", result.broken_links.len());

    for category in CATEGORY_ORDER {
        let links = match result.broken_by_category.get(*category) {
            Some(l) if !l.is_empty() => l,
            _ => continue,
        };

        let _ = write!(out, "\n## {} ({} links)\n", category, links.len());

        // Group by file.
        let mut by_file: BTreeMap<String, Vec<&BrokenLink>> = BTreeMap::new();
        for link in links {
            by_file
                .entry(link.source_file.clone())
                .or_default()
                .push(link);
        }

        // BTreeMap iterates files in sorted (alphabetical) order, matching Go's sort.Strings.
        for (file, file_links) in &by_file {
            let _ = write!(out, "\n### {file}\n\n");

            // Sort links by line number (stable). Go uses sort.Slice on line number.
            let mut sorted: Vec<&&BrokenLink> = file_links.iter().collect();
            sorted.sort_by_key(|l| l.line_number);

            for link in sorted {
                let _ = writeln!(out, "- Line {}: `{}`", link.line_number, link.link_text);
            }
        }
    }

    out
}

/// JSON output shape. Mirrors Go `LinkJSONOutput`.
#[derive(Serialize)]
struct LinkJsonOutput {
    status: &'static str,
    timestamp: String,
    total_files: usize,
    total_links: usize,
    broken_count: usize,
    duration_ms: i64,
    categories: BTreeMap<String, Vec<JsonBrokenLink>>,
}

/// JSON broken-link shape. Mirrors Go `JSONBrokenLink`.
#[derive(Serialize)]
struct JsonBrokenLink {
    source_file: String,
    line_number: usize,
    link_text: String,
    target_path: String,
}

/// Formats the validation result as JSON. Mirrors Go `FormatLinkJSON`.
pub fn format_link_json(result: &LinkValidationResult) -> Result<String, Error> {
    let status = if result.broken_links.is_empty() {
        "success"
    } else {
        "failure"
    };

    // BTreeMap keys are serialized in sorted order, matching Go's map-key sorting.
    let mut categories: BTreeMap<String, Vec<JsonBrokenLink>> = BTreeMap::new();
    for (category, links) in &result.broken_by_category {
        let json_links = links
            .iter()
            .map(|l| JsonBrokenLink {
                source_file: l.source_file.clone(),
                line_number: l.line_number,
                link_text: l.link_text.clone(),
                target_path: l.target_path.clone(),
            })
            .collect();
        categories.insert(category.clone(), json_links);
    }

    let timestamp = chrono::Local::now()
        .format("%Y-%m-%dT%H:%M:%S%:z")
        .to_string();

    let out = LinkJsonOutput {
        status,
        timestamp,
        total_files: result.total_files,
        total_links: result.total_links,
        broken_count: result.broken_links.len(),
        duration_ms: i64::try_from(result.scan_duration.as_millis()).unwrap_or(i64::MAX),
        categories,
    };

    // Go's encoding/json HTML-escapes <, >, & in string values; mirror that.
    Ok(crate::internal::cliout::gojson::html_escape(
        &serde_json::to_string_pretty(&out)?,
    ))
}

/// Markdown delegates to text — the text format is already markdown-compatible.
/// Mirrors Go `FormatLinkMarkdown`.
pub fn format_link_markdown(result: &LinkValidationResult) -> String {
    format_link_text(result, false, false)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn result_with(broken: Vec<BrokenLink>) -> LinkValidationResult {
        let mut by_cat: HashMap<String, Vec<BrokenLink>> = HashMap::new();
        for b in &broken {
            by_cat
                .entry(b.category.clone())
                .or_default()
                .push(b.clone());
        }
        LinkValidationResult {
            total_files: 2,
            total_links: broken.len(),
            broken_links: broken,
            broken_by_category: by_cat,
            scan_duration: std::time::Duration::from_millis(5),
        }
    }

    #[test]
    fn text_no_broken_shows_success() {
        let r = result_with(vec![]);
        assert_eq!(
            format_link_text(&r, false, false),
            "✓ All links valid! No broken links found.\n"
        );
    }

    #[test]
    fn text_no_broken_quiet_is_empty() {
        let r = result_with(vec![]);
        assert!(format_link_text(&r, false, true).is_empty());
    }

    #[test]
    fn text_broken_report_structure() {
        let r = result_with(vec![
            BrokenLink {
                line_number: 2,
                source_file: "docs/a.md".into(),
                link_text: "./x.md".into(),
                target_path: "/repo/docs/x.md".into(),
                category: "General/other paths".into(),
            },
            BrokenLink {
                line_number: 5,
                source_file: "docs/a.md".into(),
                link_text: "workflows/y.md".into(),
                target_path: "/repo/docs/workflows/y.md".into(),
                category: "workflows/ paths".into(),
            },
        ]);
        let s = format_link_text(&r, false, false);
        assert!(s.starts_with("# Broken Links Report\n\n"));
        assert!(s.contains("**Total broken links**: 2\n"));
        assert!(s.contains("## General/other paths (1 links)\n"));
        assert!(s.contains("### docs/a.md\n\n"));
        assert!(s.contains("- Line 2: `./x.md`\n"));
        assert!(s.contains("## workflows/ paths (1 links)\n"));
        assert!(s.contains("- Line 5: `workflows/y.md`\n"));
        // General/other comes before workflows in category order.
        let gen_idx = s.find("General/other paths").unwrap();
        let wf_idx = s.find("workflows/ paths").unwrap();
        assert!(gen_idx < wf_idx);
    }

    #[test]
    fn json_success_empty_categories() {
        let r = result_with(vec![]);
        let s = format_link_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "success");
        assert_eq!(v["broken_count"], 0);
        assert!(v["categories"].as_object().unwrap().is_empty());
    }

    #[test]
    fn json_failure_with_categories() {
        let r = result_with(vec![BrokenLink {
            line_number: 2,
            source_file: "docs/a.md".into(),
            link_text: "./x.md".into(),
            target_path: "/repo/docs/x.md".into(),
            category: "General/other paths".into(),
        }]);
        let s = format_link_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failure");
        assert_eq!(v["broken_count"], 1);
        assert_eq!(
            v["categories"]["General/other paths"][0]["link_text"],
            "./x.md"
        );
    }

    #[test]
    fn markdown_delegates_to_text() {
        let r = result_with(vec![]);
        assert_eq!(format_link_markdown(&r), format_link_text(&r, false, false));
    }
}
