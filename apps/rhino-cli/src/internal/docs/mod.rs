//! Markdown internal-link scanning and validation.
//!
//! Scans markdown files for `[text](url)` links and validates that internal links resolve
//! to existing files and that `#fragment` anchors match a heading in the target file
//! (`broken-anchor` findings, via [`headings`]). External URLs, absolute paths, and placeholder
//! links are skipped.

pub mod categorizer;
pub mod fences;
pub mod heading_hierarchy;
pub mod headings;
pub mod links;
pub mod reporter;
pub mod scanner;
pub mod types;
pub mod validator;

// Re-export application-layer modules expected by the ose-public command surface.
/// YAML frontmatter validation (from application layer).
pub use crate::application::docs::frontmatter;
/// Markdown filename convention validation (from application layer).
pub use crate::application::docs::naming;
