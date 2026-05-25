//! `validate-sync` orchestration.
//!
//! Byte-for-byte port of
//! `apps/rhino-cli-go/internal/agents/sync_validator.go`. Confirms `.claude/`
//! and `.opencode/agents/` are semantically equivalent and that no stale
//! singular agent dir or mirrored skill copies exist.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::time::Instant;

use anyhow::Error;

use super::converter::{OPENCODE_AGENT_DIR, convert_model, convert_tools};
use super::frontmatter::{YamlValue, extract_frontmatter, parse_claude_tools, parse_yaml_value};
use super::types::ValidationCheck;
use super::types::ValidationResult;

/// Validates that `.claude/` and `.opencode/` are in sync. Mirrors Go
/// `ValidateSync`.
pub fn validate_sync(repo_root: &std::path::Path) -> Result<ValidationResult, Error> {
    let start = Instant::now();
    let mut result = ValidationResult {
        checks: Vec::new(),
        ..Default::default()
    };

    // 0. No stale singular agent directory.
    tally(&mut result, validate_no_stale_agent_dir(repo_root));

    // 1. Agent count.
    tally(&mut result, validate_agent_count(repo_root));

    // 2. Agent equivalence (per Claude-side agent).
    for check in validate_agent_equivalence(repo_root) {
        tally(&mut result, check);
    }

    // 3. No synced skill mirror.
    tally(&mut result, validate_no_synced_skills(repo_root));

    result.duration = start.elapsed();
    Ok(result)
}

fn tally(result: &mut ValidationResult, check: ValidationCheck) {
    if check.status == "passed" {
        result.passed_checks += 1;
    } else {
        result.failed_checks += 1;
    }
    result.total_checks += 1;
    result.checks.push(check);
}

/// Asserts the legacy singular `.opencode/agent/` does not exist. Mirrors Go
/// `validateNoStaleAgentDir`.
fn validate_no_stale_agent_dir(repo_root: &std::path::Path) -> ValidationCheck {
    let stale = repo_root.join(".opencode").join("agent");
    match std::fs::symlink_metadata(&stale) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => ValidationCheck::passed(
            "No Stale Agent Directory",
            "Legacy singular .opencode/agent/ does not exist",
        ),
        Err(e) => ValidationCheck {
            name: "No Stale Agent Directory".to_string(),
            status: "failed".to_string(),
            expected: String::new(),
            actual: String::new(),
            message: format!("Failed to stat .opencode/agent/: {e}"),
        },
        Ok(meta) => {
            if meta.is_dir() {
                ValidationCheck {
                    name: "No Stale Agent Directory".to_string(),
                    status: "failed".to_string(),
                    expected: ".opencode/agent/ does not exist".to_string(),
                    actual: ".opencode/agent/ exists as a directory".to_string(),
                    message: "Stale singular .opencode/agent/ reappeared; canonical OpenCode path is .opencode/agents/ (plural). Remove the stale directory.".to_string(),
                }
            } else {
                ValidationCheck {
                    name: "No Stale Agent Directory".to_string(),
                    status: "failed".to_string(),
                    expected: ".opencode/agent/ does not exist".to_string(),
                    actual: ".opencode/agent/ exists".to_string(),
                    message: "Stale .opencode/agent/ entry reappeared; canonical OpenCode path is .opencode/agents/ (plural). Remove the stale entry.".to_string(),
                }
            }
        }
    }
}

/// Checks that the OpenCode dir contains at least as many agents as Claude.
/// Mirrors Go `validateAgentCount`.
fn validate_agent_count(repo_root: &std::path::Path) -> ValidationCheck {
    let claude_dir = repo_root.join(".claude").join("agents");
    let opencode_dir = repo_root.join(OPENCODE_AGENT_DIR);

    let claude_count = count_markdown_files(&claude_dir);
    let opencode_count = count_markdown_files(&opencode_dir);

    if opencode_count >= claude_count {
        ValidationCheck {
            name: "Agent Count".to_string(),
            status: "passed".to_string(),
            expected: format!(">= {claude_count} agents"),
            actual: format!("{opencode_count} agents"),
            message: "OpenCode agents directory contains every Claude agent".to_string(),
        }
    } else {
        ValidationCheck {
            name: "Agent Count".to_string(),
            status: "failed".to_string(),
            expected: format!(">= {claude_count} agents"),
            actual: format!("{opencode_count} agents"),
            message: "OpenCode agents directory missing one or more Claude agents".to_string(),
        }
    }
}

