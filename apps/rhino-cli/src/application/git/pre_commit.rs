//! Pre-commit pipeline use case — backward-compat shim.
//!
//! The implementation lives in `crate::internal::git::runner`.
//! This module re-exports `Deps` and `run` so that callers using
//! `crate::application::git::pre_commit` continue to compile.
pub use crate::internal::git::runner::{Deps, run};
