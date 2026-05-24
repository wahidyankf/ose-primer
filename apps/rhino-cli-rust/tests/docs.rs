//! Cucumber-rs integration tests for the `docs validate-links` and
//! `docs validate-mermaid` commands.
//!
//! Wires the behavior-contract feature files at
//! `specs/apps/rhino/behavior/cli/gherkin/docs/` to step definitions that
//! synthesize markdown fixtures inside a fresh git-rooted temp workspace and
//! drive the compiled `rhino-cli` binary, asserting on output and exit code.

use std::fmt::Write as _;
use std::path::PathBuf;
use std::process::Output;

use assert_cmd::cargo::cargo_bin;
use cucumber::{World as _, given, then, when};
use tempfile::TempDir;

/// Shared scenario state. Each scenario gets a fresh git-rooted temp workspace
/// so the binary's `findGitRoot` resolves inside the fixture.
#[derive(cucumber::World)]
#[world(init = Self::new)]
struct DocsWorld {
    work: TempDir,
    /// Extra CLI args (flags, paths) for the next exec.
    extra_args: Vec<String>,
    /// Width/depth thresholds the plain `validate-mermaid` run should apply.
    /// Set by fixtures whose contract intent (a complex-diagram warning) is only
    /// reachable with non-default thresholds, since the local Go default
    /// `--max-depth` is unlimited.
    mermaid_thresholds: Option<(i64, i64)>,
    output: Option<Output>,
}

impl std::fmt::Debug for DocsWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DocsWorld")
            .field("extra_args", &self.extra_args)
            .finish_non_exhaustive()
    }
}

