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

Coupled with this, **consolidate the harness-compat tooling to a single workflow and a single
checker/fixer pair**, matching `ose-public`: ose-primer currently carries two overlapping gates
(`repo-cross-vendor-parity-quality-gate` + `repo-harness-compatibility-quality-gate`) and four agents.
The five cross-vendor parity invariants become the harness-compat checker's deterministic Phase 0,
and the corrected binding-sync Invariant 3 (`generate:bindings` + `.opencode/`/`.amazonq/` diff) lands
there. This removes duplicated surface area and a contributor's confusion over which gate to run.

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
- **Cross-vendor parity checker** (`repo-parity-checker`) — its Invariant 3 (npm script +
  `.opencode/` diff) is **merged into `repo-harness-compatibility-checker` Phase 0** and the standalone
  agent is deleted; the merged Invariant 3 uses `generate:bindings` and adds `.amazonq/`
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

4. **Observable fact**: exactly ONE harness-compat workflow remains:
   `ls repo-governance/workflows/repo/ | grep -ciE "parity|harness"` returns `1`, and
   `grep -rn "repo-parity-checker\|repo-parity-fixer\|repo-cross-vendor-parity-quality-gate" --include="*.md" --include="*.json" --include="*.sh" . | grep -v "plans/\|generated-reports/\|worktrees/\|node_modules"`
   returns zero matches — matching `ose-public`'s single-gate end-state.

5. **Judgment call**: Future agent instructions will be unambiguous — `generate:bindings` is
   sufficient for all harnesses; contributors no longer wonder whether their harness is covered, and
   there is a single harness-compat checker/fixer pair to reason about.

## Business-Scope Non-Goals

- Renaming `rhino-cli` CLI subcommands (`agents sync`, `agents emit-bindings`) — implementation
  details not visible in npm scripts
- Removing `sync:agents` / `sync:skills` / `sync:dry-run` targeted scripts — valid for focused
  operations
- Removing the `validate-cross-vendor-parity.sh` scripts / `validate:cross-vendor-parity` Nx targets
  / pre-push guard — these survive the merge as the deterministic byte guard (as in `ose-public`)
- Adding new harnesses — this plan does not change which harnesses are supported
- Changing Rust or Go rhino-cli source logic — only the npm wrapper, docs, and agent/workflow prose change

## Business Risks and Mitigations

| Risk                                                               | Mitigation                                                                                                         |
| ------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| Old `sync:claude-to-opencode` name in long-lived generated reports | Reports are historical; checkers skip `generated-reports/` by convention; no active use                            |
| Missing a documentation reference during bulk rename               | Delivery checklist includes explicit zero-match grep-verify step BEFORE committing; any miss is caught before push |
| `generate:bindings` runs `emit-bindings`, adding latency           | `agents emit-bindings` is fast (deterministic overwrite, no web I/O); build runs once for both subcommands         |
| `.opencode/` mirrors not regenerated for non-agent files           | Phase 3 reruns `generate:bindings` and greps `.opencode/` for residuals; stale mirror files fixed manually         |
| Dual-CLI parity scripts (Rust + Go) drift                          | Both `validate-cross-vendor-parity.sh` scripts updated in the same delivery batch                                  |
