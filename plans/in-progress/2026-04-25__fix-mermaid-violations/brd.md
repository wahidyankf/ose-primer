# BRD — Fix Mermaid Violations

## Business Problem

`rhino-cli docs validate-mermaid` reports 107 files failing with 247 violations across
`docs/`. The pre-push hook runs `validate-mermaid` but targets only `governance/` and
`.claude/` — so violations in `docs/` are not currently blocking pushes. However, the
validator fails on those files as a code-quality check, and the violations are real:
wide diagrams render poorly on GitHub's markdown viewer and in VS Code preview.
Diagrams with span > 3 generate horizontal scrollbars or overflow their containers,
undermining the documentation's value as a learning resource.

Fixing all violations establishes a zero-violation baseline so that any future
`docs/` expansion of the hook scope will not surface a backlog of pre-existing errors,
and so that diagrams are readable today.

## Business Goals

1. Achieve zero mermaid validation errors on `main` — clean baseline for the entire repo.
2. Improve diagram readability across GitHub, IDE previews, and any generated doc site.
3. Establish a baseline of zero violations so future violations are caught immediately
   at the PR boundary rather than accumulating.

## Affected Roles

| Role                    | Hat worn                                                                    |
| ----------------------- | --------------------------------------------------------------------------- |
| Contributor / committer | Running pre-push hook; wanting clean validator output on future hook scopes |
| Documentation reader    | Reading diagrams in GitHub preview or VS Code; needing non-overflowing view |
| Plan executor           | Running the delivery checklist; fixing files batch-by-batch                 |

## Scope

- **In scope**: All markdown files in `docs/` of `ose-primer` with `width_exceeded` or
  `label_too_long` violations. (`governance/` files were audited and found clean — no
  violations to fix.)
- **Out of scope**: App source code, specs, test files, generated files.
- **No dependency**: No other repo (ose-public, ose-infra) is affected by these changes;
  this is self-contained within ose-primer.

## Non-Goals

- Not fixing `complex_diagram` warnings (12 instances across 6 files) — deferred to a
  future pass. These are warnings, not errors, and do not affect validator exit code.
- Not modifying the pre-push hook's `validate:mermaid` Nx target to extend scanning to
  `docs/` — that is a separate infrastructure change with its own risk profile.
- Not improving diagram visual quality beyond what is required to pass the validator
  rules (`width_exceeded` and `label_too_long` thresholds only).

## Success Criteria

`go run ./apps/rhino-cli/main.go docs validate-mermaid` exits 0 with no output after
all batches are complete and committed to `main`.

## Risks

| Risk                                                | Likelihood | Mitigation                                                               |
| --------------------------------------------------- | ---------- | ------------------------------------------------------------------------ |
| Diagram restructuring breaks semantic meaning       | Medium     | Re-read each diagram's surrounding prose; preserve logical relationships |
| Subgraph grouping changes visual hierarchy          | Low        | Validate in GitHub preview before committing                             |
| Label truncation loses important context            | Medium     | Move truncated context to surrounding prose instead of losing it         |
| Wide diagrams need splitting — increases doc length | Low        | Acceptable trade-off; shorter diagrams are more scannable                |
