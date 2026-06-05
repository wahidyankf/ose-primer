# Adopt Post-Mortem Convention

**Status**: In Progress
**Created**: 2026-06-05
**Identifier**: `adopt-post-mortem-convention`

## Context

`ose-primer` has a strong [Root Cause Orientation](../../../repo-governance/principles/general/root-cause-orientation.md)
principle and a [Proactive Preexisting Error Resolution](../../../repo-governance/development/practice/proactive-preexisting-error-resolution.md)
practice, but it has **no documented post-mortem process** — searching the repo for
`post-mortem` / `postmortem` returns zero hits [Repo-grounded]. When something breaks (a failed
deploy, a broken CI run, a regression that ships), there is no shared format for capturing what
happened, why, and which durable follow-ups prevent recurrence.

This plan **faithfully adopts** the existing blameless post-mortem convention shipped by the
sibling `ose-infra` repository, repointing its paths and generalizing its illustrative examples to
the application/service domain of `ose-primer`. It delivers: an authoritative **convention**
document, a writer-facing **template + index**, and one **illustrative sample post-mortem** so the
format is concrete rather than abstract.

## Scope

### In scope

- Create the convention document
  `repo-governance/conventions/structure/post-mortems.md` (title: **"Post-Mortem Convention"**)
  defining location/naming, the blameless principle, the 14 mandatory sections (plus optional
  Background and Supporting Data), the authoritative severity scale, action-item tracking, the
  `doc_status` lifecycle, the no-secrets rule, and diagram guidance.
- Create the writer-facing template + index at
  `docs/explanation/post-mortems/README.md` (copy-paste skeleton + the post-mortems index).
- Create one illustrative sample post-mortem at
  `docs/explanation/post-mortems/2025-01-15-sample-be-service-db-pool-exhaustion.md` (clearly
  marked as a fabricated teaching example, **not a real incident**).
- Wire the convention into its index READMEs
  (`repo-governance/conventions/structure/README.md`, `repo-governance/conventions/README.md`),
  add a post-mortems subdir entry to `docs/explanation/README.md`, and add reciprocal cross-links
  to the Root Cause Orientation principle and the Proactive Preexisting Error Resolution practice.

### Out of scope

- **Any tooling, automation, or CI gate** that detects incidents or enforces post-mortem creation.
- **Backfilling post-mortems** for past incidents.
- **Redesigning the format.** This is a faithful adoption of `ose-infra`'s post-mortem convention:
  the severity scale, blameless principle, `doc_status` lifecycle, and 14 mandatory sections are
  kept **identical**; only the paths and illustrative examples (app/service domain instead of
  Tailscale/Proxmox) are adapted.

## Approach Summary

Pure governance + documentation adoption. Three new markdown documents (the convention, the
template/index, and the sample) plus index/cross-link wiring. No application code, no Nx target, no
manifest changes. Work proceeds phase-by-phase: convention doc → template + index → sample
post-mortem → cross-reference wiring → governance quality gate
([repo-rules-quality-gate](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md) at
`strict` mode, fixing every CRITICAL/HIGH/MEDIUM finding before pushing) → local gates, commit, and
push. Each phase ends with a green gate.

## Navigation

- [Business Requirements](./brd.md) — why a post-mortem convention matters here.
- [Product Requirements](./prd.md) — what is delivered, with Gherkin acceptance criteria.
- [Technical Documentation](./tech-docs.md) — placement decisions and file-impact map.
- [Delivery Checklist](./delivery.md) — phased, executable steps.

## Related Documentation

- [Root Cause Orientation Principle](../../../repo-governance/principles/general/root-cause-orientation.md)
- [Proactive Preexisting Error Resolution Practice](../../../repo-governance/development/practice/proactive-preexisting-error-resolution.md)
- [No Secrets in Committed Files Convention](../../../repo-governance/development/quality/no-secrets-in-committed-files.md)
- [Diátaxis Framework](../../../repo-governance/conventions/structure/diataxis-framework.md)
- [Content Quality Principles](../../../repo-governance/conventions/writing/quality.md)
