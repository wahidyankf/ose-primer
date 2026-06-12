# BRD — Mermaid State Diagram Validation (ose-primer)

> Business Requirements Document. WHY this exists. Solo-maintainer repo — no sign-off ceremonies.

## Business Goal

Make rhino-cli's Mermaid render-discipline rules (width `≤4 nodes/rank`, label `≤30 chars per
segment`) apply to **state diagrams** exactly as they already apply to flowcharts, and re-shape
ose-primer's validator onto a fresh kind-agnostic module design that is identical across the three
sibling repos so the rule sets cannot silently diverge again.

## Business Impact

Pain points addressed:

- **Silent escape hatch.** State diagrams render with no width guard. Authors can ship an 11-state
  `direction LR` chain that is unreadable on a mobile viewport and the gate passes it. [Repo-grounded:
  `apps/rhino-cli/src/internal/mermaid/parser.rs:12-15` — header regex matches only
  `flowchart`/`graph`.]
- **Downstream-template credibility.** ose-primer is a public template teams adopt to scaffold
  their own Sharia-compliant products. A documented "all Mermaid diagrams are width-checked" promise
  that quietly excludes state diagrams undermines trust in the scaffolding the template ships.
- **Cross-repo drift risk.** Three independently-evolved validators (public monolith, infra
  version-behind monolith, primer modular split) make it likely a future fix lands in one repo and
  not the others. A shared design + shared corpus removes that risk.

Expected benefits:

- State diagrams obey the same readability discipline as flowcharts in the template repo.
- A single kind-agnostic core means future diagram-type support (sequence, class, ER) is a
  front-end addition, not a per-repo rewrite.
- A machine-checked parity lock (the shared golden corpus) guarantees the three repos behave
  identically. [Judgment call: drift-prevention value is qualitative, not measured.]

## Affected Roles

Solo-maintainer repo; the maintainer wears these hats:

- **Rust toolsmith** — re-shapes the modular validator and adds the state front-end.
- **Docs/diagram author** — fixes the markdown files containing state diagrams. Run
  `grep -rln stateDiagram --include='*.md' | grep -v node_modules | wc -l` at Phase C start for
  the current count. [Repo-grounded: command verified live; inline count omitted to prevent drift.]
- **Governance maintainer** — propagates the rule into `repo-governance` and re-syncs bindings.

Consuming agents: `swe-rust-dev` (implementation), `repo-rules-maker` (governance propagation),
`repo-setup-manager` (Phase 0 baseline), `plan-execution-checker` (final verification).

## Business-Level Success Metrics

- **State diagrams are width-checked.** After this plan, a `stateDiagram-v2` with 5+ nodes on one
  rank produces a `width_exceeded` violation. Observable: run `rhino-cli docs validate-mermaid`
  against the golden corpus fixture and confirm the violation appears. [Repo-grounded once corpus
  lands.]
- **Zero behavioral regression on flowcharts.** Every pre-existing flowchart test stays green.
  Observable: `nx run rhino-cli:test:unit` passes with no flowchart test removed or weakened.
- **Cross-repo parity holds.** The identical golden corpus produces byte-identical violation JSON
  in ose-public, ose-primer, and ose-infra. Observable: the committed `expected.json` fixtures
  match across repos. [Judgment call: verified by inspection during the parity run, not a standing
  metric.]
- **Repo-wide hygiene.** No state diagram anywhere in the repo (including `plans/done/`) exceeds
  4 nodes/rank or 30-char labels after Phase C. Observable: a repo-wide
  `validate-mermaid` scan with no exclusions reports zero state-diagram violations.

## Business-Scope Non-Goals

- Validating other Mermaid block types (`sequenceDiagram`, `classDiagram`, `erDiagram`,
  `gitGraph`). Deferred to a future plan.
- Changing the width/label thresholds (`max_width = 4`, `max_label_len = 30`) or the
  `complex_diagram` AND-gate downgrade behavior.
- Re-wiring the `validate:mermaid` Nx target, pre-commit hook, or CI workflow.
- Re-introducing the propagation-PR sync loop for this objective (deliberately bypassed for
  direct-to-main delivery).

## Business Risks and Mitigations

| Risk                                                         | Likelihood | Mitigation                                                                                                                             |
| ------------------------------------------------------------ | ---------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| Re-shaping the modular split regresses flowchart behavior    | Medium     | Phase A is a pure behavior-preserving refactor gated by the unchanged flowchart test suite staying green before any state work begins. |
| Primer parser drifts from the ose-public reference           | Medium     | Shared golden corpus with identical expected JSON is committed here; Phase B gate fails if output differs from the reference fixtures. |
| Repo-wide cleanup misses a state diagram in an excluded path | Low        | Phase C runs an explicit no-exclusion scan over the whole repo, not the gate's default scan.                                           |
| Governance doc and validator describe different rules        | Low        | Phase E runs `repo-rules-maker` to update `diagrams.md` and re-sync bindings as a gated step.                                          |
