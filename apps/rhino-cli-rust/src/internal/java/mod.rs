//! Java source-tree validation: null-safety annotation enforcement.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/internal/java/`. The `java
//! validate-annotations` command scans a source root and verifies every package
//! (any directory containing at least one `.java` file) has a
//! `package-info.java` carrying the required null-safety annotation.

pub mod reporter;
pub mod scanner;
pub mod types;
pub mod validator;
