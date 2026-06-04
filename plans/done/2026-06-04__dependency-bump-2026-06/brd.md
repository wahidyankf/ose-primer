# Business Requirements Document — Dependency Bump June 2026

## Business goal

Bring every dependency-bearing manifest in `ose-primer` to a security-cleared, policy-compliant
state for the June 2026 cycle: resolve all in-scope CVEs, eliminate floating/caret/tilde version
specifiers in favour of exact pins, and record every unavoidable security trade-off in the
long-lived [security-waivers register][waiver-register] so it remains auditable across plan
archival.

## Business rationale (why this exists)

`ose-primer` is the MIT-licensed **template** repository from which downstream OSE-style repos are
forked [Repo-grounded — `AGENTS.md` Sibling repositories table]. A template's dependency hygiene
propagates to every fork: a CVE left unpatched here, or a sloppy floating tag, is inherited by
every downstream consumer. Keeping the template clean and exactly-pinned is therefore a
higher-leverage activity than the same work in a leaf application.

The [June 2026 clearance report][clearance-report] surfaced concrete drivers [Web-cited]:

- **One CRITICAL Actuator auth-bypass** in Spring Boot (CVE-2026-40976, CVSS 9.1).
- **One CRITICAL .NET DataProtection EoP** (CVE-2026-40372, CVSS 9.1) affecting both .NET backends.
- **One residual CRITICAL transitive Jetty CVE** (CVE-2026-2332, CVSS 9.1) in the Clojure/Pedestal
  stack requiring a residual-risk waiver.
- **One EPSS-elevated CVE** — PyJWT crit-header bypass (CVE-2026-32597, CVSS 7.5, EPSS 4.69%),
  resolved cleanly under Path B (fix pre-cutoff).
- **12 Path C waivers** where no CVE-clean pre-cutoff version exists.
- **1 FUNCTIONAL-HOLD** (FluentAssertions 8.x relicensed to paid commercial).
- **1 supply-chain HUMAN decision** — the discontinued Flutter build base image.
- Widespread floating Docker tags and 10-majors-behind GitHub Actions.

## Business impact

### Pain points addressed

- **Inherited vulnerability risk** — unpatched CVEs in a template flow to every fork.
- **Non-reproducible builds** — floating Docker tags (`golang:1.25-alpine`, `node:24-alpine`,
  `postgres:17-alpine`) and caret/tilde specifiers make builds non-deterministic and silently
  drift across CI runs.
- **Supply-chain exposure** — a discontinued upstream Flutter image (`cirruslabs/flutter:stable`,
  EOL 2026-05-01 [Web-cited]) is an unmaintained dependency.
- **Audit gaps** — security trade-offs not recorded anywhere durable.

### Expected benefits

- All in-scope CVEs resolved or explicitly waived with documented justification [Judgment call:
  benefit follows directly from the clearance report's resolution of each row].
- Fully deterministic, exactly-pinned builds across all 11 ecosystems + Docker + CI.
- A populated, auditable waiver register that survives plan archival.
- A clean template state that downstream forks inherit.

## Affected roles

Solo-maintainer repository — no sign-off ceremonies. The maintainer wears these hats:

- **Security reviewer** — owns the waiver/hold decisions recorded in the register.
- **Polyglot maintainer** — applies and verifies per-ecosystem edits.
- **Release engineer** — owns deterministic Docker/CI pins.

Consuming agents: `repo-setup-manager` (Phase 0 baseline), the `swe-*-dev` family (per-ecosystem
edits), and the human operator (the two `[HUMAN]` decision items: Flutter image replacement and
`flutter upgrade`).

## Business-level success metrics

- **All in-scope CVEs from the clearance report are resolved or waived** — observable: post-bump
  `npm audit --audit-level=moderate`, `govulncheck ./...`, and per-ecosystem audits are clean, and
  the post-bump CISA KEV cross-reference is clean.
- **Zero floating/caret/tilde specifiers remain in in-scope manifests** — observable:
  `grep`-based phase-gate checks return no matches.
- **Every WAIVER / FUNCTIONAL-HOLD entry is recorded** — observable: the
  [security-waivers register][waiver-register] gains one row per Path C waiver and the
  FUNCTIONAL-HOLD, with KEV + EPSS columns populated.
- **All affected-project quality gates pass** — observable:
  `npx nx affected -t typecheck lint test:quick spec-coverage` exits 0.

## Business-scope Non-Goals

- **No major-version feature upgrades for their own sake.** Breaking upgrades are included only
  where security-driven or explicitly approved (Exposed 0.59→1.0, kotlinx-datetime 0.6→0.8). Dart
  3.12.1 and tailwindcss 4.3.0 are deferred (post-cutoff).
- **No lockfile hand-editing** — lockfiles are regenerated only.
- **No CISA KEV Fast-Track** — the only KEV CVE (CVE-2025-32433, Erlang SSH RCE) is already patched
  at the current pin `27.3.3` [Web-cited].
- **No new dependencies added** beyond the explicit `plug` pin required by CVE-2026-8468.

## Business risks and mitigations

| Risk                                                                   | Likelihood | Mitigation                                                                                                   |
| ---------------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------ |
| Path C waiver pins an un-soaked version that later regresses           | Low        | Each waiver records justification + CVE in the register; the residual risk is bounded and reviewable.        |
| Breaking upgrade (Exposed 1.0, kotlinx-datetime 0.8) breaks build      | Medium     | Those items are TDD-shaped in `delivery.md` (RED → GREEN → REFACTOR); phase gate blocks progress on failure. |
| Flutter base image replacement chosen poorly by operator               | Low        | `[HUMAN]` decision item surfaces the trade-off; operator picks a maintained image; agent verifies the build. |
| Plan goes stale before execution                                       | Medium     | Snapshot caveat mandates an eligibility re-run if promotion is delayed.                                      |
| Post-cutoff version accidentally pinned (drift from cutoff discipline) | Low        | Every version traces to the clearance report; phase gates grep for caret/tilde and re-audit.                 |

## References

- [June 2026 clearance report][clearance-report]
- [Dependency Bump Stability & Safety Policy][policy]
- [Security Waivers Register][waiver-register]
- Product specification → [`prd.md`](./prd.md)
- Technical clearance tables → [`tech-docs.md`](./tech-docs.md)

[clearance-report]: ../../../generated-reports/repo-dependency-bump-planning__be6560__2026-06-04--13-24__report.md
[policy]: ../../../repo-governance/development/workflow/dependency-bump-policy.md
[waiver-register]: ../../../docs/reference/security-waivers.md
