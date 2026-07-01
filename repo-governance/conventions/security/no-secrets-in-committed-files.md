---
title: "No Secrets in Committed Files"
description: Hard iron rule — no system secret may enter any git-tracked file. Full standards in secrets-and-env-standards.md.
category: explanation
subcategory: conventions
tags:
  - security
  - secrets
  - git
  - data-protection
created: 2026-06-01
---

# No Secrets in Committed Files Convention

Never put system secrets (SSH keys, passwords, API keys, tokens, connection strings with
real credentials, or similar) into any file committed to git — including plans, docs, source
code, tests, configuration, and CI workflows. Git history is permanent; a pushed secret is a
leaked secret.

**Full detail**: [Secrets and Environment Variable Standards](./secrets-and-env-standards.md)
