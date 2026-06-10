//! Core env-validate logic: parse .env.example, scan source, compute drift.

use std::collections::BTreeSet;
use std::path::Path;

use anyhow::{Context, Error};

use super::contract::{AppSurface, GLOBAL_ALLOWLIST};
use super::extractor::{
    extract_clojure, extract_csharp, extract_elixir, extract_fsharp, extract_go, extract_java,
    extract_kotlin, extract_python, extract_rust, extract_typescript,
};
use super::types::SurfaceResult;

/// Parse declared env var keys from `.env.example` content.
/// Skips blank lines, comment lines (`# …`), and fully-commented-out vars (`# KEY=…`).
pub fn parse_declared(content: &str) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim();
            if !key.is_empty() {
                keys.insert(key.to_string());
            }
        }
    }
    keys
}

/// Extract env var names read by source files under `source_root`, respecting `surface.source_exts`.
pub fn extract_read_keys(
    source_root: &Path,
    surface: &AppSurface,
) -> Result<BTreeSet<String>, Error> {
    let scan_root = if surface.source_subdir.is_empty() {
        source_root.to_path_buf()
    } else {
        source_root.join(surface.source_subdir)
    };

    let mut keys = BTreeSet::new();

    if !scan_root.exists() {
        return Ok(keys);
    }

    for entry in walkdir::WalkDir::new(&scan_root).sort_by_file_name() {
        let entry = entry.with_context(|| format!("walk error in {}", scan_root.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !surface.source_exts.contains(&ext) {
            continue;
        }
        // Skip generated and test directories
        let path_str = path.to_string_lossy();
        if path_str.contains("/node_modules/")
            || path_str.contains("/dist/")
            || path_str.contains("/.next/")
            || path_str.contains("/build/")
            || path_str.contains("/bin/")
            || path_str.contains("/obj/")
            || path_str.contains("/_build/")
            || path_str.contains("/generated-contracts/")
            || path_str.contains("generated-contracts")
            || path_str.contains("/test/")
            || path_str.contains("/tests/")
            || path_str.contains("/deps/")
            || path_str.contains("_test.go")
            || path_str.contains(".integration_test.")
        {
            continue;
        }

        let Ok(content) = std::fs::read_to_string(path) else {
            continue; // skip binary or unreadable files
        };

        let is_yaml = ext == "yml" || ext == "yaml";
        let extracted = match ext {
            "rs" => extract_rust(&content),
            "go" => extract_go(&content),
            "ts" | "tsx" => extract_typescript(&content),
            "clj" | "cljs" => extract_clojure(&content),
            "cs" => extract_csharp(&content),
            "ex" | "exs" => extract_elixir(&content),
            "fs" | "fsx" => extract_fsharp(&content),
            "java" => extract_java(&content, false),
            "yml" | "yaml" => extract_java(&content, is_yaml),
            "kt" => extract_kotlin(&content),
            "py" => extract_python(&content),
            _ => BTreeSet::new(),
        };
        keys.extend(extracted);
    }

    Ok(keys)
}

/// Validate a single app surface. Returns a SurfaceResult with violations filtered through
/// the global + per-app allowlist.
pub fn validate_surface(repo_root: &Path, surface: &AppSurface) -> Result<SurfaceResult, Error> {
    let env_example = repo_root
        .join("infra")
        .join("dev")
        .join(surface.app)
        .join(".env.example");

    let declared = if env_example.exists() {
        let content = std::fs::read_to_string(&env_example)
            .with_context(|| format!("read {}", env_example.display()))?;
        parse_declared(&content)
    } else {
        BTreeSet::new()
    };

    let source_root = repo_root.join("apps").join(surface.app);
    let read = extract_read_keys(&source_root, surface)?;

    // Build effective allowlist: global + per-app
    let allowlist: BTreeSet<&str> = GLOBAL_ALLOWLIST
        .iter()
        .chain(surface.allowlist.iter())
        .copied()
        .collect();

    let declared_not_read: BTreeSet<String> = declared
        .iter()
        .filter(|k| !read.contains(*k) && !allowlist.contains(k.as_str()))
        .cloned()
        .collect();

    let read_not_declared: BTreeSet<String> = read
        .iter()
        .filter(|k| !declared.contains(*k) && !allowlist.contains(k.as_str()))
        .cloned()
        .collect();

    Ok(SurfaceResult {
        app: surface.app.to_string(),
        declared_not_read,
        read_not_declared,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn write(root: &Path, rel: &str, content: &str) {
        let p = root.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, content).unwrap();
    }

    fn fixture_surface(
        app: &'static str,
        exts: &'static [&'static str],
        allowlist: &'static [&'static str],
    ) -> AppSurface {
        AppSurface {
            app,
            source_exts: exts,
            source_subdir: "src",
            allowlist,
        }
    }

    // Task #71: RED tests — these fail before GREEN is implemented.
    // They test the core declared-but-unread and read-but-undeclared logic.

    #[test]
    fn declared_but_unread_key_produces_violation() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        // .env.example declares FIXTURE_JWT_SECRET
        write(
            root,
            "infra/dev/fixture-app/.env.example",
            "FIXTURE_JWT_SECRET=change-me\n",
        );
        // Source does NOT read FIXTURE_JWT_SECRET
        write(
            root,
            "apps/fixture-app/src/config.rs",
            "fn main() { println!(\"no env reads here\"); }\n",
        );

        let surface = fixture_surface("fixture-app", &["rs"], &[]);
        let result = validate_surface(root, &surface).unwrap();

        assert!(
            result.declared_not_read.contains("FIXTURE_JWT_SECRET"),
            "expected FIXTURE_JWT_SECRET in declared_not_read, got: {:?}",
            result.declared_not_read
        );
        assert!(result.read_not_declared.is_empty());
        assert!(!result.is_ok());
    }

    #[test]
    fn read_but_undeclared_key_produces_violation() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        // .env.example is empty
        write(root, "infra/dev/fixture-app/.env.example", "\n");
        // Source reads FIXTURE_JWT_SECRET
        write(
            root,
            "apps/fixture-app/src/config.rs",
            r#"let s = env::var("FIXTURE_JWT_SECRET").unwrap();"#,
        );

        let surface = fixture_surface("fixture-app", &["rs"], &[]);
        let result = validate_surface(root, &surface).unwrap();

        assert!(
            result.read_not_declared.contains("FIXTURE_JWT_SECRET"),
            "expected FIXTURE_JWT_SECRET in read_not_declared, got: {:?}",
            result.read_not_declared
        );
        assert!(result.declared_not_read.is_empty());
        assert!(!result.is_ok());
    }

    #[test]
    fn matching_declared_and_read_exits_ok() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(
            root,
            "infra/dev/fixture-app/.env.example",
            "FIXTURE_JWT_SECRET=change-me\n",
        );
        write(
            root,
            "apps/fixture-app/src/config.rs",
            r#"let s = env::var("FIXTURE_JWT_SECRET").context("required")?;"#,
        );

        let surface = fixture_surface("fixture-app", &["rs"], &[]);
        let result = validate_surface(root, &surface).unwrap();
        assert!(result.is_ok(), "expected no violations, got: {result:?}");
    }

    #[test]
    fn allowlisted_keys_are_ignored() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        // .env.example does NOT declare ENABLE_TEST_API or FIXTURE_PORT
        write(
            root,
            "infra/dev/fixture-app/.env.example",
            "FIXTURE_JWT_SECRET=change-me\n",
        );
        // Source reads FIXTURE_JWT_SECRET, ENABLE_TEST_API (global allowlist), and FIXTURE_PORT (per-app)
        write(
            root,
            "apps/fixture-app/src/config.rs",
            concat!(
                r#"let s = env::var("FIXTURE_JWT_SECRET").context("required")?;"#,
                "\n",
                r#"let t = env::var("ENABLE_TEST_API").unwrap_or_default();"#,
                "\n",
                r#"let p = env::var("FIXTURE_PORT").unwrap_or("8080".into());"#,
            ),
        );

        let surface = AppSurface {
            app: "fixture-app",
            source_exts: &["rs"],
            source_subdir: "src",
            allowlist: &["FIXTURE_PORT"],
        };
        let result = validate_surface(root, &surface).unwrap();
        assert!(
            result.is_ok(),
            "expected no violations (ENABLE_TEST_API and FIXTURE_PORT allowlisted), got: {result:?}"
        );
    }

    #[test]
    fn parse_declared_skips_comments_and_blanks() {
        let content = "# comment\nFOO=bar\n\n# BAR=baz\nBAZ=qux\n";
        let keys = parse_declared(content);
        assert!(keys.contains("FOO"));
        assert!(keys.contains("BAZ"));
        assert!(!keys.contains("BAR"));
        assert_eq!(keys.len(), 2);
    }
}
