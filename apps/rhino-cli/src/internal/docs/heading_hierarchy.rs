//! Heading-hierarchy validation over prose markdown trees (Gate C).
//!
//! Greenfield validator (plan DD-7): reports `missing-h1`, `duplicate-h1`,
//! and `skipped-level` findings. File selection is **allowlist default-deny**
//! — only prose trees (`docs/`, `repo-governance/`, `specs/`, `plans/` minus
//! `plans/done/`, root-level `*.md`, `apps|libs/<name>/README.md`, and
//! `apps|libs/<name>/docs/**`) are ever scanned, so prompt/skill artifacts
//! (`.claude/**`, `.opencode/**`, deep `apps/`/`libs/` internals) can never
//! trip a finding, regardless of caller. Reuses fence-aware
//! [`super::headings::collect_atx_headings`] so headings inside code fences
//! are ignored.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Error};
use serde::Serialize;

use super::headings::collect_atx_headings;
use super::scanner::get_all_markdown_files;

/// Kind of heading-hierarchy finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadingFindingKind {
    /// File has zero H1 headings.
    MissingH1,
    /// File has more than one H1 heading.
    DuplicateH1,
    /// Heading level increases by more than one (e.g. `#` → `###`).
    SkippedLevel,
}

impl HeadingFindingKind {
    /// Stable kind string used in reports: `missing-h1`, `duplicate-h1`,
    /// or `skipped-level`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingH1 => "missing-h1",
            Self::DuplicateH1 => "duplicate-h1",
            Self::SkippedLevel => "skipped-level",
        }
    }
}

/// A single heading-hierarchy finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadingFinding {
    /// File containing the finding (relative to repo root).
    pub file: String,
    /// Line number of the offending heading (1-based; `1` for `missing-h1`).
    pub line: usize,
    /// Finding kind.
    pub kind: HeadingFindingKind,
    /// Human-readable description.
    pub message: String,
}

/// Configures a heading-hierarchy scan.
#[derive(Debug, Clone, Default)]
pub struct HeadingScanOptions {
    /// Absolute path to repository root.
    pub root: PathBuf,
    /// Explicit repo-relative paths to scan instead of the full allowlist
    /// walk (e.g. staged files). The allowlist predicate still applies.
    pub paths: Vec<String>,
    /// Repo-relative path prefixes subtracted on top of the allowlist
    /// (`--exclude` semantics).
    pub exclude: Vec<String>,
}

/// Returns `true` ONLY for prose-allowlisted repo-relative paths:
///
/// - `docs/**`, `repo-governance/**`, `specs/**`
/// - `plans/**` EXCEPT `plans/done/**`
/// - root-level `*.md` (no `/` in the repo-relative path)
/// - `apps/<name>/README.md`, `libs/<name>/README.md`
/// - `apps/<name>/docs/**`, `libs/<name>/docs/**`
///
/// Everything else is default-denied — in particular `.claude/**`,
/// `.opencode/**`, and deep `apps/`/`libs/` internals.
pub fn is_prose_allowlisted(repo_rel: &str) -> bool {
    // Whole prose trees.
    if repo_rel.starts_with("docs/")
        || repo_rel.starts_with("repo-governance/")
        || repo_rel.starts_with("specs/")
    {
        return true;
    }

    // plans/ minus the frozen plans/done/ archive.
    if repo_rel.starts_with("plans/") {
        return !repo_rel.starts_with("plans/done/");
    }

    // Root-level *.md (no `/` in the repo-relative path).
    if !repo_rel.contains('/') {
        return repo_rel.ends_with(".md");
    }

    // apps/<name>/README.md, libs/<name>/README.md,
    // apps/<name>/docs/**, libs/<name>/docs/**.
    let parts: Vec<&str> = repo_rel.split('/').collect();
    if (parts[0] == "apps" || parts[0] == "libs") && parts.len() >= 3 {
        if parts.len() == 3 && parts[2] == "README.md" {
            return true;
        }
        if parts.len() >= 4 && parts[2] == "docs" {
            return true;
        }
    }

    false
}

