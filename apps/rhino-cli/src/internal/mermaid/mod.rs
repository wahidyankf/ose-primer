//! Structural validation for Mermaid flowchart diagrams in markdown files.
//!
//! Enforces four rules — three blocking violations (label-too-long, width-exceeded,
//! multiple-diagrams) and two density/complexity warnings. Non-flowchart diagram types are
//! silently ignored. Parsing is pure regex/string based; no tree-sitter (matching Go).

pub mod extractor;
pub mod graph;
pub mod parser;
pub mod reporter;
pub mod types;
pub mod validator;
