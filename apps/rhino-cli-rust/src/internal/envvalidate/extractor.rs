//! Line-oriented env-read extractors, one per source language.
//!
//! All extractors use simple regex matches — no language parser dependency.
//! Known false-positive/negative modes are documented in tech-docs.md §6.1.

use std::collections::BTreeSet;

/// Extract env var names read by Rust source (`env::var("KEY")` / `std::env::var("KEY")`).
pub fn extract_rust(content: &str) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("//") {
            continue;
        }
        let mut rest = line;
        while let Some(pos) = rest.find("env::var(\"") {
            rest = &rest[pos + 10..];
            if let Some(end) = rest.find('"') {
                let key = &rest[..end];
                if is_env_key(key) {
                    keys.insert(key.to_string());
                }
                rest = &rest[end..];
            } else {
                break;
            }
        }
    }
    keys
}

/// Extract env var names read by Go source:
/// - `os.Getenv("KEY")` / `os.LookupEnv("KEY")`
/// - struct tags `` `env:"KEY"` `` / `` `env:"KEY,required"` ``
pub fn extract_go(content: &str) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("//") {
            continue;
        }
        // os.Getenv / os.LookupEnv
        for pattern in &[r"os.Getenv(", r"os.LookupEnv("] {
            let mut rest = line;
            while let Some(pos) = rest.find(pattern) {
                let after = &rest[pos + pattern.len()..];
                if let Some(after) = after.strip_prefix('"')
                    && let Some(end) = after.find('"')
                {
                    let key = &after[..end];
                    if is_env_key(key) {
                        keys.insert(key.to_string());
                    }
                }
                rest = &rest[pos + pattern.len()..];
            }
        }
        // struct tags: `env:"KEY"` or `env:"KEY,..."``
        let mut rest = line;
        while let Some(pos) = rest.find(r#"`env:""#) {
            let after = &rest[pos + 6..];
            if let Some(end) = after.find('"') {
                let tag_val = &after[..end];
                // Strip options like ",required"
                let key = tag_val.split(',').next().unwrap_or(tag_val);
                if is_env_key(key) {
                    keys.insert(key.to_string());
                }
            }
            rest = &rest[pos + 6..];
        }
    }
    keys
}

/// Extract env var names read by TypeScript/JavaScript source:
/// - `process.env.KEY`
/// - `process.env["KEY"]`
/// - `Config.string("KEY")` / `Config.integer("KEY")` / `Config.number("KEY")` / `Config.boolean("KEY")`
pub fn extract_typescript(content: &str) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("//") {
            continue;
        }
        // process.env.KEY
        let mut rest = line;
        while let Some(pos) = rest.find("process.env.") {
            let after = &rest[pos + 12..];
            let key: String = after
                .chars()
                .take_while(|c| c.is_ascii_uppercase() || *c == '_' || c.is_ascii_digit())
                .collect();
            if is_env_key(&key) {
                keys.insert(key);
            }
            rest = &rest[pos + 12..];
        }
        // process.env["KEY"]
        let mut rest = line;
        while let Some(pos) = rest.find("process.env[\"") {
            let after = &rest[pos + 13..];
            if let Some(end) = after.find('"') {
                let key = &after[..end];
                if is_env_key(key) {
                    keys.insert(key.to_string());
                }
            }
            rest = &rest[pos + 13..];
        }
        // Config.string("KEY") / Config.integer("KEY") etc.
        for variant in &[
            "Config.string(\"",
            "Config.integer(\"",
            "Config.number(\"",
            "Config.boolean(\"",
        ] {
            let mut rest = line;
            while let Some(pos) = rest.find(variant) {
                let after = &rest[pos + variant.len()..];
                if let Some(end) = after.find('"') {
                    let key = &after[..end];
                    if is_env_key(key) {
                        keys.insert(key.to_string());
                    }
                }
                rest = &rest[pos + variant.len()..];
            }
        }
    }
    keys
}

/// Extract env var names read by Clojure source.
///
/// Matches both `(System/getenv "KEY")` (Java interop) and `(getenv "KEY")`
/// (local wrapper — the pedestal app defines `getenv` as a thin `System/getenv` shim).
pub fn extract_clojure(content: &str) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with(';') {
            continue;
        }
        // Match `getenv "KEY"` — covers both `(System/getenv "KEY")` and `(getenv "KEY")`
        let mut rest = line;
        while let Some(pos) = rest.find("getenv \"") {
            let after = &rest[pos + 8..];
            if let Some(end) = after.find('"') {
                let key = &after[..end];
                if is_env_key(key) {
                    keys.insert(key.to_string());
                }
            }
            rest = &rest[pos + 8..];
        }
    }
    keys
}

