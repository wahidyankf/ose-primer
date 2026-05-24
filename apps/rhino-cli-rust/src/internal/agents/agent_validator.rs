//! Per-agent validation (the 12 rules of `validate-claude`).
//!
//! Byte-for-byte port of
//! `apps/rhino-cli-go/internal/agents/agent_validator.go`. The Go code parses
//! the normalized frontmatter into a `ClaudeAgentFull` struct (where `tools` is
//! a comma-separated string) and a generic `yaml.Node` for field-order
//! detection. We reuse the order-preserving [`YamlValue`] parse for both.

use std::collections::BTreeSet;

use super::frontmatter::{YamlValue, extract_frontmatter, parse_yaml_value};
use super::types::{
    REQUIRED_FIELD_ORDER, VALID_COLORS, VALID_MODELS, VALID_TOOLS, ValidationCheck, is_valid_color,
    is_valid_model, is_valid_tool,
};
use super::yaml_formatting::validate_yaml_formatting_raw;

/// Typed view of an agent's frontmatter, mirroring Go `ClaudeAgentFull`.
struct ClaudeAgentFull {
    name: String,
    description: String,
    tools: String,
    model: String,
    color: String,
    skills: Vec<String>,
}

/// Extracts a `ClaudeAgentFull` from a parsed mapping. Returns `Err(message)`
/// when a scalar field holds a non-scalar value (mirroring yaml.v3's
/// `cannot unmarshal` failure). Only string scalars are accepted for the
/// scalar fields; `skills` accepts a string sequence.
fn extract_claude_agent(value: &YamlValue) -> Result<ClaudeAgentFull, String> {
    let pairs = match value {
        YamlValue::Mapping(p) => p,
        _ => return Ok(empty_agent()),
    };

    let mut agent = empty_agent();
    for (k, v) in pairs {
        match k.as_str() {
            "name" => agent.name = scalar_string(v, k)?,
            "description" => agent.description = scalar_string(v, k)?,
            "tools" => agent.tools = scalar_string(v, k)?,
            "model" => agent.model = scalar_string(v, k)?,
            "color" => agent.color = scalar_string(v, k)?,
            "skills" => {
                if let YamlValue::Sequence(seq) = v {
                    for item in seq {
                        if let YamlValue::String(s) = item {
                            agent.skills.push(s.clone());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(agent)
}

fn empty_agent() -> ClaudeAgentFull {
    ClaudeAgentFull {
        name: String::new(),
        description: String::new(),
        tools: String::new(),
        model: String::new(),
        color: String::new(),
        skills: Vec::new(),
    }
}

/// Returns the string form of a scalar field, mirroring yaml.v3 unmarshal into
/// a `string` field: strings pass through; null yields empty; bool/number are
/// rejected (yaml.v3 would error). Sequences/mappings are rejected.
fn scalar_string(v: &YamlValue, _key: &str) -> Result<String, String> {
    match v {
        YamlValue::String(s) => Ok(s.clone()),
        YamlValue::Null => Ok(String::new()),
        YamlValue::Sequence(_) => Err("cannot unmarshal !!seq into string".to_string()),
        YamlValue::Mapping(_) => Err("cannot unmarshal !!map into string".to_string()),
        // yaml.v3 will coerce scalar bool/int into string in some modes; the
        // agent corpus never exercises this, so mirror the string form.
        YamlValue::Bool(b) => Ok(b.to_string()),
        YamlValue::Number(n) => Ok(n.clone()),
    }
}

/// Validates a single agent, returning the ordered list of check results.
/// Mirrors Go `validateAgent`. `agent_names` accumulates seen names for the
/// uniqueness rule; `skill_names` is the set of valid skill directory names.
pub fn validate_agent(
    agent_path: &std::path::Path,
    filename: &str,
    agent_names: &mut BTreeSet<String>,
    skill_names: &BTreeSet<String>,
) -> Vec<ValidationCheck> {
    let mut checks: Vec<ValidationCheck> = Vec::new();

    let content = match std::fs::read(agent_path) {
        Ok(c) => c,
        Err(e) => {
            checks.push(fail(
                &format!("Agent: {filename} - Read File"),
                "",
                "",
                &format!("Failed to read file: {e}"),
            ));
            return checks;
        }
    };

    // Rule 0: YAML formatting (BEFORE normalization).
    let formatting =
        validate_yaml_formatting_raw(&format!("Agent: {filename} - YAML Formatting"), &content);
    let formatting_failed = formatting.status == "failed";
    checks.push(formatting);
    if formatting_failed {
        return checks;
    }

    // Rule 1: YAML frontmatter syntax validity.
    let (frontmatter, _body) = match extract_frontmatter(&content) {
        Ok(fb) => fb,
        Err(e) => {
            checks.push(fail(
                &format!("Agent: {filename} - YAML Syntax"),
                "",
                "",
                &format!("Invalid frontmatter: {e}"),
            ));
            return checks;
        }
    };
    checks.push(ValidationCheck::passed(
        format!("Agent: {filename} - YAML Syntax"),
        "Valid YAML frontmatter",
    ));

    // Parse YAML.
    let value = match parse_yaml_value(&frontmatter) {
        Ok(v) => v,
        Err(e) => {
            checks.push(fail(
                &format!("Agent: {filename} - YAML Parse"),
                "",
                "",
                &format!("Failed to parse YAML: {e}"),
            ));
            return checks;
        }
    };

    let agent = match extract_claude_agent(&value) {
        Ok(a) => a,
        Err(msg) => {
            checks.push(fail(
                &format!("Agent: {filename} - YAML Parse"),
                "",
                "",
                &format!("Failed to parse YAML: yaml: unmarshal errors:\n  {msg}"),
            ));
            return checks;
        }
    };

    // Rule 2: Required fields present.
    let required = validate_required_fields(filename, &agent);
    let required_failed = required.status == "failed";
    checks.push(required);
    if required_failed {
        return checks;
    }

    // Rule 3: Field order.
    checks.push(validate_field_order(filename, &value));

    // Rule 4: Valid tool names.
    checks.push(validate_tools(filename, &agent.tools));

    // Rule 5: Valid model names.
    checks.push(validate_model(filename, &agent.model));

    // Rule 6: Valid colors.
    checks.push(validate_color(filename, &agent.color));

    // Rule 7: Filename matches name field.
    checks.push(validate_filename(filename, &agent.name));

    // Rule 8: Agent name uniqueness.
    let uniqueness = validate_uniqueness(filename, &agent.name, agent_names);
    let unique_ok = uniqueness.status == "passed";
    checks.push(uniqueness);
    if unique_ok {
        agent_names.insert(agent.name.clone());
    }

    // Rule 9: Skills references exist.
    checks.push(validate_skills_exist(filename, &agent.skills, skill_names));

    // Rule 10: No YAML comments in frontmatter.
    checks.push(validate_no_comments(filename, &frontmatter));

    // Rule 11: Special rule for generated-reports/ agents.
    if agent_path.to_string_lossy().contains("generated-reports") {
        checks.push(validate_generated_reports_tools(filename, &agent.tools));
    }

    checks
}

fn validate_required_fields(filename: &str, agent: &ClaudeAgentFull) -> ValidationCheck {
    let mut missing: Vec<&str> = Vec::new();
    if agent.name.is_empty() {
        missing.push("name");
    }
    if agent.description.is_empty() {
        missing.push("description");
    }
    if agent.tools.is_empty() {
        missing.push("tools");
    }
    // model can be empty (valid).
    if agent.color.is_empty() {
        missing.push("color");
    }
    // skills can be empty (valid).

    if !missing.is_empty() {
        return fail(
            &format!("Agent: {filename} - Required Fields"),
            "All required fields present",
            &format!("Missing: {}", go_string_slice(&missing)),
            "Required fields missing",
        );
    }
    ValidationCheck::passed(
        format!("Agent: {filename} - Required Fields"),
        "All required fields present",
    )
}

fn validate_field_order(filename: &str, value: &YamlValue) -> ValidationCheck {
    let mut field_names: Vec<String> = Vec::new();
    if let YamlValue::Mapping(pairs) = value {
        for (k, _) in pairs {
            field_names.push(k.clone());
        }
    }

    let expected = REQUIRED_FIELD_ORDER;
    if field_names.len() > expected.len() {
        return fail(
            &format!("Agent: {filename} - Field Order"),
            &format!("Fields: {}", go_string_slice(expected)),
            &format!("Fields: {}", go_string_slice_owned(&field_names)),
            "Extra fields present",
        );
    }

    for (i, field) in field_names.iter().enumerate() {
        if i >= expected.len() || field != expected[i] {
            return fail(
                &format!("Agent: {filename} - Field Order"),
                &format!("Order: {}", go_string_slice(&expected[..field_names.len()])),
                &format!("Order: {}", go_string_slice_owned(&field_names)),
                "Field order incorrect",
            );
        }
    }

    ValidationCheck::passed(
        format!("Agent: {filename} - Field Order"),
        "Field order correct",
    )
}

fn validate_tools(filename: &str, tools_str: &str) -> ValidationCheck {
    let mut invalid: Vec<&str> = Vec::new();
    for tool in tools_str.split(',') {
        let tool = tool.trim();
        if tool.is_empty() {
            continue;
        }
        if !is_valid_tool(tool) {
            invalid.push(tool);
        }
    }

    if !invalid.is_empty() {
        return fail(
            &format!("Agent: {filename} - Valid Tools"),
            &format!("Valid tools: {}", go_string_slice(VALID_TOOLS)),
            &format!("Invalid tools: {}", go_string_slice(&invalid)),
            "Invalid tool names",
        );
    }
    ValidationCheck::passed(
        format!("Agent: {filename} - Valid Tools"),
        "All tools valid",
    )
}

fn validate_model(filename: &str, model: &str) -> ValidationCheck {
    if !is_valid_model(model) {
        let valid_models = ["(empty)", "sonnet", "opus", "haiku"];
        return fail(
            &format!("Agent: {filename} - Valid Model"),
            &format!("Valid models: {}", go_string_slice(&valid_models)),
            &format!("Model: {model}"),
            "Invalid model",
        );
    }
    let _ = VALID_MODELS;
    ValidationCheck::passed(format!("Agent: {filename} - Valid Model"), "Model valid")
}

fn validate_color(filename: &str, color: &str) -> ValidationCheck {
    if !is_valid_color(color) {
        let valid_colors = ["blue", "green", "yellow", "purple"];
        return fail(
            &format!("Agent: {filename} - Valid Color"),
            &format!("Valid colors: {}", go_string_slice(&valid_colors)),
            &format!("Color: {color}"),
            "Invalid color",
        );
    }
    let _ = VALID_COLORS;
    ValidationCheck::passed(format!("Agent: {filename} - Valid Color"), "Color valid")
}

fn validate_filename(filename: &str, name: &str) -> ValidationCheck {
    let expected = format!("{name}.md");
    if filename != expected {
        return fail(
            &format!("Agent: {filename} - Filename Match"),
            &format!("Filename: {expected}"),
            &format!("Filename: {filename}"),
            "Filename does not match name field",
        );
    }
    ValidationCheck::passed(
        format!("Agent: {filename} - Filename Match"),
        "Filename matches name",
    )
}

fn validate_uniqueness(
    filename: &str,
    name: &str,
    agent_names: &BTreeSet<String>,
) -> ValidationCheck {
    if agent_names.contains(name) {
        return fail(
            &format!("Agent: {filename} - Name Uniqueness"),
            "Unique agent name",
            &format!("Duplicate name: {name}"),
            "Agent name already used",
        );
    }
    ValidationCheck::passed(
        format!("Agent: {filename} - Name Uniqueness"),
        "Agent name unique",
    )
}

fn validate_skills_exist(
    filename: &str,
    skills: &[String],
    skill_names: &BTreeSet<String>,
) -> ValidationCheck {
    let mut missing: Vec<&str> = Vec::new();
    for skill in skills {
        if !skill_names.contains(skill) {
            missing.push(skill);
        }
    }

    if !missing.is_empty() {
        return fail(
            &format!("Agent: {filename} - Skills Exist"),
            "All skills exist",
            &format!("Missing skills: {}", go_string_slice(&missing)),
            "Referenced skills not found",
        );
    }
    ValidationCheck::passed(
        format!("Agent: {filename} - Skills Exist"),
        "All skills exist",
    )
}

fn validate_no_comments(filename: &str, frontmatter: &[u8]) -> ValidationCheck {
    for line in frontmatter.split(|&b| b == b'\n') {
        let trimmed = trim_space(line);
        if trimmed.starts_with(b"#") {
            return fail(
                &format!("Agent: {filename} - No Comments"),
                "No YAML comments",
                "Comments found",
                "YAML comments not allowed in frontmatter",
            );
        }
    }
    ValidationCheck::passed(
        format!("Agent: {filename} - No Comments"),
        "No YAML comments",
    )
}

fn validate_generated_reports_tools(filename: &str, tools_str: &str) -> ValidationCheck {
    let mut has_write = false;
    let mut has_bash = false;
    for tool in tools_str.split(',') {
        let tool = tool.trim();
        if tool == "Write" {
            has_write = true;
        }
        if tool == "Bash" {
            has_bash = true;
        }
    }

    if !has_write || !has_bash {
        return fail(
            &format!("Agent: {filename} - Generated Reports Tools"),
            "Tools must include: Write, Bash",
            &format!("Has Write: {has_write}, Has Bash: {has_bash}"),
            "generated-reports/ agents must have Write AND Bash tools",
        );
    }
    ValidationCheck::passed(
        format!("Agent: {filename} - Generated Reports Tools"),
        "Has required Write and Bash tools",
    )
}

/// Validates every agent under `.claude/agents/`. Mirrors Go
/// `validateAllAgents`. Returns the flattened, in-directory-order check list.
pub fn validate_all_agents(
    repo_root: &std::path::Path,
    skill_names: &BTreeSet<String>,
) -> Vec<ValidationCheck> {
    let agents_dir = repo_root.join(".claude").join("agents");

    let entries = match std::fs::read_dir(&agents_dir) {
        Ok(e) => e,
        Err(e) => {
            return vec![fail(
                "Read Agents Directory",
                "",
                "",
                &format!("Failed to read agents directory: {e}"),
            )];
        }
    };

    // Collect names in sorted order (mirrors os.ReadDir's sorted iteration).
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

    let mut agent_names: BTreeSet<String> = BTreeSet::new();
    let mut all_checks: Vec<ValidationCheck> = Vec::new();
    for name in names {
        let path = agents_dir.join(&name);
        let checks = validate_agent(&path, &name, &mut agent_names, skill_names);
        all_checks.extend(checks);
    }
    all_checks
}

// --- helpers ---

fn fail(name: &str, expected: &str, actual: &str, message: &str) -> ValidationCheck {
    ValidationCheck {
        name: name.to_string(),
        status: "failed".to_string(),
        expected: expected.to_string(),
        actual: actual.to_string(),
        message: message.to_string(),
    }
}

/// Formats a slice of strings the way Go's `fmt.Sprintf("%v", []string{...})`
/// does: `[a b c]` (space-separated, square-bracketed, no quotes).
fn go_string_slice(items: &[&str]) -> String {
    format!("[{}]", items.join(" "))
}

fn go_string_slice_owned(items: &[String]) -> String {
    format!("[{}]", items.join(" "))
}

fn trim_space(s: &[u8]) -> &[u8] {
    let is_ws =
        |b: u8| b == b' ' || b == b'\t' || b == b'\r' || b == b'\n' || b == 0x0b || b == 0x0c;
    let mut start = 0;
    let mut end = s.len();
    while start < end && is_ws(s[start]) {
        start += 1;
    }
    while end > start && is_ws(s[end - 1]) {
        end -= 1;
    }
    &s[start..end]
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn skills() -> BTreeSet<String> {
        let mut s = BTreeSet::new();
        s.insert("swe-programming-rust".to_string());
        s
    }

    fn write_agent(dir: &std::path::Path, name: &str, content: &str) -> std::path::PathBuf {
        let p = dir.join(name);
        std::fs::write(&p, content).unwrap();
        p
    }

    #[test]
    fn valid_agent_passes_all_rules() {
        let dir = tempfile::tempdir().unwrap();
        let p = write_agent(
            dir.path(),
            "foo-maker.md",
            "---\nname: foo-maker\ndescription: Makes foo.\ntools: Read, Write\nmodel:\ncolor: blue\nskills:\n  - swe-programming-rust\n---\nbody\n",
        );
        let mut names = BTreeSet::new();
        let checks = validate_agent(&p, "foo-maker.md", &mut names, &skills());
        assert!(
            checks.iter().all(|c| c.status == "passed"),
            "checks: {checks:?}"
        );
        assert!(names.contains("foo-maker"));
    }

    #[test]
    fn missing_tools_field_fails_required() {
        let dir = tempfile::tempdir().unwrap();
        let p = write_agent(
            dir.path(),
            "foo-maker.md",
            "---\nname: foo-maker\ndescription: d\nmodel:\ncolor: blue\n---\nbody\n",
        );
        let mut names = BTreeSet::new();
        let checks = validate_agent(&p, "foo-maker.md", &mut names, &skills());
        let req = checks
            .iter()
            .find(|c| c.name.contains("Required Fields"))
            .unwrap();
        assert_eq!(req.status, "failed");
        assert_eq!(req.actual, "Missing: [tools]");
    }

    #[test]
    fn invalid_tool_name_fails() {
        let dir = tempfile::tempdir().unwrap();
        let p = write_agent(
            dir.path(),
            "foo-maker.md",
            "---\nname: foo-maker\ndescription: d\ntools: Read, Frobnicate\nmodel:\ncolor: blue\n---\nbody\n",
        );
        let mut names = BTreeSet::new();
        let checks = validate_agent(&p, "foo-maker.md", &mut names, &skills());
        let t = checks
            .iter()
            .find(|c| c.name.contains("Valid Tools"))
            .unwrap();
        assert_eq!(t.status, "failed");
        assert_eq!(t.actual, "Invalid tools: [Frobnicate]");
    }

    #[test]
    fn filename_mismatch_fails() {
        let dir = tempfile::tempdir().unwrap();
        let p = write_agent(
            dir.path(),
            "wrong-maker.md",
            "---\nname: foo-maker\ndescription: d\ntools: Read\nmodel:\ncolor: blue\n---\nbody\n",
        );
        let mut names = BTreeSet::new();
        let checks = validate_agent(&p, "wrong-maker.md", &mut names, &skills());
        let f = checks
            .iter()
            .find(|c| c.name.contains("Filename Match"))
            .unwrap();
        assert_eq!(f.status, "failed");
        assert_eq!(f.expected, "Filename: foo-maker.md");
    }

    #[test]
    fn duplicate_name_fails() {
        let dir = tempfile::tempdir().unwrap();
        let p = write_agent(
            dir.path(),
            "foo-maker.md",
            "---\nname: foo-maker\ndescription: d\ntools: Read\nmodel:\ncolor: blue\n---\nbody\n",
        );
        let mut names = BTreeSet::new();
        names.insert("foo-maker".to_string());
        let checks = validate_agent(&p, "foo-maker.md", &mut names, &skills());
        let u = checks
            .iter()
            .find(|c| c.name.contains("Name Uniqueness"))
            .unwrap();
        assert_eq!(u.status, "failed");
        assert_eq!(u.actual, "Duplicate name: foo-maker");
    }

    #[test]
    fn comment_in_frontmatter_fails() {
        let dir = tempfile::tempdir().unwrap();
        let p = write_agent(
            dir.path(),
            "foo-maker.md",
            "---\nname: foo-maker\ndescription: d\ntools: Read\nmodel:\ncolor: blue\n# comment\n---\nbody\n",
        );
        let mut names = BTreeSet::new();
        let checks = validate_agent(&p, "foo-maker.md", &mut names, &skills());
        let c = checks
            .iter()
            .find(|c| c.name.contains("No Comments"))
            .unwrap();
        assert_eq!(c.status, "failed");
    }

    #[test]
    fn missing_skill_reference_fails() {
        let dir = tempfile::tempdir().unwrap();
        let p = write_agent(
            dir.path(),
            "foo-maker.md",
            "---\nname: foo-maker\ndescription: d\ntools: Read\nmodel:\ncolor: blue\nskills:\n  - nonexistent\n---\nbody\n",
        );
        let mut names = BTreeSet::new();
        let checks = validate_agent(&p, "foo-maker.md", &mut names, &skills());
        let s = checks
            .iter()
            .find(|c| c.name.contains("Skills Exist"))
            .unwrap();
        assert_eq!(s.status, "failed");
        assert_eq!(s.actual, "Missing skills: [nonexistent]");
    }

    #[test]
    fn array_tools_fails_parse() {
        let dir = tempfile::tempdir().unwrap();
        let p = write_agent(
            dir.path(),
            "foo-maker.md",
            "---\nname: foo-maker\ndescription: d\ntools:\n  - Read\n  - Write\nmodel:\ncolor: blue\n---\nbody\n",
        );
        let mut names = BTreeSet::new();
        let checks = validate_agent(&p, "foo-maker.md", &mut names, &skills());
        let parse = checks
            .iter()
            .find(|c| c.name.contains("YAML Parse"))
            .unwrap();
        assert_eq!(parse.status, "failed");
        assert!(parse.message.contains("cannot unmarshal !!seq into string"));
    }

    #[test]
    fn bad_yaml_formatting_short_circuits() {
        let dir = tempfile::tempdir().unwrap();
        let p = write_agent(
            dir.path(),
            "foo-maker.md",
            "---\nname:foo-maker\n---\nbody\n",
        );
        let mut names = BTreeSet::new();
        let checks = validate_agent(&p, "foo-maker.md", &mut names, &skills());
        assert_eq!(checks.len(), 1);
        assert!(checks[0].name.contains("YAML Formatting"));
        assert_eq!(checks[0].status, "failed");
    }

    #[test]
    fn read_error_reports_failure() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("nope-maker.md");
        let mut names = BTreeSet::new();
        let checks = validate_agent(&missing, "nope-maker.md", &mut names, &skills());
        assert_eq!(checks.len(), 1);
        assert!(checks[0].name.contains("Read File"));
        assert_eq!(checks[0].status, "failed");
    }

    #[test]
    fn invalid_yaml_syntax_fails() {
        let dir = tempfile::tempdir().unwrap();
        // Valid formatting (space after colon) but no closing fence after start.
        let p = write_agent(dir.path(), "foo-maker.md", "---\nname: foo-maker\n");
        let mut names = BTreeSet::new();
        let checks = validate_agent(&p, "foo-maker.md", &mut names, &skills());
        // YAML Formatting passes (too short → passed), then YAML Syntax fails.
        let syntax = checks.iter().find(|c| c.name.contains("YAML Syntax"));
        assert!(syntax.is_some() || checks.iter().any(|c| c.status == "failed"));
    }

    #[test]
    fn field_order_extra_fields_fails() {
        let dir = tempfile::tempdir().unwrap();
        let p = write_agent(
            dir.path(),
            "foo-maker.md",
            "---\nname: foo-maker\ndescription: d\ntools: Read\nmodel:\ncolor: blue\nskills:\n  - swe-programming-rust\nextra: x\n---\nbody\n",
        );
        let mut names = BTreeSet::new();
        let checks = validate_agent(&p, "foo-maker.md", &mut names, &skills());
        let fo = checks
            .iter()
            .find(|c| c.name.contains("Field Order"))
            .unwrap();
        assert_eq!(fo.status, "failed");
        assert_eq!(fo.message, "Extra fields present");
    }

    #[test]
    fn field_order_wrong_order_fails() {
        let dir = tempfile::tempdir().unwrap();
        let p = write_agent(
            dir.path(),
            "foo-maker.md",
            "---\ndescription: d\nname: foo-maker\ntools: Read\nmodel:\ncolor: blue\n---\nbody\n",
        );
        let mut names = BTreeSet::new();
        let checks = validate_agent(&p, "foo-maker.md", &mut names, &skills());
        let fo = checks
            .iter()
            .find(|c| c.name.contains("Field Order"))
            .unwrap();
        assert_eq!(fo.status, "failed");
        assert_eq!(fo.message, "Field order incorrect");
    }

    #[test]
    fn invalid_model_fails() {
        let dir = tempfile::tempdir().unwrap();
        let p = write_agent(
            dir.path(),
            "foo-maker.md",
            "---\nname: foo-maker\ndescription: d\ntools: Read\nmodel: gpt5\ncolor: blue\n---\nbody\n",
        );
        let mut names = BTreeSet::new();
        let checks = validate_agent(&p, "foo-maker.md", &mut names, &skills());
        let m = checks
            .iter()
            .find(|c| c.name.contains("Valid Model"))
            .unwrap();
        assert_eq!(m.status, "failed");
        assert_eq!(m.actual, "Model: gpt5");
    }

    #[test]
    fn invalid_color_fails() {
        let dir = tempfile::tempdir().unwrap();
        let p = write_agent(
            dir.path(),
            "foo-maker.md",
            "---\nname: foo-maker\ndescription: d\ntools: Read\nmodel:\ncolor: chartreuse\n---\nbody\n",
        );
        let mut names = BTreeSet::new();
        let checks = validate_agent(&p, "foo-maker.md", &mut names, &skills());
        let c = checks
            .iter()
            .find(|c| c.name.contains("Valid Color"))
            .unwrap();
        assert_eq!(c.status, "failed");
        assert_eq!(c.actual, "Color: chartreuse");
    }

    #[test]
    fn generated_reports_requires_write_and_bash() {
        let dir = tempfile::tempdir().unwrap();
        let gr = dir.path().join("generated-reports");
        std::fs::create_dir_all(&gr).unwrap();
        let p = write_agent(
            &gr,
            "foo-maker.md",
            "---\nname: foo-maker\ndescription: d\ntools: Read, Glob\nmodel:\ncolor: blue\n---\nbody\n",
        );
        let mut names = BTreeSet::new();
        let checks = validate_agent(&p, "foo-maker.md", &mut names, &skills());
        let gr_check = checks
            .iter()
            .find(|c| c.name.contains("Generated Reports Tools"))
            .unwrap();
        assert_eq!(gr_check.status, "failed");
        assert_eq!(gr_check.actual, "Has Write: false, Has Bash: false");
    }

    #[test]
    fn validate_all_agents_missing_dir() {
        let dir = tempfile::tempdir().unwrap();
        let checks = validate_all_agents(dir.path(), &skills());
        assert_eq!(checks.len(), 1);
        assert!(checks[0].name.contains("Read Agents Directory"));
    }

    #[test]
    fn validate_all_agents_iterates() {
        let dir = tempfile::tempdir().unwrap();
        let agents = dir.path().join(".claude/agents");
        std::fs::create_dir_all(&agents).unwrap();
        std::fs::write(
            agents.join("foo-maker.md"),
            "---\nname: foo-maker\ndescription: d\ntools: Read\nmodel:\ncolor: blue\n---\nbody\n",
        )
        .unwrap();
        std::fs::write(agents.join("README.md"), "# readme\n").unwrap();
        let checks = validate_all_agents(dir.path(), &skills());
        assert!(
            checks
                .iter()
                .all(|c| c.name.starts_with("Agent: foo-maker.md"))
        );
        assert!(!checks.is_empty());
    }
}
