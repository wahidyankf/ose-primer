# Technical Documentation — Adopt Dependency Bump Policy & Planning Workflow

## Architecture / Approach

This is a governance-layer adoption. The six-layer governance hierarchy
(Vision → Principles → Conventions → Development → Agents → Workflows) gains:

- one **Development** practice (`dependency-bump-policy.md`),
- one **Convention** amendment (`workflow-naming.md` gains the `planning` type),
- one **Agents** convention (`subagent-orchestration.md`),
- one **Workflows** document (`repo-dependency-bump-planning.md`),
- one **reference register** (`docs/reference/security-waivers.md`),
- plus a supporting code change in the `rhino-cli` naming validators so the new workflow filename
  is mechanically accepted.

All upstream content is adopted faithfully; only repo-specific references are adapted.

## Repo-Grounding: verified references (current commit)

All targets below were verified to **exist** via `Bash test -f` / `find` / `grep` at authoring time
unless marked `_New file_`.

### Referenced docs that already exist (link targets — no creation needed) [Repo-grounded]

- `repo-governance/principles/software-engineering/reproducibility.md`
- `repo-governance/principles/software-engineering/explicit-over-implicit.md`
- `repo-governance/principles/software-engineering/automation-over-manual.md`
- `repo-governance/principles/general/root-cause-orientation.md`
- `repo-governance/principles/general/deliberate-problem-solving.md`
- `repo-governance/principles/content/no-time-estimates.md`
- `repo-governance/development/workflow/reproducible-environments.md`
- `repo-governance/development/workflow/commit-messages.md`
- `repo-governance/development/workflow/trunk-based-development.md`
- `repo-governance/development/workflow/native-first-toolchain.md`
- `repo-governance/development/quality/ci-blocker-resolution.md`
- `repo-governance/conventions/structure/workflow-naming.md`
- `repo-governance/conventions/structure/plans.md`
- `repo-governance/conventions/writing/web-research-delegation.md`
- `repo-governance/conventions/formatting/linking.md`
- `repo-governance/workflows/plan/plan-establishment-execution.md`
- `repo-governance/workflows/plan/plan-execution.md`

### Files to CREATE [Repo-grounded as missing]

