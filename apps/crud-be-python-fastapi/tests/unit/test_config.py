"""Unit tests for application configuration settings."""

import pytest

from crud_be_python_fastapi.config import Settings


@pytest.mark.unit
class TestSettings:
    """Tests for Settings loaded from environment variables."""

    def test_crud_be_python_fastapi_jwt_secret_reads_from_env(
        self, monkeypatch: pytest.MonkeyPatch
    ) -> None:
        """Settings reads CRUD_BE_PYTHON_FASTAPI_JWT_SECRET from env."""
        monkeypatch.setenv(
            "CRUD_BE_PYTHON_FASTAPI_JWT_SECRET",
            "test-secret-value-at-least-32-chars-long",
        )
        s = Settings()
        assert s.crud_be_python_fastapi_jwt_secret == "test-secret-value-at-least-32-chars-long"

    def test_crud_be_python_fastapi_jwt_secret_has_default(self) -> None:
        """Settings has a safe default for the JWT secret when env var is absent."""
        s = Settings()
        assert (
            s.crud_be_python_fastapi_jwt_secret == "dev-jwt-secret-at-least-32-chars-long-for-dev"
        )
