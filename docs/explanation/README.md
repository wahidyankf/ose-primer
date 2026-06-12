---
title: Explanation
description: Conceptual documentation for open-sharia-enterprise
category: explanation
tags:
  - index
  - explanation
  - concepts
---

# Explanation

**Understanding-oriented documentation** that provides background, context, and conceptual knowledge about the open-sharia-enterprise project.

## What is Explanation Documentation?

Per the [Diátaxis framework](../../repo-governance/conventions/structure/diataxis-framework.md), Explanation documentation:

- **Deepens understanding** of concepts, design decisions, and systems
- **Answers "why?"** questions and provides context
- **Clarifies background** and alternative approaches
- **Discusses trade-offs** and decision rationale

This is distinct from:

- **Tutorials** (learning by doing)
- **How-to Guides** (solving specific problems)
- **Reference** (technical specifications)

## Documentation Scope

This directory contains conceptual documentation across multiple areas:

- **Repository Governance** - How we organize, validate, and enforce standards
- **Software Design** - System design and technical decisions
- **Domain Concepts** - Shariah-compliant enterprise principles and Islamic business foundations
- **Technical Background** - Technologies, patterns, and frameworks used in the project

---

## 🏛️ Repository Governance

The repository follows a **six-layer governance architecture** (Vision → Principles → Conventions → Development → Agents → Workflows) where each layer builds on the foundation above.

**See [Rules](../../repo-governance/README.md)** for governance overview with architecture diagram, layer descriptions, and decision trees.

**See [Repository Governance Architecture](../../repo-governance/repository-governance-architecture.md)** for comprehensive explanation with traceability examples, usage guidance, and verification methods.

---

## 📋 Documentation Index

### Repository Governance

- **[Rules](../../repo-governance/README.md)** - All governance layers (Vision, Principles, Conventions, Development, Workflows)
- **[Repository Governance Architecture](../../repo-governance/repository-governance-architecture.md)** - Comprehensive architecture deep-dive
- **[Plan Domain Parity — Design Decisions](./plan-domain-parity-decisions.md)** - All 26 deviation-matrix decisions from the 2026-06-06 cross-repo parity effort, including the recorded Safety Invariant 6 direct-push deviation
- **[Gherkin Step-Keyword Cardinality — Cross-Repo Parity Decisions](./gherkin-step-keyword-cardinality-parity-decisions.md)** - All 13 deviation-matrix decisions from the 2026-06-07 cross-repo parity effort, including the four deliberate deviations (standalone Rust CLI command, Step 0.5 preflight port, per-repo CI wiring, direct-main-push mode)
- **[Standardize App Spec Trees — Parity Decisions](./standardize-app-spec-trees-parity-decisions.md)** - All parity decisions from the 2026-06-11 cross-repo spec-tree rename effort, including the flat product-surface naming scheme, the `be`-over-`api` rule, the TDD-shaped Rust test update, and the recorded main-to-main delivery deviation
- **[Lint and Safety Parity — Design Decisions](./lint-safety-parity-decisions.md)** - Every deviation-matrix decision from the 2026-06-12 cross-repo lint-and-safety effort, including the executed gates (Rust `forbid(unsafe_code)`, C# `AnalysisLevel=latest-All`, Python basedpyright-strict, hadolint/shellcheck/actionlint), the skipped rows (D1b/D2/D5/D9/D10), the D5 deferral and exemption philosophy, and the recorded main-to-main delivery deviation

### Software Engineering

- **[Software Engineering](./software-engineering/README.md)** - Complete index of programming languages, frameworks, architecture patterns, and development practices
- **[C4 Architecture Model](./software-engineering/architecture/c4-architecture-model/README.md)** - Visualizing software architecture through hierarchical abstraction levels
- **[Domain-Driven Design (DDD)](./software-engineering/architecture/domain-driven-design-ddd/README.md)** - Strategic and tactical patterns for modeling complex business domains

### Post-Mortems

- **[Post-Mortems](./post-mortems/README.md)** - Blameless incident retrospectives (what happened, why, and what changed). Includes the copy-paste template and index; governed by the [Post-Mortem Convention](../../repo-governance/conventions/structure/post-mortems.md)

### Domain Concepts

_Documentation for Shariah-compliant enterprise principles and Islamic business foundations to be added as the project evolves._

### Technical Background

_Documentation for key technologies, patterns, and frameworks to be added as the project evolves._

---
