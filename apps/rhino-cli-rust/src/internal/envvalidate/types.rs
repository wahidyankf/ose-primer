//! Types for the env-validate subsystem.
//!
//! Byte-for-byte port target: `apps/rhino-cli-go/internal/envvalidate/types.go`.

use std::collections::BTreeSet;

/// Result for a single app surface validation.
#[derive(Debug, Default, Clone)]
pub struct SurfaceResult {
    /// App name (basename of infra/dev/<app>/).
    pub app: String,
    /// Keys declared in .env.example but not found in any source read.
    pub declared_not_read: BTreeSet<String>,
    /// Keys read in source but not declared in .env.example.
    pub read_not_declared: BTreeSet<String>,
}

impl SurfaceResult {
    /// Returns true when there are no violations (after allowlist filtering).
    pub fn is_ok(&self) -> bool {
        self.declared_not_read.is_empty() && self.read_not_declared.is_empty()
    }
}

/// Aggregate result returned by the validate command.
#[derive(Debug, Default)]
pub struct ValidateResult {
    pub surfaces: Vec<SurfaceResult>,
}

impl ValidateResult {
    /// Returns true when all surfaces passed.
    pub fn is_ok(&self) -> bool {
        self.surfaces.iter().all(SurfaceResult::is_ok)
    }

    pub fn violation_count(&self) -> usize {
        self.surfaces
            .iter()
            .map(|s| s.declared_not_read.len() + s.read_not_declared.len())
            .sum()
    }
}
