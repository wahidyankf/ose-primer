"""JWT token generation and validation."""

from datetime import UTC, datetime, timedelta
from typing import Any
from uuid import uuid4

import jwt

from crud_be_python_fastapi.config import settings
from crud_be_python_fastapi.domain.errors import UnauthorizedError

# PyJWT's encode/decode signatures reference key-type unions (AllowedPrivateKeyTypes /
# AllowedPublicKeys) that pull in the optional `cryptography` backend. Without those stubs
# installed, basedpyright resolves the key parameter to Unknown and flags the call as a
# partially-unknown member. Our usage only ever passes the HS256 shared secret (a str), so
# the gap is purely in the third-party signature. The narrow ignores below are scoped to
# reportUnknownMemberType on the jwt.encode/jwt.decode call sites.


def create_access_token(user_id: str, username: str, role: str) -> str:
    """Create a signed JWT access token."""
    now = datetime.now(UTC)
    payload: dict[str, Any] = {
        "sub": user_id,
        "username": username,
        "role": role,
        "iss": settings.app_jwt_issuer,
        "iat": now,
        "exp": now + timedelta(minutes=15),
        "jti": str(uuid4()),
        "type": "access",
    }
    return jwt.encode(  # pyright: ignore[reportUnknownMemberType]
        payload, settings.crud_be_python_fastapi_jwt_secret, algorithm="HS256"
    )


def create_refresh_token(user_id: str) -> str:
    """Create a signed JWT refresh token."""
    now = datetime.now(UTC)
    payload: dict[str, Any] = {
        "sub": user_id,
        "iss": settings.app_jwt_issuer,
        "iat": now,
        "exp": now + timedelta(days=7),
        "jti": str(uuid4()),
        "type": "refresh",
    }
    return jwt.encode(  # pyright: ignore[reportUnknownMemberType]
        payload, settings.crud_be_python_fastapi_jwt_secret, algorithm="HS256"
    )


def create_expired_refresh_token(user_id: str) -> str:
    """Create an already-expired refresh token (for testing only)."""
    now = datetime.now(UTC)
    payload: dict[str, Any] = {
        "sub": user_id,
        "iss": settings.app_jwt_issuer,
        "iat": now - timedelta(days=8),
        "exp": now - timedelta(days=1),
        "jti": str(uuid4()),
        "type": "refresh",
    }
    return jwt.encode(  # pyright: ignore[reportUnknownMemberType]
        payload, settings.crud_be_python_fastapi_jwt_secret, algorithm="HS256"
    )


def decode_token(token: str) -> dict[str, Any]:
    """Decode and verify a JWT token.

    Raises UnauthorizedError on invalid or expired tokens.
    """
    try:
        return jwt.decode(  # pyright: ignore[reportUnknownMemberType]
            token, settings.crud_be_python_fastapi_jwt_secret, algorithms=["HS256"]
        )
    except jwt.ExpiredSignatureError as err:
        raise UnauthorizedError("Token has expired") from err
    except jwt.InvalidTokenError as err:
        raise UnauthorizedError("Invalid token") from err


def decode_token_unverified(token: str) -> dict[str, Any]:
    """Decode a JWT token without verifying expiry (used for logout)."""
    try:
        return jwt.decode(  # pyright: ignore[reportUnknownMemberType]
            token,
            settings.crud_be_python_fastapi_jwt_secret,
            algorithms=["HS256"],
            options={"verify_exp": False},
        )
    except jwt.InvalidTokenError as err:
        raise UnauthorizedError("Invalid token") from err


def get_jwks() -> dict[str, Any]:
    """Return JWKS endpoint response (symmetric key info for HS256)."""
    return {
        "keys": [
            {
                "kty": "oct",
                "alg": "HS256",
                "use": "sig",
                "kid": "crud-be-python-fastapi-key-1",
            }
        ]
    }
