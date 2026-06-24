---
title: Development
description: Development conventions and standards for open-sharia-enterprise
category: explanation
subcategory: development
tags:
  - index
  - development
  - conventions
  - ai-agents
---

# Development

Development conventions and standards for the open-sharia-enterprise project. These documents define how to create and manage development practices, tools, and workflows.

**Governance**: All development practices in this directory serve the [Vision](../vision/open-sharia-enterprise.md) (Layer 0), implement the [Core Principles](../principles/README.md) (Layer 1), and implement/enforce [Documentation Conventions](../conventions/README.md) (Layer 2) as part of the six-layer architecture. Each practice MUST include TWO mandatory sections: "Principles Implemented/Respected" and "Conventions Implemented/Respected". See [Repository Governance Architecture](../repository-governance-architecture.md) for complete governance model and [AI Agents Convention](./agents/ai-agents.md) for structure requirements.

## 🎯 Scope

**This directory contains conventions for SOFTWARE DEVELOPMENT:**

**✅ Belongs Here:**

- Software development methodologies (BDD, testing, agile practices)
- Build processes, tooling, and automation workflows
- static-site **theme/layout development** (historical - no active legacy sites remain)
- Development infrastructure (temporary files, build artifacts, reports)
- Git workflows and commit message standards
- AI agent development and configuration
- Code quality, testing, and deployment practices
- Acceptance criteria and testable requirements

**❌ Does NOT Belong Here (use [Conventions](../conventions/README.md) instead):**

- How to write and format documentation
- Markdown writing standards and style guides
- Documentation organization (Diátaxis framework)
- File naming and linking in docs
- static-site **content** writing (historical - no active legacy sites remain)
- Visual documentation elements (diagrams, colors in docs)
- Documentation quality and accessibility

## The Layer Test for Development

**Question**: Does this document answer "**HOW do we develop software?**"

