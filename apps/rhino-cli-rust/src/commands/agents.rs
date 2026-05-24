//! `agents sync`, `agents validate-claude`, `agents validate-sync`, and
//! `agents validate-naming` commands.
//!
//! Byte-for-byte ports of the Go `cmd/agents_*.go` handlers. Output is written
//! with `print!` (no implicit trailing newline) to mirror Go's `Fprint`. The
//! cobra-style usage blocks (printed to stderr on error by the dispatcher)
//! reproduce the Go binary's help text, including cobra's alphabetical flag
//! ordering.

use anyhow::{anyhow, Error};
use clap::Args;

use crate::internal::agents::{
    bindings, claude_validator, naming, reporter, sync, sync_validator,
    types::{SyncOptions, ValidateClaudeOptions},
};
use crate::internal::cliout::OutputFormat;
use crate::internal::git;

// ---------------------------------------------------------------------------
// sync
// ---------------------------------------------------------------------------

/// Cobra-style usage block printed to stderr when `sync` errors.
pub const SYNC_USAGE: &str = "Usage:\n  \
rhino-cli agents sync [flags]\n\n\
Examples:\n  \
# Sync all agents and skills\n  \
rhino-cli agents sync\n\n  \
# Preview changes without modifying files\n  \
rhino-cli agents sync --dry-run\n\n  \
# Sync only agents (skip skills)\n  \
rhino-cli agents sync --agents-only\n\n  \
# Sync only skills (skip agents)\n  \
rhino-cli agents sync --skills-only\n\n  \
# Output as JSON\n  \
rhino-cli agents sync -o json\n\n  \
# Verbose mode\n  \
rhino-cli agents sync -v\n\n\
Flags:\n      \
--agents-only   sync only agents (skip skills)\n      \
--dry-run       preview changes without modifying files\n  \
-h, --help          help for sync\n      \
--skills-only   sync only skills (skip agents)\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

#[derive(Args, Debug)]
pub struct SyncArgs {
    /// Preview changes without modifying files.
    #[arg(long = "dry-run")]
    pub dry_run: bool,
    /// Sync only agents (skip skills).
    #[arg(long = "agents-only")]
    pub agents_only: bool,
    /// Sync only skills (skip agents).
    #[arg(long = "skills-only")]
    pub skills_only: bool,
}