/// Extract env var names read by C# source:
/// - `Configuration["KEY"]`
/// - `Environment.GetEnvironmentVariable("KEY")`
pub fn extract_csharp(content: &str) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("//") {
            continue;
        }
        // Configuration["KEY"]
        let mut rest = line;
        while let Some(pos) = rest.find("Configuration[\"") {
            let after = &rest[pos + 15..];
            if let Some(end) = after.find('"') {
                let key = &after[..end];
                if is_env_key(key) {
                    keys.insert(key.to_string());
                }
            }
            rest = &rest[pos + 15..];
        }
        // GetEnvironmentVariable("KEY")
        let mut rest = line;
        while let Some(pos) = rest.find("GetEnvironmentVariable(\"") {
            let after = &rest[pos + 24..];
            if let Some(end) = after.find('"') {
                let key = &after[..end];
                if is_env_key(key) {
                    keys.insert(key.to_string());
                }
            }
            rest = &rest[pos + 24..];
        }
    }
    keys
}

/// Extract env var names read by Elixir/Elixir config source: `System.get_env("KEY")`.
pub fn extract_elixir(content: &str) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') {
            continue;
        }
        let mut rest = line;
        while let Some(pos) = rest.find("System.get_env(\"") {
            let after = &rest[pos + 16..];
            if let Some(end) = after.find('"') {
                let key = &after[..end];
                if is_env_key(key) {
                    keys.insert(key.to_string());
                }
            }
            rest = &rest[pos + 16..];
        }
    }
    keys
}

/// Extract env var names read by F# source: `Environment.GetEnvironmentVariable("KEY")`.
pub fn extract_fsharp(content: &str) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("//") {
            continue;
        }
        let mut rest = line;
        while let Some(pos) = rest.find("GetEnvironmentVariable(\"") {
            let after = &rest[pos + 24..];
            if let Some(end) = after.find('"') {
                let key = &after[..end];
                if is_env_key(key) {
                    keys.insert(key.to_string());
                }
            }
            rest = &rest[pos + 24..];
        }
    }
    keys
}

/// Extract env var names from Java source (`System.getenv("KEY")`) and Spring
/// YAML (`${KEY}` without default — i.e. `${KEY}` not `${KEY:default}`).
pub fn extract_java(content: &str, is_yaml: bool) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    if is_yaml {
        // YAML: ${KEY} (no colon = required; ${KEY:default} = optional, skip)
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('#') {
                continue;
            }
            let mut rest = line;
            while let Some(pos) = rest.find("${") {
                let after = &rest[pos + 2..];
                // Find end: '}' but stop if we see ':' first (that's a default)
                let mut end = None;
                let mut has_default = false;
                for (i, c) in after.char_indices() {
                    if c == ':' {
                        has_default = true;
                        break;
                    }
                    if c == '}' {
                        end = Some(i);
                        break;
                    }
                }
                if !has_default && let Some(end) = end {
                    let key = &after[..end];
                    if is_env_key(key) {
                        keys.insert(key.to_string());
                    }
                }
                rest = &rest[pos + 2..];
            }
        }
    } else {
        // Java: System.getenv("KEY")
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("//") {
                continue;
            }
            let mut rest = line;
            while let Some(pos) = rest.find("System.getenv(\"") {
                let after = &rest[pos + 15..];
                if let Some(end) = after.find('"') {
                    let key = &after[..end];
                    if is_env_key(key) {
                        keys.insert(key.to_string());
                    }
                }
                rest = &rest[pos + 15..];
            }
        }
    }
    keys
}

/// Extract env var names read by Kotlin source: `System.getenv("KEY")`.
pub fn extract_kotlin(content: &str) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("//") {
            continue;
        }
        let mut rest = line;
        while let Some(pos) = rest.find("System.getenv(\"") {
            let after = &rest[pos + 15..];
            if let Some(end) = after.find('"') {
                let key = &after[..end];
                if is_env_key(key) {
                    keys.insert(key.to_string());
                }
            }
            rest = &rest[pos + 15..];
        }
    }
    keys
}

