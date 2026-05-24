//! Per-skill validation (the rules of `validate-claude` for skills).
//!
//! Byte-for-byte port of
//! `apps/rhino-cli-go/internal/agents/skill_validator.go`.

use std::collections::BTreeSet;

use regex::Regex;
use std::sync::OnceLock;

use super::frontmatter::{YamlValue, extract_frontmatter, parse_yaml_value};
use super::types::ValidationCheck;
use super::yaml_formatting::validate_yaml_formatting_raw;

/// Skill name pattern: lowercase letters, numbers, hyphens, 1–64 chars.
/// Mirrors Go `ValidSkillNamePattern`.
fn skill_name_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[a-z0-9-]{1,64}$").expect("valid regex"))
}

/// Validates a single skill (all rules). Mirrors Go `validateSkill`.
pub fn validate_skill(skill_path: &std::path::Path, skill_name: &str) -> Vec<ValidationCheck> {
    let mut checks: Vec<ValidationCheck> = Vec::new();

    // Rule 1: SKILL.md exists.
    let skill_file = skill_path.join("SKILL.md");
    if !skill_file.exists() {
        checks.push(fail(
            &format!("Skill: {skill_name} - SKILL.md Exists"),
            "SKILL.md file present",
            "SKILL.md file not found",
            "SKILL.md file missing",
        ));
        return checks;
    }
    checks.push(ValidationCheck::passed(
        format!("Skill: {skill_name} - SKILL.md Exists"),
        "SKILL.md file exists",
    ));

    let content = match std::fs::read(&skill_file) {
        Ok(c) => c,
        Err(e) => {
            checks.push(fail(
                &format!("Skill: {skill_name} - Read SKILL.md"),
                "",
                "",
                &format!("Failed to read SKILL.md: {e}"),
            ));
            return checks;
        }
    };

    // Rule 0: YAML formatting (BEFORE normalization).
    let formatting =
        validate_yaml_formatting_raw(&format!("Skill: {skill_name} - YAML Formatting"), &content);
    let formatting_failed = formatting.status == "failed";
    checks.push(formatting);
    if formatting_failed {
        return checks;
    }

    // Rule 3: YAML syntax validity.
    let (frontmatter, _body) = match extract_frontmatter(&content) {
        Ok(fb) => fb,
        Err(e) => {
            checks.push(fail(
                &format!("Skill: {skill_name} - YAML Syntax"),
                "",
                "",
                &format!("Invalid frontmatter: {e}"),
            ));
            return checks;
        }
    };
    checks.push(ValidationCheck::passed(
        format!("Skill: {skill_name} - YAML Syntax"),
        "Valid YAML frontmatter",
    ));

    // Parse into name + description.
    let (name, description) = match parse_yaml_value(&frontmatter) {
        Ok(v) => extract_skill(&v),
        Err(e) => {
            checks.push(fail(
                &format!("Skill: {skill_name} - YAML Parse"),
                "",
                "",
                &format!("Failed to parse YAML: {e}"),
            ));
            return checks;
        }
    };

    // Rule 2: description present.
    if description.is_empty() {
        checks.push(fail(
            &format!("Skill: {skill_name} - Description Field Required"),
            "description field present",
            "description field missing or empty",
            "Required description field missing",
        ));
        return checks;
    }
    checks.push(ValidationCheck::passed(
        format!("Skill: {skill_name} - Description Field Required"),
        "Required description field present",
    ));

    // Rule 4: name present.
    if name.is_empty() {
        checks.push(fail(
            &format!("Skill: {skill_name} - Name Field Required"),
            "name field present",
            "name field missing or empty",
            "Required name field missing",
        ));
        return checks;
    }
    checks.push(ValidationCheck::passed(
        format!("Skill: {skill_name} - Name Field Required"),
        "Required name field present",
    ));

    // Rule 5: name format.
    if !skill_name_re().is_match(&name) {
        checks.push(fail(
            &format!("Skill: {skill_name} - Name Format"),
            "Lowercase letters/numbers/hyphens only, max 64 chars",
            &format!("Name: {name}"),
            "Invalid skill name format",
        ));
        return checks;
    }
    checks.push(ValidationCheck::passed(
        format!("Skill: {skill_name} - Name Format"),
        "Name format valid",
    ));

    // Rule 6: name matches directory.
    if name != skill_name {
        checks.push(fail(
            &format!("Skill: {skill_name} - Name Match"),
            &format!("name field matches directory: {skill_name}"),
            &format!("name field: {name}"),
            "Skill name must match directory name",
        ));
        return checks;
    }
    checks.push(ValidationCheck::passed(
        format!("Skill: {skill_name} - Name Match"),
        "Name matches directory name",
    ));

    checks
}

/// Extracts `(name, description)` from a parsed skill mapping. Only string
/// scalars are read (mirrors yaml.v3 unmarshal into `ClaudeSkill`).
fn extract_skill(value: &YamlValue) -> (String, String) {
    let mut name = String::new();
    let mut description = String::new();
    if let YamlValue::Mapping(pairs) = value {
        for (k, v) in pairs {
            match k.as_str() {
                "name" => {
                    if let YamlValue::String(s) = v {
                        name = s.clone();
                    }
                }
                "description" => {
                    if let YamlValue::String(s) = v {
                        description = s.clone();
                    }
                }
                _ => {}
            }
        }
    }
    (name, description)
}