/// Returns `true` when `repo_rel` starts with any `--exclude` prefix, raw or
/// trailing-slash-trimmed — the same practical semantics as the link
/// scanner's `filter_skip_paths`. Deliberately a self-contained string
/// predicate rather than a shared helper: the scanner's filter is
/// `PathBuf`-shaped, while this validator operates on repo-relative strings.
fn is_excluded(repo_rel: &str, exclude: &[String]) -> bool {
    exclude.iter().any(|prefix| {
        let cleaned = prefix.trim_end_matches('/');
        repo_rel.starts_with(prefix.as_str())
            || (!cleaned.is_empty() && repo_rel.starts_with(cleaned))
    })
}

/// Scans markdown files under `opts.root` (or `opts.paths` when non-empty),
/// keeping only `is_prose_allowlisted` survivors minus `opts.exclude`
/// prefixes, and returns all heading-hierarchy findings in discovery order.
pub fn validate_heading_hierarchy(opts: &HeadingScanOptions) -> Result<Vec<HeadingFinding>, Error> {
    let mut findings = Vec::new();
    for rel in collect_candidate_rels(opts) {
        let content = std::fs::read_to_string(opts.root.join(&rel))
            .with_context(|| format!("failed to read {rel}"))?;
        findings.extend(validate_content(&rel, &content));
    }
    Ok(findings)
}

/// Resolves the candidate repo-relative markdown paths: the full repo-wide
/// walk (shared with the link scanner, noise dirs skipped) when `opts.paths`
/// is empty, otherwise the explicit paths (directories are walked). The
/// allowlist and `--exclude` prefixes apply to BOTH modes.
fn collect_candidate_rels(opts: &HeadingScanOptions) -> Vec<String> {
    let to_rel = |abs: &Path| -> Option<String> {
        abs.strip_prefix(&opts.root)
            .ok()
            .map(|rel| rel.to_string_lossy().into_owned())
    };

    let mut rels: Vec<String> = Vec::new();
    if opts.paths.is_empty() {
        for abs in get_all_markdown_files(&opts.root) {
            rels.extend(to_rel(&abs));
        }
    } else {
        for p in &opts.paths {
            let abs = if Path::new(p).is_absolute() {
                PathBuf::from(p)
            } else {
                opts.root.join(p)
            };
            if abs.is_dir() {
                for f in get_all_markdown_files(&abs) {
                    rels.extend(to_rel(&f));
                }
            } else {
                rels.push(to_rel(&abs).unwrap_or_else(|| p.clone()));
            }
        }
    }

    rels.retain(|rel| {
        rel.ends_with(".md") && is_prose_allowlisted(rel) && !is_excluded(rel, &opts.exclude)
    });
    rels
}

/// Validates one file's heading sequence: zero H1s → `missing-h1` (line 1),
/// every H1 after the first → `duplicate-h1`, and any heading whose level
/// increases by more than one over the previous heading → `skipped-level`.
/// The first heading in a file never produces `skipped-level` on its own.
fn validate_content(file: &str, content: &str) -> Vec<HeadingFinding> {
    let mut first_h1: Option<usize> = None;
    let mut prev_level: Option<usize> = None;
    let mut per_heading = Vec::new();

    for (line, level, title) in collect_atx_headings(content) {
        if level == 1 {
            if let Some(first_line) = first_h1 {
                per_heading.push(HeadingFinding {
                    file: file.to_string(),
                    line,
                    kind: HeadingFindingKind::DuplicateH1,
                    message: format!("duplicate H1 \"{title}\" (first H1 at line {first_line})"),
                });
            } else {
                first_h1 = Some(line);
            }
        }
        if let Some(prev) = prev_level
            && level > prev + 1
        {
            per_heading.push(HeadingFinding {
                file: file.to_string(),
                line,
                kind: HeadingFindingKind::SkippedLevel,
                message: format!("heading level jumps from H{prev} to H{level} at \"{title}\""),
            });
        }
        prev_level = Some(level);
    }

    let mut findings = Vec::new();
    if first_h1.is_none() {
        findings.push(HeadingFinding {
            file: file.to_string(),
            line: 1,
            kind: HeadingFindingKind::MissingH1,
            message: "file has no H1 heading".to_string(),
        });
    }
    findings.extend(per_heading);
    findings
}

