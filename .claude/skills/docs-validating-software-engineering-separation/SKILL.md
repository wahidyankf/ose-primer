---
name: docs-validating-software-engineering-separation
description: Validates software engineering documentation separation — ensures docs/explanation/ style guides focus on repository-specific conventions only (not generic language tutorials), and that every programming language README has proper prerequisite statements linking to external learning resources.
created: 2026-02-07
---

# Validating Software Engineering Documentation Separation

This Skill provides comprehensive guidance for validating the separation between repository-specific style guides (`docs/explanation/software-engineering/`) and generic educational content (which belongs in external resources, not this repository), as defined in the [Programming Language Documentation Separation Convention](../../../repo-governance/conventions/structure/programming-language-docs-separation.md).

## Purpose

Use this Skill when:

- Implementing style guide separation validation in checker agents
- Validating `docs/explanation/` content doesn't duplicate generic language tutorials
- Ensuring prerequisite knowledge statements exist and link to external resources
- Checking style guides focus on repository-specific conventions only
- Understanding content separation patterns

## Validation Scope

**Default scope**: All language directories under `docs/explanation/software-engineering/programming-languages/`.

**When user specifies scope** (e.g., "check Go docs"): Validate only that language directory.

## Core Validation Principle

**CRITICAL**: `docs/explanation/` content MUST NOT contain generic language tutorials or duplicate official language documentation.

**Separation Pattern**:

- **External resources** (official docs, standard guides) = Language education (syntax, fundamentals, generic patterns)
- **`docs/explanation/`** = Style guides (platform naming conventions, framework choices, repository-specific patterns)

See [Programming Language Documentation Separation Convention](../../../repo-governance/conventions/structure/programming-language-docs-separation.md) for complete rules.

## What to Validate

### 1. Language Directory Discovery

1. Glob `docs/explanation/software-engineering/programming-languages/*/`
2. For each directory, collect README.md and all `.md` files
3. Validate each language directory independently

### 2. Prerequisite Knowledge Statements

**For each language README**:

- Check README.md has "Prerequisite Knowledge" or "Before You Begin" section
- Section links to official/external language learning resources (absolute URLs)
- Section states this is NOT a language tutorial
- Cross-reference links resolve correctly

### 3. No Content Duplication

**For each `.md` file in docs/explanation/**:

- Check for generic language syntax tutorials (VIOLATION)
- Check for basic examples explaining how the language works without platform context (VIOLATION)
- Check for content duplicating official language documentation (VIOLATION)
- Verify content focuses on repository-specific conventions

**FAIL patterns**:

- Teaching language syntax (variables, loops, functions)
- By-example learning content without platform-specific context
- Generic error handling patterns (not platform-specific)
- Content paraphrasing or copying from official docs

**PASS patterns**:

- Platform-specific naming conventions
- Framework choice rationale ("We use Gin because...")
- Repository-specific architecture patterns
- Platform-specific anti-patterns

### 4. Cross-Reference Link Validation

**For each language README**:

- External prerequisite links resolve correctly (use WebFetch to verify)
- Link text is descriptive and accurate
- Absolute URLs used for external resources

## Validation Workflow

### Step 1: Discover Language Directories

```bash
# Glob all language directories
docs/explanation/software-engineering/programming-languages/*/
```

### Step 2: Validate Each Language Directory

For each language directory:

1. Check README.md exists
2. Verify prerequisite statement exists and has external links
3. Read all .md files and check for tutorial content
4. Check external links resolve

### Step 3: Report Findings

- Group findings by criticality
- Write findings progressively (don't buffer)

## Common Separation Violations

### Violation 1: Duplicating Generic Language Content

**FAIL** (`docs/explanation/.../golang/`):

```markdown
## Variables in Go

Go variables can be declared multiple ways:
var x int = 10
y := 20
```

**Why**: Teaching Go syntax (belongs in official Go docs/external resources)

**PASS** (`docs/explanation/.../golang/`):

```markdown
**Prerequisite**: Know Go basics — see [A Tour of Go](https://go.dev/tour/).

## Variable Naming in This Platform

- Domain entities: `CrudPayment`, `RhinoCommand`
- Repository variables: `crudRepo`, `rhinoRepo`
```

**Why**: Platform-specific naming conventions (not syntax tutorial)

### Violation 2: Missing Prerequisite Statement

**FAIL**:

```markdown
# Java

Java is used for...

## Best Practices
```

**Why**: No prerequisite statement linking to external resources

**PASS**:

```markdown
# Java

## Prerequisite Knowledge

**This documentation assumes you already know Java.** If you are new:

- [Java documentation](https://docs.oracle.com/en/java/)
- [Java tutorials](https://docs.oracle.com/javase/tutorial/)

These are platform-specific style guides, not Java tutorials.
```

## Criticality Levels

**CRITICAL**:

- Prerequisite statement completely missing in docs/explanation README
- Clear generic language tutorial content in docs/explanation/

**HIGH**:

- Prerequisite statement exists but has no external links
- Content duplicates official docs without platform-specific context
- Broken external link in prerequisite statement

**MEDIUM**:

- Prerequisite statement poorly formatted
- External link text not descriptive

**LOW**:

- Enhanced prerequisite explanations possible
- Additional external cross-references could be added

## Related Conventions

**Primary**: [Programming Language Documentation Separation Convention](../../../repo-governance/conventions/structure/programming-language-docs-separation.md)

**Supporting**:

- [Diátaxis Framework](../../../repo-governance/conventions/structure/diataxis-framework.md)
- [Content Quality Standards](../../../repo-governance/conventions/writing/quality.md)

## Related Skills

- repo-assessing-criticality-confidence
- repo-applying-maker-checker-fixer
- repo-generating-validation-reports

## Related Agents

- docs-software-engineering-separation-checker — Validates separation using this skill
- docs-software-engineering-separation-fixer — Fixes violations
- docs-maker — Creates style guide content
