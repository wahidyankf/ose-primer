//! Version readers, parsers, comparators, and the subprocess runner.
//!
//! The comparator return strings include the `≥` (U+2265) character, which is
//! emitted verbatim in the text/JSON/markdown output.

use std::path::Path;
use std::process::Command;
use std::time::Instant;

use anyhow::Error;

use super::tools::{ToolDef, build_tool_defs};
use super::types::{DoctorResult, Scope, ToolCheck, ToolStatus, is_minimal_tool};

// ===========================================================================
// CommandRunner
// ===========================================================================

/// Output of a command run: (stdout, stderr, found). `found` is false when the
/// binary is not in PATH (Go returns an error in that case).
pub struct RunOutput {
    pub stdout: String,
    pub stderr: String,
    pub found: bool,
}

/// Injectable command runner. Production uses [`real_runner`]; tests inject
/// fakes. The exit code is ignored downstream — only "found in PATH or not"
/// matters.
pub type CommandRunner<'a> = dyn Fn(&str, &[&str]) -> RunOutput + 'a;

/// Executes a command via `std::process::Command`, returning captured output.
/// Returns `found: false` when the binary cannot be located. A non-zero exit is
/// not treated as "not found" — the output is still captured.
pub fn real_runner(name: &str, args: &[&str]) -> RunOutput {
    let output = Command::new(name).args(args).output();
    match output {
        Ok(out) => RunOutput {
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
            found: true,
        },
        Err(_) => RunOutput {
            stdout: String::new(),
            stderr: String::new(),
            found: false,
        },
    }
}

// ===========================================================================
// Version reader helpers
// ===========================================================================

/// Reads the required Node.js version from package.json (`volta.node`).
pub fn read_node_version(package_json_path: &Path) -> Option<String> {
    read_volta_field(package_json_path, "node")
}

/// Reads the required npm version from package.json (`volta.npm`).
pub fn read_npm_version(package_json_path: &Path) -> Option<String> {
    read_volta_field(package_json_path, "npm")
}

fn read_volta_field(package_json_path: &Path, field: &str) -> Option<String> {
    let data = std::fs::read_to_string(package_json_path).ok()?;
    let v: serde_json::Value = serde_json::from_str(&data).ok()?;
    v.get("volta")?
        .get(field)?
        .as_str()
        .map(std::string::ToString::to_string)
}

/// Reads the required Java version from pom.xml (`<java.version>`).
pub fn read_java_version(pom_xml_path: &Path) -> Option<String> {
    // The Go reader uses encoding/xml against `<project><properties><java.version>`.
    // A targeted scan of the properties block reproduces the value without a
    // full XML model.
    let data = std::fs::read_to_string(pom_xml_path).ok()?;
    let open = "<java.version>";
    let close = "</java.version>";
    let start = data.find(open)? + open.len();
    let rest = &data[start..];
    let end = rest.find(close)?;
    Some(rest[..end].trim().to_string())
}

/// Reads the required Go version from the `go X.Y` directive in a `go.work`
/// (or `go.mod`) file.
pub fn read_go_version(go_version_path: &Path) -> Option<String> {
    let data = std::fs::read_to_string(go_version_path).ok()?;
    for line in data.split('\n') {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("go ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                return Some(parts[1].to_string());
            }
            let _ = rest;
        }
    }
    None
}

/// Reads the required Python version from a `.python-version` file.
pub fn read_python_version(path: &Path) -> Option<String> {
    let data = std::fs::read_to_string(path).ok()?;
    Some(data.trim().to_string())
}

/// Reads a tool version from a `.tool-versions` file.
pub fn read_tool_versions_entry(path: &Path, tool_name: &str) -> Option<String> {
    let data = std::fs::read_to_string(path).ok()?;
    for line in data.split('\n') {
        let line = line.trim();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == tool_name {
            return Some(parts[1].to_string());
        }
    }
    None
}

