use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Error;

/// A single step line from a Gherkin scenario.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedStep {
    /// Given/When/Then/And/But (title case, trimmed).
    pub keyword: String,
    /// Trimmed step text.
    pub text: String,
}

/// A Gherkin Scenario block with its steps.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParsedScenario {
    pub title: String,
    pub steps: Vec<ParsedStep>,
}

const STEP_KEYWORDS: [&str; 5] = ["Given ", "When ", "Then ", "And ", "But "];

/// Reads a `.feature` file and returns all scenarios and their steps.
///
/// Background steps are collected and prepended as a synthetic `(Background)`
/// scenario so that Background step definitions are validated too. Mirrors
/// Go `ParseFeatureFile`.
pub fn parse_feature_file(path: &Path) -> std::result::Result<Vec<ParsedScenario>, Error> {
    let file = File::open(path)?;
    let mut scenarios: Vec<ParsedScenario> = Vec::new();
    let mut bg_steps: Vec<ParsedStep> = Vec::new();
    let mut in_background = false;
    let mut current_idx: Option<usize> = None;

    for raw in BufReader::new(file).lines() {
        let Ok(line_owned) = raw else { continue };
        let line = line_owned.trim();

        if line.starts_with("Background:") {
            in_background = true;
            current_idx = None;
            continue;
        }

        if let Some(rest) = line.strip_prefix("Scenario:") {
            in_background = false;
            scenarios.push(ParsedScenario {
                title: rest.trim().to_string(),
                steps: Vec::new(),
            });
            current_idx = Some(scenarios.len() - 1);
            continue;
        }

        for kw in STEP_KEYWORDS {
            if let Some(rest) = line.strip_prefix(kw) {
                let step = ParsedStep {
                    keyword: kw.trim().to_string(),
                    text: rest.trim().to_string(),
                };
                if in_background {
                    bg_steps.push(step);
                } else if let Some(idx) = current_idx {
                    scenarios[idx].steps.push(step);
                }
                break;
            }
        }
    }

    if !bg_steps.is_empty() {
        let bg = ParsedScenario {
            title: "(Background)".to_string(),
            steps: bg_steps,
        };
        scenarios.insert(0, bg);
    }

    Ok(scenarios)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write_feature(content: &str) -> (TempDir, std::path::PathBuf) {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("x.feature");
        fs::write(&p, content).unwrap();
        (tmp, p)
    }

    #[test]
    fn parses_simple_scenario_with_three_steps() {
        let (_tmp, p) = write_feature(
            "Feature: foo\n\nScenario: bar\n  Given a precondition\n  When an action\n  Then an outcome\n",
        );
        let scenarios = parse_feature_file(&p).unwrap();
        assert_eq!(scenarios.len(), 1);
        assert_eq!(scenarios[0].title, "bar");
        assert_eq!(scenarios[0].steps.len(), 3);
        assert_eq!(scenarios[0].steps[0].keyword, "Given");
        assert_eq!(scenarios[0].steps[0].text, "a precondition");
        assert_eq!(scenarios[0].steps[2].keyword, "Then");
    }

    #[test]
    fn background_steps_yield_synthetic_first_scenario() {
        let (_tmp, p) = write_feature(
            "Feature: foo\n\nBackground:\n  Given baseline\n\nScenario: bar\n  Then result\n",
        );
        let scenarios = parse_feature_file(&p).unwrap();
        assert_eq!(scenarios.len(), 2);
        assert_eq!(scenarios[0].title, "(Background)");
        assert_eq!(scenarios[0].steps[0].text, "baseline");
        assert_eq!(scenarios[1].title, "bar");
    }

    #[test]
    fn missing_file_returns_error() {
        let err = parse_feature_file(Path::new("/nonexistent/foo.feature")).unwrap_err();
        assert!(!err.to_string().is_empty());
    }
}