/// Per-agent semantic equivalence checks for the Claude-side set. Mirrors Go
/// `validateAgentEquivalence`.
fn validate_agent_equivalence(repo_root: &std::path::Path) -> Vec<ValidationCheck> {
    let claude_dir = repo_root.join(".claude").join("agents");
    let opencode_dir = repo_root.join(OPENCODE_AGENT_DIR);

    let entries = match std::fs::read_dir(&claude_dir) {
        Ok(e) => e,
        Err(e) => {
            return vec![ValidationCheck {
                name: "Agent Equivalence".to_string(),
                status: "failed".to_string(),
                expected: String::new(),
                actual: String::new(),
                message: format!("Failed to read Claude agents directory: {e}"),
            }];
        }
    };

    let mut names: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let is_dir = entry.file_type().is_ok_and(|t| t.is_dir());
        if is_dir || !name.ends_with(".md") || name == "README.md" {
            continue;
        }
        names.push(name);
    }
    names.sort();

    let mut checks = Vec::new();
    for name in names {
        let claude_path = claude_dir.join(&name);
        let opencode_path = opencode_dir.join(&name);
        checks.push(validate_agent_file(&name, &claude_path, &opencode_path));
    }
    checks
}

/// Checks a single agent file pair for semantic equivalence. Mirrors Go
/// `validateAgentFile`.
fn validate_agent_file(
    name: &str,
    claude_path: &std::path::Path,
    opencode_path: &std::path::Path,
) -> ValidationCheck {
    let check_name = format!("Agent: {name}");

    let claude_content = match std::fs::read(claude_path) {
        Ok(c) => c,
        Err(e) => {
            return fail(
                &check_name,
                "",
                "",
                &format!("Failed to read Claude agent: {e}"),
            );
        }
    };
    let opencode_content = match std::fs::read(opencode_path) {
        Ok(c) => c,
        Err(e) => {
            return fail(
                &check_name,
                "",
                "",
                &format!("Failed to read OpenCode agent: {e}"),
            );
        }
    };

    let (claude_data, claude_body, opencode_agent, opencode_body) =
        match parse_agent_pair(&claude_content, &opencode_content) {
            Ok(t) => t,
            Err(msg) => return fail(&check_name, "", "", &msg),
        };

    compare_agent_fields(
        check_name,
        &claude_data,
        &claude_body,
        &opencode_agent,
        &opencode_body,
    )
}

/// Parses both sides of an agent pair and returns the four values needed for
/// comparison, or an error message string on the first parse failure.
fn parse_agent_pair(
    claude_content: &[u8],
    opencode_content: &[u8],
) -> Result<(YamlValue, Vec<u8>, ParsedOpenCode, Vec<u8>), String> {
    let (claude_front, claude_body) = extract_frontmatter(claude_content)
        .map_err(|e| format!("Failed to parse Claude frontmatter: {e}"))?;
    let (opencode_front, opencode_body) = extract_frontmatter(opencode_content)
        .map_err(|e| format!("Failed to parse OpenCode frontmatter: {e}"))?;
    let claude_data =
        parse_yaml_value(&claude_front).map_err(|e| format!("Failed to parse Claude YAML: {e}"))?;
    let opencode_agent = parse_opencode_agent(&opencode_front)
        .map_err(|e| format!("Failed to parse OpenCode YAML: {e}"))?;
    Ok((claude_data, claude_body, opencode_agent, opencode_body))
}

