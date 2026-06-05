# Business Requirements — Adopt Post-Mortem Convention

## Business Goal

Establish a shared, blameless post-mortem convention so that every significant failure in
`ose-primer` (and the downstream forks that inherit its governance) produces a durable,
consistently-structured record of what happened, why, and which concrete follow-ups prevent
recurrence.

## Business Rationale

This is a solo-maintainer, template repository. Its governance and conventions propagate
`ose-primer → downstream forks` [Repo-grounded — see Repository Ecosystem Convention]. The sibling
`ose-infra` repository already ships a mature blameless post-mortem convention; this work adopts it
faithfully so the three OSE repositories share one format. Two gaps motivate this work:

- **No capture format today.** The repo has a Root Cause Orientation principle and a Proactive
  Preexisting Error Resolution practice, but neither prescribes _how to record_ a failure after
  the fire is out. Searching the repo for `post-mortem` returns zero results [Repo-grounded].
  Without a format, lessons live only in commit messages and memory, and they do not propagate to
  forks.
- **Inconsistent recall.** When the same class of failure recurs months apart, the absence of a
  written post-mortem means the root cause is re-investigated from scratch. A standard format makes
  the prior analysis discoverable and reusable.

## Affected Roles

This is a solo-maintainer repo with no sign-off ceremonies. The relevant "hats" the maintainer
wears and the agents that consume these documents:

- **Maintainer-as-incident-responder** — writes a post-mortem after a significant failure, using
  the template.
- **Maintainer-as-reviewer** — reads past post-mortems before making risky changes.
- **`repo-rules-checker` / `repo-rules-fixer`** — validate the new convention document against
  governance conventions.
- **Downstream-fork maintainers** — inherit the convention and template verbatim.

## Business-Level Success Metrics

- **Convention discoverable** — the convention document is linked from the structure-conventions
  index and from the Root Cause Orientation principle. _Observable fact_ after delivery: both index
  entries resolve.
- **Template usable without external reference** — the writer-facing template page contains a
  complete copy-paste skeleton a contributor can fill without reading any other document.
  _Qualitative reasoning_: a self-contained template lowers the activation cost of writing a
  post-mortem, which is the single biggest barrier to the habit forming. [Judgment call]
- **Concreteness** — at least one fully-worked sample post-mortem exists so the format is never
  abstract. _Observable fact_ after delivery: the sample file exists and renders.
- **Cross-repo consistency** — the convention's structure (14 mandatory sections, severity scale,
  `doc_status` lifecycle, blameless principle) matches the `ose-infra` original. _Observable fact_:
  section-by-section parity holds; only paths and examples differ.
- **Governance-consistent at merge** — the new convention plus all index and back-link edits pass
  the [repo-rules-quality-gate](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md)
  workflow at `strict` mode before push. _Observable fact_ after delivery: the gate returns `pass`
  with zero CRITICAL/HIGH/MEDIUM findings on two consecutive validations.

## Business-Scope Non-Goals

- Not building incident-detection tooling or alerting.
- Not mandating a post-mortem for every trivial bug — only for failures meeting the
  significance threshold defined in the convention.
- Not introducing blame, performance review, or individual-accountability framing — the
  convention is explicitly blameless.
- Not redesigning the adopted format — structure stays identical to the `ose-infra` original.

## Business Risks and Mitigations

| Risk                                                                       | Mitigation                                                                                                                                                             |
| -------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Convention is written but never used (dead governance).                    | Ship a self-contained template + a worked sample so the activation cost is near-zero; cross-link from the principle the maintainer already follows.                    |
| Sample post-mortem is mistaken for a record of a real incident.            | The sample is explicitly labeled as a fabricated teaching example in its Summary and uses a clearly non-existent placeholder service name (`sample-be-service`).       |
| Over-process: post-mortems demanded for trivial issues, creating busywork. | The convention defines an explicit significance threshold via the severity scale; below it, no post-mortem is required.                                                |
| Adopted convention drifts from the `ose-infra` original.                   | This plan keeps the 14-section structure, severity scale, `doc_status` lifecycle, and blameless principle identical; only paths and illustrative examples are adapted. |
