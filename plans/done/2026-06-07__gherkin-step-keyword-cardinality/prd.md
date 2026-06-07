# Product Requirements — Gherkin Step-Keyword Cardinality Rule

## Product Overview

Ship an explicit HARD Gherkin convention rule, propagate it across the governance
surface, enforce it with a deterministic `repo-governance gherkin-keyword-cardinality`
audit command implemented in **both** rhino-cli implementations (Rust canonical + Go
parity twin) driven by a new Gherkin behavior contract, and retrofit the real
`specs/**/*.feature` corpus to conform. The product is a combination of governance
text, a dual-implementation linter with parity coverage, and normalized spec files.

## The HARD Rule (canonical text)

> **HARD rule — one primary keyword each**: Every `Scenario` MUST use exactly **one**
> primary `Given` line, exactly **one** primary `When` line, and exactly **one** primary
> `Then` line. Every additional precondition, action, or outcome MUST be chained with
> `And` or `But` — never a repeated `Given` / `When` / `Then` keyword. This reinforces
> the "one action / one behavior per scenario" norm.
>
> **Exemptions**: `Background` blocks and `Scenario Outline` `Examples` tables are
> exempt from the one-each constraint.

**Conforming example** (dogfooded throughout this plan):

```gherkin
Scenario: Login succeeds
  Given a registered user
  And the login page is open
  When the user submits valid credentials
  Then the dashboard is shown
  And a session token is set
```

**Non-conforming example** (violates — two primary `When` keyword lines):

```gherkin
# NON-CONFORMING EXAMPLE — deliberate illustration of the violation
Scenario: Login succeeds
  Given a registered user
  When the user opens the login page
  When the user submits valid credentials
  Then the dashboard is shown
```

(The fix replaces the second `When` with `And`.)

## Personas

- **Governance author** (maintainer hat / `repo-rules-maker`) — authors and propagates
  the rule.
- **Tooling engineers** (maintainer hats / `swe-rust-dev`, `swe-golang-dev`) — build
  the audit in both CLI implementations from one behavior contract.
- **Spec maintainer** (maintainer hats / `swe-typescript-dev`, `swe-golang-dev`,
  `swe-rust-dev`) — retrofits feature files and step definitions.
- **Consuming agents** — `plan-maker`, `plan-checker`, `repo-rules-checker`, the two
  affected skills, and any agent that emits Gherkin.

## User Stories

- **US-1**: As a governance author, I want the keyword-cardinality rule stated explicitly
  in the canonical convention, so that authors and agents cannot silently violate it.
- **US-2**: As a consuming agent, I want the rule reflected in my prompt and skills, so
  that I emit conforming Gherkin by default.
- **US-3**: As a maintainer, I want a deterministic audit that flags violations, so that
  CI blocks non-conforming scenarios without manual review.
- **US-4**: As a maintainer of the dual-CLI parity model, I want the audit specified in
  the behavior contract and implemented byte-identically in both CLIs, so that the
  parity gate keeps both implementations honest.
- **US-5**: As a spec maintainer, I want existing offenders fixed per-subtree with gates,
  so that no project's step bindings break during the retrofit.
- **US-6**: As a maintainer, I want the Step 0.5 deterministic preflight ported into the
  quality-gate workflow, so that deterministic findings are harvested before AI judgment.
- **US-7**: As a maintainer, I want a strict double-zero gate after the sweep, so that
  the rule is provably consistent repo-wide.

## Acceptance Criteria (Gherkin — dogfoods the new HARD rule)

```gherkin
Scenario: Canonical convention states the HARD rule
  Given the acceptance-criteria convention is open
  When a reader searches for the keyword-cardinality rule
  Then exactly one HARD rule line for one-Given-one-When-one-Then is present
  And the Background and Scenario Outline exemptions are documented
  And a conforming example and a non-conforming example are shown
```

```gherkin
Scenario: Governance sweep propagates the rule via repo-rules-maker
  Given repo-rules-maker has authored the rule in the canonical convention
  When the broad governance sweep completes
  Then every Gherkin-referencing repo-governance doc that discusses scenario structure references the rule
  And the plan-maker, plan-checker, and repo-rules-checker prompts reference the rule
```

