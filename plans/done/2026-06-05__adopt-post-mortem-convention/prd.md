# Product Requirements — Adopt Post-Mortem Convention

## Product Overview

Three new markdown documents, plus index/cross-link wiring, that together adopt the `ose-infra`
blameless post-mortem convention for `ose-primer`:

1. **Convention document** — the authoritative governance rule (location/naming, blameless
   principle, 14 mandatory sections, severity scale, action-item tracking, `doc_status` lifecycle,
   no-secrets rule, diagram guidance).
2. **Template + index** — the writer-facing working surface a contributor copies and fills, plus
   the post-mortems index.
3. **Sample post-mortem** — one fully-worked, clearly-illustrative example.

## Personas

- **Incident-responder (maintainer hat)** — has just resolved a significant failure and needs to
  record it.
- **Reviewer (maintainer hat)** — about to make a risky change and wants to check whether a similar
  failure has a recorded post-mortem.
- **`repo-rules-checker`** — validates the new convention document conforms to governance
  conventions.
- **Downstream-fork maintainer** — inherits and applies the convention.

## User Stories

- **US-1**: As an incident-responder, I want an authoritative convention defining _when_ a
  post-mortem is required and _what_ structure it takes, so that I am not guessing whether a given
  failure warrants one or which sections to write.
- **US-2**: As an incident-responder, I want a self-contained copy-paste template, so that I can
  write a post-mortem by copying one skeleton and filling sections, without reading other docs.
- **US-3**: As a reviewer, I want a worked sample post-mortem, so that I understand the expected
  depth and tone before writing my own.
- **US-4**: As a maintainer, I want the convention cross-linked from the Root Cause Orientation
  principle and the Proactive Preexisting Error Resolution practice, so that it is discoverable from
  the documents I already follow.

## Acceptance Criteria (Gherkin)

```gherkin
Scenario: Convention document defines the post-mortem standard
  Given the repository has no post-mortem convention today
  When the adoption plan is executed
  Then a file repo-governance/conventions/structure/post-mortems.md should exist
  And its H1 title should be "Post-Mortem Convention"
  And it should define the location docs/explanation/post-mortems/ and the filename pattern YYYY-MM-DD-<system>-<short-failure>.md
  And it should state the blameless principle explicitly
  And it should enumerate the 14 mandatory sections in order
  And it should define the authoritative severity scale (Sev-1 through Sev-4)
  And it should describe the doc_status lifecycle (draft → reviewed → closed)
  And it should reference the No Secrets in Committed Files rule

Scenario: The 14 mandatory sections are present in order
  Given the convention prescribes a fixed section structure
  When a reader opens repo-governance/conventions/structure/post-mortems.md
  Then the Mandatory Sections list should contain, in this order: Frontmatter, Metadata Table, Summary, Impact, Detection, Timeline, Root Cause, Trigger, Contributing Factors, Resolution & Mitigations, Action Items, What Went Well, Lessons Learned, References
  And the optional sections Background and Supporting Data should be documented separately

Scenario: Template page provides a self-contained copy-paste skeleton
  Given a contributor needs to write a post-mortem
  When they open docs/explanation/post-mortems/README.md
  Then the document should contain a complete blank template inside a fenced markdown code block
  And the template should include a heading for every mandatory section named in the convention
  And the contributor should be able to copy the template without reading any other file
  And the page should serve as the post-mortems index

Scenario: A worked sample post-mortem exists and is clearly illustrative
  Given the format must be concrete, not abstract
  When the plan is executed
  Then docs/explanation/post-mortems/2025-01-15-sample-be-service-db-pool-exhaustion.md should exist
  And its Summary should state it is a fabricated teaching example, not a real incident
  And it should use the placeholder service name "sample-be-service"
  And it should populate every mandatory section from the convention
  And it should classify the incident using the authoritative severity scale

Scenario: The convention is discoverable from existing governance
  Given the convention must be reachable from documents the maintainer already follows
  When the plan is executed
  Then repo-governance/conventions/structure/README.md should link the new convention alphabetically
  And repo-governance/conventions/README.md (Structure section) should link the new convention
  And docs/explanation/README.md should link the post-mortems subdirectory
  And the Root Cause Orientation principle should reference the post-mortem convention
  And the Proactive Preexisting Error Resolution practice should reference the post-mortem convention

Scenario: All new and edited markdown passes repository quality gates
  Given markdown quality is enforced repo-wide
  When the plan's quality gate runs
  Then Prettier should report no formatting changes for the new and edited files
  And markdownlint should report zero errors for the new and edited files
  And every relative cross-link in the new documents should resolve to an existing file

Scenario: The governance quality gate passes at strict mode
  Given the new convention plus all index and back-link edits exist
  When the orchestrator runs the repo-rules-quality-gate workflow at strict mode
  Then the gate should return "pass"
  And it should report zero CRITICAL, HIGH, and MEDIUM findings on two consecutive validations
  And any CRITICAL, HIGH, or MEDIUM finding surfaced is fixed (via repo-rules-fixer) before pushing
```

## Product Scope

### In-scope features

- One authoritative convention document with: location/naming rules, blameless principle, the 14
  mandatory sections (plus optional Background and Supporting Data), the authoritative severity
  scale, action-item table structure, `doc_status` lifecycle, no-secrets rule, diagram guidance,
  and reciprocal cross-links.
- One writer-facing template + index page containing a complete, copy-ready blank skeleton.
- One worked, clearly-labeled sample post-mortem.
- Index registration in the two convention READMEs and the `docs/explanation` README, plus two
  reciprocal back-links.

### Out-of-scope features

- A post-mortem workflow document (this is a documentation/governance convention, not a workflow).
- Incident-detection tooling, alerting, or CI enforcement.
- Backfilling post-mortems for historical incidents.
- A dedicated `specs/` feature file (this is governance/docs, no executable behavior).
- Any redesign of the adopted format — structure matches the `ose-infra` original.

## Product-Level Risks

| Risk                                                               | Mitigation                                                                                                                          |
| ------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------- |
| Mandatory sections in the convention and the template drift apart. | The delivery checklist verifies the template includes every mandatory section named in the convention before the phase gate passes. |
| Convention or sample drifts from the `ose-infra` original.         | The 14-section structure, severity scale, and `doc_status` lifecycle are copied identically; only paths and examples are adapted.   |
| Sample post-mortem read as a real incident, polluting the record.  | Mandatory "illustrative example, not a real incident" banner in the Summary plus a clearly fictitious service name.                 |
| Cross-links rot at authoring time.                                 | Every relative link is verified with `Bash test -f` in the delivery steps and re-checked at the final gate.                         |
