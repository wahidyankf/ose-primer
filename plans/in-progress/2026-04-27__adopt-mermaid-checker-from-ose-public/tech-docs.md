---
title: Tech Docs — Adopt ose-public Mermaid Checker Enhancements
---

# Technical Approach

## Source of truth

Canonical reference: `ose-public/apps/rhino-cli/internal/mermaid/` and
`ose-public/apps/rhino-cli/cmd/docs_validate_mermaid*.go`.
Where ose-public uses the import path
`github.com/wahidyankf/ose-public/...`, ose-primer uses
`github.com/wahidyankf/ose-public/apps/rhino-cli/...` (already
present in the existing primer file).

This is a **structural port plus one primer-specific extension**:

- All seven Go source files (`internal/mermaid/{types,parser,
validator,reporter,extractor,graph}.go` plus `cmd/docs_validate_mermaid.go`)
  and their test counterparts come straight from ose-public,
  modulo import paths.
- The Nx target `validate:mermaid` in `apps/rhino-cli/project.json`
  is **broadened beyond ose-public's scope**. ose-public's target
  also restricts to `governance/` + `.claude/`; this plan widens
  it to `docs/`, `governance/`, `.claude/`, `plans/`, root `*.md`
  to satisfy the primer-side pre-push goal (FR-4).

## File-level port map

### Source files

| File                            | Action    | Diff size        | Notes                                                                                                                                                                                                                                                                                                                |
| ------------------------------- | --------- | ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `internal/mermaid/types.go`     | replace   | +24 added lines  | Add `Subgraph` struct, `WarningSubgraphDense` constant, `SubgraphLabel`/`SubgraphNodeCount`/`MaxSubgraphNodes` fields on `Warning`, `Subgraphs []Subgraph` on `ParsedDiagram`. Update package doc to mention four rules instead of three.                                                                            |
| `internal/mermaid/parser.go`    | replace   | +154 added lines | Add `subgraphHeaderRe` regex, stack-based subgraph tracking in `ParseDiagram`, `parseSubgraphHeader` helper, `snapshotKeys`/`newKeys`/`dedupOrder` helpers. New import: `slices`.                                                                                                                                    |
| `internal/mermaid/validator.go` | replace   | +38 added lines  | Add `MaxSubgraphNodes int` to `ValidateOptions`, default 6 in `DefaultValidateOptions`. Map raw `span`/`depth` to direction-aware `horizontal`/`vertical` and assign those to the warning's `ActualWidth`/`ActualDepth`. Append Rule 4 loop after Rule 2.                                                            |
| `internal/mermaid/reporter.go`  | replace   | +38 added lines  | Add `case WarningSubgraphDense` rendering for text format. Add `MaxSubgraphNodes`, `SubgraphLabel`, `SubgraphNodeCount` fields to JSON warning struct. Surface in markdown formatter.                                                                                                                                |
| `internal/mermaid/extractor.go` | unchanged | 0                | No change — extractor is identical.                                                                                                                                                                                                                                                                                  |
| `internal/mermaid/graph.go`     | unchanged | 0                | No change — rank/span/depth math is identical.                                                                                                                                                                                                                                                                       |
| `cmd/docs_validate_mermaid.go`  | replace   | +21 added lines  | Add `validateMermaidMaxSubgraphNodes int`, register `--max-subgraph-nodes 6` flag, plumb into `ValidateOptions`. Update `Long` description to mention four rules. Note: ose-public's `collectMDDefaultDirs` already includes `docs/` and `plans/`, so the CLI default scan widens automatically as part of the port. |

### Test files

