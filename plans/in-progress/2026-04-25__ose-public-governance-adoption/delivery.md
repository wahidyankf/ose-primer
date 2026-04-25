# Delivery Checklist

## Phase 1 — Change A: git-push-default Convention

- [ ] Create `governance/development/workflow/git-push-default.md` (adapt from ose-public,
      remove ose-public-specific content)
- [ ] Update `governance/development/workflow/README.md` — add git-push-default entry
- [ ] Update `.claude/agents/plan-maker.md` — add no-unsolicited-PR rule in checklist
      authoring section
- [ ] Update `.claude/agents/plan-checker.md` — add HIGH finding for unsolicited PR step
- [ ] Update `.claude/agents/plan-fixer.md` — add rule to remove unsolicited PR steps
- [ ] Update `governance/workflows/plan/plan-execution.md` — add rebase + opt-in-PR rules
- [ ] Update `CLAUDE.md` — add reference to git-push-default convention

## Phase 2 — Change B: no-date-metadata Convention

- [ ] Create `governance/conventions/writing/no-date-metadata.md`
- [ ] Update `governance/conventions/writing/README.md` — add no-date-metadata entry
- [ ] Update `CLAUDE.md` — add No Date Metadata to Key Conventions section
- [ ] Update `.claude/agents/docs-tutorial-maker.md` — remove `created:` / `updated:`
      from frontmatter template example
- [ ] Update `.claude/agents/docs-maker.md` — remove "Use for both created and updated
      fields" instruction
- [ ] Update `.claude/skills/agent-developing-agents/SKILL.md` — remove
      `- **Last Updated**: YYYY-MM-DD` template lines (~2 occurrences)
- [ ] Update `.claude/skills/repo-defining-workflows/SKILL.md` — remove `created:` /
      `updated:` from workflow frontmatter template
- [ ] Run Pass 1: strip `- **Last Updated**: DATE` rows from all `.claude/agents/` files
- [ ] Run Pass 2: strip `- **Last Updated**: DATE` rows from all `.claude/skills/` files
- [ ] Run Pass 3: strip `created:` / `updated:` frontmatter from `governance/` files
- [ ] Run Pass 4: strip `**Last Updated**: DATE` footer lines from `governance/` files
- [ ] Run Pass 5: strip `created:` / `updated:` frontmatter from `docs/` files
- [ ] Run Pass 6: strip `**Last Updated**: DATE` footer lines from `docs/` files
- [ ] Verify: grep for residual date metadata returns 0 matches
- [ ] Verify: grep for `YYYY-MM-DD` placeholders in `.claude/` skill/agent templates

## Phase 3 — Change C: rhino-cli docs validate-mermaid

- [ ] Create `apps/rhino-cli/internal/mermaid/` directory with 11 Go files from ose-public
      (types.go, extractor.go, extractor_test.go, parser.go, parser_test.go, graph.go,
      graph_test.go, validator.go, validator_test.go, reporter.go, reporter_test.go)
- [ ] Create `apps/rhino-cli/cmd/docs_validate_mermaid.go`
- [ ] Create `apps/rhino-cli/cmd/docs_validate_mermaid_test.go`
- [ ] Create `apps/rhino-cli/cmd/docs_validate_mermaid_helpers_test.go`
- [ ] Create `apps/rhino-cli/cmd/docs_validate_mermaid.integration_test.go`
- [ ] Update `apps/rhino-cli/cmd/testable.go` — add 4 injectable vars + mermaid import
- [ ] Update `apps/rhino-cli/project.json` — add `validate:mermaid` Nx target
- [ ] Update `.husky/pre-push` — add mermaid check in md-files branch
- [ ] Create `specs/apps/rhino/cli/gherkin/docs-validate-mermaid.feature`
- [ ] Update `specs/apps/rhino/cli/gherkin/README.md` — add feature entry
- [ ] Update `apps/rhino-cli/README.md` — add docs validate-mermaid command
- [ ] Update `governance/conventions/formatting/diagrams.md` — add automated enforcement
      note

## Phase 4 — Verification

- [ ] `CGO_ENABLED=0 go build -C apps/rhino-cli ./...` — build passes
- [ ] `npx nx run rhino-cli:test:quick` — unit tests pass with ≥90% coverage
- [ ] `npx nx run rhino-cli:validate:mermaid` — exits 0 (fix any pre-existing violations)
- [ ] Verify: `grep -rn "^- \*\*Last Updated\*\*:" .claude/agents/ .claude/skills/ | wc -l`
      → 0
- [ ] Verify: `grep -rn "^created: \|^updated: " governance/ docs/ | wc -l` → 0

## Phase 5 — OpenCode Sync + Lint + Commit

- [ ] `npm run sync:claude-to-opencode`
- [ ] `npm run lint:md` — fix any violations with `npm run lint:md:fix`
- [ ] Commit Change A: `feat(governance): add git-push-default convention and update plan agents`
- [ ] Commit Change B: `feat(governance): add no-date-metadata convention and strip all manual dates`
- [ ] Commit Change C: `feat(rhino-cli): port docs validate-mermaid with internal/mermaid package`
- [ ] `git push origin main`
