# Demo apps standards recheck

One-line summary: re-verify that the demo apps still meet current repo standards after the recent
toolchain and governance churn.

> Idea, added (original capture undated; standing hygiene item).
> Relocated from ose-private/plans/ideas/demo-apps-standards-recheck.md on 2026-08-06 by plan-ideas-grooming.

## Problem / context

The demo apps were built against an earlier snapshot of repo standards, and the standards have since
moved (toolchain parity, CI conventions, lint strictness, Nx target shapes). Nothing has re-audited the
demo apps against the current bar, so any drift between what they do and what the standards now require
is invisible. No baseline measured — the size of the gap, if any, is unknown until the recheck runs.

## Why now

Standards have changed recently enough that quiet drift is plausible, and a demo app that violates
current standards is a bad reference for anyone reading it as an example.

## Prior art / precedents

- **standardize-repo-toolchain-parity plan** — the toolchain/CI churn that moved the bar these demo
  apps predate. [done plan](https://github.com/wahidyankf/ose-private/blob/main/plans/done/2026-06-13__standardize-repo-toolchain-parity/README.md)
- **Nx targets convention** — the canonical target shapes an app is checked against.
  [nx-targets.md](https://github.com/wahidyankf/ose-private/blob/main/repo-governance/development/infra/nx-targets.md)
- **Maker-checker-fixer pattern (swe-code-checker / ci-checker)** — the audit primitive this recheck
  would run per app. [maker-checker-fixer.md](https://github.com/wahidyankf/ose-private/blob/main/repo-governance/development/pattern/maker-checker-fixer.md)
- **Standardize CIs idea** — sibling audit whose CI portions overlap this recheck.
  [standardize-cis.md](https://github.com/wahidyankf/ose-public/blob/main/plans/ideas/q2-not-urgent-important/standardize-cis.md)

## Proposed direction (sketch)

- Enumerate the current repo standards that apply to an app (Nx targets, CI wiring, lint gates,
  structure conventions).
- Check each demo app against that checklist and record concrete pass/fail per item.
- File the actual gaps found as their own follow-up work rather than fixing blindly.

## Rough scope & non-goals

In scope: an audit of the demo apps against current standards, producing a concrete gap list.

Out of scope (for now): fixing the gaps (that is downstream work); changing the standards themselves;
non-demo production apps.

## Risks & open questions

- Which apps count as "demo apps" in this repo, exactly? (open)
- Which standards are in scope for a demo app vs. only production apps? (open)
- Is any gap large enough to warrant its own plan rather than an inline fix? (open — depends on findings)

## What success looks like + promotion signal

Success: every demo app has an auditable pass/fail verdict against current standards, and any real gaps
are captured as concrete follow-ups. Ready to promote to a `backlog/` plan once the standards checklist
and the demo-app set are pinned down — the audit mechanics are straightforward.