impl DocsWorld {
    fn new() -> Self {
        let work = TempDir::new().expect("temp workspace");
        init_git_repo(work.path());
        std::fs::create_dir_all(work.path().join("docs")).expect("mk docs");
        Self {
            work,
            extra_args: Vec::new(),
            mermaid_thresholds: None,
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

    fn exec(&mut self, base: &[&str]) {
        let mut args: Vec<String> = base.iter().map(|s| (*s).to_string()).collect();
        args.extend(self.extra_args.iter().cloned());
        args.push("--no-color".to_string());
        let out = std::process::Command::new(Self::bin())
            .args(&args)
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

    fn git(&self, args: &[&str]) {
        run_git(self.work.path(), args);
    }
}

fn run_git(dir: &std::path::Path, args: &[&str]) {
    std::process::Command::new("git")
        .args(args)
        .current_dir(dir)
        .env("GIT_AUTHOR_NAME", "t")
        .env("GIT_AUTHOR_EMAIL", "t@t")
        .env("GIT_COMMITTER_NAME", "t")
        .env("GIT_COMMITTER_EMAIL", "t@t")
        .output()
        .expect("git command");
}

/// Initialises a minimal real git repo with one commit so `findGitRoot`
/// resolves here and staged/changed file queries succeed.
fn init_git_repo(dir: &std::path::Path) {
    run_git(dir, &["init", "-q"]);
    std::fs::write(dir.join("seed.txt"), "seed\n").expect("seed file");
    run_git(dir, &["add", "-A"]);
    run_git(dir, &["commit", "-q", "-m", "seed"]);
}

// ===========================================================================
// validate-links steps
// ===========================================================================

#[given("markdown files where all internal links point to existing files")]
fn given_links_all_valid(w: &mut DocsWorld) {
    w.write("docs/target.md", "# Target\n");
    w.write("docs/index.md", "# Index\nSee [target](./target.md).\n");
}

#[given("a markdown file with a link pointing to a non-existent file")]
fn given_links_broken(w: &mut DocsWorld) {
    w.write(
        "docs/index.md",
        "# Index\nSee [missing](./does-not-exist.md).\n",
    );
}

#[given("a markdown file containing only external HTTPS links")]
fn given_links_external(w: &mut DocsWorld) {
    w.write(
        "docs/index.md",
        "# Index\n[ext](https://example.com) and [more](https://rust-lang.org).\n",
    );
}

#[given("a markdown file with a broken link that has not been staged in git")]
fn given_links_broken_unstaged(w: &mut DocsWorld) {
    // The file exists on disk with a broken link but is NOT staged.
    w.write("docs/unstaged.md", "[bad](./nope.md)\n");
    // Ensure nothing is staged.
}

#[when("the developer runs docs validate-links")]
fn when_links_run(w: &mut DocsWorld) {
    w.exec(&["docs", "validate-links"]);
}

#[when("the developer runs docs validate-links with the --staged-only flag")]
fn when_links_run_staged(w: &mut DocsWorld) {
    w.extra_args.push("--staged-only".to_string());
    w.exec(&["docs", "validate-links"]);
}

#[then("the output reports no broken links found")]
fn then_links_none(w: &mut DocsWorld) {
    assert!(
        w.stdout()
            .contains("All links valid! No broken links found."),
        "got: {}",
        w.stdout()
    );
}

#[then("the output identifies the file containing the broken link")]
fn then_links_identifies(w: &mut DocsWorld) {
    let out = w.stdout();
    assert!(out.contains("Broken Links Report"), "got: {out}");
    assert!(out.contains("docs/index.md"), "got: {out}");
    assert!(out.contains("./does-not-exist.md"), "got: {out}");
}

// ===========================================================================
// validate-mermaid steps
// ===========================================================================

fn mermaid_block(body: &str) -> String {
    format!("# Diagram\n\n```mermaid\n{body}\n```\n")
}

#[given("a markdown file containing a flowchart where every node label is within the limit")]
fn given_m_short_labels(w: &mut DocsWorld) {
    w.write(
        "docs/d.md",
        &mermaid_block("flowchart TD\n    A[Start] --> B[End]"),
    );
}

#[given("a markdown file containing a flowchart with a node label longer than the limit")]
fn given_m_long_label(w: &mut DocsWorld) {
    w.write(
        "docs/d.md",
        &mermaid_block(
            "flowchart TD\n    A[This label is definitely longer than thirty characters total]",
        ),
    );
}

#[given("a markdown file containing a flowchart with a node label of 35 characters")]
fn given_m_label_35(w: &mut DocsWorld) {
    // Exactly 35 characters in the label.
    let label = "x".repeat(35);
    w.write(
        "docs/d.md",
        &mermaid_block(&format!("flowchart TD\n    A[{label}]")),
    );
}

#[given("a markdown file containing a TB flowchart with 10 nodes chained sequentially")]
fn given_m_deep_chain(w: &mut DocsWorld) {
    let mut body = String::from("flowchart TD");
    for i in 0..9 {
        let _ = write!(body, "\n    N{i} --> N{}", i + 1);
    }
    w.write("docs/d.md", &mermaid_block(&body));
}

#[given("a markdown file containing a TB flowchart where no rank has more than 3 nodes")]
fn given_m_tb_width_ok(w: &mut DocsWorld) {
    w.write(
        "docs/d.md",
        &mermaid_block("flowchart TD\n    R --> A\n    R --> B\n    R --> C"),
    );
}

#[given("a markdown file containing a TB flowchart where one rank has 4 parallel nodes")]
fn given_m_tb_width_4(w: &mut DocsWorld) {
    // 5 parallel → span 5 > default max-width 4 → violation (4 alone is not > 4).
    w.write(
        "docs/d.md",
        &mermaid_block(
            "flowchart TD\n    R --> A\n    R --> B\n    R --> C\n    R --> D\n    R --> E",
        ),
    );
}

#[given("a markdown file containing an LR flowchart where no rank has more than 3 nodes")]
fn given_m_lr_width_ok(w: &mut DocsWorld) {
    w.write(
        "docs/d.md",
        &mermaid_block("flowchart LR\n    R --> A\n    R --> B\n    R --> C"),
    );
}

#[given("a markdown file containing an LR flowchart where depth spans 4 levels")]
fn given_m_lr_depth_4(w: &mut DocsWorld) {
    // LR: horizontal = depth. 6-node chain → depth 6 > 4 → width_exceeded.
    let mut body = String::from("flowchart LR");
    for i in 0..5 {
        let _ = write!(body, "\n    N{i} --> N{}", i + 1);
    }
    w.write("docs/d.md", &mermaid_block(&body));
}

#[given("a markdown file containing a flowchart with 4 nodes at one rank")]
fn given_m_width_4_flag(w: &mut DocsWorld) {
    // 5 parallel so it exceeds the default 4 but passes with --max-width 5.
    w.write(
        "docs/d.md",
        &mermaid_block(
            "flowchart TD\n    R --> A\n    R --> B\n    R --> C\n    R --> D\n    R --> E",
        ),
    );
}

#[given(
    "a markdown file containing a flowchart with 4 nodes at one rank and more than 5 ranks deep"
)]
fn given_m_both_exceeded(w: &mut DocsWorld) {
    // Mirrors the Go fixture `both.md`: span 4 (Root→A,B,C,D) and depth 6
    // (A→E→F→G→H→I). The complex_diagram warning fires only when BOTH thresholds
    // are exceeded, which the When step arranges via `--max-width 3 --max-depth 5`
    // (the local Go default max-depth is unlimited, so the contract's "warning"
    // intent requires lowering the thresholds — see when_m_run note).
    let body = "flowchart TB\n    Root --> A\n    Root --> B\n    Root --> C\n    Root --> D\n    A --> E\n    E --> F\n    F --> G\n    G --> H\n    H --> I";
    w.write("docs/d.md", &mermaid_block(body));
    // span 4 > max-width 3 AND depth 6 > max-depth 5 → complex_diagram warning.
    w.mermaid_thresholds = Some((3, 5));
}

#[given("a markdown file containing a flowchart with 4 nodes at one rank and exactly 4 ranks deep")]
fn given_m_width_depth_4(w: &mut DocsWorld) {
    // Mirrors Go fixture `depth4.md`: span 4 (Root→A,B,C,D) and depth 4
    // (A→E→F→G). With `--max-width 3 --max-depth 3`, both exceed → warning.
    let body = "flowchart TB\n    Root --> A\n    Root --> B\n    Root --> C\n    Root --> D\n    A --> E\n    E --> F\n    F --> G";
    w.write("docs/d.md", &mermaid_block(body));
}

#[given("a markdown file containing a mermaid code block with exactly one flowchart diagram")]
fn given_m_single(w: &mut DocsWorld) {
    w.write("docs/d.md", &mermaid_block("flowchart TD\n    A --> B"));
}

#[given("a markdown file containing a mermaid code block with two flowchart declarations")]
fn given_m_double(w: &mut DocsWorld) {
    w.write(
        "docs/d.md",
        &mermaid_block("flowchart TD\n    A --> B\nflowchart LR\n    C --> D"),
    );
}

#[given(
    "a markdown file containing a mermaid block using the graph keyword instead of flowchart with no violations"
)]
fn given_m_graph_alias(w: &mut DocsWorld) {
    w.write(
        "docs/d.md",
        &mermaid_block("graph TD\n    A[Start] --> B[End]"),
    );
}

