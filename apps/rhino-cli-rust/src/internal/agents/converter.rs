//! Claude → OpenCode agent conversion.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/internal/agents/converter.go`.
//!
//! The Go encoder uses `gopkg.in/yaml.v3` with `SetIndent(2)`. To reach
//! byte-identical output we replicate that emitter's behavior for the specific
//! `OpenCodeAgent` shape with a hand-rolled serializer ([`emit_opencode_yaml`]).
//! yaml.v3 emits plain (unquoted) scalars whenever the value is "plain-safe"
//! and never folds long scalars by default, so the emitter mirrors exactly
//! those rules. Map keys (the tool flags) are emitted in sorted order, matching
//! yaml.v3's deterministic map-key sorting.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::path::Path;
use std::sync::OnceLock;

use anyhow::{Error, anyhow};

use super::frontmatter::{YamlValue, extract_frontmatter, parse_claude_tools, parse_yaml_value};
use super::types::OpenCodeAgent;

/// Canonical relative path (from repo root) where converted OpenCode agent
/// files are written. Plural form per opencode.ai/docs/agents/. Mirrors Go
/// `OpenCodeAgentDir`.
pub const OPENCODE_AGENT_DIR: &str = ".opencode/agents";

/// Converts a Claude tools array to an OpenCode tools map (lowercased keys,
/// all `true`). Mirrors Go `ConvertTools`. Returns a `BTreeMap` so iteration is
/// alphabetically sorted (matching yaml.v3 map emission).
pub fn convert_tools(claude_tools: &[String]) -> BTreeMap<String, bool> {
    let mut tools = BTreeMap::new();
    for tool in claude_tools {
        let lower = tool.trim().to_lowercase();
        if !lower.is_empty() {
            tools.insert(lower, true);
        }
    }
    tools
}

/// Converts a Claude model name to an OpenCode model ID. Mirrors Go
/// `ConvertModel`: `haiku` → `opencode-go/glm-5`, everything else (including
/// empty/unknown) → `opencode-go/minimax-m2.7`.
pub fn convert_model(claude_model: &str) -> String {
    match claude_model.trim() {
        "haiku" => "opencode-go/glm-5".to_string(),
        _ => "opencode-go/minimax-m2.7".to_string(),
    }
}

/// Return the lazily-initialized Claude-to-OpenCode color token translation
/// map. Single source of truth for the color translation; mirrored in the Go
/// converter's `claudeToOpenCodeColor` and documented in
/// `repo-governance/development/agents/ai-agents.md`.
fn claude_to_opencode_color() -> &'static HashMap<&'static str, &'static str> {
    static M: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    M.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("blue", "primary");
        m.insert("green", "success");
        m.insert("yellow", "warning");
        m.insert("purple", "secondary");
        m.insert("red", "error");
        m.insert("orange", "warning");
        m.insert("pink", "accent");
        m.insert("cyan", "info");
        m
    })
}

/// Translate a Claude named color to the corresponding `OpenCode` theme token.
/// An empty string stays empty; a value that is already an `OpenCode` token or
/// an unknown/hex value passes through unchanged (escape hatch). Mirrors Go
/// `ConvertColor`.
pub fn convert_color(c: &str) -> String {
    let color = c.trim();
    if color.is_empty() {
        return String::new();
    }
    if let Some(mapped) = claude_to_opencode_color().get(color) {
        return (*mapped).to_string();
    }
    color.to_string()
}

