package config_test

import (
	"os"
	"strings"
	"testing"

	"github.com/wahidyankf/ose-public/apps/crud-be-golang-gin/internal/config"
)

func mustUnsetenv(t *testing.T, key string) {
	t.Helper()
	if err := os.Unsetenv(key); err != nil {
		t.Fatalf("failed to unset %s: %v", key, err)
	}
}

func mustSetenv(t *testing.T, key, val string) {
	t.Helper()
	if err := os.Setenv(key, val); err != nil {
		t.Fatalf("failed to set %s: %v", key, err)
	}
}

func TestUnitConfigLoad(t *testing.T) {
	t.Run("defaults when jwt secret provided", func(t *testing.T) {
		mustUnsetenv(t, "CRUD_BE_GOLANG_GIN_PORT")
		mustSetenv(t, "CRUD_BE_GOLANG_GIN_JWT_SECRET", "test-jwt-secret-at-least-32-chars!!")
		mustUnsetenv(t, "DATABASE_URL")
		defer mustUnsetenv(t, "CRUD_BE_GOLANG_GIN_JWT_SECRET")

		cfg, err := config.Load()
		if err != nil {
			t.Fatalf("Load() must succeed when JWT secret is set, got: %v", err)
		}
		if cfg.Port != "8201" {
			t.Errorf("expected default port 8201, got %s", cfg.Port)
		}
		if cfg.JWTSecret != "test-jwt-secret-at-least-32-chars!!" {
			t.Errorf("expected jwt secret to match env var, got %s", cfg.JWTSecret)
		}
		if cfg.DatabaseURL != "" {
			t.Errorf("expected empty DATABASE_URL, got %s", cfg.DatabaseURL)
		}
	})

	t.Run("custom values", func(t *testing.T) {
		mustSetenv(t, "CRUD_BE_GOLANG_GIN_PORT", "9000")
		mustSetenv(t, "CRUD_BE_GOLANG_GIN_JWT_SECRET", "my-secret")
		mustSetenv(t, "DATABASE_URL", "postgres://localhost/test")
		defer func() {
			mustUnsetenv(t, "CRUD_BE_GOLANG_GIN_PORT")
			mustUnsetenv(t, "CRUD_BE_GOLANG_GIN_JWT_SECRET")
			mustUnsetenv(t, "DATABASE_URL")
		}()
		cfg, err := config.Load()
		if err != nil {
			t.Fatalf("Load() must succeed, got: %v", err)
		}
		if cfg.Port != "9000" {
			t.Errorf("expected port 9000, got %s", cfg.Port)
		}
		if cfg.JWTSecret != "my-secret" {
			t.Errorf("expected jwt secret 'my-secret', got %s", cfg.JWTSecret)
		}
		if cfg.DatabaseURL != "postgres://localhost/test" {
			t.Errorf("expected database URL, got %s", cfg.DatabaseURL)
		}
	})
}

func TestUnitConfigMissingJWTSecretIsError(t *testing.T) {
	mustUnsetenv(t, "CRUD_BE_GOLANG_GIN_JWT_SECRET")
	cfg, err := config.Load()
	if err == nil {
		t.Fatal("Load() must return error when CRUD_BE_GOLANG_GIN_JWT_SECRET is absent")
	}
	if cfg != nil {
		t.Fatal("Load() must return nil config on error")
	}
	if !strings.Contains(err.Error(), "CRUD_BE_GOLANG_GIN_JWT_SECRET") &&
		!strings.Contains(err.Error(), "jwt") {
		t.Errorf("error must name the missing var, got: %v", err)
	}
}