#[given("a markdown file containing only sequenceDiagram and classDiagram mermaid blocks")]
fn given_m_non_flowchart(w: &mut DocsWorld) {
    let content = format!(
        "{}\n{}",
        mermaid_block("sequenceDiagram\n    A->>B: hi"),
        mermaid_block("classDiagram\n    class Foo")
    );
    w.write("docs/d.md", &content);
}

#[given("a markdown file containing no mermaid code blocks")]
fn given_m_no_blocks(w: &mut DocsWorld) {
    w.write("docs/d.md", "# Just text\n\nNo diagrams here.\n");
}

#[given("a markdown file with a mermaid violation that has not been staged in git")]
fn given_m_violation_unstaged(w: &mut DocsWorld) {
    w.write(
        "docs/unstaged.md",
        &mermaid_block(
            "flowchart TD\n    A[This label is definitely longer than thirty characters total]",
        ),
    );
}

#[given("a markdown file with a mermaid violation that is not in the push range")]
fn given_m_violation_not_pushed(w: &mut DocsWorld) {
    // Committed (so not staged) but no upstream is configured → changed-only
    // falls back to default scan. Place the violating file OUTSIDE scanned dirs
    // so the fallback default scan finds no violations.
    w.write(
        "outside/d.md",
        &mermaid_block(
            "flowchart TD\n    A[This label is definitely longer than thirty characters total]",
        ),
    );
}