pub fn run_sync(
    args: &SyncArgs,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<(), Error> {
    // Validate flags (mirrors Go's exact message).
    if args.agents_only && args.skills_only {
        return Err(anyhow!("cannot use both --agents-only and --skills-only"));
    }

    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let opts = SyncOptions {
        repo_root,
        dry_run: args.dry_run,
        agents_only: args.agents_only,
        skills_only: args.skills_only,
        verbose,
        quiet,
    };

    let result = sync::sync_all(&opts).map_err(|e| anyhow!("sync failed: {e}"))?;

    let out = match output {
        OutputFormat::Text => reporter::format_sync_text(&result, verbose, quiet),
        OutputFormat::Json => reporter::format_sync_json(&result)?,
        OutputFormat::Markdown => reporter::format_sync_markdown(&result),
    };
    print!("{out}");

    if !result.failed_files.is_empty() {
        return Err(anyhow!(
            "sync completed with {} failures",
            result.failed_files.len()
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// validate-claude
// ---------------------------------------------------------------------------

/// Cobra-style usage block printed to stderr when `validate-claude` errors.
pub const VALIDATE_CLAUDE_USAGE: &str = "Usage:\n  \
rhino-cli agents validate-claude [flags]\n\n\
Examples:\n  \
# Validate all agents and skills\n  \
rhino-cli agents validate-claude\n\n  \
# Output as JSON\n  \
rhino-cli agents validate-claude -o json\n\n  \
# Verbose mode (show all checks)\n  \
rhino-cli agents validate-claude -v\n\n  \
# Validate only agents\n  \
rhino-cli agents validate-claude --agents-only\n\n  \
# Validate only skills\n  \
rhino-cli agents validate-claude --skills-only\n\n\
Flags:\n      \
--agents-only   validate only agents\n  \
-h, --help          help for validate-claude\n      \
--skills-only   validate only skills\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

#[derive(Args, Debug)]
pub struct ValidateClaudeArgs {
    /// Validate only agents.
    #[arg(long = "agents-only")]
    pub agents_only: bool,
    /// Validate only skills.
    #[arg(long = "skills-only")]
    pub skills_only: bool,
}

pub fn run_validate_claude(
    args: &ValidateClaudeArgs,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<(), Error> {
    if args.agents_only && args.skills_only {
        return Err(anyhow!(
            "cannot use --agents-only and --skills-only together"
        ));
    }

    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let opts = ValidateClaudeOptions {
        repo_root,
        agents_only: args.agents_only,
        skills_only: args.skills_only,
    };

    let result =
        claude_validator::validate_claude(&opts).map_err(|e| anyhow!("validation failed: {e}"))?;

    let out = match output {
        OutputFormat::Text => reporter::format_validation_text(&result, verbose, quiet),
        OutputFormat::Json => reporter::format_validation_json(&result)?,
        OutputFormat::Markdown => reporter::format_validation_markdown(&result, verbose),
    };
    print!("{out}");

    if result.failed_checks > 0 {
        return Err(anyhow!(
            "validation failed: {} checks failed",
            result.failed_checks
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// validate-sync
// ---------------------------------------------------------------------------

/// Cobra-style usage block printed to stderr when `validate-sync` errors.
pub const VALIDATE_SYNC_USAGE: &str = "Usage:\n  \
rhino-cli agents validate-sync [flags]\n\n\
Examples:\n  \
# Validate sync\n  \
rhino-cli agents validate-sync\n\n  \
# Output as JSON\n  \
rhino-cli agents validate-sync -o json\n\n  \
# Verbose mode (show all checks)\n  \
rhino-cli agents validate-sync -v\n\n  \
# Quiet mode (show only summary)\n  \
rhino-cli agents validate-sync -q\n\n\
Flags:\n  \
-h, --help   help for validate-sync\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

pub fn run_validate_sync(output: OutputFormat, verbose: bool, quiet: bool) -> Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let result =
        sync_validator::validate_sync(&repo_root).map_err(|e| anyhow!("validation failed: {e}"))?;

    let out = match output {
        OutputFormat::Text => reporter::format_validation_text(&result, verbose, quiet),
        OutputFormat::Json => reporter::format_validation_json(&result)?,
        OutputFormat::Markdown => reporter::format_validation_markdown(&result, verbose),
    };
    print!("{out}");

    if result.failed_checks > 0 {
        return Err(anyhow!(
            "validation failed: {} checks failed",
            result.failed_checks
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// validate-naming
// ---------------------------------------------------------------------------

/// Cobra-style usage block printed to stderr when `validate-naming` errors.
pub const VALIDATE_NAMING_USAGE: &str = "Usage:\n  \
rhino-cli agents validate-naming [flags]\n\n\
Examples:\n  \
# Validate agent naming across both harnesses\n  \
rhino-cli agents validate-naming\n\n  \
# Output as JSON\n  \
rhino-cli agents validate-naming -o json\n\n  \
# Markdown output (for PR comments, reports)\n  \
rhino-cli agents validate-naming -o markdown\n\n\
Flags:\n  \
-h, --help   help for validate-naming\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

pub fn run_validate_naming(output: OutputFormat, verbose: bool, quiet: bool) -> Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let violations =
        naming::validate_naming(&repo_root).map_err(|e| anyhow!("validation failed: {e}"))?;

    let out = match output {
        OutputFormat::Text => reporter::format_naming_text("Agents", &violations, verbose, quiet),
        OutputFormat::Json => reporter::format_naming_json("agents", &violations)?,
        OutputFormat::Markdown => reporter::format_naming_markdown("Agents", &violations),
    };
    print!("{out}");

    if !violations.is_empty() {
        return Err(anyhow!("{} naming violation(s) found", violations.len()));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// emit-bindings
// ---------------------------------------------------------------------------

/// Cobra-style usage block printed to stderr when `emit-bindings` errors.
pub const EMIT_BINDINGS_USAGE: &str = "Usage:\n  \
rhino-cli agents emit-bindings [flags]\n\n\
Examples:\n  \
# Write the binding files\n  \
rhino-cli agents emit-bindings\n\n  \
# Preview without writing\n  \
rhino-cli agents emit-bindings --dry-run\n\n\
Flags:\n      \
--dry-run   Preview the files that would be written without writing them\n  \
-h, --help      help for emit-bindings\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

#[derive(Args, Debug)]
pub struct EmitBindingsArgs {
    /// Preview changes without writing files.
    #[arg(long)]
    pub dry_run: bool,
}

pub fn run_emit_bindings(args: &EmitBindingsArgs) -> Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let result = bindings::emit_bindings(&repo_root, args.dry_run)?;
    print!("{}", result.output);
    Ok(())
}

// ---------------------------------------------------------------------------
// validate-bindings
// ---------------------------------------------------------------------------

/// Cobra-style usage block printed to stderr when `validate-bindings` errors.
pub const VALIDATE_BINDINGS_USAGE: &str = "Usage:\n  \
rhino-cli agents validate-bindings [flags]\n\n\
Examples:\n  \
# Validate bindings\n  \
rhino-cli agents validate-bindings\n\n\
Flags:\n  \
-h, --help   help for validate-bindings\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

pub fn run_validate_bindings() -> Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let result = bindings::validate_bindings(&repo_root);
    print!("{}", result.output);

    if result.problems > 0 {
        return Err(anyhow!(
            "binding validation failed: {} problem(s)",
            result.problems
        ));
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn sync_args_defaults() {
        let args = SyncArgs {
            dry_run: false,
            agents_only: false,
            skills_only: false,
        };
        assert!(!args.dry_run);
    }

    #[test]
    fn validate_claude_args_defaults() {
        let args = ValidateClaudeArgs {
            agents_only: false,
            skills_only: false,
        };
        assert!(!args.agents_only);
    }
}
