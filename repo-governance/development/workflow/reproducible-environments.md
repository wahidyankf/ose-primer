# Reproducible Environments

Practices for creating consistent, reproducible development and build environments:
runtime version management (Volta), dependency locking (`package-lock.json`, `npm ci`),
environment configuration (`.env.example` as the committed template with obviously-dev
placeholders; real values in gitignored `.env`), and container definitions (Docker/Compose).

The secrets and environment-variable rules from this document have moved to the hub convention:

**Full detail**: [Secrets and Environment Variable Standards](../../conventions/security/secrets-and-env-standards.md)