/// Reads the required .NET SDK version from global.json (`sdk.version`).
pub fn read_dotnet_version(global_json_path: &Path) -> Option<String> {
    let data = std::fs::read_to_string(global_json_path).ok()?;
    let v: serde_json::Value = serde_json::from_str(&data).ok()?;
    v.get("sdk")?
        .get("version")?
        .as_str()
        .map(std::string::ToString::to_string)
}

/// Reads the Dart SDK version from pubspec.yaml `environment.sdk`, stripping
/// `^`/`>=` prefixes.
pub fn read_dart_sdk_version(pubspec_path: &Path) -> Option<String> {
    read_pubspec_env_field(pubspec_path, "sdk:")
}

/// Reads the Flutter constraint from pubspec.yaml `environment.flutter`.
pub fn read_flutter_version(pubspec_path: &Path) -> Option<String> {
    read_pubspec_env_field(pubspec_path, "flutter:")
}

fn read_pubspec_env_field(pubspec_path: &Path, key: &str) -> Option<String> {
    let data = std::fs::read_to_string(pubspec_path).ok()?;
    let mut in_env = false;
    for line in data.split('\n') {
        let trimmed = line.trim();
        if trimmed == "environment:" {
            in_env = true;
            continue;
        }
        if in_env {
            if !line.starts_with(' ') && !line.starts_with('\t') && !trimmed.is_empty() {
                break; // left the environment block
            }
            if let Some(rest) = trimmed.strip_prefix(key) {
                let mut ver = rest.trim();
                ver = ver.trim_matches('"');
                ver = ver.strip_prefix('^').unwrap_or(ver);
                ver = ver.strip_prefix(">=").unwrap_or(ver);
                return Some(ver.trim().to_string());
            }
        }
    }
    None
}

/// Reads the MSRV from Cargo.toml's `rust-version` field.
pub fn read_rust_version(cargo_toml_path: &Path) -> Option<String> {
    let data = std::fs::read_to_string(cargo_toml_path).ok()?;
    for line in data.split('\n') {
        let trimmed = line.trim();
        if trimmed.starts_with("rust-version")
            && let Some((_, rhs)) = trimmed.split_once('=')
        {
            let ver = rhs.trim().trim_matches('"');
            return Some(ver.to_string());
        }
    }
    Some(String::new())
}

// ===========================================================================
// Normalisation + line parsing
// ===========================================================================

/// Strips a leading `v` from a version string.
pub fn normalize_simple_version(s: &str) -> String {
    s.strip_prefix('v').unwrap_or(s).to_string()
}

/// Trims whitespace then strips a leading `v`.
pub fn parse_trim_version(s: &str) -> String {
    normalize_simple_version(s.trim())
}

/// Returns the `word_idx`-th whitespace token from the first line starting with
/// `line_prefix`, optionally stripping `token_prefix`.
pub fn parse_line_word(
    output: &str,
    line_prefix: &str,
    word_idx: usize,
    token_prefix: &str,
) -> String {
    for line in output.split('\n') {
        let trimmed = line.trim();
        if trimmed.starts_with(line_prefix) {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if word_idx < parts.len() {
                let tok = parts[word_idx];
                return tok.strip_prefix(token_prefix).unwrap_or(tok).to_string();
            }
        }
    }
    String::new()
}

// ===========================================================================
// Per-tool parsers
// ===========================================================================

/// Extracts the Java major version from `java -version` stderr.
pub fn parse_java_version(stderr: &str) -> String {
    for line in stderr.split('\n') {
        if line.contains("version") {
            let start = line.find('"');
            let end = line.rfind('"');
            if let (Some(s), Some(e)) = (start, end)
                && s != e
            {
                let version = &line[s + 1..e];
                let parts: Vec<&str> = version.split('.').collect();
                if !parts.is_empty() && !parts[0].is_empty() {
                    if parts[0] == "1" && parts.len() > 1 {
                        return parts[1].to_string();
                    }
                    return parts[0].to_string();
                }
            }
        }
    }
    String::new()
}

pub fn parse_python_version(output: &str) -> String {
    parse_line_word(output, "Python ", 1, "")
}

