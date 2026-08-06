# AGENTS.md

> Canonical instruction file for any AI coding agent or human contributor working in this repo.
> Aligned with the [AGENTS.md standard](https://agents.md/) (Agentic AI Foundation / Linux Foundation).

**Problem**: Maintaining quality and consistency across many specialized agents, agent skills, and extensive documentation is time-consuming and error-prone when done manually.

**Solution**: Specialized AI (Artificial Intelligence) agents automate documentation, validation, content generation, and project planning — ensuring consistent quality, catching errors early, and freeing developers for high-value work.

---

Instructions for AI agents working with this repository.

## Project Overview

**ose-primer** — repository template for OSE-style polyglot Nx monorepos. Node.js-based, Nx workspace, MIT-licensed.

### Sibling repositories (no parent monorepo)

`ose-primer` is one of four independently cloned repositories in the OSE (Open Sharia Enterprise) family. Treat each as a standalone git repository — there is no umbrella workspace, and the previously-used `ose-projects` parent has been deleted. **"All of the OSE repos" means exactly these four**, `beaver-nest` included despite sitting outside the propagation chain.

- [`ose-public`](https://github.com/wahidyankf/ose-public) — public, MIT. Upstream platform monorepo.
- [`ose-primer`](https://github.com/wahidyankf/ose-primer) — public, MIT. This repo; the template.
- [`ose-private`](https://github.com/wahidyankf/ose-private) — private, proprietary. Unexposed surface.
- [`beaver-nest`](https://github.com/wahidyankf/beaver-nest) — public, MIT. Product on this ecosystem.

Propagation flows `ose-public → ose-primer → downstream forks` for governance, agents, and skills; infrastructure-only concerns flow `ose-public ↔ ose-private`. `beaver-nest` is a full family member sitting **outside** the propagation chain — it syncs nothing either way and is never a parity target.

`apps/rhino-cli` must be byte-identical (zero carve-outs) across the three sync-loop repos
(`ose-public`, `ose-primer`, `ose-private`), including its Gherkin behavior tree at
`specs/apps/rhino/behavior/rhino-cli/gherkin/**` (every `.feature` and `README.md`), per the
[SDLC Gate Standard](./docs/reference/sdlc-gate-standard.md#rhino-cli-byte-identity-boundary).
`beaver-nest` carries a fork of it, not bound by that rule.

**See**: [Repository Ecosystem Convention](./repo-governance/conventions/structure/repository-ecosystem.md) (canonical rules) and [Related Repositories reference](./docs/reference/related-repositories.md) (full catalogue).

- **Node.js**: 24.13.1 (LTS - Long-Term Support, managed by Volta)
- **npm**: 11.10.1
- **Monorepo**: Nx with `apps/` and `libs/` structure
- **Git Workflow**: Trunk Based Development (TBD). Every plan resolves to one of four **Delivery Modes** -- `worktree-to-pr` is the repo-wide default; the four-mode work-location/integration-target table is in the linked convention. `*-to-pr` modes run the
  **PR-Review Maker→Fixer Cycle** (default 3 sequential CI-gated cycles) before the merge. **`[AI]` merges by default** in every mode; a `[HUMAN]` merge gate applies only where a plan's own step says so explicitly, with identical preconditions -- only the actor differs. **The PR is the independent merge point** -- N parallel units become N independently reviewed and merged PRs, which is why `worktree-to-pr` is the default; each change-producing DAG leaf gets its own worktree and PR (strict 1-PR ↔ 1-worktree), dependent nodes staying one PR. **Phase 0 opens no PR under any mode** -- setup/baseline is not a delivery node, so it pushes no branch and merges nothing; **the earliest PR is Phase 1**, and Phase 0's evidence rides it (see [§Phase 0 Opens No PR](./repo-governance/conventions/structure/plans.md#phase-0-opens-no-pr--the-earliest-pr-is-phase-1-hard-rule)). **PRs open at delivery boundaries, not every phase** -- a PR covers a **delivery unit**, the contiguous phases ending where work becomes independently shippable, so a plan opens one once at the end or several times through; folding independent nodes together to cut PR count stays forbidden (see [§PRs Open at Delivery Boundaries](./repo-governance/conventions/structure/plans.md#prs-open-at-delivery-boundaries-not-every-phase-hard-rule)). A PR merges only when **all five hardened preconditions** (a)-(e) hold, and the review loop did not exit `escalated` -- see the [PR Merge Protocol](./repo-governance/development/workflow/pr-merge-protocol.md). See the [Trunk Based Development Convention](./repo-governance/development/workflow/trunk-based-development.md#default-delivery-mode-worktree-to-pr), the [Git Push Default Convention](./repo-governance/development/workflow/git-push-default.md), and the [Plans Organization Convention §Delivery Mode](./repo-governance/conventions/structure/plans.md#delivery-mode) for the full three-tier precedence (invocation argument > plan `## Delivery Mode` field > default) and mechanics.
- **Worktree path**: Default worktree location is `worktrees/<name>/` per the [Worktree Path Convention](./repo-governance/conventions/structure/worktree-path.md) — parallel-safe, gitignored, no override.
- **Worktree toolchain init**: After creating or entering a worktree, agents must run BOTH `npm install` AND `npm run doctor -- --fix` in the root repository worktree, in that order — `postinstall` runs `npm run doctor || true`, which silently tolerates drift, so the explicit `--fix` call is required to converge the 18+ polyglot toolchains (Go, Java, Rust, Elixir, Python, .NET, Dart, Clojure, Kotlin, C#, Node). See [Infra: Development Environment Setup](./repo-governance/workflows/infra/infra-development-environment-setup.md) for the one-shot bootstrap and [Worktree Toolchain Initialization](./repo-governance/development/workflow/worktree-setup.md) for full rationale.

## Dual-Binding Configuration

This repository maintains **dual compatibility** with two coding-agent platforms via separate binding directories. Per the [Governance Vendor Independence convention](./repo-governance/conventions/structure/governance-vendor-independence.md), platform-specific terminology lives under [Platform Binding Examples](#platform-binding-examples) at the bottom of this file.

- **Primary binding directory**: source of truth — edit here first
- **Secondary binding directory**: auto-generated — synced from primary

**Sync command**: `npm run generate:bindings`

**Format differences** (canonical):

- **Tools**: primary binding uses tool arrays; secondary binding uses boolean flag maps; the sync translates between them
- **Models**: primary Claude tiers; secondary `zai-coding-plan/glm-5.2`. See [model-selection.md](./repo-governance/development/agents/model-selection.md)
- **Agent skills**: same SKILL.md format; skills are read natively by the secondary binding from the primary binding directory — no mirror is written
- **Permissions**: each binding has its own permission file with equivalent access configured
- **Plugins/MCP**: each binding has its own extension format (plugins for one, MCP servers for the other)

## AI Agents

### Agent Organization

Specialized agents organized into families:

The **[agent catalog](./.claude/agents/README.md) is authoritative** — every agent is listed there by
family. Do not maintain a second roster here. Names follow `<domain>-<role>`:

1. **maker / checker / fixer triads** — docs (plus tutorial, link, file-manager, and
   software-engineering-separation variants), readme, specs, ci, `swe-{code,ui}`,
   `repo-{rules,workflow}`, and `repo-harness-compatibility` (internal cross-vendor parity in
   Phase 0, external harness-convention drift in Phase 1).
2. **`swe-*-dev`** — one implementer per supported language. **Meta** — `agent-maker`,
   `social-linkedin-post-maker`. **Research** — `web-researcher`.
3. **Project Planning** — `plan-{maker,checker,execution-checker,fixer}` and `repo-setup-manager`
   (Phase 0 setup and baseline). `plan-maker` grills the user before and after plan creation with
   2–4 concrete options per question, one marked recommended, per the
   [Grilling-With-Options Convention](./repo-governance/development/workflow/grilling-with-options.md);
   checklists begin at Phase 0, every checkbox carries an `[AI]`/`[HUMAN]` marker, and every phase
   closes with a `### Phase N Gate` plus a `> **Pause Safety**:` note. Execution is orchestrated by
   the calling context via the
   [plan-execution](./repo-governance/workflows/plan/plan-execution.md) and
   [plan-planning](./repo-governance/workflows/plan/plan-planning.md) workflows — no dedicated
   executor subagent.
4. **Testing** — `web-{exploratory,usability,design}-tester` (spec-aware correctness / spec-blind
   first-time-user usability / design-aware runtime fidelity, the counterpart to `swe-ui-checker`)
   plus `api-exploratory-tester` (live REST or GraphQL, HTTP/curl-driven, never a browser). All
   non-destructive, each with a selectable **`output-mode`**: `plan` (default — a new backlog plan
   folder), `delivery` (appends to an existing plan's `delivery.md`; the rule-15 retest mechanism),
   `local-temp` (a scratch `local-temp/<YYYY-MM-DD>__<slug>/findings.md`).
5. **PR Review Cycle** — `pr-review-scout-maker` classifies risk tier and assembles a shared brief;
   selected specialists fan out to `pr-review-synthesis-maker` (coordinator), which hands off to
   `pr-review-fixer`, for `*-to-pr` Delivery Mode plans. See
   [§Delivery Mode](./repo-governance/conventions/structure/plans.md#delivery-mode) and
   [PR Review Quality Gate](./repo-governance/workflows/pr/pr-review-quality-gate.md).

### Agent Format

Agent definition files use YAML frontmatter. The exact tool encoding differs between bindings; see the [Platform Binding Examples](#platform-binding-examples) section at the bottom for binding-specific YAML samples.

This format is auto-generated from the primary binding's array form (tool arrays → `permission` object; the older boolean-flags output is deprecated/legacy and no longer emitted) by the sync command.

## Maker-Checker-Fixer Pattern

Three-stage quality workflow:

1. **Maker** - Creates content (tools: read, write, edit, glob, grep)
2. **Checker** - Validates content, generates audit reports (tools: read, glob, grep, write for reports)
3. **Fixer** - Applies validated fixes (tools: read, edit, write, glob, grep)

**Criticality Levels**: CRITICAL, HIGH, MEDIUM, LOW
**Confidence Levels**: HIGH, MEDIUM, FALSE_POSITIVE

**See**: `.claude/skills/repo-applying-maker-checker-fixer/SKILL.md` (read natively by the secondary binding)

**Web Research Default**: `web-researcher` is the default primitive for public-web information gathering across all agents. See [Web Research Delegation Convention](./repo-governance/conventions/writing/web-research-delegation.md) for the normative rule, delegation threshold (2+ `WebSearch` or 3+ `WebFetch` per claim), and enumerated exceptions (single-shot known URL; fixer re-validation; link-reachability checkers).

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

- **Documentation**: `docs-applying-content-quality`, `docs-applying-diataxis-framework`, `docs-creating-accessible-diagrams`, `docs-creating-by-example-tutorials`, `docs-creating-in-the-field-tutorials`, `docs-validating-factual-accuracy`, `docs-validating-links`, `docs-validating-software-engineering-separation`
- **README**: `readme-writing-readme-files`
- **Planning**: `grill-me`, `plan-creating-project-plans`, `plan-writing-gherkin-criteria`
- **Agent Development**: `agent-developing-agents`
- **CI Standards**: `ci-standards`
- **Repository Patterns**: `repo-applying-maker-checker-fixer`, `repo-assessing-criticality-confidence`, `repo-defining-workflows`, `repo-generating-validation-reports`, `repo-understanding-repository-architecture`
- **Development Workflow**: `repo-practicing-trunk-based-development`, `swe-developing-applications-common`, `swe-developing-e2e-test-with-playwright`, `swe-developing-frontend-ui`
- **Programming Languages**: `swe-programming-clojure`, `swe-programming-csharp`, `swe-programming-dart`, `swe-programming-elixir`, `swe-programming-fsharp`, `swe-programming-golang`, `swe-programming-java`, `swe-programming-kotlin`, `swe-programming-python`, `swe-programming-rust`, `swe-programming-typescript`

**Service Relationship**: Agent skills serve agents with knowledge and execution but don't govern them (service infrastructure, not governance layer).

**Full agent-skill catalog**: See [`.claude/skills/README.md`](./.claude/skills/README.md) (read natively by the secondary binding)

## Security Policy

**Trusted Sources Only**: Only use agent skills from trusted repositories. All agent skills in this repository are maintained by the project team.

**Rationale**: Agent skills execute with agent permissions and can access repository content. Only load agent skills from verified sources.

**Environment File Guard**: AI agents MUST NOT read, write, edit, or commit real `.env*` files (`.env`, `.env.local`, `.env.production`, etc.). Only `.env.example` is permitted, plus non-dotfile course fixtures (`kata.env`, `app.env`) under an app's published `apps/<app>/content/**` tree. See [env-file-access convention](./repo-governance/conventions/security/env-file-access.md) for the full six-layer policy, script carve-out, content-fixture exclusion, and known gaps.

**No Secrets in Committed Files (iron rule)**: NEVER put system secrets — SSH keys, passwords, sensitive usernames, API keys, tokens, connection strings with real credentials, or similar — into ANY file committed to git, including plans (`plans/**`), docs, code, config, and commit messages. Git history is permanent; a pushed secret is a leaked secret. Put real secrets only in uncommitted files: `.env*` (except `.env.example`) or another gitignored location, and reference them by variable name. See [No Secrets in Committed Files convention](./repo-governance/conventions/security/no-secrets-in-committed-files.md) for the full rule, examples, and remediation.

## Cross-Language Lint Gates

Beyond markdown, the repo gates shell scripts, Dockerfiles, and GitHub Actions
workflows at a uniform **warning-and-above** threshold, enforced in both CI
(`.github/workflows/pr-quality-gate.yml`) and the local Husky hooks:

- **shellcheck** (`--severity=warning`, root `.shellcheckrc`) — all tracked `.sh` files (CI `shellcheck` job)
- **hadolint** (`--failure-threshold warning`, root `.hadolint.yaml`) — all Dockerfiles (CI `hadolint` job)
- **actionlint** — all `.github/workflows/*.yml` (CI `actionlint` job)

All three linters are installed by `npm run doctor -- --fix`. The CI jobs are named
after the tool they run (Invariant A in the parity checklist).

**See**: [Cross-Language Lint Strictness](./repo-governance/development/quality/cross-language-lint-strictness.md)

## Specs & Gherkin Completeness (Both Paths)

Code under `apps/`/`libs/` never lands without its companion `specs/` Gherkin. This binds **both** ways a behavior change arrives at `apps/`, `libs/`, or `specs/`:

- **Direct change (no plan doc)**: edit app/lib code and add/update the matching `specs/apps/**` or `specs/libs/**` Gherkin `.feature` files (plus contracts/tests/docs) in the **same commit or PR**. Enforced by the `specs:coverage` Nx target and `swe-code-checker` (Step 6.6).
- **Planned change (plan doc)**: any plan whose scope touches `apps/`, `libs/`, or `specs/` MUST carry explicit delivery-checklist steps that add/update the companion Gherkin and run `specs:coverage`. `plan-maker` emits them; `plan-checker` (Step 5j) flags their absence.

Pure refactors that preserve behavior, dependency bumps with no behavior change, and docs/governance-only changes are exempt.

**See**: [feature-change-completeness.md](./repo-governance/development/quality/feature-change-completeness.md)

## Regression Test Mandate (Every Bug Fix)

Every fix for a discovered bug or regression lands with a **reproducing test** (failing before the fix,
passing after) in the **same commit/PR**. This is **blocking with no exemption** — it applies to all
defect types including cosmetic/visual, though the test form adapts (Gherkin + consuming test for
behaviour; DOM/computed-style/component test for visual; string assertion for content/i18n). A fixed bug
must become impossible to silently reintroduce. Enforced by `swe-code-checker` (Step 6.7) and
`plan-checker` (Step 16b). This is the bug-driven dual of Specs & Gherkin Completeness above.

**See**: [regression-test-mandate.md](./repo-governance/development/quality/regression-test-mandate.md)

## Knowledge Capture

Every substantive plan ends its `delivery.md` with a Knowledge Capture phase: the plan's transient
`learnings.md` running log is triaged to durable homes (or discarded with a reason) before archival,
with an explicit "none" escape when nothing generalizable surfaced.

**See**: [knowledge-capture.md](./repo-governance/development/quality/knowledge-capture.md)

## rhino-cli Command Surface

All callers (hooks, CI workflows, `package.json` scripts) use the canonical
`{domain}:{work}` Nx target form or `rhino {group} {verb}` CLI form. The old
`validate:*` prefix is abolished.

**Enumerating the surface**: the two namespaces have separate live authorities — never
transcribe either into a table here, which drifts silently. CLI groups come from
`cargo run --quiet --manifest-path apps/rhino-cli/Cargo.toml -- --help`; Nx targets come from
`nx show project rhino-cli --json`.

**Reserved namespace**: `docs` is reserved — do not add targets under `docs:*`.

**Target naming rule**: governance/validation targets use `{domain}:{work}` where
`{work}` ends in `-validation` for pure checks or is a bare verb (`check`). Never
invent `validate:{thing}` prefixes.

**See**: [Nx Target Naming Convention](./repo-governance/development/infra/nx-target-naming.md),
[CI/CD Conventions](./repo-governance/development/infra/ci-conventions.md)

## Manual Verification & CI Blockers

- **Verify behavior**: Browser-facing work first discovers a healthy installed integration, preferring
  Chrome/Chromium through Chrome DevTools MCP or Playwright MCP, with an equivalent browser-driving
  tool as fallback. Record the tool, fallback, and capability gaps; static inspection cannot replace a
  working browser integration. Use curl for API-only surfaces
  ([manual-behavioral-verification.md](./repo-governance/development/quality/manual-behavioral-verification.md)).
- **User-facing delivery hardening**: For any user-facing change, follow the sixteen rules — visual-parity sign-off against the design mockups per breakpoint/locale **before archival**, name the design-system primitive, per-breakpoint responsive deliverables, value-bearing tests, mockup-colors-as-theme-tokens, deploy-config-is-code, checkbox lockstep, and — for web-UI feature-change plans — a near-end three-tester retest round (the `web-ux-test-fixing-planning` workflow: `web-exploratory-tester` + `web-usability-tester` + `web-design-tester`) invoked with **`output-mode: delivery`** and the plan's **`plan-path`** so EWT/UWT/DWT findings are appended in-place to `delivery.md` as unchecked task-list items and fixed before archival; and — for API feature-change plans (REST/GraphQL) — a near-end `api-exploratory-tester` retest round (`output-mode: delivery`, the plan's `plan-path`) whose AET findings are appended to `delivery.md` and fixed before archival, exactly as the web-triad findings are (Rule 16) ([user-facing-delivery-hardening.md](./repo-governance/development/quality/user-facing-delivery-hardening.md))
- **CI blockers**: Investigate root cause, fix properly, never bypass ([ci-blocker-resolution.md](./repo-governance/development/quality/ci-blocker-resolution.md))
- **Build-artifact sweeper**: An ambient sweeper deletes gitignored build output/caches at any time, mid-plan. Regenerate (`nx build`, `npm run doctor -- --fix`) and continue — never file a finding or blame a concurrent agent; it never touches tracked files ([build-artifact-sweeper.md](./repo-governance/development/infra/build-artifact-sweeper.md))
- **CI post-push verification**: After pushing app or lib code, trigger and verify relevant GitHub CI workflows pass before declaring work done — pre-push hook alone is not sufficient ([ci-post-push-verification.md](./repo-governance/development/workflow/ci-post-push-verification.md))

## Git Hooks (Automated Quality)

The three Husky files are registry shims (`rhino-cli gate list/run/validate`). `repo-config.yml`'s
`gates:` registry is authoritative — never hand-maintain hook or CI command lists. `main-ci.yml` is
deleted; never trigger, monitor, or gate plan work on it.

**See**: [SDLC Gate Standard](./docs/reference/sdlc-gate-standard.md)

## Agent Workflow Orchestration

Plan mode for non-trivial tasks (3+ steps or architecture decisions), delegated agents for focused subtasks, verify before done, autonomous bug fixing, self-improvement loop after corrections.

**Parallel-by-default**: When work has independent sub-units (multiple reads/edits, searches, or delegated agents), run them **in parallel**, not serially, under the **N+1 model** — `1 main thread + N background agents = N+1 total`, **default N=3** (4 total) — the deliberate optimum bounding compute-budget burn while delivering real speedup. Raise N per-plan only when independent work, machine capacity, and budget headroom all allow; lower it under pressure; never self-promote beyond the declared N. Dependent steps stay sequential.

**Subagent concurrency**: When spawning background subagents via the Agent tool, N is the background-agent count; the main thread is the +1. Poll output file mtime every **3 minutes**; if mtime unchanged for 30 minutes, call `TaskStop` and relaunch.

**Same-machine assumption**: Assume other agents, engineers, and processes run simultaneously on the **same shared machine** — sharing its disk, git object store, worktrees, and CI runners — so every orchestration and git action must be safe under concurrent actors. Never run a destructive or irreversible local git operation that could discard another actor's uncommitted work.

**File-touch ledger**: those other actors edit constantly — in worktrees, on branches, and on local `main` — so keep a deliberate, append-only record of every file you touch, **reproduce it in full through every compaction, summary, and handoff**, and reconcile it against `git status` before staging. `git status` is the union of everyone's work, never a report of yours. Anything not on your ledger is another actor's in-flight work: leave it untouched; without a ledger, assume **nothing** is yours. See [File-Touch Discipline](./repo-governance/development/practice/file-touch-discipline.md).

**Harness mirrors are generated, not hand-written**: `.claude/` is the only hand-authored surface; `.opencode/`, `.cursor/`, and `.amazonq/` are emitted by `rhino-cli harness bindings generate` (`npm run generate:bindings`, also run and auto-staged by pre-commit). Those mirrors are files you touched — they go on your ledger and into the **same commit** as their source, never a follow-up sync commit. Verify with `npm run validate:sync`; never hand-edit a mirror.

**DAG-first**: Every non-trivial task list and delivery checklist declares a dependency DAG (`blocks`/`blockedBy`); independent nodes fan out up to N, dependent nodes serialize, cleanup is the terminal node. DAG width is the fan-out — N only caps it; sequence is not dependency.

**Background-slot preference**: Fill background slots up to N, keeping the main thread vacant and responsive — never split dependent work merely to fill a slot. Harnesses without background subagents degrade to a serial DAG walk.

**Status cadence**: report every **5 min** (generic) or **3 min** (GitHub CI); mixed takes 3. Reporting only — poll floors unchanged.

**Task-list discipline**: For non-trivial multi-step work (3+ steps, or spanning multiple files/phases), maintain a live task list from the start (harness Task tool or a plan's delivery checklist) and keep it **continuously in sync** — mark in-progress before starting, completed right after verifying, and add discovered tasks on the spot. A stale list is a defect.

**See**: [agent-workflow-orchestration.md](./repo-governance/development/agents/agent-workflow-orchestration.md), [Subagent Orchestration Convention](./repo-governance/development/agents/subagent-orchestration.md), [Parallel-by-Default Practice](./repo-governance/development/practice/parallel-by-default.md), [Task List Discipline](./repo-governance/development/practice/task-list-discipline.md), [No Destructive Git Operations](./repo-governance/development/workflow/no-destructive-git-operations.md), [Worktree and Artifact Cleanup](./repo-governance/development/workflow/worktree-and-artifact-cleanup.md)

## Governance Alignment

All agents follow foundational principles:

1. **Deliberate Problem-Solving** - Think before coding; surface assumptions and tradeoffs rather than hiding confusion
2. **Documentation First** - Documentation is mandatory, not optional
3. **Accessibility First** - WCAG AA (Web Content Accessibility Guidelines Level AA) compliance
4. **Simplicity Over Complexity** - Minimum viable abstraction
5. **Explicit Over Implicit** - Clear tool permissions
6. **Automation Over Manual** - Automate repetitive tasks
7. **Root Cause Orientation** - Fix root causes, not symptoms; minimal impact; senior engineer standard

**See**: [Principles README.md](./repo-governance/principles/README.md)

## Related Documentation

- **CLAUDE.md** - thin shim importing this canonical file via `@AGENTS.md`; documents primary-binding-specific notes
- **Primary-binding agent catalog** - `[primary binding]/agents/README.md` (canonical; synced to the secondary binding directory)
- **Primary-binding agent-skill catalog** - `[primary binding]/skills/README.md` (read natively by the secondary binding)
- **repo-governance/repository-governance-architecture.md** - Six-layer governance hierarchy
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

This section documents binding-specific details. Per the [Governance Vendor Independence convention](./repo-governance/conventions/structure/governance-vendor-independence.md), the vendor-audit scanner skips every line under this heading until the next same-level heading or end of file.

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
- Agent files: `.opencode/agents/*.md` with `permission` frontmatter and `zai-coding-plan/glm-5.2` model IDs
- Skills: NOT mirrored — OpenCode reads `.claude/skills/{name}/SKILL.md` natively per opencode.ai/docs/skills/
- Permission scheme: `.opencode/opencode.json`
- MCP servers (Playwright, Nx, Perplexity)

```binding-example
---
description: Brief description of what the agent does
model: zai-coding-plan/glm-5.2
permission:
  read: allow
  write: allow
  edit: allow
  glob: allow
  grep: allow
---
```