/// Validates all skills under `.claude/skills/`. Mirrors Go
/// `validateAllSkills`. Returns `(checks, valid_skill_names)`; a skill is added
/// to the name set only when ALL its checks pass.
pub fn validate_all_skills(
    repo_root: &std::path::Path,
) -> (Vec<ValidationCheck>, BTreeSet<String>) {
    let skills_dir = repo_root.join(".claude").join("skills");

    let entries = match std::fs::read_dir(&skills_dir) {
        Ok(e) => e,
        Err(e) => {
            return (
                vec![fail(
                    "Read Skills Directory",
                    "",
                    "",
                    &format!("Failed to read skills directory: {e}"),
                )],
                BTreeSet::new(),
            );
        }
    };

    // Collect directory names in sorted order (mirrors os.ReadDir sorting).
    let mut names: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let is_dir = entry.file_type().is_ok_and(|t| t.is_dir());
        if !is_dir || name.starts_with('.') {
            continue;
        }
        names.push(name);
    }
    names.sort();

    let mut skill_names: BTreeSet<String> = BTreeSet::new();
    let mut all_checks: Vec<ValidationCheck> = Vec::new();
    for name in names {
        let skill_path = skills_dir.join(&name);
        let checks = validate_skill(&skill_path, &name);
        let all_passed = checks.iter().all(|c| c.status != "failed");
        all_checks.extend(checks);
        if all_passed {
            skill_names.insert(name);
        }
    }

    (all_checks, skill_names)
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

    fn write_skill(root: &std::path::Path, name: &str, content: &str) {
        let dir = root.join(name);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("SKILL.md"), content).unwrap();
    }

    #[test]
    fn valid_skill_passes() {
        let dir = tempfile::tempdir().unwrap();
        write_skill(
            dir.path(),
            "my-skill",
            "---\nname: my-skill\ndescription: A skill.\n---\nbody\n",
        );
        let checks = validate_skill(&dir.path().join("my-skill"), "my-skill");
        assert!(checks.iter().all(|c| c.status == "passed"), "{checks:?}");
    }

    #[test]
    fn missing_skill_md_fails() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("empty")).unwrap();
        let checks = validate_skill(&dir.path().join("empty"), "empty");
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].status, "failed");
        assert!(checks[0].name.contains("SKILL.md Exists"));
    }

    #[test]
    fn missing_description_fails() {
        let dir = tempfile::tempdir().unwrap();
        write_skill(dir.path(), "my-skill", "---\nname: my-skill\n---\nbody\n");
        let checks = validate_skill(&dir.path().join("my-skill"), "my-skill");
        let d = checks
            .iter()
            .find(|c| c.name.contains("Description Field Required"))
            .unwrap();
        assert_eq!(d.status, "failed");
    }

    #[test]
    fn name_mismatch_fails() {
        let dir = tempfile::tempdir().unwrap();
        write_skill(
            dir.path(),
            "my-skill",
            "---\nname: other-name\ndescription: d\n---\nbody\n",
        );
        let checks = validate_skill(&dir.path().join("my-skill"), "my-skill");
        let m = checks
            .iter()
            .find(|c| c.name.contains("Name Match"))
            .unwrap();
        assert_eq!(m.status, "failed");
        assert_eq!(m.actual, "name field: other-name");
    }

    #[test]
    fn invalid_name_format_fails() {
        let dir = tempfile::tempdir().unwrap();
        write_skill(
            dir.path(),
            "Bad_Name",
            "---\nname: Bad_Name\ndescription: d\n---\nbody\n",
        );
        let checks = validate_skill(&dir.path().join("Bad_Name"), "Bad_Name");
        let f = checks
            .iter()
            .find(|c| c.name.contains("Name Format"))
            .unwrap();
        assert_eq!(f.status, "failed");
    }

    #[test]
    fn validate_all_collects_valid_names() {
        let dir = tempfile::tempdir().unwrap();
        let skills = dir.path().join(".claude/skills");
        write_skill(&skills, "good", "---\nname: good\ndescription: d\n---\nx\n");
        write_skill(&skills, "bad", "---\nname: bad\n---\nx\n");
        let (_checks, names) = validate_all_skills(dir.path());
        assert!(names.contains("good"));
        assert!(!names.contains("bad"));
    }

    #[test]
    fn validate_all_missing_dir_fails() {
        let dir = tempfile::tempdir().unwrap();
        let (checks, names) = validate_all_skills(dir.path());
        assert_eq!(checks.len(), 1);
        assert!(checks[0].name.contains("Read Skills Directory"));
        assert!(names.is_empty());
    }

    #[test]
    fn validate_all_skips_dotfiles_and_files() {
        let dir = tempfile::tempdir().unwrap();
        let skills = dir.path().join(".claude/skills");
        std::fs::create_dir_all(&skills).unwrap();
        write_skill(&skills, "good", "---\nname: good\ndescription: d\n---\nx\n");
        // A dotdir and a plain file are skipped.
        std::fs::create_dir_all(skills.join(".hidden")).unwrap();
        std::fs::write(skills.join("loose.txt"), "x").unwrap();
        let (_checks, names) = validate_all_skills(dir.path());
        assert!(names.contains("good"));
        assert_eq!(names.len(), 1);
    }

    #[test]
    fn missing_name_field_fails() {
        let dir = tempfile::tempdir().unwrap();
        write_skill(dir.path(), "my-skill", "---\ndescription: d\n---\nbody\n");
        let checks = validate_skill(&dir.path().join("my-skill"), "my-skill");
        let n = checks
            .iter()
            .find(|c| c.name.contains("Name Field Required"))
            .unwrap();
        assert_eq!(n.status, "failed");
    }

    #[test]
    fn invalid_yaml_syntax_fails() {
        let dir = tempfile::tempdir().unwrap();
        // Valid formatting but no closing fence → extract_frontmatter errors.
        write_skill(dir.path(), "my-skill", "---\nname: my-skill\n");
        let checks = validate_skill(&dir.path().join("my-skill"), "my-skill");
        assert!(checks.iter().any(|c| c.status == "failed"));
    }
}
