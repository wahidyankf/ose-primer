# Environment File Access Convention

AI agents operating in this repository MUST NOT read, write, edit, or commit real `.env*`
files (`.env`, `.env.local`, `.env.production`, etc.). Only `.env.example` is permitted.
This rule is enforced across six layers: coding-agent PreToolUse hooks, declarative deny
rules, Bash command guards, secondary-binding permission blocks, gitignore + pre-commit
guard, and this governance rule.

**Full detail**: [Secrets and Environment Variable Standards](./secrets-and-env-standards.md)
