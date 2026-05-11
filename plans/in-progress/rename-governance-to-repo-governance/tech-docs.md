# Tech Docs — Rename `governance/` to `repo-governance/`

## Architecture

Six sed passes + three `git mv` operations:

| Pass | Token                                                      | Scope                                      | What it covers                                                |
| ---- | ---------------------------------------------------------- | ------------------------------------------ | ------------------------------------------------------------- |
| A    | `governance/` → `repo-governance/`                         | All text files (excl. `.opencode/agents/`) | Path tokens with trailing slash                               |
| B    | `"governance"` → `"repo-governance"`                       | `*.go` only                                | Bare quoted directory-name string literals + test fixtures    |
| C    | `governance vendor-audit` → `repo-governance vendor-audit` | All text files                             | CLI verb (space-separated, no slash)                          |
| D    | `governance-vendor-audit` → `repo-governance-vendor-audit` | All text files                             | Hyphenated form: Nx target, Gherkin tag, spec/convention refs |
| E    | Go package rename (manual + sed)                           | `internal/repo-governance/`                | Package decl + import paths                                   |
| F    | Cobra cmd rename (manual)                                  | `cmd/governance.go` + callers              | `Use:`, variable name, registration                           |

Plus one sync command after agents update: `npm run sync:claude-to-opencode`.

No logic changes. No schema changes. No API changes.

---

## Pass A — Path Tokens (`governance/`)

Catches all string literals with a trailing slash across `.md`, `.sh`, `.go`, `.json`, `.yaml`,
`.yml`, `.feature` files.

**Important exclusion**: `.opencode/agents/` is EXCLUDED. Those files are auto-generated from
`.claude/agents/`. Applying Pass A there and then running the sync would produce a double-update.
Correct sequence: update `.claude/agents/` via Pass A, then regenerate `.opencode/agents/` via
`npm run sync:claude-to-opencode`.

**Pre-push hook**: `.husky/pre-push` has no file extension and is NOT caught by the `find` command.
It must be updated explicitly. Two affected lines (lines 19 and 27):

- `grep -qE '^governance/workflows/'` → `grep -qE '^repo-governance/workflows/'`
- `grep -qE '^(governance/.*\.md|...'` → `grep -qE '^(repo-governance/.*\.md|...'`

Key files caught by Pass A:

- All `.claude/agents/*.md` (52 files) — relative links into `governance/conventions/`,
  `governance/development/`, etc.
- All `.claude/skills/*/SKILL.md` (34 skill packages) — layer-to-path mappings,
  cross-references to governance conventions
- `governance/conventions/structure/governance-vendor-independence.md` — path examples
- `governance/workflows/` — all workflow files with self-referential paths
- `apps/rhino-cli/project.json` — target inputs `{workspaceRoot}/governance/**/*.md`
- `AGENTS.md`, `CLAUDE.md`, `README.md` — top-level instruction files

---

## Pass B — Bare Go String Literals (`"governance"`)

Scoped to `*.go` only. Catches `filepath.Join` segments, default-value assignments, test fixtures.

| File                                              | Pattern                                              |
| ------------------------------------------------- | ---------------------------------------------------- |
| `apps/rhino-cli/cmd/governance_vendor_audit.go`   | `scanPath := "governance"`                           |
| `apps/rhino-cli/cmd/workflows_validate_naming.go` | `filepath.Join(repoRoot, "governance", "workflows")` |
| `apps/rhino-cli/cmd/docs_validate_mermaid.go`     | `filepath.Join(repoRoot, "governance")`              |
| `apps/rhino-cli/internal/docs/links_scanner.go`   | `[]string{"governance", "docs", ".claude"}`          |
| Test occurrences (`*_test.go`)                    | Mock paths + default-path assertions                 |

**Safety**: import paths like `"github.com/.../internal/governance"` do NOT match bare `"governance"`.

---

## Pass C — CLI Verb (`governance vendor-audit` with space)

NOT caught by Pass A (no slash). Applies to all text file types.

Confirmed locations:

| File                                                                 | Content                               |
| -------------------------------------------------------------------- | ------------------------------------- |
| `apps/rhino-cli/project.json`                                        | `governance vendor-audit governance/` |
| `apps/rhino-cli/scripts/validate-cross-vendor-parity.sh`             | `governance vendor-audit ...`         |
| `governance/conventions/structure/governance-vendor-independence.md` | CLI usage examples                    |
| `specs/apps/rhino/cli/gherkin/governance-vendor-audit.feature`       | Gherkin step text                     |

---

## Pass D — Hyphenated Form (`governance-vendor-audit`)

NOT caught by Pass A (no slash). Applies to all text file types.

Confirmed locations:

| File                                                                 | Content                                                 |
| -------------------------------------------------------------------- | ------------------------------------------------------- |
| `apps/rhino-cli/project.json`                                        | target key `validate:governance-vendor-audit`           |
| `.husky/pre-push`                                                    | `npx nx run rhino-cli:validate:governance-vendor-audit` |
| `governance/conventions/structure/governance-vendor-independence.md` | Nx target reference                                     |
| `governance/conventions/structure/README.md`                         | Feature file entry reference                            |
| `specs/apps/rhino/cli/gherkin/governance-vendor-audit.feature`       | `@governance-vendor-audit` tag                          |
| `apps/rhino-cli/README.md`                                           | Nx target name in docs                                  |
| `.claude/agents/repo-parity-checker.md`                              | Target reference                                        |
| `.claude/agents/repo-parity-fixer.md`                                | Target reference                                        |

**Note**: The `.feature` file is named `governance-vendor-audit.feature` — it also needs a `git mv`
(see §File Renames below).

---

## Pass E — Go Package Rename (`internal/governance` → `internal/repo-governance`)

Module: `github.com/wahidyankf/ose-public/apps/rhino-cli` (ose-primer rhino-cli shares this module
path with ose-public by design).

### Files in `internal/governance/`

| File                                                  | Package declaration  |
| ----------------------------------------------------- | -------------------- |
| `internal/governance/governance_vendor_audit.go`      | `package governance` |
| `internal/governance/governance_vendor_audit_test.go` | `package governance` |

### E1 — `git mv`

```bash
git mv apps/rhino-cli/internal/governance apps/rhino-cli/internal/repo-governance
```

### E2 — Package declarations

Go disallows hyphens in package names. New name: **`package repogovernance`**

Update both files in `internal/repo-governance/`.

### E3 — Import paths (2 files)

ose-primer does NOT have `cmd/golden_test.go` (differs from ose-public).

Files: `cmd/governance_vendor_audit.go`, `cmd/governance_vendor_audit_test.go`

Use import alias `governance` to preserve all call sites (`governance.Walk(...)` unchanged):

```go
// before
"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/governance"

// after
governance "github.com/wahidyankf/ose-public/apps/rhino-cli/internal/repo-governance"
```

---

## Pass F — Cobra Command Rename (manual)

| File                                                     | Change                                                                |
| -------------------------------------------------------- | --------------------------------------------------------------------- |
| `apps/rhino-cli/cmd/governance.go` line ~6               | `Use: "governance"` → `Use: "repo-governance"`                        |
| `apps/rhino-cli/cmd/governance.go` line ~7               | `Short:` update prose                                                 |
| `apps/rhino-cli/cmd/governance.go` line ~8               | `Long:` update prose                                                  |
| `apps/rhino-cli/cmd/governance.go` var                   | `governanceCmd` → `repoGovernanceCmd`                                 |
| `apps/rhino-cli/cmd/governance_vendor_audit.go` line ~43 | `governanceCmd.AddCommand(...)` → `repoGovernanceCmd.AddCommand(...)` |

---

## File Renames (git mv)

Three directories/files renamed via `git mv`:

| Old path                                                       | New path                                                            |
| -------------------------------------------------------------- | ------------------------------------------------------------------- |
| `governance/`                                                  | `repo-governance/`                                                  |
| `apps/rhino-cli/internal/governance/`                          | `apps/rhino-cli/internal/repo-governance/`                          |
| `specs/apps/rhino/cli/gherkin/governance-vendor-audit.feature` | `specs/apps/rhino/cli/gherkin/repo-governance-vendor-audit.feature` |

