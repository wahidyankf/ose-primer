# CLAUDE.md

Guidance for Claude Code (claude.ai/code) working with code in this repository.

## Project Overview

**ose-primer** — repository template for OSE-style polyglot Nx monorepos. Ship it as a cloneable or cherry-pickable source so new repos bootstrap with governance, agents, skills, demo scaffolding, and repo tooling already wired together.

**License**: MIT across the entire repo.
**Main Branch**: `main` (Trunk Based Development)

### Tech Stack

- **Node.js**: 24.13.1 (LTS, managed by Volta)
- **npm**: 11.10.1
- **Monorepo**: Nx workspace
- **Current Apps**:
  - `rhino-cli` — Go CLI for repository management (Repository Hygiene & INtegration Orchestrator; includes `java validate-annotations`, `test-coverage`, `spec-coverage`, `agents validate-naming`, `workflows validate-naming`, `env backup|restore`, and `doctor`).
  - `crud-be-golang-gin` — Go/Gin REST API backend (default backend).
  - `crud-be-java-springboot` — Spring Boot REST API (alternative).
  - `crud-be-elixir-phoenix` — Elixir/Phoenix REST API (alternative).
  - `crud-be-fsharp-giraffe` — F#/Giraffe REST API (alternative).
  - `crud-be-python-fastapi` — Python/FastAPI REST API (alternative).
  - `crud-be-rust-axum` — Rust/Axum REST API (alternative).
  - `crud-be-kotlin-ktor` — Kotlin/Ktor REST API (alternative).
  - `crud-be-java-vertx` — Java/Vert.x REST API (alternative).
  - `crud-be-ts-effect` — TypeScript/Effect REST API (alternative).
  - `crud-be-csharp-aspnetcore` — C#/ASP.NET Core REST API (alternative).
  - `crud-be-clojure-pedestal` — Clojure/Pedestal REST API (alternative).
  - `crud-contracts` — OpenAPI 3.1 API contract spec at `specs/apps/crud/contracts/`; generates types + encoders/decoders for all demo apps via `codegen` Nx target.
  - `crud-be-e2e` — Playwright E2E tests for crud-be backends.
  - `crud-fe-ts-nextjs` — Next.js 16 frontend (TypeScript, App Router).
  - `crud-fe-ts-tanstack-start` — TanStack Start frontend (TypeScript, alternative).
  - `crud-fe-dart-flutterweb` — Flutter Web frontend (Dart, alternative).
  - `crud-fe-e2e` — Playwright E2E tests for crud-fe frontends.
  - `crud-fs-ts-nextjs` — Next.js 16 fullstack (TypeScript, App Router + Route Handlers).

## Project Structure

```
ose-primer/
├── apps/                      # Deployable applications (Nx)
│   ├── rhino-cli/             # Repository management CLI
│   ├── crud-be-*/           # 11 polyglot backend demos (Go, Java, Elixir, F#, Python, Rust, Kotlin, Java/Vertx, TS/Effect, C#, Clojure)
│   ├── crud-be-e2e/         # Playwright E2E for backends
│   ├── crud-fe-ts-nextjs/   # Next.js 16 frontend
│   ├── crud-fe-ts-tanstack-start/  # TanStack Start frontend
│   ├── crud-fe-dart-flutterweb/    # Flutter Web frontend
│   ├── crud-fe-e2e/         # Playwright E2E for frontends
│   └── crud-fs-ts-nextjs/   # Next.js 16 fullstack
├── apps-labs/                 # Experimental apps (NOT in Nx)
├── libs/                      # Reusable libraries (Nx, flat structure)
│   └── golang-commons/        # Shared Go utilities
├── specs/                     # Gherkin specs, OpenAPI contracts, C4 diagrams
│   ├── apps/demo/           # Demo app specs (be/, fe/, contracts/)
│   └── apps/rhino/            # rhino-cli specs
├── docs/                      # Documentation (Diátaxis framework)
│   ├── tutorials/             # Learning-oriented
│   ├── how-to/                # Problem-solving
│   ├── reference/             # Technical reference
│   └── explanation/           # Conceptual understanding
├── governance/                # Governance documentation
│   ├── conventions/           # Documentation standards
│   ├── development/           # Development practices
│   ├── principles/            # Core principles
│   ├── workflows/             # Multi-step processes
│   └── vision/                # Project vision
├── plans/                     # Project planning
│   ├── ideas.md
│   ├── in-progress/
│   ├── backlog/
│   └── done/
├── .claude/                   # Claude Code configuration
│   ├── agents/                # specialized AI agents
│   └── skills/                # skill packages
├── .opencode/                 # OpenCode mirror (auto-generated from .claude/)
├── .husky/                    # Git hooks
├── nx.json                    # Nx workspace config
└── package.json               # Volta pinning + npm workspaces
```

