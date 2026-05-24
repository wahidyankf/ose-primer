//! Cucumber-rs integration tests for the `git pre-commit` command.
//!
//! Wires the behavior-contract feature file at
//! `specs/apps/rhino/behavior/cli/gherkin/git/` to step definitions that drive
//! the compiled `rhino-cli` binary from inside a temp directory containing NO
//! `.git` entry, so the orchestrator fails at `findGitRoot` before any
//! external-tool step runs.
//!
//! The success path of `git pre-commit` shells out to `docker`, `nx`, `npx`,
//! `npm`, and `git`, whose output is environment-dependent and which mutate the
//! working tree. Only the deterministic "outside a git repository" failure path
//! is exercised here and in `shadow-diff.sh`; the per-step orchestration logic
//! is covered by the runner's unit tests with injected dependencies.

use std::path::PathBuf;
use std::process::Output;

use assert_cmd::cargo::cargo_bin;
use cucumber::{World as _, given, then, when};
use tempfile::TempDir;

/// Shared scenario state. The temp workspace deliberately has no `.git` entry.
#[derive(cucumber::World)]
#[world(init = Self::new)]
struct GitWorld {
    work: TempDir,
    output: Option<Output>,
}

impl std::fmt::Debug for GitWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitWorld").finish_non_exhaustive()
    }
}

impl GitWorld {
    fn new() -> Self {
        // No git init: ensures find_root walks up and fails (the temp dir lives
        // outside any repo because TempDir uses the OS temp root).
        let work = TempDir::new().expect("temp workspace");
        Self { work, output: None }
    }

    fn exec(&mut self) {
        let out = std::process::Command::new(cargo_bin("rhino-cli"))
            .args(["git", "pre-commit", "--no-color"])
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

    fn exit_code(&self) -> i32 {
        self.output
            .as_ref()
            .expect("ran")
            .status
            .code()
            .unwrap_or(-1)
    }
}

// ===========================================================================
// Given steps
// ===========================================================================

#[given("the developer is outside a git repository")]
fn given_outside_git_repo(_w: &mut GitWorld) {
    // The temp workspace was created without `git init`; nothing to do.
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

#[then("the output mentions that a git repository was not found")]
fn then_mentions_git_not_found(w: &mut GitWorld) {
    let combined = format!("{}{}", w.stdout(), w.stderr());
    assert!(
        combined.contains("git"),
        "expected output or error to mention 'git', got: {combined}"
    );
}

#[tokio::main]
async fn main() {
    GitWorld::run(feature_dir()).await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/cli/gherkin/git")
        .canonicalize()
        .expect("feature dir resolvable")
}