**ose-primer vs ose-public difference**: specs path is `specs/apps/rhino/cli/gherkin/` (not
`specs/apps/rhino/behavior/cli/gherkin/` as in ose-public).

---

## OpenCode Agent Sync

`.opencode/agents/` (52 files) are auto-generated mirrors of `.claude/agents/`. Do NOT apply Pass A
to `.opencode/agents/` directly. After all `.claude/agents/` updates are complete, regenerate:

```bash
npm run sync:claude-to-opencode
```

---

## Full File Impact Summary

### Critical — functional breakage if missed

| File                                                                 | Pass(es)      | Risk if missed                                          |
| -------------------------------------------------------------------- | ------------- | ------------------------------------------------------- |
| `apps/rhino-cli/cmd/governance_vendor_audit.go`                      | A, B, E       | Scan uses wrong path; broken import                     |
| `apps/rhino-cli/cmd/workflows_validate_naming.go`                    | A, B          | Wrong workflow walk root                                |
| `apps/rhino-cli/cmd/docs_validate_mermaid.go`                        | A, B          | Wrong Mermaid default dir                               |
| `apps/rhino-cli/internal/docs/links_scanner.go`                      | B             | Wrong default scan dirs                                 |
| `apps/rhino-cli/internal/docs/links_categorizer.go`                  | A             | Wrong exclusion logic                                   |
| `apps/rhino-cli/internal/repo-governance/governance_vendor_audit.go` | A, E          | Wrong exemption path; broken pkg                        |
| `apps/rhino-cli/cmd/governance.go`                                   | F             | CLI verb unchanged — subcommand not found               |
| `apps/rhino-cli/project.json`                                        | A, C, D       | Wrong CLI verb; wrong Nx target key                     |
| `.husky/pre-push`                                                    | A, D          | Hook invokes non-existent Nx target; wrong path pattern |
| `apps/rhino-cli/scripts/validate-cross-vendor-parity.sh`             | A, C          | Wrong CLI verb + wrong paths                            |
| `specs/apps/rhino/cli/gherkin/governance-vendor-audit.feature`       | C, D + git mv | Wrong CLI verb in step text; tag mismatch               |

### High-impact documentation

| Group                                                                | Pass     | Refs                   |
| -------------------------------------------------------------------- | -------- | ---------------------- |
| Root docs (`AGENTS.md`, `CLAUDE.md`, `README.md`)                    | A        | ~50                    |
| `.claude/agents/` (52 files)                                         | A        | ~400                   |
| `.claude/skills/` (34 packages)                                      | A        | ~250                   |
| `.opencode/agents/` (52 files)                                       | sync cmd | ~400 (regenerated)     |
| `governance/conventions/structure/governance-vendor-independence.md` | A, C, D  | CLI + target refs      |
| `specs/apps/rhino/` files                                            | A, C, D  | Path + verb + tag refs |

### Excluded

| Path                  | Reason                                            |
| --------------------- | ------------------------------------------------- |
| `.nx/workspace-data/` | Auto-regenerated by Nx                            |
| `worktrees/`          | Live branches, recreated fresh                    |
| `*.out` / `cover.out` | Test coverage output, regenerated                 |
| `.opencode/agents/`   | Regenerated via `npm run sync:claude-to-opencode` |
| `plans/done/`         | Historical record; stale refs acceptable          |
| `generated-reports/`  | Historical audit output; stale refs acceptable    |

---

## Design Decisions

**Pass A excludes `.opencode/agents/`**
These files are auto-generated from `.claude/agents/`. Applying Pass A then sync would double-update
or produce inconsistent state. Correct sequence: update source (`.claude/`), then sync.

**Import alias `governance` for the renamed package**
Using `import governance "...internal/repo-governance"` preserves all `governance.Walk(...)` call
sites. Minimises diff noise while making import path accurate.

