---
title: "GitHub Actions Workflow Naming Convention"
description: Workflow filenames must mirror their workflow name field using kebab-case derivation
category: explanation
subcategory: development
tags:
  - github-actions
  - ci-cd
  - naming
  - workflow
---

# GitHub Actions Workflow Naming Convention

GitHub Actions workflow files live in `.github/workflows/`. The filename of each workflow file must mirror its `name:` field. Developers must be able to derive the filename from the workflow name shown in the GitHub Actions UI, and vice versa.

## Principles Implemented/Respected

This convention implements/respects the following core principles:

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: The mapping between what GitHub Actions displays and what lives on disk is made explicit and deterministic. No guessing which file corresponds to a failing workflow run.

- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**: A consistent mechanical derivation rule makes it possible to validate filename/name alignment automatically, without relying on human review.

## Conventions Implemented/Respected

This practice respects the following conventions:

- **[File Naming Convention](../../conventions/structure/file-naming.md)**: Workflow filenames use kebab-case, consistent with the broader file naming rules applied across the repository.

## Purpose

GitHub shows the `name:` field in the Actions tab, in PR status checks, and in email notifications. When a workflow fails, developers look at the name in the UI then need to find and edit the corresponding `.yml` file. Without a consistent mapping rule, locating the right file requires opening files until the matching name is found.

This convention eliminates that friction by requiring the filename to be a mechanical kebab-case derivation of the `name:` field.

## Scope

### What This Convention Covers

- All workflow files under `.github/workflows/`
- The relationship between the `name:` field and the `.yml` filename

### What This Convention Does NOT Cover

- Workflow content, structure, or job naming
- Reusable workflows called via `workflow_call`
- Scheduled or manually triggered workflow naming beyond the filename/name mapping

## Standards

### Derivation Rule

Derive the filename from the `name:` field by applying these transformations in order:

1. Convert to lowercase
2. Replace spaces with hyphens
3. Remove special characters: `+`, `(`, `)`, `/`, `#`
4. Replace `-` (space-hyphen-space) with `-`
5. Collapse consecutive hyphens to a single hyphen
6. Append `.yml`

The result must exactly match the filename (without path).

### Transformation Table

| Character or pattern in `name:` | Becomes in filename |
| ------------------------------- | ------------------- |
| Space (` `)                     | `-`                 |
| `-` (spaced hyphen)             | `-`                 |
| `+`                             | removed             |
| `(`                             | removed             |
| `)`                             | removed             |
| `/`                             | removed             |
| `#`                             | removed             |
| Consecutive hyphens (`--`)      | `-`                 |

### Complete Codebase Reference