pub fn parse_rust_version(output: &str) -> String {
    parse_line_word(output, "rustc ", 1, "")
}

pub fn parse_cargo_llvm_cov(output: &str) -> String {
    parse_line_word(output, "cargo-llvm-cov ", 1, "")
}

pub fn parse_elixir_version(output: &str) -> String {
    parse_line_word(output, "Elixir ", 1, "")
}

pub fn parse_erlang_version(output: &str) -> String {
    output.trim().to_string()
}

pub fn parse_dotnet_version(output: &str) -> String {
    output.trim().to_string()
}

pub fn parse_clojure_version(output: &str) -> String {
    parse_line_word(output, "Clojure CLI version ", 3, "")
}

pub fn parse_dart_version(output: &str) -> String {
    for line in output.split('\n') {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("Dart SDK version:") {
            let ver = rest.trim();
            if let Some(first) = ver.split_whitespace().next() {
                return first.to_string();
            }
        }
    }
    String::new()
}

pub fn parse_flutter_version(output: &str) -> String {
    parse_line_word(output, "Flutter ", 1, "")
}

pub fn parse_docker_version(output: &str) -> String {
    for line in output.split('\n') {
        let trimmed = line.trim();
        if trimmed.starts_with("Docker version") {
            let fields: Vec<&str> = trimmed.split_whitespace().collect();
            if fields.len() >= 3 {
                return fields[2].strip_suffix(',').unwrap_or(fields[2]).to_string();
            }
        }
    }
    String::new()
}

pub fn parse_jq_version(output: &str) -> String {
    let trimmed = output.trim();
    trimmed.strip_prefix("jq-").unwrap_or(trimmed).to_string()
}

pub fn parse_playwright_version(output: &str) -> String {
    parse_line_word(output, "Version ", 1, "")
}

// ===========================================================================
// Version comparators
// ===========================================================================

/// Compares versions exactly (after normalisation).
pub fn compare_exact(installed: &str, required: &str) -> (ToolStatus, String) {
    if required.is_empty() {
        return (ToolStatus::Ok, "no version requirement".to_string());
    }
    let inst = normalize_simple_version(installed);
    let req = normalize_simple_version(required);
    if inst == req {
        (ToolStatus::Ok, format!("required: {required}"))
    } else {
        (
            ToolStatus::Warning,
            format!("required: {required}, version mismatch"),
        )
    }
}

/// Compares only the major component.
pub fn compare_major(installed: &str, required: &str) -> (ToolStatus, String) {
    if required.is_empty() {
        return (ToolStatus::Ok, "no version requirement".to_string());
    }
    let inst = normalize_simple_version(installed);
    let req = normalize_simple_version(required);
    let inst_major = inst.split('.').next().unwrap_or("");
    let req_major = req.split('.').next().unwrap_or("");
    if !inst_major.is_empty() && inst_major == req_major {
        (ToolStatus::Ok, format!("required: {required}"))
    } else {
        (
            ToolStatus::Warning,
            format!("required: {required}, version mismatch"),
        )
    }
}

/// Splits a version into (major, minor, patch). Returns None if any present
/// part is non-numeric.
fn parse_version_parts(s: &str) -> Option<(i64, i64, i64)> {
    let s = normalize_simple_version(s);
    let parts: Vec<&str> = s.splitn(3, '.').collect();
    let mut nums = [0_i64; 3];
    for (i, p) in parts.iter().enumerate() {
        nums[i] = p.parse::<i64>().ok()?;
    }
    Some((nums[0], nums[1], nums[2]))
}

/// Checks installed major >= required major.
pub fn compare_major_gte(installed: &str, required: &str) -> (ToolStatus, String) {
    if required.is_empty() {
        return (ToolStatus::Ok, "no version requirement".to_string());
    }
    let inst = normalize_simple_version(installed);
    let req = normalize_simple_version(required);
    let inst_major = inst.split('.').next().unwrap_or("");
    let req_major = req.split('.').next().unwrap_or("");
    match (inst_major.parse::<i64>(), req_major.parse::<i64>()) {
        (Ok(i_maj), Ok(r_maj)) => {
            if i_maj >= r_maj {
                (
                    ToolStatus::Ok,
                    format!("required: \u{2265}{required} (major)"),
                )
            } else {
                (
                    ToolStatus::Warning,
                    format!("required: \u{2265}{required} (major), version too old"),
                )
            }
        }
        _ => compare_exact(installed, required),
    }
}