**No parent CLAUDE.md update**
Parent repo has zero occurrences of `ose-primer/governance/` — verified at plan creation time.

**ose-primer has no `golden_test.go`**
Pass E (import path update) applies to 2 files only (`cmd/governance_vendor_audit.go` and
`cmd/governance_vendor_audit_test.go`), unlike ose-public which has 3.

---

## Dependencies

This is an independent rename with no external dependencies:

- Go toolchain (`go build ./...` for compile verification)
- Nx CLI (`npx nx run rhino-cli:*` targets for build, lint, test, vendor-audit)
- `npm run sync:claude-to-opencode` (OpenCode agent mirror regeneration after Pass A)
- `.husky/pre-push` (updated explicitly — no file extension, not caught by `find`)

No new libraries, packages, or infrastructure are introduced or required.

## Testing Strategy

All verification is grep-based (zero matches for old token) plus compile-pass (compile error
surfaces any missed import path). This is a complete strategy for a pure rename because:

- A missed `governance/` path token shows up as a broken reference at runtime or test time
- A missed Go import or package declaration causes `go build ./...` to fail
- The Phase 10 full-verification checklist enumerates every grep check needed
- Phase 11 quality gates (rhino-cli build, lint, test:unit, test:integration, vendor-audit,
  markdown lint, Nx affected typecheck/lint/test:quick/spec-coverage) provide exhaustive
  automated coverage

No new test logic is required. The existing test suite is the test strategy.

## Rollback

```bash
# 1. Reverse directory + file renames
git mv repo-governance governance
git mv apps/rhino-cli/internal/repo-governance apps/rhino-cli/internal/governance
git mv specs/apps/rhino/cli/gherkin/repo-governance-vendor-audit.feature \
       specs/apps/rhino/cli/gherkin/governance-vendor-audit.feature

# 2. Reverse Pass A
find . -not -path './.git/*' -not -path './node_modules/*' \
  -not -path './.nx/*' -not -path './worktrees/*' -not -path './.opencode/agents/*' \
  -not -name '*.out' \
  -type f \( -name '*.md' -o -name '*.sh' -o -name '*.go' -o -name '*.json' \
    -o -name '*.yaml' -o -name '*.yml' -o -name '*.feature' \) \
  | xargs grep -l 'repo-governance/' \
  | xargs sed -i '' 's|repo-governance/|governance/|g'

# 3. Reverse pre-push hook
sed -i '' 's|repo-governance/|governance/|g' .husky/pre-push

# 4. Reverse Pass B
find apps/rhino-cli -name '*.go' | xargs grep -l '"repo-governance"' \
  | xargs sed -i '' 's|"repo-governance"|"governance"|g'

# 5. Reverse Pass C
find . -not -path './.git/*' -not -path './node_modules/*' \
  -not -path './.nx/*' -not -path './worktrees/*' -not -path './.opencode/agents/*' \
  -type f \( -name '*.md' -o -name '*.sh' -o -name '*.go' -o -name '*.json' -o -name '*.feature' \) \
  | xargs grep -l 'repo-governance vendor-audit' \
  | xargs sed -i '' 's|repo-governance vendor-audit|governance vendor-audit|g'

# 6. Reverse Pass D
find . -not -path './.git/*' -not -path './node_modules/*' \
  -not -path './.nx/*' -not -path './worktrees/*' -not -path './.opencode/agents/*' \
  -type f \( -name '*.md' -o -name '*.sh' -o -name '*.go' -o -name '*.json' -o -name '*.feature' \) \
  | xargs grep -l 'repo-governance-vendor-audit' \
  | xargs sed -i '' 's|repo-governance-vendor-audit|governance-vendor-audit|g'
sed -i '' 's|repo-governance-vendor-audit|governance-vendor-audit|g' .husky/pre-push

# 7. Reverse Pass E — edit package decls and import aliases manually

# 8. Reverse Pass F — edit cmd/governance.go manually

# 9. Re-sync OpenCode agents
npm run sync:claude-to-opencode
```
