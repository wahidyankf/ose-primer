# Business Requirements — Adopt Dependency Bump Policy & Planning Workflow

## Business Goal

Keep `ose-primer` faithful to its role as the MIT-licensed template extracted from `ose-public`.
Cross-repo propagation flows `ose-public → ose-primer → downstream forks`
([Repository Ecosystem Convention](../../../repo-governance/conventions/structure/repository-ecosystem.md)).
When `ose-public` formalizes a governance rule, `ose-primer` must carry it so that repositories
bootstrapped from this template inherit the same safety guarantees.

Today `ose-primer` has **no written policy** governing how dependencies are bumped. Contributors
and AI agents have no canonical rule for choosing a version, no soak requirement, no CVE-clearance
checklist, and no auditable waiver trail. This adoption closes that gap.

## Business Impact

- **Risk reduction**: A documented LTS-first / 60-day-soak / CVE-clearance policy prevents shipping
  fresh, unproven, or vulnerable dependency versions into template-derived repositories.
- **Auditability**: Every bump becomes traceable (path classification, cutoff date, clearance
  status, waiver sign-off) instead of an undocumented judgment call.
- **Template fidelity**: Downstream forks inherit the same dependency discipline as the upstream
  platform, reducing drift between the ecosystem's repositories.
- **Automation readiness**: The planning workflow gives the repo a repeatable, agent-driven
  dependency-hygiene sweep that produces a reviewable backlog plan rather than ad-hoc edits.

## Affected Roles

- **Repository maintainer** — gains a canonical policy to point to in review and a one-command
  planning sweep.
- **AI coding agents** (`web-research-maker`, `repo-rules-checker`, plan-execution orchestrator) —
  gain an explicit rule set to enforce and operationalize.
- **Downstream template consumers** — inherit the policy automatically when they fork.

## Business-Level Success Metrics

- The two upstream documents exist in `ose-primer`, adapted to this repo's real structure, and
  pass the repository's own naming, linking, and markdown quality gates (observable: CI green).
- A contributor can answer "which version do I pin, and how do I prove it is safe?" entirely from
  in-repo documentation (observable: policy doc present and linked from its index README).

## Business-Scope Non-Goals

- No change to any actual dependency version (this is policy adoption, not a bump).
- No new CI job or scheduled automation is created by this plan (the workflow is invoked on demand).
- No license, ownership, or visibility change to the repository.

## Business Risks

- **Reference drift**: Upstream docs reference `ose-public`-specific apps/paths. Mitigation: adapt
  every repo-specific reference to `ose-primer`'s verified structure (see `tech-docs.md`).
- **Naming-gate breakage**: The new `planning` workflow type must be accepted by both `rhino-cli`
  validators or CI/pre-push fails. Mitigation: update the convention **and** both validators in the
  same phase, with their tests, before the workflow file lands.
