# Product Requirements — standardize-app-spec-trees (ose-primer)

## Product Overview

This plan delivers a structural rename of the ose-primer `specs/apps/<family>/behavior/` directories
from the bare-surface scheme to the flat product-surface scheme, with full consumer rewiring, a
convention amendment, agent updates, and a rationale doc. The "product" here is the repository's
spec-and-governance surface itself — the deliverable is a coherent, enforceable, ecosystem-aligned
spec layout.

## Personas

Solo-maintainer repo — personas are the maintainer's hats and the consuming agents.

- **Spec author** — authors and edits Gherkin feature files; needs an unambiguous, predictable path
  to place a new feature under the right product-surface.
- **Build/CI engineer** — maintains `project.json` spec-coverage targets and `*-e2e` playwright
  configs; needs every moved path rewired so `nx affected` stays green.
- **Governance maintainer** — owns the convention and agent definitions; needs the amendment to be
  byte-identical with ose-public.
- **`specs-checker` agent** — enforces "one tree per family + flat product-surface behavior dirs".
- **`specs-maker` agent** — scaffolds new specs in the flat product-surface form.
- **Template adopter** — clones ose-primer and inherits the renamed layout.

## User Stories

- **US-1** — As a spec author, I want every behavior dir to name its product and surface, so that I
  can tell `crud-web` from `rhino-cli` at a glance and never place a feature under an ambiguous
  `web/`.
- **US-2** — As a build/CI engineer, I want every consumer of an old behavior path rewired in the
  same change, so that `nx affected -t test:quick spec-coverage` stays green after the rename.
- **US-3** — As a governance maintainer, I want the convention amendment to match the ose-public
  sibling plan byte-for-byte, so that the primer-sync classifier sees the convention as identity.
- **US-4** — As the `specs-checker` agent, I want a documented rule rejecting bare-surface and `api`
  behavior dirs, so that I can fail any non-conforming tree.
- **US-5** — As a future maintainer, I want a rationale doc capturing the locked decisions, so that
  the reasoning survives beyond this plan.

## Acceptance Criteria (Gherkin)

Each scenario uses exactly one primary `Given`, one `When`, one `Then`; extras chain with
`And`/`But`.

```gherkin
Scenario: crud backend behavior dir adopts the flat product-surface form
  Given the crud family stores backend specs at specs/apps/crud/behavior/be/gherkin/
  When the standardize-app-spec-trees plan is executed
  Then the backend specs live at specs/apps/crud/behavior/crud-be/gherkin/
  And no specs/apps/crud/behavior/be/ directory remains on disk
```

```gherkin
Scenario: crud web behavior dir adopts the flat product-surface form
  Given the crud family stores web specs at specs/apps/crud/behavior/web/gherkin/
  When the standardize-app-spec-trees plan is executed
  Then the web specs live at specs/apps/crud/behavior/crud-web/gherkin/
  And no specs/apps/crud/behavior/web/ directory remains on disk
```

```gherkin
Scenario: rhino CLI behavior dir adopts the flat product-surface form
  Given the rhino family stores CLI specs at specs/apps/rhino/behavior/cli/gherkin/
  When the standardize-app-spec-trees plan is executed
  Then the CLI specs live at specs/apps/rhino/behavior/rhino-cli/gherkin/
  And no specs/apps/rhino/behavior/cli/ directory remains on disk
```

```gherkin
Scenario: every crud-be backend project rewires its spec-coverage path
  Given the 11 crud-be backends declare specs/apps/crud/behavior/be/gherkin in project.json
  When the rename is applied with consumer rewiring
  Then every crud-be project.json spec-coverage command and inputs reference crud-be/gherkin
  And nx affected -t spec-coverage for the crud-be projects passes
```

```gherkin
Scenario: every crud frontend and e2e project rewires its web spec path
  Given crud-fe and crud-fs and crud-fe-e2e projects reference specs/apps/crud/behavior/web/gherkin
  When the rename is applied with consumer rewiring
  Then every such project.json and playwright.config.ts references crud-web/gherkin
  And nx affected -t spec-coverage test:quick for those projects passes
```

```gherkin
Scenario: rhino-cli Rust unit-test default path is updated
  Given apps/rhino-cli tests join the old specs/apps/rhino/behavior/cli/gherkin path
  When the rename is applied with the TDD-shaped source update
  Then the rhino-cli tests join specs/apps/rhino/behavior/rhino-cli/gherkin
  And nx run rhino-cli:test:quick passes
```

```gherkin
Scenario: convention amendment is byte-identical to the ose-public sibling plan
  Given the ose-public sibling plan amends specs-directory-structure.md with the flat scheme
  When this plan amends specs-directory-structure.md with the flat product-surface rule
  Then the amended subsection text is byte-identical to the ose-public amended subsection
  And the be-over-api rule and worked examples are present
```

```gherkin
Scenario: specs-checker and specs-maker enforce and scaffold the new scheme
  Given specs-checker.md and specs-maker.md reference bare-surface example paths
  When this plan updates both agents and re-syncs bindings
  Then specs-checker rejects bare-surface and api behavior dirs
  And specs-maker scaffolds behavior dirs in the flat product-surface form
  And the .opencode mirrors match the .claude sources after npm run generate:bindings
```

```gherkin
Scenario: rationale doc records the locked parity decisions
  Given the parity effort locks ten decisions in the shared decisions brief
  When this plan writes the rationale doc
  Then docs/explanation/standardize-app-spec-trees-parity-decisions.md exists
  And it records the flat product-surface scheme, the be-over-api rule, and the main-to-main deviation
```

```gherkin
Scenario: no dangling old-path reference remains in live consumers
  Given old behavior paths appear in project.json, e2e configs, READMEs, and governance docs
  When the full consumer sweep is complete
  Then a repo-wide grep for behavior/be/ behavior/web/ behavior/cli/ returns no live consumer
  But plans/done/ and archived/ historical references are exempt
```

## Product Scope

### In-scope features

- Three behavior-dir relocations via `git mv` (crud-be, crud-web, rhino-cli).
- Consumer rewiring across `project.json`, `*-e2e` playwright configs, app/specs READMEs,
  `rhino-cli` Rust test defaults, `.features-gen/` regeneration, and governance/docs cross-refs.
- Convention amendment (byte-identical to ose-public).
- `specs-checker.md` + `specs-maker.md` updates and binding re-sync.
- Rationale doc.

### Out-of-scope features

- Nx project / app-directory / contracts-project renames.
- ose-public and ose-infra family renames.
- Any `api` → `be` rename (no `api` surface exists in ose-primer).
- New families or product consolidation.

## Product-Level Risks

| Risk                                                      | Mitigation                                                                                |
| --------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| `.features-gen/` hand-edited instead of regenerated       | Delivery step regenerates from `playwright.config.ts`; gate verifies via the e2e target.  |
| A governance/docs cross-ref missed, leaving a broken link | Markdown link validation (`validate:links`) in the final gate catches broken `.md` links. |
| Convention amendment diverges from ose-public             | Gate diffs the amended subsection against the ose-public sibling plan.                    |
