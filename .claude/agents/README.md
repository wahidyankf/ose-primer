# Claude Code Agents

This directory contains specialized AI agents for the ose-primer repository template. These agents are organized by role and follow the Maker-Checker-Fixer pattern where applicable.

## Agent Organization

### 🟦 Content Creation (Makers)

- **[docs-maker](docs-maker.md)** - Expert documentation writer
- **[docs-tutorial-maker](docs-tutorial-maker.md)** - Tutorial creation specialist
- **[readme-maker](readme-maker.md)** - README file writer
- **[plan-maker](plan-maker.md)** - Project plan creation
- **[repo-rules-maker](repo-rules-maker.md)** - Governance document creation
- **[repo-workflow-maker](repo-workflow-maker.md)** - Workflow documentation
- **[specs-maker](specs-maker.md)** - Spec area scaffolding and feature file creation
- **[social-linkedin-post-maker](social-linkedin-post-maker.md)** - LinkedIn content creation
- **[agent-maker](agent-maker.md)** - Agent definition creation
- **[swe-ui-maker](swe-ui-maker.md)** - UI component creation
- **[pr-review-architecture-maker](pr-review-architecture-maker.md)** - PR-review specialist scoped to architecture only: new tradeoffs, module boundaries, reversibility, blast radius, quality-attribute effects, novel dependencies
- **[pr-review-logic-maker](pr-review-logic-maker.md)** - PR-review specialist scoped to business-logic/correctness only: behavior vs. domain intent plus Gherkin acceptance-criteria conformance across edge/error cases
- **[pr-review-governance-maker](pr-review-governance-maker.md)** - PR-review specialist scoped to governance/rules-conformance only: mechanical conformance to already-documented repo-governance/ conventions, naming/structure, ADRs, spec-file presence
- **[pr-review-security-maker](pr-review-security-maker.md)** - PR-review specialist scoped to security only: secrets in diffs, injection, untrusted-input handling, git-fixture isolation, unsafe git/FS operations
- **[pr-review-integrity-maker](pr-review-integrity-maker.md)** - PR-review specialist scoped to CI-gaming/test-integrity only: weakened/skipped/narrowed tests, coverage-gaming, missing regression tests
- **[pr-review-performance-maker](pr-review-performance-maker.md)** - PR-review specialist scoped to performance only: concrete/likely regressions, hot-path changes, algorithmic-complexity growth, resource concerns
- **[pr-review-docs-maker](pr-review-docs-maker.md)** - PR-review specialist scoped to documentation-quality only: substantive README/docs/Diátaxis fit, doc drift vs. code, clarity, doc alt-text/accessibility
- **[pr-review-instruction-maker](pr-review-instruction-maker.md)** - PR-review specialist scoped to instruction-decay only: a framework/build-tool/package-manager/env-var/CI change not reflected in AGENTS.md/CLAUDE.md/.claude/, and instruction bloat
- **[pr-review-synthesis-maker](pr-review-synthesis-maker.md)** - Mandatory coordinator (opus) atop the eight sonnet-tier specialists: classifies PR risk tier + specialist set, assembles the shared full-diff context once, reads prior-cycle human-dismissal status, then dedups/re-categorizes/reasonableness-filters/tool-verifies raw findings into exactly ONE consolidated review

### 🟩 Validation (Checkers)

- **[docs-checker](docs-checker.md)** - Factual accuracy validation
- **[docs-tutorial-checker](docs-tutorial-checker.md)** - Tutorial quality validation
- **[docs-link-checker](docs-link-checker.md)** - Link validity checking
- **[readme-checker](readme-checker.md)** - README quality validation
- **[plan-checker](plan-checker.md)** - Project plan validation
- **[plan-execution-checker](plan-execution-checker.md)** - Plan execution validation
- **[repo-rules-checker](repo-rules-checker.md)** - Governance compliance validation
- **[repo-workflow-checker](repo-workflow-checker.md)** - Workflow documentation validation
- **[specs-checker](specs-checker.md)** - Gherkin/BDD specs directory structural and content validation
- **[swe-code-checker](swe-code-checker.md)** - Validates projects against platform coding standards (validates application code rather than documentation)
- **[swe-ui-checker](swe-ui-checker.md)** - UI component quality validation
- **[ci-checker](ci-checker.md)** - CI/CD standards validation (mandatory Nx targets, coverage thresholds, Docker setup, Gherkin specs)
- **[docs-software-engineering-separation-checker](docs-software-engineering-separation-checker.md)** - Validates the boundary between generic dev docs and language-specific (Go, TypeScript, Rust, etc.) docs per the [Programming Language Docs Separation](../../repo-governance/conventions/structure/programming-language-docs-separation.md) convention
- **[repo-harness-compatibility-checker](repo-harness-compatibility-checker.md)** - The single harness-compat checker. **Phase 0** runs 5 deterministic cross-vendor parity invariants (governance/root-surface vendor-neutrality, binding sync no-op over `.opencode/` + `.amazonq/`, agent count parity, color + tier maps); **Phase 1** detects external drift between each supported harness's current upstream config conventions and the platform-bindings catalog + committed binding files (delegates multi-page research to `web-researcher`). Writes a dual-label audit to `generated-reports/`
- **[repo-setup-manager](repo-setup-manager.md)** - Executes Phase 0 environment setup (npm install, doctor --fix, baseline tests) before plan execution; resolves all preexisting failures to establish a clean, known-good baseline

