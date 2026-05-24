//! Cucumber-rs integration tests for the `repo-governance vendor-audit` command.
//!
//! Wires the behavior-contract feature file at
//! `specs/apps/rhino/behavior/cli/gherkin/repo-governance/` to step definitions
//! that synthesize governance markdown fixtures inside a fresh git-rooted temp
//! workspace and drive the compiled `rhino-cli` binary, asserting on its output
//! and exit code.

use std::path::PathBuf;
use std::process::Output;

use assert_cmd::cargo::cargo_bin;
use cucumber::{World as _, given, then, when};
use tempfile::TempDir;

/// Shared scenario state. Each scenario gets a fresh git-rooted temp workspace
/// so the binary's `findGitRoot` resolves inside the fixture.
#[derive(cucumber::World)]
#[world(init = Self::new)]
struct GovernanceWorld {
    work: TempDir,
    /// Repo-relative path of the fixture file or directory to audit.
    target: String,
    output: Option<Output>,
}

impl std::fmt::Debug for GovernanceWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GovernanceWorld")
            .field("target", &self.target)
            .finish_non_exhaustive()
    }
}

impl GovernanceWorld {
    fn new() -> Self {
        let work = TempDir::new().expect("temp workspace");
        init_git_repo(work.path());
        Self {
            work,
            target: String::new(),
            output: None,
        }
    }

    fn write(&self, rel: &str, content: &str) {
        let p = self.work.path().join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).expect("mk fixture dir");
        }
        std::fs::write(p, content).expect("write fixture");
    }

    fn exec(&mut self) {
        let out = std::process::Command::new(cargo_bin("rhino-cli"))
            .args([
                "repo-governance",
                "vendor-audit",
                &self.target,
                "--no-color",
            ])
            .current_dir(self.work.path())
            .output()
            .expect("run rhino-cli");
        self.output = Some(out);
    }

    fn stdout(&self) -> String {
        String::from_utf8_lossy(&self.output.as_ref().expect("ran").stdout).into_owned()
    }

    fn exit_code(&self) -> i32 {
        self.output
            .as_ref()
            .expect("ran")
            .status
            .code()
            .unwrap_or(-1)
    }
}

fn init_git_repo(dir: &std::path::Path) {
    std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(dir)
        .env("GIT_AUTHOR_NAME", "t")
        .env("GIT_AUTHOR_EMAIL", "t@t")
        .env("GIT_COMMITTER_NAME", "t")
        .env("GIT_COMMITTER_EMAIL", "t@t")
        .output()
        .expect("git init");
}

// ===========================================================================
// Given steps
// ===========================================================================

#[given(r#"a governance markdown file containing "Claude Code" in plain prose"#)]
fn given_brand_in_prose(w: &mut GovernanceWorld) {
    w.target = "repo-governance/doc.md".to_string();
    w.write(&w.target.clone(), "# Doc\n\nWe use Claude Code daily.\n");
}

#[given(r#"a governance markdown file containing "Claude Code" inside a code fence"#)]
fn given_brand_in_fence(w: &mut GovernanceWorld) {
    w.target = "repo-governance/doc.md".to_string();
    w.write(&w.target.clone(), "# Doc\n\n```\nClaude Code\n```\n");
}

#[given(r#"a governance markdown file containing "Claude Code" inside a binding-example fence"#)]
fn given_brand_in_binding_example(w: &mut GovernanceWorld) {
    w.target = "repo-governance/doc.md".to_string();
    w.write(
        &w.target.clone(),
        "# Doc\n\n```binding-example\nClaude Code\n```\n",
    );
}

#[given(
    r#"a governance markdown file containing "Claude Code" under a "Platform Binding Examples" heading"#
)]
fn given_brand_under_pb_heading(w: &mut GovernanceWorld) {
    w.target = "repo-governance/doc.md".to_string();
    w.write(
        &w.target.clone(),
        "# Doc\n\n## Platform Binding Examples\n\nClaude Code is fine here.\n",
    );
}

#[given("a governance directory with no forbidden terms in prose")]
fn given_clean_directory(w: &mut GovernanceWorld) {
    w.target = "repo-governance".to_string();
    w.write(
        "repo-governance/a.md",
        "# A\n\nVendor-neutral prose only.\n",
    );
    w.write(
        "repo-governance/b.md",
        "# B\n\nThe coding agent does the work.\n",
    );
}

#[given(r#"a governance markdown file containing "Skills" in plain prose"#)]
fn given_skills_in_prose(w: &mut GovernanceWorld) {
    w.target = "repo-governance/doc.md".to_string();
    w.write(&w.target.clone(), "# Doc\n\nWe rely on Skills heavily.\n");
}

#[given(r#"a governance markdown file containing "Skills" inside a code fence"#)]
fn given_skills_in_fence(w: &mut GovernanceWorld) {
    w.target = "repo-governance/doc.md".to_string();
    w.write(&w.target.clone(), "# Doc\n\n```\nSkills\n```\n");
}

// ===========================================================================
// When steps
// ===========================================================================

#[when("the developer runs repo-governance vendor-audit on the file")]
#[when("the developer runs repo-governance vendor-audit on the directory")]
fn when_run_audit(w: &mut GovernanceWorld) {
    w.exec();
}

// ===========================================================================
// Then steps
// ===========================================================================

#[then("the command exits with a failure code")]
fn then_exit_fail(w: &mut GovernanceWorld) {
    assert_eq!(w.exit_code(), 1, "stdout: {}", w.stdout());
}

#[then("the command exits successfully")]
fn then_exit_ok(w: &mut GovernanceWorld) {
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
}

#[then("the output identifies the forbidden term and its location")]
fn then_identifies_term(w: &mut GovernanceWorld) {
    let out = w.stdout();
    assert!(out.contains("GOVERNANCE VENDOR AUDIT FAILED"), "got: {out}");
    assert!(out.contains("doc.md:"), "got: {out}");
}

#[then("the output reports zero findings")]
fn then_zero_findings(w: &mut GovernanceWorld) {
    let out = w.stdout();
    assert!(
        out.contains("GOVERNANCE VENDOR AUDIT PASSED: no violations found"),
        "got: {out}"
    );
}

#[tokio::main]
async fn main() {
    GovernanceWorld::run(feature_dir()).await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/cli/gherkin/repo-governance")
        .canonicalize()
        .expect("feature dir resolvable")
}
