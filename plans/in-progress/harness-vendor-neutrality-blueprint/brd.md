---
title: "BRD: Harness/Vendor Neutrality Blueprint — Phase 1 (ose-primer)"
---

# Business Requirements Document: Harness/Vendor Neutrality Blueprint — Phase 1

## Business Goal

Establish a **harness/vendor neutrality blueprint** for ose-primer — a framework defining where
vendor names are permitted, where they are forbidden, how violations are detected, and how
compliance propagates to downstream forks.

The first concrete deliverable: replace the vendor-locked `sync:claude-to-opencode` npm script
with a unified, vendor-neutral `generate:bindings` script that regenerates **all** secondary
binding artifacts (OpenCode + Amazon Q) in a single command. Remove the old script completely
(hard delete — no alias, no passthrough).

## Business Impact

### Current Pain Points

**1. Silent correctness gap** [Repo-grounded]

`npm run sync:claude-to-opencode` only runs `agents sync` (OpenCode). It never runs
`agents emit-bindings` (Amazon Q). Every agent, hook, and documentation instruction that says
"run `sync:claude-to-opencode` after editing agents" silently leaves Amazon Q bindings stale.

The cross-vendor parity Invariant 3 check in `repo-parity-checker` uses:

```bash
npm run sync:claude-to-opencode && git diff --quiet .opencode/
```

It passes even when `.amazonq/` is out of date — a correctness hole in the parity gate.
[Repo-grounded: `.claude/agents/repo-parity-checker.md` Invariant 3 tool string]

**2. Vendor-locked naming** [Repo-grounded]

`sync:claude-to-opencode` encodes two vendor names (Claude, OpenCode) in a script name that lives
in the shared `package.json`. The repo's governance explicitly requires vendor neutrality in
shared artifacts (see Multi-Harness Binding Convention). This creates friction when onboarding new
harnesses — contributors ask "does this script handle my harness?"

**3. No formal blueprint** [Judgment call]

The repo has a governance vendor-independence convention and a multi-harness-binding convention,
but no single document that defines what "harness neutrality" means as a system: which zones are
neutral, which are vendor-specific, how violations are detected, and how the rule propagates. This
gap means violations (like `sync:claude-to-opencode`) emerge naturally over time with no
systematic enforcement.

**4. Ecosystem name clarity** [Judgment call]

The word `sync` describes a push/pull operation with a remote, not artifact generation. The word
`emit` is compiler-internal terminology (rustc, tsc) — not idiomatic for user-facing npm scripts.
The `generate:` namespace is used in comparable pipelines (Prisma, GraphQL Code Generator, OpenAPI
Generator) for artifact generation, though the exact `generate:bindings` form is a design
decision, not an ecosystem standard. [Judgment call — not a web-confirmed idiom.]

## Affected Roles

- **AI agents** — every agent definition that instructs agents to run `sync:claude-to-opencode`
  after editing `.claude/` files; these will use `generate:bindings` after this plan
- **Pre-commit / pre-push hooks** — call rhino-cli directly, not via the npm script; no change
  needed [Repo-grounded: `.husky/`]
- **Cross-vendor parity checker** (`repo-parity-checker`) — Invariant 3 uses the npm script and
  the `.opencode/` diff; must be updated to use `generate:bindings` and add `.amazonq/`
- **Human contributors** reading docs — will find `generate:bindings` in instructions
- **Downstream forks** — inherit `generate:bindings` via propagation; never re-create the old name

## Business-Level Success Metrics

1. **Observable fact**:
   `grep -r "sync:claude-to-opencode" --include="*.md" --include="*.json" --include="*.sh" . | grep -v "node_modules\|\.git\|target/\|dist/\|generated-reports/\|plans/\|worktrees/"`
   returns zero matches after migration. The old name is completely absent — no alias, no
   passthrough, no stale reference. (`worktrees/` is excluded — the unrelated
   `worktrees/iterative-prancing-bentley/` is out of scope.)

2. **Observable fact**: `npm run generate:bindings && git diff --quiet .opencode/ .amazonq/`
   exits 0 immediately after a fresh edit-and-generate cycle (no stale Amazon Q files).

3. **Observable fact**:
   `./apps/rhino-cli-rust/dist/rhino-cli repo-governance vendor-audit repo-governance/`
   exits 0 — governance prose is clean of vendor names outside exempt sections.

4. **Judgment call**: Future agent instructions will be unambiguous — `generate:bindings` is
   sufficient for all harnesses; contributors no longer wonder whether their harness is covered.

## Business-Scope Non-Goals

- Renaming `rhino-cli` CLI subcommands (`agents sync`, `agents emit-bindings`) — implementation
  details not visible in npm scripts
- Removing `sync:agents` / `sync:skills` / `sync:dry-run` targeted scripts — valid for focused
  operations
- Adding new harnesses — this plan does not change which harnesses are supported
- Changing Rust or Go rhino-cli source logic — only the npm wrapper and docs change
- Merging `repo-parity-*` agents into `repo-harness-compatibility-*` — ose-primer keeps them
  separate; that merge is a different concern

## Business Risks and Mitigations

| Risk                                                               | Mitigation                                                                                                         |
| ------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| Old `sync:claude-to-opencode` name in long-lived generated reports | Reports are historical; checkers skip `generated-reports/` by convention; no active use                            |
| Missing a documentation reference during bulk rename               | Delivery checklist includes explicit zero-match grep-verify step BEFORE committing; any miss is caught before push |
| `generate:bindings` runs `emit-bindings`, adding latency           | `agents emit-bindings` is fast (deterministic overwrite, no web I/O); build runs once for both subcommands         |
| `.opencode/` mirrors not regenerated for non-agent files           | Phase 3 reruns `generate:bindings` and greps `.opencode/` for residuals; stale mirror files fixed manually         |
| Dual-CLI parity scripts (Rust + Go) drift                          | Both `validate-cross-vendor-parity.sh` scripts updated in the same delivery batch                                  |