// ---------------------------------------------------------------------------
// Report formatting (mirrors the link reporter's text/json/markdown trio)
// ---------------------------------------------------------------------------

/// Formats findings as human-readable text. Findings are already in
/// discovery order; consecutive findings for the same file share a section.
pub fn format_heading_text(findings: &[HeadingFinding], quiet: bool) -> String {
    let mut out = String::new();

    if findings.is_empty() {
        if !quiet {
            out.push_str("✓ All heading hierarchies valid! No findings found.\n");
        }
        return out;
    }

    out.push_str("# Heading Hierarchy Report\n\n");
    let _ = writeln!(out, "**Total findings**: {}", findings.len());

    let mut current_file: Option<&str> = None;
    for finding in findings {
        if current_file != Some(finding.file.as_str()) {
            let _ = write!(out, "\n## {}\n\n", finding.file);
            current_file = Some(finding.file.as_str());
        }
        let _ = writeln!(
            out,
            "- Line {}: {}: {}",
            finding.line,
            finding.kind.as_str(),
            finding.message
        );
    }

    out
}

/// JSON output shape for `-o json`.
#[derive(Serialize)]
struct HeadingJsonOutput {
    status: &'static str,
    total_findings: usize,
    findings: Vec<HeadingJsonFinding>,
}

/// JSON finding shape.
#[derive(Serialize)]
struct HeadingJsonFinding {
    file: String,
    line: usize,
    kind: &'static str,
    message: String,
}

/// Formats findings as JSON (HTML-escaped, matching the link reporter's
/// output shape).
pub fn format_heading_json(findings: &[HeadingFinding]) -> Result<String, Error> {
    let status = if findings.is_empty() {
        "success"
    } else {
        "failure"
    };
    let out = HeadingJsonOutput {
        status,
        total_findings: findings.len(),
        findings: findings
            .iter()
            .map(|f| HeadingJsonFinding {
                file: f.file.clone(),
                line: f.line,
                kind: f.kind.as_str(),
                message: f.message.clone(),
            })
            .collect(),
    };
    Ok(crate::internal::cliout::gojson::html_escape(
        &serde_json::to_string_pretty(&out)?,
    ))
}

