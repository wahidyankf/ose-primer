//! `git pre-commit` — runs the rhino-cli pre-commit validation suite.
//!
//! Adapter for ose-primer's pre-commit hook orchestrator.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::git;
use crate::internal::git::runner::{Deps, run};

/// CLI arguments for `git pre-commit` (none required).
#[derive(Args, Debug)]
pub struct PreCommitArgs {}

/// Run the `git pre-commit` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found or if any pre-commit
/// check fails.
pub fn run_cmd(_args: &PreCommitArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
    let git_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    run(&git_root, Deps::production())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_constructible() {
        let _ = PreCommitArgs {};
    }
}
