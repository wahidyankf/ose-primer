# Multi-Harness Compatibility

**Status**: In Progress
**Created**: 2026-05-25
**Plan identifier**: `multi-harness-compatibility`
**Plan type**: Multi-file (README + brd + prd + tech-docs + delivery)
**Git workflow**: Trunk Based Development — direct push to `main` (worktree is isolation only; this run executes on `main` per explicit user instruction).

## Context

This repository already serves multiple AI coding agents through a vendor-neutral
canonical instruction surface (`AGENTS.md`) plus per-tool platform-binding directories
(`.claude/`, `.opencode/`, `.codex/`, `.github/`). Governance prose under `repo-governance/`
is held vendor-neutral by an automated audit (`rhino-cli repo-governance vendor-audit`).
[Repo-grounded — `docs/reference/platform-bindings.md`,
`repo-governance/conventions/structure/governance-vendor-independence.md`]

`ose-primer` is the **MIT-licensed template** in the OSE family, downstream of `ose-public`
and upstream of every fork that adopts the scaffolding. Whatever harness-agnostic posture
`ose-primer` ships propagates to those downstream forks.
[Repo-grounded — `repo-governance/conventions/structure/repository-ecosystem.md`]

This plan adopts the `2026-05-24__multi-harness-compatibility` plan from `ose-public` and
**adapts it to the structural reality of `ose-primer`**: the CLI is not a single `rhino-cli`,
it is **two co-equal implementations** — `apps/rhino-cli-rust/` (the binary CI and the developer
toolchain invoke) and `apps/rhino-cli-go/` (a permanently-kept, behaviorally-identical twin).
A shadow-diff parity gate (`apps/rhino-cli-rust/scripts/shadow-diff.sh`, CI job `parity`) asserts
the two emit **byte-identical** output for every command. Therefore every rhino behavior this plan
adds or changes (vendor-audit vocabulary, the new `agents emit-bindings`, the new
`agents validate-bindings`) **must land in both implementations and stay byte-identical**.
[Repo-grounded — `plans/done/2026-05-24__have-two-rhino-versions/README.md`,
`.github/workflows/pr-quality-gate.yml` job `parity`, `apps/rhino-cli-rust/scripts/shadow-diff.sh`]

The repository owner wants the repo to be **harness-agnostic in governance** and **explicitly
compatible** with nine named coding-agent harnesses:

GitHub Copilot, Cursor, Windsurf, JetBrains Junie, Amazon Q Developer, Claude Code, OpenAI Codex CLI,
Google Antigravity CLI, and Pi (pi.dev).

(OpenCode is already active and is treated as a tenth, already-covered harness.)

Web research carried over from the `ose-public` adoption (delegated to `web-research-maker`,
2026-05-24) established that **seven of the nine read the root `AGENTS.md` natively** — Copilot,
Cursor, Windsurf, Junie, Codex CLI, Antigravity, and Pi. Only **Amazon Q Developer does not read
`AGENTS.md`** (open upstream feature request), and **Claude Code reads `CLAUDE.md`** (already bridged
via the existing `@AGENTS.md` shim). This makes the canonical surface the load-bearing asset and
reduces most of the work to thin, drift-resistant pointer files plus one genuine bridge (Amazon Q).
These external-harness facts are tool properties, not repo properties, so they transfer unchanged
from the `ose-public` research; the new compatibility-audit workflow exists to re-verify them over
time. [Web-cited — see `tech-docs.md` §Harness Compatibility Matrix for per-tool citations]

## Scope

**In scope:**

- Keep `repo-governance/` (and `AGENTS.md` / `CLAUDE.md` prose) harness-agnostic — extend the
  vendor-audit to cover the new vendor names and binding paths in **both** `rhino-cli-rust` and
  `rhino-cli-go`, including false-positive-safe handling of short/ambiguous names (Amazon Q, Pi, `agy`).
- Add or formalize platform bindings for the nine harnesses, using a **two-tier strategy** (native
  AGENTS.md readers get thin pointers / native reads; non-readers get an explicit bridge).
