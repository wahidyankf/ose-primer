//! Git helpers and the pre-commit hook orchestrator.
//!
//! [`root::find_root`] mirrors the Go `findGitRoot` (walks up the directory
//! tree looking for `.git`). [`runner::run`] orchestrates the `git pre-commit`
//! hook steps, mirroring `apps/rhino-cli-go/internal/git/runner.go`.

pub mod root;
pub mod runner;
