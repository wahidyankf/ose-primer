# Infrastructure Workflows

Infrastructure-related workflows for environment setup, toolchain initialization, and operational procedures.

## Workflows

- [Infra: Development Environment Setup](./infra-development-environment-setup.md) — One-shot bootstrap that installs and verifies the full polyglot toolchain (Node.js, Go, Java, Rust, Elixir, Python, .NET, Dart, Clojure, Kotlin, C#) for pre-commit, pre-push, integration, and E2E tests. Documents the `OPENCODE_GO_API_KEY` env var needed for secondary-binding sessions.

## Related Documentation

- [Workflows Index](../README.md) — All orchestrated workflows
- [Workflow Naming Convention](../../conventions/structure/workflow-naming.md) — Filename rule applied to this directory (`infra-` scope prefix mandatory)
- [Worktree Toolchain Initialization](../../development/workflow/worktree-setup.md) — Mandatory `npm install` + `npm run doctor -- --fix` after entering a fresh worktree