/// Checks installed >= required.
pub fn compare_gte(installed: &str, required: &str) -> (ToolStatus, String) {
    if required.is_empty() {
        return (ToolStatus::Ok, "no version requirement".to_string());
    }
    let inst = parse_version_parts(installed);
    let req = parse_version_parts(required);
    match (inst, req) {
        (Some((i_maj, i_min, i_pat)), Some((r_maj, r_min, r_pat))) => {
            if i_maj > r_maj
                || (i_maj == r_maj && i_min > r_min)
                || (i_maj == r_maj && i_min == r_min && i_pat >= r_pat)
            {
                (ToolStatus::Ok, format!("required: \u{2265}{required}"))
            } else {
                (
                    ToolStatus::Warning,
                    format!("required: \u{2265}{required}, version too old"),
                )
            }
        }
        _ => compare_exact(installed, required),
    }
}

/// Playwright comparator: warns when browsers are missing. The browser-presence
/// probe is injectable so tests stay deterministic.
pub fn compare_playwright(
    _installed: &str,
    _required: &str,
    browsers_present: bool,
) -> (ToolStatus, String) {
    if browsers_present {
        (ToolStatus::Ok, "no version requirement".to_string())
    } else {
        (
            ToolStatus::Warning,
            "browsers not installed \u{2014} run: npx playwright install".to_string(),
        )
    }
}

/// Checks whether Playwright chromium browsers exist in the platform cache.
pub fn check_playwright_browsers() -> bool {
    let Ok(home) = std::env::var("HOME") else {
        return false;
    };
    let cache_dir = if cfg!(target_os = "macos") {
        Path::new(&home)
            .join("Library")
            .join("Caches")
            .join("ms-playwright")
    } else {
        Path::new(&home).join(".cache").join("ms-playwright")
    };
    let Ok(entries) = std::fs::read_dir(&cache_dir) else {
        return false;
    };
    for entry in entries.flatten() {
        if entry.file_name().to_string_lossy().starts_with("chromium-") {
            return true;
        }
    }
    false
}

// ===========================================================================
// Check execution
// ===========================================================================

/// Executes a single tool-check definition.
fn run_one_def(runner: &CommandRunner, def: &ToolDef, browsers_present: bool) -> ToolCheck {
    let required_version = (def.read_req)();
    let mut check = ToolCheck {
        name: def.name.to_string(),
        binary: def.binary.to_string(),
        status: ToolStatus::Missing,
        installed_version: String::new(),
        required_version: required_version.clone(),
        source: def.source.to_string(),
        note: String::new(),
    };

    let out = runner(def.binary, &def.args);
    if !out.found {
        check.status = ToolStatus::Missing;
        check.note = "not found in PATH".to_string();
        return check;
    }

    let output = if def.use_stderr {
        &out.stderr
    } else {
        &out.stdout
    };
    check.installed_version = (def.parse_ver)(output);
    let (status, note) = (def.compare)(
        &check.installed_version,
        &required_version,
        browsers_present,
    );
    check.status = status;
    check.note = note;
    check
}

