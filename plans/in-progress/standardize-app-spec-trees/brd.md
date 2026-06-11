# Business Requirements — standardize-app-spec-trees (ose-primer)

## Business Goal

Give every `specs/apps/<family>/` tree in ose-primer one uniform, self-describing behavior-surface
naming scheme — **flat product-surface** — so that the spec layout matches the rest of the
open-sharia-enterprise ecosystem (ose-public, ose-infra) and downstream template consumers inherit
a single, predictable convention.

## Business Rationale (WHY)

ose-primer is the downstream public **template** repository: teams clone it to start their own
Sharia-compliant enterprise products. The `specs/apps/<family>/behavior/<surface>/` paths it ships
become the paths those teams copy. Today the bare-surface scheme (`behavior/be/`, `behavior/web/`,
`behavior/cli/`) reads ambiguously once a repo grows more than one product family — a reader cannot
tell from `behavior/web/` alone _which_ product's web surface it is. The flat product-surface scheme
(`behavior/crud-web/`, `behavior/rhino-cli/`) encodes the product token in the directory name, so
every path is unambiguous and the same `specs-checker` rule can enforce it everywhere.

Because conventions sync bidirectionally between ose-public and ose-primer (the primer-sync
classifier treats convention text as identity), the convention amendment authored here must match
the ose-public sibling plan byte-for-byte. Standardizing now keeps the template and its upstream in
lock-step and avoids future divergence churn.

## Business Impact

### Pain points addressed

- **Ambiguous spec paths in the shipped template** — `behavior/web/` does not name its product, so
  template adopters who add a second family inherit a collision-prone layout.
- **Inconsistent vocabulary across the ecosystem** — without this change, ose-primer would carry a
  different behavior-dir scheme than ose-public/ose-infra, breaking the parity the ecosystem
  depends on.
- **No enforceable single rule** — bare-surface dirs cannot be machine-validated for product
  attribution; flat product-surface dirs can.

### Expected benefits

- A single, self-describing behavior-dir scheme that `specs-checker` can enforce mechanically.
- Byte-identical convention text shared with ose-public, eliminating sync-classifier drift.
- Template adopters inherit an unambiguous, future-proof spec layout.

## Affected Roles

This is a solo-maintainer repository; "roles" denote the hats the maintainer wears and the agents
that consume the affected artifacts. No sign-off ceremony applies.

- **Spec author hat** — writes Gherkin under the renamed behavior dirs; consumes the amended
  convention.
- **Build/CI hat** — owns the `project.json` spec-coverage commands and inputs that move with each
  rename.
- **Governance hat** — owns the convention amendment and the agent updates.
- **`specs-checker` / `specs-maker` agents** — consume the amended convention and the updated agent
  definitions to enforce and scaffold the new scheme.
- **Template-adopter (downstream)** — inherits the renamed paths and the new convention when they
  clone ose-primer.

## Business-Level Success Metrics

- **Every ose-primer behavior dir uses the flat product-surface form.** _Observable fact:_ after
  execution, `find specs/apps -type d -path '*/behavior/*/gherkin'` lists only
  `crud-be`, `crud-web`, `rhino-cli` segments — no bare `be`/`web`/`cli`. `[Repo-grounded]` (the
  current tree shows exactly `crud/behavior/{be,web}` and `rhino/behavior/cli`).
- **Convention amendment text matches ose-public byte-for-byte.** _Observable check:_ a `diff` of
  the amended subsection against the ose-public sibling plan's amended subsection is empty.
- **No dangling old-path reference remains.** _Observable check:_ a repo-wide grep for
  `behavior/be/`, `behavior/web/`, `behavior/cli/` (excluding `plans/done/` and archived trees)
  returns zero live consumers. `[Judgment call]` — the sweep is exhaustive only to the extent the
  authoring-time grep enumerated consumers; the delivery gate re-runs the grep to confirm.

## Business-Scope Non-Goals

- Not changing the C4 five-folder layout (`product`, `system-context`, `containers`, `components`,
  `behavior`) — only the behavior-dir leaf names.
- Not renaming Nx projects, app directories, or contracts projects.
- Not introducing any new product family.
- Not delivering via PR — this plan is intentionally main-to-main (see business risk below and the
  delivery-mode deviation in `tech-docs.md`).

## Business Risks & Mitigations

| Risk                                                                      | Likelihood | Impact | Mitigation                                                                                                              |
| ------------------------------------------------------------------------- | ---------- | ------ | ----------------------------------------------------------------------------------------------------------------------- |
| Convention text drifts from ose-public, breaking sync identity            | Low        | High   | Author the amendment as byte-identical to the ose-public sibling plan; delivery gate diffs the two.                     |
| A consumer of an old behavior path is missed, breaking `spec-coverage`/CI | Medium     | High   | tech-docs enumerates every consumer with file path + line; each phase gate re-runs `nx affected` to catch a missed ref. |
| main-to-main delivery bypasses the PR-only Sync Convention default        | Accepted   | Low    | Recorded deviation (docs-and-structure only, no code/config behavior change); justification in `tech-docs.md`.          |
