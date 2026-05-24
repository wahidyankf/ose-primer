//! Markdown internal-link scanning and validation.
//!
//! Byte-for-byte port of the Go `internal/docs` link package
//! (`apps/rhino-cli-go/internal/docs/links_*.go`). Scans markdown files for
//! `[text](url)` links and validates that internal links resolve to existing
//! files. External URLs, Hugo paths, anchors, and placeholder links are skipped.

pub mod categorizer;
pub mod reporter;
pub mod scanner;
pub mod types;
pub mod validator;
