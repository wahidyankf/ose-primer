//! Cucumber-rs integration tests for the `git pre-commit` command.
//!
//! Wires the behavior-contract feature file at
//! `specs/apps/rhino/behavior/rhino-cli/gherkin/git/` to step definitions that drive
//! the compiled `rhino-cli` binary.
//!
//! Two fixture shapes are used:
//!
//! - **Outside a git repository**: a temp directory containing NO `.git`
//!   entry, so the orchestrator fails at `findGitRoot` before any
//!   external-tool step runs.
//! - **Staged-file scenarios (DD-8)**: a freshly `git init`-ed temp repo with
//!   exactly the scenario's file staged. The orchestrator's external tools
//!   (`docker`, `nx`, `npx`, `npm`) are replaced by always-succeeding PATH
//!   stubs so only the deterministic in-process steps (staged mermaid,
//!   heading-hierarchy, and link validation) decide the outcome. `git`
//!   itself stays real — staged-file detection needs it.
//!
//! The remaining per-step orchestration logic is covered by the runner's unit
//! tests with injected dependencies.

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Output;

use assert_cmd::cargo::cargo_bin;
use cucumber::{World as _, given, then, when};
use tempfile::TempDir;

/// External tools stubbed out of the pre-commit run (each exits 0). `git` is
/// deliberately NOT stubbed.
const STUBBED_TOOLS: &[&str] = &["docker", "nx", "npx", "npm"];

/// Shared scenario state. The temp workspace has no `.git` entry until a
/// staged-file Given step runs `git init`.
#[derive(cucumber::World)]
#[world(init = Self::new)]
struct GitWorld {
    work: TempDir,
    stubs: TempDir,
    output: Option<Output>,
}

impl std::fmt::Debug for GitWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitWorld").finish_non_exhaustive()
    }
}

impl GitWorld {
    fn new() -> Self {
        // No git init here: the outside-a-repo scenario needs find_root to
        // walk up and fail (the temp dir lives outside any repo because
        // TempDir uses the OS temp root).
        let work = TempDir::new().expect("temp workspace");
        let stubs = TempDir::new().expect("temp stub bin dir");
        for tool in STUBBED_TOOLS {
            write_stub(stubs.path(), tool);
        }
        Self {
            work,
            stubs,
            output: None,
        }
    }

    /// Turns the temp workspace into a fresh git repository.
    fn init_repo(&self) {
        let status = std::process::Command::new("git")
            .args(["init", "-q"])
            .current_dir(self.work.path())
            .status()
            .expect("git init");
        assert!(status.success(), "git init failed");
    }

    /// Writes `content` to `work/rel`, creating parent directories.
    fn write_file(&self, rel: &str, content: &str) {
        let path = self.work.path().join(rel);
        std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
        std::fs::write(path, content).expect("write fixture");
    }

    /// Stages `rel` in the workspace repository.
    fn stage(&self, rel: &str) {
        let status = std::process::Command::new("git")
            .args(["add", rel])
            .current_dir(self.work.path())
            .status()
            .expect("git add");
        assert!(status.success(), "git add {rel} failed");
    }

    fn exec(&mut self) {
        // Prepend the stub bin dir so the orchestrator's external tools
        // succeed deterministically; harmless for the outside-a-repo
        // scenario, which fails before any tool runs.
        let path_var = format!(
            "{}:{}",
            self.stubs.path().display(),
            std::env::var("PATH").unwrap_or_default()
        );
        let out = std::process::Command::new(cargo_bin("rhino-cli"))
            .args(["git", "pre-commit", "--no-color"])
            .env("PATH", path_var)
            .current_dir(self.work.path())
            .output()
            .expect("run rhino-cli");
        self.output = Some(out);
    }

    fn stdout(&self) -> String {
        String::from_utf8_lossy(&self.output.as_ref().expect("ran").stdout).into_owned()
    }

    fn stderr(&self) -> String {
        String::from_utf8_lossy(&self.output.as_ref().expect("ran").stderr).into_owned()
    }

