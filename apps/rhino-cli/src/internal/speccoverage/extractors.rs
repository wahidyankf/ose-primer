use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use anyhow::Error;
use regex::Regex;

use super::cucumber_expr::{
    convert_python_parsers_expr, cucumber_expr_to_regex, has_cucumber_expressions,
    is_python_parsers_expr, unescape_cucumber_expr,
};
use super::util::{escape_re2_literal_braces, first_non_empty, normalize_ws, unescape_string};

/// Holds exact step texts (literal matches) and compiled regex patterns.
/// Step texts are whitespace-normalized on insert.
#[derive(Debug, Default)]
pub struct StepMatcher {
    exact: HashMap<String, bool>,
    patterns: Vec<Regex>,
}

impl StepMatcher {
    pub fn new() -> Self {
        Self::default()
    }

    /// True if `step_text` matches an exact entry or any compiled pattern.
    ///
    pub fn matches(&self, step_text: &str) -> bool {
        let normalized = normalize_ws(step_text);
        if self.exact.contains_key(&normalized) {
            return true;
        }
        for re in &self.patterns {
            if re.is_match(&normalized) {
                return true;
            }
        }
        false
    }

    fn add_exact(&mut self, text: String) {
        self.exact.insert(text, true);
    }

    fn add_pattern(&mut self, re: Regex) {
        self.patterns.push(re);
    }

    /// Adds a precompiled regex pattern. Used by the TS/Go extractors in
    /// `checker`, which compile raw patterns directly.
    pub fn add_pattern_public(&mut self, re: Regex) {
        self.patterns.push(re);
    }
}

/// Generic step-text → matcher inserter.
/// - Text starting with `^` → traditional regex.
/// - Text containing `{...}` → Cucumber expression (`^…$` anchored).
/// - Otherwise → exact literal (Cucumber-unescaped first).
pub fn add_step_to_matcher(sm: &mut StepMatcher, text: &str) {
    let text = normalize_ws(text);
    if text.is_empty() {
        return;
    }
    if text.starts_with('^') {
        if let Ok(re) = Regex::new(&escape_re2_literal_braces(&text)) {
            sm.add_pattern(re);
        }
        return;
    }
    if has_cucumber_expressions(&text) {
        let pattern = format!("^{}$", cucumber_expr_to_regex(&text));
        if let Ok(re) = Regex::new(&pattern) {
            sm.add_pattern(re);
        }
        return;
    }
    sm.add_exact(unescape_cucumber_expr(&text));
}

/// Python-specific variant — handles `parsers.parse({name:d})` format strings
/// before falling back to the generic path.
pub fn add_python_step_to_matcher(sm: &mut StepMatcher, text: &str) {
    let text = normalize_ws(text);
    if text.is_empty() {
        return;
    }
    if text.starts_with('^') {
        if let Ok(re) = Regex::new(&escape_re2_literal_braces(&text)) {
            sm.add_pattern(re);
        }
        return;
    }
    if is_python_parsers_expr(&text) {
        let pattern = format!("^{}$", convert_python_parsers_expr(&text));
        if let Ok(re) = Regex::new(&pattern) {
            sm.add_pattern(re);
        }
        return;
    }
    if has_cucumber_expressions(&text) {
        let pattern = format!("^{}$", cucumber_expr_to_regex(&text));
        if let Ok(re) = Regex::new(&pattern) {
            sm.add_pattern(re);
        }
        return;
    }
    sm.add_exact(text);
}

// --- Per-language compiled regexes (mirroring the Go `*_steps.go` files) ---

fn jvm_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"@(?:Given|When|Then|And|But)\s*\(\s*"((?:[^"\\]|\\.)*)"\s*\)"#)
            .expect("valid regex")
    })
}

fn py_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r#"(?s)@(?:given|when|then|step)\s*\(\s*(?:parsers\.(?:parse|cfparse)\s*\(\s*)?(?:"((?:[^"\\]|\\.)*)"|'((?:[^'\\]|\\.)*)')\s*\)?\s*(?:,\s*[^)]*)?\)"#,
        )
        .expect("valid regex")
    })
}

fn py_scenario_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"@scenario\s*\(\s*"[^"]*"\s*,\s*"((?:[^"\\]|\\.)*)"\s*\)"#)
            .expect("valid regex")
    })
}

fn rs_step_literal_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"#\[(?:given|when|then)\s*\(\s*"((?:[^"\\]|\\.)*)"\s*\)\s*\]"#)
            .expect("valid regex")
    })
}

