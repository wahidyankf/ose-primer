---
title: Post-Mortems
description: Blameless incident retrospectives for OSE applications and systems
category: explanation
subcategory: post-mortem
tags:
  - index
  - explanation
  - post-mortems
  - incidents
  - reliability
---

# Post-Mortems

**Blameless incident retrospectives.** Each post-mortem explains _what happened_, _why_, and _what we changed_ so the same failure does not recur — and so the reasoning survives past the people who were in the room.

## What is a Post-Mortem?

Per the [Diátaxis framework](../../../repo-governance/conventions/structure/diataxis-framework.md), post-mortems are **understanding-oriented** (explanation): they answer "why did this happen?" rather than "how do I do X?". They are written **blameless** — focused on systems and contributing factors, never individuals.

## Authoritative Standard

The full rules — mandatory sections, severity scale, action-item tracking, `doc_status` lifecycle, naming, and the no-secrets requirement — live in the [Post-Mortem Convention](../../../repo-governance/conventions/structure/post-mortems.md). This page is the practical working surface: a copy-paste template and the index. When the two disagree, the convention wins.

## Template

Copy this skeleton when starting a new post-mortem. Section order is mandatory; sections marked _(optional)_ may be dropped for low-severity incidents. Exception: `Background` may be placed before `Summary` when substantial up-front context is needed — see the [Post-Mortem Convention](../../../repo-governance/conventions/structure/post-mortems.md) for the full placement rule and what each section must contain.

```markdown
---
title: "Post-Mortem: <System> — <Short Failure>"
description: <one sentence>
category: explanation
subcategory: post-mortem
doc_status: draft # draft → reviewed → closed (document lifecycle, not incident status)
tags:
  - post-mortem
  - <system-tag>
---

# Post-Mortem: <System> — <Short Failure>

| Field              | Value                            |
| ------------------ | -------------------------------- |
| Incident date      | YYYY-MM-DD                       |
| Investigation date | YYYY-MM-DD                       |
| Severity           | Sev-N — Label (see convention)   |
| Status             | Investigating / Resolved         |
| Author             | <role> (blameless retrospective) |

## Summary <!-- 2–4 sentences: what, how long, outcome -->

## Impact <!-- quantify; MTTD/MTTR or "unknown — no alerting" -->

## Detection <!-- + category: Manual | Monitoring Alert | Automated Health Check | User Report -->

## Timeline <!-- absolute timestamps, stated timezone (WIB, UTC+7) -->

## Root Cause <!-- the systemic condition -->

## Trigger <!-- the proximate event — distinct from root cause -->

## Contributing Factors

## Resolution & Mitigations <!-- applied fixes vs open root-cause fix -->

## Action Items <!-- table: # | Action | Owner | Priority | Ticket | Status -->

## What Went Well <!-- include "where we got lucky" -->

## Lessons Learned

## References

## Background <!-- optional -->

## Supporting Data <!-- optional -->
```

## Filing Conventions

- **Filename**: `YYYY-MM-DD-<system>-<short-failure>.md` (incident-date prefix + kebab-case). The prefix keeps the flat directory in chronological order. Example: `2025-01-15-sample-be-service-db-pool-exhaustion.md`.
- **Layout**: flat directory — no subdirectories inside `docs/explanation/post-mortems/`. Revisit folder grouping only if volume grows.
- **Timing**: write promptly while details are fresh (within a few days of the incident).
- **`doc_status`**: `draft` → `reviewed` → `closed` (close only once all P0 action items resolve).
- **No secrets**: never commit real API tokens, connection strings, service hosts, or keys. Use placeholders (`<api-token>`, `<db-connection-string>`, `<service-host>`) per the [No Secrets in Committed Files](../../../repo-governance/conventions/security/no-secrets-in-committed-files.md) rule.
- **Blameless tone**: describe systems and decisions, not people.

## Index

- **[Sample BE Service — DB Connection Pool Exhaustion](./2025-01-15-sample-be-service-db-pool-exhaustion.md)** (2025-01-15, Sev-3) — illustrative/sample post-mortem (not a real incident): a sample backend service exhausts its DB connection pool under load.
