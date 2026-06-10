//! `validate-claude` orchestration.
//!
//! Skills are validated first (their names feed the agent skill-reference rule), then
//! agents.

use std::collections::BTreeSet;
use std::time::Instant;

use anyhow::Error;

use super::agent_validator::validate_all_agents;
use super::skill_validator::validate_all_skills;
use super::types::{ValidateClaudeOptions, ValidationResult};

/// Validates the `.claude/` directory format.
pub fn validate_claude(opts: &ValidateClaudeOptions) -> Result<ValidationResult, Error> {
    let start = Instant::now();
    let mut result = ValidationResult {
        checks: Vec::new(),
        ..Default::default()
    };

    let skill_names: BTreeSet<String>;

    // Validate skills first (needed for agent validation).
    if opts.agents_only {
        // agents-only: still build skill names for the reference check.
        let (_checks, names) = validate_all_skills(&opts.repo_root);
        skill_names = names;
    } else {
        let (skill_checks, names) = validate_all_skills(&opts.repo_root);
        skill_names = names;
        for check in skill_checks {
            tally(&mut result, check);
        }
    }

    // Validate agents.
    if !opts.skills_only {
        let agent_checks = validate_all_agents(&opts.repo_root, &skill_names);
        for check in agent_checks {
            tally(&mut result, check);
        }
    }

    result.duration = start.elapsed();
    Ok(result)
}

fn tally(result: &mut ValidationResult, check: super::types::ValidationCheck) {
    if check.status == "passed" {
        result.passed_checks += 1;
    } else {
        result.failed_checks += 1;
    }
    result.total_checks += 1;
    result.checks.push(check);
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn fixture(root: &std::path::Path) {
        let agents = root.join(".claude/agents");
        let skills = root.join(".claude/skills/my-skill");
        std::fs::create_dir_all(&agents).unwrap();
        std::fs::create_dir_all(&skills).unwrap();
        std::fs::write(
            skills.join("SKILL.md"),
            "---\nname: my-skill\ndescription: d\n---\nx\n",
        )
        .unwrap();
        std::fs::write(
            agents.join("foo-maker.md"),
            "---\nname: foo-maker\ndescription: d\ntools: Read\nmodel:\ncolor: blue\nskills:\n  - my-skill\n---\nbody\n",
        )
        .unwrap();
    }

    #[test]
    fn all_valid_passes() {
        let dir = tempfile::tempdir().unwrap();
        fixture(dir.path());
        let opts = ValidateClaudeOptions {
            repo_root: dir.path().to_path_buf(),
            agents_only: false,
            skills_only: false,
        };
        let result = validate_claude(&opts).unwrap();
        assert_eq!(result.failed_checks, 0, "checks: {:?}", result.checks);
        assert!(result.total_checks > 0);
    }

    #[test]
    fn agents_only_skips_skill_checks() {
        let dir = tempfile::tempdir().unwrap();
        fixture(dir.path());
        let opts = ValidateClaudeOptions {
            repo_root: dir.path().to_path_buf(),
            agents_only: true,
            skills_only: false,
        };
        let result = validate_claude(&opts).unwrap();
        // No skill checks recorded.
        assert!(result.checks.iter().all(|c| c.name.starts_with("Agent:")));
    }

    #[test]
    fn skills_only_skips_agent_checks() {
        let dir = tempfile::tempdir().unwrap();
        fixture(dir.path());
        let opts = ValidateClaudeOptions {
            repo_root: dir.path().to_path_buf(),
            agents_only: false,
            skills_only: true,
        };
        let result = validate_claude(&opts).unwrap();
        assert!(result.checks.iter().all(|c| c.name.starts_with("Skill:")));
    }
}
