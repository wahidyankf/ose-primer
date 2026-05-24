//! `test-coverage` command family: `validate`, `diff`, `merge`.
//!
//! Byte-for-byte ports of the Go `cmd/test_coverage_*.go` handlers. Output is
//! written with `print!` (no trailing newline added) to mirror Go's `Fprint`.

use anyhow::{Context, Error, anyhow};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::git;
use crate::internal::testcoverage::diff::{DiffCoverageOptions, compute_diff_coverage};
use crate::internal::testcoverage::merge::{
    CoverageMap, merge_coverage_maps, result_from_coverage_map, to_coverage_map, write_lcov,
};
use crate::internal::testcoverage::{
    cobertura, detect, exclude, go_coverage, jacoco, lcov, reporter,
    types::{Format, Result as CoverageResult},
};

/// Cobra-style usage block printed to stderr when `validate` returns an error.
pub const VALIDATE_USAGE: &str = "Usage:\n  \
rhino-cli test-coverage validate <coverage-file> <threshold> [flags]\n\n\
Examples:\n  \
# Check Go coverage\n  \
rhino-cli test-coverage validate apps/rhino-cli/cover.out 85\n\n  \
# Check LCOV coverage\n  \
rhino-cli test-coverage validate apps/crud-fe-ts-nextjs/coverage/lcov.info 85\n\n  \
# Check JaCoCo XML coverage\n  \
rhino-cli test-coverage validate apps/crud-be-java-springboot/target/site/jacoco-integration/jacoco.xml 85\n\n  \
# Output as JSON\n  \
rhino-cli test-coverage validate apps/rhino-cli/cover.out 85 -o json\n\n  \
# Output as markdown\n  \
rhino-cli test-coverage validate apps/rhino-cli/cover.out 85 -o markdown\n\n\
Flags:\n      \
--below-threshold float   with --per-file, show only files below this coverage percentage\n      \
--exclude stringArray     exclude files matching glob pattern (repeatable)\n  \
-h, --help                    help for validate\n      \
--per-file                show per-file coverage breakdown\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

/// Cobra-style usage block printed to stderr when `diff` returns an error.
pub const DIFF_USAGE: &str = "Usage:\n  \
rhino-cli test-coverage diff <coverage-file> [flags]\n\n\
Examples:\n  \
# Diff coverage against main\n  \
rhino-cli test-coverage diff apps/myapp/coverage/lcov.info\n\n  \
# Diff against specific branch\n  \
rhino-cli test-coverage diff apps/myapp/coverage/lcov.info --base develop\n\n  \
# Fail if diff coverage below 80%\n  \
rhino-cli test-coverage diff apps/myapp/coverage/lcov.info --threshold 80\n\n  \
# Show per-file breakdown\n  \
rhino-cli test-coverage diff apps/myapp/coverage/lcov.info --per-file\n\n\
Flags:\n      \
--base string           git ref to diff against (default \"main\")\n      \
--exclude stringArray   exclude files matching glob pattern (repeatable)\n  \
-h, --help                  help for diff\n      \
--per-file              show per-file diff coverage breakdown\n      \
--staged                diff staged changes instead of branch diff\n      \
--threshold float       fail if diff coverage below this percentage\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

/// Cobra-style usage block printed to stderr when `merge` returns an error.
pub const MERGE_USAGE: &str = "Usage:\n  \
rhino-cli test-coverage merge <file1> <file2> [file3...] [flags]\n\n\
Examples:\n  \
# Merge two LCOV files\n  \
rhino-cli test-coverage merge coverage1.info coverage2.info --out-file merged.info\n\n  \
# Merge and validate\n  \
rhino-cli test-coverage merge unit.info integration.info --out-file merged.info --validate 90\n\n  \
# Merge with exclusion\n  \
rhino-cli test-coverage merge coverage.info --exclude \"generated/*\" --out-file merged.info\n\n\
Flags:\n      \
--exclude stringArray   exclude files matching glob pattern (repeatable)\n  \
-h, --help                  help for merge\n      \
--out-file string       output file path (LCOV format)\n      \
--validate string       validate merged coverage against threshold\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

