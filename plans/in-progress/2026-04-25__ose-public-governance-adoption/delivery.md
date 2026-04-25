# Delivery Checklist

## Phase 0 — Environment Setup

- [x] Install dependencies: `npm install`
  - Date: 2026-04-25 | Status: done | npm install completed, audit warnings only (no blockers)
- [x] Converge the full polyglot toolchain: `npm run doctor -- --fix` (required — the
      `postinstall` hook runs `doctor || true` and silently tolerates drift)
  - Date: 2026-04-25 | Status: done | 19/19 tools OK, 0 warnings, 0 missing
- [x] Verify rhino-cli baseline build: `CGO_ENABLED=0 go build -C apps/rhino-cli ./...`
  - Date: 2026-04-25 | Status: done | Build passes cleanly
- [x] Verify existing tests pass before making changes:
      `npx nx run rhino-cli:test:quick`
  - Date: 2026-04-25 | Status: done | 90.02% coverage, all packages pass
- [x] Note any preexisting failures — they must be fixed as part of this plan
  - Date: 2026-04-25 | Status: done | No preexisting failures; cmd at 82.9% (aggregate passes threshold)

## Phase 1 — Change A: git-push-default Convention

- [x] Create `governance/development/workflow/git-push-default.md` (adapt from ose-public,
      remove ose-public-specific content)
  - Date: 2026-04-25 | Status: done | Files Changed: governance/development/workflow/git-push-default.md
- [x] Update `governance/development/workflow/README.md` — add git-push-default entry
  - Date: 2026-04-25 | Status: done | Added entry in Documents section
- [x] Update `.claude/agents/plan-maker.md` — add no-unsolicited-PR rule in checklist
      authoring section
  - Date: 2026-04-25 | Status: done | Added no-unsolicited-PR rule in Delivery Checklist Quality section
- [x] Update `.claude/agents/plan-checker.md` — add HIGH finding for unsolicited PR step
  - Date: 2026-04-25 | Status: done | Added unsolicited PR step HIGH finding to Delivery Checklist Validation
- [x] Update `.claude/agents/plan-fixer.md` — add rule to remove unsolicited PR steps
  - Date: 2026-04-25 | Status: done | Added Delivery Checklist Fixes section with unsolicited PR removal rule
- [x] Update `governance/workflows/plan/plan-execution.md` — add rebase + opt-in-PR rules
  - Date: 2026-04-25 | Status: done | Added rebase + no-unsolicited-PR text to Iron Rule 5
- [x] Update `CLAUDE.md` — add reference to git-push-default convention
  - Date: 2026-04-25 | Status: done | Added See reference after commit-messages.md link in Git Workflow section

## Phase 2 — Change B: no-date-metadata Convention

- [x] Create `governance/conventions/writing/no-date-metadata.md`
  - Date: 2026-04-25 | Status: done | Files Changed: governance/conventions/writing/no-date-metadata.md
- [x] Update `governance/conventions/writing/README.md` — add no-date-metadata entry
  - Date: 2026-04-25 | Status: done | Added entry in Documents section
- [x] Update `CLAUDE.md` — add No Date Metadata to Key Conventions section
  - Date: 2026-04-25 | Status: done | Added No Date Metadata section after Dynamic Collection References
- [x] Update `.claude/agents/docs-tutorial-maker.md` — remove `created:` / `updated:`
      from frontmatter template example
  - Date: 2026-04-25 | Status: done | Removed created/updated from YAML template and Required/Optional field lists; removed "Update frontmatter updated field" instruction
- [x] Update `.claude/agents/docs-maker.md` — remove "Use for both created and updated
      fields" instruction
  - Date: 2026-04-25 | Status: done | Removed date fields from frontmatter template and replaced Date Fields block with no-date-metadata pointer
- [x] Update `.claude/skills/agent-developing-agents/SKILL.md` — remove both
      `- **Created**: YYYY-MM-DD` and `- **Last Updated**: YYYY-MM-DD` template lines
      (2 Agent Metadata template blocks, each has both rows)
  - Date: 2026-04-25 | Status: done | Removed Created/Last Updated rows from both Agent Metadata template blocks (lines ~413-414 and ~817-818)
- [x] Update `.claude/skills/repo-defining-workflows/SKILL.md` — remove `created:` /
      `updated:` from workflow frontmatter template
  - Date: 2026-04-25 | Status: done | Removed created/updated lines from YAML frontmatter template block