/// Compares the parsed fields of a Claude/OpenCode agent pair and returns the
/// result check.
fn compare_agent_fields(
    check_name: String,
    claude_data: &YamlValue,
    claude_body: &[u8],
    opencode_agent: &ParsedOpenCode,
    opencode_body: &[u8],
) -> ValidationCheck {
    // Description.
    let claude_desc = map_string(claude_data, "description");
    if claude_desc != opencode_agent.description {
        return ValidationCheck {
            name: check_name,
            status: "failed".to_string(),
            expected: "Matching descriptions".to_string(),
            actual: "Descriptions differ".to_string(),
            message: "Description mismatch".to_string(),
        };
    }

    // Model.
    let claude_model = map_string(claude_data, "model");
    let expected_model = convert_model(&claude_model);
    if expected_model != opencode_agent.model {
        return ValidationCheck {
            name: check_name,
            status: "failed".to_string(),
            expected: format!("Model: {expected_model}"),
            actual: format!("Model: {}", opencode_agent.model),
            message: "Model mismatch".to_string(),
        };
    }

    // Tools.
    let claude_tools = match map_value(claude_data, "tools") {
        Some(v) => parse_claude_tools(v),
        None => Vec::new(),
    };
    let expected_tools = convert_tools(&claude_tools);
    if !tools_match(&expected_tools, &opencode_agent.tools) {
        return ValidationCheck {
            name: check_name,
            status: "failed".to_string(),
            expected: format!("Tools: {}", sorted_keys(&expected_tools)),
            actual: format!("Tools: {}", sorted_keys(&opencode_agent.tools)),
            message: "Tools mismatch".to_string(),
        };
    }

    // Skills.
    let claude_skills = map_skills(claude_data);
    if !skills_match(&claude_skills, &opencode_agent.skills) {
        return ValidationCheck {
            name: check_name,
            status: "failed".to_string(),
            expected: format!("Skills: {}", go_string_slice(&claude_skills)),
            actual: format!("Skills: {}", go_string_slice(&opencode_agent.skills)),
            message: "Skills mismatch".to_string(),
        };
    }

    // Body.
    if claude_body != opencode_body {
        return ValidationCheck {
            name: check_name,
            status: "failed".to_string(),
            expected: "Matching body content".to_string(),
            actual: "Body content differs".to_string(),
            message: "Body mismatch".to_string(),
        };
    }

    ValidationCheck::passed(check_name, "Agent is semantically equivalent")
}

/// Parsed OpenCode agent view used for equivalence comparison.
struct ParsedOpenCode {
    description: String,
    model: String,
    tools: BTreeMap<String, bool>,
    skills: Vec<String>,
}