- `repo-governance/development/workflow/dependency-bump-policy.md` — _New file_
- `repo-governance/workflows/repo/repo-dependency-bump-planning.md` — _New file_
- `repo-governance/development/agents/subagent-orchestration.md` — _New file_
  (referenced by the planning workflow's concurrency cap; absent here — `grep` returned no match)
- `docs/reference/security-waivers.md` — _New file_ (policy: "create if missing")

### Validator source locations [Repo-grounded]

- `apps/rhino-cli-rust/src/commands/workflows.rs` line ~41:
  `const WORKFLOW_TYPES: &[&str] = &["quality-gate", "execution", "setup"];`
- `apps/rhino-cli-go/cmd/workflows_validate_naming.go` line ~17:
  `var workflowTypes = []string{"quality-gate", "execution", "setup"}`
- Convention enforcement regex in `workflow-naming.md`:
  `grep -vE -- '-(quality-gate|execution|setup)$'` (must become
  `'-(quality-gate|execution|setup|planning)$'`).

## ose-primer ecosystem inventory (for adapting the workflow's scope section) [Repo-grounded]

The upstream workflow names `ose-public` apps (`organiclever-be`, `ose-app-be`, `crane-cli`,
`ayokoding-cli`, …). These MUST be replaced with `ose-primer`'s actual manifests:

| Ecosystem      | Real manifests in `ose-primer`                                                                                                                                         |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| npm            | root `package.json` (`volta`: node `24.13.1`, npm `11.10.1`; `dependencies`/`devDependencies`), `apps/*/package.json`, `libs/*/package.json`, `.opencode/package.json` |
| Cargo          | `apps/crud-be-rust-axum/Cargo.toml`, `apps/rhino-cli-rust/Cargo.toml` + each app's `rust-toolchain.toml`                                                               |
| .NET           | `apps/crud-be-csharp-aspnetcore/**/*.csproj`, `apps/crud-be-fsharp-giraffe/**/*.fsproj`, and `global.json` in each of those two app roots                              |
| Go             | `apps/crud-be-golang-gin/go.mod`, `apps/rhino-cli-go/go.mod`, `libs/golang-commons/go.mod`                                                                             |
| Docker         | `apps/*/Dockerfile`, `infra/dev/**/Dockerfile*`                                                                                                                        |
| GitHub Actions | `.github/actions/setup-*/action.yml` input defaults, inline pins in `.github/workflows/*.yml`, third-party `uses:` references                                          |

Adaptation rule: keep the policy/workflow **procedure** verbatim; replace only the enumerated app
names, the `.opencode/` manifest reference (kept — `.opencode/package.json` exists), and the
`infra/` path shape (`infra/dev/**` here). Drop the upstream claim that ".NET SDK is not pinned via
global.json" — in `ose-primer` both .NET apps DO carry a `global.json`, so the workflow must
inventory them.

## Design Decisions

1. **Add `planning` as a first-class workflow type** (not rename the workflow to fit existing
   vocab). Rationale: faithful upstream propagation; the upstream type is `planning`. The token is
   added to the convention table, the enforcement regex, and both validators + their tests.
2. **Create the security-waivers register as a stub now** (rather than on first waiver). Rationale:
   the policy and workflow both link to it; a missing target would be a broken cross-link (AP-10).
   The stub documents the schema and records "no waivers yet."
3. **Adopt a dedicated `subagent-orchestration.md` convention** rather than redirect the reference
   to the existing `agent-workflow-orchestration.md`. Rationale: the planning workflow cites a
   specific "cap concurrent agents at 3" rule; a focused convention is the faithful target and is
   reusable by other multi-agent workflows.
4. **Policy/workflow stay vendor-neutral** per the
   [Governance Vendor-Independence Convention](../../../repo-governance/conventions/structure/governance-vendor-independence.md):
   no `.claude/`- or `.opencode/`-specific terminology in the governance bodies.
5. **The adopted workflow MUST be invocable in `ose-primer`** (explicit user requirement). Two
   adaptations are required for invocability, because every agent/workflow the workflow body
   references must resolve in this repo:
   - **Phase 5 interface gap**: ose-primer's
     [`plan-establishment-execution`](../../../repo-governance/workflows/plan/plan-establishment-execution.md)
     has **no `target-stage` input** — it explicitly places plans in `plans/in-progress/` and says
     "Do NOT create in backlog/". The upstream Phase 5 passes `target-stage: backlog`. Adaptation:
     the adopted workflow's Phase 5 invokes `plan-establishment-execution` as-is (lands the plan in
     `plans/in-progress/<identifier>/`), then **relocates** it to
     `plans/backlog/<YYYY-MM-DD>__<identifier>/` via `git mv` (backlog uses the date prefix per the
     [Plans Organization Convention](../../../repo-governance/conventions/structure/plans.md)), and
     updates the two plans index READMEs. The workflow body documents this verbatim so an invoking
     agent follows it deterministically.
   - **Referenced agents exist** [Repo-grounded]: `web-researcher` agent exists at
     `.claude/agents/web-researcher.md`; `plan-establishment-execution` and `plan-execution`
     workflows exist. No referenced primitive is missing.
   - **Discoverability**: the workflow is indexed in `repo-governance/workflows/repo/README.md`
     and the top-level workflows catalog so the user can invoke it by path
     (`repo-governance/workflows/repo/repo-dependency-bump-planning.md`), exactly as
     `plan-execution.md` is invoked.

## File-Impact Map

**Create**

- `repo-governance/development/workflow/dependency-bump-policy.md`
- `repo-governance/workflows/repo/repo-dependency-bump-planning.md`
- `repo-governance/development/agents/subagent-orchestration.md`
- `docs/reference/security-waivers.md`

**Edit**

- `repo-governance/conventions/structure/workflow-naming.md` (type table + regex + enforcement text)
- `apps/rhino-cli-rust/src/commands/workflows.rs` (+ any unit test asserting the suffix list)
- `apps/rhino-cli-go/cmd/workflows_validate_naming.go` (+ `workflows_validate_naming_test.go` help/message)
- `repo-governance/development/workflow/README.md` (index entry)
- `repo-governance/workflows/repo/README.md` (index entry)
- `repo-governance/workflows/README.md` (catalog entry, if present)
- `repo-governance/development/agents/README.md` (index entry)
- `docs/reference/README.md` (index entry)

## Dependencies

- `rhino-cli-rust` (Cargo) and `rhino-cli-go` (Go) toolchains must build/test locally — covered by
  Phase 0 `npm run doctor -- --fix`.

## Rollback

Each phase is an isolated set of additive files plus a small, reversible validator edit. Rollback =
`git revert` of the phase commit; no data migration, no manifest change, nothing stateful.

## Security Waivers

None. This plan introduces no dependency bump and therefore issues no Path C waiver. The
`security-waivers.md` register is created empty (schema only).
