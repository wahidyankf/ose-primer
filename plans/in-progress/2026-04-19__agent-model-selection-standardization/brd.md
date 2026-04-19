# Business Requirements Document

## Business Goal

Document budget-adaptive design, extend policy to dual-platform coverage, and correct
1 agent tier assignment that is over-budgeted (rubric-bound governance work incorrectly
on opus-inherit tier). Adopted from the equivalent plan in `ose-public`
(phases 1-6 completed there 2026-04-19); `ose-primer` has the same structural gaps.

## Background and Pain Points

### Pain Point 1: Budget-Adaptive Design Is Undocumented

Opus-tier agents intentionally omit the `model` field so the agent inherits the session's
active model. This is the **correct** and **intended** behavior:

- Max / Team Premium session → inherits Opus 4.7 (high-capability output)
- Pro / Standard / API session → inherits Sonnet 4.6 (matches user's budget)

The design decision is: agents should adapt to the user's purchasing decision rather than
force a specific model tier regardless of cost. A Pro-tier user who gets Sonnet-quality
output from `plan-maker` is getting behavior that matches their account — not a bug.

The problem is this design is **nowhere documented**. Someone reading the policy today sees
"omit the model field" with no explanation of why. The risk: a well-intentioned developer
adds `model: opus` to an opus-tier agent to "make it more explicit", unknowingly breaking
the budget-adaptive behavior and forcing Opus charges on users who chose a lower tier.

**Model resolution chain** (Claude Code agent documentation,
[anthropic.com/docs/claude-code/agents](https://anthropic.com/docs/claude-code/agents),
accessed 2026-04-19):
env var → caller parameter → frontmatter `model:` → session default (inherit)

Omitting the field uses step 4 (inherit). This is intentional and correct.

### Pain Point 2: OpenCode Mapping Undocumented

`model-selection.md` covers Claude Code only. The OpenCode GLM model equivalents
(`zai-coding-plan/glm-5.1`, `zai-coding-plan/glm-5-turbo`), their capability mapping, and
the 3-to-2 tier collapse (both `opus` and `sonnet` collapse to `glm-5.1` in OpenCode) are
nowhere in the policy documentation. Developers adding new agents or reviewing agent tiers
have no authoritative source for OpenCode model selection.

### Pain Point 3: Model Version References Are Stale

`model-selection.md` contains no Claude 4.x model IDs and no reference to Haiku 3
retirement (retired 2026-04-19, the day this plan was created). Stale version references
erode trust in the policy document.

### Pain Point 4: CLAUDE.md Lacks Inline Plan Format Guidance

The Plans Organization section in `CLAUDE.md` points to `governance/conventions/structure/plans.md`
without describing the 5-doc format inline. The `plan-maker` agent and human developers
must navigate to the convention file to learn how many documents to create and what to
name them — creating unnecessary indirection and increasing the risk of format errors on
the first attempt.

The correct format (five documents: README.md, brd.md, prd.md, tech-docs.md, delivery.md)
should be visible inline in `CLAUDE.md`, the same way the parent-repo `ose-projects/CLAUDE.md`
documents it.

### Pain Point 5: Benchmark Data Undocumented — Tier Rationale Unverifiable

No authoritative benchmark reference exists in the project. The `model-selection.md`
policy states tier assignments but provides no benchmark numbers to support them. A
maintainer reading the policy cannot verify whether, say, Sonnet 4.6 is genuinely
adequate for checker work or whether Haiku 4.5 is capable enough for deployer tasks.

The research for this plan (conducted 2026-04-19 in `ose-public`) surfaced important
constraints:

- GLM-5-turbo (OpenCode fast tier) has **no published standard benchmark scores** — no
  SWE-bench, GPQA, MMLU, or HumanEval data exist as of April 2026. Its use is a
  platform constraint, not a benchmark-validated choice.
- GLM-5.1 (OpenCode top tier) scores (SWE-bench Pro 58.4) are **self-reported by Z.ai**
  with no independent third-party replication confirmed as of April 2026 (Awesome Agents
  review, 2026-04-17).
- Claude model scores are well-cited from Anthropic's official release posts and system
  cards, but no single project file links to these sources.

Additionally, `repo-rules-maker` is on opus-inherit tier despite performing rubric-bound
governance work (convention creation following the six-layer hierarchy templates) — not
open-ended creative reasoning. Sonnet 4.6 is fully capable for this task.

## Business Impact

| Affected Role                        | Impact today                                                                  | Impact after fix                      |
| ------------------------------------ | ----------------------------------------------------------------------------- | ------------------------------------- |
| Maintainer on Pro plan               | 1 agent incorrectly on opus-inherit tier, wasting budget on rubric-bound work | Correct tier guaranteed               |
| Maintainer adding new agent          | No OpenCode model docs to reference                                           | Clear dual-platform policy            |
| `plan-maker` creating a new plan     | Must navigate to convention doc for format; risk of wrong file count          | Format visible inline in CLAUDE.md    |
| Any reviewer auditing agents         | Cannot verify OpenCode tier from policy                                       | Single authoritative reference        |
| Any reviewer auditing tier rationale | Cannot verify tier choice without external research                           | Benchmark reference with cited scores |

## Affected Roles

- **Maintainer (all hats)** — affected by over-budgeted tier and undocumented policy
- **`plan-maker` agent** — reads CLAUDE.md to determine plan format; currently must
  follow an external link to learn the 5-doc structure
- **`agent-maker` agent** — reads model-selection.md when creating new agents; gets
  incomplete info (no OpenCode coverage)
- **`repo-rules-checker` agent** — validates model selection compliance; policy gaps
  reduce check quality

## Business-Level Success Metrics

1. **Observable**: `model-selection.md` contains a "Budget-Adaptive Inheritance" section
   explaining why opus-tier agents omit the `model` field
2. **Observable**: `npm run validate:claude` exits 0 after all changes
3. **Observable**: `model-selection.md` diff contains an "OpenCode / GLM Equivalents"
   section and a "Current Model Versions" table
4. **Observable**: `CLAUDE.md` Plans Organization section includes inline 5-doc format
   description (README.md, brd.md, prd.md, tech-docs.md, delivery.md)
5. **Observable**: `docs/reference/ai-model-benchmarks.md` exists and every benchmark
   number cites a source URL, date, and confidence level
6. **Observable**: `repo-rules-maker` model field changes from omit to `sonnet`;
   opus-inherit count drops from 15 to 14
7. _Judgment call:_ documentation now prevents a maintainer from accidentally adding
   `model: opus` or over-assigning expensive tiers to rubric-bound agents

## Business-Scope Non-Goals

- This does not change any agent's cognitive task category or purpose — only the 1 model
  tier where the decision tree clearly indicates over-budgeting
- This is not a rhino-cli source code change — the tooling already supports `model: opus`
- This is not an agent capability expansion — no new agents, no new features
- This does not address the GLM-5.1 capability gap vs Claude Opus 4.7 — that is a
  platform-level constraint outside this plan's scope

## Business Risks and Mitigations

| Risk                                                                        | Likelihood | Impact     | Mitigation                                                                    |
| --------------------------------------------------------------------------- | ---------- | ---------- | ----------------------------------------------------------------------------- |
| rhino-cli test fixtures hard-code empty model and fail after sonnet change  | Low        | Low        | Update fixture files (not source) if needed                                   |
| An agent missed in the audit retains wrong tier                             | Low        | Low        | `validate:claude` + grep audit catch it                                       |
| CLAUDE.md update conflicts with concurrent plans                            | Low        | Low        | Only one in-progress plan active                                              |
| Stale model-selection.md wording surfaces in agent outputs until re-indexed | Negligible | Negligible | Agents re-read files per invocation; no caching                               |
| GLM-5-turbo capability gap worsens for haiku-tier agents in OpenCode        | Low        | Low        | Acceptable platform constraint; documented in benchmark reference             |
| GLM-5.1 self-reported scores overstated                                     | Low        | Low        | Documented with confidence labels; does not affect Claude-session assignments |