### 🟨 Fixing (Fixers)

- **[docs-file-manager](docs-file-manager.md)** - File organization and management
- **[docs-fixer](docs-fixer.md)** - Apply validated documentation fixes
- **[docs-tutorial-fixer](docs-tutorial-fixer.md)** - Apply tutorial fixes
- **[readme-fixer](readme-fixer.md)** - Apply README fixes
- **[plan-fixer](plan-fixer.md)** - Apply plan fixes
- **[repo-rules-fixer](repo-rules-fixer.md)** - Fix governance compliance issues
- **[repo-workflow-fixer](repo-workflow-fixer.md)** - Fix workflow documentation
- **[specs-fixer](specs-fixer.md)** - Fix specs structural and accuracy issues
- **[swe-ui-fixer](swe-ui-fixer.md)** - Apply validated UI component fixes
- **[ci-fixer](ci-fixer.md)** - Apply validated CI/CD standards fixes
- **[docs-software-engineering-separation-fixer](docs-software-engineering-separation-fixer.md)** - Auto-moves misplaced language docs to the canonical destination flagged by the separation checker
- **[repo-harness-compatibility-fixer](repo-harness-compatibility-fixer.md)** - Applies validated fixes from a harness-compatibility audit; auto-remediates Phase 0 binding-sync drift (Invariant 3) via `npm run generate:bindings`, applies Phase 1 catalog/binding updates, flags higher-judgement gaps (vendor-audit prose, color/tier maps, orphan agents, generator-logic changes) for human resolution, and re-validates each finding before applying
- **[pr-review-fixer](pr-review-fixer.md)** - Triages and resolves `pr-review-synthesis-maker`'s consolidated findings on a GitHub PR (fix / reject-with-reason / defer-with-reason / clarify), replying to and resolving review threads; the fixer half of the PR-Review Maker→Fixer Cycle

### 🟩 Research (validation-adjacent)

