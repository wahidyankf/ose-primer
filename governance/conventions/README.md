---
title: "Conventions"
description: Documentation conventions and standards for open-sharia-enterprise
category: explanation
subcategory: conventions
tags:
  - index
  - conventions
  - standards
created: 2025-11-22
updated: 2026-04-04
---

# Conventions

Documentation conventions and standards for the open-sharia-enterprise project. These documents define how documentation should be organized, named, written, and formatted.

**Governance**: All conventions in this directory serve the [Vision](../vision/open-sharia-enterprise.md) (Layer 0) and implement the [Core Principles](../principles/README.md) (Layer 1) as part of the six-layer architecture. Each convention MUST include a "Principles Implemented/Respected" section that explicitly traces back to foundational principles. See [Repository Governance Architecture](../repository-governance-architecture.md) for complete governance model and [Convention Writing Convention](./writing/conventions.md) for structure requirements.

## 🎯 Scope

**This directory contains conventions for DOCUMENTATION:**

**Belongs Here:**

- How to write and format markdown content
- Documentation organization and structure (Diataxis)
- File naming, linking, and cross-referencing
- Visual elements in docs (diagrams, colors, emojis, math notation)
- Content quality and accessibility standards
- Documentation file formats (tutorials, plans)
- static-site **content** writing conventions (historical - no active legacy sites remain)
- Repository documentation standards (README, CONTRIBUTING)

**Does NOT Belong Here (use [Development](../development/README.md) instead):**

- Software development methodologies (BDD, testing, agile)
- Build processes and tooling workflows
- static-site **theme/layout development** (historical - no active legacy sites remain)
- Development infrastructure (temporary files, build artifacts)
- Git workflows and commit practices
- AI agent development standards
- Code quality and testing practices

## 🧭 The Layer Test for Conventions

**Question**: Does this document answer "**WHAT are the documentation rules?**"

