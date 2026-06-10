//! Vendor-independence audit scanner.
//!
//! The scanner detects forbidden vendor-specific terms in prose and reports
//! them with suggested replacements. Several regions are exempt: code fences
//! (all backtick-delimited blocks), YAML frontmatter, multi-line HTML comments,
//! inline code spans, link URL portions, and sections under "Platform Binding
//! Examples" headings. The governance-vendor-independence.md convention
//! definition file is skipped entirely (it names the terms by design).

use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use anyhow::{Context, Error};
use regex::Regex;
use walkdir::WalkDir;

/// A single vendor-term violation found in a file. This struct is internal and
/// not serialized directly; the command's own DTO lower-cases the JSON keys
/// (`path`, `line`, `match`, `replacement`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub path: String,
    pub line: usize,
    pub r#match: String,
    pub replacement: String,
}

/// Path suffix of the convention definition file, which is exempt from
/// scanning.
const FORBIDDEN_CONVENTION_SUFFIX: &str =
    "repo-governance/conventions/structure/governance-vendor-independence.md";

struct ForbiddenTerm {
    re: Regex,
    display_term: &'static str,
    replacement: &'static str,
}

fn mk(pattern: &str, term: &'static str, replacement: &'static str) -> ForbiddenTerm {
    ForbiddenTerm {
        re: Regex::new(pattern).expect("valid hardcoded regex"),
        display_term: term,
        replacement,
    }
}

/// Forbidden patterns mapped to suggested replacements. Order matters: longer /
/// more-specific patterns appear before shorter overlapping ones.
/// strings).
fn forbidden_terms() -> &'static Vec<ForbiddenTerm> {
    static TERMS: OnceLock<Vec<ForbiddenTerm>> = OnceLock::new();
    TERMS.get_or_init(|| {
        let mut v = forbidden_agent_names();
        v.extend(forbidden_binding_dirs());
        v.extend(forbidden_model_names());
        v
    })
}

/// Harness / coding-agent product and vendor names.
fn forbidden_agent_names() -> Vec<ForbiddenTerm> {
    vec![
        mk(r"Claude Code", "Claude Code", "\"the coding agent\""),
        mk(
            r"OpenCode",
            "OpenCode",
            "\"the coding agent\" or drop where redundant",
        ),
        mk(
            r"\bCursor\b",
            "Cursor",
            "\"the coding agent\" or \"AI coding editor\"",
        ),
        mk(
            r"\bWindsurf\b",
            "Windsurf",
            "\"the coding agent\" or \"AI coding editor\"",
        ),
        mk(
            r"\bCodeium\b",
            "Codeium",
            "\"the coding agent\" (legacy Windsurf brand)",
        ),
        mk(
            r"\bCopilot\b",
            "Copilot",
            "\"the coding agent\" or \"AI coding assistant\"",
        ),
        mk(
            r"\bAider\b",
            "Aider",
            "\"the coding agent\" or \"AI coding assistant\"",
        ),
        mk(
            r"\bCline\b",
            "Cline",
            "\"the coding agent\" or \"AI coding assistant\"",
        ),
        mk(
            r"\bDevin\b",
            "Devin",
            "\"the coding agent\" (false-positive risk: personal name; review context)",
        ),
        mk(
            r"\bJunie\b",
            "Junie",
            "\"the coding agent\" or \"AI coding assistant\"",
        ),
        mk(
            r"\bJetBrains\b",
            "JetBrains",
            "\"the coding-agent vendor\" or drop",
        ),
        mk(r"Amazon Q\b", "Amazon Q", "\"the coding agent\""),
        mk(r"\bAntigravity\b", "Antigravity", "\"the coding agent\""),
        mk(
            r"Pi Coding Agent",
            "Pi Coding Agent",
            "\"the coding agent\"",
        ),
        mk(r"pi\.dev", "pi.dev", "\"the coding agent\""),
        mk(
            r"\bEarendil\b",
            "Earendil",
            "\"the coding-agent vendor\" or drop",
        ),
    ]
}

