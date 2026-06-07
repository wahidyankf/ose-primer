//! Gherkin step-keyword cardinality audit.
//!
//! Byte-for-byte port of
//! `apps/rhino-cli-go/internal/repo-governance/governance_gherkin_keyword_cardinality.go`.
//!
//! Flags every `Scenario` (and `Scenario Outline` body) that uses more than
//! one primary `Given`, `When`, or `Then` keyword line. Primary keywords start
//! the trimmed line; `And`/`But`/`*` chains are not counted. `Background`
//! blocks and `Scenario Outline` `Examples` tables are exempt, and lines
//! inside doc-strings (`"""` or fenced) and comment lines (`#`) are ignored.

use std::fs;
use std::path::Path;

use anyhow::{Context, Error};
use walkdir::WalkDir;

/// A single scenario violating the one-each primary-keyword rule. Mirrors Go
/// `CardinalityFinding`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub path: String,
    pub line: usize,
    pub scenario: String,
    pub detail: String,
}

/// Directory names skipped at any depth during the feature-file walk (build
/// outputs, dependency trees, worktrees, archives). Mirrors Go
/// `cardinalityExcludedDirs`.
const EXCLUDED_DIR_NAMES: [&str; 7] = [
    "bin",
    "build",
    "target",
    "dist",
    "node_modules",
    "worktrees",
    "archived",
];

/// Slash-path fragments identifying BDD-library self-test fixture trees,
/// excluded wherever they appear (those fixtures test the Gherkin parser
/// itself and may deliberately use odd shapes). Mirrors Go
/// `cardinalityExcludedPathParts`.
const EXCLUDED_PATH_PARTS: [&str; 2] = [
    "libs/elixir-cabbage/test/features/",
    "libs/elixir-gherkin/test/fixtures/",
];

/// Reads the feature file at `path` and returns all cardinality findings.
/// Mirrors Go `ScanFeatureFile`.
pub fn scan_feature_file(path: &Path) -> Result<Vec<Finding>, Error> {
    let data = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    Ok(scan_feature_content(&path.to_string_lossy(), &data))
}

/// Walks all `.feature` files under `root` recursively and returns all
/// findings sorted by (path, line). Excluded directory names and BDD-library
/// fixture trees are skipped. A missing `root` yields an empty slice, not an
/// error. Mirrors Go `WalkFeatures`.
///
/// Entries are visited in lexical (sorted) order to match Go's
/// `filepath.WalkDir`, ensuring byte-identical finding ordering.
pub fn walk_features(root: &Path) -> Result<Vec<Finding>, Error> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut findings = Vec::new();
    let walker = WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| {
            e.depth() == 0
                || !(e.file_type().is_dir()
                    && EXCLUDED_DIR_NAMES.contains(&e.file_name().to_string_lossy().as_ref()))
        });
    for entry in walker.flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".feature") {
            continue;
        }
        let slashed = entry.path().to_string_lossy().replace('\\', "/");
        if EXCLUDED_PATH_PARTS.iter().any(|p| slashed.contains(p)) {
            continue;
        }
        findings.extend(scan_feature_file(entry.path())?);
    }
    sort_findings(&mut findings);
    Ok(findings)
}

/// Orders findings by (path, line) ascending. Mirrors Go
/// `sortCardinalityFindings`.
fn sort_findings(findings: &mut [Finding]) {
    findings.sort_by(|a, b| a.path.cmp(&b.path).then(a.line.cmp(&b.line)));
}

/// Core line-by-line scan of a feature file's content, tracking doc-string
/// state and the current scenario block. Mirrors Go `ScanFeatureContent`.
pub fn scan_feature_content(path: &str, content: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    let mut in_doc_string = false;
    let mut doc_string_delim = "";

    let mut scenario: Option<ScenarioState> = None;

    for (i, line) in content.split('\n').enumerate() {
        let line_num = i + 1;
        let trimmed = line.trim();

        // ── Doc-strings (""" or ```) ──────────────────────────────────────
        if in_doc_string {
            if trimmed.starts_with(doc_string_delim) {
                in_doc_string = false;
            }
            continue;
        }
        if let Some(delim) = doc_string_delimiter(trimmed) {
            in_doc_string = true;
            doc_string_delim = delim;
            continue;
        }

        // ── Comments ──────────────────────────────────────────────────────
        if trimmed.starts_with('#') {
            continue;
        }

        // ── Block headers ─────────────────────────────────────────────────
        if let Some(name) = scenario_header_name(trimmed) {
            flush(&mut scenario, path, &mut findings);
            scenario = Some(ScenarioState::new(name, line_num));
            continue;
        }
        if is_exempt_or_structural_header(trimmed) {
            // Exempt regions (Background, Examples tables) and structural
            // headers end the current scenario's counted body.
            flush(&mut scenario, path, &mut findings);
            continue;
        }

        // ── Primary keyword counting ──────────────────────────────────────
        if let Some(state) = scenario.as_mut() {
            match primary_keyword(trimmed) {
                Some("Given") => state.given += 1,
                Some("When") => state.when += 1,
                Some("Then") => state.then += 1,
                _ => {}
            }
        }
    }
    flush(&mut scenario, path, &mut findings);
    findings
}

