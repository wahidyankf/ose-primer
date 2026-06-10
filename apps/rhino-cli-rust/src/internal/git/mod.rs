//! Git helpers and the pre-commit hook orchestrator.
//!
//! [`root::find_root`] walks up the directory tree looking for `.git`.
//! [`runner::run`] orchestrates the `git pre-commit` hook steps.

pub mod root;
pub mod runner;
