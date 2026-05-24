//! `spec-coverage validate` command.
//!
//! Byte-for-byte port of the Go `cmd/spec_coverage_validate.go` handler. Takes
//! exactly two positional args (`<specs-dir> <app-dir>`). Output is written
//! with `print!` (no trailing newline) to mirror Go's `Fprint`.

use anyhow::{Context, Error, anyhow};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::git;
use crate::internal::speccoverage::{checker, reporter, types::ScanOptions};

/// Cobra-style usage block printed to stderr when `validate` returns an error.
pub const VALIDATE_USAGE: &str = "Usage:\n  \
rhino-cli spec-coverage validate <specs-dir> <app-dir> [flags]\n\n\
Examples:\n  \
# Check crud-fe-ts-nextjs spec coverage\n  \
rhino-cli spec-coverage validate specs/apps/crud-fe-ts-nextjs apps/crud-fe-ts-nextjs\n\n  \
# Output as JSON\n  \
rhino-cli spec-coverage validate specs/apps/crud-fe-ts-nextjs apps/crud-fe-ts-nextjs -o json\n\n  \
# Quiet mode\n  \
rhino-cli spec-coverage validate specs/apps/crud-fe-ts-nextjs apps/crud-fe-ts-nextjs -q\n\n\
Flags:\n      \
--exclude-dir strings   spec directory names to exclude (e.g., --exclude-dir test-support)\n  \
-h, --help                  help for validate\n      \
--shared-steps          skip file matching, validate steps across ALL source files\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

#[derive(Args, Debug)]
pub struct ValidateArgs {
    /// Specs directory, then app directory (both relative to git repo root).
    #[arg(required = true, num_args = 2, value_names = ["SPECS_DIR", "APP_DIR"])]
    pub paths: Vec<String>,
    /// Skip file matching; validate steps across ALL source files.
    #[arg(long = "shared-steps")]
    pub shared_steps: bool,
    /// Spec directory names to exclude (comma-separated or repeatable).
    #[arg(long = "exclude-dir", value_name = "DIR", value_delimiter = ',')]
    pub exclude_dir: Vec<String>,
}

pub fn run_validate(
    args: &ValidateArgs,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let specs_dir = repo_root.join(&args.paths[0]);
    let app_dir = repo_root.join(&args.paths[1]);

    let opts = ScanOptions {
        repo_root: repo_root.clone(),
        specs_dir,
        app_dir,
        verbose,
        quiet,
        shared_steps: args.shared_steps,
        exclude_dirs: args.exclude_dir.clone(),
    };

    let result = checker::check_all(&opts).context("spec coverage check failed")?;

    let out = match output {
        OutputFormat::Text => reporter::format_text(&result, verbose, quiet),
        OutputFormat::Json => reporter::format_json(&result)?,
        OutputFormat::Markdown => reporter::format_markdown(&result),
    };
    print!("{out}");

    let has_gaps =
        !result.gaps.is_empty() || !result.scenario_gaps.is_empty() || !result.step_gaps.is_empty();

    if has_gaps {
        if !quiet && matches!(output, OutputFormat::Text) {
            if !result.gaps.is_empty() {
                eprintln!(
                    "\n❌ Found {} spec(s) without matching test files",
                    result.gaps.len()
                );
            }
            if !result.scenario_gaps.is_empty() {
                eprintln!(
                    "❌ Found {} scenario(s) without matching test implementations",
                    result.scenario_gaps.len()
                );
            }
            if !result.step_gaps.is_empty() {
                eprintln!(
                    "❌ Found {} step(s) without matching step definitions",
                    result.step_gaps.len()
                );
            }
        }
        return Err(anyhow!(
            "spec coverage gaps found: {} file gap(s), {} scenario gap(s), {} step gap(s)",
            result.gaps.len(),
            result.scenario_gaps.len(),
            result.step_gaps.len()
        ));
    }

    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn validate_args_construct() {
        let args = ValidateArgs {
            paths: vec!["specs".into(), "app".into()],
            shared_steps: false,
            exclude_dir: vec![],
        };
        assert_eq!(args.paths.len(), 2);
    }
}