/// Runs all tool checks (filtered by scope) and aggregates results.
/// `browsers_present` is the injectable Playwright probe; production passes
/// [`check_playwright_browsers`]`()`.
pub fn check_all_with(
    repo_root: &Path,
    scope: Scope,
    scope_raw: &str,
    runner: &CommandRunner,
    browsers_present: bool,
) -> Result<DoctorResult, Error> {
    let start = Instant::now();

    let mut defs = build_tool_defs(repo_root);
    if scope == Scope::Minimal {
        defs.retain(|d| is_minimal_tool(d.name));
    }

    let mut checks: Vec<ToolCheck> = Vec::with_capacity(defs.len());
    for def in &defs {
        checks.push(run_one_def(runner, def, browsers_present));
    }

    let mut result = DoctorResult {
        checks,
        ok_count: 0,
        warn_count: 0,
        missing_count: 0,
        duration_ms: i64::try_from(start.elapsed().as_millis()).unwrap_or(i64::MAX),
        scope_raw: scope_raw.to_string(),
    };

    for c in &result.checks {
        match c.status {
            ToolStatus::Ok => result.ok_count += 1,
            ToolStatus::Warning => result.warn_count += 1,
            ToolStatus::Missing => result.missing_count += 1,
        }
    }

    Ok(result)
}

