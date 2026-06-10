//! `java` command family: `validate-annotations`.
//!
//! The source-root argument is resolved with Go `filepath.Abs` semantics. After emitting
//! the formatted output, when violations exist the handler prints a trailing `❌ Found N
//! violation(s)` line to stderr (text, non-quiet only) and returns an error so the process
//! exits non-zero — mirroring the Go command.

use anyhow::{Context, Error, anyhow};
use clap::Args;

use crate::commands::contracts::go_abs;
use crate::internal::cliout::OutputFormat;
use crate::internal::java::reporter;
use crate::internal::java::types::ValidationOptions;
use crate::internal::java::validator::validate_all;

/// Usage block printed to stderr when `validate-annotations` errors.
pub const VALIDATE_ANNOTATIONS_USAGE: &str = "Usage:\n  \
rhino-cli java validate-annotations <source-root> [flags]\n\n\
Examples:\n  \
# Validate with default annotation (@NullMarked)\n  \
rhino-cli java validate-annotations apps/crud-be-fsharp-giraffe-jasb/src/main/java\n\n  \
# Use a custom annotation\n  \
rhino-cli java validate-annotations apps/crud-be-fsharp-giraffe-jasb/src/main/java --annotation NonNull\n\n  \
# Output as JSON\n  \
rhino-cli java validate-annotations apps/crud-be-fsharp-giraffe-jasb/src/main/java -o json\n\n  \
# Output as markdown report\n  \
rhino-cli java validate-annotations apps/crud-be-fsharp-giraffe-jasb/src/main/java -o markdown\n\n\
Flags:\n      \
--annotation string   annotation name to require in package-info.java files (default \"NullMarked\")\n  \
-h, --help                help for validate-annotations\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

#[derive(Args, Debug)]
pub struct ValidateAnnotationsArgs {
    /// Java source root to scan.
    pub source_root: String,
    /// Annotation name to require in package-info.java files.
    #[arg(long, default_value = "NullMarked")]
    pub annotation: String,
}

/// Runs `java validate-annotations`.
pub fn run_validate_annotations(
    args: &ValidateAnnotationsArgs,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<(), Error> {
    let abs_source_root = go_abs(&args.source_root)
        .with_context(|| format!("failed to resolve source root {:?}", args.source_root))?;

    let opts = ValidationOptions {
        source_root: abs_source_root.to_string_lossy().into_owned(),
        annotation: args.annotation.clone(),
    };

    let result = validate_all(&opts).context("validation failed")?;

    let out = match output {
        OutputFormat::Text => reporter::format_text(&result, verbose, quiet),
        OutputFormat::Json => reporter::format_json(&result)?,
        OutputFormat::Markdown => reporter::format_markdown(&result),
    };
    print!("{out}");

    let num_violations = result.total_packages - result.valid_packages;
    if num_violations > 0 {
        if !quiet && output == OutputFormat::Text {
            eprintln!("\n❌ Found {num_violations} violation(s)");
        }
        return Err(anyhow!("found {num_violations} violation(s)"));
    }

    Ok(())
}
