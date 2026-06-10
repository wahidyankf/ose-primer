//! Development-environment health-check subsystem.
//!
//! Probes required tools, parses and compares their versions against repository
//! config, optionally installs missing tools (`--fix`), and renders
//! text/JSON/markdown reports.

pub mod checker;
pub mod fixer;
pub mod reporter;
pub mod tools;
pub mod types;

pub use checker::{check_all, check_all_with, real_runner};
pub use fixer::{FixOptions, FixResult, fix_all, format_fix_summary, real_fix_runner};
pub use reporter::{format_json, format_markdown, format_text};
pub use types::{DoctorResult, Scope, ToolCheck, ToolStatus};