/// Production entry point: real subprocess runner + real Playwright probe.
pub fn check_all(repo_root: &Path, scope: Scope, scope_raw: &str) -> Result<DoctorResult, Error> {
    let browsers = check_playwright_browsers();
    check_all_with(repo_root, scope, scope_raw, &real_runner, browsers)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn normalize_and_trim() {
        assert_eq!(normalize_simple_version("v1.2.3"), "1.2.3");
        assert_eq!(normalize_simple_version("1.2.3"), "1.2.3");
        assert_eq!(parse_trim_version("  v24.11.1\n"), "24.11.1");
    }

    #[test]
    fn parse_line_word_extracts_token() {
        assert_eq!(
            parse_line_word("go version go1.24.2 linux/amd64", "go version ", 2, "go"),
            "1.24.2"
        );
        assert_eq!(
            parse_line_word("git version 2.43.0", "git version ", 2, ""),
            "2.43.0"
        );
        assert_eq!(parse_line_word("nope", "Python ", 1, ""), "");
    }

    #[test]
    fn parse_java_handles_old_and_new() {
        assert_eq!(parse_java_version("openjdk version \"21.0.1\" 2024"), "21");
        assert_eq!(parse_java_version("java version \"1.8.0_292\""), "8");
        assert_eq!(parse_java_version("garbage"), "");
    }

    #[test]
    fn tool_specific_parsers() {
        assert_eq!(parse_python_version("Python 3.13.1"), "3.13.1");
        assert_eq!(parse_rust_version("rustc 1.90.0 (abc)"), "1.90.0");
        assert_eq!(parse_cargo_llvm_cov("cargo-llvm-cov 0.6.0"), "0.6.0");
        assert_eq!(
            parse_elixir_version("Erlang/OTP 27\n\nElixir 1.19.5 (x)"),
            "1.19.5"
        );
        assert_eq!(parse_erlang_version("27"), "27");
        assert_eq!(parse_dotnet_version("10.0.103\n"), "10.0.103");
        assert_eq!(
            parse_clojure_version("Clojure CLI version 1.12.0.1488"),
            "1.12.0.1488"
        );
        assert_eq!(
            parse_dart_version("Dart SDK version: 3.11.1 (stable)"),
            "3.11.1"
        );
        assert_eq!(parse_flutter_version("Flutter 3.41.0 • channel"), "3.41.0");
        assert_eq!(
            parse_docker_version("Docker version 29.0.0, build abc"),
            "29.0.0"
        );
        assert_eq!(parse_jq_version("jq-1.7.1"), "1.7.1");
        assert_eq!(parse_playwright_version("Version 1.58.0"), "1.58.0");
    }

    #[test]
    fn compare_exact_matches_and_mismatches() {
        assert_eq!(compare_exact("1.2.3", "").0, ToolStatus::Ok);
        assert_eq!(
            compare_exact("v1.2.3", "1.2.3"),
            (ToolStatus::Ok, "required: 1.2.3".to_string())
        );
        assert_eq!(
            compare_exact("1.0.0", "2.0.0"),
            (
                ToolStatus::Warning,
                "required: 2.0.0, version mismatch".to_string()
            )
        );
    }

    #[test]
    fn compare_major_only() {
        assert_eq!(compare_major("21.0.1", "21").0, ToolStatus::Ok);
        assert_eq!(compare_major("17.0.1", "21").0, ToolStatus::Warning);
        assert_eq!(compare_major("x", "").0, ToolStatus::Ok);
    }

    #[test]
    fn compare_gte_orders() {
        assert_eq!(compare_gte("1.24.2", "1.24.0").0, ToolStatus::Ok);
        assert_eq!(compare_gte("1.24.0", "1.24.2").0, ToolStatus::Warning);
        assert!(compare_gte("1.25.0", "1.24.9").0 == ToolStatus::Ok);
        // Non-numeric falls back to exact.
        assert_eq!(compare_gte("abc", "1.0.0").0, ToolStatus::Warning);
        assert_eq!(compare_gte("x", "").0, ToolStatus::Ok);
    }

    #[test]
    fn compare_major_gte_uses_unicode_geq() {
        let (status, note) = compare_major_gte("27", "27.0");
        assert_eq!(status, ToolStatus::Ok);
        assert!(note.contains('\u{2265}'));
        assert_eq!(compare_major_gte("26", "27").0, ToolStatus::Warning);
        assert_eq!(compare_major_gte("x", "").0, ToolStatus::Ok);
    }

    #[test]
    fn playwright_comparator_branches() {
        assert_eq!(compare_playwright("", "", true).0, ToolStatus::Ok);
        let (s, note) = compare_playwright("", "", false);
        assert_eq!(s, ToolStatus::Warning);
        assert!(note.contains("npx playwright install"));
    }

    #[test]
    fn version_readers_parse_config_files() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(
            root.join("package.json"),
            "{\"volta\":{\"node\":\"24.11.1\",\"npm\":\"11.0.0\"}}",
        )
        .unwrap();
        std::fs::write(
            root.join("pom.xml"),
            "<project><properties><java.version>21</java.version></properties></project>",
        )
        .unwrap();
        std::fs::write(root.join("go.mod"), "module x\n\ngo 1.24.0\n").unwrap();
        std::fs::write(root.join(".python-version"), "3.13\n").unwrap();
        std::fs::write(
            root.join(".tool-versions"),
            "erlang 27.0\nelixir 1.19.0-otp-27\n",
        )
        .unwrap();
        std::fs::write(
            root.join("global.json"),
            "{\"sdk\":{\"version\":\"10.0.100\"}}",
        )
        .unwrap();
        std::fs::write(
            root.join("pubspec.yaml"),
            "name: d\nenvironment:\n  sdk: ^3.11.0\n  flutter: \">=3.41.0\"\n",
        )
        .unwrap();
        std::fs::write(
            root.join("Cargo.toml"),
            "[package]\nrust-version = \"1.80\"\n",
        )
        .unwrap();

        assert_eq!(
            read_node_version(&root.join("package.json")).unwrap(),
            "24.11.1"
        );
        assert_eq!(
            read_npm_version(&root.join("package.json")).unwrap(),
            "11.0.0"
        );
        assert_eq!(read_java_version(&root.join("pom.xml")).unwrap(), "21");
        assert_eq!(read_go_version(&root.join("go.mod")).unwrap(), "1.24.0");
        assert_eq!(
            read_python_version(&root.join(".python-version")).unwrap(),
            "3.13"
        );
        assert_eq!(
            read_tool_versions_entry(&root.join(".tool-versions"), "erlang").unwrap(),
            "27.0"
        );
        assert_eq!(
            read_dotnet_version(&root.join("global.json")).unwrap(),
            "10.0.100"
        );
        assert_eq!(
            read_dart_sdk_version(&root.join("pubspec.yaml")).unwrap(),
            "3.11.0"
        );
        assert_eq!(
            read_flutter_version(&root.join("pubspec.yaml")).unwrap(),
            "3.41.0"
        );
        assert_eq!(read_rust_version(&root.join("Cargo.toml")).unwrap(), "1.80");
    }

    #[test]
    fn rust_version_missing_field_returns_empty() {
        let tmp = tempfile::tempdir().unwrap();
        let p = tmp.path().join("Cargo.toml");
        std::fs::write(&p, "[package]\nname = \"x\"\n").unwrap();
        assert_eq!(read_rust_version(&p).unwrap(), "");
    }

    fn fake_runner(found: &'static [&'static str]) -> impl Fn(&str, &[&str]) -> RunOutput {
        move |name: &str, args: &[&str]| {
            if !found.contains(&name) {
                return RunOutput {
                    stdout: String::new(),
                    stderr: String::new(),
                    found: false,
                };
            }
            let (stdout, stderr) = match name {
                "git" => ("git version 2.43.0".to_string(), String::new()),
                "volta" => ("2.0.2".to_string(), String::new()),
                "node" => ("v24.11.1".to_string(), String::new()),
                "npm" => ("11.0.0".to_string(), String::new()),
                "java" => (String::new(), "openjdk version \"21.0.1\"".to_string()),
                "go" => ("go version go1.24.2 linux/amd64".to_string(), String::new()),
                "docker" => ("Docker version 29.0.0, build a".to_string(), String::new()),
                "jq" => ("jq-1.7.1".to_string(), String::new()),
                _ => (String::new(), String::new()),
            };
            let _ = args;
            RunOutput {
                stdout,
                stderr,
                found: true,
            }
        }
    }

    fn write_min_config(root: &Path) {
        std::fs::write(
            root.join("package.json"),
            "{\"volta\":{\"node\":\"24.11.1\",\"npm\":\"11.0.0\"}}",
        )
        .unwrap();
        std::fs::create_dir_all(root.join("apps/crud-be-fsharp-giraffe-jasb")).unwrap();
        std::fs::write(
            root.join("apps/crud-be-fsharp-giraffe-jasb/pom.xml"),
            "<project><properties><java.version>21</java.version></properties></project>",
        )
        .unwrap();
        std::fs::write(root.join("go.work"), "go 1.24.0\n\nuse (\n\t./x\n)\n").unwrap();
    }

    #[test]
    fn check_all_with_minimal_scope_filters_and_counts() {
        let tmp = tempfile::tempdir().unwrap();
        write_min_config(tmp.path());
        let runner = fake_runner(&["git", "volta", "node", "npm", "go", "docker", "jq"]);
        let result = check_all_with(tmp.path(), Scope::Minimal, "minimal", &runner, true).unwrap();
        // Minimal set: git, volta, node, npm, golang, docker, jq.
        assert_eq!(result.checks.len(), 7);
        assert_eq!(result.missing_count, 0);
        assert_eq!(result.ok_count, 7);
        let names: Vec<&str> = result.checks.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["git", "volta", "node", "npm", "golang", "docker", "jq"]
        );
    }

    #[test]
    fn check_all_with_reports_missing_tools() {
        let tmp = tempfile::tempdir().unwrap();
        write_min_config(tmp.path());
        // Empty found-set → all 19 missing.
        let runner = fake_runner(&[]);
        let result = check_all_with(tmp.path(), Scope::Full, "full", &runner, false).unwrap();
        assert_eq!(result.checks.len(), 19);
        assert_eq!(result.missing_count, 19);
        assert!(
            result
                .checks
                .iter()
                .all(|c| c.status == ToolStatus::Missing)
        );
        assert!(result.checks.iter().all(|c| c.note == "not found in PATH"));
    }

    #[test]
    fn check_all_with_node_mismatch_warns() {
        let tmp = tempfile::tempdir().unwrap();
        // node requirement 1.0.0 but runner reports 24.11.1.
        std::fs::write(
            tmp.path().join("package.json"),
            "{\"volta\":{\"node\":\"1.0.0\",\"npm\":\"11.0.0\"}}",
        )
        .unwrap();
        let runner = fake_runner(&["node"]);
        let result = check_all_with(tmp.path(), Scope::Minimal, "minimal", &runner, true).unwrap();
        let node = result.checks.iter().find(|c| c.name == "node").unwrap();
        assert_eq!(node.status, ToolStatus::Warning);
    }
}
