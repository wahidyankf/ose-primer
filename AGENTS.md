# AGENTS.md

> Canonical instruction file for any AI coding agent or human contributor working in this repo.
> Aligned with the [AGENTS.md standard](https://agents.md/) (Agentic AI Foundation / Linux Foundation).

**Problem**: Maintaining quality and consistency across many specialized agents, agent skills, and extensive documentation is time-consuming and error-prone when done manually.

**Solution**: This repository uses specialized AI (Artificial Intelligence) agents that automate documentation creation, validation, content generation, and project planning—ensuring consistent quality, catching errors early, and freeing developers to focus on high-value work.

---

Instructions for AI agents working with this repository.

## Project Overview

**ose-primer** — repository template for OSE-style polyglot Nx monorepos. Node.js-based, Nx workspace, MIT-licensed.

### Sibling repositories (no parent monorepo)

`ose-primer` is one of three independently cloned repositories in the OSE (Open Sharia Enterprise) family. Agents should treat each as a standalone git repository — there is no umbrella workspace, and the previously-used `ose-projects` parent has been deleted.

| Repository                                                           | Role                                                                                              | Visibility | License           |
| -------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ---------- | ----------------- |
| [`ose-public`](https://github.com/wahidyankf/ose-public)             | Main OSE platform monorepo. Upstream source of governance, conventions, and AI agent patterns.    | Public     | Open source (MIT) |
| [`ose-primer`](https://github.com/wahidyankf/ose-primer) (this repo) | MIT-licensed template extracted from `ose-public`. Clean starting point for new OSE-style repos.  | Public     | MIT               |
| [`ose-infra`](https://github.com/wahidyankf/ose-infra)               | Private infrastructure (Terraform, deploy pipelines, cloud config) for the `ose-public` platform. | Private    | Proprietary       |

Cross-repo propagation flows `ose-public → ose-primer → downstream forks` for governance, agents, and skills; infrastructure-only concerns flow `ose-public ↔ ose-infra`. See the [Repository Ecosystem Convention](./repo-governance/conventions/structure/repository-ecosystem.md) for the canonical rules.

`apps/rhino-cli` is required to be byte-identical (zero carve-outs) across all three repos, including
its Gherkin behavior tree at `specs/apps/rhino/behavior/rhino-cli/gherkin/**` (every `.feature` file and
every `README.md`), per the
[SDLC Gate Standard](./docs/reference/sdlc-gate-standard.md#rhino-cli-byte-identity-boundary).

- **Node.js**: 24.13.1 (LTS - Long-Term Support, managed by Volta)
- **npm**: 11.10.1
- **Monorepo**: Nx with `apps/` and `libs/` structure
- **Git Workflow**: Trunk Based Development (TBD). Every plan resolves to one of four **Delivery Modes**: `worktree-to-pr` (worktree → draft PR -- **the repo-wide default**), `worktree-to-origin-main` (worktree → direct push), `main-to-origin-main` (primary checkout → direct push), `main-to-pr` (primary checkout → draft PR). `*-to-pr` modes run the **PR-Review Maker→Fixer Cycle** (fan-out → `pr-review-synthesis-maker` → `pr-review-fixer`, default 3 sequential CI-gated cycles) before the merge. **`[AI]` merges by default** in every mode; a `[HUMAN]` merge gate applies only where a plan's own step says so explicitly, with identical preconditions -- only the actor differs. **The PR is the independent merge point** -- N parallel units become N PRs that review, gate, and merge independently, which is why `worktree-to-pr` is the default; each DAG leaf producing changes gets its own worktree and PR (strict 1-PR ↔ 1-worktree), while genuinely dependent nodes stay one PR. **Phase 0 opens no PR under any mode** -- setup/baseline is not a delivery node, so it pushes no branch and merges nothing; **the earliest PR is Phase 1**, and Phase 0's evidence rides it (see [§Phase 0 Opens No PR](./repo-governance/conventions/structure/plans.md#phase-0-opens-no-pr--the-earliest-pr-is-phase-1-hard-rule)). A PR merges only when **all five hardened preconditions** hold -- review cycles complete and the loop not exited `escalated`; 0 CRITICAL + 0 HIGH outstanding; branch non-destructively up to date with `origin/main`; all quality gates green; tester gates run or exemption recorded. Normative lettering (a)-(e) in the [PR Merge Protocol](./repo-governance/development/workflow/pr-merge-protocol.md). See the [Trunk Based Development Convention](./repo-governance/development/workflow/trunk-based-development.md#default-delivery-mode-worktree-to-pr), the [Git Push Default Convention](./repo-governance/development/workflow/git-push-default.md), and the [Plans Organization Convention §Delivery Mode](./repo-governance/conventions/structure/plans.md#delivery-mode) for the full three-tier precedence (invocation argument > plan `## Delivery Mode` field > default) and mechanics.
- **Worktree path**: Default worktree location is `worktrees/<name>/` per the [Worktree Path Convention](./repo-governance/conventions/structure/worktree-path.md) — parallel-safe, gitignored, no override.
- **Worktree toolchain init**: After creating or entering a worktree, agents must run BOTH `npm install` AND `npm run doctor -- --fix` in the root repository worktree, in that order. See [Infra: Development Environment Setup](./repo-governance/workflows/infra/infra-development-environment-setup.md) for the full one-shot bootstrap (polyglot toolchain + `OPENCODE_GO_API_KEY` env var). The `package.json` `postinstall` hook runs `npm run doctor || true` which silently tolerates toolchain drift, so the explicit `doctor --fix` invocation is required to converge the 18+ polyglot toolchains (Go, Java, Rust, Elixir, Python, .NET, Dart, Clojure, Kotlin, C#, Node). See [Worktree Toolchain Initialization](./repo-governance/development/workflow/worktree-setup.md) for the full rationale and procedure.

## Dual-Binding Configuration

This repository maintains **dual compatibility** with two coding-agent platforms via separate binding directories. Per the [Governance Vendor Independence convention](./repo-governance/conventions/structure/governance-vendor-independence.md), platform-specific terminology lives under [Platform Binding Examples](#platform-binding-examples) at the bottom of this file.

- **Primary binding directory**: source of truth — edit here first
- **Secondary binding directory**: auto-generated — synced from primary

**Sync command**: `npm run generate:bindings`

**Format differences** (canonical):

- **Tools**: primary binding uses tool arrays; secondary binding uses boolean flag maps; the sync translates between them
- **Models**: primary binding uses Claude tier names (sonnet/opus/haiku, or omits for inheritance); secondary binding uses opencode-go model IDs. See [model-selection.md](./repo-governance/development/agents/model-selection.md) for full capability-tier mapping
- **Agent skills**: same SKILL.md format; skills are read natively by the secondary binding from the primary binding directory — no mirror is written
- **Permissions**: each binding has its own permission file with equivalent access configured
- **Plugins/MCP**: each binding has its own extension format (plugins for one, MCP servers for the other)

## AI Agents

### Agent Organization

Specialized agents organized into families:

1. **Documentation**: `docs-maker`, `docs-checker`, `docs-fixer`, `docs-tutorial-maker`, `docs-tutorial-checker`, `docs-tutorial-fixer`, `docs-link-checker`, `docs-file-manager`, `docs-software-engineering-separation-checker`, `docs-software-engineering-separation-fixer`
2. **README**: `readme-maker`, `readme-checker`, `readme-fixer`
3. **Project Planning**: `plan-maker` (mandates grilling before and after plan creation
   using 2–4 concrete options per question with a recommended option marked, per the
   [Grilling-With-Options Convention](./repo-governance/development/workflow/grilling-with-options.md);
   delivery checklists must begin with Phase 0; every checkbox carries an `[AI]`/`[HUMAN]`
   execution marker with a legend and handoff/resume signal for any `[HUMAN]` step; every phase
   closes with a `### Phase N Gate` and a `> **Pause Safety**:` note making it a natural pause),
   `plan-checker`, `plan-execution-checker`,
   `plan-fixer`, `repo-setup-manager` (executes Phase 0 environment setup and baseline in
   every plan) — plan execution is orchestrated directly by the calling context via the
   [plan-execution workflow](./repo-governance/workflows/plan/plan-execution.md) and the
   [plan-planning workflow](./repo-governance/workflows/plan/plan-planning.md);
   no dedicated executor subagent
4. **Software Engineering & Specialized**: `agent-maker`, `swe-code-checker`, `swe-ui-maker`, `swe-ui-checker`, `swe-ui-fixer`, `swe-clojure-dev`, `swe-csharp-dev`, `swe-dart-dev`, `swe-e2e-dev`, `swe-elixir-dev`, `swe-fsharp-dev`, `swe-golang-dev`, `swe-java-dev`, `swe-kotlin-dev`, `swe-python-dev`, `swe-rust-dev`, `swe-typescript-dev`, `social-linkedin-post-maker`
5. **Repository Governance**: `repo-rules-maker`, `repo-rules-checker`, `repo-rules-fixer`, `repo-workflow-maker`, `repo-workflow-checker`, `repo-workflow-fixer`
6. **Harness Compatibility**: `repo-harness-compatibility-checker`, `repo-harness-compatibility-fixer` — the single harness-compat pair covering internal cross-vendor parity invariants (Phase 0) and external harness-convention drift (Phase 1)
7. **Specs Validation**: `specs-maker`, `specs-checker`, `specs-fixer`
8. **CI/CD**: `ci-checker`, `ci-fixer`
9. **Testing**: `web-exploratory-tester` (spec-aware correctness), `web-usability-tester` (spec-blind first-time-user usability), `web-design-tester` (design-aware live mockup/token/design-system fidelity — runtime counterpart to `swe-ui-checker`) — the live-site advocate triad; plus `api-exploratory-tester` (spec-aware, contract-aware exploratory testing of a live REST or GraphQL API — the live-API counterpart; HTTP/curl-driven, never a browser) — all non-destructive; each supports a selectable **`output-mode`**: `plan` (default — files a new backlog plan folder), `delivery` (appends findings in-place to an existing plan's `delivery.md` given a `plan-path`; the rule-15 near-end retest mechanism), `local-temp` (writes a scratch `local-temp/<YYYY-MM-DD>__<slug>/findings.md` for immediate fixing)
10. **Research**: `web-researcher`
11. **PR Review Cycle**: fan-out (`pr-review-{architecture,logic,governance,security,integrity,performance,docs,instruction}-maker`)
    to `pr-review-synthesis-maker` (coordinator) to `pr-review-fixer` — GitHub-Reviews-API-driven review
    cycle for `*-to-pr` Delivery Mode plans (see
    [Plans Organization Convention §Delivery Mode](./repo-governance/conventions/structure/plans.md#delivery-mode),
    [PR Review Quality Gate workflow](./repo-governance/workflows/pr/pr-review-quality-gate.md), and
    [PR Reviewer-Discipline Convention](./repo-governance/development/quality/pr-review-disciplines.md))

**Full agent catalog**: See [`.claude/agents/README.md`](./.claude/agents/README.md) (canonical source synced to the secondary binding directory)

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

**Environment File Guard**: AI agents MUST NOT read, write, edit, or commit real `.env*` files (`.env`, `.env.local`, `.env.production`, etc.). Only `.env.example` is permitted. See [env-file-access convention](./repo-governance/conventions/security/env-file-access.md) for the full six-layer policy, script carve-out, and known gaps.

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

**See**: [repo-governance/development/quality/feature-change-completeness.md](./repo-governance/development/quality/feature-change-completeness.md)

## Regression Test Mandate (Every Bug Fix)

Every fix for a discovered bug or regression lands with a **reproducing test** (failing before the fix,
passing after) in the **same commit/PR**. This is **blocking with no exemption** — it applies to all
defect types including cosmetic/visual, though the test form adapts (Gherkin + consuming test for
behaviour; DOM/computed-style/component test for visual; string assertion for content/i18n). A fixed bug
must become impossible to silently reintroduce. Enforced by `swe-code-checker` (Step 6.7) and
`plan-checker` (Step 16b). This is the bug-driven dual of Specs & Gherkin Completeness above.

**See**: [repo-governance/development/quality/regression-test-mandate.md](./repo-governance/development/quality/regression-test-mandate.md)

## Knowledge Capture

Every substantive plan ends its `delivery.md` with a Knowledge Capture phase: the plan's transient
`learnings.md` running log is triaged to durable homes (or discarded with a reason) before archival,
with an explicit "none" escape when nothing generalizable surfaced.

**See**: [repo-governance/development/quality/knowledge-capture.md](./repo-governance/development/quality/knowledge-capture.md)

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

- **Verify behavior**: Playwright MCP for UI, curl for API ([manual-behavioral-verification.md](./repo-governance/development/quality/manual-behavioral-verification.md))
- **User-facing delivery hardening**: For any user-facing change, follow the sixteen rules — visual-parity sign-off against the design mockups per breakpoint/locale **before archival**, name the design-system primitive, per-breakpoint responsive deliverables, value-bearing tests, mockup-colors-as-theme-tokens, deploy-config-is-code, checkbox lockstep, and — for web-UI feature-change plans — a near-end three-tester retest round (the `web-ux-test-fixing-planning` workflow: `web-exploratory-tester` + `web-usability-tester` + `web-design-tester`) invoked with **`output-mode: delivery`** and the plan's **`plan-path`** so EWT/UWT/DWT findings are appended in-place to `delivery.md` as unchecked task-list items and fixed before archival; and — for API feature-change plans (REST/GraphQL) — a near-end `api-exploratory-tester` retest round (`output-mode: delivery`, the plan's `plan-path`) whose AET findings are appended to `delivery.md` and fixed before archival, exactly as the web-triad findings are (Rule 16) ([user-facing-delivery-hardening.md](./repo-governance/development/quality/user-facing-delivery-hardening.md))
- **CI blockers**: Investigate root cause, fix properly, never bypass ([ci-blocker-resolution.md](./repo-governance/development/quality/ci-blocker-resolution.md))
- **CI post-push verification**: After pushing app or lib code — to a PR branch under the default `worktree-to-pr`, or to `origin main` under the direct-push modes — trigger relevant GitHub CI workflows and verify they pass before declaring work done — pre-push hook alone is not sufficient ([ci-post-push-verification.md](./repo-governance/development/workflow/ci-post-push-verification.md))

## Agent Workflow Orchestration

Plan mode for non-trivial tasks (3+ steps or architecture decisions), delegated agents for focused subtasks, verify before done, autonomous bug fixing, self-improvement loop after corrections.

**Parallel-by-default**: When work has independent sub-units (multiple reads/edits, searches, or delegated agents), run them **in parallel**, not serially, under the **N+1 model** — `1 main thread + N background agents = N+1 total`, **default N=3** (4 total). N=3 is the deliberate optimum: it bounds token/compute-budget burn while still delivering real speedup. Raise N per-plan only when independent work, machine capacity, and budget headroom all allow; lower it under pressure; never self-promote beyond the declared N. Dependent steps stay sequential.

**Subagent concurrency**: When spawning background subagents via the Agent tool, N is the background-agent count; the main thread is the +1. Poll output file mtime every **3 minutes**; if mtime unchanged for 30 minutes, call `TaskStop` and relaunch.

**Same-machine assumption**: Always assume other agents, engineers, and processes run simultaneously on the **same shared machine** — sharing its disk, git object store, worktrees, and CI runners — so every orchestration and git action must be safe under concurrent actors. Never run a destructive or irreversible local git operation that could discard another actor's uncommitted work.

**DAG-first**: Every non-trivial task list and delivery checklist declares a dependency DAG (`blocks`/`blockedBy`); independent nodes fan out up to N, dependent nodes serialize, cleanup is the terminal node. DAG width is the fan-out — N only caps it. Sequence is not dependency.

**Background-slot preference**: Fill background slots up to N, keeping the main thread vacant and responsive (orchestrator, not worker) — never split dependent work merely to fill a slot. On harnesses without background subagents, degrade to a serial DAG walk.

**Status cadence**: Update the user every **3-5 minutes, not faster**, while items are active.

**Task-list discipline**: For any non-trivial multi-step work (3+ steps, or spanning multiple files/phases), maintain a live task list from the start (harness Task tool or a plan's delivery checklist) and keep it **continuously in sync** — mark a task in-progress before starting, completed right after verifying, and add discovered tasks on the spot. A stale list is a defect.

**See**: [repo-governance/development/agents/agent-workflow-orchestration.md](./repo-governance/development/agents/agent-workflow-orchestration.md), [Subagent Orchestration Convention](./repo-governance/development/agents/subagent-orchestration.md), [Parallel-by-Default Practice](./repo-governance/development/practice/parallel-by-default.md), [Task List Discipline](./repo-governance/development/practice/task-list-discipline.md), [No Destructive Git Operations](./repo-governance/development/workflow/no-destructive-git-operations.md), [Worktree and Artifact Cleanup](./repo-governance/development/workflow/worktree-and-artifact-cleanup.md)

## Governance Alignment

All agents follow foundational principles:

1. **Deliberate Problem-Solving** - Think before coding; surface assumptions and tradeoffs rather than hiding confusion
2. **Documentation First** - Documentation is mandatory, not optional
3. **Accessibility First** - WCAG AA (Web Content Accessibility Guidelines Level AA) compliance
4. **Simplicity Over Complexity** - Minimum viable abstraction
5. **Explicit Over Implicit** - Clear tool permissions
6. **Automation Over Manual** - Automate repetitive tasks
7. **Root Cause Orientation** - Fix root causes, not symptoms; minimal impact; senior engineer standard

**See**: [repo-governance/principles/README.md](./repo-governance/principles/README.md)

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
- Agent files: `.opencode/agents/*.md` with frontmatter using a `permission` object (the older boolean tool flags are deprecated/legacy and no longer emitted) and `opencode-go/*` model IDs
- Skills: NOT mirrored — OpenCode reads `.claude/skills/{name}/SKILL.md` natively per opencode.ai/docs/skills/
- Permission scheme: `.opencode/opencode.json`
- MCP servers (Playwright, Nx, Perplexity)

```binding-example
---
description: Brief description of what the agent does
model: opencode-go/glm-5.2
permission:
  read: allow
  write: allow
  edit: allow
  glob: allow
  grep: allow
---
```
