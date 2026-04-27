---
title: BRD — Adopt ose-public Mermaid Checker Enhancements
---

# Business Rationale

## Problem

The Mermaid checker in `apps/rhino-cli/internal/mermaid/` has drifted
behind the equivalent code in `ose-public`. ose-primer is the upstream
template that downstream repositories clone; running the older checker
in the canonical template means every cloner inherits a less-correct
baseline.

Three concrete gaps exist today.

1. **Missing rule for dense subgraphs.** A flowchart can contain a
   `subgraph ... end` block with 20 child nodes. Rules 1–3 do not
   notice — overall span and depth may still be small. The result is
   a wall-of-boxes cluster that renders unreadably. ose-public ships
   a `subgraph_density` warning (default threshold 6 direct children)
   that closes this hole. Warning-level so it is non-blocking but
   visible.
2. **Direction-handling bug in warning fields.** When the checker
   emits a `complex_diagram` warning, ose-primer reports `ActualWidth`
   and `ActualDepth` as the raw graph metrics (`span`, `depth`) without
   applying the direction mapping. For a `graph LR` diagram with
   `span=2, depth=8`, the user sees the warning say `width=2 depth=8`
   even though the rendered horizontal axis is 8 wide. ose-public's
   version reports the post-direction-mapping values, which match what
   the human actually sees on screen.
3. **Pre-push coverage gap.** The `rhino-cli:validate:mermaid` Nx
   target is restricted to `governance/` and `.claude/` paths, even
   though the CLI's default scan covers `docs/`, `governance/`,
   `.claude/`, and root `*.md`. Mermaid diagrams in `docs/` (153 of
   them across the AI primer, the BDD/TDD docs, programming-language
   docs) and in `plans/` are never validated before push.

## Why now

- The ose-public ↔ ose-primer drift was surfaced during a routine
  comparison while writing the AI primer documentation. Closing it
  immediately keeps the template canonical.
- The newly-added AI primer mermaid blocks include direction-aware
  diagrams that will benefit from accurate warning fields and from
  density gating on the RAG pipeline subgraph.
- The drift is small enough today (≈150 LOC plus tests) to land in
  one focused plan; deferring guarantees more skew.

## Why beneficial — net assessment

| Dimension         | Direction                                                                                              |
| ----------------- | ------------------------------------------------------------------------------------------------------ |
| Correctness       | Direction-mapped warning fields fix a user-visible bug                                                 |
| Coverage          | New rule catches a real readability failure mode                                                       |
| Consistency       | Eliminates upstream/downstream skew in canonical template                                              |
| Backward-compat   | New rule emits warning only (exit 0); new flag has default; opt-out with `--max-subgraph-nodes 0`      |
| Cost              | ≈150 LOC core + ≈300 LOC tests; remediation budget for repo diagrams variable but bounded by 153 files |
| Mission alignment | Directly serves "polyglot Nx monorepo template" mission — better repo hygiene tooling for every cloner |

## Stakeholders

- **Owners**: rhino-cli maintainers (this is a tooling-internal change).
- **Affected**: every contributor pushing markdown changes once the
  Nx target broadens to `docs/` + `plans/`. Failure mode is a soft
  warning unless they hit the existing label-length or width rules.
- **Downstream**: every repository that clones ose-primer (current
  known consumer: `ose-public`). After this plan ships, the next
  ose-primer-sync run from ose-public reaches parity in this module.

## Risks and trade-offs

- **Risk**: existing diagrams may emit new warnings the day the
  rule lands. Mitigation: warnings do not block CI; remediate in the
  same plan run before the merge so origin/main is clean.
- **Risk**: direction-mapped change to `ActualWidth`/`ActualDepth`
  flips the displayed values for any graph that was previously
  warning. Internal package, no external consumers; impact is on
  the human reading the warning. Net positive.
- **Trade-off**: adding `docs/` and `plans/` to the Nx target inputs
  invalidates the cache for that target whenever any of those
  markdown files change. Acceptable — the target is fast (Go binary
  plus regex scan over a few hundred files).

## Non-Goals

The following are explicitly out of scope for this plan.

- **Reciprocal sync from ose-public into ose-infra.** ose-infra tracks
  its own rhino-cli instance; that alignment is a separate plan if ever
  needed.
- **New rules beyond Rule 4.** No node-count limit, edge-count limit,
  density metric, or diameter check is introduced here.
- **Changing existing three rules' default thresholds.** The defaults
  for `--max-label-len 30`, `--max-width 4`, and `--max-depth 4` are
  unchanged; this plan ports behaviour from ose-public, not recalibrates
  existing gates.

## Success definition

- `nx run rhino-cli:validate:mermaid` exits 0 on the full repo
  (observable fact — the validator's exit code is the gate).
- Husky pre-push hook fires the upgraded validator on any `*.md`
  change and surfaces any new violations or warnings.
- The `ose-primer/apps/rhino-cli/internal/mermaid/` package is
  byte-comparable in shape to `ose-public` modulo the
  package-import path.
- Commits pushed direct to `origin main` per
  [Git Push Default Convention](../../../governance/development/workflow/git-push-default.md)
  Standard 1; no draft PR is opened unless the user explicitly
  requests one for this plan.