/// Per-scenario counting state.
struct ScenarioState {
    name: String,
    line: usize,
    given: usize,
    when: usize,
    then: usize,
}

impl ScenarioState {
    fn new(name: String, line: usize) -> Self {
        Self {
            name,
            line,
            given: 0,
            when: 0,
            then: 0,
        }
    }
}

/// Closes the current scenario block, emitting a finding when any primary
/// keyword repeats.
fn flush(scenario: &mut Option<ScenarioState>, path: &str, findings: &mut Vec<Finding>) {
    if let Some(state) = scenario.take() {
        let detail = cardinality_detail(state.given, state.when, state.then);
        if !detail.is_empty() {
            findings.push(Finding {
                path: path.to_string(),
                line: state.line,
                scenario: state.name,
                detail,
            });
        }
    }
}

/// Returns the scenario name when the trimmed line opens a counted scenario
/// block (`Scenario:`, `Scenario Outline:`, `Scenario Template:`, `Example:`).
fn scenario_header_name(trimmed: &str) -> Option<String> {
    for header in [
        "Scenario Outline:",
        "Scenario Template:",
        "Scenario:",
        "Example:",
    ] {
        if let Some(rest) = trimmed.strip_prefix(header) {
            return Some(rest.trim().to_string());
        }
    }
    None
}

/// Reports whether the trimmed line opens an exempt region (`Background:`,
/// `Examples:` tables) or a structural header (`Rule:`, `Feature:`).
fn is_exempt_or_structural_header(trimmed: &str) -> bool {
    [
        "Background:",
        "Examples:",
        "Scenarios:",
        "Rule:",
        "Feature:",
    ]
    .iter()
    .any(|h| trimmed.starts_with(h))
}

/// Returns the doc-string delimiter opening on this trimmed line (`"""` or
/// "```", possibly followed by a content type), or `None`. Mirrors Go
/// `docStringDelimiter`.
fn doc_string_delimiter(trimmed: &str) -> Option<&'static str> {
    if trimmed.starts_with("\"\"\"") {
        return Some("\"\"\"");
    }
    if trimmed.starts_with("```") {
        return Some("```");
    }
    None
}

/// Classifies a trimmed step line, returning the primary keyword starting it
/// (`Given`, `When`, `Then`) or `None` when the line is a chain step
/// (`And`/`But`/`*`) or not a step at all. Mirrors Go `primaryKeyword`.
fn primary_keyword(trimmed: &str) -> Option<&'static str> {
    ["Given", "When", "Then"].into_iter().find(|kw| {
        trimmed
            .strip_prefix(kw)
            .is_some_and(|rest| rest.starts_with(' '))
    })
}