## Common Development Commands

```bash
# Install dependencies (automatically runs doctor to verify tool versions)
npm install

# Build/test/lint all projects
npm run build
npm run lint

# Specific project operations
nx build [project-name]
nx run [project-name]:test:quick
nx lint [project-name]
nx dev [project-name]

# Affected projects only (canonical target names)
nx affected -t build
nx affected -t test:quick
nx affected -t lint

# Three-level test targets
nx run [project-name]:test:unit          # Mocked dependencies, no Docker, cacheable
nx run [project-name]:test:integration   # Demo-be: real PostgreSQL via docker-compose; others: MSW/Godog. NOT cacheable by default
nx run [project-name]:test:e2e           # Real HTTP via Playwright. NOT cacheable

# Contract codegen (generates types from OpenAPI spec into generated-contracts/)
nx run crud-contracts:lint       # Lint + bundle the OpenAPI spec
nx run crud-contracts:docs       # Generate browsable API documentation
nx run [project-name]:codegen      # Generate types for a specific app
nx run-many -t codegen --projects=demo-*  # Generate for all demo apps

# Dependency graph
nx graph

# Markdown linting and formatting
npm run lint:md          # Lint all markdown files
npm run lint:md:fix      # Auto-fix markdown violations
npm run format:md        # Format markdown with Prettier
npm run format:md:check  # Check markdown formatting

# Verify local development environment
npm run doctor                    # Check all required tools
npm run doctor -- --fix           # Auto-install missing tools
npm run doctor -- --fix --dry-run # Preview what would be installed
npm run doctor -- --scope minimal # Check only core tools (git, volta, node, npm, go, docker, jq)
```

