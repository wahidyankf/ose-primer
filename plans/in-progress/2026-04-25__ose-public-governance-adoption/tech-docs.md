# Technical Approach

## Architecture

Three independent changes shipped sequentially. Each has its own file set.
No cross-change dependencies except the final OpenCode sync (applies after all
`.claude/` edits).

## Module Path Note

`ose-primer/apps/rhino-cli/go.mod` declares:

```go
module github.com/wahidyankf/ose-public/apps/rhino-cli
```

This is intentional — ose-primer shares the same module path as ose-public. No import
path changes are needed when porting Go files from ose-public.

---

## Change A: git-push-default Convention

### Files to Create

**`governance/development/workflow/git-push-default.md`**

Adapt from `ose-public` commit `9abf43f4a`. Identical substance; remove any
ose-public-specific wording (app names, subrepo worktree guidance). Keep:

- Default push = direct to `main`, no PR unless prompt or plan doc explicitly requests.
- PR is opt-in (phrases that constitute explicit instruction).
- Plans must not include unsolicited PR steps.
- Linear history via `git pull --rebase origin main` before every push.
- Proactive fix of preexisting violations.
- Agent responsibility table (plan-maker, plan-checker, plan-fixer, plan-execution
  workflow).
- PASS/FAIL examples for each standard.
- Related docs section.

### Files to Update

**`governance/development/workflow/README.md`**

Add entry in the Documents section:

```markdown
- [Git Push Default](./git-push-default.md) — Default push behavior convention.
  Direct push to main; PR creation is opt-in. Governs plan-maker, plan-checker,
  plan-fixer, and plan-execution workflow.
```

**`.claude/agents/plan-maker.md`**

Add to the delivery checklist authoring rules section a paragraph:

```text
Delivery checklists MUST NOT include a `- [ ] Create PR` step (or any equivalent)
unless the user's original prompt or the plan's prd.md/README.md explicitly requests
a pull request. See [git-push-default convention](../../governance/development/workflow/
git-push-default.md).
```

**`.claude/agents/plan-checker.md`**

Add to the Delivery section a HIGH finding rule:

```text
- Unsolicited PR step: delivery.md contains a `- [ ] Create PR` or `- [ ] Open PR`
  step with no explicit PR instruction in the user prompt or plan document. → HIGH
```

**`.claude/agents/plan-fixer.md`**

Add to the fix rules:

```text
- Remove unsolicited `- [ ] Create PR` / `- [ ] Open PR` steps from delivery.md when
  no explicit PR instruction exists in the prompt or plan.
```

**`governance/workflows/plan/plan-execution.md`**

Before the push step, add:

```text
Rebase before push to maintain linear history: `git pull --rebase origin main`. Never
create a merge commit. Do not open a PR unless the active delivery checklist contains
an explicit `- [ ] Create PR` step that satisfies the git-push-default convention.
```

**`CLAUDE.md`**

In the Git Workflow section, add reference after trunk-based-development:

```markdown
**See**: [git-push-default convention](./governance/development/workflow/git-push-default.md) —
explicit opt-in-PR rule for plan agents.
```

---

## Change B: no-date-metadata Convention

### Files to Create

**`governance/conventions/writing/no-date-metadata.md`**

New convention document covering:

- `created:` and `updated:` frontmatter fields are forbidden in non-website markdown.
- `- **Last Updated**: DATE` metadata rows in agent/skill files are forbidden.
- Inline `**Created**: DATE` / `**Updated**: DATE` body annotations are forbidden.
- Git history (`git log --follow -1 --pretty=%ai <file>`) is the authoritative source.
- Website content exception: not applicable to ose-primer (no CMS website app).
- Proactive fix: when encountering a file with date metadata during any task, strip it.
- Related docs section.

### Files to Update

**`governance/conventions/writing/README.md`**

Add entry:

```markdown
- [No Date Metadata](./no-date-metadata.md) — Forbids manual date fields in markdown.
  Git history is the single source of truth for file age.
```