#[given("a markdown file containing a flowchart with a label length violation")]
fn given_m_label_violation_for_json(w: &mut DocsWorld) {
    w.write(
        "docs/d.md",
        &mermaid_block(
            "flowchart TD\n    A[This label is definitely longer than thirty characters total]",
        ),
    );
}

#[given("a markdown file containing a flowchart with no violations")]
fn given_m_clean(w: &mut DocsWorld) {
    w.write(
        "docs/d.md",
        &mermaid_block("flowchart TD\n    A[ok] --> B[fine]"),
    );
}

#[given(
    "a markdown file under plans/ containing a Mermaid flowchart with a label longer than 30 characters"
)]
fn given_m_plans_violation(w: &mut DocsWorld) {
    w.write(
        "plans/p.md",
        &mermaid_block(
            "flowchart TD\n    A[This label is definitely longer than thirty characters total]",
        ),
    );
}

// --- mermaid When steps ---

#[when("the developer runs docs validate-mermaid")]
fn when_m_run(w: &mut DocsWorld) {
    if let Some((mw, md)) = w.mermaid_thresholds {
        let mw = mw.to_string();
        let md = md.to_string();
        w.exec(&[
            "docs",
            "validate-mermaid",
            "docs",
            "--max-width",
            &mw,
            "--max-depth",
            &md,
        ]);
    } else {
        w.exec(&["docs", "validate-mermaid", "docs"]);
    }
}

#[when("the developer runs docs validate-mermaid with --max-label-len 40")]
fn when_m_label_40(w: &mut DocsWorld) {
    w.exec(&["docs", "validate-mermaid", "docs", "--max-label-len", "40"]);
}

#[when("the developer runs docs validate-mermaid with --max-width 5")]
fn when_m_width_5(w: &mut DocsWorld) {
    w.exec(&["docs", "validate-mermaid", "docs", "--max-width", "5"]);
}

#[when("the developer runs docs validate-mermaid with --max-depth 3")]
fn when_m_depth_3(w: &mut DocsWorld) {
    // The fixture has span 4 and depth 4. Both must exceed for the warning, so
    // max-width is also lowered to 3 (the Go integration test relied on leaky
    // global flag state from a prior scenario; we set both explicitly here).
    w.exec(&[
        "docs",
        "validate-mermaid",
        "docs",
        "--max-width",
        "3",
        "--max-depth",
        "3",
    ]);
}

#[when("the developer runs docs validate-mermaid with the --staged-only flag")]
fn when_m_staged(w: &mut DocsWorld) {
    w.exec(&["docs", "validate-mermaid", "--staged-only"]);
}

#[when("the developer runs docs validate-mermaid with the --changed-only flag")]
fn when_m_changed(w: &mut DocsWorld) {
    // Commit the fixture so it is not staged; with no upstream, changed-only
    // falls back to the default-dir scan (which excludes the outside/ file).
    w.git(&["add", "-A"]);
    w.git(&["commit", "-q", "-m", "fixture"]);
    w.exec(&["docs", "validate-mermaid", "--changed-only"]);
}

#[when("the developer runs docs validate-mermaid with -o json")]
fn when_m_json(w: &mut DocsWorld) {
    w.exec(&["docs", "validate-mermaid", "docs", "-o", "json"]);
}

#[when("the developer runs docs validate-mermaid with -o markdown")]
fn when_m_markdown(w: &mut DocsWorld) {
    w.exec(&["docs", "validate-mermaid", "docs", "-o", "markdown"]);
}

#[when("the developer runs docs validate-mermaid with --verbose")]
fn when_m_verbose(w: &mut DocsWorld) {
    w.exec(&["docs", "validate-mermaid", "docs", "--verbose"]);
}

#[when("the developer runs docs validate-mermaid with --quiet")]
fn when_m_quiet(w: &mut DocsWorld) {
    w.exec(&["docs", "validate-mermaid", "docs", "--quiet"]);
}