**Note on `npm install` + doctor**: `postinstall` hook runs `npm run doctor || true` — trailing `|| true` swallows doctor failures silently. `npm install` can complete while polyglot toolchain broken. For **worktree setup** (after `git worktree add`, `EnterWorktree`, or entering existing worktree session), run BOTH `npm install` AND `npm run doctor -- --fix` explicitly in root worktree, that order. Explicit `doctor --fix` only action guaranteeing 18+ polyglot toolchains (Go, Java, Rust, Elixir, Python, .NET, Dart, Clojure, Kotlin, C#, Node) converge. See [Worktree Toolchain Initialization](./governance/development/workflow/worktree-setup.md) for full rationale and procedure.

**See**: [governance/development/infra/nx-targets.md](./governance/development/infra/nx-targets.md) for canonical target names, mandatory targets per project type, and caching rules.

**Coverage thresholds** (all enforced via `rhino-cli test-coverage validate` in `test:quick`):

| Project(s)                                                             | Threshold | Report format                         | Notes                                                  |
| ---------------------------------------------------------------------- | --------- | ------------------------------------- | ------------------------------------------------------ |
| Go projects (`rhino-cli`, `libs/golang-commons`, `crud-be-golang-gin`) | ≥90%      | `cover.out` (go test)                 |                                                        |
| `crud-be-ts-effect`                                                    | ≥90%      | LCOV (Vitest)                         |                                                        |
| `crud-be-java-springboot`, `crud-be-java-vertx`                        | ≥90%      | JaCoCo XML                            |                                                        |
| `crud-be-kotlin-ktor`                                                  | ≥90%      | Kover JaCoCo XML                      |                                                        |
| `crud-be-python-fastapi`                                               | ≥90%      | LCOV (coverage.py)                    |                                                        |
| `crud-be-rust-axum`                                                    | ≥90%      | LCOV (cargo-llvm-cov)                 |                                                        |
| `crud-be-fsharp-giraffe`                                               | ≥90%      | AltCover LCOV (`altcov.info`)         | Uses `--linecover` to avoid F# `task{}` BRDA inflation |
| `crud-be-csharp-aspnetcore`                                            | ≥90%      | Coverlet LCOV (`coverage.info`)       |                                                        |
| `crud-be-clojure-pedestal`                                             | ≥90%      | cloverage LCOV (`--lcov`)             |                                                        |
| `crud-be-elixir-phoenix`                                               | ≥90%      | LCOV (ExCoveralls, `cover/lcov.info`) |                                                        |
| `crud-fs-ts-nextjs`                                                    | ≥75%      | LCOV (Vitest)                         |                                                        |
| `crud-fe-ts-nextjs`, `crud-fe-dart-flutterweb`                         | ≥70%      | LCOV                                  | fe threshold: API/auth layers fully mocked by design   |

**`test:integration` caching**: Default `cache: false` in `nx.json`. Demo-be backends use docker-compose with real PostgreSQL — non-deterministic, must never cache. Projects using in-process mocking only (MSW, Godog) override to `cache: true` in their `project.json`: Go CLI apps (Godog at both unit and integration levels), `golang-commons` (Godog + mock closures).

**Three-level testing standard** (crud-be backends):

1. **Unit (`test:unit`)**: All mocked deps; must consume Gherkin specs from `specs/apps/crud/be/gherkin/`; call service functions directly with mocked repos; coverage measured here (>=90%)
2. **Integration (`test:integration`)**: Real PostgreSQL via docker-compose; **no HTTP calls** (no MockMvc, TestClient, httptest, ConnTest, WebApplicationFactory, fetch, clj-http, Router.oneshot); must consume Gherkin specs; call service functions directly with real DB
3. **E2E (`test:e2e`)**: Full stack via Playwright; real HTTP + real DB; must consume Gherkin specs

All three levels consume same Gherkin specs — only step implementations change. `test:quick` includes only `test:unit` + coverage validation. Does NOT include `lint`, `typecheck`, `test:integration`, or `test:e2e`. `spec-coverage` (`rhino-cli spec-coverage validate`) runs as separate Nx target enforced by pre-push hook; active for crud-be backends and most other projects.

**Three-level testing standard** (Go CLI apps):

1. **Unit (`test:unit`)**: All mocked deps; consumes Gherkin specs from `specs/apps/<cli-name>/` via godog (no build tag); mocks all I/O via package-level function variables; coverage measured here (>=90%)
2. **Integration (`test:integration`)**: Real filesystem via `/tmp` fixtures; consumes same Gherkin specs via godog (`//go:build integration`); drives commands in-process via `cmd.RunE()`; cacheable
3. **E2E**: Not applicable for CLI apps

Both unit and integration levels consume same Gherkin specs — step implementations differ (mocked I/O vs real filesystem). `test:quick` includes `test:unit` (with godog BDD scenarios) + coverage validation.

**Mandatory Nx targets for demo apps**: All `crud-be-*` and `crud-fe-*` apps must have 7 targets: `codegen`, `typecheck`, `lint`, `build`, `test:unit`, `test:quick`, `test:integration`. Coverage thresholds: backends ≥90%, frontends ≥70%.

**Contract enforcement**: All demo apps have `codegen` Nx target generating types + encoders/decoders from OpenAPI spec at `specs/apps/crud/contracts/`. Generated code lives in `generated-contracts/` (gitignored). `codegen` target is dependency of `typecheck` and `build` — contract violations caught by `nx affected -t typecheck` and `test:quick` in pre-push hook and PR quality gate. (Exception: Rust and Flutter also declare `codegen` as dependency of `test:unit` due to generated code required at compile time.)

**See**: [governance/development/quality/three-level-testing-standard.md](./governance/development/quality/three-level-testing-standard.md)

## Markdown Quality

All markdown files auto-linted and formatted:

- **Prettier** (v3.6.2): Formatting (runs on pre-commit)
- **markdownlint-cli2** (v0.20.0): Linting (runs on pre-push)
- **Claude Code Hook**: Auto-formats and lints after Edit/Write operations (requires `jq`)

**Quick Fix**: If pre-push hook blocks push due to markdown violations:

```bash
npm run lint:md:fix
```

**See**: [governance/development/quality/markdown.md](./governance/development/quality/markdown.md)

## Monorepo Architecture

Uses **Nx** to manage apps and libs:

- **`apps/`** - Deployable apps (naming: `[domain]-[type]`)
  - Apps import libs but never export
  - Each app independently deployable
  - Apps never import other apps
- **`libs/`** - Reusable libraries (naming: `ts-[name]`, future: `java-*`, `py-*`)
  - Flat structure, no nesting
  - Import via `@open-sharia-enterprise/ts-[lib-name]`
  - Libs can import other libs (no circular deps)
- **`apps-labs/`** - Experimental apps outside Nx (framework evaluation, POCs)

**Nx Commands**:

```bash
nx dev [app-name]            # Start development server
nx build [app-name]          # Build specific project
nx affected -t build         # Build only affected projects
nx affected -t test:quick    # Run pre-push quality gate for affected projects
nx graph                     # Visualize dependencies
```

**See**: [docs/reference/monorepo-structure.md](./docs/reference/monorepo-structure.md), [docs/how-to/add-new-app.md](./docs/how-to/add-new-app.md), [governance/development/infra/nx-targets.md](./governance/development/infra/nx-targets.md)

## Git Workflow

**Trunk Based Development** — all development on `main`:

- **Default branch**: `main`
- **Commit format**: Conventional Commits `<type>(<scope>): <description>`
  - Types: feat, fix, docs, style, refactor, perf, test, chore, ci, revert
  - Scope optional but recommended
  - Imperative mood (e.g., "add" not "added")
  - No period at end
- **Split commits by domain**: Different types/domains/concerns = separate commits

Cloners who deploy to Vercel (or similar) typically mint their own `prod-*` env branches downstream; the template ships with none.

**See**: [governance/development/workflow/commit-messages.md](./governance/development/workflow/commit-messages.md)

**See**: [git-push-default convention](./governance/development/workflow/git-push-default.md) —
explicit opt-in-PR rule for plan agents.

## Git Hooks (Automated Quality)

Husky + lint-staged enforce quality:

- **Pre-commit**:
  - Validates `.claude/` and `.opencode/` config (if changed in staged files)
    - Validates `.claude/` source format (YAML, tools, model, skills)
    - Auto-syncs `.claude/` → `.opencode/`
    - Validates `.opencode/` output (semantic equivalence)
  - Formats staged files with Prettier (JS/TS/JSON/YAML/CSS/MD), gofmt (Go), and mix format (Elixir)
  - Validates markdown links in staged files
  - Validates all markdown files (markdownlint)
  - Auto-stages changes
- **Commit-msg**: Validates Conventional Commits format (Commitlint)
- **Pre-push**: Runs `typecheck`, `lint`, `test:quick`, and `spec-coverage` for affected projects (parallelism: cores-1)
  - Runs markdown linting
  - All four Nx targets cacheable — if pre-push times out, run `npx nx affected -t typecheck lint test:quick spec-coverage` first to warm cache, then push again

**See**: [governance/development/quality/code.md](./governance/development/quality/code.md)

## Documentation Organization

**Diátaxis Framework** - Four categories:

- **Tutorials** (`docs/tutorials/`) - Learning-oriented
- **How-to** (`docs/how-to/`) - Problem-solving
- **Reference** (`docs/reference/`) - Technical specs
- **Explanation** (`docs/explanation/`) - Conceptual understanding

**File Naming**: Lowercase kebab-case (standard markdown + GitHub compatibility)

**Examples**:

- `file-naming.md` (governance/conventions/structure)
- `getting-started.md` (tutorials)
- `deploy-docker.md` (how-to)

**Exception**: Index files use `README.md` for GitHub compatibility

**See**: [governance/conventions/structure/file-naming.md](./governance/conventions/structure/file-naming.md), [governance/conventions/structure/diataxis-framework.md](./governance/conventions/structure/diataxis-framework.md)

## Core Principles

All work follows foundational principles from `governance/principles/` (key ones below — see [Principles Index](./governance/principles/README.md) for complete list):

- **Deliberate Problem-Solving**: Understand before acting; prefer reversible decisions
- **Simplicity Over Complexity**: Minimum viable abstraction
- **Root Cause Orientation**: Fix root causes, not symptoms; minimal impact; senior engineer standard; proactively fix preexisting errors encountered during work (do not mention and defer)
- **Accessibility First**: WCAG AA compliance, color-blind friendly
- **Documentation First**: Documentation mandatory, not optional
- **No Time Estimates**: Never give time estimates; focus on outcomes
- **Progressive Disclosure**: Layer complexity; start simple
- **Automation Over Manual**: Automate repetitive tasks
- **Explicit Over Implicit**: Explicit config over magic
- **Immutability Over Mutability**: Prefer immutable data structures
- **Pure Functions Over Side Effects**: Functional core, imperative shell
- **Reproducibility First**: Deterministic builds and environments

**See**: [governance/principles/README.md](./governance/principles/README.md)

## Key Conventions

### File Naming

Lowercase kebab-case (`[a-z0-9-]+`) with standard extension; rule anchored on standard markdown and GitHub compatibility
Exception: `README.md` for index files, `docs/metadata/` files

**See**: [governance/conventions/structure/file-naming.md](./governance/conventions/structure/file-naming.md)

### Linking

GitHub-compatible markdown: `[Text](path.md)` with `.md` extension for internal references.

**See**: [governance/conventions/formatting/linking.md](./governance/conventions/formatting/linking.md)

### Indentation

Markdown nested bullets: 2 spaces per level
YAML frontmatter: 2 spaces
Code: language-specific

**See**: [governance/conventions/formatting/indentation.md](./governance/conventions/formatting/indentation.md)

### Emoji Usage

Allowed: `docs/`, README files, `plans/`, `governance/`, CLAUDE.md, `AGENTS.md`, `.claude/agents/`, `.opencode/agent/`, `.opencode/skill/`
Forbidden: config files (`*.json`, `*.yaml`, `*.toml`), source code
Tasteful usage: emojis must aid scannability (section markers, status indicators, plan checklist status) — at most ~1 per heading, ~1 per paragraph; no decorative emojis, no emoji-as-bullet, no emoji in every heading.

**See**: [governance/conventions/formatting/emoji.md](./governance/conventions/formatting/emoji.md)

### Diagrams

Mermaid diagrams with color-blind friendly palette, proper accessibility. Plans SHOULD include Mermaid diagrams when they clarify component interactions, sequence/flow between agents or systems, state transitions, or decision branches; text-only is fine for trivial plans. Palette and accessibility rules live in the `docs-creating-accessible-diagrams` skill — don't redefine them.

**See**: [governance/conventions/formatting/diagrams.md](./governance/conventions/formatting/diagrams.md), [governance/conventions/structure/plans.md](./governance/conventions/structure/plans.md)

### Content Quality

Active voice, single H1, proper heading nesting, alt text for images, WCAG AA color contrast

**See**: [governance/conventions/writing/quality.md](./governance/conventions/writing/quality.md)

### Dynamic Collection References

Never hardcode counts of dynamic collections (agents, skills, conventions, practices, principles, workflows) in docs. Reference collection by name and link.

**See**: [governance/conventions/writing/dynamic-collection-references.md](./governance/conventions/writing/dynamic-collection-references.md)

### No Date Metadata

Manual `created:`/`updated:` frontmatter and `**Last Updated**` rows are forbidden in
all markdown files. Use `git log --follow -1 --pretty=%ai <file>` for dates.

**See**: [governance/conventions/writing/no-date-metadata.md](./governance/conventions/writing/no-date-metadata.md)

## Development Practices

### Functional Programming

Prefer immutability, pure functions, functional core/imperative shell

**See**: [governance/development/pattern/functional-programming.md](./governance/development/pattern/functional-programming.md)

### Implementation Workflow

Make it work → Make it right → Make it fast

**See**: [governance/development/workflow/implementation.md](./governance/development/workflow/implementation.md)

### Reproducible Environments

Volta for Node.js/npm pinning, package-lock.json, .env.example

**See**: [governance/development/workflow/reproducible-environments.md](./governance/development/workflow/reproducible-environments.md)

### Agent Workflow Orchestration

Plan mode for non-trivial tasks (3+ steps or architecture decisions), subagents for focused subtasks, verify before done, autonomous bug fixing, self-improvement loop after corrections

**See**: [governance/development/agents/agent-workflow-orchestration.md](./governance/development/agents/agent-workflow-orchestration.md)

### Manual Verification & CI Blockers

- **Verify behavior**: Playwright MCP for UI, curl for API ([manual-behavioral-verification.md](./governance/development/quality/manual-behavioral-verification.md))
- **CI blockers**: Investigate root cause, fix properly, never bypass ([ci-blocker-resolution.md](./governance/development/quality/ci-blocker-resolution.md))

## AI Agents

**Content Creation**: docs-maker, docs-tutorial-maker, readme-maker, specs-maker, swe-ui-maker

**Validation**: docs-checker, docs-tutorial-checker, docs-link-checker, readme-checker, specs-checker, swe-code-checker, swe-ui-checker, ci-checker

**Fixing**: docs-fixer, docs-tutorial-fixer, readme-fixer, specs-fixer, docs-file-manager, swe-ui-fixer, ci-fixer

**Planning**: plan-maker, plan-checker, plan-execution-checker, plan-fixer (see [plan-execution workflow](./governance/workflows/plan/plan-execution.md))

**Development**: swe-elixir-dev, swe-golang-dev, swe-java-dev, swe-python-dev, swe-typescript-dev, swe-e2e-dev, swe-dart-dev, swe-kotlin-dev, swe-csharp-dev, swe-fsharp-dev, swe-clojure-dev, swe-rust-dev

**Research**: web-research-maker

**Meta** _(CLAUDE.md grouping — in [agents/README.md](./.claude/agents/README.md) distributed by role: Makers, Checkers, Fixers)_: agent-maker, repo-rules-maker, repo-rules-checker, repo-rules-fixer, repo-workflow-maker, repo-workflow-checker, repo-workflow-fixer, social-linkedin-post-maker

**Maker-Checker-Fixer Pattern**: Three-stage workflow with criticality levels (CRITICAL/HIGH/MEDIUM/LOW), confidence assessment (HIGH/MEDIUM/FALSE_POSITIVE)

**Web Research Default**: `web-research-maker` is the default primitive for public-web information gathering across all agents. See [Web Research Delegation Convention](./governance/conventions/writing/web-research-delegation.md) for the normative rule, delegation threshold (2+ `WebSearch` or 3+ `WebFetch` per claim), and enumerated exceptions (single-shot known URL; fixer re-validation; link-reachability checkers).

**Skills Infrastructure**: Agents leverage skills providing two modes:

- **Inline skills** (default) - Inject knowledge into current conversation
- **Fork skills** (`context: fork`) - Trigger subagent spawning, delegate tasks to isolated agent contexts, return summarized results

Skills serve agents with knowledge and execution services but don't govern them (service relationship, not governance).

### Working with .claude/ and .opencode/ Directories

Edit `.claude/` and `.opencode/` files with normal `Write` / `Edit` tools. Both paths pre-authorized in `.claude/settings.json` (`Write(.claude/**)`, `Edit(.claude/**)`, `Write(.opencode/**)`, `Edit(.opencode/**)`), no approval prompt fires. `Bash` heredoc and `sed` remain fine for bulk mechanical substitutions, but no rule against direct edits.

**Applies to all paths**:

- `.claude/agents/*.md` — agent definitions
- `.claude/skills/*/SKILL.md` — skill files
- `.claude/skills/*/reference/*.md` — skill reference modules
- `.opencode/agent/*.md` — OpenCode agent mirrors
- `.opencode/skill/*/SKILL.md` — OpenCode skill mirrors

**See**: [.claude/agents/README.md](./.claude/agents/README.md), [governance/development/pattern/maker-checker-fixer.md](./governance/development/pattern/maker-checker-fixer.md), [Agent Naming Convention](./governance/conventions/structure/agent-naming.md), [Workflow Naming Convention](./governance/conventions/structure/workflow-naming.md)

## Dual-Mode Configuration (Claude Code + OpenCode)

Repo maintains **dual compatibility** with Claude Code and OpenCode:

- **`.claude/`**: Source of truth (PRIMARY) - All updates happen here first
- **`.opencode/`**: Auto-generated (SECONDARY) - Synced from `.claude/`

**Making Changes:**

1. Edit agents/skills in `.claude/` first
2. Run sync: `npm run sync:claude-to-opencode`
3. Both systems stay synced automatically

**Format Differences:**

- **Tools**: Claude Code uses arrays `[Read, Write]`, OpenCode uses boolean flags `{ read: true, write: true }`
- **Models**: Claude Code uses `sonnet`/`opus`/`haiku` (or omits for budget-adaptive inheritance); OpenCode uses `zai-coding-plan/glm-5.1` (opus/sonnet/omitted) and `zai-coding-plan/glm-5-turbo` (haiku). See [model-selection.md](./governance/development/agents/model-selection.md) for full tier mapping.
- **Skills**: Folder structure maintained (`.claude/skills/{name}/SKILL.md` → `.opencode/skill/{name}/SKILL.md`)
- **Permissions**: Claude Code uses `settings.json` permissions, OpenCode uses `opencode.json` permission block (both configured with equivalent access)
- **MCP/Plugins**: Claude Code uses plugins (Context7, Playwright, Nx, LSPs), OpenCode uses MCP servers (Playwright, Nx, Z.ai, Perplexity)

**Security Policy**: Only use skills from trusted sources. All skills in this repo maintained by project team.

**See**: [.claude/agents/README.md](./.claude/agents/README.md), [AGENTS.md](./AGENTS.md) for OpenCode docs

## Repository Architecture

Six-layer governance hierarchy:

- **Layer 0: Vision** - WHY we exist (repository template enabling polyglot monorepo bootstrap)
- **Layer 1: Principles** - WHY we value approaches
- **Layer 2: Conventions** - WHAT documentation rules
- **Layer 3: Development** - HOW we develop
- **Layer 4: AI Agents** - WHO enforces rules
- **Layer 5: Workflows** - WHEN we run processes (orchestrated sequences)

**Skills**: Delivery infrastructure serving agents, two modes:

- **Inline skills** - Knowledge injection into current conversation
- **Fork skills** (`context: fork`) - Task delegation to agents in isolated contexts
- Service relationship: Skills serve agents but don't govern them

**See**: [governance/repository-governance-architecture.md](./governance/repository-governance-architecture.md)

## Temporary Files for AI Agents

AI agents use designated directories:

- **`generated-reports/`**: Validation/audit reports (Write + Bash tools required)
  - Pattern: `{agent-family}__{uuid-chain}__{YYYY-MM-DD--HH-MM}__audit.md`
  - Checkers MUST write progressive reports during execution
- **`local-temp/`**: Misc temporary files

**See**: [governance/development/infra/temporary-files.md](./governance/development/infra/temporary-files.md)

## Plans Organization

Project planning in `plans/` folder:

- **ideas.md**: 1-3 liner ideas
- **backlog/**: Future plans
- **in-progress/**: Active work
- **done/**: Completed plans

**Folder naming**: `YYYY-MM-DD__[project-identifier]/`

**Default plan layout**: **five documents** — `README.md` (overview + navigation),
`brd.md` (business rationale), `prd.md` (product requirements + Gherkin acceptance
criteria), `tech-docs.md` (how), `delivery.md` (step-by-step checklist with `- [ ]`
items). Plan may collapse to single `README.md` only when trivially small (all content
≤ 1000 lines and condensed BRD + condensed PRD fit comfortably). See [Plans Organization
Convention](./governance/conventions/structure/plans.md) for full rules.

**See**: [governance/conventions/structure/plans.md](./governance/conventions/structure/plans.md)

## Important Notes

- **Do NOT stage or commit** unless explicitly instructed. Per-request commits one-time only.
- **License**: MIT across the entire repo. See [LICENSE](./LICENSE) and [LICENSING-NOTICE.md](./LICENSING-NOTICE.md).
- **AI agent invocation**: Use natural language to invoke agents/workflows
- **Token budget**: Don't worry about token limits - reliable compaction available
- **No time estimates**: Never give time estimates. Focus on what needs doing, not how long.

## Related Documentation

- **Conventions Index**: [governance/conventions/README.md](./governance/conventions/README.md) - Documentation writing and org standards
- **Development Index**: [governance/development/README.md](./governance/development/README.md) - Software dev practices and workflows
- **Principles Index**: [governance/principles/README.md](./governance/principles/README.md) - Foundational values governing all layers
- **Agents Index**: [.claude/agents/README.md](./.claude/agents/README.md) - Specialized agents organized by role
- **Workflows Index**: [governance/workflows/README.md](./governance/workflows/README.md) - Orchestrated processes
- **Repository Architecture**: [governance/repository-governance-architecture.md](./governance/repository-governance-architecture.md) - Six-layer governance hierarchy

<!-- nx configuration start-->
<!-- Leave the start & end comments to automatically receive updates. -->

## General Guidelines for working with Nx

- For navigating/exploring workspace, invoke `nx-workspace` skill first - has patterns for querying projects, targets, and deps
- When running tasks (build, lint, test, e2e, etc.), prefer running through `nx` (`nx run`, `nx run-many`, `nx affected`) instead of underlying tooling directly
- Prefix nx commands with workspace package manager (e.g., `pnpm nx build`, `npm exec nx test`) - avoids using globally installed CLI
- You have access to Nx MCP server and its tools, use them
- For Nx plugin best practices, check `node_modules/@nx/<plugin>/PLUGIN.md`. Not all plugins have this file - proceed without it if unavailable.
- NEVER guess CLI flags - check nx_docs or `--help` first when unsure

## Scaffolding & Generators

- For scaffolding tasks (creating apps, libs, project structure, setup), ALWAYS invoke `nx-generate` skill FIRST before exploring or calling MCP tools

## When to use nx_docs

- USE for: advanced config options, unfamiliar flags, migration guides, plugin config, edge cases
- DON'T USE for: basic generator syntax (`nx g @nx/react:app`), standard commands, things you know
- `nx-generate` skill handles generator discovery internally - don't call nx_docs to look up generator syntax

<!-- nx configuration end-->

<!-- rtk-instructions v2 -->

# RTK (Rust Token Killer) - Token-Optimized Commands

## Golden Rule

**Always prefix commands with `rtk`**. If RTK has dedicated filter, uses it. If not, passes through unchanged. RTK always safe to use.

**Important**: Even in command chains with `&&`, use `rtk`:

```bash
# ❌ Wrong
git add . && git commit -m "msg" && git push

# ✅ Correct
rtk git add . && rtk git commit -m "msg" && rtk git push
```

## Meta Commands

```bash
rtk gain              # Show token savings analytics
rtk gain --history    # Show command usage history with savings
rtk discover          # Analyze Claude Code history for missed opportunities
rtk proxy <cmd>       # Execute raw command without filtering (for debugging)
```

Full command reference with all workflows and savings: <https://github.com/rtk-ai/rtk>

<!-- /rtk-instructions -->