/// Renders the repeated-keyword summary (e.g. "2 When, 2 Then") for counts
/// above one, in Given/When/Then order. Returns "" when the scenario
/// conforms. Mirrors Go `cardinalityDetail`.
fn cardinality_detail(given: usize, when: usize, then: usize) -> String {
    let mut parts = Vec::new();
    if given > 1 {
        parts.push(format!("{given} Given"));
    }
    if when > 1 {
        parts.push(format!("{when} When"));
    }
    if then > 1 {
        parts.push(format!("{then} Then"));
    }
    parts.join(", ")
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn flags_scenario_with_repeated_when() {
        let content = "Feature: F\n\n  Scenario: Double when offender\n    Given a start\n    When the first action runs\n    When the second action runs\n    Then the outcome is checked\n";
        let findings = scan_feature_content("x.feature", content);
        assert_eq!(findings.len(), 1, "got {findings:?}");
        assert_eq!(findings[0].path, "x.feature");
        assert_eq!(findings[0].line, 3);
        assert_eq!(findings[0].scenario, "Double when offender");
        assert_eq!(findings[0].detail, "2 When");
    }

    #[test]
    fn exempts_background_block() {
        let content = "Feature: F\n\n  Background:\n    Given one precondition\n    Given another precondition\n\n  Scenario: Conforming\n    Given a thing\n    When it acts\n    Then it is checked\n";
        let findings = scan_feature_content("x.feature", content);
        assert!(findings.is_empty(), "got {findings:?}");
    }

    #[test]
    fn exempts_scenario_outline_examples() {
        let content = "Feature: F\n\n  Scenario Outline: Outline body obeys the rule\n    Given a value <v>\n    When it is processed\n    Then it succeeds\n\n    Examples:\n      | v |\n      | 1 |\n      | 2 |\n      | 3 |\n";
        let findings = scan_feature_content("x.feature", content);
        assert!(findings.is_empty(), "got {findings:?}");
    }

    #[test]
    fn ignores_docstrings_and_comments() {
        let content = "Feature: F\n\n  Scenario: Docstring and comment heavy\n    Given a setup\n    When something runs with this payload\n      \"\"\"\n      When this line is data, not a step\n      Then neither is this one\n      \"\"\"\n    # Then this comment line is ignored\n    Then the result is checked\n";
        let findings = scan_feature_content("x.feature", content);
        assert!(findings.is_empty(), "got {findings:?}");
    }

    #[test]
    fn sorts_findings_by_path_and_line() {
        let violating = "Feature: F\n\n  Scenario: Late offender\n    Given a start\n    Then one outcome\n    Then another outcome\n\n  Scenario: Early offender on second file\n    Given a start\n    When one action\n    When another action\n    Then an outcome\n";
        let tmp = TempDir::new().unwrap();
        for rel in ["bbb/late.feature", "aaa/early.feature"] {
            let p = tmp.path().join(rel);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(&p, violating).unwrap();
        }

        let findings = walk_features(tmp.path()).unwrap();
        assert_eq!(findings.len(), 4, "got {findings:?}");
        for pair in findings.windows(2) {
            let (prev, cur) = (&pair[0], &pair[1]);
            assert!(
                prev.path < cur.path || (prev.path == cur.path && prev.line <= cur.line),
                "findings not sorted by (path, line): {prev:?} before {cur:?}"
            );
        }
        assert!(
            findings[0]
                .path
                .starts_with(&tmp.path().join("aaa").to_string_lossy().into_owned()),
            "expected first finding under aaa/, got {}",
            findings[0].path
        );
    }

    #[test]
    fn conforming_file_passes() {
        let content = "Feature: F\n\n  Scenario: Conforming chained scenario\n    Given a start\n    And another precondition\n    When the action runs\n    Then the outcome is checked\n    And a second outcome is checked\n    But a third outcome is absent\n";
        let findings = scan_feature_content("x.feature", content);
        assert!(findings.is_empty(), "got {findings:?}");
    }

    #[test]
    fn flags_repeated_given_and_reports_combined_detail() {
        let content = "Feature: F\n\n  Scenario: Multi offender\n    Given a start\n    Given another start\n    When one action\n    When another action\n    Then one outcome\n    Then another outcome\n";
        let findings = scan_feature_content("x.feature", content);
        assert_eq!(findings.len(), 1, "got {findings:?}");
        assert_eq!(findings[0].detail, "2 Given, 2 When, 2 Then");
    }

    #[test]
    fn ignores_backtick_docstrings() {
        let content = "Feature: F\n\n  Scenario: Backtick docstring\n    Given a setup\n    When something runs with this payload\n      ```\n      When this line is data\n      ```\n    Then the result is checked\n";
        let findings = scan_feature_content("x.feature", content);
        assert!(findings.is_empty(), "got {findings:?}");
    }

    #[test]
    fn example_and_rule_headers_handled() {
        let content = "Feature: F\n\n  Rule: A rule\n\n  Example: Offending example block\n    Given a start\n    When one action\n    When another action\n    Then an outcome\n";
        let findings = scan_feature_content("x.feature", content);
        assert_eq!(findings.len(), 1, "got {findings:?}");
        assert_eq!(findings[0].scenario, "Offending example block");
    }

    #[test]
    fn walk_missing_root_is_empty() {
        let tmp = TempDir::new().unwrap();
        let missing = tmp.path().join("does-not-exist");
        let findings = walk_features(&missing).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn walk_skips_excluded_dirs_and_fixture_trees() {
        let violating = "Feature: F\n\n  Scenario: Offender\n    Given a start\n    When one action\n    When another action\n    Then an outcome\n";
        let tmp = TempDir::new().unwrap();
        for rel in [
            "node_modules/dep.feature",
            "worktrees/w/x.feature",
            "archived/old.feature",
            "libs/elixir-cabbage/test/features/self.feature",
            "libs/elixir-gherkin/test/fixtures/self.feature",
        ] {
            let p = tmp.path().join(rel);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(&p, violating).unwrap();
        }
        fs::create_dir_all(tmp.path().join("specs")).unwrap();
        fs::write(tmp.path().join("specs/bad.feature"), violating).unwrap();
        fs::write(tmp.path().join("specs/notes.txt"), violating).unwrap();

        let findings = walk_features(tmp.path()).unwrap();
        assert_eq!(findings.len(), 1, "got {findings:?}");
        assert!(
            findings[0].path.ends_with("specs/bad.feature"),
            "got {}",
            findings[0].path
        );
    }

    #[test]
    fn scan_feature_file_reads_disk() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("doc.feature");
        fs::write(
            &p,
            "Feature: F\n\n  Scenario: Offender\n    Given a start\n    Then one outcome\n    Then another outcome\n",
        )
        .unwrap();
        let findings = scan_feature_file(&p).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].detail, "2 Then");
    }

    #[test]
    fn primary_keyword_classification() {
        assert_eq!(primary_keyword("Given a thing"), Some("Given"));
        assert_eq!(primary_keyword("When acted"), Some("When"));
        assert_eq!(primary_keyword("Then checked"), Some("Then"));
        assert_eq!(primary_keyword("And chained"), None);
        assert_eq!(primary_keyword("But negated"), None);
        assert_eq!(primary_keyword("* bulleted"), None);
        assert_eq!(primary_keyword("Whenever it happens"), None);
        assert_eq!(primary_keyword("Given"), None);
    }
}
