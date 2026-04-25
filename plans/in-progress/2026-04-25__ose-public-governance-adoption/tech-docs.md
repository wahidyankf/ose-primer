# Technical Approach

## Architecture

Three independent changes shipped sequentially. Each has its own file set.
No cross-change dependencies except the final OpenCode sync (applies after all
`.claude/` edits).

## Module Path Note

`ose-primer/apps/rhino-cli/go.mod` declares:

```
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

```
Delivery checklists MUST NOT include a `- [ ] Create PR` step (or any equivalent)
unless the user's original prompt or the plan's prd.md/README.md explicitly requests
a pull request. See [git-push-default convention](../../governance/development/workflow/
git-push-default.md).
```

**`.claude/agents/plan-checker.md`**

Add to the Delivery section a HIGH finding rule:

```
- Unsolicited PR step: delivery.md contains a `- [ ] Create PR` or `- [ ] Open PR`
  step with no explicit PR instruction in the user prompt or plan document. → HIGH
```

**`.claude/agents/plan-fixer.md`**

Add to the fix rules:

```
- Remove unsolicited `- [ ] Create PR` / `- [ ] Open PR` steps from delivery.md when
  no explicit PR instruction exists in the prompt or plan.
```

**`governance/workflows/plan/plan-execution.md`**

Before the push step, add:

```
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

Remove `- **Last Updated**: YYYY-MM-DD` template lines (appear at lines ~414 and ~818).

**`.claude/skills/repo-defining-workflows/SKILL.md`**

Remove `created: YYYY-MM-DD` and `updated: YYYY-MM-DD` from the workflow frontmatter
template block (lines ~41-42).

**`.claude/agents/docs-maker.md`**

Remove or update the instruction around line 264: "Use for both `created` and `updated`
fields when creating new docs" — this guidance contradicts the new convention.

### Mass Mechanical Cleanup

Run the following sed passes. Execute each independently so failures are isolated.

**Pass 1 — strip `- **Last Updated**: DATE` rows from agent and skill files:**

```bash
# Agents
find /Users/wkf/ose-projects/ose-primer/.claude/agents -name "*.md" \
  -exec sed -i '' '/^- \*\*Last Updated\*\*: /d' {} \;

# Skills (SKILL.md files and README)
find /Users/wkf/ose-projects/ose-primer/.claude/skills -name "*.md" \
  -exec sed -i '' '/^- \*\*Last Updated\*\*: /d' {} \;
```

**Pass 2 — strip `created:` / `updated:` frontmatter from governance:**

```bash
find /Users/wkf/ose-projects/ose-primer/governance -name "*.md" \
  -exec sed -i '' '/^created: /d; /^updated: /d' {} \;
```

**Pass 3 — strip standalone `**Last Updated**: DATE` footer lines from governance:**

```bash
find /Users/wkf/ose-projects/ose-primer/governance -name "*.md" \
  -exec sed -i '' 's/^\*\*Last Updated\*\*: .*$//' {} \;
```

**Pass 4 — strip `created:` / `updated:` frontmatter from docs:**

```bash
find /Users/wkf/ose-projects/ose-primer/docs -name "*.md" \
  -exec sed -i '' '/^created: /d; /^updated: /d' {} \;
```

**Pass 5 — strip standalone `**Last Updated**: DATE` footer lines from docs:**

```bash
find /Users/wkf/ose-projects/ose-primer/docs -name "*.md" \
  -exec sed -i '' 's/^\*\*Last Updated\*\*: .*$//' {} \;
```

**Verification after all passes:**

```bash
# Should return 0
grep -rn "^- \*\*Last Updated\*\*:" \
  /Users/wkf/ose-projects/ose-primer/.claude/agents/ \
  /Users/wkf/ose-projects/ose-primer/.claude/skills/ | wc -l

grep -rn "^created: \|^updated: " \
  /Users/wkf/ose-projects/ose-primer/governance/ \
  /Users/wkf/ose-projects/ose-primer/docs/ | wc -l
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
