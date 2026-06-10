# No Secrets in Committed Files Convention

Never put system secrets (SSH keys, passwords, API keys, tokens, connection strings with
real credentials, or similar) into any file committed to git — including plans, docs, source
code, tests, configuration, and CI workflows. Git history is permanent; a pushed secret is a
leaked secret.

**Full detail**: [Secrets and Environment Variable Standards](./secrets-and-env-standards.md)