    fn combined(&self) -> String {
        format!("{}{}", self.stdout(), self.stderr())
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

/// Writes an always-succeeding executable stub named `name` into `dir`.
fn write_stub(dir: &Path, name: &str) {
    let path = dir.join(name);
    std::fs::write(&path, "#!/bin/sh\nexit 0\n").expect("write stub");
    let mut perms = std::fs::metadata(&path).expect("stat stub").permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&path, perms).expect("chmod stub");
}

// ===========================================================================
// Given steps
// ===========================================================================

#[given("the developer is outside a git repository")]
fn given_outside_git_repo(_w: &mut GitWorld) {
    // The temp workspace was created without `git init`; nothing to do.
}

#[given("a staged markdown file containing a flowchart with a malformed mermaid block")]
fn given_staged_malformed_flowchart(w: &mut GitWorld) {
    w.init_repo();
    // Node label far exceeds the 30-character limit — a blocking violation.
    w.write_file(
        "docs/diagram.md",
        "# Doc\n\n```mermaid\nflowchart TD\n  \
         A[This label is far longer than the thirty character limit] --> B[Ok]\n```\n",
    );
    w.stage("docs/diagram.md");
}

#[given("a staged markdown file under docs/ containing a duplicate H1 heading")]
fn given_staged_docs_duplicate_h1(w: &mut GitWorld) {
    w.init_repo();
    w.write_file("docs/guide.md", "# First Title\n\ntext\n\n# Second Title\n");
    w.stage("docs/guide.md");
}

#[given("a staged SKILL.md file under .claude/skills/ containing multiple H1 headings")]
fn given_staged_skill_with_many_h1s(w: &mut GitWorld) {
    w.init_repo();
    // A staged `.claude/` path also triggers the config-validation step
    // (step 1), so the fixture must be a minimally VALID `.claude/` tree:
    // an empty agents dir plus a skill whose frontmatter passes
    // `validate-claude`. The body's multiple H1s are the heading-gate
    // exemption under test.
    std::fs::create_dir_all(w.work.path().join(".claude/agents")).expect("mkdir agents");
    w.write_file(
        ".claude/skills/example-skill/SKILL.md",
        "---\nname: example-skill\n\
         description: Fixture skill for the pre-commit heading-gate exemption test.\n---\n\n\
         # First H1\n\ntext\n\n# Second H1\n\ntext\n\n# Third H1\n",
    );
    w.stage(".claude/skills/example-skill/SKILL.md");
}

#[given("a staged markdown file under plans/done/ containing a broken internal link")]
fn given_staged_plans_done_broken_link(w: &mut GitWorld) {
    w.init_repo();
    w.write_file(
        "plans/done/2026-01-01__archived-plan/notes.md",
        "# Archived Notes\n\nSee the [missing target](./missing.md) for details.\n",
    );
    w.stage("plans/done/2026-01-01__archived-plan/notes.md");
}

// ===========================================================================
// When steps
// ===========================================================================

#[when("the developer runs rhino-cli git pre-commit")]
fn when_run_pre_commit(w: &mut GitWorld) {
    w.exec();
}

// ===========================================================================
// Then steps
// ===========================================================================

#[then("the command exits with a failure code")]
fn then_exit_fail(w: &mut GitWorld) {
    assert_eq!(
        w.exit_code(),
        1,
        "stdout: {}\nstderr: {}",
        w.stdout(),
        w.stderr()
    );
}

#[then("the command exits successfully")]
fn then_exit_success(w: &mut GitWorld) {
    assert_eq!(
        w.exit_code(),
        0,
        "stdout: {}\nstderr: {}",
        w.stdout(),
        w.stderr()
    );
}

#[then("the output mentions that a git repository was not found")]
fn then_mentions_git_not_found(w: &mut GitWorld) {
    let combined = w.combined();
    assert!(
        combined.contains("git"),
        "expected output or error to mention 'git', got: {combined}"
    );
}

#[then("the output reports a mermaid violation for the staged file")]
fn then_reports_mermaid_violation(w: &mut GitWorld) {
    let combined = w.combined();
    assert!(
        combined.contains("docs/diagram.md") && combined.contains("mermaid violation"),
        "expected a mermaid violation naming docs/diagram.md, got: {combined}"
    );
}

#[then("the output reports a heading hierarchy violation for the staged file")]
fn then_reports_heading_violation(w: &mut GitWorld) {
    let combined = w.combined();
    assert!(
        combined.contains("docs/guide.md") && combined.contains("heading hierarchy"),
        "expected a heading hierarchy finding naming docs/guide.md, got: {combined}"
    );
}

#[then("no heading violation is reported for the skill file")]
fn then_no_heading_violation_for_skill(w: &mut GitWorld) {
    let combined = w.combined();
    assert!(
        !combined.contains("duplicate-h1") && !combined.contains("heading hierarchy"),
        "expected no heading finding for the exempt SKILL.md, got: {combined}"
    );
}

#[then("no broken-link violation is reported for the plans/done file")]
fn then_no_broken_link_for_plans_done(w: &mut GitWorld) {
    let combined = w.combined();
    assert!(
        !combined.contains("broken links") && !combined.contains("missing.md"),
        "expected the plans/done broken link to be skipped, got: {combined}"
    );
}

#[tokio::main]
async fn main() {
    GitWorld::run(feature_dir()).await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/rhino-cli/gherkin/git")
        .canonicalize()
        .expect("feature dir resolvable")
}