/// Vendor-specific binding directory paths.
fn forbidden_binding_dirs() -> Vec<ForbiddenTerm> {
    vec![
        mk(r"\.claude/", ".claude/", "\"primary binding directory\""),
        mk(
            r"\.opencode/",
            ".opencode/",
            "\"secondary binding directory\"",
        ),
        mk(
            r"\.cursor/",
            ".cursor/",
            "\"the platform binding directory\"",
        ),
        mk(
            r"\.windsurf/",
            ".windsurf/",
            "\"the platform binding directory\"",
        ),
        mk(
            r"\.continue/",
            ".continue/",
            "\"the platform binding directory\"",
        ),
        mk(
            r"\.clinerules/",
            ".clinerules/",
            "\"the platform binding directory\"",
        ),
        mk(r"\.junie/", ".junie/", "\"the platform binding directory\""),
        mk(
            r"\.amazonq/",
            ".amazonq/",
            "\"the platform binding directory\"",
        ),
        mk(r"\.pi/", ".pi/", "\"the platform binding directory\""),
        mk(
            r"\.gemini/",
            ".gemini/",
            "\"the platform binding directory\"",
        ),
        mk(r"\.agent/", ".agent/", "\"the platform binding directory\""),
        mk(
            r"\.agents/",
            ".agents/",
            "\"the platform binding directory\"",
        ),
    ]
}

/// Model-vendor company names, model families, and vendor-branded concepts.
fn forbidden_model_names() -> Vec<ForbiddenTerm> {
    vec![
        mk(r"Anthropic", "Anthropic", "\"the model vendor\" or drop"),
        mk(r"\bOpenAI\b", "OpenAI", "\"the model vendor\" or drop"),
        mk(r"\bxAI\b", "xAI", "\"the model vendor\" or drop"),
        mk(r"\bSonnet\b", "Sonnet", "\"execution-grade\""),
        mk(r"\bOpus\b", "Opus", "\"planning-grade\""),
        mk(r"\bHaiku\b", "Haiku", "\"fast\""),
        mk(r"\bGPT\b", "GPT", "\"AI model\" or capability tier"),
        mk(r"\bGemini\b", "Gemini", "\"AI model\" or capability tier"),
        mk(
            r"\bDeepSeek\b",
            "DeepSeek",
            "\"AI model\" or capability tier",
        ),
        mk(r"\bQwen\b", "Qwen", "\"AI model\" or capability tier"),
        mk(r"\bLlama\b", "Llama", "\"AI model\" or capability tier"),
        mk(r"\bMistral\b", "Mistral", "\"AI model\" or capability tier"),
        mk(
            r"\bGrok\b",
            "Grok",
            "\"AI model\" (false-positive risk: verb \"to grok\"; review context)",
        ),
        mk(r"\bSkills\b", "Skills", "\"agent skills\" (lowercase)"),
    ]
}

fn html_comment_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"<!--.*?-->").expect("valid hardcoded regex"))
}

fn inline_code_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"`[^`]*`").expect("valid hardcoded regex"))
}

fn link_url_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\[([^\]]*)\]\([^)]*\)").expect("valid hardcoded regex"))
}

/// Reads the file at `path` and returns all vendor-term findings.
pub fn scan_file(path: &Path) -> Result<Vec<Finding>, Error> {
    let data = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    Ok(scan_lines(&path.to_string_lossy(), &data))
}

/// Walks all `.md` files under `root` recursively and returns all findings.
/// Skips the governance-vendor-independence.md convention definition file. A
/// missing `root` yields an empty slice, not an error.
///
/// Entries are visited in lexical (sorted) order for deterministic finding
/// ordering.
pub fn walk(root: &Path) -> Result<Vec<Finding>, Error> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut findings = Vec::new();
    for entry in WalkDir::new(root).sort_by_file_name().into_iter().flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".md") {
            continue;
        }
        let p = entry.path();
        let p_slash = p.to_string_lossy().replace('\\', "/");
        if p_slash.ends_with(FORBIDDEN_CONVENTION_SUFFIX) {
            continue;
        }
        findings.extend(scan_file(p)?);
    }
    Ok(findings)
}

