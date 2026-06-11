//! Cucumber integration tests for `env validate`.
//!
//! Exercises `validate_surface` + `format_text` directly from the library crate
//! against temp-dir fixtures. The binary uses hardcoded SURFACES, so library-
//! level tests are the right layer for Gherkin fixture scenarios. Step text
//! mirrors the feature file verbatim for spec-coverage compatibility.

use cucumber::{World as _, given, then, when};
use tempfile::TempDir;

use rhino_cli::internal::envvalidate::{
    contract::AppSurface, format_text, types::ValidateResult, validator::validate_surface,
};

/// State shared across each scenario.
#[derive(cucumber::World)]
#[world(init = Self::new)]
struct EnvValidateWorld {
    /// Temp repo with synthetic fixture files.
    repo: TempDir,
    /// Result of the last `env validate` run.
    result: Option<ValidateResult>,
    /// Exit-code equivalent: true = success (exit 0), false = failure.
    exit_ok: Option<bool>,
    /// Formatted text output from the last run.
    output_text: Option<String>,
}

impl std::fmt::Debug for EnvValidateWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnvValidateWorld").finish_non_exhaustive()
    }
}

impl EnvValidateWorld {
    fn new() -> Self {
        let repo = TempDir::new().expect("temp repo");
        std::fs::create_dir_all(repo.path().join(".git")).expect("mk .git");
        Self {
            repo,
            result: None,
            exit_ok: None,
            output_text: None,
        }
    }

    fn write(&self, rel: &str, content: &str) {
        let p = self.repo.path().join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).expect("mk dir");
        }
        std::fs::write(p, content).expect("write fixture");
    }

    /// Run validate_surface for "fixture-app" (Rust source, no per-app allowlist).
    fn run_validate_fixture(&mut self) {
        let surface = AppSurface {
            app: "fixture-app",
            source_exts: &["rs"],
            source_subdir: "src",
            allowlist: &["FIXTURE_PORT"],
        };
        let sr = validate_surface(self.repo.path(), &surface).expect("validate_surface");
        let ok = sr.is_ok();
        let text = format_text(&ValidateResult {
            surfaces: vec![sr.clone()],
        });
        self.exit_ok = Some(ok);
        self.output_text = Some(text);
        self.result = Some(ValidateResult { surfaces: vec![sr] });
    }
}

// ===========================================================================
// Given steps
// ===========================================================================

#[given(
    "a fixture app whose .env.example declares FIXTURE_JWT_SECRET but whose source never reads it"
)]
fn given_declared_not_read(w: &mut EnvValidateWorld) {
    w.write(
        "infra/dev/fixture-app/.env.example",
        "FIXTURE_JWT_SECRET=change-me\n",
    );
    w.write(
        "apps/fixture-app/src/config.rs",
        "fn main() { println!(\"no env reads here\"); }\n",
    );
}

#[given(
    "a fixture app whose source reads FIXTURE_JWT_SECRET but whose .env.example does not declare it"
)]
fn given_read_not_declared(w: &mut EnvValidateWorld) {
    w.write("infra/dev/fixture-app/.env.example", "\n");
    w.write(
        "apps/fixture-app/src/config.rs",
        "let s = env::var(\"FIXTURE_JWT_SECRET\").unwrap();\n",
    );
}

#[given("a fixture app whose .env.example declares FIXTURE_JWT_SECRET and whose source reads it")]
fn given_matching(w: &mut EnvValidateWorld) {
    w.write(
        "infra/dev/fixture-app/.env.example",
        "FIXTURE_JWT_SECRET=change-me\n",
    );
    w.write(
        "apps/fixture-app/src/config.rs",
        "let s = env::var(\"FIXTURE_JWT_SECRET\").context(\"required\")?;\n",
    );
}

#[given(
    "a fixture app that reads ENABLE_TEST_API and a framework-injected PORT variant but neither is declared in .env.example"
)]
fn given_allowlisted_reads(w: &mut EnvValidateWorld) {
    w.write(
        "infra/dev/fixture-app/.env.example",
        "FIXTURE_JWT_SECRET=change-me\n",
    );
    w.write(
        "apps/fixture-app/src/config.rs",
        concat!(
            "let s = env::var(\"FIXTURE_JWT_SECRET\").context(\"required\")?;\n",
            "let t = env::var(\"ENABLE_TEST_API\").unwrap_or_default();\n",
            "let p = env::var(\"FIXTURE_PORT\").unwrap_or(\"8080\".into());\n",
        ),
    );
}

// ===========================================================================
// When step
// ===========================================================================

#[when("the developer runs rhino-cli env validate")]
fn when_run_env_validate(w: &mut EnvValidateWorld) {
    w.run_validate_fixture();
}

// ===========================================================================
// Then steps
// ===========================================================================

#[then("the command exits with a failure code")]
fn then_failure_exit(w: &mut EnvValidateWorld) {
    assert!(
        !w.exit_ok.expect("ran"),
        "expected non-zero exit, got success"
    );
}

#[then("the output names FIXTURE_JWT_SECRET as a declared-but-unread key")]
fn then_names_declared_not_read(w: &mut EnvValidateWorld) {
    let out = w.output_text.as_deref().expect("output");
    assert!(
        out.contains("FIXTURE_JWT_SECRET"),
        "expected FIXTURE_JWT_SECRET in output, got:\n{out}"
    );
    assert!(
        out.contains("declared-but-unread"),
        "expected 'declared-but-unread' in output, got:\n{out}"
    );
}

#[then("the output names FIXTURE_JWT_SECRET as a read-but-undeclared key")]
fn then_names_read_not_declared(w: &mut EnvValidateWorld) {
    let out = w.output_text.as_deref().expect("output");
    assert!(
        out.contains("FIXTURE_JWT_SECRET"),
        "expected FIXTURE_JWT_SECRET in output, got:\n{out}"
    );
    assert!(
        out.contains("read-but-undeclared"),
        "expected 'read-but-undeclared' in output, got:\n{out}"
    );
}

#[then("the command exits successfully")]
fn then_success_exit(w: &mut EnvValidateWorld) {
    assert!(
        w.exit_ok.expect("ran"),
        "expected exit 0, but validate found violations:\n{}",
        w.output_text.as_deref().unwrap_or("")
    );
}

#[then("the output reports validation passed")]
fn then_reports_passed(w: &mut EnvValidateWorld) {
    let out = w.output_text.as_deref().expect("output");
    assert!(
        out.contains("passed"),
        "expected 'passed' in output, got:\n{out}"
    );
}

// ===========================================================================
// Main
// ===========================================================================

#[tokio::main]
async fn main() {
    EnvValidateWorld::run(
        "../../specs/apps/rhino/behavior/rhino-cli/gherkin/env/env-validate.feature",
    )
    .await;
}