| File                                            | Action             | Notes                                                                 |
| ----------------------------------------------- | ------------------ | --------------------------------------------------------------------- |
| `internal/mermaid/parser_test.go`               | replace            | Adds subgraph parsing tests.                                          |
| `internal/mermaid/validator_test.go`            | replace            | Adds Rule 4 tests, asserts direction-mapped values on warnings.       |
| `internal/mermaid/reporter_test.go`             | replace            | Adds `subgraph_density` rendering coverage.                           |
| `cmd/docs_validate_mermaid_test.go`             | replace            | Adds CLI flag wiring tests.                                           |
| `cmd/docs_validate_mermaid.integration_test.go` | replace            | Adds end-to-end fixture tests for the new rule.                       |
| `internal/mermaid/{extractor,graph}_test.go`    | unchanged          | No code change.                                                       |
| `cmd/docs_validate_mermaid_helpers_test.go`     | unchanged-or-touch | Only if helper signatures shift — diff against ose-public to confirm. |

### Configuration

- **`apps/rhino-cli/project.json`** — primer-specific extension
  beyond ose-public. ose-public's own `validate:mermaid` Nx target
  also restricts to `governance/ .claude/`; this plan widens the
  primer's target only. Pick option A from
  `## Nx target — coverage extension` below: drop positional args
  on the `command`, broaden `inputs` to list `docs/`, `governance/`,
  `.claude/`, `plans/`, root `*.md`. The CLI default scan already
  reaches the same five trees once the cmd port lands, so the new
  command (without args) and new `inputs` reach parity.
- **`.husky/pre-push`** — unchanged. The existing block that runs
  `npx nx run rhino-cli:validate:mermaid` whenever `git diff --name-only @{u}..HEAD`
  shows any `*.md` change already triggers correctly. Phase 10 of
  delivery.md verifies the hook still fires after the Nx target
  broadens.

## Algorithm: subgraph tracking

The ported parser uses a stack-based attribution pass. Pseudocode:

```text
stack := []
subgraphs := []

for each line in block.Source:
    if line starts with "subgraph":
        push Subgraph{ID, Label, StartLine} onto stack
        continue
    if line == "end":
        pop top of stack into subgraphs
        continue
    if line is header or empty:
        continue

    before := keys(nodeMap)
    if line has arrow:
        extractEdgeLine(line, nodeMap, edges)   # may add nodeMap entries
    else:
        extractStandaloneNode(line, nodeMap)    # may add a nodeMap entry
    new := keys(nodeMap) - before

    if stack non-empty AND new non-empty:
        attribute deduped(new) to stack.top.NodeIDs

# any stack entries left at EOF = unclosed subgraphs
while stack non-empty:
    pop into subgraphs
```

Net effect: any node ID introduced for the first time inside a
subgraph block is attributed to the innermost open subgraph.
Mentions of the same ID later (at any depth, inside or outside the
subgraph) do not re-attribute.

## Algorithm: direction-mapped warning fields

Validator change is local to the warning emit:

```go
var horizontal, vertical int
switch diagram.Direction {
case DirectionLR, DirectionRL:
    horizontal, vertical = depth, span
case DirectionTB, DirectionTD, DirectionBT:
    horizontal, vertical = span, depth
}
// ... warning emission uses horizontal/vertical, not span/depth
warnings = append(warnings, Warning{
    Kind:        WarningComplexDiagram,
    ActualWidth: horizontal,   // was: span
    ActualDepth: vertical,     // was: depth
    MaxWidth:    opts.MaxWidth,
    MaxDepth:    opts.MaxDepth,
    ...
})
```

The violation path is unchanged because the violation already
operated on `horizontal` (the post-mapping value).

## Algorithm: Rule 4 application

```go
if opts.MaxSubgraphNodes > 0 {
    for _, sg := range diagram.Subgraphs {
        if len(sg.NodeIDs) > opts.MaxSubgraphNodes {
            warnings = append(warnings, Warning{
                Kind:              WarningSubgraphDense,
                FilePath:          block.FilePath,
                BlockIndex:        block.BlockIndex,
                StartLine:         block.StartLine + sg.StartLine,
                SubgraphLabel:     sg.Label,
                SubgraphNodeCount: len(sg.NodeIDs),
                MaxSubgraphNodes:  opts.MaxSubgraphNodes,
            })
        }
    }
}
```

Threshold default 6. `MaxSubgraphNodes <= 0` disables the rule
entirely (used for opt-out scenarios where a dense cluster is
intentional).

## Nx target — coverage extension

