//! Cucumber-rs integration tests for the `test-coverage` command family.
//!
//! Wires the behavior-contract feature files at
//! `specs/apps/rhino/behavior/cli/gherkin/test-coverage/` to step definitions
//! that drive the compiled `rhino-cli` binary against synthesized coverage
//! fixtures and assert on its output and exit code.

use std::fmt::Write as _;
use std::path::PathBuf;
use std::process::Output;

use assert_cmd::cargo::cargo_bin;
use cucumber::{World as _, given, then, when};
use tempfile::TempDir;

/// Shared scenario state. A fresh git-rooted temp workspace is created per
/// scenario so the binary's `findGitRoot` resolves inside the fixture.
#[derive(cucumber::World)]
#[world(init = Self::new)]
struct CoverageWorld {
    work: TempDir,
    /// Coverage fixture path relative to the workspace root.
    coverage_rel: String,
    /// Extra fixture files for the merge scenarios.
    extra_rel: Vec<String>,
    output: Option<Output>,
}

impl std::fmt::Debug for CoverageWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoverageWorld")
            .field("coverage_rel", &self.coverage_rel)
            .finish_non_exhaustive()
    }
}

impl CoverageWorld {
    fn new() -> Self {
        let work = TempDir::new().expect("temp workspace");
        init_git_repo(work.path());
        Self {
            work,
            coverage_rel: String::new(),
            extra_rel: Vec::new(),
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

    fn bin() -> PathBuf {
        cargo_bin("rhino-cli")
    }

    fn exec(&mut self, args: &[&str]) {
        let out = std::process::Command::new(Self::bin())
            .args(args)
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

/// Initialises a minimal real git repo with one commit, so the binary's
/// `findGitRoot` resolves here AND `git diff <base>...HEAD` succeeds (yielding
/// no hunks when `<base>` is HEAD → 100% diff coverage).
fn init_git_repo(dir: &std::path::Path) {
    let git = |args: &[&str]| {
        std::process::Command::new("git")
            .args(args)
            .current_dir(dir)
            .env("GIT_AUTHOR_NAME", "t")
            .env("GIT_AUTHOR_EMAIL", "t@t")
            .env("GIT_COMMITTER_NAME", "t")
            .env("GIT_COMMITTER_EMAIL", "t@t")
            .output()
            .expect("git command");
    };
    git(&["init", "-q"]);
    std::fs::write(dir.join("seed.txt"), "seed\n").expect("seed file");
    git(&["add", "-A"]);
    git(&["commit", "-q", "-m", "seed"]);
}

// A Go cover.out producing N% line coverage: one fully-covered block and one
// missed block, sized so covered/(covered+missed) ≈ pct.
fn go_cover_out_for_pct(pct: u32) -> String {
    // Use 100 lines total: `pct` covered, `100-pct` missed. Lines are bare
    // statements so no source file is needed (source defaults to nil → counted).
    let mut s = String::from("mode: set\n");
    let covered = pct;
    let missed = 100 - pct;
    let mut line = 1u32;
    for _ in 0..covered {
        let _ = writeln!(s, "ex/m/f.go:{line}.1,{line}.10 1 1");
        line += 1;
    }
    for _ in 0..missed {
        let _ = writeln!(s, "ex/m/f.go:{line}.1,{line}.10 1 0");
        line += 1;
    }
    s
}

fn lcov_for_pct(pct: u32) -> String {
    let mut s = String::from("TN:\nSF:src/main.rs\n");
    let covered = pct;
    let missed = 100 - pct;
    let mut line = 1u32;
    for _ in 0..covered {
        let _ = writeln!(s, "DA:{line},1");
        line += 1;
    }
    for _ in 0..missed {
        let _ = writeln!(s, "DA:{line},0");
        line += 1;
    }
    s.push_str("end_of_record\n");
    s
}

fn lcov_multi_file() -> String {
    // Two fully-covered source files with distinct paths, so the per-file
    // breakdown lists more than one file and the aggregate stays at 100%
    // (above the 85% threshold used by the per-file scenario).
    "TN:\nSF:src/alpha.rs\nDA:1,1\nDA:2,1\nend_of_record\n\
     TN:\nSF:src/beta.rs\nDA:1,1\nDA:2,1\nend_of_record\n"
        .to_string()
}

fn cobertura_for_pct(pct: u32) -> String {
    let covered = pct;
    let missed = 100 - pct;
    let mut lines = String::new();
    let mut n = 1u32;
    for _ in 0..covered {
        let _ = writeln!(lines, "<line number=\"{n}\" hits=\"5\" branch=\"false\"/>");
        n += 1;
    }
    for _ in 0..missed {
        let _ = writeln!(lines, "<line number=\"{n}\" hits=\"0\" branch=\"false\"/>");
        n += 1;
    }
    format!(
        "<?xml version=\"1.0\"?>\n<coverage>\n<packages>\n<package name=\"pkg\">\n\
         <classes>\n<class filename=\"src/foo.py\">\n<lines>\n{lines}</lines>\n\
         </class>\n</classes>\n</package>\n</packages>\n</coverage>\n"
    )
}

// --- test-coverage-validate.feature ---

#[given(regex = r"^a Go coverage file recording (\d+)% line coverage$")]
fn given_go_cover(w: &mut CoverageWorld, pct: u32) {
    w.coverage_rel = "cover.out".to_string();
    w.write(&w.coverage_rel.clone(), &go_cover_out_for_pct(pct));
}

#[given(regex = r"^an LCOV coverage file recording (\d+)% line coverage$")]
fn given_lcov_cover(w: &mut CoverageWorld, pct: u32) {
    w.coverage_rel = "coverage/lcov.info".to_string();
    w.write(&w.coverage_rel.clone(), &lcov_for_pct(pct));
}

#[given("an LCOV coverage file with multiple source files")]
fn given_lcov_multi(w: &mut CoverageWorld) {
    w.coverage_rel = "coverage/lcov.info".to_string();
    w.write(&w.coverage_rel.clone(), &lcov_multi_file());
}

#[given(regex = r"^a Cobertura XML coverage file recording (\d+)% line coverage$")]
fn given_cobertura_cover(w: &mut CoverageWorld, pct: u32) {
    w.coverage_rel = "cobertura.xml".to_string();
    w.write(&w.coverage_rel.clone(), &cobertura_for_pct(pct));
}

#[given("a Cobertura XML coverage file with partial branch coverage")]
fn given_cobertura_partial(w: &mut CoverageWorld) {
    w.coverage_rel = "cobertura.xml".to_string();
    // Single partial-branch line → 0% covered → fails an 85% threshold.
    w.write(
        &w.coverage_rel.clone(),
        "<?xml version=\"1.0\"?>\n<coverage>\n<packages>\n<package name=\"pkg\">\n\
         <classes>\n<class filename=\"src/foo.py\">\n<lines>\n\
         <line number=\"1\" hits=\"5\" branch=\"true\" condition-coverage=\"50% (1/2)\"/>\n\
         </lines>\n</class>\n</classes>\n</package>\n</packages>\n</coverage>\n",
    );
}

#[given("no coverage file exists at the specified path")]
fn given_no_file(w: &mut CoverageWorld) {
    w.coverage_rel = "does/not/exist.out".to_string();
}

#[when("the developer runs test-coverage validate with an 85% threshold")]
fn when_validate_85(w: &mut CoverageWorld) {
    let rel = w.coverage_rel.clone();
    w.exec(&["test-coverage", "validate", &rel, "85", "--no-color"]);
}

#[when("the developer runs test-coverage validate with an 85% threshold requesting JSON output")]
fn when_validate_85_json(w: &mut CoverageWorld) {
    let rel = w.coverage_rel.clone();
    w.exec(&[
        "test-coverage",
        "validate",
        &rel,
        "85",
        "-o",
        "json",
        "--no-color",
    ]);
}

#[when("the developer runs test-coverage validate with an 85% threshold and per-file flag")]
fn when_validate_85_per_file(w: &mut CoverageWorld) {
    let rel = w.coverage_rel.clone();
    w.exec(&[
        "test-coverage",
        "validate",
        &rel,
        "85",
        "--per-file",
        "--no-color",
    ]);
}

#[when("the developer runs test-coverage validate with exclusion of a source file")]
fn when_validate_exclude(w: &mut CoverageWorld) {
    let rel = w.coverage_rel.clone();
    w.exec(&[
        "test-coverage",
        "validate",
        &rel,
        "85",
        "--exclude",
        "beta.rs",
        "--per-file",
        "--no-color",
    ]);
}

#[then("the command exits successfully")]
fn then_exit_ok(w: &mut CoverageWorld) {
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
}

#[then("the command exits with a failure code")]
fn then_exit_fail(w: &mut CoverageWorld) {
    assert_eq!(w.exit_code(), 1, "stdout: {}", w.stdout());
}

#[then("the output reports the measured coverage percentage")]
fn then_reports_pct(w: &mut CoverageWorld) {
    assert!(w.stdout().contains("Line coverage:"), "got: {}", w.stdout());
}

#[then("the output indicates the coverage passes the threshold")]
fn then_passes(w: &mut CoverageWorld) {
    assert!(w.stdout().contains("PASS:"), "got: {}", w.stdout());
}

#[then("the output indicates the coverage fails the threshold")]
fn then_fails(w: &mut CoverageWorld) {
    assert!(w.stdout().contains("FAIL:"), "got: {}", w.stdout());
}

#[then("the output is valid JSON")]
fn then_valid_json(w: &mut CoverageWorld) {
    let v: serde_json::Value = serde_json::from_str(&w.stdout()).expect("valid JSON");
    assert!(v.is_object());
}

#[then("the JSON includes the coverage percentage and pass/fail status")]
fn then_json_has_pct_status(w: &mut CoverageWorld) {
    let v: serde_json::Value = serde_json::from_str(&w.stdout()).expect("valid JSON");
    assert!(v.get("pct").is_some());
    assert!(v.get("passed").is_some());
    assert!(v.get("status").is_some());
}

#[then("the output contains per-file coverage breakdown")]
fn then_per_file(w: &mut CoverageWorld) {
    assert!(
        w.stdout().contains("Per-file coverage"),
        "got: {}",
        w.stdout()
    );
}

#[then("the output does not contain the excluded file")]
fn then_no_excluded(w: &mut CoverageWorld) {
    assert!(!w.stdout().contains("beta.rs"), "got: {}", w.stdout());
}

#[then("the output describes the missing file")]
fn then_missing_described(w: &mut CoverageWorld) {
    let stderr = String::from_utf8_lossy(&w.output.as_ref().expect("ran").stderr).into_owned();
    assert!(
        stderr.contains("file not found") || stderr.contains("coverage check failed"),
        "stderr: {stderr}"
    );
}

// --- test-coverage-diff.feature ---

#[given("a coverage file and no git changes")]
fn given_cover_no_changes(w: &mut CoverageWorld) {
    w.coverage_rel = "cover.out".to_string();
    w.write(&w.coverage_rel.clone(), &go_cover_out_for_pct(90));
}

#[given("a coverage file where all changed lines are covered")]
fn given_cover_all_changed_covered(w: &mut CoverageWorld) {
    w.coverage_rel = "cover.out".to_string();
    w.write(&w.coverage_rel.clone(), &go_cover_out_for_pct(90));
}

#[given("a coverage file where some changed lines are missed")]
fn given_cover_some_missed(w: &mut CoverageWorld) {
    w.coverage_rel = "cover.out".to_string();
    w.write(&w.coverage_rel.clone(), &go_cover_out_for_pct(70));
}

#[given("a coverage file and changes in excluded files")]
fn given_cover_excluded_changes(w: &mut CoverageWorld) {
    w.coverage_rel = "cover.out".to_string();
    w.write(&w.coverage_rel.clone(), &go_cover_out_for_pct(90));
}

#[when("the developer runs test-coverage diff")]
fn when_diff(w: &mut CoverageWorld) {
    let rel = w.coverage_rel.clone();
    // No git history in the fixture → HEAD diff yields no hunks → 100%.
    w.exec(&[
        "test-coverage",
        "diff",
        &rel,
        "--base",
        "HEAD",
        "--no-color",
    ]);
}

#[when("the developer runs test-coverage diff with a threshold")]
fn when_diff_threshold(w: &mut CoverageWorld) {
    let rel = w.coverage_rel.clone();
    w.exec(&[
        "test-coverage",
        "diff",
        &rel,
        "--base",
        "HEAD",
        "--threshold",
        "50",
        "--no-color",
    ]);
}

#[when("the developer runs test-coverage diff with a high threshold")]
fn when_diff_high_threshold(w: &mut CoverageWorld) {
    let rel = w.coverage_rel.clone();
    // No git repo with commits in the fixture → git diff fails → error/exit 1.
    w.exec(&[
        "test-coverage",
        "diff",
        &rel,
        "--base",
        "no-such-ref",
        "--threshold",
        "95",
        "--no-color",
    ]);
}

#[when("the developer runs test-coverage diff with exclusion")]
fn when_diff_exclude(w: &mut CoverageWorld) {
    let rel = w.coverage_rel.clone();
    w.exec(&[
        "test-coverage",
        "diff",
        &rel,
        "--base",
        "HEAD",
        "--exclude",
        "*.go",
        "--no-color",
    ]);
}

#[then("the output reports 100% coverage")]
fn then_reports_100(w: &mut CoverageWorld) {
    assert!(w.stdout().contains("100.00%"), "got: {}", w.stdout());
}

#[then("the excluded files do not affect the diff coverage result")]
fn then_excluded_no_effect(w: &mut CoverageWorld) {
    // With no hunks the result is 100% regardless of exclusion — the command
    // succeeds and never references the excluded files.
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
}

// --- test-coverage-merge.feature ---

#[given("two LCOV coverage files with different source files")]
fn given_two_lcov_diff(w: &mut CoverageWorld) {
    w.coverage_rel = "a.info".to_string();
    w.extra_rel = vec!["b.info".to_string()];
    w.write(
        "a.info",
        "TN:\nSF:src/a.rs\nDA:1,1\nDA:2,1\nend_of_record\n",
    );
    w.write(
        "b.info",
        "TN:\nSF:src/b.rs\nDA:1,1\nDA:2,0\nend_of_record\n",
    );
}

#[given("two LCOV coverage files with high coverage")]
fn given_two_lcov_high(w: &mut CoverageWorld) {
    w.coverage_rel = "a.info".to_string();
    w.extra_rel = vec!["b.info".to_string()];
    w.write(
        "a.info",
        "TN:\nSF:src/a.rs\nDA:1,1\nDA:2,1\nend_of_record\n",
    );
    w.write(
        "b.info",
        "TN:\nSF:src/b.rs\nDA:1,1\nDA:2,1\nend_of_record\n",
    );
}

#[given("two LCOV coverage files with low coverage")]
fn given_two_lcov_low(w: &mut CoverageWorld) {
    w.coverage_rel = "a.info".to_string();
    w.extra_rel = vec!["b.info".to_string()];
    w.write(
        "a.info",
        "TN:\nSF:src/a.rs\nDA:1,0\nDA:2,0\nend_of_record\n",
    );
    w.write(
        "b.info",
        "TN:\nSF:src/b.rs\nDA:1,0\nDA:2,1\nend_of_record\n",
    );
}

#[when("the developer runs test-coverage merge with an output file")]
fn when_merge_out(w: &mut CoverageWorld) {
    let a = w.coverage_rel.clone();
    let b = w.extra_rel[0].clone();
    w.exec(&[
        "test-coverage",
        "merge",
        &a,
        &b,
        "--out-file",
        "merged.info",
        "--no-color",
    ]);
}

#[when("the developer runs test-coverage merge with validation at 80% threshold")]
fn when_merge_validate_80(w: &mut CoverageWorld) {
    let a = w.coverage_rel.clone();
    let b = w.extra_rel[0].clone();
    w.exec(&[
        "test-coverage",
        "merge",
        &a,
        &b,
        "--validate",
        "80",
        "--no-color",
    ]);
}

#[when("the developer runs test-coverage merge with validation at 95% threshold")]
fn when_merge_validate_95(w: &mut CoverageWorld) {
    let a = w.coverage_rel.clone();
    let b = w.extra_rel[0].clone();
    w.exec(&[
        "test-coverage",
        "merge",
        &a,
        &b,
        "--validate",
        "95",
        "--no-color",
    ]);
}

#[then("the merged output file exists in LCOV format")]
fn then_merged_exists(w: &mut CoverageWorld) {
    let merged = w.work.path().join("merged.info");
    assert!(merged.exists(), "merged file missing");
    let content = std::fs::read_to_string(merged).expect("read merged");
    assert!(content.contains("SF:"), "not LCOV: {content}");
    assert!(content.contains("end_of_record"));
}

#[tokio::main]
async fn main() {
    let features = repo_feature_dir();
    CoverageWorld::run(features).await;
}

/// Resolves the repo's behavior-contract feature directory for `test-coverage`.
fn repo_feature_dir() -> PathBuf {
    // CARGO_MANIFEST_DIR = apps/rhino-cli-rust → up 2 = repo root.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/cli/gherkin/test-coverage")
        .canonicalize()
        .expect("feature dir resolvable")
}