✅ **Belongs in development/** if it defines:

- HOW to develop software systems (code, themes, layouts, build processes)
- WHAT development workflows to follow (git, commits, testing)
- HOW to automate development tasks (git hooks, CI/CD, AI agents)
- WHAT development tools and standards to use

❌ **Does NOT belong** if it defines:

- WHY we value something (that's a principle)
- HOW to write documentation (that's a convention)
- HOW to solve a specific user problem (that's a how-to guide)

**Examples**:

- "Use Trunk Based Development for git workflow" → ✅ Development (software practice)
- "Commit messages must follow Conventional Commits" → ✅ Development (development workflow)
- "static-site themes use static-site Pipes for asset processing" → ✅ Development (software development)
- "Markdown files use 2-space indentation" → ❌ Convention (documentation rule)
- "Why we automate repetitive tasks" → ❌ Principle (foundational value)

## Document Types

Development practices in this directory fall into several categories:

### Workflow Documentation

**Purpose:** Define step-by-step processes for development activities
**Examples:** Trunk Based Development, Commit Messages
**Structure:** Context → Process → Examples → Exceptions

### Standards Documentation

**Purpose:** Establish quality gates and requirements
**Examples:** Code Quality, Acceptance Criteria
**Structure:** Purpose → Requirements → Checklist → Examples

### Tool-Specific Documentation

**Purpose:** Define technology-specific best practices
**Structure:** Overview → Conventions → Patterns → Anti-patterns

### Infrastructure Documentation

**Purpose:** Document system design decisions
**Examples:** Temporary Files
**Structure:** Problem → Solution → Organization → Usage

## 📋 Contents

### Workflow Documentation

- [Implementation Workflow Convention](./workflow/implementation.md) - Three-stage development workflow: make it work (functionality first), make it right (refactor for quality), make it fast (optimize only if needed). Includes surgical changes (touch only what you must when editing) and goal-driven execution (define success criteria, loop until verified). Implements Simplicity Over Complexity, YAGNI, and Progressive Disclosure principles
- [Trunk Based Development Convention](./workflow/trunk-based-development.md) - Git workflow using Trunk Based Development for continuous integration
- [Commit Message Convention](./workflow/commit-messages.md) - Understanding Conventional Commits, commit granularity, and why we use them
- [Reproducible Environments Convention](./workflow/reproducible-environments.md) - Practices for creating consistent, reproducible development and build environments. Covers runtime version management (Volta), dependency locking, environment configuration, and containerization
- [Worktree Toolchain Initialization](./workflow/worktree-setup.md) - Mandatory two-step init (`npm install` then `npm run doctor -- --fix`) in the root repository worktree after creating or entering a git worktree. The first step keeps `node_modules/` consistent with `package-lock.json`; the second actively converges the 18+ polyglot toolchains (Go, Java, Rust, Elixir, Python, .NET, Dart, Clojure, Kotlin, C#, Node) managed by `rhino-cli doctor` — required because `package.json`'s `postinstall` hook swallows doctor failures with `|| true`
- [Git Push Default Convention](./workflow/git-push-default.md) - Default git push behavior: direct push to origin main unless user prompt or plan explicitly requests a PR. Governs plan-maker, plan-checker, plan-fixer, and plan-execution workflow
- [Git Push Safety Convention](./workflow/git-push-safety.md) - Requires explicit per-instance user approval before any AI agent or automation executes `git push --force`, `--force-with-lease`, or `--no-verify`; prior approval does not carry forward
- [Native-First Toolchain Management Convention](./workflow/native-first-toolchain.md) - Architectural decision to use native package managers and `rhino-cli doctor` instead of Terraform, Ansible, or Docker Dev Containers for development environment setup
- [PR Merge Protocol Convention](./workflow/pr-merge-protocol.md) - Practice requiring explicit user approval before merging pull requests and mandating all quality gates pass before merge
- [CI Monitoring Convention](./workflow/ci-monitoring.md) - Standards for monitoring GitHub Actions CI runs without exhausting the GitHub API rate limit — required tooling, poll intervals, trigger discipline, and recovery procedures
- [CI Post-Push Verification Convention](./workflow/ci-post-push-verification.md) - After pushing to origin main, manually trigger all related GitHub CI workflows and verify they pass before considering the work complete
- [Test-Driven Development Convention](./workflow/test-driven-development.md) - Mandates TDD (Red→Green→Refactor) as the required practice for all code changes across the repository
- [Dependency Bump Stability & Safety Policy](./workflow/dependency-bump-policy.md) - Three-path decision tree (LTS / 60-day soak / security waiver), exact-pin hard rule, five-source CVE clearance, and EPSS escalation governing every dependency bump across npm, Cargo, .NET, Go, Docker, and GitHub Actions

### Quality Standards Documentation

- [Code Quality Convention](./quality/code.md) - Automated code quality tools and git hooks (Prettier, Husky, lint-staged) for consistent formatting and commit validation
- [Content Preservation Convention](./quality/content-preservation.md) - Principles and processes for preserving knowledge when condensing files and extracting duplications. Covers the MOVE NOT DELETE principle and offload decision tree
- [Repository Validation Methodology Convention](./quality/repository-validation.md) - Standard validation methods and patterns for repository consistency checking. Covers frontmatter extraction, validation checks, and best practices
- [Criticality Levels Convention](./quality/criticality-levels.md) - Universal criticality level system for categorizing validation findings by importance and urgency (CRITICAL/HIGH/MEDIUM/LOW)
- [Fixer Confidence Levels Convention](./quality/fixer-confidence-levels.md) - Universal confidence level system for fixer agents to assess and apply validated fixes (HIGH/MEDIUM/FALSE_POSITIVE)
- [Markdown Quality Convention](./quality/markdown.md) - Standards for markdown linting and formatting using markdownlint-cli2 and Prettier for consistent markdown quality
- [Three-Level Testing Standard](./quality/three-level-testing-standard.md) - Mandatory three-level testing architecture for all projects: unit (all mocked dependencies + Gherkin specs for crud-be), integration (real PostgreSQL, no HTTP for crud-be; in-process mocking for MSW/Godog projects), E2E (full stack + Gherkin specs via Playwright for web apps and API backends)
- [No Machine-Specific Information in Commits](./quality/no-machine-specific-commits.md) - Practice prohibiting absolute local paths, usernames, IP addresses, and environment-specific configuration from committed code
- [Specs-Application Sync Convention](./quality/specs-application-sync.md) - Bidirectional synchronization requirement between specs/ and application code in apps/ and libs/: C4 diagrams, Gherkin feature files, and specs READMEs must reflect actual architecture and behavior
- [Manual Behavioral Verification Convention](./quality/manual-behavioral-verification.md) - Practice requiring manual verification of UI features and API endpoints using Playwright MCP tools and curl after implementing changes, across ALL supported locales for multi-locale apps
- [Evidence Capture Convention](./quality/evidence-capture.md) - Standards for capturing and organizing testing evidence (screenshots, curl outputs, console logs) in the plan's committed `evidence/` subfolder and inline in `delivery.md` during plan execution, with locale and breakpoint coverage requirements
- [Feature Change Completeness Convention](./quality/feature-change-completeness.md) - Practice requiring all related specs, contracts, tests, and documentation to be updated as part of any feature change
- [CI Blocker Resolution Convention](./quality/ci-blocker-resolution.md) - Practice mandating that preexisting CI blockers are investigated at the root cause and fixed properly, never bypassed
- [Post-Push CI Verification Convention](./quality/post-push-ci-verification.md) - Requirement to trigger and verify related GitHub Actions CI workflows after pushing to origin main, for both human contributors and AI agents
- [Plan Anti-Hallucination Convention](./quality/plan-anti-hallucination.md) - Anti-hallucination guardrails for plan-maker and plan-checker agents: repo-grounding rules, web-research delegation thresholds, and claim verification requirements
- [No Secrets in Committed Files Convention](../conventions/security/no-secrets-in-committed-files.md) - Hard iron rule prohibiting system secrets (SSH keys, passwords, tokens, API keys) from any git-committed file, including plans and docs
- [Environment File Access Convention](../conventions/security/env-file-access.md) - Six-layer policy governing AI agent access to `.env*` files; only `.env.example` is permitted for reading, writing, editing, and committing
- [User-Facing Delivery Hardening Convention](./quality/user-facing-delivery-hardening.md) - Fourteen durable rules so design-parity and behavioral defects cannot ship past green gates for user-facing work
- [Regression Test Mandate](./quality/regression-test-mandate.md) - Blocking rule requiring every bug fix to land with a reproducing test in the same commit/PR; the bug-driven dual of Feature Change Completeness, covering all defect types (behavioral, visual, content, API)
- [Live-Tester Systematic Coverage](./quality/live-tester-systematic-coverage.md) - Six forcing-functions (shared-control matrix, URL round-trip, declared-invariant conformance, styling consistency audit, usability probes, recurrence critic) that convert sampling into enumeration for the three live-site tester agents and the web-ux-test-fixing-planning workflow

### Pattern Documentation

- [Database Audit Trail Pattern](./pattern/database-audit-trail.md) - Required 6-column audit trail (created_at/by, updated_at/by, deleted_at/by) that every database table must include. Covers the migration tool for each of the 12 demo backends, language-agnostic migration requirements, Java/Spring Boot (Liquibase + JPA Auditing), and soft-delete discipline
- [Maker-Checker-Fixer Pattern Convention](./pattern/maker-checker-fixer.md) - Three-stage quality workflow for content creation and validation. Covers agent roles, workflow stages with user review gates, and confidence level integration
- [Functional Programming Practices](./pattern/functional-programming.md) - Guidelines for applying functional programming principles in TypeScript/JavaScript. Covers immutability patterns, pure functions, and function composition
- [Hexagonal Architecture](./pattern/hexagonal-architecture.md) - Architectural pattern for structuring applications with clear separation between business logic and external adapters
- [Hexagonal Architecture — Backend](./pattern/hexagonal-architecture-be.md) - Backend-specific application of hexagonal architecture for API and service layers
- [Hexagonal Architecture — Web](./pattern/hexagonal-architecture-web.md) - Web UI application of hexagonal architecture for frontend projects
- [Hexagonal Architecture — CLI](./pattern/hexagonal-architecture-cli.md) - CLI application of hexagonal architecture for command-line tools
- [OpenAPI Contract-First Development](./pattern/openapi-contract-first.md) - Contract-first API development pattern using OpenAPI 3.1 specs as the single source of truth for types and codegen

### Practice Documentation

- [Proactive Preexisting Error Resolution](./practice/proactive-preexisting-error-resolution.md) - When encountering preexisting errors, bugs, broken tests, or incorrect configurations during any work, fix the root cause rather than ignoring, monkey-patching, or passively mentioning the problem. Covers the three anti-patterns (acting ignorant, monkey-patching, passive mentioning), scope judgment (inline/separate commit/plan), and full agent requirements
- [Parallel-by-Default Practice](./practice/parallel-by-default.md) - When work has independent sub-units (tool calls, file reads/edits, searches, or delegated agents), default to running them in parallel rather than serially, capped at three concurrent units of work. Covers the parallel-unless-dependent norm, the cap-3 rationale, and the subagent specialization relationship
- [Task List Discipline](./practice/task-list-discipline.md) - For any non-trivial multi-step work (3+ distinct steps, or any task spanning multiple files or phases), maintain a live task list from the start and keep it continuously in sync with actual progress. Covers the five standards (create before starting, mark in-progress, mark completed immediately, add discovered tasks, one task per outcome) and the relationship to plan delivery checklists

### Agent Standards Documentation

- [AI Agents Convention](./agents/ai-agents.md) - Standards for creating and managing AI agents in the `.claude/agents/` directory (primary source of truth), synced to `.opencode/agents/`. Covers agent naming, file structure, frontmatter requirements, tool access patterns, model selection, and size limits
- [Skill Context Architecture](./agents/skill-context-architecture.md) - Architectural constraint requiring all repository skills to use inline context for universal subagent compatibility. Documents subagent spawning limitation and fork skill alternatives
- [Agent Workflow Orchestration Convention](./agents/agent-workflow-orchestration.md) - Standards for how AI agents plan, execute, verify, and self-improve during multi-step tasks. Covers plan mode triggers, subagent strategy, verification before done, autonomous bug fixing, the self-improvement loop, and task management
- [Model Selection Convention](./agents/model-selection.md) - Standards for selecting the appropriate model tier (opus, sonnet, haiku) for AI agents based on task complexity, with justification requirements and tier comparison
- [Subagent Orchestration Convention](./agents/subagent-orchestration.md) - Rules governing how agents spawn and coordinate subagents, including concurrency limits, result handling, and orchestration patterns

### Infrastructure Documentation

- [Nx Target Standards](./infra/nx-targets.md) - Standard Nx targets that apps and libs must expose, canonical target names, caching rules, and build output conventions
- [Temporary Files Convention](./infra/temporary-files.md) - Guidelines for AI agents creating temporary uncommitted files and folders
- [Acceptance Criteria Convention](./infra/acceptance-criteria.md) - Writing testable acceptance criteria using Gherkin format for clarity and automation. Covers Gherkin syntax and common patterns
- [BDD Spec-to-Test Mapping Convention](./infra/bdd-spec-test-mapping.md) - Mandatory 1:1 mapping between CLI commands and Gherkin specifications. Covers domain-prefixed subcommand pattern, Go file naming (underscores), feature file naming (hyphens), and coverage enforcement via `spec-coverage validate`
- [GitHub Actions Workflow Naming Convention](./infra/github-actions-workflow-naming.md) - Workflow filenames must mirror their `name:` field using a consistent kebab-case derivation rule, enabling developers to navigate between the GitHub UI and the filesystem without ambiguity
- [Docker Monorepo Build Patterns](./infra/docker-monorepo-builds.md) - Patterns and pitfalls for building Docker images in an npm workspace monorepo (workspace symlink resolution, direct node_modules injection, transitive dependency hoisting)
- [CI/CD Conventions](./infra/ci-conventions.md) - Central reference for CI/CD conventions: git hooks, test level definitions, coverage thresholds, Docker patterns, GitHub Actions structure, and naming rules
- [Quality Gate Workflow Defaults Convention](./infra/quality-gate-defaults.md) - Canonical default values (`mode: strict`, `max-iterations: 7`) that all quality gate workflows must use for consistency and auditability

### Frontend Development Documentation

- [Design Tokens Convention](./frontend/design-tokens.md) - Token categories (structural vs. brand), naming rules, per-app override pattern, dark mode requirements, and Tailwind v4 integration
- [Component Patterns Convention](./frontend/component-patterns.md) - CVA variant definitions, Radix UI composition, React.ComponentProps pattern, cn() utility, data-slot attributes, and required component states
- [Accessibility Convention](./frontend/accessibility.md) - WCAG AA compliance, focus-visible management, reduced-motion support, ARIA attributes by component type, hit targets, and form input requirements
- [Styling Convention](./frontend/styling.md) - Tailwind v4 patterns, utility-first approach, class ordering via prettier-plugin-tailwindcss, responsive design, and defensive CSS

## Companion Documents

Each primary practice document in this directory has companion files providing practical guidance:

- **anti-patterns.md** - Common mistakes to avoid (with examples and corrections)
- **best-practices.md** - Recommended patterns and techniques

These companion files exist in each subdirectory: `workflow/`, `quality/`, `pattern/`, `agents/`, and `infra/`. The `frontend/` directory embeds anti-patterns and best practices inline within its convention documents. The `practice/` subdirectory currently contains only one document; companion files will be added as the category grows.

## 🔗 Related Documentation

- [Repository Governance Architecture](../repository-governance-architecture.md) - Complete six-layer architecture (Layer 3: Development)
- [Core Principles](../principles/README.md) - Layer 1: Foundational values that govern development practices
- [Conventions](../conventions/README.md) - Layer 2: Documentation conventions (parallel governance with development)
- [Workflows](../workflows/README.md) - Layer 5: Multi-step processes orchestrating agents

---
