---
title: "Environment File Access Convention"
description: AI agents must not directly read, write, edit, or commit any .env* file except .env.example. Full policy in secrets-and-env-standards.md.
category: explanation
subcategory: conventions
tags:
  - security
  - env-files
  - agents
  - guard-env-file-access
created: 2026-05-24
---

# Environment File Access Convention

AI agents operating in this repository MUST NOT read, write, edit, or commit real `.env*`
files (`.env`, `.env.local`, `.env.production`, etc.). Only `.env.example` is permitted.
This rule is enforced across six layers: coding-agent PreToolUse hooks, declarative deny
rules, Bash command guards, secondary-binding permission blocks, gitignore + pre-commit
guard, and this governance rule.

**Full detail**: [Secrets and Environment Variable Standards](./secrets-and-env-standards.md)