```gherkin
Scenario: Skill packages propagate the rule without repo-rules-maker
  Given the two Gherkin-referencing skill packages are edited by hand
  When the binding generator is run
  Then plan-writing-gherkin-criteria reflects the rule
  And plan-creating-project-plans reflects the rule
  And the secondary bindings are re-synced with no parity drift
```

```gherkin
Scenario: Behavior contract owns the new audit command
  Given the dual-implementation parity convention requires spec-first behavior
  When the gherkin-keyword-cardinality contract feature file is authored
  Then the contract lives under the rhino behavior gherkin repo-governance directory
  And every contract scenario itself obeys the one-each keyword rule
  And both implementations bind the contract through their BDD test suites
```

```gherkin
Scenario: Deterministic audit flags a multi-When scenario
  Given a feature file with two primary When keyword lines in one scenario
  When the gherkin-keyword-cardinality audit runs on the file
  Then the audit reports a violation naming the file and scenario
  And the audit exits with a non-zero status
```

```gherkin
Scenario: Deterministic audit exempts Background and Scenario Outline
  Given a feature file whose only repeated keywords are in a Background block or an Examples table
  When the gherkin-keyword-cardinality audit runs on the file
  Then the audit reports zero violations
  And the audit exits with a zero status
```

```gherkin
Scenario: Rust and Go implementations are byte-identical
  Given both implementations of the gherkin-keyword-cardinality command are built
  When the shadow-diff harness runs the repo-governance corpus
  Then stdout, stderr, and exit codes match byte-for-byte across both binaries
  And the parity CI job passes
```

```gherkin
Scenario: Per-subtree retrofit fixes offenders without breaking bindings
  Given a spec subtree owning feature files that violate the rule
  When the offending scenarios are normalized and step definitions verified in lockstep
  Then the gherkin-keyword-cardinality audit reports zero violations for that subtree
  And every binding project's test:quick passes
  And every binding project's spec-coverage passes where the target exists
```

```gherkin
Scenario: Subtree with zero offenders is handled gracefully
  Given a spec subtree whose feature files already conform to the rule
  When the retrofit phase for that subtree runs the audit
  Then the audit reports zero violations
  And no feature file is edited for that subtree
  And the phase gate still runs and passes
```

```gherkin
Scenario: Step 0.5 deterministic preflight is ported and enumerates the category
  Given the quality-gate workflow doc previously had no deterministic preflight
  When the Step 0.5 section is ported from the ose-public sibling and adapted to standalone commands
  Then the workflow doc lists vendor-audit and gherkin-keyword-cardinality as preflight categories
  And the section stays vendor-neutral under the governance vendor audit
```

```gherkin
Scenario: Strict quality gate confirms repo-wide consistency
  Given the rule is authored, propagated, and enforced
  When the repo-rules-quality-gate workflow runs in strict mode
  Then the gate terminates with a pass status
  And the deterministic preflight reports zero gherkin-keyword-cardinality findings
```

## Product Scope

**In-scope features**:

- Canonical rule text + normalized example snippets.
- Broad governance propagation (with and without `repo-rules-maker`).
- Behavior contract feature file + deterministic `gherkin-keyword-cardinality` audit
  implemented in BOTH CLIs + shadow-diff corpus + Nx `validate:` targets + Step 0.5
  preflight port + CI wiring.
- Per-subtree `.feature` + step-definition retrofit.
- Strict `repo-rules-quality-gate` double-zero pass.

**Out-of-scope features**:

- BDD-mapping semantic changes beyond cardinality.
- Behavioral rewrites of scenarios.
- New feature files / new coverage beyond the contract + retrofit.
- Vendor-specific governance content.
- Edits to the excluded Elixir BDD-library self-test fixtures.

## Product Risks

| Risk                                                                 | Mitigation                                                                                                                 |
| -------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| Audit parser mis-detects keyword lines inside doc-strings / comments | RED tests and contract scenarios cover doc-string and comment edge cases before GREEN — in both languages.                 |
| Go map iteration makes text output non-deterministic (known Go trap) | Both implementations emit findings in sorted (path, line) order by design; shadow-diff corpus exercises multi-file output. |
| Retrofit changes step text and orphans a step definition             | Keyword-only edits leave step text unchanged; phase gates run `test:quick` + `spec-coverage` for binding projects.         |
| Skill edits drift from agent-prompt edits                            | Strict gate cross-validates after both propagation phases.                                                                 |