**`CLAUDE.md`**

In the Key Conventions section, add:

```markdown
### No Date Metadata

Manual `created:`/`updated:` frontmatter and `**Last Updated**` rows are forbidden in
non-website markdown files. Use `git log --follow -1 --pretty=%ai <file>` for dates.

**See**: [governance/conventions/writing/no-date-metadata.md](./governance/conventions/writing/no-date-metadata.md)
```

### Template Examples to Update

**`.claude/agents/docs-tutorial-maker.md`**

Remove `created: YYYY-MM-DD` and `updated: YYYY-MM-DD` from the frontmatter template
block shown around lines 108-109.

**`.claude/skills/agent-developing-agents/SKILL.md`**

Remove both `- **Created**: YYYY-MM-DD` and `- **Last Updated**: YYYY-MM-DD` template
lines. Both appear in two Agent Metadata template blocks (lines ~413-414 and ~817-818).

**`.claude/skills/repo-defining-workflows/SKILL.md`**

Remove `created: YYYY-MM-DD` and `updated: YYYY-MM-DD` from the workflow frontmatter
template block (lines ~41-42).

**`.claude/agents/docs-maker.md`**

Remove or update the instruction around line 264: "Use for both `created` and `updated`
fields when creating new docs" — this guidance contradicts the new convention.

### Mass Mechanical Cleanup

Run the following sed passes from the repo root. Execute each independently so failures
are isolated.

**Pass 1 — strip `- **Last Updated**: DATE`and`- **Created**: DATE` rows from agent and skill files:**

```bash
# Agents — both Created and Last Updated rows
find .claude/agents -name "*.md" \
  -exec sed -i '' '/^- \*\*Last Updated\*\*: /d; /^- \*\*Created\*\*: /d' {} \;

# Skills (SKILL.md files and README) — same patterns
find .claude/skills -name "*.md" \
  -exec sed -i '' '/^- \*\*Last Updated\*\*: /d; /^- \*\*Created\*\*: /d' {} \;
```

**Pass 2 — strip `created:` / `updated:` frontmatter from governance:**

```bash
find governance -name "*.md" \
  -exec sed -i '' '/^created: /d; /^updated: /d' {} \;
```

**Pass 3 — strip standalone `**Last Updated**: DATE` footer lines from governance:**

```bash
find governance -name "*.md" \
  -exec sed -i '' 's/^\*\*Last Updated\*\*: .*$//' {} \;
```

**Pass 4 — strip `created:` / `updated:` frontmatter from docs:**

```bash
find docs -name "*.md" \
  -exec sed -i '' '/^created: /d; /^updated: /d' {} \;
```

**Pass 5 — strip standalone `**Last Updated**: DATE` footer lines from docs:**

```bash
find docs -name "*.md" \
  -exec sed -i '' 's/^\*\*Last Updated\*\*: .*$//' {} \;
```

**Verification after all passes:**

> Absolute paths below assume the repo root is `/Users/wkf/ose-projects/ose-primer`.
> Substitute your actual checkout path if different, or run from the repo root using
> relative paths (`./.claude/agents/`, `./governance/`, `./docs/`).

```bash
# Should return 0 — Last Updated and Created rows in agents/skills
grep -rn "^- \*\*Last Updated\*\*:\|^- \*\*Created\*\*:" \
  .claude/agents/ .claude/skills/ | wc -l

# Should return 0 — frontmatter in governance and docs
grep -rn "^created: \|^updated: " governance/ docs/ | wc -l
```

> **Note on template examples**: After the mass passes, grep for residual
> `YYYY-MM-DD` patterns in SKILL.md files to catch any template copy that
> survived (e.g. `created: YYYY-MM-DD` as placeholder). These are also
> convention violations — remove them.

```bash
grep -rn "YYYY-MM-DD" /Users/wkf/ose-projects/ose-primer/.claude/ --include="*.md"
```

---

## Change C: rhino-cli `docs validate-mermaid`

### Source Files (all from ose-public, no import path changes needed)

