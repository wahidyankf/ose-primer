---
title: "No Secrets in Committed Files Convention"
description: >
  Hard iron rule prohibiting system secrets (SSH keys, passwords, sensitive
  usernames, tokens, API keys, and similar) from any file committed to git in
  this repository — including plans, docs, code, and configuration.
category: explanation
subcategory: development
tags:
  - security
  - secrets
  - git
  - plans
  - docs
  - conventions
---

# No Secrets in Committed Files Convention

## Purpose

Prevent system secrets from ever entering this repository's git history. Anything
committed to git is permanent and, in a public repository, world-readable forever — a
secret pushed once is a secret leaked, even if a later commit removes it.

This convention states the rule bluntly so every human contributor and AI agent applies
it without interpretation: **secrets belong in uncommitted files, never in committed
ones.**

## The Iron Rule

> **Never put system secrets into any file committed to git in this repository.**

"System secrets" includes, but is not limited to:

- SSH keys (public or private) and other cryptographic key material
- Passwords and passphrases
- Sensitive or privileged usernames (admin accounts, service accounts, real operator identities)
- API keys, access tokens, OAuth client secrets, session tokens, bearer tokens
- Database connection strings containing real credentials
- Cloud provider credentials (AWS/GCP/Azure access keys, service-account JSON)
- Webhook secrets, signing secrets, and HMAC keys
- Any other credential, secret, or sensitive identifier that grants access to a system

This rule applies to **every committed file type**, explicitly including:

- **Plans** (`plans/**` — `brd.md`, `prd.md`, `tech-docs.md`, `delivery.md`, `README.md`, etc.)
- **Documentation** (`docs/**`, `README.md`, governance, conventions, principles)
- **Source code, tests, and test fixtures**
- **Configuration files** (Nx `project.json`, `nx.json`, CI workflow YAML, Docker, etc.)
- **Shell scripts, commit messages, and audit reports**

## Where Secrets Belong Instead

When work requires a real secret, place it in an **uncommitted** location:

- A real environment file: `.env`, `.env.local`, `.env.production`, etc.
  (every `.env*` variant **except** `.env.example`, which is committed and must hold only
  placeholders)
- Any other file listed in `.gitignore`
- A dedicated secrets manager / vault external to the repository

The committed surface references secrets only **by name**:

- Document the _variable name_ and a safe placeholder in `.env.example`
- Reference the variable in code/config (`${DATABASE_URL}`, `process.env.API_KEY`)
- In plans and docs, name the secret and where it lives ("set `OPENCODE_GO_API_KEY` in
  `.env`") — never the value itself

```bash
# .env.example — COMMITTED. Placeholders only.
DATABASE_URL=postgres://user:password@localhost:5432/mydb
API_KEY=your-api-key-here
SSH_DEPLOY_KEY=path-to-your-key
```

```bash
# .env — GITIGNORED. Real values live here, never committed.
DATABASE_URL=postgres://realuser:r3alS3cr3t@db.internal:5432/prod
API_KEY=sk-live-abc123def456
```

## Examples

### Prohibited: secret pasted into a plan

```markdown
<!-- WRONG — plans/in-progress/deploy-service/delivery.md -->

Deploy with the production token: `sk-live-9f8a7b6c5d4e3f2a1b0c`
SSH in as `root` with password `Pr0dPa55!`.
```

```markdown
<!-- CORRECT -->

Deploy with the token stored in `DEPLOY_TOKEN` (`.env`, see `.env.example`).
SSH in using the deploy key referenced by `SSH_DEPLOY_KEY` in `.env`.
```

### Prohibited: credential in committed docs

```markdown
<!-- WRONG — docs/how-to/connect-db.md -->

Connection string: `postgres://admin:SuperSecret1@10.0.0.5:5432/app`
```

```markdown
<!-- CORRECT -->

Set `DATABASE_URL` in `.env` (template in `.env.example`); the app reads it at runtime.
```

### Acceptable: synthetic placeholder in committed example

```bash
# CORRECT — clearly fake placeholder, documents the variable name only
API_KEY=your-api-key-here
```

## Relationship to Sibling Conventions

This convention is the broad, file-type-agnostic statement of the rule. Two related
conventions enforce specific facets:

- **[Environment File Access Convention](./env-file-access.md)** — the six-layer technical
  enforcement preventing agents from reading/writing/committing real `.env*` files.
- **[No Machine-Specific Information in Commits](./no-machine-specific-commits.md)** —
  prohibits absolute local paths, machine usernames, and local IPs (portability + accidental
  disclosure), of which literal credentials are the most severe case.

Where those two define _mechanisms_ and _machine-portability_, this convention states the
_universal prohibition_ and explicitly names plans and docs as in-scope.

## Verifying Before Committing

Inspect staged changes for obvious secret patterns before pushing:

```bash
git diff --cached | grep -iE "BEGIN (RSA|OPENSSH|EC|DSA|PGP) PRIVATE KEY|password\s*[:=]|secret\s*[:=]|sk-live-|AKIA[0-9A-Z]{16}|api[_-]?key\s*[:=]"
```

Any match is a signal to review that line. Move the value to `.env` (or another gitignored
file) and replace the committed reference with the variable name.

## Remediation

If a secret has already been committed:

1. Remove it from the working tree; replace with an environment-variable reference.
2. Commit the corrected version.
3. **Rotate the secret immediately** — git history is permanent, so the value is considered
   compromised even after removal from `HEAD`. Rotation is mandatory, not optional.
4. If the repository is public or shared, treat the secret as fully exposed from the moment
   it was pushed.

## Scope

Applies to every file committed to this repository — source, tests, fixtures, configuration,
CI workflows, shell scripts, commit messages, documentation, and all `plans/**` documents.

Does **not** apply to:

- `.env` and other `.env*` files except `.env.example` (gitignored; never committed)
- Any file listed in `.gitignore`

## Principles Implemented/Respected

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**:
  The prohibition is stated as a single unambiguous rule with an enumerated secret list and
  explicit in-scope file types — no room for "I assumed it was fine."

- **[Root Cause Orientation](../../principles/general/root-cause-orientation.md)**:
  Keeping secrets out of committed files at authoring time fixes the root cause, rather than
  relying on post-commit history scrubbing (which cannot un-leak a pushed secret).

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**:
  One blunt rule — secrets go in uncommitted files — is easier to apply correctly than a
  nuanced per-case judgment.

## Conventions Implemented/Respected

- **[File Naming Convention](../../conventions/structure/file-naming.md)**: This file uses
  kebab-case and lives under the correct governance layer.

- **[Content Quality Principles](../../conventions/writing/quality.md)**: Active voice, single
  H1, concrete examples over abstract description, no time estimates.

- **[Linking Convention](../../conventions/formatting/linking.md)**: All cross-references use
  relative paths with `.md` extensions.

## Related Documentation

- [Environment File Access Convention](./env-file-access.md) — technical enforcement layers for `.env*` files
- [No Machine-Specific Information in Commits](./no-machine-specific-commits.md) — paths, usernames, and IPs in commits
- [Code Quality Convention](./code.md) — git hooks and pre-commit automation
- [Plans Organization Convention](../../conventions/structure/plans.md) — plans are in-scope committed documents
