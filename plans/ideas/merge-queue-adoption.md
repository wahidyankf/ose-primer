# Merge queue: make merge-precondition (c) hold under concurrent integration

One-line summary: adopt a merge queue so that "the branch is non-destructively up to date with
`origin/main`" is re-validated against the actual `main` a PR lands on, instead of against whatever
`main` looked like when the PR last went green — currently blocked because GitHub's native merge queue
is not offered to personal-account-owned repositories.

> Demoted from a full `backlog/` plan to a two-pager on 2026-08-05. The full plan carried five
> documents: a README with the blocking-discovery narrative, `brd.md` (business goal, three-repo
> parity posture, risk table), `prd.md` (five personas, user stories US-1 to US-7, Gherkin acceptance
> criteria AC-1 to AC-7), `tech-docs.md` (availability matrix, mechanism comparison table, the
> precondition-(c) reword surface, rollback, and open decisions MQ-1/MQ-2/MQ-3), `delivery.md`
> (Phases 0 through 8 with per-phase gates and a dependency DAG), and an empty `learnings.md`. All of
> that is compressed here; the delivery checklist, Gherkin, personas, and diagrams are dropped.

## Problem / context

The repo-wide default delivery mode is `worktree-to-pr`, and its stated rationale is maximum
parallelization — N independent units become N independent PRs that review, gate, and merge
independently, with the PR as the independent merge point. A PR merges only when all five hardened
merge preconditions (a)-(e) hold, and precondition (c) requires the branch to be up to date with the
latest `origin/main` at merge time, brought forward non-destructively if behind. A static, per-PR
"branch up to date" check cannot guarantee (c) under concurrency: PR-A and PR-B are each green against
base `X`; A merges, so `main` becomes `X+A`; B is now silently stale and can carry a semantic (not
textual) conflict that no per-PR check ever saw. The more the repo leans on its parallel-by-default
posture, the more often two PRs are ready at overlapping times — exactly the window (c) is weakest in.

A merge queue closes that window structurally: a ready PR is enqueued rather than merged, the queue
builds a speculative merge (the PR onto the current queue head), runs CI on that artifact via the
GitHub `merge_group` event, fast-forwards on pass, and auto-evicts on fail without touching `main`.
Each PR keeps its own queue entry, so the strict 1-PR ↔ 1-worktree model survives intact.

## Why now

Not now, and that is the finding worth preserving. The original work was dropped from a parent plan on
2026-07-23 when the maintainer reported being unable to find a merge-queue toggle in the repo's branch
settings. That report turned out to be factually correct rather than a UI-navigation mistake: GitHub
gates merge queue on **repository owner type**, not on visibility or plan tier. Live verification on
2026-07-23 returned `User` for all three repos then in scope:

```text
gh api repos/wahidyankf/ose-public  --jq '.owner.type'   → User
gh api repos/wahidyankf/ose-primer  --jq '.owner.type'   → User
gh api repos/wahidyankf/ose-private --jq '.owner.type'   → User
```

There is no toggle to find, on any of them. The unlock is organization ownership, which is a
significant human infrastructure decision (new billing entity, re-pointed remotes and CI credentials,
possible permission changes) well outside a CI-config-plus-docs plan's authority. So the honest state
is: the problem is real, the mechanism is understood, and the enabling condition is absent. The brief
exists so the work is one grep away the moment the ownership model changes for any reason.

## Prior art / precedents