/// Builds an [`OpenCodeAgent`] from raw Claude frontmatter bytes (already
/// extracted + normalized). Shared by [`convert_agent`] and the sync validator.
pub fn build_opencode_agent(frontmatter: &[u8]) -> Result<OpenCodeAgent, Error> {
    let claude_data =
        parse_yaml_value(frontmatter).map_err(|e| anyhow!("failed to parse YAML: {e}"))?;

    let map = match &claude_data {
        YamlValue::Mapping(m) => m,
        _ => &Vec::new(),
    };

    let mut description = String::new();
    let mut model = String::new();
    let mut color = String::new();
    let mut tools_raw: Option<&YamlValue> = None;
    let mut skills: Vec<String> = Vec::new();

    for (k, v) in map {
        match k.as_str() {
            "description" => {
                if let YamlValue::String(s) = v {
                    description.clone_from(s);
                }
            }
            "model" => {
                if let YamlValue::String(s) = v {
                    model.clone_from(s);
                }
            }
            "color" => {
                if let YamlValue::String(s) = v {
                    color.clone_from(s);
                }
            }
            "tools" => tools_raw = Some(v),
            "skills" => {
                if let YamlValue::Sequence(seq) = v {
                    for item in seq {
                        if let YamlValue::String(s) = item {
                            skills.push(s.clone());
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let tools = match tools_raw {
        Some(v) => convert_tools(&parse_claude_tools(v)),
        None => BTreeMap::new(),
    };

    Ok(OpenCodeAgent {
        description,
        model: convert_model(&model),
        tools,
        color: convert_color(&color),
        skills,
    })
}

/// Converts a single Claude agent file to OpenCode format. Mirrors Go
/// `ConvertAgent`. When `dry_run` is true, performs the full transform but
/// writes nothing.
pub fn convert_agent(input_path: &Path, output_path: &Path, dry_run: bool) -> Result<(), Error> {
    let content = std::fs::read(input_path).map_err(|e| anyhow!("failed to read file: {e}"))?;

    let (frontmatter, body) =
        extract_frontmatter(&content).map_err(|e| anyhow!("failed to extract frontmatter: {e}"))?;

    let agent = build_opencode_agent(&frontmatter).map_err(|e| anyhow!("{e}"))?;

    let new_frontmatter = emit_opencode_yaml(&agent);

    // Reconstruct markdown: "---\n" + frontmatter + "---\n" + body.
    let mut output: Vec<u8> = Vec::new();
    output.extend_from_slice(b"---\n");
    output.extend_from_slice(new_frontmatter.as_bytes());
    output.extend_from_slice(b"---\n");
    output.extend_from_slice(&body);

    if !dry_run {
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| anyhow!("failed to create output directory: {e}"))?;
        }
        std::fs::write(output_path, &output).map_err(|e| anyhow!("failed to write file: {e}"))?;
    }

    Ok(())
}

/// Serializes an [`OpenCodeAgent`] to YAML frontmatter (without the `---`
/// fences), byte-identical to Go's `yaml.v3` encoder with `SetIndent(2)`.
///
/// Emission order is `description`, `model`, `tools`, `color`, `skills`.
/// `color` and `skills` are omitted entirely when empty (Go `omitempty`). The
/// `tools` map keys are already sorted (BTreeMap). A trailing newline
/// terminates the document, as yaml.v3 produces.
pub fn emit_opencode_yaml(agent: &OpenCodeAgent) -> String {
    let mut out = String::new();

    out.push_str("description: ");
    out.push_str(&emit_scalar(&agent.description));
    out.push('\n');

    out.push_str("model: ");
    out.push_str(&emit_scalar(&agent.model));
    out.push('\n');

    if agent.tools.is_empty() {
        // yaml.v3 emits an empty map as `tools: {}`.
        out.push_str("tools: {}\n");
    } else {
        out.push_str("tools:\n");
        for (k, v) in &agent.tools {
            out.push_str("  ");
            out.push_str(&emit_scalar(k));
            out.push_str(": ");
            out.push_str(if *v { "true" } else { "false" });
            out.push('\n');
        }
    }

    if !agent.color.is_empty() {
        out.push_str("color: ");
        out.push_str(&emit_scalar(&agent.color));
        out.push('\n');
    }

    if !agent.skills.is_empty() {
        out.push_str("skills:\n");
        for skill in &agent.skills {
            out.push_str("  - ");
            out.push_str(&emit_scalar(skill));
            out.push('\n');
        }
    }

    out
}

/// Emits a YAML scalar exactly as `gopkg.in/yaml.v3` would: a plain (unquoted)
/// scalar when safe, otherwise a double-quoted scalar. yaml.v3 does not fold
/// long plain scalars, so the entire value stays on one line.
fn emit_scalar(s: &str) -> String {
    if needs_quoting(s) {
        emit_double_quoted(s)
    } else {
        s.to_string()
    }
}

/// Mirrors `gopkg.in/yaml.v3`'s decision to quote a plain scalar. Returns true
/// when the value cannot be safely emitted as a plain scalar in block context.
fn needs_quoting(s: &str) -> bool {
    if s.is_empty() {
        // yaml.v3 emits an empty string as `""`.
        return true;
    }

    // Values that would be reinterpreted as non-string types on reparse.
    if resembles_non_string(s) {
        return true;
    }

    let bytes = s.as_bytes();
    let first = bytes[0];
    let last = bytes[s.len() - 1];

    // Leading/trailing whitespace is not preserved by a plain scalar.
    if first == b' ' || last == b' ' {
        return true;
    }

    // Indicator characters that cannot start a plain scalar.
    // (`-`, `?`, `:` are only indicators when followed by space, but yaml.v3
    // conservatively quotes when they start the value; the others are always
    // indicators.)
    match first {
        b'!' | b'&' | b'*' | b'#' | b'|' | b'>' | b'@' | b'`' | b'"' | b'\'' | b'%' | b'['
        | b']' | b'{' | b'}' | b',' => return true,
        b'-' | b'?' | b':'
            // Indicator only when followed by a space or end-of-string.
            if (s.len() == 1 || bytes[1] == b' ') => {
                return true;
            }
        _ => {}
    }

    // Embedded sequences that break a plain scalar: `: ` (mapping indicator),
    // ` #` (comment indicator), control chars, tabs, or newlines.
    let chars: Vec<char> = s.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        match c {
            '\n' | '\t' => return true,
            ':'
                // `:` followed by space (or at end) is a mapping indicator.
                if (i + 1 >= chars.len() || chars[i + 1] == ' ') => {
                    return true;
                }
            '#'
                // ` #` (space then hash) is a comment indicator.
                if i > 0 && chars[i - 1] == ' ' => {
                    return true;
                }
            c if (c as u32) < 0x20 => return true,
            _ => {}
        }
    }

    false
}

/// Returns true when a plain scalar would be reparsed as a bool, null, int,
/// float, or other non-string YAML type — matching yaml.v3's resolver. Such
/// values must be quoted to round-trip as strings.
fn resembles_non_string(s: &str) -> bool {
    // yaml.v3 (YAML 1.1 core resolver) treats these as bool/null.
    matches!(
        s,
        "true"
            | "True"
            | "TRUE"
            | "false"
            | "False"
            | "FALSE"
            | "yes"
            | "Yes"
            | "YES"
            | "no"
            | "No"
            | "NO"
            | "on"
            | "On"
            | "ON"
            | "off"
            | "Off"
            | "OFF"
            | "null"
            | "Null"
            | "NULL"
            | "~"
            | ".inf"
            | "-.inf"
            | ".nan"
    ) || looks_like_number(s)
}

/// Returns true when `s` parses as a YAML int or float (so it must be quoted to
/// stay a string).
fn looks_like_number(s: &str) -> bool {
    if s.parse::<i64>().is_ok() {
        return true;
    }
    if s.parse::<f64>().is_ok() {
        // Reject lone "." / "-" style false positives Rust would not parse,
        // but `parse::<f64>` already handles those.
        return true;
    }
    false
}

/// Emits a YAML double-quoted scalar, escaping the characters yaml.v3 escapes.
fn emit_double_quoted(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            '\0' => out.push_str("\\0"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\x{:02X}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Converts every Claude agent in `.claude/agents/` to the canonical plural
/// OpenCode directory. Mirrors Go `ConvertAllAgents`. Returns
/// `(converted, failed, failed_files)`.
pub fn convert_all_agents(
    repo_root: &Path,
    dry_run: bool,
) -> Result<(i64, i64, Vec<String>), Error> {
    let claude_agents_dir = repo_root.join(".claude").join("agents");
    let opencode_agent_dir = repo_root.join(OPENCODE_AGENT_DIR);

    let entries = std::fs::read_dir(&claude_agents_dir)
        .map_err(|e| anyhow!("failed to read .claude/agents directory: {e}"))?;

    // Collect names first so iteration order matches Go's os.ReadDir (sorted).
    let mut names: Vec<String> = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| anyhow!("failed to read .claude/agents directory: {e}"))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let file_type = entry
            .file_type()
            .map_err(|e| anyhow!("failed to read .claude/agents directory: {e}"))?;
        if file_type.is_dir() || !name.ends_with(".md") {
            continue;
        }
        if name == "README.md" {
            continue;
        }
        names.push(name);
    }
    names.sort();

    let mut converted = 0i64;
    let mut failed = 0i64;
    let mut failed_files: Vec<String> = Vec::new();

    for name in names {
        let input_path = claude_agents_dir.join(&name);
        let output_path = opencode_agent_dir.join(&name);
        if convert_agent(&input_path, &output_path, dry_run).is_err() {
            failed += 1;
            failed_files.push(name);
        } else {
            converted += 1;
        }
    }

    Ok((converted, failed, failed_files))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn convert_model_haiku_to_glm() {
        assert_eq!(convert_model("haiku"), "opencode-go/glm-5");
    }

    #[test]
    fn convert_model_others_to_minimax() {
        assert_eq!(convert_model(""), "opencode-go/minimax-m2.7");
        assert_eq!(convert_model("sonnet"), "opencode-go/minimax-m2.7");
        assert_eq!(convert_model("opus"), "opencode-go/minimax-m2.7");
        assert_eq!(convert_model("  opus  "), "opencode-go/minimax-m2.7");
        assert_eq!(convert_model("unknown"), "opencode-go/minimax-m2.7");
    }

    #[test]
    fn convert_tools_lowercases_and_sorts() {
        let tools = convert_tools(&["Read".to_string(), "Write".to_string(), "Bash".to_string()]);
        let keys: Vec<&String> = tools.keys().collect();
        assert_eq!(keys, vec!["bash", "read", "write"]);
        assert!(tools.values().all(|v| *v));
    }

    #[test]
    fn convert_tools_skips_empty() {
        let tools = convert_tools(&[String::new(), "  ".to_string(), "Read".to_string()]);
        assert_eq!(tools.len(), 1);
        assert!(tools.contains_key("read"));
    }

    #[test]
    fn convert_color_translates_known() {
        assert_eq!(convert_color("blue"), "primary");
        assert_eq!(convert_color("green"), "success");
        assert_eq!(convert_color("yellow"), "warning");
        assert_eq!(convert_color("purple"), "secondary");
        assert_eq!(convert_color("red"), "error");
        assert_eq!(convert_color("orange"), "warning");
        assert_eq!(convert_color("pink"), "accent");
        assert_eq!(convert_color("cyan"), "info");
    }

    #[test]
    fn convert_color_passes_through_unknown_and_empty() {
        assert_eq!(convert_color("primary"), "primary");
        assert_eq!(convert_color("#ff00aa"), "#ff00aa");
        assert_eq!(convert_color(""), "");
        assert_eq!(convert_color("  "), "");
    }

    #[test]
    fn emit_yaml_with_color() {
        let agent = OpenCodeAgent {
            description: "X".to_string(),
            model: "opencode-go/minimax-m2.7".to_string(),
            tools: convert_tools(&["Read".to_string()]),
            color: "primary".to_string(),
            skills: vec!["a-skill".to_string()],
        };
        let yaml = emit_opencode_yaml(&agent);
        assert_eq!(
            yaml,
            "description: X\nmodel: opencode-go/minimax-m2.7\ntools:\n  read: true\ncolor: primary\nskills:\n  - a-skill\n"
        );
    }

    #[test]
    fn emit_yaml_plain_scalars_no_skills() {
        let agent = OpenCodeAgent {
            description: "Does a thing. Use when needed.".to_string(),
            model: "opencode-go/minimax-m2.7".to_string(),
            tools: convert_tools(&["Read".to_string(), "Write".to_string()]),
            color: String::new(),
            skills: vec![],
        };
        let yaml = emit_opencode_yaml(&agent);
        assert_eq!(
            yaml,
            "description: Does a thing. Use when needed.\n\
             model: opencode-go/minimax-m2.7\n\
             tools:\n  read: true\n  write: true\n"
        );
    }

    #[test]
    fn emit_yaml_with_skills() {
        let agent = OpenCodeAgent {
            description: "X".to_string(),
            model: "opencode-go/glm-5".to_string(),
            tools: convert_tools(&["Grep".to_string()]),
            color: String::new(),
            skills: vec!["a-skill".to_string(), "b-skill".to_string()],
        };
        let yaml = emit_opencode_yaml(&agent);
        assert_eq!(
            yaml,
            "description: X\nmodel: opencode-go/glm-5\ntools:\n  grep: true\nskills:\n  - a-skill\n  - b-skill\n"
        );
    }

    #[test]
    fn emit_yaml_empty_tools() {
        let agent = OpenCodeAgent {
            description: "X".to_string(),
            model: "opencode-go/minimax-m2.7".to_string(),
            tools: BTreeMap::new(),
            color: String::new(),
            skills: vec![],
        };
        let yaml = emit_opencode_yaml(&agent);
        assert_eq!(
            yaml,
            "description: X\nmodel: opencode-go/minimax-m2.7\ntools: {}\n"
        );
    }

    #[test]
    fn emit_scalar_does_not_fold_long_strings() {
        let long = "a ".repeat(200);
        let long = long.trim_end();
        let agent = OpenCodeAgent {
            description: long.to_string(),
            model: "opencode-go/minimax-m2.7".to_string(),
            tools: convert_tools(&["Read".to_string()]),
            color: String::new(),
            skills: vec![],
        };
        let yaml = emit_opencode_yaml(&agent);
        // The whole description stays on one line (no folding).
        let first_line = yaml.lines().next().unwrap();
        assert_eq!(first_line, format!("description: {long}"));
    }

    #[test]
    fn needs_quoting_bools_and_numbers() {
        assert!(needs_quoting("true"));
        assert!(needs_quoting("false"));
        assert!(needs_quoting("yes"));
        assert!(needs_quoting("null"));
        assert!(needs_quoting("42"));
        assert!(needs_quoting("3.14"));
        assert!(needs_quoting(""));
        assert!(!needs_quoting("hello world"));
        assert!(!needs_quoting("opencode-go/minimax-m2.7"));
    }

    #[test]
    fn needs_quoting_indicator_and_embedded() {
        assert!(needs_quoting("- leading dash space"));
        assert!(needs_quoting("key: value")); // embedded colon-space
        assert!(needs_quoting("text # comment")); // embedded space-hash
        assert!(needs_quoting("@at-start"));
        assert!(needs_quoting(" leading space"));
        assert!(needs_quoting("trailing space "));
        assert!(!needs_quoting("a-b:c")); // colon not followed by space → plain ok
    }

    #[test]
    fn emit_double_quoted_escapes() {
        let agent = OpenCodeAgent {
            description: "has \"quotes\" and: colon".to_string(),
            model: "opencode-go/minimax-m2.7".to_string(),
            tools: convert_tools(&["Read".to_string()]),
            color: String::new(),
            skills: vec![],
        };
        let yaml = emit_opencode_yaml(&agent);
        assert!(yaml.starts_with("description: \"has \\\"quotes\\\" and: colon\"\n"));
    }

    #[test]
    fn build_opencode_agent_from_string_tools() {
        let fm = b"name: foo-maker\ndescription: A maker.\ntools: Read, Write\nmodel: sonnet\ncolor: blue\nskills:\n  - s1\n  - s2\n";
        let agent = build_opencode_agent(fm).unwrap();
        assert_eq!(agent.description, "A maker.");
        assert_eq!(agent.model, "opencode-go/minimax-m2.7");
        assert_eq!(agent.color, "primary");
        let keys: Vec<&String> = agent.tools.keys().collect();
        assert_eq!(keys, vec!["read", "write"]);
        assert_eq!(agent.skills, vec!["s1", "s2"]);
    }

    #[test]
    fn build_opencode_agent_array_tools_and_haiku() {
        let fm = b"name: foo\ndescription: d\ntools:\n  - Glob\n  - Grep\nmodel: haiku\n";
        let agent = build_opencode_agent(fm).unwrap();
        assert_eq!(agent.model, "opencode-go/glm-5");
        let keys: Vec<&String> = agent.tools.keys().collect();
        assert_eq!(keys, vec!["glob", "grep"]);
        assert!(agent.skills.is_empty());
    }

    #[test]
    fn convert_agent_writes_file() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("in.md");
        let output = dir.path().join("out/in.md");
        std::fs::write(
            &input,
            "---\nname: foo-maker\ndescription: A maker.\ntools: Read\nmodel:\ncolor: blue\n---\n# Body\ntext\n",
        )
        .unwrap();
        convert_agent(&input, &output, false).unwrap();
        let written = std::fs::read_to_string(&output).unwrap();
        assert_eq!(
            written,
            "---\ndescription: A maker.\nmodel: opencode-go/minimax-m2.7\ntools:\n  read: true\ncolor: primary\n---\n# Body\ntext\n"
        );
    }

    #[test]
    fn convert_agent_dry_run_writes_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("in.md");
        let output = dir.path().join("out/in.md");
        std::fs::write(
            &input,
            "---\nname: foo\ndescription: d\ntools: Read\nmodel:\ncolor: blue\n---\nbody\n",
        )
        .unwrap();
        convert_agent(&input, &output, true).unwrap();
        assert!(!output.exists());
    }

    #[test]
    fn convert_agent_read_error() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("nope.md");
        let out = dir.path().join("out.md");
        let err = convert_agent(&missing, &out, false).unwrap_err();
        assert!(err.to_string().contains("failed to read file"));
    }

    #[test]
    fn convert_all_agents_skips_readme_and_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let claude = dir.path().join(".claude/agents");
        std::fs::create_dir_all(&claude).unwrap();
        std::fs::write(
            claude.join("foo-maker.md"),
            "---\nname: foo-maker\ndescription: d\ntools: Read\nmodel:\ncolor: blue\n---\nbody\n",
        )
        .unwrap();
        std::fs::write(claude.join("README.md"), "# readme\n").unwrap();
        std::fs::create_dir_all(claude.join("subdir")).unwrap();
        let (converted, failed, files) = convert_all_agents(dir.path(), false).unwrap();
        assert_eq!(converted, 1);
        assert_eq!(failed, 0);
        assert!(files.is_empty());
        assert!(dir.path().join(".opencode/agents/foo-maker.md").exists());
    }

    #[test]
    fn convert_all_agents_counts_failures() {
        let dir = tempfile::tempdir().unwrap();
        let claude = dir.path().join(".claude/agents");
        std::fs::create_dir_all(&claude).unwrap();
        // A file with no frontmatter fences → extract_frontmatter fails.
        std::fs::write(claude.join("broken-maker.md"), "no frontmatter here\n").unwrap();
        let (converted, failed, files) = convert_all_agents(dir.path(), false).unwrap();
        assert_eq!(converted, 0);
        assert_eq!(failed, 1);
        assert_eq!(files, vec!["broken-maker.md"]);
    }

    #[test]
    fn convert_all_agents_missing_dir_errors() {
        let dir = tempfile::tempdir().unwrap();
        let err = convert_all_agents(dir.path(), false).unwrap_err();
        assert!(
            err.to_string()
                .contains("failed to read .claude/agents directory")
        );
    }
}
