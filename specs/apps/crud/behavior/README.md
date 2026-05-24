---
title: "crud — Behavioral Specs"
description: Index of behavioral specifications for the crud app group
category: specs
---

# crud — Behavioral Specs

This directory contains the behavioral specifications (Gherkin feature files)
for the `crud` app group, organized by surface.

## Structure

```
behavior/
├── be/         # Backend behavioral specs (REST API, business logic)
│   └── gherkin/
│       └── {domain}/
└── web/        # Web frontend behavioral specs (UI flows, accessibility)
    └── gherkin/
        └── {domain}/
```

## Surfaces

- [be/](./be/) — Backend (Go Gin, Java Spring Boot, Kotlin Ktor, etc.)
- [web/](./web/) — Web frontend (Next.js, TanStack Start, Flutter Web)

## Related

- [crud spec root](../README.md)
- [Specs Directory Structure Convention](../../../../repo-governance/conventions/structure/specs-directory-structure.md)
