---
title: "Post-Mortem Convention"
description: Standards for writing and organizing blameless incident post-mortems in this repository, including location, naming, mandatory sections, severity scale, and action-item tracking
category: explanation
subcategory: conventions
tags:
  - post-mortem
  - incidents
  - blameless
  - reliability
  - structure
---

# Post-Mortem Convention

This convention defines how to write, name, and organize blameless incident post-mortems for this repository. Post-mortems are permanent retrospective documents that examine what happened during an incident, why it made sense at the time, and what systemic changes prevent recurrence.

The practical writer-facing template and index live in
[`docs/explanation/post-mortems/README.md`](../../../docs/explanation/post-mortems/README.md).
This document is the **authoritative governance rule**; that directory is the working surface.

## Principles Implemented/Respected

This convention implements the following core principles:

- **[Documentation First](../../principles/content/documentation-first.md)**: Post-mortems are
  mandatory permanent documentation. Writing them promptly while details are fresh treats
  documentation as a first-class deliverable, not an afterthought.

- **[Root Cause Orientation](../../principles/general/root-cause-orientation.md)**: The blameless
  framing, "second story" questions, and explicit root cause / contributing factors structure keep
  analysis focused on systemic conditions rather than individual missteps. Each action item must
  address a root cause, not just the proximate trigger.

- **[Deliberate Problem-Solving](../../principles/general/deliberate-problem-solving.md)**:
  Structured timelines, quantified impact, and severity classification demand that authors
  understand what actually happened before proposing fixes. The retrospective process favors
  reversible, targeted interventions over reflexive procedural changes.

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**:
  Mandatory frontmatter status, an explicit severity tier, absolute timestamps with stated
  timezone, and typed detection categories make every incident's state and context unambiguous.

## Purpose

Post-mortems serve two purposes:

1. **Learning** — document what actually happened and why decisions made sense at the time, so the
   team builds accurate mental models of its systems.
2. **Improvement** — convert that learning into concrete, owned, prioritized action items that
   reduce the probability or impact of similar incidents.

A post-mortem is not a punishment mechanism. It is a systems-thinking tool applied retrospectively.

## Scope

### What This Convention Covers

- Location and filename rules for post-mortem documents
- The blameless culture standard
- Mandatory and optional sections (in order)
- Severity scale definition (authoritative)
- Action-item table structure and tracking
- `doc_status` lifecycle
- Timing expectation
- Security constraints (no secrets)
- Diagram guidance

### What This Convention Does NOT Cover

- Incident response procedures during an active outage (that is an operational runbook)
- On-call rotation or escalation policies
- Post-mortem review meeting facilitation
- Plan content structure (see [Plans Organization](./plans.md))

## Standards

### Location and Naming

Post-mortems live in the **Diátaxis "explanation" tier** because they build conceptual
understanding of how a system behaved under stress.

**Location**: `docs/explanation/post-mortems/`

**Filename pattern**: `YYYY-MM-DD-<system>-<short-failure>.md`

Where:

- `YYYY-MM-DD` is the **incident date** (not the writing date)
- `<system>` is the affected app or service (kebab-case)
- `<short-failure>` is a brief kebab-case description of what failed

**Rules**:

- Flat directory — no subdirectories inside `docs/explanation/post-mortems/`
- All components lowercase kebab-case
- Index maintained in `docs/explanation/post-mortems/README.md`

**Examples**:

| PASS: Correct                                           | FAIL: Incorrect                          | Why                                                            |
| ------------------------------------------------------- | ---------------------------------------- | -------------------------------------------------------------- |
| `2025-01-15-sample-be-service-db-pool-exhaustion.md`    | `post-mortem-2025-01-15.md`              | Missing system and failure                                     |
| `2025-09-14-rhino-cli-coverage-threshold-regression.md` | `2025-09-14__rhino-cli__coverage.md`     | Double underscore is plans-folder style, not post-mortem style |
| `2026-03-22-auth-service-token-signing-failure.md`      | `2026-03-22-AuthService-TokenSigning.md` | Uppercase not allowed                                          |

### Blameless Principle

Post-mortems examine **systems and processes**, not individuals.

**Apply the "second story" (Allspaw / Dekker)**: Ask "how did this sequence of events make sense
to the people involved at the time?" rather than "who made a mistake?" The first story is the
incident timeline; the second story is the context, pressures, and system state that made each
decision reasonable.

**Practical rules**:

- Avoid "human error" as a root cause. Human error is a symptom; the question is what system
  condition made that error likely or consequential.
- Never name individuals in a blame context. Roles and team descriptions are fine
  ("the on-call engineer", "the deployment automation"); attributing fault to a person is not.
- Avoid hindsight bias: document what was known at each decision point, not what you know now.
- Avoid "blameless buck-passing": shifting blame from a person to a team, a vendor, or a tool is
  still blame. Contributing factors name conditions, not culprits.

**Sources**: Google SRE "Postmortem Culture: Learning from Failure"; Allspaw, J. (2012)
"Blameless PostMortems and a Just Culture."

### Timing

Write the post-mortem **promptly while details are fresh**. Industry norm: within a few days of
the incident. Delay degrades timeline accuracy and action-item momentum. The `doc_status` field
starts as `draft` until a review pass confirms factual accuracy.

### Mandatory Sections

Every post-mortem MUST contain the following sections in this order.

#### 1. Frontmatter

```yaml
---
title: "Post-Mortem: <System> — <Short Failure>"
description: One-sentence summary of the incident
category: explanation
subcategory: post-mortem
tags:
  - post-mortem
  - <system>
  - <relevant-tag>
doc_status: draft
---
```

`doc_status` values:

- `draft` — initial write-up, may have gaps
- `reviewed` — factual accuracy confirmed by at least one other perspective (second reading,
  peer review, or log cross-check)
- `closed` — all P0 action items resolved; document is the settled record

`doc_status` is the **document status**, distinct from the incident `Status` field in the
metadata table below.

#### 2. Metadata Table

Immediately after the H1, before any prose:

```markdown
| Field              | Value                                             |
| ------------------ | ------------------------------------------------- |
| Incident date      | YYYY-MM-DD                                        |
| Investigation date | YYYY-MM-DD                                        |
| Severity           | Sev-N — label (see severity scale)                |
| Status             | Investigating / Resolved                          |
| Author             | Role or initials (not full name unless preferred) |
```

#### 3. Summary

Two to four sentences. State what failed, how long it lasted, and the outcome. Write it last but
place it first — it is the executive snapshot.

#### 4. Impact

Quantify impact wherever possible. Include:

- Services or users affected
- Duration
- MTTD (Mean Time to Detect) and MTTR (Mean Time to Resolve) — use `unknown — no alerting` if
  detection was manual and latency is not measurable

#### 5. Detection

How the incident was discovered. Append one of the following category labels in parentheses:

- **Manual** — a person noticed through inspection
- **Monitoring Alert** — an automated alerting rule fired
- **Automated Health Check** — a health-check endpoint or watchdog detected failure
- **User Report** — an end-user or external party reported the problem

Example: `"Engineer noticed elevated error rates during a routine deployment review. (Manual)"`

#### 6. Timeline

Absolute timestamps with stated timezone. Never use relative offsets ("T+5min") as the primary
form — they cannot be interpreted without an anchor and degrade over time.

```markdown
| Time (WIB UTC+7) | Event                                                      |
| ---------------- | ---------------------------------------------------------- |
| 2025-01-15 14:00 | Deploy raised concurrent request volume above pool ceiling |
| 2025-01-15 14:12 | Health check started returning 503s                        |
| 2025-01-15 14:25 | On-call engineer identified DB connection pool exhaustion  |
| 2025-01-15 14:40 | Pool limit raised; service restored                        |
```

Use the repository's standard [Timestamp Format](../formatting/timestamp.md) (UTC+7 WIB).

#### 7. Root Cause

The deepest systemic condition that made the incident possible. A root cause explains **why**
the trigger was able to cause harm, not just what happened.

Distinguish from Trigger (below). Do not name a person as a root cause.

#### 8. Trigger

The proximate event that started the incident chain — the "what pulled the thread." The trigger
is distinct from the root cause, which is the condition that made the trigger consequential.

Example distinction: Trigger = "a deploy raised concurrent request volume past the configured
pool ceiling"; Root Cause = "no pool-saturation alerting or autoscaling existed to detect or
absorb the load increase before connections were exhausted."

#### 9. Contributing Factors

Systemic conditions that made the incident worse or made recovery harder. Use a bullet list.
These are **conditions**, not causes to blame.

Avoid single-cause fixation: most non-trivial incidents involve several compounding conditions.
Naming them all produces richer action items.

#### 10. Resolution & Mitigations

Describe what restored service. Explicitly distinguish:

- **Applied fix**: what was done to resolve this incident
- **Open root-cause fix**: what still needs to happen to prevent recurrence (tracked in Action Items)

#### 11. Action Items

Each action item must be:

- **Actionable** — starts with a verb
- **Specific** — names the system, file, or process to change
- **Bounded** — has a clear definition of done
- **Owned** — assigned to a role or team
- **Prioritized** — P0 / P1 / P2 (see below)
- **Tracked** — linked to a `plans/` reference or issue id

Priority definitions:

| Priority | Meaning                                                                              |
| -------- | ------------------------------------------------------------------------------------ |
| **P0**   | Blocks recurrence or eliminates data-loss risk; complete before `doc_status: closed` |
| **P1**   | Important improvement; schedule promptly                                             |
| **P2**   | Nice-to-have; schedule when capacity allows                                          |

Table columns (use this exact structure):

```markdown
| #   | Action                                               | Owner        | Priority | Ticket                                        | Status |
| --- | ---------------------------------------------------- | ------------ | -------- | --------------------------------------------- | ------ |
| 1   | Add pool-saturation alerting to sample-be-service    | Backend Team | P0       | plans/backlog/2025-01-15\_\_db-pool-alerting/ | Open   |
| 2   | Document DB pool tuning procedure in service runbook | Backend Team | P1       | —                                             | Open   |
```

`Ticket` must be a `plans/` folder reference or an issue id. Use `—` only if the item has not
yet been promoted to a plan — do not leave it empty permanently.

#### 12. What Went Well

Include things that limited impact and places where the team got lucky. "Where we got lucky"
is important: luck is a latent risk to address, not a thing to celebrate silently.

#### 13. Lessons Learned

Distill the key insights from this incident that generalize beyond the immediate fix. Keep it
concise — two to five bullets. These should inform monitoring strategy, design decisions, or
operating procedures going forward.

#### 14. References

Links to logs, dashboards, related plans, related post-mortems, or external sources consulted.

### Optional Sections

These sections are encouraged when they add clarity:

- **Background** — relevant system context a reader outside the incident would need
- **Supporting Data** — graphs, log excerpts, metrics snapshots (use Mermaid or fenced code
  blocks; never paste raw secrets or credential material)

**Placement**: `Background` may appear **before Summary** when substantial up-front context is
required to understand the incident; otherwise place optional sections after References. Their
placement is flexible — clarity for the reader wins.

### Severity Scale (Authoritative)

Every post-mortem must classify the incident with one of the following tiers. This scale is
the single source of truth for incident severity in this repository.

| Tier      | Label    | Definition                                                                                   |
| --------- | -------- | -------------------------------------------------------------------------------------------- |
| **Sev-1** | Critical | Data loss or prolonged total outage of a critical service                                    |
| **Sev-2** | Major    | Significant outage or degradation of a key service                                           |
| **Sev-3** | Moderate | Degraded or intermittent non-critical service; no data loss; self-heals or workaround exists |
| **Sev-4** | Minor    | Cosmetic or low-impact issue                                                                 |

Use the format `Sev-N — Label` in the metadata table, e.g., `Sev-3 — Moderate`.

### No Secrets Rule

Post-mortems are committed to git and become permanent record. Apply the
[No Secrets in Committed Files](../../development/quality/no-secrets-in-committed-files.md) rule
without exception.

Use placeholders for any sensitive identifier that appears in timelines, log excerpts, or
configuration references:

| Type                 | Example placeholder      |
| -------------------- | ------------------------ |
| API token or key     | `<api-token>`            |
| DB connection string | `<db-connection-string>` |
| Service host         | `<service-host>`         |
| Internal endpoint    | `<internal-endpoint>`    |
| Signing secret       | `<signing-secret>`       |

