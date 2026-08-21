# Idea Briefs (Two-Pagers)

This folder holds **two-pagers**: shortened, promotable idea briefs that are richer than a one-line
todo but deliberately **not** full five-document plans. Each idea is one `<slug>.md` file. `ideas/`
is the first stage of the plan lifecycle:

```text
ideas/ (two-pagers) → backlog/ (full 5-doc plans) → in-progress/ → done/
```

## Two-Pagers

Grouped into Eisenhower quadrants by [`plan-ideas-grooming`](../../repo-governance/workflows/plan/plan-ideas-grooming.md).

### Q2 — Important, Not Urgent

No active plan waits on these and no live defect is running, but each carries a real stake. This is the plan-from-here quadrant.

- [demo-apps-standards-recheck](./q2-not-urgent-important/demo-apps-standards-recheck.md) — re-verify the ose-primer demo apps still meet current repo standards.
- [kotlin-gradle-jdk-toolchain-convergence](./q2-not-urgent-important/kotlin-gradle-jdk-toolchain-convergence.md) — converge `crud-be-kotlin-ktor`'s per-target JDK pinning into an asdf pin or a Gradle 9.1+ bump.

### Q4 — Neither Urgent nor Important

Parked deliberately. Kept because the need may become real, not because it is real now.

- [rust-msrv-1-94-1-upgrade](./q4-not-urgent-not-important/rust-msrv-1-94-1-upgrade.md) — bump the Rust MSRV to 1.94.1 to pick up the `CVE-2026-33056` Cargo tar fix, once the toolchain ships it.

## What a Two-Pager Is

A two-pager sits between a throwaway one-liner and a full backlog plan: short enough to write in one
sitting and triage at a glance, yet structured enough to decide whether to promote it. Target ≤ ~2
printed pages, ~8 short sections:

1. **Title + one-line summary** (plus a provenance note when it came from a plan)
2. **Problem / context** — a specific example of why the status quo doesn't work, with concrete data points (counts/sizes/measurements; never fabricated)
3. **Why now** — the urgency, dependency, or opportunity window
4. **Prior art / precedents** — 2-5 named precedents (tool/pattern/standard/prior plan) with links; lightweight at capture, deep `web-researcher` study deferred to promotion
5. **Proposed direction (sketch)** — core elements only; **not** wireframes, file paths, or Gherkin
6. **Rough scope & non-goals** — in-scope bullets + an explicit out-of-scope list
7. **Risks & open questions** — rabbit holes + the unknowns that block promotion
8. **What success looks like + promotion signal**

Keep it a brief, not a plan: one paragraph per section, no fabricated metrics, no secrets, and no
BRD/PRD/tech-docs/delivery split (that is the backlog plan's job).

## Before You Add — Integrate, Don't Duplicate

Before creating a new two-pager, scan the index above for an existing brief on the same problem or
area and **fold the new thought into it** rather than adding a near-duplicate. Two two-pagers about
the same underlying problem should be one. This applies equally to learnings routed here by the
Knowledge Capture phase — check for an existing home first.

## Promoting a Two-Pager to a Plan

Promotion is a **completeness gate, not a perfection gate**: an idea is ripe when every section holds
a real answer — including honest open questions — and the remaining questions genuinely need a full
plan's deeper work to answer. When a two-pager is ripe, create `backlog/<slug>/` as a full plan, carry
the problem/scope/questions forward, then **delete** the two-pager and drop its line above. "Not
promoted yet" is a legitimate state, distinct from "rejected".

## See Also

- [Plans Organization Convention → Ideas Folder (Two-Pagers)](../../repo-governance/conventions/structure/plans.md#ideas-folder-two-pagers)
  — the authoritative convention, template, and discipline.
- [Knowledge Capture Convention](../../repo-governance/development/quality/knowledge-capture.md) —
  routes future-work learnings from plan execution here as two-pagers.

## Grooming Log

### 2026-08-06 — plan-ideas-grooming (all four OSE repos in one run)

Swept 120 two-pagers across `ose-public`, `ose-primer`, `ose-private`, and `beaver-nest`; 79 survive. Every surviving idea carries a residency verdict (R1 secrets-bearing, R2 single-repo-only, R3 generalizable) and an Eisenhower quadrant.

- **Classified**: 2 idea(s) resident here, filed into quadrant folders.
- **Relocated in** (1):
  - `demo-apps-standards-recheck.md` from `ose-private` — rule R2: apps/crud-\* demo apps exist only in ose-primer
- **Deduplicated out** (3) — the surviving copy is named for each:
  - `merge-queue-adoption.md` → `ose-public/plans/ideas/q2-not-urgent-important/merge-queue-adoption.md`
  - `pr-review-bot-identity.md` → `ose-public/plans/ideas/q2-not-urgent-important/pr-review-bot-identity.md`
  - `source-code-credential-scanning.md` → `ose-public/plans/ideas/q2-not-urgent-important/source-code-credential-scanning.md`
- **Unresolved follow-ups**: none. No relocation was interrupted and no filename collision was deferred.

> Last groomed: 2026-08-06
