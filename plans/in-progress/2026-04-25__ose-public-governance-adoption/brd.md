# Business Rationale

## Business Goal

Adopt three governance improvements from `ose-public` that close gaps in `ose-primer`'s
agent behavior, documentation hygiene, and code quality tooling. These are generic
template-level improvements — not product features — that make `ose-primer` a more
reliable upstream for repos that clone it.

## Changes and Rationale

### A — git-push-default convention

**Gap**: `ose-primer` has `trunk-based-development.md`, `git-push-safety.md`, and
`pr-merge-protocol.md` but no explicit convention governing the _default push behavior_
for AI agents. The plan agents (`plan-maker`, `plan-checker`, `plan-fixer`) currently
have no stated rule against inserting unsolicited PR steps in delivery checklists.

**Impact of gap**: Agents may add `- [ ] Create PR` steps to checklists without being
asked, generating unnecessary branches and PR overhead on a TBD repo where direct push
to main is the norm.

**Value of adoption**: Codifies the explicit opt-in-PR rule with a responsibility table
for each plan agent. Prevents governance debt accumulation in delivery checklists. Aligns
with the Explicit Over Implicit and Simplicity Over Complexity principles.

### B — no-date-metadata convention

**Gap**: ~466 markdown files across `.claude/agents/`, `.claude/skills/`, `governance/`,
and `docs/` carry manual `- **Last Updated**: DATE` rows, `created:` frontmatter, or
`updated:` frontmatter. Some agent and skill template examples also show these fields,
teaching consumers to include them.

**Impact of gap**: Manual dates rot immediately after creation — no process keeps them
accurate, and git history already records this information with full precision. Template
examples with date fields produce clutter in consumer repos. Checking 466 files for
stale dates is not a scalable maintenance practice.

**Value of adoption**: Establishes git as the single source of truth for file age.
Removes ~880 stale metadata lines from the codebase. Updates template examples so
consumer repos do not inherit the anti-pattern.

### C — rhino-cli `docs validate-mermaid`

**Gap**: `ose-primer` governs Mermaid diagram structure via
`governance/conventions/formatting/diagrams.md` (label length, rank width, single
diagram per block) but has no automated enforcement. Rule compliance is manual and
fragile.

**Impact of gap**: Diagram violations (oversized node labels, overcrowded ranks, multiple
diagrams in one block) go undetected until a human reviewer notices. CI does not catch
structural rendering failures.

**Value of adoption**: Mechanical enforcement of the three diagram rules already
documented in the convention. Pre-push hook checks only changed `.md` files, keeping
latency low. `validate:mermaid` Nx target is cacheable, so clean runs cost nothing.
Adds 11-file `internal/mermaid` package (extractor, parser, graph, validator, reporter)
with full test coverage, and a `docs-validate-mermaid.feature` Gherkin spec consuming
the command's behavior.

## Affected Roles

- **Template consumers** (repos cloning ose-primer): inherit cleaner agents, no stale
  dates, and automatic diagram validation on first push after adoption.
- **AI agents** (`plan-maker`, `plan-checker`, `plan-fixer`): governed by explicit
  PR-opt-in rule.
- **Documentation authors**: get automated Mermaid feedback at push time.

## Success Metrics

- Zero agent files carry `- **Last Updated**: DATE` rows after Phase B.
- Zero governance/docs files carry `created:` or `updated:` frontmatter after Phase B.
- `nx run rhino-cli:test:quick` passes with ≥90% coverage after Phase C.
- `nx run rhino-cli:validate:mermaid` exits 0 on the repo's existing diagrams.
- `plan-checker` flags any delivery checklist with an unsolicited `- [ ] Create PR` step.

## Non-Goals

- Migrating ose-primer's own plans from five-doc to single-doc format.
- Adding other ose-public features not related to governance or rhino-cli.
- Changing the module path in `go.mod` (shared with ose-public by design).
- Backfilling Gherkin scenarios for existing rhino-cli commands.

## Risks

| Risk                                                                     | Likelihood | Mitigation                                                                                                   |
| ------------------------------------------------------------------------ | ---------- | ------------------------------------------------------------------------------------------------------------ |
| sed strips date fields from intentional examples in skill templates      | Medium     | Target SKILL.md files individually; verify by diff review before commit                                      |
| Mermaid parser in ose-primer hits a diagram that ose-public never tested | Low        | Run `validate:mermaid` on full repo after port; treat any violation as a fixture for the diagrams convention |
| Pre-push hook latency increase from mermaid scan on large push           | Low        | `--changed-only` flag limits scan to push range; Nx cache skips repeat runs                                  |
