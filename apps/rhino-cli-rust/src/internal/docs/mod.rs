//! Markdown internal-link scanning and validation.
//!
//! Byte-for-byte port of the Go `internal/docs` link package
//! (`apps/rhino-cli-go/internal/docs/links_*.go`). Scans markdown files for
//! `[text](url)` links and validates that internal links resolve to existing
//! files and that `#fragment` anchors match a heading in the target file
//! (`broken-anchor` findings, via [`headings`]). External URLs, Hugo paths,
//! and placeholder links are skipped.

pub mod categorizer;
pub mod fences;
pub mod heading_hierarchy;
pub mod headings;
pub mod reporter;
pub mod scanner;
pub mod types;
pub mod validator;