/// Extract env var names from Python pydantic-settings source.
///
/// Scans for field declarations inside `class Settings(BaseSettings)`:
/// - Required fields: `fieldname: type` (no `= default`)
/// - Optional fields: `fieldname: type = default` (also extracted; optional ones go in allowlist)
///
/// Field names are upper-cased to match env var convention.
pub fn extract_python(content: &str) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    let mut in_settings_class = false;
    let mut class_indent: Option<usize> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        // Detect `class Settings(BaseSettings):` (or subclass)
        if trimmed.starts_with("class ") && trimmed.contains("BaseSettings") {
            in_settings_class = true;
            class_indent = None;
            continue;
        }

        if !in_settings_class {
            // Also extract os.environ.get and os.getenv
            for pat in &["os.environ.get(\"", "os.environ[\"", "os.getenv(\""] {
                let mut rest = trimmed;
                while let Some(pos) = rest.find(pat) {
                    let after = &rest[pos + pat.len()..];
                    if let Some(end) = after.find('"') {
                        let key = &after[..end];
                        if is_env_key(key) {
                            keys.insert(key.to_string());
                        }
                    }
                    rest = &rest[pos + pat.len()..];
                }
            }
            continue;
        }

        // Inside class body
        let indent = line.len() - line.trim_start().len();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Detect end of class (line dedented past class body indent)
        if let Some(ci) = class_indent
            && indent < ci
        {
            in_settings_class = false;
            continue;
        }

        // Set class body indent on first non-empty line
        if class_indent.is_none() && indent > 0 {
            class_indent = Some(indent);
        }

        // Skip model_config lines
        if trimmed.starts_with("model_config") {
            continue;
        }

        // Match field: `identifier: type` (with or without `= default`)
        // Identifier must be a valid Python name starting with letter/underscore.
        if let Some(colon_pos) = trimmed.find(':') {
            let name_part = &trimmed[..colon_pos];
            // Must be a pure identifier (no spaces, starts with alpha/underscore)
            let name = name_part.trim();
            if !name.is_empty()
                && name
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_alphabetic() || c == '_')
                && name.chars().all(|c| c.is_alphanumeric() || c == '_')
            {
                // Convert snake_case to UPPER_CASE for env var name
                let key = name.to_uppercase();
                if is_env_key(&key) {
                    keys.insert(key);
                }
            }
        }
    }
    keys
}