- [x] Run Pass 1: strip `- **Last Updated**: DATE` and `- **Created**: DATE` rows from
      all `.claude/agents/` and `.claude/skills/` files (single sed pass covers both patterns)
  - Date: 2026-04-25 | Status: done | Stripped both patterns from all .claude/agents/ and .claude/skills/ .md files
- [x] Run Pass 2: strip `created:` / `updated:` frontmatter from `governance/` files
  - Date: 2026-04-25 | Status: done | Stripped created/updated frontmatter from all governance/ .md files
- [x] Run Pass 3: strip `**Last Updated**: DATE` footer lines from `governance/` files
  - Date: 2026-04-25 | Status: done | Stripped standalone **Last Updated** lines from all governance/ .md files
- [x] Run Pass 4: strip `created:` / `updated:` frontmatter from `docs/` files
  - Date: 2026-04-25 | Status: done | Stripped created/updated frontmatter from all docs/ .md files
- [x] Run Pass 5: strip `**Last Updated**: DATE` footer lines from `docs/` files
  - Date: 2026-04-25 | Status: done | Stripped standalone **Last Updated** lines from all docs/ .md files
- [x] Verify: grep for residual date metadata returns 0 matches
  - Date: 2026-04-25 | Status: done | Both grep counts = 0 ✓
- [x] Verify: grep for `YYYY-MM-DD` placeholders in `.claude/` skill/agent templates
  - Date: 2026-04-25 | Status: done | Remaining occurrences are all legitimate (report filename patterns, plan folder naming convention) — no date metadata templates remain

## Phase 3 — Change C: rhino-cli docs validate-mermaid

- [x] Create `apps/rhino-cli/internal/mermaid/` directory with 11 Go files from ose-public
      (types.go, extractor.go, extractor_test.go, parser.go, parser_test.go, graph.go,
      graph_test.go, validator.go, validator_test.go, reporter.go, reporter_test.go)
  - Date: 2026-04-25 | Status: done | Copied all 11 files verbatim from ose-public
- [x] Create `apps/rhino-cli/cmd/docs_validate_mermaid.go`
  - Date: 2026-04-25 | Status: done | Copied verbatim from ose-public
- [x] Create `apps/rhino-cli/cmd/docs_validate_mermaid_test.go`
  - Date: 2026-04-25 | Status: done | Copied verbatim from ose-public
- [x] Create `apps/rhino-cli/cmd/docs_validate_mermaid_helpers_test.go`
  - Date: 2026-04-25 | Status: done | Copied verbatim from ose-public
- [x] Create `apps/rhino-cli/cmd/docs_validate_mermaid.integration_test.go`
  - Date: 2026-04-25 | Status: done | Copied verbatim from ose-public
- [x] Update `apps/rhino-cli/cmd/steps_common_test.go` — append 30 step constant
      declarations for mermaid scenarios (do NOT copy whole file; append block only)
  - Date: 2026-04-25 | Status: done | Appended 30-constant mermaid block to end of file
- [x] Update `apps/rhino-cli/cmd/testable.go` — add 4 injectable vars + mermaid import
  - Date: 2026-04-25 | Status: done | Added mermaid import and 4 injectable vars (docsValidateMermaidFn, readFileFn, getMermaidStagedFilesFn, getMermaidChangedFilesFn)
- [x] Update `apps/rhino-cli/project.json` — add `validate:mermaid` Nx target
  - Date: 2026-04-25 | Status: done | Added validate:mermaid target with cache:true and correct inputs
- [x] Update `.husky/pre-push` — add mermaid check in md-files branch
  - Date: 2026-04-25 | Status: done | Added mermaid check after naming-workflows block; runs --changed-only when .md files in push range
- [x] Create `specs/apps/rhino/cli/gherkin/docs-validate-mermaid.feature`
  - Date: 2026-04-25 | Status: done | Copied verbatim from ose-public commit 17b8a3a0d
- [x] Update `specs/apps/rhino/cli/gherkin/README.md` — add feature entry
  - Date: 2026-04-25 | Status: done | Added docs-validate-mermaid.feature row (22 scenarios) to feature table
- [x] Update `apps/rhino-cli/README.md` — add docs validate-mermaid command
  - Date: 2026-04-25 | Status: done | Added docs validate-mermaid section with usage examples and what-it-does bullets
