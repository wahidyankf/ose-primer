//! Minimal git helpers needed by the coverage commands.
//!
//! Only [`root::find_root`] is required at this phase; it mirrors the Go
//! `findGitRoot` which walks up the directory tree looking for `.git`.

pub mod root;