/// Markdown delegates to text — the text format is already markdown-compatible.
pub fn format_heading_markdown(findings: &[HeadingFinding]) -> String {
    format_heading_text(findings, false)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use std::path::Path;

    use super::*;

    /// Writes `content` to `root/rel`, creating parent directories.
    fn write(root: &Path, rel: &str, content: &str) {
        let path = root.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    fn scan(root: &Path) -> Vec<HeadingFinding> {
        let opts = HeadingScanOptions {
            root: root.to_path_buf(),
            ..Default::default()
        };
        validate_heading_hierarchy(&opts).unwrap()
    }

    // --- Finding-kind strings ---

    #[test]
    fn finding_kind_as_str_matches_spec() {
        assert_eq!(HeadingFindingKind::MissingH1.as_str(), "missing-h1");
        assert_eq!(HeadingFindingKind::DuplicateH1.as_str(), "duplicate-h1");
        assert_eq!(HeadingFindingKind::SkippedLevel.as_str(), "skipped-level");
    }

    // --- (a) duplicate H1 in docs/ ---

    #[test]
    fn docs_file_with_two_h1s_reports_duplicate_h1() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "docs/guide.md",
            "# First Title\n\ntext\n\n# Second Title\n",
        );

        let findings = scan(dir.path());

        assert_eq!(findings.len(), 1, "expected one finding: {findings:?}");
        assert_eq!(findings[0].kind, HeadingFindingKind::DuplicateH1);
        assert_eq!(findings[0].file, "docs/guide.md");
        assert_eq!(
            findings[0].line, 5,
            "duplicate-h1 reports the second H1's line"
        );
        assert!(!findings[0].message.is_empty());
    }

    // --- (b) missing H1 in docs/ ---

    #[test]
    fn docs_file_with_zero_h1s_reports_missing_h1() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "docs/notes.md", "## Only A Section\n\ntext\n");

        let findings = scan(dir.path());

        assert_eq!(findings.len(), 1, "expected one finding: {findings:?}");
        assert_eq!(findings[0].kind, HeadingFindingKind::MissingH1);
        assert_eq!(findings[0].file, "docs/notes.md");
        assert_eq!(findings[0].line, 1);
    }

    // --- (c) skipped level in docs/ ---

    #[test]
    fn docs_file_jumping_h1_to_h3_reports_skipped_level() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "docs/jump.md", "# Title\n\n### Jumped Here\n");

        let findings = scan(dir.path());

        assert_eq!(findings.len(), 1, "expected one finding: {findings:?}");
        assert_eq!(findings[0].kind, HeadingFindingKind::SkippedLevel);
        assert_eq!(findings[0].file, "docs/jump.md");
        assert_eq!(
            findings[0].line, 3,
            "skipped-level reports the jumping heading's line"
        );
    }

    // --- (d) headings inside code fences are ignored ---

    #[test]
    fn headings_inside_code_fences_produce_no_findings() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "docs/fenced.md",
            "# Title\n\n```bash\n# not a duplicate h1\n### not a skipped level\n```\n\n## Real Section\n",
        );

        let findings = scan(dir.path());

        assert!(
            findings.is_empty(),
            "fenced pseudo-headings must not be findings: {findings:?}"
        );
    }

    // --- (e) .claude/agents/ default-denied ---

    #[test]
    fn claude_agents_file_with_zero_h1s_is_not_scanned() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            ".claude/agents/swe-rust-dev.md",
            "## No H1 In Agent Files\n\nbody\n",
        );

        let findings = scan(dir.path());

        assert!(
            findings.is_empty(),
            ".claude/agents/ is default-denied: {findings:?}"
        );
    }

    // --- (f) SKILL.md under .claude/skills/ default-denied ---

    #[test]
    fn skill_md_with_many_h1s_is_not_scanned() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            ".claude/skills/example/SKILL.md",
            "# One\n\n# Two\n\n# Three\n",
        );

        let findings = scan(dir.path());

        assert!(
            findings.is_empty(),
            ".claude/skills/ is default-denied: {findings:?}"
        );
    }

    // --- (g) plans/done/ excluded ---

    #[test]
    fn plans_done_file_with_skipped_level_is_not_scanned() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "plans/done/2026-01-01__archived/delivery.md",
            "# Title\n\n### Skipped In Archive\n",
        );

        let findings = scan(dir.path());

        assert!(
            findings.is_empty(),
            "plans/done/ is excluded from the allowlist: {findings:?}"
        );
    }

    // --- (h) plans/in-progress/ scanned ---

    #[test]
    fn plans_in_progress_file_with_duplicate_h1_reports_finding() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "plans/in-progress/some-plan/prd.md",
            "# Plan\n\ntext\n\n# Plan Again\n",
        );

        let findings = scan(dir.path());

        assert_eq!(findings.len(), 1, "expected one finding: {findings:?}");
        assert_eq!(findings[0].kind, HeadingFindingKind::DuplicateH1);
        assert_eq!(findings[0].file, "plans/in-progress/some-plan/prd.md");
    }

    // --- (i) apps/<name>/README.md scanned, apps/<name>/src/** denied ---

    #[test]
    fn apps_readme_is_scanned_while_apps_src_is_not() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "apps/example/README.md",
            "# Example\n\n### Skipped In Readme\n",
        );
        write(
            dir.path(),
            "apps/example/src/notes.md",
            "## Zero H1s Here But Default-Denied\n",
        );

        let findings = scan(dir.path());

        assert_eq!(
            findings.len(),
            1,
            "only the README finding is expected: {findings:?}"
        );
        assert_eq!(findings[0].kind, HeadingFindingKind::SkippedLevel);
        assert_eq!(findings[0].file, "apps/example/README.md");
    }

    // --- (j) specs/ scanned ---

    #[test]
    fn specs_file_with_duplicate_h1_reports_finding() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "specs/apps/rhino/overview.md",
            "# Spec\n\n# Spec Duplicate\n",
        );

        let findings = scan(dir.path());

        assert_eq!(findings.len(), 1, "expected one finding: {findings:?}");
        assert_eq!(findings[0].kind, HeadingFindingKind::DuplicateH1);
        assert_eq!(findings[0].file, "specs/apps/rhino/overview.md");
    }

    // --- (k) --exclude subtracts on top of the allowlist ---

    #[test]
    fn exclude_prefix_suppresses_docs_but_other_allowlist_trees_still_report() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "docs/excluded.md", "## Missing H1 In Docs\n");
        write(
            dir.path(),
            "repo-governance/rule.md",
            "## Missing H1 In Governance\n",
        );

        let opts = HeadingScanOptions {
            root: dir.path().to_path_buf(),
            exclude: vec!["docs".to_string()],
            ..Default::default()
        };
        let findings = validate_heading_hierarchy(&opts).unwrap();

        assert_eq!(
            findings.len(),
            1,
            "only the non-excluded tree should report: {findings:?}"
        );
        assert_eq!(findings[0].kind, HeadingFindingKind::MissingH1);
        assert_eq!(findings[0].file, "repo-governance/rule.md");
    }

    // --- Allowlist predicate unit tests ---

    #[test]
    fn allowlist_accepts_docs_tree() {
        assert!(is_prose_allowlisted("docs/guide.md"));
        assert!(is_prose_allowlisted("docs/explanation/deep/file.md"));
    }

    #[test]
    fn allowlist_accepts_repo_governance_tree() {
        assert!(is_prose_allowlisted("repo-governance/principles/README.md"));
    }

    #[test]
    fn allowlist_accepts_specs_tree() {
        assert!(is_prose_allowlisted("specs/apps/rhino/overview.md"));
    }

    #[test]
    fn allowlist_accepts_plans_except_done() {
        assert!(is_prose_allowlisted("plans/README.md"));
        assert!(is_prose_allowlisted("plans/in-progress/x/prd.md"));
        assert!(is_prose_allowlisted("plans/backlog/idea.md"));
        assert!(!is_prose_allowlisted("plans/done/2026-01-01__x/prd.md"));
    }

    #[test]
    fn allowlist_accepts_root_level_markdown_only() {
        assert!(is_prose_allowlisted("README.md"));
        assert!(is_prose_allowlisted("AGENTS.md"));
        assert!(!is_prose_allowlisted("README.txt"));
    }

    #[test]
    fn allowlist_accepts_apps_and_libs_readme() {
        assert!(is_prose_allowlisted("apps/example/README.md"));
        assert!(is_prose_allowlisted("libs/ts-utils/README.md"));
    }

    #[test]
    fn allowlist_accepts_apps_and_libs_docs_trees() {
        assert!(is_prose_allowlisted("apps/example/docs/design.md"));
        assert!(is_prose_allowlisted("libs/ts-utils/docs/api/usage.md"));
    }

    #[test]
    fn allowlist_default_denies_everything_else() {
        assert!(!is_prose_allowlisted(".claude/agents/swe-rust-dev.md"));
        assert!(!is_prose_allowlisted(".claude/skills/example/SKILL.md"));
        assert!(!is_prose_allowlisted(".opencode/agents/docs-maker.md"));
        assert!(!is_prose_allowlisted("apps/example/src/notes.md"));
        assert!(!is_prose_allowlisted("libs/ts-utils/src/README.md"));
        assert!(!is_prose_allowlisted("node_modules/pkg/README.md"));
    }

    // --- Explicit-paths mode (positional / staged inputs) ---

    #[test]
    fn explicit_paths_still_apply_the_allowlist() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            ".claude/skills/example/SKILL.md",
            "# One\n\n# Two\n",
        );
        write(dir.path(), "docs/dup.md", "# A\n\n# B\n");

        let opts = HeadingScanOptions {
            root: dir.path().to_path_buf(),
            paths: vec![
                ".claude/skills/example/SKILL.md".to_string(),
                "docs/dup.md".to_string(),
            ],
            ..Default::default()
        };
        let findings = validate_heading_hierarchy(&opts).unwrap();

        assert_eq!(findings.len(), 1, "skill file must be denied: {findings:?}");
        assert_eq!(findings[0].file, "docs/dup.md");
        assert_eq!(findings[0].kind, HeadingFindingKind::DuplicateH1);
    }

    #[test]
    fn explicit_directory_path_is_walked() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "docs/sub/no-h1.md", "## Section Only\n");
        write(dir.path(), "repo-governance/no-h1.md", "## Also Missing\n");

        let opts = HeadingScanOptions {
            root: dir.path().to_path_buf(),
            paths: vec!["docs".to_string()],
            ..Default::default()
        };
        let findings = validate_heading_hierarchy(&opts).unwrap();

        assert_eq!(
            findings.len(),
            1,
            "only the docs tree was requested: {findings:?}"
        );
        assert_eq!(findings[0].file, "docs/sub/no-h1.md");
        assert_eq!(findings[0].kind, HeadingFindingKind::MissingH1);
    }

    #[test]
    fn explicit_missing_path_errors() {
        let dir = tempfile::tempdir().unwrap();
        let opts = HeadingScanOptions {
            root: dir.path().to_path_buf(),
            paths: vec!["docs/does-not-exist.md".to_string()],
            ..Default::default()
        };
        assert!(validate_heading_hierarchy(&opts).is_err());
    }

    // --- Report formatting ---

    fn sample_findings() -> Vec<HeadingFinding> {
        vec![
            HeadingFinding {
                file: "docs/a.md".to_string(),
                line: 1,
                kind: HeadingFindingKind::MissingH1,
                message: "file has no H1 heading".to_string(),
            },
            HeadingFinding {
                file: "docs/b.md".to_string(),
                line: 5,
                kind: HeadingFindingKind::DuplicateH1,
                message: "duplicate H1 \"Two\" (first H1 at line 1)".to_string(),
            },
        ]
    }

    #[test]
    fn text_no_findings_shows_success() {
        assert_eq!(
            format_heading_text(&[], false),
            "✓ All heading hierarchies valid! No findings found.\n"
        );
    }

    #[test]
    fn text_no_findings_quiet_is_empty() {
        assert!(format_heading_text(&[], true).is_empty());
    }

    #[test]
    fn text_report_groups_findings_by_file() {
        let s = format_heading_text(&sample_findings(), false);
        assert!(s.starts_with("# Heading Hierarchy Report\n\n"), "got: {s}");
        assert!(s.contains("**Total findings**: 2\n"), "got: {s}");
        assert!(s.contains("\n## docs/a.md\n\n"), "got: {s}");
        assert!(
            s.contains("- Line 1: missing-h1: file has no H1 heading\n"),
            "got: {s}"
        );
        assert!(s.contains("\n## docs/b.md\n\n"), "got: {s}");
        assert!(
            s.contains("- Line 5: duplicate-h1: duplicate H1 \"Two\" (first H1 at line 1)\n"),
            "got: {s}"
        );
    }

    #[test]
    fn json_success_empty_findings() {
        let s = format_heading_json(&[]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "success");
        assert_eq!(v["total_findings"], 0);
        assert!(v["findings"].as_array().unwrap().is_empty());
    }

    #[test]
    fn json_failure_with_findings() {
        let s = format_heading_json(&sample_findings()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failure");
        assert_eq!(v["total_findings"], 2);
        assert_eq!(v["findings"][0]["file"], "docs/a.md");
        assert_eq!(v["findings"][0]["kind"], "missing-h1");
        assert_eq!(v["findings"][1]["line"], 5);
    }

    #[test]
    fn markdown_delegates_to_text() {
        let f = sample_findings();
        assert_eq!(format_heading_markdown(&f), format_heading_text(&f, false));
    }
}
