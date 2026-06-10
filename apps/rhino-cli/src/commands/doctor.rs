//! `doctor` command: check required tool versions, optionally auto-install.
//!
//! Delegates the checks/fix to [`crate::internal::doctor`]. On error the dispatcher prints
//! [`DOCTOR_USAGE`] to stderr; only missing tools cause a non-zero exit (version warnings
//! are advisory).

use std::io::Write as _;

use anyhow::{Context, Error, anyhow};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::doctor::{self, FixOptions, Scope};
use crate::internal::git::root::find_root;

/// Usage block for `doctor`, printed to stderr on error.
pub const DOCTOR_USAGE: &str = "Usage:\n  \
rhino-cli doctor [flags]\n\n\
Examples:\n  \
# Check all required tools\n  \
rhino-cli doctor\n\n  \
# Check only core tools (git, volta, node, npm, golang, docker, jq)\n  \
rhino-cli doctor --scope minimal\n\n  \
# Output as JSON\n  \
rhino-cli doctor -o json\n\n  \
# Output as markdown report\n  \
rhino-cli doctor -o markdown\n\n  \
# Verbose output with duration\n  \
rhino-cli doctor --verbose\n\n  \
# Auto-install missing tools\n  \
rhino-cli doctor --fix\n\n  \
# Preview what would be installed\n  \
rhino-cli doctor --fix --dry-run\n\n  \
# Fix only core tools\n  \
rhino-cli doctor --fix --scope minimal\n\n\
Flags:\n      \
--dry-run        preview what --fix would install (only effective with --fix)\n      \
--fix            attempt to install missing tools\n  \
-h, --help           help for doctor\n      \
--scope string   tool scope: full or minimal (default \"full\")\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

#[derive(Args, Debug)]
pub struct DoctorArgs {
    /// Tool scope: full or minimal.
    #[arg(long, default_value = "full")]
    pub scope: String,
    /// Attempt to install missing tools.
    #[arg(long)]
    pub fix: bool,
    /// Preview what --fix would install (only effective with --fix).
    #[arg(long = "dry-run")]
    pub dry_run: bool,
}

/// Runs `doctor`.
pub fn run_doctor(
    args: &DoctorArgs,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<(), Error> {
    let repo_root = find_root().context("failed to find git repository root")?;

    // Go casts the raw string to Scope; only "minimal" filters, everything else
    // (including "full") runs the full set. The raw value is echoed into JSON.
    let scope_enum = if args.scope == "minimal" {
        Scope::Minimal
    } else {
        Scope::Full
    };

    let result =
        doctor::check_all(&repo_root, scope_enum, &args.scope).context("doctor check failed")?;

    let out = match output {
        OutputFormat::Text => doctor::format_text(&result, verbose, quiet),
        OutputFormat::Json => doctor::format_json(&result).context("failed to format JSON")?,
        OutputFormat::Markdown => doctor::format_markdown(&result),
    };
    print!("{out}");

    if args.fix && result.missing_count > 0 {
        let platform = if cfg!(target_os = "macos") {
            "darwin"
        } else {
            "linux"
        };
        let fix_opts = FixOptions {
            dry_run: args.dry_run,
        };
        let mut stdout = std::io::stdout();
        let printf = |line: &str| {
            print!("{line}");
            let _ = stdout.flush();
        };
        let fix_result = doctor::fix_all(
            &result,
            &repo_root,
            scope_enum,
            &fix_opts,
            platform,
            &doctor::real_fix_runner,
            printf,
        );
        print!("{}", doctor::format_fix_summary(&fix_result));

        if fix_result.failed > 0 {
            return Err(anyhow!("{} tool(s) failed to install", fix_result.failed));
        }
        if !args.dry_run && fix_result.fixed > 0 {
            return Ok(()); // Tools were fixed, don't report missing.
        }
    }

    if args.fix && result.missing_count == 0 {
        println!("\nNothing to fix \u{2014} all tools are installed.");
    }

    // Only missing tools cause a non-zero exit; version warnings are advisory.
    if result.missing_count > 0 {
        return Err(anyhow!(
            "{} tool(s) not found in PATH",
            result.missing_count
        ));
    }
    Ok(())
}
