//! `env validate` command: detect drift between .env.example and source reads.
//!
//! Byte-for-byte port target: `apps/rhino-cli-go/cmd/env_validate.go`.

use anyhow::{Context, Error};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::envvalidate::validator::validate_surface;
use crate::internal::envvalidate::{SURFACES, ValidateResult, format_json, format_text};
use crate::internal::git::root::find_root;

pub const ENV_VALIDATE_USAGE: &str = "Usage:\n  \
rhino-cli env validate [flags]\n\n\
Flags:\n  \
-h, --help   help for validate\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

#[derive(Args, Debug)]
pub struct EnvValidateArgs {}

pub fn run_env_validate(
    _args: &EnvValidateArgs,
    output: OutputFormat,
    _verbose: bool,
    _quiet: bool,
) -> Result<(), Error> {
    let repo_root = find_root().context("failed to find git repository root")?;

    let mut surfaces = Vec::new();
    for surface in SURFACES {
        let result = validate_surface(&repo_root, surface)
            .with_context(|| format!("env validate failed for {}", surface.app))?;
        surfaces.push(result);
    }

    let result = ValidateResult { surfaces };

    let out = match output {
        OutputFormat::Json => format_json(&result).context("failed to format JSON")?,
        _ => format_text(&result),
    };

    print!("{out}");

    if !result.is_ok() {
        return Err(anyhow::anyhow!("env validate found violations"));
    }
    Ok(())
}
