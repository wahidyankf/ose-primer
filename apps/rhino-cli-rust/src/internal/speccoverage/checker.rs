use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Instant;

use anyhow::Error;
use regex::Regex;
use walkdir::WalkDir;

use super::extractors::{self, StepMatcher, add_step_to_matcher};
use super::parser::parse_feature_file;
use super::types::{CheckResult, CoverageGap, ScanOptions, ScenarioGap, StepGap};
use super::util::{first_non_empty, normalize_ws, unescape_string};

// --- TS/Go scenario + step regexes (mirroring the Go `checker.go` vars) ---

fn scenario_def_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"Scenario\s*\(\s*(?:"((?:[^"\\]|\\.)*)"|'((?:[^'\\]|\\.)*)')\s*,"#)
            .expect("valid regex")
    })
}

fn step_def_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r#"(?s)(?:Given|When|Then|And|But)\s*\(\s*(?:"((?:[^"\\]|\\.)*)"|'((?:[^'\\]|\\.)*)')\s*,"#,
        )
        .expect("valid regex")
    })
}

fn ts_regex_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?s)(?:Given|When|Then|And|But)\s*\(\s*/\^?(.*?)\$?\s*/\s*,")
            .expect("valid regex")
    })
}

fn go_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\.Step\(`([^`]+)`").expect("valid regex"))
}

fn go_scenario_comment_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"//\s*Scenario:\s*(.+?)\s*$").expect("valid regex"))
}

fn skip_dirs() -> &'static HashSet<&'static str> {
    static SET: OnceLock<HashSet<&'static str>> = OnceLock::new();
    SET.get_or_init(|| {
        [
            "node_modules",
            ".next",
            "build",
            "dist",
            "storybook-static",
            "coverage",
            ".git",
            "target",
            "_build",
            "deps",
            "bin",
            "obj",
            "__pycache__",
            ".pytest_cache",
            ".venv",
            "generated-contracts",
            "generated_contracts",
            ".dart_tool",
            ".features-gen",
        ]
        .into_iter()
        .collect()
    })
}

/// Entry point. In default mode: 1:1 file matching + scenario + step validation.
/// In `--shared-steps` mode: step-only validation across ALL source files.
/// Mirrors Go `CheckAll`.
pub fn check_all(opts: &ScanOptions) -> std::result::Result<CheckResult, Error> {
    if opts.shared_steps {
        check_shared_steps(opts)
    } else {
        check_one_to_one(opts)
    }
}

fn check_shared_steps(opts: &ScanOptions) -> std::result::Result<CheckResult, Error> {
    let start = Instant::now();
    let spec_files = walk_feature_files(&opts.specs_dir, &opts.exclude_dirs)?;
    let all_step_texts = extract_all_step_texts(&opts.app_dir)?;
    let mut step_gaps: Vec<StepGap> = Vec::new();
    let mut total_scenarios = 0usize;
    let mut total_steps = 0usize;

    for spec_file in &spec_files {
        let rel_spec = rel_to(&opts.repo_root, spec_file);
        let scenarios = parse_feature_file(spec_file)?;
        for sc in &scenarios {
            total_scenarios += 1;
            for step in &sc.steps {
                total_steps += 1;
                if !all_step_texts.matches(&step.text) {
                    step_gaps.push(StepGap {
                        spec_file: rel_spec.clone(),
                        scenario_title: sc.title.clone(),
                        step_keyword: step.keyword.clone(),
                        step_text: step.text.clone(),
                    });
                }
            }
        }
    }

    Ok(CheckResult {
        total_specs: spec_files.len(),
        total_scenarios,
        total_steps,
        gaps: Vec::new(),
        scenario_gaps: Vec::new(),
        step_gaps,
        duration: start.elapsed(),
    })
}

fn check_one_to_one(opts: &ScanOptions) -> std::result::Result<CheckResult, Error> {
    let start = Instant::now();
    let spec_files = walk_feature_files(&opts.specs_dir, &opts.exclude_dirs)?;
    let all_step_texts = extract_all_step_texts(&opts.app_dir)?;
    let mut gaps: Vec<CoverageGap> = Vec::new();
    let mut scenario_gaps: Vec<ScenarioGap> = Vec::new();
    let mut step_gaps: Vec<StepGap> = Vec::new();
    let mut total_scenarios = 0usize;
    let mut total_steps = 0usize;

    for spec_file in &spec_files {
        let stem = spec_file
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.strip_suffix(".feature").unwrap_or(s).to_string())
            .unwrap_or_default();

        let test_file_path = find_matching_test_file(&opts.app_dir, &stem)?;

        if test_file_path.is_none() {
            let rel_path = rel_to(&opts.repo_root, spec_file);
            gaps.push(CoverageGap {
                spec_file: rel_path,
                stem: stem.clone(),
            });
            continue; // skip scenario/step check — no test file to check against
        }
        let test_file_path = test_file_path.expect("checked is_none above");

        let rel_spec = rel_to(&opts.repo_root, spec_file);
        let scenarios = parse_feature_file(spec_file)?;
        let scenario_titles = extract_scenario_titles(&test_file_path)?;

        for sc in &scenarios {
            total_scenarios += 1;
            let normalized = normalize_ws(&sc.title);
            if !scenario_titles.contains(&normalized) {
                scenario_gaps.push(ScenarioGap {
                    spec_file: rel_spec.clone(),
                    scenario_title: sc.title.clone(),
                });
            }

            for step in &sc.steps {
                total_steps += 1;
                if !all_step_texts.matches(&step.text) {
                    step_gaps.push(StepGap {
                        spec_file: rel_spec.clone(),
                        scenario_title: sc.title.clone(),
                        step_keyword: step.keyword.clone(),
                        step_text: step.text.clone(),
                    });
                }
            }
        }
    }

    Ok(CheckResult {
        total_specs: spec_files.len(),
        total_scenarios,
        total_steps,
        gaps,
        scenario_gaps,
        step_gaps,
        duration: start.elapsed(),
    })
}

/// Returns all `.feature` files under `dir` recursively (lexically ordered to
/// mirror Go's `filepath.Walk`), excluding directories named in `exclude_dirs`.
fn walk_feature_files(
    dir: &Path,
    exclude_dirs: &[String],
) -> std::result::Result<Vec<PathBuf>, Error> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let excl: HashSet<&str> = exclude_dirs.iter().map(String::as_str).collect();
    let mut files = Vec::new();
    let walker = WalkDir::new(dir)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() && e.depth() > 0 {
                let name = e.file_name().to_string_lossy();
                !excl.contains(name.as_ref())
            } else {
                true
            }
        });
    for entry in walker {
        let entry = entry?;
        if entry.file_type().is_file() && entry.path().to_string_lossy().ends_with(".feature") {
            files.push(entry.path().to_path_buf());
        }
    }
    Ok(files)
}

/// Converts a kebab-case stem to PascalCase. Mirrors Go `toPascalCase`.
fn to_pascal_case(stem: &str) -> String {
    let mut b = String::new();
    for p in stem.split('-') {
        if p.is_empty() {
            continue;
        }
        let mut chars = p.chars();
        if let Some(c) = chars.next() {
            for u in c.to_uppercase() {
                b.push(u);
            }
            b.push_str(chars.as_str());
        }
    }
    b
}

/// Whether a file's base name matches a feature stem (kebab/snake/Pascal/test_).
/// Mirrors Go `matchesStem`.
fn matches_stem(base: &str, stem: &str) -> bool {
    let snake = stem.replace('-', "_");
    let pascal = to_pascal_case(stem);
    let test_snake = format!("test_{snake}");

    let prefixes = [
        format!("{stem}."),
        format!("{stem}_"),
        format!("{snake}."),
        format!("{snake}_"),
        pascal.clone(),
        format!("{test_snake}."),
        format!("{test_snake}_"),
    ];
    for prefix in &prefixes {
        if base.starts_with(prefix) {
            return true;
        }
    }
    base == stem || base == snake
}

/// Whether a file is a test file per language conventions. Mirrors Go
/// `isTestFile`. `ext` is the extension WITHOUT the leading dot.
fn is_test_file(path: &Path) -> bool {
    let base = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    match ext {
        "" => true,
        "go" => base.ends_with("_test.go"),
        "ts" | "tsx" | "js" | "jsx" => {
            base.contains(".test.")
                || base.contains(".spec.")
                || base.contains(".steps.")
                || base.contains(".integration.")
                || base.contains("_test.")
        }
        "java" | "kt" => is_in_test_dir(path),
        "py" => base.starts_with("test_") || base.ends_with("_test.py") || is_in_test_dir(path),
        "exs" => base.ends_with("_test.exs") || base.ends_with("_steps.exs"),
        "rs" => base.ends_with("_test.rs") || is_in_test_dir(path),
        "fs" | "cs" => {
            is_in_test_dir(path)
                || base.ends_with("Steps.cs")
                || base.ends_with("Tests.cs")
                || base.ends_with("Steps.fs")
                || base.ends_with("Tests.fs")
        }
        "clj" => base.ends_with("_test.clj") || base.ends_with("_steps.clj"),
        "dart" => base.ends_with("_test.dart") || is_in_test_dir(path),
        _ => false,
    }
}

fn is_in_test_dir(path: &Path) -> bool {
    path.components()
        .any(|comp| matches!(comp.as_os_str().to_str(), Some("test" | "tests" | "Tests")))
}

/// Returns the path of the FIRST test file under `app_dir` matching the stem
/// (lexical order, matching Go's `filepath.Walk` + `SkipAll`), or `None`.
/// Mirrors Go `findMatchingTestFile`.
fn find_matching_test_file(
    app_dir: &Path,
    stem: &str,
) -> std::result::Result<Option<PathBuf>, Error> {
    if !app_dir.exists() {
        return Ok(None);
    }
    let walker = WalkDir::new(app_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() && e.depth() > 0 {
                let name = e.file_name().to_string_lossy();
                !skip_dirs().contains(name.as_ref())
            } else {
                true
            }
        });
    for entry in walker {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let base = entry.file_name().to_string_lossy();
        if matches_stem(&base, stem) && is_test_file(entry.path()) {
            return Ok(Some(entry.path().to_path_buf()));
        }
    }
    Ok(None)
}

/// Reads ONLY the matching test file and returns scenario titles (normalized),
/// dispatching by extension. Mirrors Go `extractScenarioTitles`.
fn extract_scenario_titles(test_file_path: &Path) -> std::result::Result<HashSet<String>, Error> {
    let ext = test_file_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    match ext {
        "go" | "java" | "kt" | "cs" | "rs" | "dart" => extract_go_scenario_titles(test_file_path),
        "py" => extractors::extract_python_scenario_titles(test_file_path),
        "exs" | "fs" | "clj" => Ok(HashSet::new()),
        _ => extract_ts_scenario_titles(test_file_path),
    }
}

fn extract_ts_scenario_titles(p: &Path) -> std::result::Result<HashSet<String>, Error> {
    let content = fs::read_to_string(p)?;
    let mut titles = HashSet::new();
    for line in content.lines() {
        for caps in scenario_def_re().captures_iter(line) {
            let dq = caps.get(1).map_or("", |m| m.as_str());
            let sq = caps.get(2).map_or("", |m| m.as_str());
            let title = unescape_string(first_non_empty(dq, sq));
            titles.insert(normalize_ws(&title));
        }
    }
    Ok(titles)
}

fn extract_go_scenario_titles(p: &Path) -> std::result::Result<HashSet<String>, Error> {
    let content = fs::read_to_string(p)?;
    let mut titles = HashSet::new();
    for line in content.lines() {
        if let Some(caps) = go_scenario_comment_re().captures(line) {
            titles.insert(normalize_ws(caps.get(1).map_or("", |m| m.as_str())));
        }
    }
    Ok(titles)
}

/// Walks ALL source files under `app_dir`, skipping build-artifact dirs, and
/// returns a [`StepMatcher`] of all step definitions found. Mirrors Go
/// `extractAllStepTexts`.
pub fn extract_all_step_texts(app_dir: &Path) -> std::result::Result<StepMatcher, Error> {
    let mut sm = StepMatcher::new();
    if !app_dir.exists() {
        return Ok(sm);
    }

    let walker = WalkDir::new(app_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() && e.depth() > 0 {
                let name = e.file_name().to_string_lossy();
                !skip_dirs().contains(name.as_ref())
            } else {
                true
            }
        });

    for entry in walker {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let _ = match ext {
            "ts" | "tsx" | "js" | "jsx" => extract_ts_step_texts(path, &mut sm),
            "go" => extract_go_step_texts(path, &mut sm),
            "java" | "kt" => extractors::extract_jvm_step_texts(path, &mut sm),
            "py" => extractors::extract_python_step_texts(path, &mut sm),
            "ex" | "exs" => extractors::extract_elixir_step_texts(path, &mut sm),
            "rs" => extractors::extract_rust_step_texts(path, &mut sm),
            "cs" => extractors::extract_csharp_step_texts(path, &mut sm),
            "fs" => extractors::extract_fsharp_step_texts(path, &mut sm),
            "clj" => extractors::extract_clojure_step_texts(path, &mut sm),
            "dart" => extractors::extract_dart_step_texts(path, &mut sm),
            _ => Ok(()),
        };
    }
    Ok(sm)
}

/// Reads a TS/JS file (raw, no comment stripping — matching Go) and adds all
/// step texts and regex-literal patterns. Mirrors Go `extractTSStepTexts`.
fn extract_ts_step_texts(path: &Path, sm: &mut StepMatcher) -> std::result::Result<(), Error> {
    let src = fs::read_to_string(path)?;

    for caps in step_def_re().captures_iter(&src) {
        let dq = caps.get(1).map_or("", |m| m.as_str());
        let sq = caps.get(2).map_or("", |m| m.as_str());
        let text = unescape_string(first_non_empty(dq, sq));
        add_step_to_matcher(sm, &text);
    }
    for caps in ts_regex_step_re().captures_iter(&src) {
        let pattern = caps.get(1).map_or("", |m| m.as_str());
        if let Ok(re) = Regex::new(pattern) {
            sm.add_pattern_public(re);
        }
    }
    Ok(())
}

fn extract_go_step_texts(path: &Path, sm: &mut StepMatcher) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        for caps in go_step_re().captures_iter(line) {
            let pattern = caps.get(1).map_or("", |m| m.as_str());
            if let Ok(re) = Regex::new(pattern) {
                sm.add_pattern_public(re);
            }
        }
    }
    Ok(())
}

fn rel_to(root: &Path, p: &Path) -> String {
    if root.as_os_str().is_empty() {
        return p.to_string_lossy().to_string();
    }
    p.strip_prefix(root).map_or_else(
        |_| p.to_string_lossy().to_string(),
        |r| r.to_string_lossy().to_string(),
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn to_pascal_case_handles_kebab() {
        assert_eq!(to_pascal_case("health-check"), "HealthCheck");
        assert_eq!(to_pascal_case("user"), "User");
        assert_eq!(to_pascal_case("a-b-c"), "ABC");
    }

    #[test]
    fn matches_stem_kebab_snake_pascal_test_prefix() {
        assert!(matches_stem("user-login.test.ts", "user-login"));
        assert!(matches_stem("user_login.test.ts", "user-login"));
        assert!(matches_stem("UserLogin", "user-login"));
        assert!(matches_stem("test_user_login.py", "user-login"));
        assert!(matches_stem("user-login", "user-login"));
        assert!(!matches_stem("other.ts", "user-login"));
    }

    #[test]
    fn is_test_file_cases() {
        assert!(is_test_file(Path::new("x.test.ts")));
        assert!(is_test_file(Path::new("x.spec.ts")));
        assert!(!is_test_file(Path::new("x.ts")));
        assert!(is_test_file(Path::new("foo_test.go")));
        assert!(!is_test_file(Path::new("foo.go")));
        assert!(is_test_file(Path::new("test_foo.py")));
        assert!(is_test_file(Path::new("tests/foo.py")));
    }

    #[test]
    fn is_in_test_dir_detects_test_segments() {
        assert!(is_in_test_dir(Path::new("a/tests/b.rs")));
        assert!(is_in_test_dir(Path::new("a/test/b.rs")));
        assert!(is_in_test_dir(Path::new("a/Tests/b.cs")));
        assert!(!is_in_test_dir(Path::new("a/b/c.rs")));
    }

    #[test]
    fn walk_feature_files_returns_features_recursively() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("a/b")).unwrap();
        std::fs::write(tmp.path().join("x.feature"), "Feature: x").unwrap();
        std::fs::write(tmp.path().join("a/y.feature"), "Feature: y").unwrap();
        std::fs::write(tmp.path().join("a/b/z.feature"), "Feature: z").unwrap();
        std::fs::write(tmp.path().join("not.txt"), "no").unwrap();
        let files = walk_feature_files(tmp.path(), &[]).unwrap();
        assert_eq!(files.len(), 3);
    }

    #[test]
    fn walk_feature_files_skips_excluded_dirs() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("skip-me")).unwrap();
        std::fs::write(tmp.path().join("x.feature"), "Feature: x").unwrap();
        std::fs::write(tmp.path().join("skip-me/y.feature"), "Feature: y").unwrap();
        let files = walk_feature_files(tmp.path(), &["skip-me".to_string()]).unwrap();
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn extract_all_step_texts_aggregates_across_languages() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join("steps.go"),
            "func step(sc *godog.ScenarioContext) {\n  sc.Step(`^user logs in$`, login)\n}\n",
        )
        .unwrap();
        std::fs::write(
            tmp.path().join("Steps.java"),
            "@Given(\"a user\")\nvoid step() {}\n",
        )
        .unwrap();
        let sm = extract_all_step_texts(tmp.path()).unwrap();
        assert!(sm.matches("user logs in"));
        assert!(sm.matches("a user"));
    }

    #[test]
    fn check_all_one_to_one_reports_file_gap() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path().join("specs");
        let app = tmp.path().join("app");
        std::fs::create_dir_all(&specs).unwrap();
        std::fs::create_dir_all(&app).unwrap();
        std::fs::write(
            specs.join("user-login.feature"),
            "Feature: x\nScenario: T\n  Given x\n",
        )
        .unwrap();
        let opts = ScanOptions {
            repo_root: tmp.path().to_path_buf(),
            specs_dir: specs.clone(),
            app_dir: app.clone(),
            verbose: false,
            quiet: false,
            shared_steps: false,
            exclude_dirs: vec![],
        };
        let r = check_all(&opts).unwrap();
        assert_eq!(r.gaps.len(), 1);
        assert_eq!(r.gaps[0].stem, "user-login");
    }

    #[test]
    fn check_all_shared_steps_skips_file_matching() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path().join("specs");
        let app = tmp.path().join("app");
        std::fs::create_dir_all(&specs).unwrap();
        std::fs::create_dir_all(&app).unwrap();
        std::fs::write(
            specs.join("foo.feature"),
            "Feature: x\nScenario: T\n  Given user logs in\n",
        )
        .unwrap();
        std::fs::write(
            app.join("steps.go"),
            "// stub\nfunc x(sc *godog.ScenarioContext) {\n  sc.Step(`^user logs in$`, fn)\n}\n",
        )
        .unwrap();
        let opts = ScanOptions {
            repo_root: tmp.path().to_path_buf(),
            specs_dir: specs.clone(),
            app_dir: app.clone(),
            verbose: false,
            quiet: false,
            shared_steps: true,
            exclude_dirs: vec![],
        };
        let r = check_all(&opts).unwrap();
        assert_eq!(r.gaps.len(), 0);
        assert_eq!(r.step_gaps.len(), 0);
        assert_eq!(r.total_scenarios, 1);
    }

    #[test]
    fn extract_ts_scenario_titles_picks_up_titles() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("x.test.ts");
        std::fs::write(
            &p,
            "Scenario(\"User logs in\", () => {});\nScenario('Another title', () => {});\n",
        )
        .unwrap();
        let titles = extract_ts_scenario_titles(&p).unwrap();
        assert!(titles.contains("User logs in"));
        assert!(titles.contains("Another title"));
    }

    #[test]
    fn extract_go_scenario_titles_picks_up_comment() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("x_test.go");
        std::fs::write(&p, "// Scenario: Foo bar baz\nfunc Test() {}\n").unwrap();
        let titles = extract_go_scenario_titles(&p).unwrap();
        assert!(titles.contains("Foo bar baz"));
    }

    #[test]
    fn find_matching_test_file_respects_skip_dirs_and_first_match() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("node_modules")).unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::write(tmp.path().join("node_modules/x.test.ts"), "").unwrap();
        std::fs::write(tmp.path().join("src/x.test.ts"), "").unwrap();
        let found = find_matching_test_file(tmp.path(), "x").unwrap().unwrap();
        assert!(found.to_string_lossy().contains("src"));
    }

    #[test]
    fn find_matching_test_file_returns_none_for_missing_dir() {
        assert!(
            find_matching_test_file(Path::new("/nonexistent"), "x")
                .unwrap()
                .is_none()
        );
    }

    fn opts_for(tmp: &TempDir, specs: &Path, app: &Path, shared: bool) -> ScanOptions {
        ScanOptions {
            repo_root: tmp.path().to_path_buf(),
            specs_dir: specs.to_path_buf(),
            app_dir: app.to_path_buf(),
            verbose: false,
            quiet: false,
            shared_steps: shared,
            exclude_dirs: vec![],
        }
    }

    #[test]
    fn check_one_to_one_reports_scenario_and_step_gaps() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path().join("specs");
        let app = tmp.path().join("app");
        std::fs::create_dir_all(&specs).unwrap();
        std::fs::create_dir_all(&app).unwrap();
        // Feature matched by a TS test file, but the title and step differ.
        std::fs::write(
            specs.join("user-login.feature"),
            "Feature: x\nScenario: Real Title\n  Given an unmatched step\n",
        )
        .unwrap();
        std::fs::write(
            app.join("user-login.test.ts"),
            "Scenario(\"Other Title\", () => {});\nGiven(\"a different step\", () => {});\n",
        )
        .unwrap();
        let r = check_all(&opts_for(&tmp, &specs, &app, false)).unwrap();
        assert_eq!(r.gaps.len(), 0); // file matched
        assert_eq!(r.scenario_gaps.len(), 1);
        assert_eq!(r.scenario_gaps[0].scenario_title, "Real Title");
        assert_eq!(r.step_gaps.len(), 1);
        assert_eq!(r.step_gaps[0].step_text, "an unmatched step");
    }

    #[test]
    fn check_one_to_one_all_covered_no_gaps() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path().join("specs");
        let app = tmp.path().join("app");
        std::fs::create_dir_all(&specs).unwrap();
        std::fs::create_dir_all(&app).unwrap();
        std::fs::write(
            specs.join("user-login.feature"),
            "Feature: x\nScenario: Logs in\n  Given a user\n",
        )
        .unwrap();
        std::fs::write(
            app.join("user-login.test.ts"),
            "Scenario(\"Logs in\", () => {});\nGiven(\"a user\", () => {});\n",
        )
        .unwrap();
        let r = check_all(&opts_for(&tmp, &specs, &app, false)).unwrap();
        assert_eq!(r.gaps.len(), 0);
        assert_eq!(r.scenario_gaps.len(), 0);
        assert_eq!(r.step_gaps.len(), 0);
        assert_eq!(r.total_scenarios, 1);
        assert_eq!(r.total_steps, 1);
    }

    #[test]
    fn check_shared_steps_reports_step_gap() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path().join("specs");
        let app = tmp.path().join("app");
        std::fs::create_dir_all(&specs).unwrap();
        std::fs::create_dir_all(&app).unwrap();
        std::fs::write(
            specs.join("a.feature"),
            "Feature: x\nScenario: T\n  Given an uncovered step\n",
        )
        .unwrap();
        std::fs::write(app.join("steps.ts"), "Given(\"other\", () => {});\n").unwrap();
        let r = check_all(&opts_for(&tmp, &specs, &app, true)).unwrap();
        assert_eq!(r.step_gaps.len(), 1);
        assert_eq!(r.gaps.len(), 0);
    }

    #[test]
    fn extract_scenario_titles_dispatches_by_extension() {
        let tmp = TempDir::new().unwrap();
        let go = tmp.path().join("x_test.go");
        std::fs::write(&go, "// Scenario: Go T\n").unwrap();
        assert!(extract_scenario_titles(&go).unwrap().contains("Go T"));

        let ts = tmp.path().join("x.test.ts");
        std::fs::write(&ts, "Scenario(\"TS T\", () => {});\n").unwrap();
        assert!(extract_scenario_titles(&ts).unwrap().contains("TS T"));

        let py = tmp.path().join("test_x.py");
        std::fs::write(&py, "@scenario(\"x.feature\", \"Py T\")\ndef t(): pass\n").unwrap();
        assert!(extract_scenario_titles(&py).unwrap().contains("Py T"));

        let fs = tmp.path().join("x.fs");
        std::fs::write(&fs, "// nothing\n").unwrap();
        assert!(extract_scenario_titles(&fs).unwrap().is_empty());
    }

    #[test]
    fn extract_all_step_texts_covers_each_language_dispatch() {
        let tmp = TempDir::new().unwrap();
        let d = tmp.path();
        std::fs::write(d.join("a.ts"), "Given(\"ts step\", () => {});\n").unwrap();
        std::fs::write(d.join("a.py"), "@given(\"py step\")\ndef s(): pass\n").unwrap();
        std::fs::write(d.join("a.rs"), "#[given(\"rs step\")]\nfn s() {}\n").unwrap();
        std::fs::write(d.join("S.java"), "@Given(\"java step\")\nvoid s() {}\n").unwrap();
        std::fs::write(d.join("a.cs"), "[Given(\"cs step\")]\nvoid S() {}\n").unwrap();
        std::fs::write(d.join("a.fs"), "let [<Given>] ``fs step`` () = ()\n").unwrap();
        std::fs::write(d.join("a.clj"), "(Given \"clj step\" [] ...)\n").unwrap();
        std::fs::write(d.join("a.ex"), "defgiven ~r/^ex step$/ do\nend\n").unwrap();
        std::fs::write(
            d.join("a.dart"),
            "s.given(\"dart step\", (Scenario s) async {});",
        )
        .unwrap();
        let sm = extract_all_step_texts(d).unwrap();
        for step in [
            "ts step",
            "py step",
            "rs step",
            "java step",
            "cs step",
            "fs step",
            "clj step",
            "ex step",
            "dart step",
        ] {
            assert!(sm.matches(step), "missing: {step}");
        }
    }

    #[test]
    fn extract_ts_step_texts_handles_regex_literal() {
        let tmp = TempDir::new().unwrap();
        let app = tmp.path();
        std::fs::write(
            app.join("steps.ts"),
            "When(/^count is (\\d+)$/, () => {});\n",
        )
        .unwrap();
        let sm = extract_all_step_texts(app).unwrap();
        assert!(sm.matches("count is 42"));
    }
}
