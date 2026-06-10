//! `contracts` command family: `java-clean-imports`, `dart-scaffold`.
//!
//! The directory argument is resolved by joining it under the process CWD and
//! lexically cleaning it (NOT canonicalizing), so the walk and relative-path
//! reporting are stable across symlinks. Output is written with `print!` (no
//! trailing newline added).

use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Error};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::contracts::dart_scaffold::scaffold_dart;
use crate::internal::contracts::java_clean_imports::clean_java_imports;
use crate::internal::contracts::reporter;
use crate::internal::contracts::types::{DartScaffoldOptions, JavaCleanImportsOptions};

/// Usage block printed to stderr when `java-clean-imports` errors.
pub const JAVA_CLEAN_IMPORTS_USAGE: &str = "Usage:\n  \
rhino-cli contracts java-clean-imports <generated-contracts-dir> [flags]\n\n\
Examples:\n  \
rhino-cli contracts java-clean-imports apps/crud-be-java-springboot/generated-contracts\n  \
rhino-cli contracts java-clean-imports apps/crud-be-java-vertx/generated-contracts -o json\n\n\
Flags:\n  \
-h, --help   help for java-clean-imports\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

/// Usage block printed to stderr when `dart-scaffold` errors.
pub const DART_SCAFFOLD_USAGE: &str = "Usage:\n  \
rhino-cli contracts dart-scaffold <generated-contracts-dir> [flags]\n\n\
Examples:\n  \
rhino-cli contracts dart-scaffold apps/crud-fe-dart-flutterweb/generated-contracts\n  \
rhino-cli contracts dart-scaffold apps/crud-fe-dart-flutterweb/generated-contracts -o json\n\n\
Flags:\n  \
-h, --help   help for dart-scaffold\n\n\
Global Flags:\n      \
--no-color        disable colored output\n  \
-o, --output string   output format: text, json, markdown (default \"text\")\n  \
-q, --quiet           quiet mode (errors only)\n      \
--say string      echo a message to stdout\n  \
-v, --verbose         verbose output with timestamps\n\n";

#[derive(Args, Debug)]
pub struct JavaCleanImportsArgs {
    /// Generated-contracts directory to scan.
    pub dir: String,
}

#[derive(Args, Debug)]
pub struct DartScaffoldArgs {
    /// Generated-contracts directory to scaffold.
    pub dir: String,
}

/// Runs `contracts java-clean-imports`.
pub fn run_java_clean_imports(
    args: &JavaCleanImportsArgs,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<(), Error> {
    let abs_dir =
        go_abs(&args.dir).with_context(|| format!("failed to resolve directory {:?}", args.dir))?;

    let opts = JavaCleanImportsOptions {
        dir: abs_dir.to_string_lossy().into_owned(),
    };

    let result = clean_java_imports(&opts).context("java import cleaning failed")?;

    let out = match output {
        OutputFormat::Text => reporter::format_java_clean_imports_text(&result, verbose, quiet),
        OutputFormat::Json => reporter::format_java_clean_imports_json(&result)?,
        OutputFormat::Markdown => reporter::format_java_clean_imports_markdown(&result),
    };
    print!("{out}");
    Ok(())
}

/// Runs `contracts dart-scaffold`.
pub fn run_dart_scaffold(
    args: &DartScaffoldArgs,
    output: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<(), Error> {
    let abs_dir =
        go_abs(&args.dir).with_context(|| format!("failed to resolve directory {:?}", args.dir))?;

    let opts = DartScaffoldOptions {
        dir: abs_dir.to_string_lossy().into_owned(),
    };

    let result = scaffold_dart(&opts).context("dart scaffolding failed")?;

    let out = match output {
        OutputFormat::Text => reporter::format_dart_scaffold_text(&result, verbose, quiet),
        OutputFormat::Json => reporter::format_dart_scaffold_json(&result)?,
        OutputFormat::Markdown => reporter::format_dart_scaffold_markdown(&result),
    };
    print!("{out}");
    Ok(())
}

/// Resolves `path` the way Go's `filepath.Abs` does: an already-absolute path is
/// lexically cleaned; a relative path is joined under the process CWD and
/// cleaned. Symlinks are NOT resolved (unlike `canonicalize`), matching Go.
pub(crate) fn go_abs(path: &str) -> Result<PathBuf, Error> {
    let p = Path::new(path);
    let joined = if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir()?.join(p)
    };
    Ok(clean(&joined))
}

/// Lexical `filepath.Clean` equivalent: collapses `.`, resolves `..` against
/// preceding components, removes redundant separators, without touching disk.
fn clean(path: &Path) -> PathBuf {
    use std::ffi::OsString;

    let is_absolute = path.is_absolute();
    let mut stack: Vec<OsString> = Vec::new();

    for comp in path.components() {
        match comp {
            Component::Prefix(p) => stack.push(p.as_os_str().to_os_string()),
            Component::RootDir | Component::CurDir => {}
            Component::ParentDir => match stack.last() {
                Some(last) if last != ".." => {
                    stack.pop();
                }
                _ => {
                    if !is_absolute {
                        stack.push("..".into());
                    }
                }
            },
            Component::Normal(c) => stack.push(c.to_os_string()),
        }
    }

    let mut out = PathBuf::new();
    if is_absolute {
        out.push(std::path::MAIN_SEPARATOR_STR);
    }
    for c in stack {
        out.push(c);
    }
    if out.as_os_str().is_empty() {
        out.push(".");
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn go_abs_absolute_is_cleaned() {
        let p = go_abs("/a/b/../c").unwrap();
        assert_eq!(p, Path::new("/a/c"));
    }

    #[test]
    fn go_abs_relative_joins_cwd() {
        let cwd = std::env::current_dir().unwrap();
        let p = go_abs("some/dir").unwrap();
        assert_eq!(p, cwd.join("some/dir"));
    }
}