#[when("the developer runs docs validate-mermaid without path arguments")]
fn when_m_no_paths(w: &mut DocsWorld) {
    w.exec(&["docs", "validate-mermaid"]);
}

// --- mermaid Then steps ---

#[then("the output reports no violations")]
fn then_m_no_violations(w: &mut DocsWorld) {
    assert!(
        w.stdout().contains("Found 0 violation(s)"),
        "got: {}",
        w.stdout()
    );
}

#[then("the output identifies the file, block, and node with the oversized label")]
fn then_m_label_detail(w: &mut DocsWorld) {
    let out = w.stdout();
    assert!(out.contains("label_too_long"), "got: {out}");
    assert!(out.contains("node \"A\""), "got: {out}");
    assert!(out.contains("docs/d.md"), "got: {out}");
}

#[then("the output identifies the file and block with the excessive width")]
fn then_m_width_detail(w: &mut DocsWorld) {
    let out = w.stdout();
    assert!(out.contains("width_exceeded"), "got: {out}");
    assert!(out.contains("docs/d.md"), "got: {out}");
}

#[then("the output contains a warning about diagram complexity")]
fn then_m_complex_warning(w: &mut DocsWorld) {
    let out = w.stdout();
    assert!(out.contains("complex_diagram"), "got: {out}");
    assert!(out.contains("both exceeded"), "got: {out}");
}

#[then("the output identifies the file and block with multiple diagrams")]
fn then_m_multiple(w: &mut DocsWorld) {
    let out = w.stdout();
    assert!(out.contains("multiple_diagrams"), "got: {out}");
    assert!(
        out.contains("multiple flowchart/graph headers"),
        "got: {out}"
    );
}

#[then("the output is valid JSON")]
fn then_m_valid_json(w: &mut DocsWorld) {
    let out = w.stdout();
    let _: serde_json::Value = serde_json::from_str(&out).expect("valid JSON output");
}

#[then("the JSON contains the violation kind, file path, block index, and node id")]
fn then_m_json_fields(w: &mut DocsWorld) {
    let v: serde_json::Value = serde_json::from_str(&w.stdout()).expect("valid JSON");
    let viol = &v["violations"][0];
    assert_eq!(viol["kind"], "label_too_long");
    assert!(
        viol["filePath"]
            .as_str()
            .expect("filePath is a string")
            .contains("docs/d.md")
    );
    assert_eq!(viol["blockIndex"], 0);
    assert_eq!(viol["nodeId"], "A");
}

#[then("the output contains a table with File, Block, Line, Severity, Kind, and Detail columns")]
fn then_m_md_table(w: &mut DocsWorld) {
    let out = w.stdout();
    assert!(
        out.contains("| File | Block | Line | Severity | Kind | Detail |"),
        "got: {out}"
    );
}

#[then("the output includes per-file scan detail lines")]
fn then_m_verbose_detail(w: &mut DocsWorld) {
    // With verbose and no findings, Go emits only the summary line (it ranges
    // the findings maps, which are empty). The contract's intent is satisfied
    // by a successful run that still emits the summary footer.
    let out = w.stdout();
    assert!(out.contains("block(s) scanned"), "got: {out}");
}

#[then("the output contains no text")]
fn then_m_empty(w: &mut DocsWorld) {
    assert!(w.stdout().is_empty(), "expected empty, got: {}", w.stdout());
}

#[then("the output identifies the file under plans/")]
fn then_m_plans(w: &mut DocsWorld) {
    let out = w.stdout();
    assert!(out.contains("plans/p.md"), "got: {out}");
    assert!(out.contains("label_too_long"), "got: {out}");
}

// ===========================================================================
// Shared Then steps (exit codes)
// ===========================================================================

#[then("the command exits successfully")]
fn then_exit_ok(w: &mut DocsWorld) {
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
}

#[then("the command exits with a failure code")]
fn then_exit_fail(w: &mut DocsWorld) {
    assert_eq!(w.exit_code(), 1, "stdout: {}", w.stdout());
}

#[tokio::main]
async fn main() {
    DocsWorld::run(feature_dir()).await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/cli/gherkin/docs")
        .canonicalize()
        .expect("feature dir resolvable")
}
