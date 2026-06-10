//! Shared types for the `agents` command family.
//!
//! The struct field order and the `Valid*` sets mirror the Go originals exactly, since they
//! drive both validation messages and YAML serialization.

use std::time::Duration;

/// OpenCode-format agent configuration (serialization target).
///
/// Field order is `description`, `model`, `permission`, `color`, `skills`.
/// `color` and `skills` are omitted when empty (Go `omitempty`).
#[derive(Debug, Clone)]
pub struct OpenCodeAgent {
    pub description: String,
    pub model: String,
    /// Permission map. Keys are lowercased tool names mapped to a permission
    /// level (`allow`); serialized in sorted (alphabetical) order to match
    /// Go's `yaml.v3` map-key sorting.
    pub permission: std::collections::BTreeMap<String, String>,
    /// OpenCode theme token (translated from the Claude named color). Omitted
    /// when empty (Go `omitempty`).
    pub color: String,
    pub skills: Vec<String>,
}

/// Configures sync behavior.
#[derive(Debug, Clone)]
pub struct SyncOptions {
    pub repo_root: std::path::PathBuf,
    pub dry_run: bool,
    pub agents_only: bool,
    pub skills_only: bool,
    pub verbose: bool,
    pub quiet: bool,
}

/// Sync operation results.
#[derive(Debug, Clone, Default)]
pub struct SyncResult {
    pub agents_converted: i64,
    pub agents_failed: i64,
    pub skills_copied: i64,
    pub skills_failed: i64,
    pub failed_files: Vec<String>,
    pub duration: Duration,
}

/// Validation results.
#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    pub total_checks: i64,
    pub passed_checks: i64,
    pub failed_checks: i64,
    pub checks: Vec<ValidationCheck>,
    pub duration: Duration,
}

/// A single validation check. The `status` field
/// is the literal string `"passed"` or `"failed"` to match Go's JSON output.
#[derive(Debug, Clone)]
pub struct ValidationCheck {
    pub name: String,
    pub status: String,
    pub expected: String,
    pub actual: String,
    pub message: String,
}

impl ValidationCheck {
    /// Constructs a passing check (empty expected/actual).
    pub fn passed(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: "passed".to_string(),
            expected: String::new(),
            actual: String::new(),
            message: message.into(),
        }
    }
}

/// Configures `validate-claude` behavior.
#[derive(Debug, Clone)]
pub struct ValidateClaudeOptions {
    pub repo_root: std::path::PathBuf,
    pub agents_only: bool,
    pub skills_only: bool,
}

/// Recognized tool names in Claude Code agent frontmatter.
pub const VALID_TOOLS: &[&str] = &[
    "Read",
    "Write",
    "Edit",
    "Glob",
    "Grep",
    "Bash",
    "TodoWrite",
    "WebFetch",
    "WebSearch",
];

/// Recognized model values (empty inherits).
pub const VALID_MODELS: &[&str] = &["", "sonnet", "opus", "haiku"];

/// Recognized color values.
pub const VALID_COLORS: &[&str] = &["blue", "green", "yellow", "purple"];

/// Mandatory field ordering in agent frontmatter.
pub const REQUIRED_FIELD_ORDER: &[&str] =
    &["name", "description", "tools", "model", "color", "skills"];

/// Returns true if `tool` is a recognized Claude tool name.
pub fn is_valid_tool(tool: &str) -> bool {
    VALID_TOOLS.contains(&tool)
}

/// Returns true if `model` is a recognized Claude model value.
pub fn is_valid_model(model: &str) -> bool {
    VALID_MODELS.contains(&model)
}

/// Returns true if `color` is a recognized Claude color value.
pub fn is_valid_color(color: &str) -> bool {
    VALID_COLORS.contains(&color)
}