- **[web-researcher](web-researcher.md)** - Read-only web research specialist; returns cited, structured findings with confidence tags in an isolated context (no file writes, no shell). Invoke for current API/library docs, fact verification, best-practice surveys. Uses `color: green` because web research is validation-adjacent (fact-checking, citation validation) and read-only by design; the `researcher` role maps to green. See [AI Agents Convention](../../repo-governance/development/agents/ai-agents.md#color-to-role-mapping) for the color-to-role mapping.

### 🧪 Testing

- **[web-exploratory-tester](web-exploratory-tester.md)** - **Spec-aware** session-based exploratory testing of a live site against a goal; actively hunts edge cases and boundary conditions; files findings (functional, behavioural consistency, edge-case/boundary, UI/responsive, accessibility, performance, URL/IA quality, safe security surface). Compares live behaviour against existing `specs/**` Gherkin and proposes new scenarios (Gherkin) for correct behaviours — especially edge cases — that lack coverage. Non-destructive; does not modify the site or fix code. Supports a selectable **`output-mode`** input: `plan` (default — files a new backlog plan folder), `delivery` (appends findings in-place to an existing plan's `delivery.md` given a `plan-path`; the rule-15 in-place retest mechanism), `local-temp` (writes a scratch `local-temp/<YYYY-MM-DD>__<slug>/findings.md` for immediate fixing with no plan paperwork).
- **[web-usability-tester](web-usability-tester.md)** - **Spec-blind** heuristic usability evaluation of a live site; judges only what a first-time user perceives (deliberately ignores specs/source/mockups) against established usability principles (Nielsen's 10 heuristics + 0–4 severity, cognitive walkthrough, information scent, first-click, Jakob's Law, ISO 9241-110, WCAG Understandable, UX laws). Evaluates predictability, internal/external consistency, information scent & flow, cognitive load, edge-case UX states (empty/loading/error), responsive usability (mobile/tablet/desktop), and URL/IA naturalness. Suggests new behaviour for `specs/**` in Gherkin (spec-blind `USS-###` candidates, flagged for reconciliation — distinct from exploratory's spec-gaps). Distinct from web-exploratory-tester (correctness); non-destructive. Supports the same selectable **`output-mode`** input as `web-exploratory-tester`: `plan` (default — files a new backlog plan), `delivery` (in-place append to an existing plan's `delivery.md`), `local-temp` (scratch findings file).
- **[web-design-tester](web-design-tester.md)** - **Design-aware** design-fidelity evaluation of a live site; judges whether the **running** rendered page matches its design and follows good design practice against five ground-truth sources (committed plan-folder mockups, design tokens/theme at runtime, design-system primitives `libs/ts-ui`, an optional external Figma/mockup source passed at invocation, and general design best-practice grounded by `web-researcher`). Evaluates mockup fidelity, runtime token/theme fidelity, design-system-primitive reuse, visual hierarchy, alignment, spacing/density (not cramped), typography, colour, and cross-surface visual consistency. Files `DWT-###` findings; locale- and evidence-aware. The **runtime** counterpart to `swe-ui-checker`'s **static** source audit, with no overlap. Distinct from web-exploratory-tester (correctness) and web-usability-tester (usability); non-destructive. Supports the same selectable **`output-mode`** input: `plan` (default — files a new backlog plan), `delivery` (in-place append to an existing plan's `delivery.md`), `local-temp` (scratch findings file).
- **[api-exploratory-tester](api-exploratory-tester.md)** - **Spec-aware, contract-aware** session-based exploratory testing of a live **REST or GraphQL** API against a goal; HTTP/curl-driven, **never** a browser. Actively hunts edge cases and boundary conditions (payloads, status codes, error envelopes, auth/authz, pagination, idempotency, GraphQL nullability/partial-errors/depth). Compares live responses against both the **API contract** (OpenAPI 3.x spec or GraphQL SDL) and existing `specs/**` Gherkin; proposes new scenarios (Gherkin) for correct behaviours — especially edge cases — that lack coverage. Files `AET-###` findings as a new backlog plan (README + brd + prd + findings + spec-gaps with exact `curl`/GraphQL steps-to-reproduce, secrets redacted). The **API-surface** counterpart to the rendered-UI web tester triad, with no overlap (it never audits HTML/CSS/visual/responsive concerns). Non-destructive (read-only by default; state-changing requests only with explicit per-run authorization). Supports the same selectable **`output-mode`** input: `plan` (default — files a new backlog plan), `delivery` (in-place append to an existing plan's `delivery.md`), `local-temp` (scratch findings file).

### 💻 Development

- **[swe-clojure-dev](swe-clojure-dev.md)** - Clojure application development
- **[swe-csharp-dev](swe-csharp-dev.md)** - C# application development
- **[swe-dart-dev](swe-dart-dev.md)** - Dart application development
- **[swe-e2e-dev](swe-e2e-dev.md)** - E2E testing with Playwright
- **[swe-elixir-dev](swe-elixir-dev.md)** - Elixir application development
- **[swe-fsharp-dev](swe-fsharp-dev.md)** - F# application development
- **[swe-golang-dev](swe-golang-dev.md)** - Go application development
- **[swe-java-dev](swe-java-dev.md)** - Java application development
- **[swe-kotlin-dev](swe-kotlin-dev.md)** - Kotlin application development
- **[swe-python-dev](swe-python-dev.md)** - Python application development
- **[swe-rust-dev](swe-rust-dev.md)** - Rust application development
- **[swe-typescript-dev](swe-typescript-dev.md)** - TypeScript application development

## Naming Rule

Every agent filename follows: `<scope>(-<qualifier>)*-<role>`

- `scope` — top-level domain (`apps`, `docs`, `exploratory`, `plan`, `repo`, `swe`, `ci`, `readme`, `specs`, `social`, `web`, `agent`).
- `qualifier` — zero or more refinement tokens (e.g., `crud-fs-ts-nextjs`, `link`, `ui`, `execution`).
- `role` — exactly one trailing token from the Role Vocabulary below.

No other structure is permitted. No exceptions.

Normative source: [Agent Naming Convention](../../repo-governance/conventions/structure/agent-naming.md).

## Role Vocabulary

| Role         | Semantics                                                                              | Example agents                                                                                  |
| ------------ | -------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `maker`      | Produces a content/research artifact                                                   | `docs-maker`, `docs-tutorial-maker`                                                             |
| `checker`    | Validates an artifact against standards                                                | `plan-checker`, `plan-execution-checker`, `swe-code-checker`                                    |
| `fixer`      | Applies validated checker findings                                                     | `plan-fixer`, `swe-ui-fixer`                                                                    |
| `dev`        | Writes code in a language or test framework                                            | `swe-rust-dev`, `swe-e2e-dev`                                                                   |
| `deployer`   | Deploys an application to an environment                                               | `apps-<scope>-deployer` (scope-specific deployer)                                               |
| `manager`    | Performs file or resource operations (rename/move/delete)                              | `docs-file-manager`                                                                             |
| `tester`     | Explores or evaluates a running system, live site, or API and reports defects/friction | `web-exploratory-tester`, `web-usability-tester`, `web-design-tester`, `api-exploratory-tester` |
| `researcher` | Gathers and verifies external information; read-only research                          | `web-researcher`                                                                                |

Enforcement: `rhino-cli agents validate-naming` (wired into pre-push and CI).

## Agent Format (Claude Code)

Agents use YAML frontmatter with the following structure:

```yaml
---
name: agent-name
description: Expert in X specializing in Y. Use when Z.
tools: Read, Glob, Grep
model:
color: blue
skills: []
---
```

**Name**: Required field - unique identifier using lowercase letters and hyphens
**Description**: Required field - when Claude should delegate to this agent
**Tools**: Comma-separated string with capitalized tool names (only tools the agent needs)
**Model**: Required field - omit for opus (default), or use \`sonnet\` or \`haiku\`

> **Opus-tier agents omit `model` by design** — this is budget-adaptive inheritance.
> The session's active model is inherited at runtime: Max/Team accounts get Claude Opus 4.7;
> Pro/Standard accounts get Claude Sonnet 4.6. Do NOT add `model: opus` to opus-tier agents
> — it bypasses this mechanism and forces Opus charges on all users regardless of account tier.
> See [model-selection.md](../../repo-governance/development/agents/model-selection.md) for full tier mapping.

**Color**: Required field - `blue` (makers), `green` (checkers), `yellow` (fixers), `purple` (implementors)
**Skills**: Required field - list of Skill names (empty array `[]` if no Skills used)

Note: Frontmatter MUST NOT contain YAML inline comments (# symbols). Put explanations in the document body.

### Model Benchmark Context

Tier assignments are based on benchmark data for each model. For cited scores (SWE-bench
Verified, GPQA Diamond, pricing, confidence levels) for all Claude and GLM models used in
this project, see
[docs/reference/ai-model-benchmarks.md](../../docs/reference/ai-model-benchmarks.md).
That document is the canonical source — all policy docs link back to it.

## Maker-Checker-Fixer Pattern

Three-stage quality workflow:

1. **Maker** (🟦 Blue) - Creates content
2. **Checker** (🟩 Green) - Validates content, generates audit reports
3. **Fixer** (🟨 Yellow) - Applies validated fixes

**Criticality Levels**: CRITICAL, HIGH, MEDIUM, LOW
**Confidence Levels**: HIGH, MEDIUM, FALSE_POSITIVE

## Dual-Mode Operation

**Source of Truth**: This directory (`.claude/agents/`) is the PRIMARY source.
**Sync Target**: Changes are synced to `.opencode/agents/` (SECONDARY) via automation.

**Making Changes**:

1. Edit agents in `.claude/agents/` directory
2. Run: `npm run generate:bindings` (powered by `rhino-cli` for fast regeneration of all secondary bindings)
3. Both systems stay synchronized

**Implementation**: Sync powered by `rhino-cli agents sync` (~121ms, 25-60x faster than bash)

**See**: [CLAUDE.md](../../CLAUDE.md) for complete guidance, [AGENTS.md](../../AGENTS.md) for OpenCode documentation, [apps/rhino-cli/README.md](../../apps/rhino-cli/README.md) for rhino-cli details

## Skills Integration

Agents leverage skills from `.claude/skills/` for progressive knowledge delivery. Skills are NOT agents - they provide reusable knowledge and execution services to agents.

**See**: [.claude/skills/README.md](../skills/README.md) for complete skills catalog

## Governance Standards

All agents follow governance principles:

- **Documentation First** - Documentation is mandatory, not optional
- **Explicit Over Implicit** - Clear tool permissions, no magic
- **Simplicity Over Complexity** - Single-purpose agents, minimal abstraction
- **Accessibility First** - WCAG AA compliance in all outputs

**See**: [repo-governance/principles/README.md](../../repo-governance/principles/README.md)

---