- Create a new repository workflow `repo-harness-compatibility-quality-gate` that delegates to
  `web-research-maker` to re-verify each harness's current config conventions and detect drift against
  the platform-bindings catalog.
- Extend **both** rhino CLIs (vendor-audit regex + a new binding emitter + a new deterministic
  binding-parity guard) and keep them byte-identical via the shadow-diff parity gate; update
  `specs/apps/rhino/` Gherkin features accordingly.
- Add a **deterministic, agent-free** `rhino-cli agents validate-bindings` guard (implemented in both
  CLIs) wired into `.husky/pre-push` so generated bindings can never silently drift from `AGENTS.md`
  or the catalog.
- Sweep and update **all related Markdown files** (catalog, agent roster, workflow/convention indexes,
  `AGENTS.md` binding lists) so nothing references a stale set.
- Propagate the resulting governance rules with `repo-rules-maker` and validate via the
  `repo-rules-quality-gate` workflow.

**Out of scope:**

- Changing the canonical instruction content itself (this plan does not rewrite `AGENTS.md` substance).
- Installing/configuring the harnesses on any machine, or per-user/global config (`~/.codex/`,
  `~/.gemini/`, etc.) — repo-level bindings only.
- MCP server selection or secrets management for any harness.
- Mobile/IDE-plugin-specific UI settings that cannot be expressed as committed repo files.
- Adding _new_ rhino CLI domains beyond what this plan needs; archiving or deleting either CLI
  implementation (both stay as parity twins per the dual-implementation model).

**Affected apps/areas:** `apps/rhino-cli-rust/` (Rust), `apps/rhino-cli-go/` (Go),
`apps/rhino-cli-rust/scripts/shadow-diff.sh`, `repo-governance/`, `docs/reference/`,
`specs/apps/rhino/`, `.claude/` + `.opencode/` (agents), `.husky/pre-push`, `package.json`, and
new/updated harness binding directories at the repo root (`.amazonq/`).

## Approach Summary

1. **Decide the binding architecture** (two tiers: native readers vs. bridge-required) and the
   no-shadowing rule for tool-specific override files that outrank `AGENTS.md`
   (`GEMINI.md`, `.junie/AGENTS.md`, `AGENTS.override.md`).
2. **Extend the vendor-audit** in both CLIs so governance prose stays neutral against the expanded
   vendor vocabulary, with documented false-positive handling; prove byte-parity via shadow-diff.
3. **Generate/author the binding files** mechanically where possible (Amazon Q bridge, thin pointer
   files) via a new `agents emit-bindings` command in both CLIs, so they cannot drift from `AGENTS.md`.
4. **Author the compatibility-audit workflow** and its checker/fixer agents that use
   `web-research-maker` to keep the catalog honest over time (external drift).
5. **Add a deterministic pre-push parity guard** (`rhino-cli agents validate-bindings`, no agent, both
   CLIs) for internal drift, wired into `.husky/pre-push`.
6. **Update specs** (`specs/apps/rhino/`) for every new/changed rhino behavior.
7. **Sweep all related Markdown files** so indexes and binding lists stay consistent.
8. **Propagate governance rules** via `repo-rules-maker`, then **validate** via `repo-rules-quality-gate`.

## Documents

- [brd.md](./brd.md) — business rationale (why this matters, who benefits)
- [prd.md](./prd.md) — product requirements, personas, user stories, Gherkin acceptance criteria
- [tech-docs.md](./tech-docs.md) — harness compatibility matrix, architecture decisions, file impact
- [delivery.md](./delivery.md) — phased delivery checklist, worktree, quality gates, verification

## Related

- [Platform Bindings Catalog](../../../docs/reference/platform-bindings.md)
- [Governance Vendor-Independence Convention](../../../repo-governance/conventions/structure/governance-vendor-independence.md)
- [Workflow Naming Convention](../../../repo-governance/conventions/structure/workflow-naming.md)
- [Repository Ecosystem Convention](../../../repo-governance/conventions/structure/repository-ecosystem.md)
- [Repository Rules Quality Gate](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md)
- [Two Rhino Versions plan (dual-implementation model)](../../done/2026-05-24__have-two-rhino-versions/README.md)
