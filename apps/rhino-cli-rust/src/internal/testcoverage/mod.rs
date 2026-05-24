//! Test coverage parsing, classification, and reporting.
//!
//! Byte-for-byte port of the Go `internal/testcoverage` package
//! (`apps/rhino-cli-go/internal/testcoverage`). Auto-detects Go cover.out,
//! LCOV, JaCoCo XML, and Cobertura XML; classifies each line as covered /
//! partial / missed; and computes `pct = covered / (covered + partial + missed)`
//! with partial counted as not covered.

pub mod cobertura;
pub mod detect;
pub mod diff;
pub mod exclude;
pub mod go_coverage;
pub mod jacoco;
pub mod lcov;
pub mod merge;
pub mod reporter;
pub mod types;
