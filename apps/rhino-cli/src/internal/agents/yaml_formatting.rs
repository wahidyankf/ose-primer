//! YAML frontmatter formatting check (space after colons).
//!
//! The check runs on the RAW (pre-normalization) content so it can flag `name:value` style
//! errors that normalization would otherwise mask.

use super::types::ValidationCheck;

/// Checks YAML frontmatter formatting (space after colons). `check_name` is the
/// full `ValidationCheck.name`, e.g. `"Agent: foo.md - YAML Formatting"`.
pub fn validate_yaml_formatting_raw(check_name: &str, content: &[u8]) -> ValidationCheck {
    let lines: Vec<&[u8]> = content.split(|&b| b == b'\n').collect();

    if lines.len() < 3 {
        return ValidationCheck::passed(check_name, "File too short to check formatting");
    }

    if trim_space(lines[0]) != b"---" {
        return ValidationCheck {
            name: check_name.to_string(),
            status: "failed".to_string(),
            expected: String::new(),
            actual: String::new(),
            message: "Frontmatter does not start with ---".to_string(),
        };
    }

    let mut end_index: isize = -1;
    for (i, line) in lines.iter().enumerate().skip(1) {
        if trim_space(line) == b"---" {
            end_index = i as isize;
            break;
        }
    }

    if end_index == -1 {
        return ValidationCheck {
            name: check_name.to_string(),
            status: "failed".to_string(),
            expected: String::new(),
            actual: String::new(),
            message: "Frontmatter closing --- not found".to_string(),
        };
    }
    let end = end_index.cast_unsigned();

    let mut issues: Vec<String> = Vec::new();
    for (i, line) in lines.iter().enumerate().take(end).skip(1) {
        let trimmed = trim_space(line);

        // Skip empty lines, list items, and comments.
        if trimmed.is_empty() || trimmed.starts_with(b"-") || trimmed.starts_with(b"#") {
            continue;
        }

        // Key-value pair without a space after the colon.
        if trimmed.contains(&b':') {
            // SplitN(trimmed, ":", 2): first colon splits into 2 parts.
            if let Some(colon_pos) = trimmed.iter().position(|&b| b == b':') {
                let after = &trimmed[colon_pos + 1..];
                if !after.is_empty() && after[0] != b' ' {
                    issues.push(format!(
                        "Line {}: '{}' (missing space after colon)",
                        i + 1,
                        String::from_utf8_lossy(trimmed)
                    ));
                }
            }
        }
    }

    if !issues.is_empty() {
        return ValidationCheck {
            name: check_name.to_string(),
            status: "failed".to_string(),
            expected: "Space after colon in YAML key-value pairs (e.g., 'name: value')".to_string(),
            actual: format!("Found {} formatting issues", issues.len()),
            message: format!("YAML formatting errors:\n  {}", issues.join("\n  ")),
        };
    }

    ValidationCheck::passed(check_name, "YAML formatting correct (spaces after colons)")
}

/// Trims ASCII whitespace (mirrors `bytes.TrimSpace`).
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

    #[test]
    fn passes_well_formatted() {
        let content = b"---\nname: foo\ndescription: bar\n---\nbody\n";
        let c = validate_yaml_formatting_raw("Agent: foo.md - YAML Formatting", content);
        assert_eq!(c.status, "passed");
    }

    #[test]
    fn flags_missing_space() {
        let content = b"---\nname:foo\n---\nbody\n";
        let c = validate_yaml_formatting_raw("Agent: foo.md - YAML Formatting", content);
        assert_eq!(c.status, "failed");
        assert!(c.message.contains("missing space after colon"));
        assert_eq!(c.actual, "Found 1 formatting issues");
    }

    #[test]
    fn skips_list_items_and_comments() {
        let content = b"---\nskills:\n  - a\n  # comment\n---\nbody\n";
        let c = validate_yaml_formatting_raw("Skill: x - YAML Formatting", content);
        assert_eq!(c.status, "passed");
    }

    #[test]
    fn too_short_passes() {
        let c = validate_yaml_formatting_raw("x", b"---\n");
        assert_eq!(c.status, "passed");
        assert_eq!(c.message, "File too short to check formatting");
    }

    #[test]
    fn no_start_fence_fails() {
        let c = validate_yaml_formatting_raw("x", b"name: a\nb: c\nd: e\n");
        assert_eq!(c.status, "failed");
        assert_eq!(c.message, "Frontmatter does not start with ---");
    }

    #[test]
    fn no_close_fence_fails() {
        let c = validate_yaml_formatting_raw("x", b"---\nname: a\nb: c\n");
        assert_eq!(c.status, "failed");
        assert_eq!(c.message, "Frontmatter closing --- not found");
    }
}
