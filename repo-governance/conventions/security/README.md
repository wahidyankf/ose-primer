---
title: "Security Conventions"
description: Repository security conventions governing agent behavior and data protection
category: explanation
subcategory: conventions
tags:
  - index
  - security
  - conventions
created: 2026-05-24
---

# Security Conventions

Conventions governing secrets, environment variables, and agent access controls.

- [Secrets and Environment Variable Standards](./secrets-and-env-standards.md) - Hub
  convention: naming standard, `.env.example` annotation format, fail-fast validation table,
  `rhino-cli env` tooling, secret-surface census, and gated IaC scaffold
- [No Secrets in Committed Files Convention](./no-secrets-in-committed-files.md) - Hard iron
  rule prohibiting system secrets from any committed file (stub)
- [Environment File Access Convention](./env-file-access.md) - Six-layer policy governing AI
  agent access to real `.env*` files (stub)
