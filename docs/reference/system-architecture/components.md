---
title: Components & Code Architecture
description: C4 Level 3 component diagrams and Level 4 code architecture
category: reference
tags:
  - architecture
  - c4-model
  - components
---

# Components & Code Architecture

C4 Level 3 component diagrams and Level 4 code architecture for the Open Sharia Enterprise platform.

## 🏗️ C4 Level 3: Component Diagrams

Shows the internal components within each container. Components are groupings of related functionality behind a well-defined interface.

### crud-fs-ts-nextjs Components (Next.js Fullstack App)

`crud-fs-ts-nextjs` is a Next.js 16 fullstack application using the App Router with tRPC for type-safe API routes. It serves as a demo educational platform with bilingual support (default English). See the [Applications inventory](./applications.md) for full details.

### rhino-cli-go Components (Go CLI Tool)

**Component Responsibilities:**

- **Root Command**: CLI entry point, command routing, help text
- **Links Check Command**: Validate internal links in crud-fs-ts-nextjs content

### rhino-cli-rust Components (Rust CLI Tool)

```mermaid
graph TB
    subgraph "CLI Interface"
        RHINO_ROOT[Root Command<br/>Repository automation]
        RHINO_FLAGS[Flags Parser<br/>Command-line arguments]
    end

    subgraph "Automation Modules"
        AUTO_MODULE[Automation Module<br/>Extensible automation]
    end

    subgraph "Infrastructure"
        RHINO_CONFIG[Config Loader<br/>Configuration]
        RHINO_LOGGER[Logger<br/>Logging]
    end

    RHINO_ROOT --> AUTO_MODULE
    RHINO_ROOT --> RHINO_FLAGS
    AUTO_MODULE --> RHINO_CONFIG
    AUTO_MODULE --> RHINO_LOGGER

    style RHINO_ROOT fill:#0077b6,stroke:#03045e,color:#ffffff
    style AUTO_MODULE fill:#2a9d8f,stroke:#264653,color:#ffffff
```

**Component Responsibilities:**

- **Root Command**: CLI entry point for repository automation tasks
- **Automation Module**: Extensible module system for automation workflows
- **Config Loader**: Load butler-specific configuration

### crud-fs-ts-nextjs Components (Next.js Fullstack Platform)

**Component Responsibilities:**

- **Next.js App Router**: Static generation and routing for educational content
- **tRPC API**: Backend API for content retrieval, search, and navigation
- **Bilingual Support**: Default English with Indonesian content

## 📋 C4 Level 4: Code Architecture

Shows implementation details for critical components. Focus on Go CLI tool package structures and key implementation patterns.

### rhino-cli-go Package Structure (Go)

rhino-cli-go now provides only `links check` for validating internal links in crud-fs-ts-nextjs content. Title update and navigation regeneration commands are not applicable to Next.js apps.
