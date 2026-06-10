"""Unit tests for application configuration settings."""

import pytest
from pydantic import ValidationError

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

    def test_missing_jwt_secret_raises_validation_error(
        self, monkeypatch: pytest.MonkeyPatch
    ) -> None:
        """Settings must raise ValidationError when CRUD_BE_PYTHON_FASTAPI_JWT_SECRET is absent."""
        monkeypatch.delenv("CRUD_BE_PYTHON_FASTAPI_JWT_SECRET", raising=False)
        with pytest.raises(ValidationError):
            Settings()