/// Core line-by-line scanner, tracking exemption state for code fences, YAML
/// frontmatter, multi-line HTML comments, and Platform Binding Examples
/// headings.
fn scan_lines(path: &str, content: &str) -> Vec<Finding> {
    let lines: Vec<&str> = content.split('\n').collect();
    let mut findings = Vec::new();

    // 0 = not in fence; >0 = fence opened with this backtick count.
    let mut in_code_fence_len: usize = 0;
    let mut in_frontmatter = false;
    let mut in_html_comment = false;
    let mut in_platform_binding_section = false;
    let mut platform_binding_heading_level: usize = 0;

    for (i, line) in lines.iter().enumerate() {
        let line_num = i + 1;

        // YAML frontmatter.
        if line_num == 1 && line.trim() == "---" {
            in_frontmatter = true;
            continue;
        }
        if in_frontmatter {
            if line.trim() == "---" {
                in_frontmatter = false;
            }
            continue;
        }

        // Multi-line HTML comment.
        if in_html_comment {
            if line.contains("-->") {
                in_html_comment = false;
            }
            continue;
        }
        if line.contains("<!--") && !line.contains("-->") {
            in_html_comment = true;
            // Check the portion before <!-- for violations.
            if let Some(idx) = line.find("<!--") {
                let before = &line[..idx];
                let stripped = strip_non_prose(before);
                if !stripped.is_empty() {
                    for ft in forbidden_terms() {
                        if ft.re.is_match(&stripped) {
                            findings.push(Finding {
                                path: path.to_string(),
                                line: line_num,
                                r#match: ft.display_term.to_string(),
                                replacement: ft.replacement.to_string(),
                            });
                        }
                    }
                }
            }
            continue;
        }

        // Code fences (length-aware per CommonMark).
        let fl = fence_line_len(line);
        if fl > 0 {
            if in_code_fence_len == 0 {
                in_code_fence_len = fl;
                continue;
            } else if fl >= in_code_fence_len {
                in_code_fence_len = 0;
                continue;
            }
            // Inner fence line (shorter than opener) — it is content; fall through.
        }

        // Lines inside a code fence are fully exempt.
        if in_code_fence_len > 0 {
            continue;
        }

        // Platform Binding Examples heading scope.
        if let Some(level) = parse_heading(line) {
            if is_platform_binding_heading(line) {
                in_platform_binding_section = true;
                platform_binding_heading_level = level;
                continue;
            }
            // A heading at same or higher level (lower number) ends the section.
            if in_platform_binding_section && level <= platform_binding_heading_level {
                in_platform_binding_section = false;
                platform_binding_heading_level = 0;
            }
        }

        // Lines inside the Platform Binding Examples section are exempt.
        if in_platform_binding_section {
            continue;
        }

        // Scan for forbidden terms.
        let stripped = strip_non_prose(line);
        for ft in forbidden_terms() {
            if ft.re.is_match(&stripped) {
                findings.push(Finding {
                    path: path.to_string(),
                    line: line_num,
                    r#match: ft.display_term.to_string(),
                    replacement: ft.replacement.to_string(),
                });
            }
        }
    }
    findings
}

/// Returns the number of leading backticks on a fence delimiter line (>= 3 to
/// be a valid fence), or 0 if not a fence line.
fn fence_line_len(line: &str) -> usize {
    let trimmed = line.trim();
    let mut n = 0;
    for ch in trimmed.chars() {
        if ch == '`' {
            n += 1;
        } else {
            break;
        }
    }
    if n >= 3 { n } else { 0 }
}

/// Removes regions of a line exempt from scanning: HTML comments, inline code
/// spans, and link URL portions.
fn strip_non_prose(line: &str) -> String {
    let s = html_comment_re().replace_all(line, "");
    let s = inline_code_re().replace_all(&s, "``");
    let s = link_url_re().replace_all(&s, "[$1]");
    s.into_owned()
}