Name the placeholder and state where the real value lives (e.g., "stored in `.env.local` or
another gitignored location"). Never include the value itself.

### Diagrams

Use accessible Mermaid diagrams (color-blind-safe palette) where they clarify causal chains
or triage sequences. A well-placed diagram costs less than a missed ambiguity.

Follow the [Diagrams Convention](../formatting/diagrams.md) and the
[Color Accessibility Convention](../formatting/color-accessibility.md). Use only the verified
WCAG AA hex codes: `#0173B2` (blue), `#DE8F05` (orange), `#029E73` (teal), `#CC78BC` (purple),
`#CA9161` (brown), `#808080` (gray).

## Examples

### PASS: Correct filename

`docs/explanation/post-mortems/2025-01-15-sample-be-service-db-pool-exhaustion.md`

- Incident date prefix
- Affected service (`sample-be-service`)
- Short kebab-case failure description (`db-pool-exhaustion`)

**Worked example**: See the full illustration at
[`docs/explanation/post-mortems/2025-01-15-sample-be-service-db-pool-exhaustion.md`](../../../docs/explanation/post-mortems/2025-01-15-sample-be-service-db-pool-exhaustion.md)
for a complete application of this convention.

### FAIL: Incorrect patterns

| FAIL: Wrong                                      | Reason                                                        |
| ------------------------------------------------ | ------------------------------------------------------------- |
| `post-mortem-db-2025.md`                         | No incident date prefix; date is a year not ISO date          |
| `2025-01-15__sample-be__db-pool.md`              | Double underscores are for `plans/` folders, not post-mortems |
| `2025-01-15-SampleBE-DBPool.md`                  | Uppercase components                                          |
| `docs/how-to/post-mortems/db-pool-exhaustion.md` | Wrong Diátaxis tier; post-mortems are explanation, not how-to |

### PASS: Action item table

```markdown
| #   | Action                                                        | Owner        | Priority | Ticket                                        | Status |
| --- | ------------------------------------------------------------- | ------------ | -------- | --------------------------------------------- | ------ |
| 1   | Add pool-saturation alerting to sample-be-service             | Backend Team | P0       | plans/backlog/2025-01-15\_\_db-pool-alerting/ | Open   |
| 2   | Document DB pool tuning procedure in service runbook          | Backend Team | P1       | —                                             | Open   |
| 3   | Evaluate autoscaling policy to absorb deploy-time load spikes | Backend Team | P2       | —                                             | Open   |
```

### FAIL: Action item anti-patterns

| FAIL: Wrong        | Reason                                                |
| ------------------ | ----------------------------------------------------- |
| "Fix the database" | Not specific, not verb-led to a concrete outcome      |
| Owner = "Team"     | Too vague; name the specific role or team responsible |
| Ticket = (empty)   | Must be `—` or a real reference; blank is ambiguous   |
| All items P0       | Priority loses meaning if everything is P0            |

## Validation

A post-mortem is complete when:

- [ ] File is in `docs/explanation/post-mortems/` with a valid `YYYY-MM-DD-<system>-<short-failure>.md` name
- [ ] Frontmatter includes `doc_status` field
- [ ] Metadata table present immediately after H1
- [ ] All mandatory sections present in the specified order
- [ ] Severity classified using the authoritative scale
- [ ] Timeline uses absolute timestamps with timezone
- [ ] Root Cause is distinct from Trigger
- [ ] Every P0 action item has a Ticket reference (or `—` with a note that promotion is pending)
- [ ] No secret values in any field — placeholders used throughout
- [ ] Index entry added to `docs/explanation/post-mortems/README.md`
- [ ] `doc_status` advances to `closed` only after all P0 items resolve

## References

**In-repo**:

- [`docs/explanation/post-mortems/README.md`](../../../docs/explanation/post-mortems/README.md) — Writer-facing template and index
- [No Secrets in Committed Files](../../development/quality/no-secrets-in-committed-files.md) — Hard iron rule; applies in full to post-mortems
- [Diagrams Convention](../formatting/diagrams.md) — Mermaid syntax and accessibility rules
- [Color Accessibility Convention](../formatting/color-accessibility.md) — Verified WCAG AA palette
- [Timestamp Format](../formatting/timestamp.md) — UTC+7 WIB standard used in timelines
- [Diátaxis Framework](./diataxis-framework.md) — Why post-mortems belong in `docs/explanation/`
- [Plans Organization](./plans.md) — How to create a `plans/` entry for action items

**Industry sources** [Web-cited]:

- Allspaw, J. (2012). _Blameless PostMortems and a Just Culture_. Etsy Code as Craft.
  <https://www.etsy.com/codeascraft/blameless-postmortems>
- Beyer, B. et al. (2016). _Site Reliability Engineering_, Chapter 15: Postmortem Culture:
  Learning from Failure. Google. <https://sre.google/sre-book/postmortem-culture/>
- PagerDuty. _Postmortem Templates and Best Practices_.
  <https://postmortems.pagerduty.com/>
- Atlassian. _Incident Handbook: How to Run a Postmortem_.
  <https://www.atlassian.com/incident-management/postmortem>