#### Internal package — copy verbatim

| Source path (ose-public)                            | Destination (ose-primer) |
| --------------------------------------------------- | ------------------------ |
| `apps/rhino-cli/internal/mermaid/types.go`          | same relative path       |
| `apps/rhino-cli/internal/mermaid/extractor.go`      | same relative path       |
| `apps/rhino-cli/internal/mermaid/extractor_test.go` | same relative path       |
| `apps/rhino-cli/internal/mermaid/parser.go`         | same relative path       |
| `apps/rhino-cli/internal/mermaid/parser_test.go`    | same relative path       |
| `apps/rhino-cli/internal/mermaid/graph.go`          | same relative path       |
| `apps/rhino-cli/internal/mermaid/graph_test.go`     | same relative path       |
| `apps/rhino-cli/internal/mermaid/validator.go`      | same relative path       |
| `apps/rhino-cli/internal/mermaid/validator_test.go` | same relative path       |
| `apps/rhino-cli/internal/mermaid/reporter.go`       | same relative path       |
| `apps/rhino-cli/internal/mermaid/reporter_test.go`  | same relative path       |

#### Command layer — copy verbatim

| Source path (ose-public)                                       | Destination (ose-primer) |
| -------------------------------------------------------------- | ------------------------ |
| `apps/rhino-cli/cmd/docs_validate_mermaid.go`                  | same relative path       |
| `apps/rhino-cli/cmd/docs_validate_mermaid_test.go`             | same relative path       |
| `apps/rhino-cli/cmd/docs_validate_mermaid_helpers_test.go`     | same relative path       |
| `apps/rhino-cli/cmd/docs_validate_mermaid.integration_test.go` | same relative path       |

#### `apps/rhino-cli/cmd/steps_common_test.go` — append const block with 30 step constant declarations (~34 lines including wrapper and comment)

`steps_common_test.go` already exists in ose-primer. Do NOT copy the whole file.
Append the following block (taken verbatim from ose-public commit `4c8397b88`) at the
end of the file, after the last existing `const` block:

