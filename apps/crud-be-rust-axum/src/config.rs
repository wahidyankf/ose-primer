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
        let jwt_secret = env::var("CRUD_BE_RUST_AXUM_JWT_SECRET")
            .context("CRUD_BE_RUST_AXUM_JWT_SECRET is required")?;
        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        let port = env::var("CRUD_BE_RUST_AXUM_PORT")
            .ok()
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
    use super::*;
    use std::env;
    use std::sync::Mutex;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_jwt_secret_read_from_crud_be_rust_axum_jwt_secret() {
        let _guard = ENV_MUTEX.lock().unwrap();
        let key = "CRUD_BE_RUST_AXUM_JWT_SECRET";
        let value = "test-secret-that-is-at-least-32-chars-long!!";
        env::set_var(key, value);
        let config = Config::from_env().expect("from_env must succeed when secret is set");
        env::remove_var(key);
        assert_eq!(config.jwt_secret, value);
    }

    #[test]
    fn test_port_read_from_crud_be_rust_axum_port() {
        let _guard = ENV_MUTEX.lock().unwrap();
        let secret_key = "CRUD_BE_RUST_AXUM_JWT_SECRET";
        let port_key = "CRUD_BE_RUST_AXUM_PORT";
        env::set_var(secret_key, "test-secret-at-least-32-chars-long!!");
        env::set_var(port_key, "9999");
        let config = Config::from_env().expect("from_env must succeed when both vars are set");
        env::remove_var(port_key);
        env::remove_var(secret_key);
        assert_eq!(config.port, 9999);
    }

    #[test]
    fn test_missing_jwt_secret_is_error() {
        let _guard = ENV_MUTEX.lock().unwrap();
        env::remove_var("CRUD_BE_RUST_AXUM_JWT_SECRET");
        let result = Config::from_env();
        assert!(
            result.is_err(),
            "from_env() must fail when CRUD_BE_RUST_AXUM_JWT_SECRET is absent"
        );
        let err_str = result.unwrap_err().to_string();
        assert!(
            err_str.contains("CRUD_BE_RUST_AXUM_JWT_SECRET"),
            "error must name the missing var, got: {err_str}"
        );
    }
}