Every non-reusable workflow currently in the repository follows this rule. Reusable workflows
(filenames prefixed `_`, triggered only via `workflow_call` — see [Scope](#what-this-convention-does-not-cover))
are intentionally omitted below: they never appear standalone in the GitHub Actions UI, so
filename/`name:` derivation has no reader-facing purpose for them. The reusable workflows currently
in the repository are `_reusable-backend-coverage.yml`, `_reusable-backend-e2e.yml`,
`_reusable-backend-integration.yml`, `_reusable-backend-lint.yml`,
`_reusable-backend-spec-coverage.yml`, `_reusable-backend-typecheck.yml`, and
`_reusable-frontend-e2e.yml`.

| `name:` field                                | Filename                                    |
| -------------------------------------------- | ------------------------------------------- |
| `PR - Quality Gate`                          | `pr-quality-gate.yml`                       |
| `validate-env`                               | `validate-env.yml`                          |
| `Dependency Vulnerability Audit`             | `dependency-vulnerability-audit.yml`        |
| `Rhino CLI Parity Audit`                     | `rhino-cli-parity-audit.yml`                |
| `Test And Deploy - Backend - Development`    | `test-and-deploy-backend-development.yml`   |
| `Test And Deploy - Frontend - Development`   | `test-and-deploy-frontend-development.yml`  |
| `Test And Deploy - Fullstack - Development`  | `test-and-deploy-fullstack-development.yml` |
| `Test - Crud FS (TypeScript/Next.js)`        | `test-crud-fs-ts-nextjs.yml`                |
| `Test - Crud BE (Java/Spring Boot)`          | `test-crud-be-java-springboot.yml`          |
| `Test - Crud BE (Java/Vert.x)`               | `test-crud-be-java-vertx.yml`               |
| `Test - Crud BE (Elixir/Phoenix)`            | `test-crud-be-elixir-phoenix.yml`           |
| `Test - Crud BE (F#/Giraffe)`                | `test-crud-be-fsharp-giraffe.yml`           |
| `Test - Crud BE (Go/Gin)`                    | `test-crud-be-golang-gin.yml`               |
| `Test - Crud BE (Python/FastAPI)`            | `test-crud-be-python-fastapi.yml`           |
| `Test - Crud BE (Rust/Axum)`                 | `test-crud-be-rust-axum.yml`                |
| `Test - Crud BE (Kotlin/Ktor)`               | `test-crud-be-kotlin-ktor.yml`              |
| `Test - Crud BE (TypeScript/Effect)`         | `test-crud-be-ts-effect.yml`                |
| `Test - Crud BE (C#/ASP.NET Core)`           | `test-crud-be-csharp-aspnetcore.yml`        |
| `Test - Crud BE (Clojure/Pedestal)`          | `test-crud-be-clojure-pedestal.yml`         |
| `Test - Crud FE (TypeScript/Next.js)`        | `test-crud-fe-ts-nextjs.yml`                |
| `Test - Crud FE (TypeScript/TanStack Start)` | `test-crud-fe-ts-tanstack-start.yml`        |
| `Test - Crud FE (Dart/Flutter Web)`          | `test-crud-fe-dart-flutterweb.yml`          |

## Examples

### ✅ Correctly aligned name and filename

```yaml
# File: .github/workflows/pr-quality-gate.yml
name: PR - Quality Gate
```

Derivation: `PR - Quality Gate` → lowercase → `pr - quality gate` → spaces to hyphens → `pr---quality-gate` → collapse hyphens → `pr-quality-gate` → append `.yml` → `pr-quality-gate.yml`. Matches filename.

---

```yaml
# File: .github/workflows/test-crud-be-java-springboot.yml
name: Test - Crud BE (Java/Spring Boot)
```

Derivation: `Test - Crud BE (Java/Spring Boot)` → lowercase → `test - crud be (java/spring boot)` → remove `(`, `)`, `/` → `test - crud be javaspring boot` → spaces to hyphens → `test---crud-be-javaspring-boot` → collapse hyphens → `test-crud-be-javaspring-boot` → append `.yml` → `test-crud-be-java-springboot.yml`.

The actual filename is `test-crud-be-java-springboot.yml`. `Java/Spring Boot` maps to `java-springboot` (slash removed, space removed). See the Special Considerations section below.

### ❌ Misaligned name and filename

```yaml
# File: .github/workflows/quality-gate.yml  ← missing "pr-" prefix
name: PR - Quality Gate
```

A developer seeing "PR - Quality Gate" fail in the UI would look for `pr-quality-gate.yml`. They would not find it under `quality-gate.yml`.

## Special Considerations

### Permitted abbreviations for long names

When the fully derived filename would be excessively long (over 60 characters before `.yml`), abbreviations are permitted provided they are applied consistently and the mapping remains obvious. Established abbreviations in this codebase:

| Full word/phrase | Abbreviation                       |
| ---------------- | ---------------------------------- |
| `Backend`        | `be`                               |
| `Spring Boot`    | `springboot` (no space, no hyphen) |
| `ASP.NET Core`   | `aspnetcore`                       |

When using an abbreviation, update this table so the mapping remains documented and reviewable.

### Language/framework identifiers in parentheses

The pattern `(Language/Framework)` in a name maps to `language-framework` in the filename: parentheses are removed, the `/` is removed, a hyphen separates language from framework, and the whole segment is lowercased. For example, `(Java/Spring Boot)` → `java-springboot`.

### Version Alignment Policy

`main-ci.yml` is deleted; it is no longer the source of truth for language versions. Each
language's version is instead pinned once, as the `default:` on that language's setup composite
action input, and every workflow that calls the action inherits it unless it explicitly overrides
the input:

| Language | Composite action               | Version input    |
| -------- | ------------------------------ | ---------------- |
| Go       | `.github/actions/setup-golang` | `go-version`     |
| Elixir   | `.github/actions/setup-elixir` | `elixir-version` |
| Python   | `.github/actions/setup-python` | `python-version` |
| Node.js  | `.github/actions/setup-node`   | `node-version`   |

**Rule**: When upgrading a language version, update the composite action's `default:` in the same
commit as any workflow that pins its own explicit override for that language. Version drift between
the composite action's default and a workflow's explicit override creates inconsistencies where CI
passes on `main` but a manually dispatched workflow fails (or vice versa).

**Frontend workflows install Go for codegen**: The three CRUD frontend workflows
(`test-crud-fe-ts-nextjs.yml`, `test-crud-fe-ts-tanstack-start.yml`,
`test-crud-fe-dart-flutterweb.yml`) install Go and run `rhino-cli` for contract codegen before
running tests, via the same `setup-golang` composite action as `test-crud-be-golang-gin.yml`.

### Adding new workflows

When creating a new workflow:

1. Choose a `name:` that describes the workflow's purpose clearly (it appears in the GitHub UI).
2. Derive the filename from the `name:` using the rule above.
3. If the derived name would exceed 60 characters, apply a documented abbreviation.
4. Add the new pair to the reference table in this document.

## Tools and Automation

Currently no automated validator enforces this rule. The `repo-rules-checker` agent validates adherence during governance audits.

## 🔗 References

**Related Development Standards:**

- [Nx Target Standards](./nx-targets.md) - Consistent naming applied to Nx target identifiers
- [Commit Message Convention](../workflow/commit-messages.md) - Another naming consistency rule for developer-facing identifiers

**Agents:**

- `repo-rules-checker` - Validates that workflow filenames match their `name:` fields
- `repo-rules-fixer` - Corrects misaligned workflow filenames or name fields