```go
// Docs validate-mermaid step patterns.
const (
    stepMermaidFileCleanFlowchart                      = `^a markdown file containing a flowchart where every node label is within the limit$`
    stepMermaidFileLabelTooLong                        = `^a markdown file containing a flowchart with a node label longer than the limit$`
    stepMermaidFileNodeLabel35Chars                    = `^a markdown file containing a flowchart with a node label of 35 characters$`
    stepMermaidFileTBChainedSequentially               = `^a markdown file containing a TB flowchart with 10 nodes chained sequentially$`
    stepMermaidFileTBNoRankMoreThan3                   = `^a markdown file containing a TB flowchart where no rank has more than 3 nodes$`
    stepMermaidFileTBOneRank4Nodes                     = `^a markdown file containing a TB flowchart where one rank has 4 parallel nodes$`
    stepMermaidFileLRNoRankMoreThan3                   = `^a markdown file containing an LR flowchart where no rank has more than 3 nodes$`
    stepMermaidFileLR4NodesSameDepth                   = `^a markdown file containing an LR flowchart where one rank has 4 nodes at the same depth$`
    stepMermaidFileFlowchart4NodesOneRank              = `^a markdown file containing a flowchart with 4 nodes at one rank$`
    stepMermaidFile4NodesMoreThan5Ranks                = `^a markdown file containing a flowchart with 4 nodes at one rank and more than 5 ranks deep$`
    stepMermaidFile4NodesExactly4RanksDeep             = `^a markdown file containing a flowchart with 4 nodes at one rank and exactly 4 ranks deep$`
    stepMermaidFileSingleFlowchart                     = `^a markdown file containing a mermaid code block with exactly one flowchart diagram$`
    stepMermaidFileTwoFlowchartDeclarations            = `^a markdown file containing a mermaid code block with two flowchart declarations$`
    stepMermaidFileGraphKeywordNoViolations            = `^a markdown file containing a mermaid block using the graph keyword instead of flowchart with no violations$`
    stepMermaidFileOnlyNonFlowchart                    = `^a markdown file containing only sequenceDiagram and classDiagram mermaid blocks$`
    stepMermaidFileNoMermaidBlocks                     = `^a markdown file containing no mermaid code blocks$`
    stepMermaidViolationNotStagedInGit                 = `^a markdown file with a mermaid violation that has not been staged in git$`
    stepMermaidViolationNotInPushRange                 = `^a markdown file with a mermaid violation that is not in the push range$`
    stepMermaidFileLabelLengthViolation                = `^a markdown file containing a flowchart with a label length violation$`
    stepMermaidFileNoViolations                        = `^a markdown file containing a flowchart with no violations$`
    stepDeveloperRunsDocsValidateMermaid               = `^the developer runs docs validate-mermaid$`
    stepDeveloperRunsDocsValidateMermaidMaxLabelLen40  = `^the developer runs docs validate-mermaid with --max-label-len 40$`
    stepDeveloperRunsDocsValidateMermaidMaxWidth5      = `^the developer runs docs validate-mermaid with --max-width 5$`
    stepDeveloperRunsDocsValidateMermaidMaxDepth3      = `^the developer runs docs validate-mermaid with --max-depth 3$`
    stepDeveloperRunsDocsValidateMermaidStagedOnly     = `^the developer runs docs validate-mermaid with the --staged-only flag$`
    stepDeveloperRunsDocsValidateMermaidChangedOnly    = `^the developer runs docs validate-mermaid with the --changed-only flag$`
    stepDeveloperRunsDocsValidateMermaidJSONOutput     = `^the developer runs docs validate-mermaid with -o json$`
    stepDeveloperRunsDocsValidateMermaidMarkdownOutput = `^the developer runs docs validate-mermaid with -o markdown$`
    stepDeveloperRunsDocsValidateMermaidVerbose        = `^the developer runs docs validate-mermaid with --verbose$`
    stepDeveloperRunsDocsValidateMermaidQuiet          = `^the developer runs docs validate-mermaid with --quiet$`
)
```

#### `apps/rhino-cli/cmd/testable.go` — add four lines

Append to the existing var block:

```go
var docsValidateMermaidFn = mermaid.ValidateBlocks

// readFileFn is a variable for dependency injection of os.ReadFile in tests.
var readFileFn = os.ReadFile

// getMermaidStagedFilesFn is injectable for unit tests (avoids real git call).
var getMermaidStagedFilesFn = getMermaidStagedFiles

// getMermaidChangedFilesFn is injectable for unit tests (avoids real git call).
var getMermaidChangedFilesFn = getMermaidChangedFiles
```

Also add the `mermaid` import to `testable.go`:

```go
"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/mermaid"
```

#### `apps/rhino-cli/project.json` — add validate:mermaid target

Add after the last existing target:

```json
"validate:mermaid": {
  "command": "CGO_ENABLED=0 go run -C apps/rhino-cli main.go docs validate-mermaid governance/ .claude/",
  "cache": true,
  "inputs": ["{projectRoot}/**/*.go", "{workspaceRoot}/governance/**/*.md", "{workspaceRoot}/.claude/**/*.md"],
  "outputs": []
}
```

#### `.husky/pre-push` — add mermaid check

In the block that runs when `.md` files are in the push range, add before the closing
`fi`:

```bash
  if echo "$CHANGED" | grep -qE '\.md$'; then
    npx nx run rhino-cli:validate:mermaid --args="--changed-only"
  fi
```

(Check the exact insertion point by reading the current pre-push hook — insert after
the existing markdown lint steps and before the final `fi`.)

#### `specs/apps/rhino/cli/gherkin/docs-validate-mermaid.feature`

Copy verbatim from ose-public commit `17b8a3a0d`. The `@docs-validate-mermaid` tag
matches the Nx spec-coverage expectations.

#### `specs/apps/rhino/cli/gherkin/README.md`

