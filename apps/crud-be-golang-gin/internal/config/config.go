// Package config loads and provides application configuration from environment variables.
package config

import env "github.com/caarlos0/env/v11"

// Config holds the application configuration loaded from environment variables.
type Config struct {
	Port          string `env:"CRUD_BE_GOLANG_GIN_PORT" envDefault:"8201"`
	JWTSecret     string `env:"CRUD_BE_GOLANG_GIN_JWT_SECRET,required"`
	DatabaseURL   string `env:"DATABASE_URL"`
	EnableTestAPI bool   `env:"ENABLE_TEST_API"`
}

// Load reads configuration from environment variables, returning an error if required vars are absent.
func Load() (*Config, error) {
	cfg := &Config{}
	if err := env.Parse(cfg); err != nil {
		return nil, err
	}
	return cfg, nil
}
