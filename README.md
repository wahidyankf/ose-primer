# ose-primer

Repository template for OSE-style polyglot Nx monorepos. Clone it whole or cherry-pick the parts you need to bootstrap a new repo that ships with governance, AI agents, skills, polyglot demo apps, and shared repo tooling already wired together.

## 🎯 What this is

`ose-primer` is a **clean, opinionated starting point** — not a product. It is everything a new OSE-style monorepo needs on day one: Nx workspace wiring, a Husky + lint-staged + commitlint pre-commit/pre-push stack, markdown tooling, doctor-based polyglot toolchain convergence, the `rhino-cli` repo-management CLI, the Diátaxis governance tree, and a ready-to-run three-level testing standard demonstrated across eleven backend and three frontend stacks.

Use it by forking, cloning, or copying the directories that fit your project — the template itself is intentionally minimal and **MIT-licensed** so you can relicense freely downstream.

## What it ships

- **Polyglot `crud-*` scaffolding** — 11 backend demos (Go, Java/Spring, Elixir/Phoenix, F#/Giraffe, Python/FastAPI, Rust/Axum, Kotlin/Ktor, Java/Vert.x, TypeScript/Effect, C#/ASP.NET, Clojure/Pedestal), 3 frontends (Next.js, TanStack Start, Flutter Web), one fullstack (Next.js), 2 E2E harnesses, and a shared OpenAPI contract (`crud-contracts`) that drives codegen across all of them.
- **`rhino-cli`** — Rust CLI for repository hygiene: `doctor`, `test-coverage`, `spec-coverage`, `agents validate-naming`, `workflows validate-naming`, `env backup|restore`, and more. `apps/rhino-cli/` is invoked by CI and all toolchain scripts, and consumes the behavior contract in `specs/apps/rhino/`.
- **Shared libs** — `golang-commons` and small TypeScript utilities.
- **Governance** — six-layer hierarchy (Vision → Principles → Conventions → Development → Agents → Workflows) under `repo-governance/`.
- **Generic AI agents + skills** — Maker/Checker/Fixer pattern for plans, repo rules, workflows, UI, code, docs, CI; plus language-specific development agents (`swe-*-dev`). No product-specific agents.
- **Dual-mode configuration** — `.claude/` (source of truth) auto-synced to `.opencode/`.

## How to use this template

1. **Clone or fork**: `git clone git@github.com:wahidyankf/ose-primer.git my-new-repo && cd my-new-repo`.
2. **Bootstrap the toolchain**: `npm install && npm run doctor -- --fix`. This pins Node via Volta, installs npm workspaces, and converges 18+ polyglot toolchains (Go, Java, Rust, Elixir, Python, .NET, Dart, Clojure, Kotlin, C#, Node).
3. **Keep what you need, delete what you don't** — every `crud-*` variant is independently deletable with a single `git rm -r apps/<name>` (plus its `specs/apps/crud/behavior/be/gherkin/<name>/` entries, if present). The `rhino-cli`, `repo-governance/`, `docs/`, `.claude/`, `.opencode/`, and `plans/` trees are expected to survive; the rest is opt-in.
4. **Rename to your project** — search-and-replace `ose-primer` across the repo, point `origin` at your new remote, and push to `main`.
5. **Start your own plans** — drop quick ideas into `plans/ideas.md` and promote mature ones to a `plans/backlog/YYYY-MM-DD__[identifier]/` folder following the five-document convention.

The template practices **Trunk Based Development**: one branch (`main`), small commits, Husky-enforced quality gates. No PRs within the template itself — downstream forks decide their own branching and deployment policy.

## Prerequisites

- **Node.js 24.13.1** + **npm 11.10.1** via [Volta](https://docs.volta.sh/guide/getting-started).
- Everything else (Go, Java, Python, Rust, Elixir, Kotlin, C#, Clojure, Dart, Docker, jq, Playwright) is auto-installed by `npm run doctor -- --fix`.

## 🛠️ Common commands

```bash
npm install                      # Install deps + set up Husky hooks
npm run doctor                   # Check polyglot toolchain
npm run doctor -- --fix          # Auto-install missing tools

npm run lint:md                  # Lint all markdown
npm run lint:md:fix              # Auto-fix markdown violations

nx dev [app-name]                # Start a dev server
nx build [app-name]              # Build one project
nx affected -t typecheck lint test:quick spec-coverage  # Pre-push gate
nx run-many -t typecheck lint test:quick spec-coverage  # Full workspace gate
nx graph                         # Visualise dependencies

npm run generate:bindings  # Regenerate .opencode/ from .claude/
npm run validate:claude          # Lint .claude/ source format
npm run validate:opencode        # Lint .opencode/ output format
```

See [CLAUDE.md](./CLAUDE.md) for the full command + convention reference tailored for AI-assisted sessions.

## 📚 Governance & conventions

The `repo-governance/` tree is the rulebook:

- **[principles/](./repo-governance/principles/README.md)** — Root values (Simplicity Over Complexity, Root Cause Orientation, Reproducibility First, No Time Estimates, …).
- **[conventions/](./repo-governance/conventions/README.md)** — File naming, linking, indentation, emoji, diagrams, agent naming, workflow naming, plans.
- **[development/](./repo-governance/development/README.md)** — Three-level testing standard, Nx targets, code quality, commit messages, worktree setup.
- **[workflows/](./repo-governance/workflows/README.md)** — Orchestrated multi-agent processes (plan-quality-gate, plan-execution, repo-rules-quality-gate, docs-quality-gate, specs-quality-gate, ci-quality-gate).
- **[vision/](./repo-governance/vision/README.md)** — High-level purpose.
- **[repository-governance-architecture.md](./repo-governance/repository-governance-architecture.md)** — How the six layers compose.

Agents live under `.claude/agents/` (source of truth) and `.opencode/agents/` (mirror). Skills live under `.claude/skills/` and `.opencode/skills/`. See [.claude/agents/README.md](./.claude/agents/README.md).

## 📂 Repository layout

```
ose-primer/
├── apps/                      # Deployable applications (Nx)
│   ├── rhino-cli/        # Rust — repository-management CLI
│   ├── crud-be-*/           # 11 polyglot backend demos
│   ├── crud-be-e2e/
│   ├── crud-fe-*/           # 3 frontend variants
│   ├── crud-fe-e2e/
│   └── crud-fs-ts-nextjs/   # Fullstack demo
├── apps-labs/                 # Experimental apps (not in Nx)
├── libs/                      # Shared libraries (flat)
├── specs/                     # Gherkin, OpenAPI contracts, C4
├── docs/                      # Diátaxis docs (tutorials/how-to/reference/explanation)
├── repo-governance/                # Principles, conventions, development, workflows, vision
├── plans/                     # ideas.md, backlog/, in-progress/, done/
├── .claude/                   # Claude Code agents, skills, settings
├── .opencode/                 # OpenCode mirror (auto-generated)
├── .husky/                    # Git hooks
├── infra/                     # docker-compose infra for demo backends
├── nx.json                    # Nx workspace config
├── tsconfig.base.json         # Base TS config
├── CLAUDE.md                  # Full repo guidance for Claude Code sessions
└── AGENTS.md                  # OpenCode equivalent of CLAUDE.md
```

## Related repositories

`ose-primer` is one of three sibling repositories in the OSE (Open Sharia Enterprise) family. Each repo stands alone — there is no parent monorepo — but governance, conventions, and tooling are kept aligned across them via explicit cross-repo propagation.

| Repository                                                           | Role                                                                                               | Visibility | License          |
| -------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | ---------- | ---------------- |
| [`ose-public`](https://github.com/wahidyankf/ose-public)             | Main OSE platform monorepo. Source of governance, conventions, and AI agent patterns adopted here. | Public     | Source-available |
| [`ose-primer`](https://github.com/wahidyankf/ose-primer) (this repo) | Repository template — clean MIT-licensed starting point for new OSE-style polyglot Nx monorepos.   | Public     | MIT              |
| [`ose-infra`](https://github.com/wahidyankf/ose-infra)               | Private infrastructure (Terraform, deploy pipelines, cloud config) backing `ose-public`.           | Private    | Proprietary      |

**Propagation direction**: governance, conventions, agents, and skills generally flow `ose-public → ose-primer → downstream forks`. Infrastructure-only concerns flow `ose-public ↔ ose-infra`. Each repo is independently clonable; no submodules, no workspace links.

See the [Repository Ecosystem Convention](./repo-governance/conventions/structure/repository-ecosystem.md) for the canonical description of the sibling relationship and propagation rules.

## License

**MIT** across the entire repo. See [LICENSE](./LICENSE) and [LICENSING-NOTICE.md](./LICENSING-NOTICE.md).

MIT is the lowest-friction choice for a template: downstream cloners can relicense freely without encountering FSL or other delayed-open-source constraints they did not choose.