Add entry for `docs-validate-mermaid.feature` in the features table.

#### `apps/rhino-cli/README.md`

Add `docs validate-mermaid` to the commands table and usage examples. Mirror the
section added in ose-public commit `17b8a3a0d`.

#### `governance/conventions/formatting/diagrams.md`

After Rule 1 (label length) enforcement guidance, add the automated enforcement note
from ose-public commit `17b8a3a0d`:

```markdown
**Automated enforcement**: Run `rhino-cli docs validate-mermaid` to check these rules
mechanically instead of counting characters manually. Use `--max-label-len 20` to enforce
the 20-character Hugo/Hextra limit (the default is 30, matching Mermaid's `wrappingWidth`
baseline). The tool also checks parallel rank width (Rule 2 above) and
single-diagram-per-block.
```

### Pre-existing Mermaid violations

After porting, run:

```bash
cd /Users/wkf/ose-projects/ose-primer
npx nx run rhino-cli:validate:mermaid
```

Any violations in `governance/` or `.claude/` are pre-existing diagram errors that
must be fixed in this plan (not deferred). Record each violation and fix the offending
diagram inline.

### Build and test commands

```bash
cd /Users/wkf/ose-projects/ose-primer

# Verify build
CGO_ENABLED=0 go build -C apps/rhino-cli ./...

# Run unit tests with coverage
npx nx run rhino-cli:test:quick

# Run mermaid validation on full repo
npx nx run rhino-cli:validate:mermaid
```

---

## Dependencies

No new external Go dependencies are introduced. All new code uses packages already
present in `apps/rhino-cli/go.mod`:

- `github.com/spf13/cobra` — already used by all other cmd files
- Standard library (`fmt`, `io/fs`, `os/exec`, `path/filepath`, `strings`) — no new
  `require` entries needed

Run `go mod tidy -C apps/rhino-cli` after porting to confirm no dependency drift.

## Testing Strategy

| Level                            | Tool                                                  | Scope                                                                                                          | Cacheable                       |
| -------------------------------- | ----------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- | ------------------------------- |
| Unit (`test:unit`)               | `go test` + godog                                     | `internal/mermaid/*_test.go`, `cmd/docs_validate_mermaid_test.go`, `cmd/docs_validate_mermaid_helpers_test.go` | Yes                             |
| Integration (`test:integration`) | `go test -tags integration`                           | `cmd/docs_validate_mermaid.integration_test.go` — real filesystem via `t.TempDir()`, no Docker                 | Yes (no non-deterministic deps) |
| Coverage                         | `go test -coverprofile` via `rhino-cli test-coverage` | Same as unit                                                                                                   | —                               |

`test:quick` runs unit tests only plus coverage validation at ≥90%. Integration tests
run via `nx run rhino-cli:test:integration`.

## Rollback

If the mermaid port causes test failures or build errors that cannot be resolved:

1. `git revert <commit-sha-of-Change-C>` — reverts all mermaid files in one step.
2. Alternatively, delete `apps/rhino-cli/internal/mermaid/`, the four new cmd files,
   and revert `testable.go`, `steps_common_test.go`, `project.json`, `.husky/pre-push`,
   `specs/apps/rhino/cli/gherkin/docs-validate-mermaid.feature`.
3. Changes A and B (governance only) are independent and do not need rollback.

## OpenCode Sync

After all `.claude/agents/` and `.claude/skills/` edits:

```bash
cd /Users/wkf/ose-projects/ose-primer
npm run sync:claude-to-opencode
```

## Markdown Lint

After all markdown edits:

```bash
cd /Users/wkf/ose-projects/ose-primer
npm run lint:md
npm run lint:md:fix  # if violations exist
```

## Commit Strategy

Split into three commits, one per change group:

```
feat(governance): add git-push-default convention and update plan agents
feat(governance): add no-date-metadata convention and strip all manual dates
feat(rhino-cli): port docs validate-mermaid command with internal/mermaid package
```

Each commit stands alone and is independently revertable.