/// Detects ATX headings (`## Heading`) and returns the level. Returns `None` if
/// the line is not a heading.
fn parse_heading(line: &str) -> Option<usize> {
    let trimmed = line.trim();
    if !trimmed.starts_with('#') {
        return None;
    }
    let mut level = 0;
    for ch in trimmed.chars() {
        if ch == '#' {
            level += 1;
        } else {
            break;
        }
    }
    if level > 6 {
        return None;
    }
    // Must be followed by a space (standard ATX heading).
    let bytes = trimmed.as_bytes();
    if bytes.len() <= level || bytes[level] != b' ' {
        return None;
    }
    Some(level)
}

/// Reports whether the heading text contains "Platform Binding Examples"
/// (case-insensitive).
fn is_platform_binding_heading(line: &str) -> bool {
    line.to_lowercase().contains("platform binding examples")
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn detects_forbidden_brand_in_prose() {
        let findings = scan_lines("x.md", "I use Claude Code.\n");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].r#match, "Claude Code");
        assert_eq!(findings[0].line, 1);
        assert_eq!(findings[0].replacement, "\"the coding agent\"");
    }

    #[test]
    fn detects_skills_brand_in_prose() {
        let findings = scan_lines("x.md", "We rely on Skills here.\n");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].r#match, "Skills");
    }

    #[test]
    fn skips_code_fences() {
        let findings = scan_lines("x.md", "```\nClaude Code\n```\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_nested_quad_fences() {
        let findings = scan_lines("x.md", "````md\n```\nClaude Code\n```\n````\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_binding_example_fence() {
        let findings = scan_lines("x.md", "```binding-example\nClaude Code\n```\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_frontmatter() {
        let findings = scan_lines("x.md", "---\ntitle: Claude Code\n---\n\nBody.\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_html_comments() {
        let findings = scan_lines("x.md", "<!-- Claude Code -->\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_multiline_html_comments() {
        let findings = scan_lines("x.md", "<!--\nClaude Code\n-->\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_inline_code_spans() {
        let findings = scan_lines("x.md", "Use `Claude Code` here.\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_link_urls() {
        let findings = scan_lines("x.md", "[link](https://example.com/Claude-Code/foo)\n");
        assert!(findings.is_empty());
    }

    #[test]
    fn skips_platform_binding_section() {
        let content = "# X\n\n## Platform Binding Examples\n\nClaude Code\n\n## Next\n";
        let findings = scan_lines("x.md", content);
        assert!(findings.is_empty());
    }

    #[test]
    fn platform_binding_scope_ends_at_same_level_heading() {
        let content = "## Platform Binding Examples\n\nClaude Code\n## Other\n\nClaude Code\n";
        let findings = scan_lines("x.md", content);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].line, 6);
    }

    #[test]
    fn detects_binding_dir_path() {
        let findings = scan_lines(
            "x.md",
            "Edit `.claude/agents/` is wrong, edit .claude/agents/ instead.\n",
        );
        assert!(findings.iter().any(|f| f.r#match == ".claude/"));
    }

    #[test]
    fn prose_before_inline_comment_scanned() {
        let findings = scan_lines("x.md", "Claude Code is bad <!-- multi\n");
        assert!(findings.iter().any(|f| f.r#match == "Claude Code"));
    }

    #[test]
    fn detects_new_harness_brands_in_prose() {
        for (text, term) in [
            ("We use Junie daily.\n", "Junie"),
            ("JetBrains ships it.\n", "JetBrains"),
            ("Amazon Q is here.\n", "Amazon Q"),
            ("Amazon Q Developer ships.\n", "Amazon Q"),
            ("Antigravity flies.\n", "Antigravity"),
            ("The Pi Coding Agent helps.\n", "Pi Coding Agent"),
            ("See pi.dev for docs.\n", "pi.dev"),
            ("Earendil builds it.\n", "Earendil"),
        ] {
            let findings = scan_lines("x.md", text);
            assert!(
                findings.iter().any(|f| f.r#match == term),
                "expected {term} flagged in {text:?}, got {findings:?}"
            );
        }
    }

    #[test]
    fn detects_new_binding_dir_paths() {
        for (text, term) in [
            ("Edit .junie/ here.\n", ".junie/"),
            ("Edit .amazonq/ here.\n", ".amazonq/"),
            ("Edit .pi/ here.\n", ".pi/"),
            ("Edit .gemini/ here.\n", ".gemini/"),
            ("Edit .agent/ here.\n", ".agent/"),
            ("Edit .agents/ here.\n", ".agents/"),
        ] {
            let findings = scan_lines("x.md", text);
            assert!(
                findings.iter().any(|f| f.r#match == term),
                "expected {term} flagged, got {findings:?}"
            );
        }
    }

    #[test]
    fn fp_safe_ignores_math_pi_and_bare_q() {
        // The math constant "pi" and a lone capital "Q" must not be flagged.
        let findings = scan_lines("x.md", "The value of pi is 3.14 and Q is a letter.\n");
        assert!(
            findings.is_empty(),
            "expected no findings, got {findings:?}"
        );
    }

    #[test]
    fn fp_safe_skips_new_brands_in_platform_binding_section() {
        let content = "## Platform Binding Examples\n\nJunie and Amazon Q and Antigravity.\n";
        let findings = scan_lines("x.md", content);
        assert!(
            findings.is_empty(),
            "expected no findings, got {findings:?}"
        );
    }

    #[test]
    fn agent_path_pattern_does_not_match_agents_path() {
        // ".agents/" is reported as `.agents/` only, never also as `.agent/`.
        let findings = scan_lines("x.md", "Edit .agents/ dir.\n");
        let matches: Vec<&str> = findings.iter().map(|f| f.r#match.as_str()).collect();
        assert!(matches.contains(&".agents/"), "got {matches:?}");
        assert!(!matches.contains(&".agent/"), "got {matches:?}");
    }

    #[test]
    fn skip_convention_definition_file() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join(FORBIDDEN_CONVENTION_SUFFIX);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, "Claude Code\n").unwrap();
        let findings = walk(tmp.path()).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn walk_missing_root_is_empty() {
        let tmp = TempDir::new().unwrap();
        let missing = tmp.path().join("does-not-exist");
        let findings = walk(&missing).unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn walk_finds_md_only() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("a.md"), "Claude Code\n").unwrap();
        fs::write(tmp.path().join("b.txt"), "Claude Code\n").unwrap();
        let findings = walk(tmp.path()).unwrap();
        assert_eq!(findings.len(), 1);
        assert!(findings[0].path.ends_with("a.md"));
    }

    #[test]
    fn parse_heading_recognises_atx_levels() {
        assert_eq!(parse_heading("## Foo"), Some(2));
        assert_eq!(parse_heading("### Bar"), Some(3));
        assert_eq!(parse_heading("####### Too deep"), None);
        assert_eq!(parse_heading("##NoSpace"), None);
        assert_eq!(parse_heading("Foo"), None);
    }

    #[test]
    fn fence_line_len_counts_backticks() {
        assert_eq!(fence_line_len("```"), 3);
        assert_eq!(fence_line_len("```js"), 3);
        assert_eq!(fence_line_len("````"), 4);
        assert_eq!(fence_line_len("``"), 0);
        assert_eq!(fence_line_len("text"), 0);
    }

    #[test]
    fn is_platform_binding_heading_case_insensitive() {
        assert!(is_platform_binding_heading("## PLATFORM BINDING EXAMPLES"));
        assert!(is_platform_binding_heading(
            "### platform binding examples (x)"
        ));
        assert!(!is_platform_binding_heading("## Other"));
    }

    #[test]
    fn scan_file_reads_disk() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("doc.md");
        fs::write(&p, "Anthropic ships models.\n").unwrap();
        let findings = scan_file(&p).unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].r#match, "Anthropic");
    }
}
