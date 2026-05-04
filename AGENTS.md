# AGENTS.md

> Canonical instruction file for any AI coding agent or human contributor working in this repo.
> Aligned with the [AGENTS.md standard](https://agents.md/) (Agentic AI Foundation / Linux Foundation).

**Problem**: Maintaining quality and consistency across many specialized agents, agent skills, and extensive documentation is time-consuming and error-prone when done manually.

**Solution**: This repository uses specialized AI (Artificial Intelligence) agents that automate documentation creation, validation, content generation, and project planning—ensuring consistent quality, catching errors early, and freeing developers to focus on high-value work.

---

Instructions for AI agents working with this repository.

## Project Overview

**ose-primer** — repository template for OSE-style polyglot Nx monorepos. Node.js-based, Nx workspace, MIT-licensed.

- **Node.js**: 24.13.1 (LTS - Long-Term Support, managed by Volta)
- **npm**: 11.10.1
- **Monorepo**: Nx with `apps/` and `libs/` structure
- **Git Workflow**: Trunk Based Development (default: commit and push directly to `main`). **Worktree work follows the same default**: any work performed inside a `git worktree add` path -- including agents using `isolation: "worktree"` and agents invoked inside an existing worktree session -- pushes directly to `main` via `git push origin HEAD:main`. The worktree branch is an isolation mechanism, not a feature branch. A draft PR (`gh pr create --draft --base main`) is created only when the user's prompt or the plan document explicitly requests one; when opened, it stays in draft status during iteration and is flipped to ready for review when the author decides the work is complete, which is when the [PR Merge Protocol](./governance/development/workflow/pr-merge-protocol.md) approval gate fires. See the [Trunk Based Development Convention](./governance/development/workflow/trunk-based-development.md#worktree-mode-direct-push-to-main-draft-pr-opt-in) and the [Git Push Default Convention](./governance/development/workflow/git-push-default.md#standard-6-worktree-branches-push-to-main-not-to-worktree-branch) for details.
- **Worktree toolchain init**: After creating or entering a worktree, agents must run BOTH `npm install` AND `npm run doctor -- --fix` in the root repository worktree, in that order. The `package.json` `postinstall` hook runs `npm run doctor || true` which silently tolerates toolchain drift, so the explicit `doctor --fix` invocation is required to converge the 18+ polyglot toolchains (Go, Java, Rust, Elixir, Python, .NET, Dart, Clojure, Kotlin, C#, Node). See [Worktree Toolchain Initialization](./governance/development/workflow/worktree-setup.md) for the full rationale and procedure.

## Dual-Binding Configuration

This repository maintains **dual compatibility** with two coding-agent platforms via separate binding directories. Per the [Governance Vendor Independence convention](./governance/conventions/structure/governance-vendor-independence.md), platform-specific terminology lives under [Platform Binding Examples](#platform-binding-examples) at the bottom of this file.

- **Primary binding directory**: source of truth — edit here first
- **Secondary binding directory**: auto-generated — synced from primary

**Sync command**: `npm run sync:claude-to-opencode`

**Format differences** (canonical):

- **Tools**: primary binding uses tool arrays; secondary binding uses boolean flag maps; the sync translates between them
- **Models**: primary binding uses Claude tier names (sonnet/opus/haiku, or omits for inheritance); secondary binding uses opencode-go model IDs. See [model-selection.md](./governance/development/agents/model-selection.md) for full capability-tier mapping
- **Agent skills**: same SKILL.md format; skills are read natively by the secondary binding from the primary binding directory — no mirror is written
- **Permissions**: each binding has its own permission file with equivalent access configured
- **Plugins/MCP**: each binding has its own extension format (plugins for one, MCP servers for the other)

## AI Agents

### Agent Organization

Specialized agents organized into families:

1. **Documentation**: `docs-maker`, `docs-checker`, `docs-fixer`, `docs-tutorial-maker`, `docs-tutorial-checker`, `docs-tutorial-fixer`, `docs-link-checker`, `docs-file-manager`
2. **README**: `readme-maker`, `readme-checker`, `readme-fixer`
3. **Project Planning**: `plan-maker`, `plan-checker`, `plan-execution-checker`, `plan-fixer` (plan execution itself is orchestrated directly by the calling context via the [plan-execution workflow](./governance/workflows/plan/plan-execution.md); no dedicated executor subagent)
4. **Software Engineering & Specialized**: `agent-maker`, `swe-code-checker`, `swe-ui-maker`, `swe-ui-checker`, `swe-ui-fixer`, `swe-clojure-dev`, `swe-csharp-dev`, `swe-dart-dev`, `swe-e2e-dev`, `swe-elixir-dev`, `swe-fsharp-dev`, `swe-golang-dev`, `swe-java-dev`, `swe-kotlin-dev`, `swe-python-dev`, `swe-rust-dev`, `swe-typescript-dev`, `social-linkedin-post-maker`
5. **Repository Governance**: `repo-rules-maker`, `repo-rules-checker`, `repo-rules-fixer`, `repo-workflow-maker`, `repo-workflow-checker`, `repo-workflow-fixer`
6. **Specs Validation**: `specs-maker`, `specs-checker`, `specs-fixer`
7. **CI/CD**: `ci-checker`, `ci-fixer`
8. **Research**: `web-research-maker`

**Full agent catalog**: See [`.claude/agents/README.md`](./.claude/agents/README.md) (canonical source synced to the secondary binding directory)

### Agent Format

Agent definition files use YAML frontmatter. The exact tool encoding differs between bindings; see the [Platform Binding Examples](#platform-binding-examples) section at the bottom for binding-specific YAML samples.

This format is auto-generated from the primary binding's array form (tool arrays → boolean flags) by the sync command.

## Maker-Checker-Fixer Pattern

Three-stage quality workflow:

1. **Maker** - Creates content (tools: read, write, edit, glob, grep)
2. **Checker** - Validates content, generates audit reports (tools: read, glob, grep, write for reports)
3. **Fixer** - Applies validated fixes (tools: read, edit, write, glob, grep)

**Criticality Levels**: CRITICAL, HIGH, MEDIUM, LOW
**Confidence Levels**: HIGH, MEDIUM, FALSE_POSITIVE

**See**: `.claude/skills/repo-applying-maker-checker-fixer/SKILL.md` (read natively by the secondary binding)

**Web Research Default**: `web-research-maker` is the default primitive for public-web information gathering across all agents. See [Web Research Delegation Convention](./governance/conventions/writing/web-research-delegation.md) for the normative rule, delegation threshold (2+ `WebSearch` or 3+ `WebFetch` per claim), and enumerated exceptions (single-shot known URL; fixer re-validation; link-reachability checkers).

## Agent-Skill Integration

**Agent-skill packages** serve agents through two modes:

**Inline agent skills** (default) - Knowledge injection:

- Progressive disclosure of conventions and standards
- Injected into current conversation context
- Examples: `docs-applying-content-quality`, `docs-applying-diataxis-framework`, `docs-creating-accessible-diagrams`

**Fork agent skills** (`context: fork`) - Task delegation:

- Spawn isolated agent contexts for focused work
- Delegate specialized tasks (research, analysis, exploration)
- Return summarized results to main conversation
- Act as lightweight orchestrators

**Categories** (representative examples — see full catalog below):

- **Documentation**: `docs-applying-content-quality`, `docs-applying-diataxis-framework`, `docs-creating-accessible-diagrams`, `docs-creating-by-example-tutorials`, `docs-creating-in-the-field-tutorials`
- **Planning**: `plan-creating-project-plans`, `plan-writing-gherkin-criteria`
- **Agent Development**: `agent-developing-agents`
- **Repository Patterns**: `repo-applying-maker-checker-fixer`, `repo-assessing-criticality-confidence`, `repo-generating-validation-reports`, `repo-understanding-repository-architecture`
- **Development Workflow**: `repo-practicing-trunk-based-development`, `swe-developing-applications-common`
- **Programming Languages**: `swe-programming-clojure`, `swe-programming-csharp`, `swe-programming-dart`, `swe-programming-elixir`, `swe-programming-fsharp`, `swe-programming-golang`, `swe-programming-java`, `swe-programming-kotlin`, `swe-programming-python`, `swe-programming-rust`, `swe-programming-typescript`

**Service Relationship**: Agent skills serve agents with knowledge and execution but don't govern them (service infrastructure, not governance layer).

**Full agent-skill catalog**: See [`.claude/skills/README.md`](./.claude/skills/README.md) (read natively by the secondary binding)

## Security Policy

**Trusted Sources Only**: Only use agent skills from trusted repositories. All agent skills in this repository are maintained by the project team.

**Rationale**: Agent skills execute with agent permissions and can access repository content. Only load agent skills from verified sources.

## Governance Alignment

All agents follow foundational principles:

1. **Deliberate Problem-Solving** - Think before coding; surface assumptions and tradeoffs rather than hiding confusion
2. **Documentation First** - Documentation is mandatory, not optional
3. **Accessibility First** - WCAG AA (Web Content Accessibility Guidelines Level AA) compliance
4. **Simplicity Over Complexity** - Minimum viable abstraction
5. **Explicit Over Implicit** - Clear tool permissions
6. **Automation Over Manual** - Automate repetitive tasks
7. **Root Cause Orientation** - Fix root causes, not symptoms; minimal impact; senior engineer standard

**See**: [governance/principles/README.md](./governance/principles/README.md)

## Related Documentation

- **CLAUDE.md** - thin shim importing this canonical file via `@AGENTS.md`; documents primary-binding-specific notes
- **Primary-binding agent catalog** - `[primary binding]/agents/README.md` (canonical; synced to the secondary binding directory)
- **Primary-binding agent-skill catalog** - `[primary binding]/skills/README.md` (read natively by the secondary binding)
- **governance/repository-governance-architecture.md** - Six-layer governance hierarchy
- **docs/reference/platform-bindings.md** - Catalog of platform-specific bindings and their conventions

---

<!-- nx configuration start-->
<!-- Leave the start & end comments to automatically receive updates. -->

## General Guidelines for working with Nx

- For navigating/exploring the workspace, invoke the `nx-workspace` agent skill first - it has patterns for querying projects, targets, and dependencies
- When running tasks (for example build, lint, test, e2e, etc.), always prefer running the task through `nx` (i.e. `nx run`, `nx run-many`, `nx affected`) instead of using the underlying tooling directly
- Prefix nx commands with the workspace's package manager (e.g., `pnpm nx build`, `npm exec nx test`) - avoids using globally installed CLI
- You have access to the Nx MCP server and its tools, use them to help the user
- For Nx plugin best practices, check `node_modules/@nx/<plugin>/PLUGIN.md`. Not all plugins have this file - proceed without it if unavailable.
- NEVER guess CLI flags - always check nx_docs or `--help` first when unsure

## Scaffolding & Generators

- For scaffolding tasks (creating apps, libs, project structure, setup), ALWAYS invoke the `nx-generate` agent skill FIRST before exploring or calling MCP tools

## When to use nx_docs

- USE for: advanced config options, unfamiliar flags, migration guides, plugin configuration, edge cases
- DON'T USE for: basic generator syntax (`nx g @nx/react:app`), standard commands, things you already know
- The `nx-generate` agent skill handles generator discovery internally - don't call nx_docs just to look up generator syntax

<!-- nx configuration end-->

## Platform Binding Examples

This section documents binding-specific details. Per the [Governance Vendor Independence convention](./governance/conventions/structure/governance-vendor-independence.md), the vendor-audit scanner skips every line under this heading until the next same-level heading or end of file.

### Primary binding: Claude Code (`.claude/`)

- Source-of-truth directory: `.claude/`
- Agent files: `.claude/agents/*.md` with frontmatter using array tools (e.g. `tools: [Read, Write]`) and Claude tier names (`sonnet` / `opus` / `haiku` / omitted)
- Skill files: `.claude/skills/*/SKILL.md` (read natively by both bindings)
- Permission scheme: `.claude/settings.json`

```binding-example
---
description: Brief description of what the agent does
model: sonnet
tools: [Read, Write, Edit, Glob, Grep]
---
```

### Secondary binding: OpenCode (`.opencode/`)

- Auto-generated directory: `.opencode/agents/` (plural per opencode.ai/docs/agents/)
- Agent files: `.opencode/agents/*.md` with frontmatter using boolean tool flags and `opencode-go/*` model IDs
- Skills: NOT mirrored — OpenCode reads `.claude/skills/{name}/SKILL.md` natively per opencode.ai/docs/skills/
- Permission scheme: `.opencode/opencode.json`
- MCP servers (Playwright, Nx, Perplexity)

```binding-example
---
description: Brief description of what the agent does
model: opencode-go/minimax-m2.7
tools:
  read: true
  write: true
  edit: true
  glob: true
  grep: true
---
```