/// Parses OpenCode frontmatter into a comparison view. Mirrors yaml.v3
/// unmarshal into `OpenCodeAgent`.
fn parse_opencode_agent(frontmatter: &[u8]) -> Result<ParsedOpenCode, Error> {
    let value = parse_yaml_value(frontmatter)?;
    let mut out = ParsedOpenCode {
        description: String::new(),
        model: String::new(),
        tools: BTreeMap::new(),
        skills: Vec::new(),
    };
    if let YamlValue::Mapping(pairs) = &value {
        for (k, v) in pairs {
            match k.as_str() {
                "description" => {
                    if let YamlValue::String(s) = v {
                        out.description.clone_from(s);
                    }
                }
                "model" => {
                    if let YamlValue::String(s) = v {
                        out.model.clone_from(s);
                    }
                }
                "tools" => {
                    if let YamlValue::Mapping(tm) = v {
                        for (tk, tv) in tm {
                            if let YamlValue::Bool(b) = tv {
                                out.tools.insert(tk.clone(), *b);
                            }
                        }
                    }
                }
                "skills" => {
                    if let YamlValue::Sequence(seq) = v {
                        for item in seq {
                            if let YamlValue::String(s) = item {
                                out.skills.push(s.clone());
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(out)
}

/// Asserts no rhino-cli-managed skill mirror exists. Mirrors Go
/// `validateNoSyncedSkills`.
fn validate_no_synced_skills(repo_root: &std::path::Path) -> ValidationCheck {
    let claude_dir = repo_root.join(".claude").join("skills");
    let mut claude_names: BTreeSet<String> = BTreeSet::new();
    if let Ok(entries) = std::fs::read_dir(&claude_dir) {
        for entry in entries.flatten() {
            if entry.file_type().is_ok_and(|t| t.is_dir()) {
                let skill_file = claude_dir.join(entry.file_name()).join("SKILL.md");
                if skill_file.exists() {
                    claude_names.insert(entry.file_name().to_string_lossy().into_owned());
                }
            }
        }
    }

    let mirror_dirs = [
        repo_root.join(".opencode").join("skill"),
        repo_root.join(".opencode").join("skills"),
    ];

    let mut offenders: Vec<String> = Vec::new();
    for dir in &mirror_dirs {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if entry.file_type().is_ok_and(|t| t.is_dir()) && claude_names.contains(&name) {
                let skill_file = dir.join(&name).join("SKILL.md");
                if skill_file.exists() {
                    offenders.push(dir.join(&name).to_string_lossy().into_owned());
                }
            }
        }
    }

    if offenders.is_empty() {
        ValidationCheck::passed(
            "No Synced Skill Mirror",
            "No rhino-cli-managed skill copies under .opencode/skill or .opencode/skills",
        )
    } else {
        ValidationCheck {
            name: "No Synced Skill Mirror".to_string(),
            status: "failed".to_string(),
            expected: "No skill copy mirroring .claude/skills/<name>".to_string(),
            actual: format!(
                "Found {} mirrored skill dir(s): {}",
                offenders.len(),
                go_string_slice(&offenders)
            ),
            message: "OpenCode reads .claude/skills/ natively; remove the mirror copies"
                .to_string(),
        }
    }
}

// --- helpers ---

fn count_markdown_files(dir: &std::path::Path) -> i64 {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut count = 0i64;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let is_dir = entry.file_type().is_ok_and(|t| t.is_dir());
        if !is_dir && name.ends_with(".md") && name != "README.md" {
            count += 1;
        }
    }
    count
}

fn tools_match(a: &BTreeMap<String, bool>, b: &BTreeMap<String, bool>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (k, v) in a {
        if b.get(k) != Some(v) {
            return false;
        }
    }
    true
}

fn skills_match(a: &[String], b: &[String]) -> bool {
    a == b
}

fn sorted_keys(m: &BTreeMap<String, bool>) -> String {
    // BTreeMap keys are already sorted; format as Go's `%v` on []string.
    let keys: Vec<&String> = m.keys().collect();
    format!(
        "[{}]",
        keys.iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn go_string_slice(items: &[String]) -> String {
    format!("[{}]", items.join(" "))
}

fn map_string(value: &YamlValue, key: &str) -> String {
    match map_value(value, key) {
        Some(YamlValue::String(s)) => s.clone(),
        _ => String::new(),
    }
}

fn map_value<'a>(value: &'a YamlValue, key: &str) -> Option<&'a YamlValue> {
    if let YamlValue::Mapping(pairs) = value {
        for (k, v) in pairs {
            if k == key {
                return Some(v);
            }
        }
    }
    None
}

fn map_skills(value: &YamlValue) -> Vec<String> {
    let mut skills = Vec::new();
    if let Some(YamlValue::Sequence(seq)) = map_value(value, "skills") {
        for item in seq {
            if let YamlValue::String(s) = item {
                skills.push(s.clone());
            }
        }
    }
    skills
}

fn fail(name: &str, expected: &str, actual: &str, message: &str) -> ValidationCheck {
    ValidationCheck {
        name: name.to_string(),
        status: "failed".to_string(),
        expected: expected.to_string(),
        actual: actual.to_string(),
        message: message.to_string(),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn synced_fixture(root: &std::path::Path) {
        let claude = root.join(".claude/agents");
        let opencode = root.join(".opencode/agents");
        std::fs::create_dir_all(&claude).unwrap();
        std::fs::create_dir_all(&opencode).unwrap();
        std::fs::write(
            claude.join("foo-maker.md"),
            "---\nname: foo-maker\ndescription: Makes foo.\ntools: Read, Write\nmodel:\ncolor: blue\n---\n# Body\n",
        )
        .unwrap();
        std::fs::write(
            opencode.join("foo-maker.md"),
            "---\ndescription: Makes foo.\nmodel: opencode-go/minimax-m2.7\ntools:\n  read: true\n  write: true\n---\n# Body\n",
        )
        .unwrap();
    }

    #[test]
    fn synced_tree_passes() {
        let dir = tempfile::tempdir().unwrap();
        synced_fixture(dir.path());
        let result = validate_sync(dir.path()).unwrap();
        assert_eq!(result.failed_checks, 0, "checks: {:?}", result.checks);
    }

    #[test]
    fn description_mismatch_fails() {
        let dir = tempfile::tempdir().unwrap();
        synced_fixture(dir.path());
        std::fs::write(
            dir.path().join(".opencode/agents/foo-maker.md"),
            "---\ndescription: DIFFERENT.\nmodel: opencode-go/minimax-m2.7\ntools:\n  read: true\n  write: true\n---\n# Body\n",
        )
        .unwrap();
        let result = validate_sync(dir.path()).unwrap();
        let c = result
            .checks
            .iter()
            .find(|c| c.name == "Agent: foo-maker.md")
            .unwrap();
        assert_eq!(c.status, "failed");
        assert_eq!(c.message, "Description mismatch");
    }

    #[test]
    fn count_mismatch_fails() {
        let dir = tempfile::tempdir().unwrap();
        synced_fixture(dir.path());
        // Add a second Claude agent with no OpenCode counterpart.
        std::fs::write(
            dir.path().join(".claude/agents/bar-maker.md"),
            "---\nname: bar-maker\ndescription: d\ntools: Read\nmodel:\ncolor: blue\n---\n# Body\n",
        )
        .unwrap();
        let result = validate_sync(dir.path()).unwrap();
        let c = result
            .checks
            .iter()
            .find(|c| c.name == "Agent Count")
            .unwrap();
        assert_eq!(c.status, "failed");
        assert_eq!(c.expected, ">= 2 agents");
        assert_eq!(c.actual, "1 agents");
    }

    #[test]
    fn stale_singular_dir_fails() {
        let dir = tempfile::tempdir().unwrap();
        synced_fixture(dir.path());
        std::fs::create_dir_all(dir.path().join(".opencode/agent")).unwrap();
        let result = validate_sync(dir.path()).unwrap();
        let c = result
            .checks
            .iter()
            .find(|c| c.name == "No Stale Agent Directory")
            .unwrap();
        assert_eq!(c.status, "failed");
        assert_eq!(c.actual, ".opencode/agent/ exists as a directory");
    }

    #[test]
    fn body_mismatch_fails() {
        let dir = tempfile::tempdir().unwrap();
        synced_fixture(dir.path());
        std::fs::write(
            dir.path().join(".opencode/agents/foo-maker.md"),
            "---\ndescription: Makes foo.\nmodel: opencode-go/minimax-m2.7\ntools:\n  read: true\n  write: true\n---\n# Different Body\n",
        )
        .unwrap();
        let result = validate_sync(dir.path()).unwrap();
        let c = result
            .checks
            .iter()
            .find(|c| c.name == "Agent: foo-maker.md")
            .unwrap();
        assert_eq!(c.message, "Body mismatch");
    }

    #[test]
    fn model_mismatch_fails() {
        let dir = tempfile::tempdir().unwrap();
        synced_fixture(dir.path());
        std::fs::write(
            dir.path().join(".opencode/agents/foo-maker.md"),
            "---\ndescription: Makes foo.\nmodel: opencode-go/glm-5\ntools:\n  read: true\n  write: true\n---\n# Body\n",
        )
        .unwrap();
        let result = validate_sync(dir.path()).unwrap();
        let c = result
            .checks
            .iter()
            .find(|c| c.name == "Agent: foo-maker.md")
            .unwrap();
        assert_eq!(c.message, "Model mismatch");
        assert_eq!(c.expected, "Model: opencode-go/minimax-m2.7");
        assert_eq!(c.actual, "Model: opencode-go/glm-5");
    }

    #[test]
    fn tools_mismatch_fails() {
        let dir = tempfile::tempdir().unwrap();
        synced_fixture(dir.path());
        std::fs::write(
            dir.path().join(".opencode/agents/foo-maker.md"),
            "---\ndescription: Makes foo.\nmodel: opencode-go/minimax-m2.7\ntools:\n  read: true\n---\n# Body\n",
        )
        .unwrap();
        let result = validate_sync(dir.path()).unwrap();
        let c = result
            .checks
            .iter()
            .find(|c| c.name == "Agent: foo-maker.md")
            .unwrap();
        assert_eq!(c.message, "Tools mismatch");
        assert_eq!(c.expected, "Tools: [read write]");
        assert_eq!(c.actual, "Tools: [read]");
    }

    #[test]
    fn skills_mismatch_fails() {
        let dir = tempfile::tempdir().unwrap();
        synced_fixture(dir.path());
        // Claude side declares a skill; opencode side omits it.
        std::fs::write(
            dir.path().join(".claude/agents/foo-maker.md"),
            "---\nname: foo-maker\ndescription: Makes foo.\ntools: Read, Write\nmodel:\ncolor: blue\nskills:\n  - some-skill\n---\n# Body\n",
        )
        .unwrap();
        let result = validate_sync(dir.path()).unwrap();
        let c = result
            .checks
            .iter()
            .find(|c| c.name == "Agent: foo-maker.md")
            .unwrap();
        assert_eq!(c.message, "Skills mismatch");
        assert_eq!(c.expected, "Skills: [some-skill]");
        assert_eq!(c.actual, "Skills: []");
    }

    #[test]
    fn missing_opencode_counterpart_read_error() {
        let dir = tempfile::tempdir().unwrap();
        synced_fixture(dir.path());
        // Remove the opencode file so the equivalence read fails.
        std::fs::remove_file(dir.path().join(".opencode/agents/foo-maker.md")).unwrap();
        let result = validate_sync(dir.path()).unwrap();
        let c = result
            .checks
            .iter()
            .find(|c| c.name == "Agent: foo-maker.md")
            .unwrap();
        assert_eq!(c.status, "failed");
        assert!(c.message.contains("Failed to read OpenCode agent"));
    }

    #[test]
    fn synced_skill_mirror_fails() {
        let dir = tempfile::tempdir().unwrap();
        synced_fixture(dir.path());
        // A .claude skill plus a .opencode mirror of the same name → offender.
        std::fs::create_dir_all(dir.path().join(".claude/skills/my-skill")).unwrap();
        std::fs::write(
            dir.path().join(".claude/skills/my-skill/SKILL.md"),
            "---\nname: my-skill\ndescription: d\n---\nx\n",
        )
        .unwrap();
        std::fs::create_dir_all(dir.path().join(".opencode/skills/my-skill")).unwrap();
        std::fs::write(
            dir.path().join(".opencode/skills/my-skill/SKILL.md"),
            "---\nname: my-skill\ndescription: d\n---\nx\n",
        )
        .unwrap();
        let result = validate_sync(dir.path()).unwrap();
        let c = result
            .checks
            .iter()
            .find(|c| c.name == "No Synced Skill Mirror")
            .unwrap();
        assert_eq!(c.status, "failed");
        assert!(c.actual.contains("Found 1 mirrored skill dir(s)"));
    }

    #[test]
    fn stale_singular_file_fails() {
        let dir = tempfile::tempdir().unwrap();
        synced_fixture(dir.path());
        // A FILE (not dir) at .opencode/agent triggers the non-dir branch.
        std::fs::write(dir.path().join(".opencode/agent"), "x").unwrap();
        let result = validate_sync(dir.path()).unwrap();
        let c = result
            .checks
            .iter()
            .find(|c| c.name == "No Stale Agent Directory")
            .unwrap();
        assert_eq!(c.status, "failed");
        assert_eq!(c.actual, ".opencode/agent/ exists");
    }
}
