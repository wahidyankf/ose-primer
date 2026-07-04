use anyhow::{Context, Result};
use std::env;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        Self::from_provider(|key| env::var(key).ok())
    }

    /// Builds `Config` from an injectable lookup function, so tests can supply
    /// values without mutating real process environment variables (which
    /// `std::env::set_var`/`remove_var` require `unsafe` for as of Rust 2024).
    fn from_provider(getenv: impl Fn(&str) -> Option<String>) -> Result<Self> {
        let jwt_secret = getenv("CRUD_BE_RUST_AXUM_JWT_SECRET")
            .context("CRUD_BE_RUST_AXUM_JWT_SECRET is required")?;
        let database_url = getenv("DATABASE_URL").unwrap_or_else(|| "sqlite::memory:".to_string());
        let port = getenv("CRUD_BE_RUST_AXUM_PORT")
            .and_then(|p| p.parse().ok())
            .unwrap_or(8201);
        Ok(Self {
            database_url,
            jwt_secret,
            port,
        })
    }
}

#[cfg(test)]
mod tests {
    // `unwrap`/`expect`/`panic` and exact float comparisons are idiomatic in
    // tests, where a failed assumption should fail the test loudly.
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::float_cmp
    )]

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_jwt_secret_read_from_crud_be_rust_axum_jwt_secret() {
        let value = "test-secret-that-is-at-least-32-chars-long!!";
        let env: HashMap<&str, &str> = HashMap::from([("CRUD_BE_RUST_AXUM_JWT_SECRET", value)]);
        let config = Config::from_provider(|k| env.get(k).map(std::string::ToString::to_string))
            .expect("from_provider must succeed when secret is set");
        assert_eq!(config.jwt_secret, value);
    }

    #[test]
    fn test_port_read_from_crud_be_rust_axum_port() {
        let env: HashMap<&str, &str> = HashMap::from([
            (
                "CRUD_BE_RUST_AXUM_JWT_SECRET",
                "test-secret-at-least-32-chars-long!!",
            ),
            ("CRUD_BE_RUST_AXUM_PORT", "9999"),
        ]);
        let config = Config::from_provider(|k| env.get(k).map(std::string::ToString::to_string))
            .expect("from_provider must succeed when both vars are set");
        assert_eq!(config.port, 9999);
    }

    #[test]
    fn test_missing_jwt_secret_is_error() {
        let env: HashMap<&str, &str> = HashMap::new();
        let result = Config::from_provider(|k| env.get(k).map(std::string::ToString::to_string));
        assert!(
            result.is_err(),
            "from_provider() must fail when CRUD_BE_RUST_AXUM_JWT_SECRET is absent"
        );
        let err_str = result.unwrap_err().to_string();
        assert!(
            err_str.contains("CRUD_BE_RUST_AXUM_JWT_SECRET"),
            "error must name the missing var, got: {err_str}"
        );
    }
}
