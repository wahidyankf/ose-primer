//! Shared types for the `doctor` subsystem.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/internal/doctor/types.go`.

/// Health status of a tool check. Mirrors Go `ToolStatus`. The string codes
/// (`ok`/`warning`/`missing`) match Go and feed the JSON `status` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolStatus {
    Ok,
    Warning,
    Missing,
}

impl ToolStatus {
    /// Canonical lowercase code, matching Go's `ToolStatus` constant values.
    pub fn code(self) -> &'static str {
        match self {
            ToolStatus::Ok => "ok",
            ToolStatus::Warning => "warning",
            ToolStatus::Missing => "missing",
        }
    }
}

/// Result of checking a single tool. Mirrors Go `ToolCheck`.
#[derive(Debug, Clone)]
pub struct ToolCheck {
    pub name: String,
    pub binary: String,
    pub status: ToolStatus,
    pub installed_version: String,
    pub required_version: String,
    pub source: String,
    pub note: String,
}

/// Tool scope. Mirrors Go `Scope` (`full` default, `minimal` core-only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Full,
    Minimal,
}

impl Scope {
    /// Parses the raw `--scope` value, matching Go's `Scope(scope)` cast:
    /// any value other than `"minimal"` behaves as full, but the JSON `scope`
    /// field echoes the raw string. We retain the raw string separately.
    pub fn code(self) -> &'static str {
        match self {
            Scope::Full => "full",
            Scope::Minimal => "minimal",
        }
    }
}

/// Tool names in the minimal scope. Mirrors Go `MinimalTools`.
pub const MINIMAL_TOOLS: &[&str] = &["git", "volta", "node", "npm", "golang", "docker", "jq"];

/// Whether `name` is part of the minimal tool set.
pub fn is_minimal_tool(name: &str) -> bool {
    MINIMAL_TOOLS.contains(&name)
}

/// Aggregated results of all tool checks. Mirrors Go `DoctorResult`.
/// `duration_ms` stores the elapsed milliseconds (Go `time.Duration`).
pub struct DoctorResult {
    pub checks: Vec<ToolCheck>,
    pub ok_count: i64,
    pub warn_count: i64,
    pub missing_count: i64,
    pub duration_ms: i64,
    /// Raw scope string as supplied on the command line (echoed into JSON).
    pub scope_raw: String,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn status_codes() {
        assert_eq!(ToolStatus::Ok.code(), "ok");
        assert_eq!(ToolStatus::Warning.code(), "warning");
        assert_eq!(ToolStatus::Missing.code(), "missing");
    }

    #[test]
    fn scope_codes() {
        assert_eq!(Scope::Full.code(), "full");
        assert_eq!(Scope::Minimal.code(), "minimal");
    }

    #[test]
    fn minimal_tool_membership() {
        assert!(is_minimal_tool("git"));
        assert!(is_minimal_tool("jq"));
        assert!(!is_minimal_tool("java"));
        assert!(!is_minimal_tool("playwright"));
        assert_eq!(MINIMAL_TOOLS.len(), 7);
    }
}
