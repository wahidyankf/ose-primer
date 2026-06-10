//! Contract codegen post-processing: Java import cleaning and Dart scaffolding.
//!
//! Backs the `contracts java-clean-imports` and `contracts dart-scaffold` commands.

pub mod dart_scaffold;
pub mod java_clean_imports;
pub mod reporter;
pub mod types;
