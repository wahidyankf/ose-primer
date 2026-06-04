"""Password hashing using bcrypt directly."""

import bcrypt

# bcrypt only uses the first 72 bytes of a password. bcrypt 4.x silently
# truncated longer inputs, but bcrypt 5.0.0 raises ValueError instead. We
# truncate explicitly to preserve the prior behavior and keep hashes stable.
_BCRYPT_MAX_BYTES = 72


def _to_bcrypt_bytes(plain: str) -> bytes:
    """Encode a password to UTF-8, truncated to bcrypt's 72-byte limit."""
    return plain.encode("utf-8")[:_BCRYPT_MAX_BYTES]


def hash_password(plain: str) -> str:
    """Hash a plain-text password using bcrypt."""
    password_bytes = _to_bcrypt_bytes(plain)
    salt = bcrypt.gensalt()
    hashed = bcrypt.hashpw(password_bytes, salt)
    return hashed.decode("utf-8")


def verify_password(plain: str, hashed: str) -> bool:
    """Verify a plain-text password against a hashed one."""
    password_bytes = _to_bcrypt_bytes(plain)
    hashed_bytes = hashed.encode("utf-8")
    return bcrypt.checkpw(password_bytes, hashed_bytes)
