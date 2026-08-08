---
title: "Reference"
description: Technical reference for ose-primer's polyglot Nx starter and its delivery conventions
category: reference
tags:
  - index
  - reference
  - technical
---

# Reference

Technical reference for `ose-primer`: the reusable polyglot Nx starter in the Open Sharia Enterprise
family. Use the [getting-started tutorial](../tutorials/getting-started-with-ose-primer.md) first;
come here when you need exact structure, configuration, or ecosystem boundaries.

## 📋 Contents

### Repository Structure

- [Monorepo Structure](./monorepo-structure.md) - Nx monorepo organization, apps, libs, and project architecture
- [Nx Configuration](./nx-configuration.md) - Nx workspace configuration, task caching, and build system
- [Project Dependency Graph](./project-dependency-graph.md) - Complete Nx dependency graph with Mermaid diagram, dependency tables, and spec directory mapping
- [System Architecture](./system-architecture/README.md) - Comprehensive reference for platform architecture, application inventory, interactions, deployment infrastructure, and CI/CD pipelines

### Quality Infrastructure

- [Code Coverage](./code-coverage.md) - How coverage is measured locally via rhino-cli, per-project details, exclusion patterns, and troubleshooting
- [CRUD Apps CI Coverage](./crud-apps-ci-coverage.md) - CI status badges and coverage tracking for all CRUD app implementations
- [Security Waivers Register](./security-waivers.md) - Long-lived register of dependency-bump security waivers (Path C) and functional holds (Rule 5b), with CVE / CISA KEV / EPSS columns, per the [Dependency Bump Stability & Safety Policy](../../repo-governance/development/workflow/dependency-bump-policy.md)

### AI Models

- [AI Model Benchmarks](./ai-model-benchmarks.md) - Cited benchmark scores for all Claude and GLM models used in this project, with confidence levels and source URLs
- [Platform Bindings](./platform-bindings.md) - Catalog of platform-specific bindings (Claude Code, OpenCode, Cursor, Copilot, etc.) — the only place vendor names are allowed outside ` ```binding-example ` fences per the [Governance Vendor Independence convention](../../repo-governance/conventions/structure/governance-vendor-independence.md)

### Cross-Repository Context

- [Related Repositories](./related-repositories.md) - Catalogue of the four sibling repositories in the Open Sharia Enterprise family (`ose-public`, `ose-primer`, `ose-private`, `beaver-nest`) with visibility, license, purpose, and each one's relationship to `ose-primer` — plus which repositories participate in content sync

---
