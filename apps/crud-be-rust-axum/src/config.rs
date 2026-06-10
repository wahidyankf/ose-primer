use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    /// Load configuration from environment variables with defaults.
    #[must_use]
    pub fn from_env() -> Self {
        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        let jwt_secret = env::var("CRUD_BE_RUST_AXUM_JWT_SECRET")
            .unwrap_or_else(|_| "dev-jwt-secret-at-least-32-chars-long".to_string());
        let port = env::var("CRUD_BE_RUST_AXUM_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8201);
        Self {
            database_url,
            jwt_secret,
            port,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_jwt_secret_read_from_crud_be_rust_axum_jwt_secret() {
        let key = "CRUD_BE_RUST_AXUM_JWT_SECRET";
        let value = "test-secret-that-is-at-least-32-chars-long!!";
        env::set_var(key, value);
        let config = Config::from_env();
        env::remove_var(key);
        assert_eq!(config.jwt_secret, value);
    }

    #[test]
    fn test_port_read_from_crud_be_rust_axum_port() {
        let key = "CRUD_BE_RUST_AXUM_PORT";
        env::set_var(key, "9999");
        let config = Config::from_env();
        env::remove_var(key);
        assert_eq!(config.port, 9999);
    }
}