// --- validate ---

#[derive(Args, Debug)]
pub struct ValidateArgs {
    /// Coverage file path relative to git repo root.
    pub coverage_file: String,
    /// Threshold percentage (e.g. 85).
    pub threshold: String,
    /// Show per-file coverage breakdown.
    #[arg(long = "per-file")]
    pub per_file: bool,
    /// With --per-file, show only files below this coverage percentage.
    #[arg(long = "below-threshold", default_value_t = 0.0)]
    pub below_threshold: f64,
    /// Exclude files matching glob pattern (repeatable).
    #[arg(long = "exclude", value_name = "PATTERN")]
    pub exclude: Vec<String>,
}

pub fn run_validate(
    args: &ValidateArgs,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let abs_path = repo_root.join(&args.coverage_file);

    let threshold: f64 = args.threshold.parse().map_err(|_| {
        anyhow!(
            "invalid threshold {:?}: must be a number (e.g. 85)",
            args.threshold
        )
    })?;

    let abs_path_str = abs_path
        .to_str()
        .ok_or_else(|| anyhow!("non-utf8 coverage file path"))?;

    let mut result: CoverageResult = match detect::detect_format(abs_path_str) {
        Format::Lcov => {
            lcov::compute_lcov_result(abs_path_str, threshold).context("coverage check failed")?
        }
        Format::Jacoco => jacoco::compute_jacoco_result(abs_path_str, threshold)
            .context("coverage check failed")?,
        Format::Cobertura => cobertura::compute_cobertura_result(abs_path_str, threshold)
            .context("coverage check failed")?,
        Format::Go => go_coverage::compute_go_result(abs_path_str, threshold)
            .context("coverage check failed")?,
        Format::Diff => {
            return Err(anyhow!(
                "diff format is not a valid input format for validate"
            ));
        }
    };

    if !args.exclude.is_empty() {
        exclude::exclude_files(&mut result, &args.exclude);
    }

    let per_file_text = if args.per_file {
        reporter::format_text_per_file(&result, args.below_threshold)
    } else {
        String::new()
    };

    let out = match output {
        OutputFormat::Text => format!(
            "{}{per_file_text}",
            reporter::format_text(&result, verbose, quiet)
        ),
        OutputFormat::Json => reporter::format_json(&result, args.per_file, args.below_threshold)?,
        OutputFormat::Markdown => {
            reporter::format_markdown(&result, args.per_file, args.below_threshold)
        }
    };
    print!("{out}");

    if !result.passed {
        return Err(anyhow!(
            "coverage {:.2}% is below threshold {:.0}%",
            result.pct,
            threshold
        ));
    }
    Ok(())
}

// --- diff ---

#[derive(Args, Debug)]
pub struct DiffArgs {
    /// Coverage file path relative to git repo root.
    pub coverage_file: String,
    /// Git ref to diff against.
    #[arg(long = "base", default_value = "main")]
    pub base: String,
    /// Fail if diff coverage below this percentage.
    #[arg(long = "threshold", default_value_t = 0.0)]
    pub threshold: f64,
    /// Diff staged changes instead of branch diff.
    #[arg(long = "staged")]
    pub staged: bool,
    /// Show per-file diff coverage breakdown.
    #[arg(long = "per-file")]
    pub per_file: bool,
    /// Exclude files matching glob pattern (repeatable).
    #[arg(long = "exclude", value_name = "PATTERN")]
    pub exclude: Vec<String>,
}