- [x] Update `governance/conventions/formatting/diagrams.md` — add automated enforcement
      note
  - Date: 2026-04-25 | Status: done | Added Automated enforcement paragraph after Rule 3 table

## Phase 4 — Verification

- [x] `CGO_ENABLED=0 go build -C apps/rhino-cli ./...` — build passes
  - Date: 2026-04-25 | Status: done | Build passes cleanly with mermaid package
- [x] `npx nx run rhino-cli:test:quick` — unit tests pass with ≥90% coverage
  - Date: 2026-04-25 | Status: done | 90.02% coverage, all packages pass; internal/mermaid at 97.0%
- [x] `npx nx run rhino-cli:validate:mermaid` — exits 0 (fix any pre-existing violations)
  - Date: 2026-04-25 | Status: done | Fixed 13 pre-existing violations across governance/ and .claude/ — pipe-label syntax, cycle-caused rank inflation, structural width; 0 violations remain (1 warning, warnings non-blocking)
- [x] Verify: `grep -rn "^- \*\*Last Updated\*\*:\|^- \*\*Created\*\*:" .claude/agents/ .claude/skills/ | wc -l`
      → 0
  - Date: 2026-04-25 | Status: done | Count = 0 ✓
- [x] Verify: `grep -rn "^created: \|^updated: " governance/ docs/ | wc -l` → 0
  - Date: 2026-04-25 | Status: done | Count = 0 ✓
- [x] `npx nx affected -t typecheck` — passes
  - Date: 2026-04-25 | Status: done | `npx nx run rhino-cli:typecheck` passes (go vet clean)
- [x] `npx nx affected -t lint` — passes
  - Date: 2026-04-25 | Status: done | `npx nx run rhino-cli:lint` passes (0 issues)
- [x] `npx nx affected -t test:quick` — passes
  - Date: 2026-04-25 | Status: done | `npx nx run rhino-cli:test:quick` passes (90.02% ≥ 90%)
- [x] `npx nx affected -t spec-coverage` — passes
  - Date: 2026-04-25 | Status: done | `npx nx run rhino-cli:spec-coverage` passes (18 specs, 126 scenarios, all covered)

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes. This follows the root cause orientation principle — proactively fix preexisting
> errors encountered during work.

## Phase 5 — OpenCode Sync + Lint + Commit

Commit thematically — each commit below is one coherent change. Follow Conventional
Commits format: `<type>(<scope>): <description>`. If during work you encounter unrelated
preexisting issues and fix them, commit those fixes separately first.

- [x] `npm run sync:claude-to-opencode`
  - Date: 2026-04-25 | Status: done | 45 agents converted, 32 skills copied
- [x] `npm run lint:md` — fix any violations with `npm run lint:md:fix`
  - Date: 2026-04-25 | Status: done | Fixed pre-existing MD012 violations in governance/workflows/ READMEs; lint clean after fix
- [x] Rebase before committing: `git pull --rebase origin main` (all changes must be
      staged or committed — no unstaged edits — before rebasing)
  - Date: 2026-04-25 | Status: done | Already up-to-date with origin/main
- [ ] Commit Change A: `feat(governance): add git-push-default convention and update plan agents`
- [ ] Commit Change B: `feat(governance): add no-date-metadata convention and strip all manual dates`
- [ ] Commit Change C: `feat(rhino-cli): port docs validate-mermaid with internal/mermaid package`
- [ ] `git push origin main`

> **Note**: `ose-primer` has no GitHub Actions (no org subscription). The Husky pre-push
> hook is the final quality gate. Verify the push completes without pre-push hook failures.
> If the hook fails, fix the issue and push again before proceeding to archival.

## Phase 6 — Plan Archival

- [ ] Verify ALL delivery checklist items above are ticked
- [ ] Verify ALL quality gates pass (Phase 4 local gates + Phase 5 pre-push hook)
- [ ] Move plan folder: `git mv plans/in-progress/2026-04-25__ose-public-governance-adoption plans/done/2026-04-25__ose-public-governance-adoption`
- [ ] Update `plans/in-progress/README.md` — remove the plan entry
- [ ] Update `plans/done/README.md` — add the plan entry with completion date
- [ ] Commit: `chore(plans): move ose-public-governance-adoption to done`
