---
title: "Programming Language Documentation Separation Convention"
description: Establishes that docs/explanation/programming-languages/ contains only repository-specific style guides, not language tutorials — external learning resources serve as prerequisites
category: explanation
subcategory: conventions
tags:
  - documentation
  - programming-languages
  - style-guides
  - content-separation
  - dry-principle
created: 2026-02-04
---

# Programming Language Documentation Separation Convention

This convention establishes the clear separation between **repository-specific programming language style guides** in `docs/explanation/software-engineering/programming-languages/` and **educational programming language content** (which lives in external resources, not this repository). It prevents duplication, defines scope boundaries, and ensures prerequisite knowledge relationships are explicit.

## Principles Implemented/Respected

This convention implements the following core principles:

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: Clear separation of concerns prevents confusion about where content belongs. Language fundamentals come from external resources; OSE-specific style lives in `docs/explanation/`.

- **[Documentation First](../../principles/content/documentation-first.md)**: Explicit prerequisite knowledge statements ensure developers know where to learn languages before applying platform styles. Documentation acknowledges the educational foundation.

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Required prerequisite statements make dependencies explicit. No assumption that developers already know languages — we tell them where to learn.

## Purpose

This convention prevents duplication and confusion by defining:

- **What belongs in `docs/explanation/software-engineering/programming-languages/{language}/`**: Repository-specific style guides, coding standards, and conventions
- **What does NOT belong here**: Generic language tutorials, syntax guides, or content duplicating official documentation
- **How to cross-reference**: Explicit prerequisite statements pointing to official docs or established external resources

This follows the **DRY principle** — language fundamentals live in official/external sources, repository-specific conventions live in `docs/explanation/`.

## Scope

### What This Convention Covers

- Scope boundaries for `docs/explanation/software-engineering/programming-languages/{language}/`
- Required prerequisite knowledge statements
- Linking patterns to external learning resources
- Content organization for all programming languages in the repository

### What This Convention Does NOT Cover

- **How to write style guides** — Covered in [Content Quality Principles](../writing/quality.md)
- **Diátaxis framework application** — Covered in [Diátaxis Framework Convention](./diataxis-framework.md)

## Content Separation Rules

### Rule 1: docs/explanation/ Focus — Repository-Specific Style Guides ONLY

**PASS: Repository-specific style guides**:

```
docs/explanation/software-engineering/programming-languages/golang/
├── README.md                  # Overview + prerequisite links to external resources
├── coding-standards.md        # OSE Platform Go conventions
├── error-handling-standards.md # OSE Platform error patterns
├── security-standards.md      # OSE Platform security standards
└── testing-standards.md       # OSE Platform testing standards
```

**Content includes**:

- Naming conventions specific to this platform (variable naming, file structure)
- Framework choices (Gin vs Echo, why we chose X)
- Repository-specific patterns (how we structure services, how we handle errors)
- Platform-specific anti-patterns (mistakes to avoid in this codebase)
- Alignment with repo-governance/principles/software-engineering/ principles

**FAIL: Educational content** (link to external resources instead):

- ❌ Language syntax tutorials (variables, loops, functions)
- ❌ Basic language examples explaining how the language works
- ❌ Beginner/intermediate/advanced learning paths
- ❌ Content duplicating official language documentation

### Rule 2: Explicit Prerequisite Knowledge Statements

**REQUIRED**: Every `docs/explanation/software-engineering/programming-languages/{language}/README.md` MUST include an explicit prerequisite knowledge statement linking to external learning resources.

**Template**:

```markdown
## Prerequisite Knowledge

**This documentation assumes you already know {LANGUAGE} fundamentals.** If you are new to {LANGUAGE}, learn it first:

- [Official {LANGUAGE} documentation](https://...official-docs...)
- [Official {LANGUAGE} tour/getting started](https://...getting-started...)

This documentation focuses exclusively on platform-specific style guides and conventions, **not language fundamentals**.

## What This Documentation Covers

This documentation is the **authoritative reference for {LANGUAGE} coding standards in this platform**. It covers:

- Repository-specific naming conventions
- Framework choices and rationale (why we chose X)
- Architecture patterns specific to this codebase
- Anti-patterns to avoid in this context
- Alignment with [Software Engineering Principles](../../principles/software-engineering/README.md)
```

**Examples**:

**PASS: Clear prerequisite statement**:

```markdown
## Prerequisite Knowledge

**This documentation assumes you already know Go fundamentals.** If you are new to Go:

- [Official Go documentation](https://go.dev/doc/)
- [A Tour of Go](https://go.dev/tour/)

This documentation focuses exclusively on platform-specific Go conventions.
```

**FAIL: No prerequisite statement**:

```markdown
# Golang

Go is used for high-performance services...

## Best Practices

Use goroutines for concurrency...
```

**Why it fails**: Doesn't tell developers where to learn Go fundamentals. Assumes knowledge.

### Rule 3: No Language Tutorial Duplication

**CRITICAL**: Content covered in official language documentation or standard learning resources MUST NOT be duplicated in `docs/explanation/`.

**Decision tree**:

```
Is this content about {LANGUAGE} fundamentals or generic patterns?
├─ Yes → Link to external resources (official docs, standard guides)
│   Examples: syntax, type system, error model, concurrency primitives
│
└─ No → Is this content platform-specific?
    ├─ Yes → docs/explanation/ (style guide)
    │   Examples: "We use Gin for HTTP", "Name variables like this in our codebase"
    │
    └─ No → Still link to external resources (generic programming knowledge)
```

**Example — Error Handling**:

**FAIL: Duplicating generic language patterns in docs/explanation/**:

````markdown
# docs/explanation/.../golang/error-handling.md

## Error Handling

Go returns errors as values:

```go
result, err := someFunction()
if err != nil {
    return fmt.Errorf("operation failed: %w", err)
}
```
````

Use errors.New() for simple errors, fmt.Errorf() with %w to wrap...

````

**Why it fails**: This is generic Go error handling — it's in the official docs. Belongs there, not here.

**PASS: Platform-specific error conventions**:

```markdown
# docs/explanation/.../golang/error-handling-standards.md

**Prerequisite**: Know [Go error handling](https://go.dev/blog/error-handling-and-go) before reading this.

## Platform Error Standards

All platform services MUST:

1. Use structured logging with `slog` for errors
2. Include request IDs for distributed tracing
3. Follow error code taxonomy: `ERRCRUD001`, `ERRRHINO001`
````

**Why it passes**: Focuses on platform-specific standards, links to external prerequisites.

### Rule 4: Cross-Referencing Pattern

**Required linking**:

**From docs/explanation/ → external resources**:

```markdown
## Prerequisite Knowledge

**This documentation assumes you already know {LANGUAGE}.** If you are new:

- [Official {LANGUAGE} docs](https://...official...)
- [Getting started guide](https://...getting-started...)
```

**Linking rules**:

- `docs/explanation/` README.md MUST link to external learning resources (prerequisite)
- Use absolute URLs for external resources
- Use relative paths for internal cross-references

## Scope for All Programming Languages

This convention applies to **ALL** programming languages in the repository:

**Current languages**:

- Java (JVM) — `docs/explanation/.../java/`
- Kotlin (JVM) — `docs/explanation/.../kotlin/`
- Python — `docs/explanation/.../python/`
- TypeScript (Node.js) — `docs/explanation/.../typescript/`
- Golang — `docs/explanation/.../golang/`
- Elixir (BEAM) — `docs/explanation/.../elixir/`
- Dart (Flutter) — `docs/explanation/.../dart/`
- Rust — `docs/explanation/.../rust/`
- Clojure (JVM) — `docs/explanation/.../clojure/`
- F# (.NET) — `docs/explanation/.../f-sharp/`
- C# (.NET) — `docs/explanation/.../c-sharp/`

**Future languages**: Apply same separation pattern when adding new languages.

## Alignment with Software Engineering Principles

Programming language style guides in `docs/explanation/` MUST align with the software engineering principles from [repo-governance/principles/software-engineering/](../../principles/software-engineering/README.md):

### 1. Automation Over Manual

Style guides document automated tooling:

- Linters (golangci-lint for Go, Ruff for Python)
- Formatters (gofmt for Go, Black for Python)
- Code generators
- CI/CD pipelines enforcing standards

### 2. Explicit Over Implicit

Style guides enforce explicitness:

- Explicit error handling (no silent failures)
- Explicit configuration (no hidden magic)
- Explicit imports (no wildcards)
- Explicit types where beneficial

### 3. Immutability Over Mutability

Style guides encourage immutable patterns:

- Value objects and immutable data structures
- Functional approaches where applicable
- Const correctness and readonly semantics

### 4. Pure Functions Over Side Effects

Style guides promote pure functions:

- Functional core, imperative shell architecture
- Pure domain logic, isolated side effects
- Testable business logic without mocks

### 5. Reproducibility First

Style guides enable reproducible builds:

- Dependency version pinning (go.mod, requirements.txt)
- Lockfiles (go.sum, poetry.lock)
- Docker build reproducibility

## Common Mistakes to Avoid

### Mistake 1: Duplicating Educational Content

**FAIL: Duplicating language basics in docs/explanation/**:

````markdown
# docs/explanation/.../golang/best-practices.md

## Variables in Go

Go variables can be declared in multiple ways:

```go
var x int = 10
y := 20
```
````

````

**Why it fails**: Generic Go syntax — belongs in the official docs, not here.

**PASS: Repository-specific convention**:

```markdown
# docs/explanation/.../golang/best-practices.md

**Prerequisite**: Know Go basics from [A Tour of Go](https://go.dev/tour/).

## Variable Naming in This Platform

All platform Go code follows these naming conventions:

- Domain entities: `CrudPayment`, `RhinoCommand`
- Repository variables: `crudRepo`, `rhinoRepo`

**Rationale**: Explicit domain terminology for codebase clarity.
````

### Mistake 2: Missing Prerequisite Statement

**FAIL: No prerequisite link**:

```markdown
# docs/explanation/.../python/README.md

# Python

Python is used for data processing...

## Best Practices

Follow PEP 8 standards...
```

**Why it fails**: Doesn't tell developers where to learn Python. Assumes knowledge.

**PASS: Explicit prerequisite**:

```markdown
# docs/explanation/.../python/README.md

# Python

## Prerequisite Knowledge

**This documentation assumes you already know Python.** If you are new:

- [Python official docs](https://docs.python.org/)
- [Python tutorial](https://docs.python.org/3/tutorial/)

## What This Documentation Covers

Platform-specific Python conventions...
```

## Validation Checklist

Before publishing programming language documentation:

### For docs/explanation/ Style Guides

- [ ] README.md includes explicit prerequisite statement linking to official/external resources
- [ ] Content focuses on platform-specific conventions, not language fundamentals
- [ ] No duplication of content covered in official language documentation
- [ ] Alignment section links to [Software Engineering Principles](../../principles/software-engineering/README.md)
- [ ] Clear scope statement: "This is NOT a tutorial"

## Related Conventions

**Documentation Organization**:

- [Diátaxis Framework](./diataxis-framework.md) — Four-category documentation organization
- [File Naming Convention](./file-naming.md) — Kebab-case file naming rules

**Content Quality**:

- [Content Quality Principles](../writing/quality.md) — Universal quality standards for markdown content
- [README Quality](../writing/readme-quality.md) — README-specific quality standards

**Principles**:

- [Documentation First](../../principles/content/documentation-first.md) — Documentation is mandatory, not optional
- [Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md) — Clear separation prevents confusion
- [Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md) — Explicit prerequisite statements
- [Software Engineering Principles Index](../../principles/software-engineering/README.md) — Software engineering principles that style guides align with

## References

**Platform Documentation**:

- [Software Design Index](../../../docs/explanation/software-engineering/README.md) — Parent documentation for programming language style guides

**Repository Architecture**:

- [Repository Governance Architecture](../../repository-governance-architecture.md) — Six-layer architecture (this convention is Layer 2)
- [Conventions Index](../README.md) — Index of all documentation conventions

## Agents

**Makers**:

- `docs-maker` — Creates style guide content in docs/explanation/ following this convention

**Checkers**:

- `docs-checker` — Validates style guides follow this convention (prerequisite statements, no duplication)
- `docs-software-engineering-separation-checker` — Validates that docs/explanation/ does not contain educational content

**Fixers**:

- `docs-fixer` — Fixes style guide violations (adds missing prerequisite statements, removes duplicated content)
- `docs-software-engineering-separation-fixer` — Fixes educational content found in docs/explanation/

---

**Scope**: All programming languages in repository (Java, Kotlin, Python, TypeScript, Golang, Elixir, Dart, Rust, Clojure, F#, C#)
**Maintainers**: Repository Governance Team