pub fn run_diff(
    args: &DiffArgs,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let abs_path = repo_root.join(&args.coverage_file);
    let abs_path_str = abs_path
        .to_str()
        .ok_or_else(|| anyhow!("non-utf8 coverage file path"))?;

    let opts = DiffCoverageOptions {
        coverage_file: abs_path_str.to_string(),
        base: args.base.clone(),
        staged: args.staged,
        threshold: args.threshold,
        per_file: args.per_file,
        exclude_patterns: args.exclude.clone(),
    };

    let result = compute_diff_coverage(&opts).map_err(|e| anyhow!("diff coverage failed: {e}"))?;

    let per_file_text = if args.per_file {
        reporter::format_text_per_file(&result, 0.0)
    } else {
        String::new()
    };

    let out = match output {
        OutputFormat::Text => format!(
            "{}{per_file_text}",
            reporter::format_text(&result, verbose, quiet)
        ),
        OutputFormat::Json => reporter::format_json(&result, args.per_file, 0.0)?,
        OutputFormat::Markdown => reporter::format_markdown(&result, args.per_file, 0.0),
    };
    print!("{out}");

    if args.threshold > 0.0 && !result.passed {
        return Err(anyhow!(
            "diff coverage {:.2}% is below threshold {:.0}%",
            result.pct,
            args.threshold
        ));
    }
    Ok(())
}

// --- merge ---

#[derive(Args, Debug)]
pub struct MergeArgs {
    /// Coverage files (minimum 2) relative to git repo root.
    #[arg(num_args = 2.., required = true)]
    pub files: Vec<String>,
    /// Output file path (LCOV format).
    #[arg(long = "out-file", default_value = "")]
    pub out_file: String,
    /// Validate merged coverage against threshold.
    #[arg(long = "validate", default_value = "")]
    pub validate: String,
    /// Exclude files matching glob pattern (repeatable).
    #[arg(long = "exclude", value_name = "PATTERN")]
    pub exclude: Vec<String>,
}

pub fn run_merge(
    args: &MergeArgs,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let mut maps: Vec<CoverageMap> = Vec::with_capacity(args.files.len());
    for arg in &args.files {
        let abs_path = repo_root.join(arg);
        let abs_path_str = abs_path
            .to_str()
            .ok_or_else(|| anyhow!("non-utf8 coverage file path"))?;
        let cm =
            to_coverage_map(abs_path_str).map_err(|e| anyhow!("failed to parse {arg}: {e}"))?;
        maps.push(cm);
    }

    let mut merged = merge_coverage_maps(&maps);

    if !args.exclude.is_empty() {
        let to_drop: Vec<String> = merged
            .keys()
            .filter(|p| {
                crate::internal::testcoverage::exclude::matches_any_exclude_pattern(
                    p,
                    &args.exclude,
                )
            })
            .cloned()
            .collect();
        for p in to_drop {
            merged.remove(&p);
        }
    }

    if !args.out_file.is_empty() {
        let out_path = repo_root.join(&args.out_file);
        write_lcov(&merged, &out_path).map_err(|e| anyhow!("failed to write output: {e}"))?;
    }

    let threshold: f64 = if args.validate.is_empty() {
        0.0
    } else {
        args.validate.parse().map_err(|_| {
            anyhow!(
                "invalid --validate threshold {:?}: must be a number",
                args.validate
            )
        })?
    };

    let mut result = result_from_coverage_map(&merged, threshold);
    result.file = "merged".into();

    let out = match output {
        OutputFormat::Text => reporter::format_text(&result, verbose, quiet),
        OutputFormat::Json => reporter::format_json(&result, false, 0.0)?,
        OutputFormat::Markdown => reporter::format_markdown(&result, false, 0.0),
    };
    print!("{out}");

    if !args.validate.is_empty() && !result.passed {
        return Err(anyhow!(
            "merged coverage {:.2}% is below threshold {:.0}%",
            result.pct,
            threshold
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
        let _ = ValidateArgs {
            coverage_file: "x".into(),
            threshold: "85".into(),
            per_file: false,
            below_threshold: 0.0,
            exclude: vec![],
        };
    }

    #[test]
    fn diff_args_construct() {
        let _ = DiffArgs {
            coverage_file: "x".into(),
            base: "main".into(),
            threshold: 0.0,
            staged: false,
            per_file: false,
            exclude: vec![],
        };
    }

    #[test]
    fn merge_args_construct() {
        let _ = MergeArgs {
            files: vec!["a".into(), "b".into()],
            out_file: String::new(),
            validate: String::new(),
            exclude: vec![],
        };
    }
}
