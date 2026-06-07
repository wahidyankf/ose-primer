# Business Requirements — Gherkin Step-Keyword Cardinality Rule

## Business Goal

Make the canonical Gherkin scenario shape an **explicit, enforceable rule** instead of
an implicit convention demonstrated only by example, so that every scenario authored by
a human or an AI agent uses exactly one primary `Given`, one `When`, and one `Then`,
with all extras chained via `And` / `But`.

## Business Rationale (WHY)

- The repo already treats "one action / one behavior per scenario" as a norm (see the
  Best Practices section "One Scenario Per Behavior" and the "Multiple Behaviors in One
  Scenario" anti-pattern in
  [`acceptance-criteria.md`](../../../repo-governance/development/infra/acceptance-criteria.md))
  [Repo-grounded], but it never states the keyword-cardinality form of that norm as a
  rule. A norm shown only by example is silently violable — and a deterministic
  pre-scan at authoring time already found one violating scenario in the corpus
  (`specs/apps/crud/behavior/web/gherkin/layout/responsive.feature`, scenario "Mobile
  viewport hides sidebar behind a hamburger menu": two primary `When` and two primary
  `Then` lines) [Repo-grounded].
- Multiple primary `When`/`Then` lines in one scenario create ambiguity in the
  BDD-to-test mapping
  ([`bdd-spec-test-mapping.md`](../../../repo-governance/development/infra/bdd-spec-test-mapping.md))
  [Repo-grounded] — it becomes unclear which action a step definition binds to. This
  matters doubly here, because the single behavior contract at
  `specs/apps/rhino/behavior/cli/gherkin/` owns **two** CLI implementations per the
  [Dual-Implementation Parity Convention](../../../repo-governance/conventions/structure/rhino-cli-dual-implementation-parity.md)
  [Repo-grounded].
- An explicit rule plus a **deterministic linter** removes interpretation from both AI
  agents and human contributors, which is consistent with the repo's
  "Explicit Over Implicit" and "Automation Over Manual" principles. [Repo-grounded]

## Business Impact

**Pain points addressed**:

- Inconsistent scenario structure across the 58 tracked `specs/**/*.feature` files
  (66 tracked `.feature` files repo-wide including the 8 excluded BDD-library self-test
  fixtures) [Repo-grounded — `git ls-files` inventory at authoring].
- AI agents (plan-maker, spec makers) can emit non-conforming Gherkin because no rule
  forbids it. [Judgment call]
- Reviewers must catch cardinality drift by eye, with no automated gate — the one
  existing offender survived review. [Repo-grounded]

**Expected benefits**:

- One unambiguous, machine-checked scenario shape repo-wide.
- Reduced reviewer burden — the linter and CI catch violations deterministically.
- Sharper BDD-to-test mapping (one action per scenario → one clear binding), which
  protects the dual-CLI behavior contract from drifting into ambiguous shapes.

## Affected Roles

This is a solo-maintainer repository; "roles" denote the hats the maintainer wears and
the agents that consume the governance surface. No sign-off ceremonies apply.

- **Governance author hat** — authors the rule and runs the broad sweep (delegated to
  `repo-rules-maker`).
- **Tooling/Rust hat** — builds the canonical audit implementation (delegated to
  `swe-rust-dev`).
- **Tooling/Go hat** — builds the parity-twin audit implementation (delegated to
  `swe-golang-dev`).
- **Spec maintainer hats** — retrofit per-subtree `.feature` files + step defs
  (delegated to `swe-typescript-dev` / `swe-golang-dev` / `swe-rust-dev` per subtree).
- **Consuming agents** — `plan-maker`, `plan-checker`, `repo-rules-checker`, the two
  affected skill packages, and any agent that emits Gherkin.

## Business-Level Success Metrics

- **Rule presence**: the HARD rule text appears verbatim in
  `acceptance-criteria.md` and is referenced by every Gherkin-touching governance doc and
  agent prompt that the sweep covers. [Judgment call — verified observationally at execution]
- **Enforcement**: the new `gherkin-keyword-cardinality` audit command exists in BOTH
  CLI implementations, passes both test suites and both spec-coverage gates, is
  byte-identical under the shadow-diff harness, and is wired into the ported Step 0.5
  preflight + CI. [Judgment call — observable via target exit codes]
- **Zero offenders**: after retrofit, the audit reports **zero** violations across the
  aligned scan scope (tracked `**/*.feature` minus exclusions; net `specs/**` today).
  [Judgment call — observable via audit exit code]
- **Double-zero gate**: `repo-rules-quality-gate` (strict) terminates with `pass`.
  [Judgment call — observable via workflow status]

No fabricated numeric KPIs are claimed; all metrics above are observable checks performed
at execution time.

## Business-Scope Non-Goals

- Not changing what scenarios _test_, only how their keyword lines are structured.
- Not introducing a new test framework or BDD harness.
- Not deferring offenders — violating `.feature` files are fixed in this plan.
- Not adding vendor-specific content to any `repo-governance/` file.
- Not modifying the BDD-library self-test fixtures under `libs/elixir-cabbage/` and
  `libs/elixir-gherkin/` (excluded from linter scope by design).

## Business Risks and Mitigations

| Risk                                                                               | Likelihood | Mitigation                                                                                                                                                                                 |
| ---------------------------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Retrofit edits silently break a project's step-definition bindings                 | Medium     | Per-subtree phased delivery; each phase gate runs the binding projects' `test:quick` + `spec-coverage` (where the target exists) before proceeding. [Repo-grounded — gates in delivery.md] |
| Dual implementations drift (one CLI gets the command, the other lags or diverges)  | Medium     | Single delivery phase implements both per parity convention Rule 1; shadow-diff corpus cases added in the same phase; the `parity` CI job is the permanent arbiter. [Repo-grounded]        |
| Linter false positives on `Background` / `Scenario Outline`                        | Medium     | Exemptions are part of the rule spec and covered by dedicated RED tests (and contract scenarios) before GREEN.                                                                             |
| Governance sweep misses a Gherkin-referencing surface                              | Low        | `repo-rules-quality-gate` (strict) double-zero pass validates repo-wide consistency after the sweep.                                                                                       |
| Broad sweep introduces inconsistency between with/without `repo-rules-maker` edits | Low        | Distinct phases with explicit file lists; final strict gate cross-checks.                                                                                                                  |