fn rs_step_expr_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"#\[(?:given|when|then)\s*\(\s*expr\s*=\s*"((?:[^"\\]|\\.)*)"\s*\)\s*\]"#)
            .expect("valid regex")
    })
}

fn rs_step_regex_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r##"(?s)#\[(?:given|when|then)\s*\(\s*regex\s*=\s*r#"(.*?)"#\s*\)\s*\]"##)
            .expect("valid regex")
    })
}

/// Matches the hash-less raw-string `regex = r"..."` step-definition form.
fn rs_step_regex_plain_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"#\[(?:given|when|then)\s*\(\s*regex\s*=\s*r"([^"]*)"\s*\)\s*\]"#)
            .expect("valid regex")
    })
}

/// Matches the literal raw-string `#[given(r#"..."#)]` step-definition form
/// (a literal step text wrapped in a hash-delimited raw string, used when the
/// text itself contains `"` characters).
fn rs_step_literal_raw_hash_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r##"(?s)#\[(?:given|when|then)\s*\(\s*r#"(.*?)"#\s*\)\s*\]"##)
            .expect("valid regex")
    })
}

/// Matches the literal hash-less raw-string `#[given(r"...")]` step-definition
/// form.
fn rs_step_literal_raw_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"#\[(?:given|when|then)\s*\(\s*r"([^"]*)"\s*\)\s*\]"#).expect("valid regex")
    })
}

fn cs_verbatim_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?s)\[(?:Given|When|Then|And|But)\s*\(\s*@"((?:[^"]|"")*)"\s*\)\s*\]"#)
            .expect("valid regex")
    })
}

fn cs_regular_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?s)\[(?:Given|When|Then|And|But)\s*\(\s*"((?:[^"\\]|\\.)*)"\s*\)\s*\]"#)
            .expect("valid regex")
    })
}

fn fs_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"let\s+(?:\[<(?:Given|When|Then)>\]\s*)?``((?:[^`]|`[^`])*)``")
            .expect("valid regex")
    })
}

fn dart_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r#"(?s)\b(?:s|scenario)\.(?:given|when|then|and|but)\s*\(\s*(?:"((?:[^"\\]|\\.)*)"|'((?:[^'\\]|\\.)*)')\s*,"#,
        )
        .expect("valid regex")
    })
}

fn clj_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"\((?:Given|When|Then|And|But)\s+"((?:[^"\\]|\\.)*)""#).expect("valid regex")
    })
}

fn ex_step_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"def(?:given|when|then|and_|but_)\s+~r/\^?(.*?)\$?/").expect("valid regex")
    })
}

// --- Per-language extractors (line- or content-based, matching the Go) ---

pub fn extract_jvm_step_texts(path: &Path, sm: &mut StepMatcher) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        for caps in jvm_step_re().captures_iter(line) {
            add_step_to_matcher(sm, caps.get(1).map_or("", |m| m.as_str()));
        }
    }
    Ok(())
}

pub fn extract_python_step_texts(
    path: &Path,
    sm: &mut StepMatcher,
) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    for caps in py_step_re().captures_iter(&content) {
        let dq = caps.get(1).map_or("", |m| m.as_str());
        let sq = caps.get(2).map_or("", |m| m.as_str());
        let text = first_non_empty(dq, sq)
            .replace("{{", "{")
            .replace("}}", "}");
        add_python_step_to_matcher(sm, &text);
    }
    Ok(())
}

/// Extracts `@scenario("feature.feature", "Title")` titles.
pub fn extract_python_scenario_titles(
    test_file_path: &Path,
) -> std::result::Result<HashSet<String>, Error> {
    let content = fs::read_to_string(test_file_path)?;
    let mut titles = HashSet::new();
    for line in content.lines() {
        for caps in py_scenario_re().captures_iter(line) {
            titles.insert(normalize_ws(caps.get(1).map_or("", |m| m.as_str())));
        }
    }
    Ok(titles)
}

