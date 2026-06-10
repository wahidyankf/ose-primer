//! `git` command family: `pre-commit`.
//!
//! The handler resolves the git root (failing with the same wrapped error as Go when no
//! `.git` is found) and delegates to the [`runner`](crate::internal::git::runner)
//! orchestrator.

use anyhow::{Error, anyhow};

use crate::internal::git;
use crate::internal::git::runner::{Deps, run};

/// Usage block printed to stderr when `pre-commit` returns an error.
pub const PRE_COMMIT_USAGE: &str = "Usage:\n  \
rhino-cli git pre-commit [flags]\n\n\
Flags:\n  \
-h, --help   help for pre-commit\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

/// Runs all pre-commit checks (config, lint, format, docs).
pub fn run_pre_commit() -> Result<(), Error> {
    let git_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    run(&git_root, Deps::production())
}