- **GitHub-native merge queue** — speculative-merge CI, FIFO ordering, auto-eviction; no new vendor
  given the existing `gh` toolchain. Availability is stated as organization-owned repositories in
  [GitHub's GA announcement](https://github.blog/news-insights/product-news/github-merge-queue-is-generally-available/)
  and corroborated by [GitHub Community Discussion #51483](https://github.com/orgs/community/discussions/51483)
  (both web-cited, accessed 2026-07-23).
- **The `merge_group` event** — fires only once a pull request is added to a merge queue, per
  [GitHub Actions: Events that trigger workflows](https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#merge_group).
  This is the canonical basis for the claim that adding the trigger is inert until a queue exists.
- **Graphite's stack-aware queue** — CI once on the stack head with binary-search failure isolation;
  Ramp Engineering reported a "74% decrease in median time between merges, with engineers merging PRs
  up to 3x faster" in [Graphite's write-up](https://graphite.com/blog/the-first-stack-aware-merge-queue).
  Whether it works on personal-account-owned repos was never independently verified.
- **PR Merge Protocol and PR Review Quality Gate (repo-internal)** — the two documents that actually
  define and restate the (a)-(e) preconditions this brief proposes to touch:
  [pr-merge-protocol.md](../../repo-governance/development/workflow/pr-merge-protocol.md) and
  [pr-review-quality-gate.md](../../repo-governance/workflows/pr/pr-review-quality-gate.md).
- **Multi-repo governance parity (repo-internal)** — the posture of authoring shared CI plus governance
  scaffolding once and propagating it, established by
  [standardize-repo-toolchain-parity](../done/2026-06-14__standardize-repo-toolchain-parity/README.md)
  and [lint-safety-parity](../done/2026-06-12__lint-safety-parity/README.md).

## Proposed direction (sketch)

- **Investigate, do not assume.** Lead with the `gh api ... --jq '.owner.type'` probe per repo; a
  ruleset or branch-protection probe cannot distinguish "not offered" from "not yet configured", so
  owner type alone is conclusive for a `User`-owned repo.
- **Land the inert scaffolding regardless of the blocker.** Add the `merge_group` event to the `on:`
  block of the workflow whose checks are required for merge on `main` — in this repo that is
  `.github/workflows/pr-quality-gate.yml`, which today triggers only on `pull_request` (types
  `opened`, `synchronize`, `reopened`) and `push` to `main`. Reuse the existing `pull_request` job set
  so queued CI equals branch CI, and keep the change `actionlint`-clean.
- **Reword precondition (c)** so it is satisfiable by the queue's speculative merge where a queue is
  enabled, while retaining the manual non-destructive branch-up-to-date form as the fallback. The
  (a)-(e) lettering and preconditions (a), (b), (d), (e) stay verbatim.
- **Write a merge-queue operations doc** covering three interactions: the queue runs after the
  three-cycle PR-Review Maker→Fixer Cycle as an integration step and never a review step; `[AI]`
  automerge must mean "add to queue" rather than a direct merge; each PR keeps its own worktree and
  its own queue entry.
- **Keep enablement human-only.** An agent prepares the runbook and verifies afterward via `gh api`;
  an agent never changes repository security settings.

## Rough scope & non-goals

In scope: the per-repo availability matrix keyed on owner type; the `merge_group` CI trigger; the
precondition-(c) reword; the operations doc; a human enablement runbook bracketed by agent prep and
agent `gh api` verification; and cross-repo parity of the scaffolding with enablement conditional per
repo.

Out of scope, carried verbatim from the source plan:

- Any `apps/` or `libs/` runtime code — this is CI config plus governance docs only.
- The PR-reviewer decomposition, owned by the parent reviewer-discipline hardening work (see the
  [PR Reviewer-Discipline Convention](../../repo-governance/development/quality/pr-review-disciplines.md)).
- Provisioning a bot or GitHub-App identity — a separate idea.
- Changing any of the other four merge preconditions (a), (b), (d), (e).
- Deciding the ownership fork on the maintainer's behalf. Migrating to a GitHub organization is a
  significant human infrastructure decision and adopting a third-party queue is a vendor decision;
  the brief records the fork and a recommendation, never a pre-made choice.

## Risks & open questions

- **The ownership fork itself.** Four branches were recorded: migrate to a GitHub organization; adopt a
  third-party queue that does not require one; harden (c) with a lightweight non-queue guard such as an
  auto-rebase-before-merge check or a serialize-merges convention; or keep the queue deferred. Deferral
  was the recorded recommendation, since it forces no decision under time pressure and keeps the other
  three available. (open)
- **Does a third-party queue actually work on personal-account repos?** The "Graphite does not require
  an org" premise was never independently verified and needs a dedicated research pass before anyone
  commits to it. (open)
- **Does `gh pr merge --auto` enqueue reliably?** Independent reports (for example `cli/cli#5653`,
  "`gh pr merge --auto` does not work with merge queues") suggest the behavior is not uniform across
  `gh` versions and configurations. If it is unreliable, the `[AI]`-merges-by-default posture needs a
  different enqueue path. (open)
- **The reword surface is smaller in this repo than the source plan assumed.** The source enumerated
  five files restating (c). Verified here, only two carry the (a)-(e) prose:
  `repo-governance/workflows/pr/pr-review-quality-gate.md` (normative, under §Hardened Merge
  Preconditions) and `repo-governance/development/workflow/pr-merge-protocol.md`, which renders (c)
  four times — §The Rule, §Agent Workflow §Before Merging, and two worked-example blocks. `AGENTS.md`
  §Delivery Mode, `repo-governance/conventions/structure/plans.md` §Delivery Mode, and
  `repo-governance/workflows/plan/plan-quality-gate.md` all link to the normative definition rather
  than restating it, so they need no edit. Re-run the enumeration at promotion rather than trusting
  either list. (open)
- **Parity scope has moved.** The source plan scoped three repos; this repo's
  [Related Repositories reference](../../docs/reference/related-repositories.md) now names four
  siblings, adding `beaver-nest`. Whether the scaffolding propagates to all four needs deciding. (open)
- **Runner load.** A queue serializes integration, so it should mean fewer concurrent full-CI runs
  rather than more, but the existing concurrency groups should be reused rather than reinvented.
- The queue changes only how (c) is satisfied — no other precondition, and not the review loop.
- The operations doc target `repo-governance/development/workflow/merge-queue-operations.md` does not
  exist in this repo; it is a new file the promoted plan would create.

## What success looks like + promotion signal

Success means precondition (c) holds under concurrency and not merely serially: two concurrently-ready
`worktree-to-pr` PRs integrate through a queue with CI on each speculative merge result, a PR whose
queued CI fails is auto-evicted without breaking `main`, each PR remains an independent merge point,
and preconditions (a), (b), (d), (e) plus the (a)-(e) lettering are untouched with (c) retaining a
manual fallback wherever no queue guards the branch.

Promotion signal: re-run `gh api repos/<owner>/<repo> --jq '.owner.type'` and promote when any repo in
scope returns `Organization` instead of `User` — that single change unblocks everything downstream. Two
weaker signals also justify promotion: a verified confirmation that a third-party queue operates on
personal-account-owned repositories, or a decision to pursue the non-queue hardening branch, which
would need its own design work and would replace rather than extend this brief. Absent any of those,
"not promoted yet" is the correct state, and the deliberately inert `merge_group` trigger is not worth
landing on its own.