pub fn extract_rust_step_texts(
    path: &Path,
    sm: &mut StepMatcher,
) -> std::result::Result<(), Error> {
    // Scan the whole file content (not line-by-line) so that step attributes
    // split across multiple lines — e.g. `#[given(` / `"text"` / `)]` — are
    // matched. The attribute regexes allow newlines between tokens.
    let content = fs::read_to_string(path)?;
    for caps in rs_step_regex_re().captures_iter(&content) {
        let pattern = caps.get(1).map_or("", |m| m.as_str());
        if let Ok(re) = Regex::new(&escape_re2_literal_braces(pattern)) {
            sm.add_pattern(re);
        }
    }
    for caps in rs_step_regex_plain_re().captures_iter(&content) {
        let pattern = caps.get(1).map_or("", |m| m.as_str());
        if let Ok(re) = Regex::new(&escape_re2_literal_braces(pattern)) {
            sm.add_pattern(re);
        }
    }
    for caps in rs_step_expr_re().captures_iter(&content) {
        add_step_to_matcher(sm, caps.get(1).map_or("", |m| m.as_str()));
    }
    for caps in rs_step_literal_re().captures_iter(&content) {
        add_step_to_matcher(sm, caps.get(1).map_or("", |m| m.as_str()));
    }
    for caps in rs_step_literal_raw_hash_re().captures_iter(&content) {
        add_step_to_matcher(sm, caps.get(1).map_or("", |m| m.as_str()));
    }
    for caps in rs_step_literal_raw_re().captures_iter(&content) {
        add_step_to_matcher(sm, caps.get(1).map_or("", |m| m.as_str()));
    }
    Ok(())
}

pub fn extract_csharp_step_texts(
    path: &Path,
    sm: &mut StepMatcher,
) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    for caps in cs_verbatim_step_re().captures_iter(&content) {
        let text = caps.get(1).map_or("", |m| m.as_str()).replace("\"\"", "\"");
        add_step_to_matcher(sm, &text);
    }
    for caps in cs_regular_step_re().captures_iter(&content) {
        add_step_to_matcher(sm, caps.get(1).map_or("", |m| m.as_str()));
    }
    Ok(())
}

pub fn extract_fsharp_step_texts(
    path: &Path,
    sm: &mut StepMatcher,
) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        for caps in fs_step_re().captures_iter(line) {
            let text = normalize_ws(caps.get(1).map_or("", |m| m.as_str()));
            let pattern = format!("^{}$", escape_re2_literal_braces(&text));
            if let Ok(re) = Regex::new(&pattern) {
                sm.add_pattern(re);
            }
        }
    }
    Ok(())
}

pub fn extract_dart_step_texts(
    path: &Path,
    sm: &mut StepMatcher,
) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    for caps in dart_step_re().captures_iter(&content) {
        let dq = caps.get(1).map_or("", |m| m.as_str());
        let sq = caps.get(2).map_or("", |m| m.as_str());
        let text = unescape_string(first_non_empty(dq, sq));
        add_step_to_matcher(sm, &text);
    }
    Ok(())
}

pub fn extract_clojure_step_texts(
    path: &Path,
    sm: &mut StepMatcher,
) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        for caps in clj_step_re().captures_iter(line) {
            add_step_to_matcher(sm, caps.get(1).map_or("", |m| m.as_str()));
        }
    }
    Ok(())
}