**Belongs in conventions/** if it defines:

- HOW to write markdown content (formatting, syntax, structure)
- WHAT files should be named or organized
- WHAT visual standards to follow in docs (colors, diagrams, emojis)
- WHAT content quality standards apply to documentation

**Does NOT belong** if it defines:

- WHY we value something (that's a principle)
- HOW to develop software/themes (that's a development practice)
- HOW to solve a specific problem (that's a how-to guide)

**Examples**:

- "Files must use lowercase kebab-case names" - Convention (documentation rule)
- "Use 2-space indentation for nested lists" - Convention (documentation formatting)
- "Web app themes use Tailwind CSS" - Development (software practice)
- "Why we avoid time estimates in tutorials" - Principle (foundational value)

## 📂 Directory Structure

Conventions are organized into 6 semantic categories:

- **[formatting/](#formatting)** - Markdown formatting, syntax, visual elements
- **[linking/](#linking)** - Cross-reference and internal linking standards
- **[writing/](#writing)** - Content quality, validation, writing standards
- **[structure/](#structure)** - Documentation organization, file naming, plans

---

## 🎨 Formatting

Standards for markdown formatting, syntax, and visual elements.

- [Color Accessibility](./formatting/color-accessibility.md) - MASTER REFERENCE for all color decisions. Verified accessible color palette (Blue #0173B2, Orange #DE8F05, Teal #029E73, Purple #CC78BC, Brown #CA9161) supporting all color blindness types, WCAG AA standards, with complete implementation guidance for Mermaid diagrams and AI agent categorization
- [Diagrams and Schemas](./formatting/diagrams.md) - Standards for Mermaid diagrams (primary) and ASCII art (optional) with color-blind friendly colors for accessibility
- [Emoji Usage](./formatting/emoji.md) - Semantic emoji usage to enhance document scannability and engagement with accessible colored emojis
- [Indentation](./formatting/indentation.md) - Standard markdown indentation using 2 spaces per indentation level. YAML frontmatter uses 2 spaces. Code blocks use language-specific conventions
- [Linking Convention](./formatting/linking.md) - Standards for linking between documentation files using GitHub-compatible markdown. Defines two-tier formatting for rule references (first mention = markdown link, subsequent mentions = inline code)
- [Mathematical Notation](./formatting/mathematical-notation.md) - Standards for LaTeX notation for mathematical equations and formulas. Defines inline (`$...$`) vs display (`$$...$$`) delimiters, forbidden contexts (code blocks, Mermaid), GitHub rendering compatibility
- [Nested Code Fences](./formatting/nested-code-fences.md) - Standards for properly nesting code fences when documenting markdown structure within markdown content. Defines fence depth rules (outer = 4 backticks, inner = 3 backticks), orphaned fence detection, and validation checklist
- [Timestamp Format](./formatting/timestamp.md) - Standard timestamp format using UTC+7 (Indonesian WIB Time)

## Linking

Standards for cross-referencing and internal linking between repository content.

See the `governance/conventions/linking/` directory for linking conventions. No standalone convention files are currently defined here; linking standards are covered by [Linking Convention](./formatting/linking.md) in the Formatting section.

## ✍️ Writing

Content quality standards, validation methodology, and writing guidelines.

- [Content Quality Principles](./writing/quality.md) - Universal markdown content quality standards applicable to ALL repository markdown contexts (docs/, Next.js web content, plans/, root files). Covers writing style and tone (active voice, professional, concise), heading hierarchy (single H1, proper nesting), accessibility (alt text, semantic HTML, color contrast, screen readers), and formatting
- [Conventions](./writing/conventions.md) - **Meta-convention** defining how to write and organize convention documents. Covers document structure, scope boundaries, quality checklist, when to create new vs update existing, length guidelines, and integration with agents. Essential reading for creating or updating conventions
- [Dynamic Collection References](./writing/dynamic-collection-references.md) - Standards for referencing dynamic collections (agents, principles, conventions, practices, skills) without hardcoding counts. Prevents documentation drift by requiring count-free references with links to authoritative index documents. **Agents**: repo-rules-checker, repo-rules-fixer
- [Factual Validation](./writing/factual-validation.md) - Universal methodology for validating factual correctness across all repository content using web verification (WebSearch + WebFetch). Defines core validation methodology (command syntax, features, versions, code examples, external refs, mathematical notation, diagram colors), web verification workflow, confidence classification (Verified, Unverified, Error, Outdated)
- [OSS Documentation](./writing/oss-documentation.md) - Standards for repository documentation files (README, CONTRIBUTING, ADRs, security) following open source best practices
- [README Quality](./writing/readme-quality.md) - Quality standards for README.md files ensuring engagement, accessibility, and scannability. Defines problem-solution hooks, jargon elimination (plain language over corporate speak), acronym context requirements, benefits-focused language, navigation structure, and paragraph length limits. **Agents**: readme-maker, readme-checker
- [Web Research Delegation](./writing/web-research-delegation.md) - Normative rule requiring AI agents to delegate public-web information gathering to the `web-research-maker` subagent when research exceeds the delegation threshold (2+ `WebSearch` calls or 3+ `WebFetch` calls for a single claim). Enumerates three exceptions (single-shot known URL; fixer re-validation; link-reachability checkers). **Agents**: web-research-maker, repo-rules-checker

## 🗂️ Structure

Documentation organization frameworks, file naming, and project planning structure.

- [Agent Naming Convention](./structure/agent-naming.md) - Single exception-free filename rule for agent files in `.claude/agents/` and `.opencode/agent/`. Defines scope vocabulary, role vocabulary (maker, checker, fixer, dev, deployer, manager), and the audit command enforced by `repo-rules-checker`
- [Diataxis Framework](./structure/diataxis-framework.md) - Understanding the four-category documentation organization framework we use (Tutorials, How-To, Reference, Explanation)
- [File Naming Convention](./structure/file-naming.md) - Lowercase kebab-case file names anchored on standard markdown and GitHub compatibility
- [Plans Organization](./structure/plans.md) - Standards for organizing project planning documents in plans/ folder including structure (ideas.md, backlog/, in-progress/, done/), naming patterns (YYYY-MM-DD\_\_identifier/), lifecycle stages, and project identifiers. Defines how plans move from ideas - backlog - in-progress - done
- [Specs Directory Structure](./structure/specs-directory-structure.md) - Canonical directory structure for Gherkin feature files, C4 architecture diagrams, and OpenAPI contracts in the specs/ directory. Defines path patterns, domain subdirectory rules (required for BE/FE, flat for CLI), and lib spec organization
- [Workflow Naming Convention](./structure/workflow-naming.md) - Single exception-free filename rule for workflow files under `governance/workflows/` (except `meta/` reference docs). Defines scope vocabulary, type vocabulary (quality-gate, execution, setup), and the audit command enforced by `repo-rules-checker` and `rhino-cli workflows validate-naming`

## Tutorials

Tutorial conventions are delivered via skills rather than standalone convention files. Use the following skills for tutorial guidance:

- `docs-creating-by-example-tutorials` — by-example tutorial creation methodology
- `docs-creating-in-the-field-tutorials` — in-the-field tutorial creation methodology

These skills build upon and extend the writing conventions above.

## static-site (Historical)

static-site site-specific content conventions. **All legacy sites have migrated to Next.js 16.** These conventions are preserved for historical reference only.

## 🔗 Related Documentation

- [Repository Governance Architecture](../repository-governance-architecture.md) - Complete six-layer architecture (Layer 2: Conventions)
- [Core Principles](../principles/README.md) - Layer 1: Foundational values that govern conventions
- [Development](../development/README.md) - Layer 3: Software practices (parallel governance with conventions)
- [Software Design Reference](../../docs/explanation/software-engineering/software-design-reference.md) - Cross-reference to authoritative software design and coding standards

---

**Last Updated**: 2026-04-04
