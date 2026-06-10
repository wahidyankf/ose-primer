//! Text and JSON reporters for env-validate results.

use std::fmt::Write as _;

use anyhow::{Error, Result};

use super::types::ValidateResult;

/// Format text output for env validate.
pub fn format_text(result: &ValidateResult) -> String {
    if result.is_ok() {
        return "✓ Env validate passed! All app surfaces clean.\n".to_string();
    }
    let mut out = String::new();
    for surface in &result.surfaces {
        if surface.is_ok() {
            continue;
        }
        let _ = writeln!(out, "✗ {}", surface.app);
        for key in &surface.declared_not_read {
            let _ = writeln!(out, "  declared-but-unread: {key}");
        }
        for key in &surface.read_not_declared {
            let _ = writeln!(out, "  read-but-undeclared: {key}");
        }
    }
    let v = result.violation_count();
    let _ = write!(
        out,
        "\nEnv validate FAILED: {v} violation{}\n",
        if v == 1 { "" } else { "s" }
    );
    out
}

/// Format JSON output for env validate.
pub fn format_json(result: &ValidateResult) -> Result<String, Error> {
    let mut surfaces_json = Vec::new();
    for s in &result.surfaces {
        let dnr: Vec<&str> = s.declared_not_read.iter().map(String::as_str).collect();
        let rnd: Vec<&str> = s.read_not_declared.iter().map(String::as_str).collect();
        let dnr_json: String = dnr
            .iter()
            .map(|k| format!("\"{}\"", escape_json(k)))
            .collect::<Vec<_>>()
            .join(",");
        let rnd_json: String = rnd
            .iter()
            .map(|k| format!("\"{}\"", escape_json(k)))
            .collect::<Vec<_>>()
            .join(",");
        surfaces_json.push(format!(
            "{{\"app\":\"{}\",\"declared_not_read\":[{}],\"read_not_declared\":[{}]}}",
            escape_json(&s.app),
            dnr_json,
            rnd_json,
        ));
    }
    let ok = result.is_ok();
    Ok(format!(
        "{{\"ok\":{ok},\"violations\":{},\"surfaces\":[{}]}}",
        result.violation_count(),
        surfaces_json.join(","),
    ))
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::internal::envvalidate::types::SurfaceResult;

    fn clean_result() -> ValidateResult {
        ValidateResult {
            surfaces: vec![SurfaceResult {
                app: "fixture-app".to_string(),
                declared_not_read: BTreeSet::new(),
                read_not_declared: BTreeSet::new(),
            }],
        }
    }

    fn failed_result() -> ValidateResult {
        let mut dnr = BTreeSet::new();
        dnr.insert("FIXTURE_JWT_SECRET".to_string());
        ValidateResult {
            surfaces: vec![SurfaceResult {
                app: "fixture-app".to_string(),
                declared_not_read: dnr,
                read_not_declared: BTreeSet::new(),
            }],
        }
    }

    #[test]
    fn text_clean_reports_passed() {
        let out = format_text(&clean_result());
        assert!(out.contains("passed"), "got: {out}");
    }

    #[test]
    fn text_failure_names_key() {
        let out = format_text(&failed_result());
        assert!(out.contains("FIXTURE_JWT_SECRET"), "got: {out}");
        assert!(out.contains("declared-but-unread"), "got: {out}");
    }

    #[test]
    fn json_clean_ok_true() {
        let out = format_json(&clean_result()).unwrap();
        assert!(out.contains("\"ok\":true"), "got: {out}");
    }

    #[test]
    fn json_failure_names_key() {
        let out = format_json(&failed_result()).unwrap();
        assert!(out.contains("FIXTURE_JWT_SECRET"), "got: {out}");
        assert!(out.contains("\"ok\":false"), "got: {out}");
    }
}