/// Returns true for a valid ALL_CAPS env var name (at least 2 chars, starts with letter).
fn is_env_key(s: &str) -> bool {
    if s.len() < 2 {
        return false;
    }
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_uppercase() {
        return false;
    }
    chars.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn set(keys: &[&str]) -> BTreeSet<String> {
        keys.iter().map(std::string::ToString::to_string).collect()
    }

    #[test]
    fn rust_extracts_env_var_calls() {
        let src = r#"
            let secret = env::var("FIXTURE_JWT_SECRET").context("missing")?;
            let port = env::var("FIXTURE_PORT").unwrap_or("8080".into());
            // let comment = env::var("COMMENTED_OUT");
        "#;
        let got = extract_rust(src);
        assert_eq!(got, set(&["FIXTURE_JWT_SECRET", "FIXTURE_PORT"]));
    }

    #[test]
    fn rust_empty_on_no_matches() {
        let got = extract_rust("fn main() { println!(\"hello\"); }");
        assert!(got.is_empty());
    }

    #[test]
    fn go_extracts_getenv_and_struct_tags() {
        let src = r#"
            os.Getenv("FIXTURE_JWT_SECRET")
            JWTSecret string `env:"CRUD_BE_GOLANG_GIN_JWT_SECRET,required"`
            Port      string `env:"CRUD_BE_GOLANG_GIN_PORT" envDefault:"8201"`
        "#;
        let got = extract_go(src);
        assert!(got.contains("FIXTURE_JWT_SECRET"));
        assert!(got.contains("CRUD_BE_GOLANG_GIN_JWT_SECRET"));
        assert!(got.contains("CRUD_BE_GOLANG_GIN_PORT"));
    }

    #[test]
    fn typescript_extracts_process_env_and_config() {
        let src = r#"
            const url = process.env.BACKEND_URL;
            const secret = process.env["CRUD_FS_TS_NEXTJS_JWT_SECRET"];
            const jwt = yield* Config.string("CRUD_BE_TS_EFFECT_JWT_SECRET");
            const port = yield* Config.integer("CRUD_BE_TS_EFFECT_PORT");
        "#;
        let got = extract_typescript(src);
        assert!(got.contains("BACKEND_URL"));
        assert!(got.contains("CRUD_FS_TS_NEXTJS_JWT_SECRET"));
        assert!(got.contains("CRUD_BE_TS_EFFECT_JWT_SECRET"));
        assert!(got.contains("CRUD_BE_TS_EFFECT_PORT"));
    }

    #[test]
    fn clojure_extracts_system_getenv() {
        let src = r#"
            (getenv "CRUD_BE_CLOJURE_PEDESTAL_JWT_SECRET")
            ; (System/getenv "COMMENTED")
        "#;
        let got = extract_clojure(src);
        assert!(got.contains("CRUD_BE_CLOJURE_PEDESTAL_JWT_SECRET"));
        assert!(!got.contains("COMMENTED"));
    }

    #[test]
    fn csharp_extracts_configuration_and_getenv() {
        let src = r#"
            builder.Configuration["CRUD_BE_CSHARP_ASPNETCORE_JWT_SECRET"]
            Environment.GetEnvironmentVariable("DATABASE_URL")
        "#;
        let got = extract_csharp(src);
        assert!(got.contains("CRUD_BE_CSHARP_ASPNETCORE_JWT_SECRET"));
        assert!(got.contains("DATABASE_URL"));
    }

    #[test]
    fn elixir_extracts_system_get_env() {
        let src = r#"
            System.get_env("CRUD_BE_ELIXIR_PHOENIX_JWT_SECRET")
            # System.get_env("COMMENTED")
        "#;
        let got = extract_elixir(src);
        assert!(got.contains("CRUD_BE_ELIXIR_PHOENIX_JWT_SECRET"));
        assert!(!got.contains("COMMENTED"));
    }

    #[test]
    fn fsharp_extracts_get_environment_variable() {
        let src = r#"
            Environment.GetEnvironmentVariable("CRUD_BE_FSHARP_GIRAFFE_JWT_SECRET")
        "#;
        let got = extract_fsharp(src);
        assert!(got.contains("CRUD_BE_FSHARP_GIRAFFE_JWT_SECRET"));
    }

    #[test]
    fn java_yaml_extracts_required_placeholder() {
        let src = r"
            secret: ${CRUD_BE_JAVA_SPRINGBOOT_JWT_SECRET}
            enabled: ${ENABLE_TEST_API:false}
        ";
        let got = extract_java(src, true);
        assert!(got.contains("CRUD_BE_JAVA_SPRINGBOOT_JWT_SECRET"));
        // ${ENABLE_TEST_API:false} has a default — should be omitted
        assert!(!got.contains("ENABLE_TEST_API"));
    }

    #[test]
    fn java_source_extracts_system_getenv() {
        let src = r#"
            String secret = System.getenv("CRUD_BE_JAVA_VERTX_JWT_SECRET");
        "#;
        let got = extract_java(src, false);
        assert!(got.contains("CRUD_BE_JAVA_VERTX_JWT_SECRET"));
    }

    #[test]
    fn kotlin_extracts_system_getenv() {
        let src = r#"
            val secret = System.getenv("CRUD_BE_KOTLIN_KTOR_JWT_SECRET")
        "#;
        let got = extract_kotlin(src);
        assert!(got.contains("CRUD_BE_KOTLIN_KTOR_JWT_SECRET"));
    }

    #[test]
    fn python_extracts_pydantic_fields_and_os_getenv() {
        let src = r#"
class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_file=".env")
    database_url: str = "sqlite:///:memory:"
    crud_be_python_fastapi_jwt_secret: str
    app_jwt_issuer: str = "crud-be"

settings = Settings()
"#;
        let got = extract_python(src);
        assert!(got.contains("DATABASE_URL"));
        assert!(got.contains("CRUD_BE_PYTHON_FASTAPI_JWT_SECRET"));
        assert!(got.contains("APP_JWT_ISSUER"));
    }

    #[test]
    fn python_extracts_os_getenv_outside_class() {
        let src = r#"
import os
x = os.getenv("ENABLE_TEST_API")
y = os.environ.get("DATABASE_URL", "")
"#;
        let got = extract_python(src);
        assert!(got.contains("ENABLE_TEST_API"));
        assert!(got.contains("DATABASE_URL"));
    }

    #[test]
    fn is_env_key_filters_short_or_lowercase() {
        assert!(is_env_key("FOO"));
        assert!(is_env_key("CRUD_BE_RUST_AXUM_JWT_SECRET"));
        assert!(!is_env_key("foo"));
        assert!(!is_env_key("F")); // too short
        assert!(!is_env_key(""));
    }
}
