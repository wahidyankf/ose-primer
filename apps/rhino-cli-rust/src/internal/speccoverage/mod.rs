//! Multi-language BDD spec coverage validation.
//!
//! Walks a specs tree for `.feature` files and validates that each has matching test files,
//! scenarios, and step definitions across many languages.

pub mod checker;
pub mod cucumber_expr;
pub mod extractors;
pub mod parser;
pub mod reporter;
pub mod types;
pub mod util;
