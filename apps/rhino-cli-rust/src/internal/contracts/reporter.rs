//! Output formatting for contract codegen post-processing results.
//!
//! JSON timestamps use the same RFC3339-with-offset shape as Go's `timeutil.Timestamp()`.

use std::fmt::Write as _;

use anyhow::Error;
use serde::Serialize;

use super::types::{DartScaffoldResult, JavaCleanImportsResult};

/// Returns the current timestamp in Go's `time.RFC3339` format with a numeric
/// offset, matching `timeutil.Timestamp()`.
fn timestamp() -> String {
    chrono::Local::now()
        .format("%Y-%m-%dT%H:%M:%S%:z")
        .to_string()
}

// ===========================================================================
// java-clean-imports
// ===========================================================================

/// Formats the Java import cleaning result as human-readable text. In quiet mode
/// with no modifications it returns an empty string; in verbose mode the
/// modified file paths are listed.
pub fn format_java_clean_imports_text(
    result: &JavaCleanImportsResult,
    verbose: bool,
    quiet: bool,
) -> String {
    let mut out = String::new();

    if result.modified_files == 0 {
        if quiet {
            return String::new();
        }
        out.push_str("No imports needed cleaning.\n");
        return out;
    }

    let _ = writeln!(
        out,
        "Cleaned imports in {} of {} Java files.",
        result.modified_files, result.total_files
    );

    if verbose {
        for f in &result.modified {
            let _ = writeln!(out, "  ✓ {f}");
        }
    }

    out
}

#[derive(Serialize)]
struct JavaCleanImportsJson<'a> {
    status: &'a str,
    timestamp: String,
    total_files: usize,
    modified_files: usize,
    modified: &'a [String],
}

/// Formats the Java import cleaning result as JSON.
pub fn format_java_clean_imports_json(result: &JavaCleanImportsResult) -> Result<String, Error> {
    let out = JavaCleanImportsJson {
        status: "success",
        timestamp: timestamp(),
        total_files: result.total_files,
        modified_files: result.modified_files,
        modified: &result.modified,
    };
    let json = crate::internal::cliout::gojson::html_escape(&serde_json::to_string_pretty(&out)?);
    Ok(json)
}

/// Formats the Java import cleaning result as markdown.
pub fn format_java_clean_imports_markdown(result: &JavaCleanImportsResult) -> String {
    let mut out = String::new();

    out.push_str("# Java Import Cleaning Report\n\n");
    let _ = writeln!(out, "- **Total files**: {}", result.total_files);
    let _ = writeln!(out, "- **Modified files**: {}", result.modified_files);

    if result.modified_files == 0 {
        out.push_str("\nNo files needed cleaning.\n");
        return out;
    }

    out.push_str("\n## Modified Files\n\n");
    for f in &result.modified {
        let _ = writeln!(out, "- `{f}`");
    }

    out
}

// ===========================================================================
// dart-scaffold
// ===========================================================================

/// Formats the Dart scaffold result as human-readable text. In quiet mode it
/// returns `"ok\n"`; in verbose mode the model files are listed.
pub fn format_dart_scaffold_text(
    result: &DartScaffoldResult,
    verbose: bool,
    quiet: bool,
) -> String {
    let mut out = String::new();

    if quiet {
        out.push_str("ok\n");
        return out;
    }

    let _ = writeln!(
        out,
        "Dart scaffold created: pubspec.yaml + barrel library ({} model files).",
        result.model_files.len()
    );

    if verbose {
        for f in &result.model_files {
            let _ = writeln!(out, "  ✓ {f}");
        }
    }

    out
}

#[derive(Serialize)]
struct DartScaffoldJson<'a> {
    status: &'a str,
    timestamp: String,
    pubspec_created: bool,
    barrel_created: bool,
    model_files: &'a [String],
}

/// Formats the Dart scaffold result as JSON.
pub fn format_dart_scaffold_json(result: &DartScaffoldResult) -> Result<String, Error> {
    let out = DartScaffoldJson {
        status: "success",
        timestamp: timestamp(),
        pubspec_created: result.pubspec_created,
        barrel_created: result.barrel_created,
        model_files: &result.model_files,
    };
    let json = crate::internal::cliout::gojson::html_escape(&serde_json::to_string_pretty(&out)?);
    Ok(json)
}