pub fn extract_elixir_step_texts(
    path: &Path,
    sm: &mut StepMatcher,
) -> std::result::Result<(), Error> {
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        for caps in ex_step_re().captures_iter(line) {
            let pattern = caps.get(1).map_or("", |m| m.as_str());
            if let Ok(re) = Regex::new(&escape_re2_literal_braces(pattern)) {
                sm.add_pattern(re);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write(dir: &Path, name: &str, content: &str) -> std::path::PathBuf {
        let p = dir.join(name);
        fs::write(&p, content).unwrap();
        p
    }

    #[test]
    fn add_exact_lookup_via_matches() {
        let mut sm = StepMatcher::new();
        add_step_to_matcher(&mut sm, "user logs in");
        assert!(sm.matches("user logs in"));
        assert!(sm.matches("user  logs  in"));
        assert!(!sm.matches("user logs out"));
    }

    #[test]
    fn add_pattern_via_cucumber_expression() {
        let mut sm = StepMatcher::new();
        add_step_to_matcher(&mut sm, "user enters {string}");
        assert!(sm.matches(r#"user enters "alice""#));
        assert!(!sm.matches("user enters alice"));
    }

    #[test]
    fn add_pattern_via_raw_caret_regex() {
        let mut sm = StepMatcher::new();
        add_step_to_matcher(&mut sm, r"^count is (\d+)$");
        assert!(sm.matches("count is 42"));
        assert!(!sm.matches("count is forty-two"));
    }

    #[test]
    fn python_parsers_d_compiles_correctly() {
        let mut sm = StepMatcher::new();
        add_python_step_to_matcher(&mut sm, "ratio {n:d}");
        assert!(sm.matches("ratio 42"));
        assert!(!sm.matches("ratio abc"));
    }

    #[test]
    fn jvm_step_extraction() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "Steps.java",
            "@Given(\"user logs in\")\nvoid step() {}\n@When(\"submits {string}\")\nvoid step2() {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_jvm_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("user logs in"));
        assert!(sm.matches(r#"submits "alice""#));
    }

    #[test]
    fn jvm_regex_anchored_pattern_with_literal_braces() {
        // Regression for the Kotlin spec-coverage divergence: a `^…$`-anchored
        // JVM annotation whose body holds literal `{…}` braces (the shape used
        // by `SpecCoverageMarkers.kt`). Go's RE2 accepts these as literal
        // braces; Rust's regex crate rejects them unless they are escaped first.
        // Before the fix the pattern was silently dropped → phantom step gap.
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "SpecCoverageMarkers.kt",
            concat!(
                "object Markers {\n",
                "  // @When(\"^the admin sends POST /api/v1/admin/users/{alice_id}/enable$\")\n",
                "  // @When(\"^the admin sends POST /api/v1/admin/users/{alice_id}/disable with body { \\\"reason\\\": \\\"([^\\\"]+)\\\" }$\")\n",
                "}\n",
            ),
        );
        let mut sm = StepMatcher::new();
        extract_jvm_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("the admin sends POST /api/v1/admin/users/{alice_id}/enable"));
        assert!(sm.matches(
            r#"the admin sends POST /api/v1/admin/users/{alice_id}/disable with body { "reason": "Policy violation" }"#
        ));
    }

    #[test]
    fn jvm_cucumber_expression_with_literal_braces_in_body() {
        // A non-anchored JVM Cucumber-expression step whose literal body holds a
        // JSON brace alongside a `{string}` parameter. The literal `{`/`}` must
        // not break compilation of the generated regex.
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "Steps.kt",
            "@When(\"alice sends PATCH /api/v1/users/me with body { \\\"displayName\\\": {string} }\")\nfun s() {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_jvm_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches(
            r#"alice sends PATCH /api/v1/users/me with body { "displayName": "Alice Smith" }"#
        ));
    }

    #[test]
    fn rust_literal_and_expr_and_regex() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "x.rs",
            "#[given(\"user logs in\")]\n#[when(expr = \"count is {int}\")]\n#[then(regex = r#\"^result is (\\d+)$\"#)]\n",
        );
        let mut sm = StepMatcher::new();
        extract_rust_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("user logs in"));
        assert!(sm.matches("count is 42"));
        assert!(sm.matches("result is 7"));
    }

    #[test]
    fn rust_plain_raw_string_regex() {
        // The hash-less `regex = r"..."` form used across the rhino-cli
        // cucumber step definitions.
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "x.rs",
            "#[given(regex = r\"^a file recording (\\d+)% coverage$\")]\n",
        );
        let mut sm = StepMatcher::new();
        extract_rust_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("a file recording 80% coverage"));
        assert!(!sm.matches("a file recording abc% coverage"));
    }

    #[test]
    fn python_step_extraction_with_parsers_parse() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "steps.py",
            "@given(parsers.parse(\"count is {n:d}\"))\ndef step(n):\n    pass\n",
        );
        let mut sm = StepMatcher::new();
        extract_python_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("count is 42"));
    }

    #[test]
    fn python_scenario_titles_extracted() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "test.py",
            "@scenario(\"foo.feature\", \"User logs in\")\ndef test_login():\n    pass\n",
        );
        let titles = extract_python_scenario_titles(&p).unwrap();
        assert!(titles.contains("User logs in"));
    }

    #[test]
    fn csharp_verbatim_string_step() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "Steps.cs",
            "[Given(@\"user says \"\"hello\"\"\")]\nvoid Step() {}\n",
        );
        let mut sm = StepMatcher::new();
        extract_csharp_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches(r#"user says "hello""#));
    }

    #[test]
    fn fsharp_inline_step() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "Steps.fs",
            "let [<Given>] ``user logs in`` () = ()\n",
        );
        let mut sm = StepMatcher::new();
        extract_fsharp_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("user logs in"));
    }

    #[test]
    fn dart_step_extraction() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "steps.dart",
            r#"s.given("user logs in", (Scenario s) async {});"#,
        );
        let mut sm = StepMatcher::new();
        extract_dart_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("user logs in"));
    }

    #[test]
    fn clojure_step_extraction() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "steps.clj",
            "(Given \"user logs in\" []\n  ...)\n",
        );
        let mut sm = StepMatcher::new();
        extract_clojure_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("user logs in"));
    }

    #[test]
    fn elixir_step_extraction() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "steps.ex",
            "defgiven ~r/^user logs in$/ do\nend\n",
        );
        let mut sm = StepMatcher::new();
        extract_elixir_step_texts(&p, &mut sm).unwrap();
        assert!(sm.matches("user logs in"));
    }
}