Two viable shapes for the upgraded `validate:mermaid` command in
`apps/rhino-cli/project.json`. Pick option A; option B is the
fallback if option A surfaces `apps/`/`apps-labs/` markdown that
should never be scanned.

```json
"validate:mermaid": {
  "command": "CGO_ENABLED=0 go run -C apps/rhino-cli main.go docs validate-mermaid",
  "cache": true,
  "inputs": [
    "{projectRoot}/**/*.go",
    "{workspaceRoot}/docs/**/*.md",
    "{workspaceRoot}/governance/**/*.md",
    "{workspaceRoot}/.claude/**/*.md",
    "{workspaceRoot}/plans/**/*.md",
    "{workspaceRoot}/*.md"
  ],
  "outputs": []
}
```

Option A drops the explicit positional args and relies on the CLI
default scan. ose-public's `collectMDDefaultDirs` already enumerates
`docs/`, `governance/`, `.claude/`, `plans/`, and root `*.md` —
verified in source. So porting the cmd file alone widens the default
scan to all five trees automatically; no extra CLI edit needed.

Option B (fallback): keep argv style and pass
`governance/ .claude/ docs/ plans/` explicitly. Use only if option A
surfaces unwanted markdown in `apps/` or `apps-labs/` trees.

Pick option A. Update tests in `cmd/docs_validate_mermaid_test.go`
to assert all five default roots are visited.

## Repository remediation strategy

After the upgraded checker lands but before commit, run:

```bash
nx run rhino-cli:validate:mermaid > /tmp/mermaid-violations.txt 2>&1 || true
```

Triage every flagged diagram into one of three buckets.

1. **Fix in place** — shrink labels, split subgraphs into multiple
   smaller subgraphs, or restructure to drop direct-child count
   below 6. This is the default action.
2. **Justify and accept warning** — never. Warnings are non-blocking
   but the success bar for this plan is zero warnings on the full
   repo.
3. **Justify and accept violation** — never. Violations block CI.

Expected hot spots based on diagram inventory:

- AI primer RAG flowchart in
  `docs/explanation/software-engineering/ai-application-development/README.md`
  has Ingest and Query subgraphs — verify direct-child counts.
- BDD/TDD documentation under `docs/explanation/software-engineering/development/`
  contains tier diagrams — recheck after the upgrade.
- Programming-language docs in
  `docs/explanation/software-engineering/programming-languages/`
  may have dense pattern-collection diagrams.

## Test plan

- Unit tests run via `nx run rhino-cli:test:unit` after every source
  port. Coverage stays ≥ 90% (Go threshold).
- Integration tests run via `nx run rhino-cli:test:integration`
  after CLI flag wiring lands.
- Full-repo sweep `nx run rhino-cli:validate:mermaid` exits 0
  before commit.
- Mock pre-push run: `git push --dry-run` after staging, confirm
  the Husky hook fires `validate:mermaid`.

## CI verification after push

Per [Git Push Default Convention](../../../governance/development/workflow/git-push-default.md)
Standards 1, 2, 6, every commit pushes direct to `origin main`. No
draft PR opens. After each direct push:

1. Watch the GitHub Actions run on `wahidyankf/ose-primer:main` —
   `markdown-lint`, `rhino-cli` build, `validate:mermaid` job. Wait
   for green.
2. If any check fails, fix the root cause in a follow-up commit
   pushed direct to main. Do not bypass with `--no-verify` or
   skip-flag.
3. The post-merge run becomes the only CI gate per push because
   there is no PR step.

Parent gitlink bump (`ose-projects/ose-primer` pointer) is a
parent-side concern outside this plan's scope per the user's
"only in this repo" directive.

## Development environment

- Go 1.22+ on PATH (`volta` does not manage Go — use the system
  toolchain or `asdf` per `governance/development/workflow/reproducible-environments.md`).
- `node` 24.13.1 and `npm` 11.10.1 (Volta-managed by repo).
- Husky hooks installed via `npm install`.
- No DB or service dependency — the validator is pure-Go and
  filesystem-bound.