/// Formats the Dart scaffold result as markdown.
pub fn format_dart_scaffold_markdown(result: &DartScaffoldResult) -> String {
    let mut out = String::new();

    out.push_str("# Dart Contract Scaffold Report\n\n");

    let pubspec_status = if result.pubspec_created {
        "created"
    } else {
        "not created"
    };
    let barrel_status = if result.barrel_created {
        "created"
    } else {
        "not created"
    };

    let _ = writeln!(out, "- **pubspec.yaml**: {pubspec_status}");
    let _ = writeln!(out, "- **Barrel library**: {barrel_status}");
    let _ = writeln!(out, "- **Model files**: {}", result.model_files.len());

    if result.model_files.is_empty() {
        return out;
    }

    out.push_str("\n## Model Files\n\n");
    for f in &result.model_files {
        let _ = writeln!(out, "- `{f}`");
    }

    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn clean_result(modified: usize, total: usize, files: &[&str]) -> JavaCleanImportsResult {
        JavaCleanImportsResult {
            total_files: total,
            modified_files: modified,
            modified: files.iter().map(|s| (*s).to_string()).collect(),
        }
    }

    fn dart_result(model_files: &[&str]) -> DartScaffoldResult {
        DartScaffoldResult {
            pubspec_created: true,
            barrel_created: true,
            model_files: model_files.iter().map(|s| (*s).to_string()).collect(),
        }
    }

    #[test]
    fn java_text_no_modifications() {
        let r = clean_result(0, 5, &[]);
        assert_eq!(
            format_java_clean_imports_text(&r, false, false),
            "No imports needed cleaning.\n"
        );
        assert_eq!(format_java_clean_imports_text(&r, false, true), "");
    }

    #[test]
    fn java_text_with_modifications_verbose() {
        let r = clean_result(2, 5, &["A.java", "B.java"]);
        let s = format_java_clean_imports_text(&r, true, false);
        assert!(s.starts_with("Cleaned imports in 2 of 5 Java files.\n"));
        assert!(s.contains("  ✓ A.java\n"));
        assert!(s.contains("  ✓ B.java\n"));
    }

    #[test]
    fn java_json_shape() {
        let r = clean_result(1, 3, &["X.java"]);
        let s = format_java_clean_imports_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "success");
        assert_eq!(v["total_files"], 3);
        assert_eq!(v["modified_files"], 1);
        assert_eq!(v["modified"][0], "X.java");
        assert!(!s.ends_with('\n'));
    }

    #[test]
    fn java_markdown_with_and_without_modifications() {
        let none = format_java_clean_imports_markdown(&clean_result(0, 2, &[]));
        assert!(none.contains("- **Total files**: 2\n"));
        assert!(none.ends_with("\nNo files needed cleaning.\n"));

        let some = format_java_clean_imports_markdown(&clean_result(1, 2, &["X.java"]));
        assert!(some.contains("## Modified Files\n\n"));
        assert!(some.contains("- `X.java`\n"));
    }

    #[test]
    fn dart_text_quiet_and_verbose() {
        let r = dart_result(&["a.dart", "b.dart"]);
        assert_eq!(format_dart_scaffold_text(&r, false, true), "ok\n");
        let s = format_dart_scaffold_text(&r, true, false);
        assert!(s.starts_with(
            "Dart scaffold created: pubspec.yaml + barrel library (2 model files).\n"
        ));
        assert!(s.contains("  ✓ a.dart\n"));
    }

    #[test]
    fn dart_json_shape() {
        let r = dart_result(&["m.dart"]);
        let s = format_dart_scaffold_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "success");
        assert_eq!(v["pubspec_created"], true);
        assert_eq!(v["barrel_created"], true);
        assert_eq!(v["model_files"][0], "m.dart");
        assert!(!s.ends_with('\n'));
    }

    #[test]
    fn dart_markdown_shapes() {
        let empty = DartScaffoldResult {
            pubspec_created: false,
            barrel_created: false,
            model_files: vec![],
        };
        let s = format_dart_scaffold_markdown(&empty);
        assert!(s.contains("- **pubspec.yaml**: not created\n"));
        assert!(s.contains("- **Barrel library**: not created\n"));
        assert!(s.contains("- **Model files**: 0\n"));
        assert!(!s.contains("## Model Files"));

        let some = format_dart_scaffold_markdown(&dart_result(&["x.dart"]));
        assert!(some.contains("- **pubspec.yaml**: created\n"));
        assert!(some.contains("## Model Files\n\n"));
        assert!(some.contains("- `x.dart`\n"));
    }
}
