# BRD — Fix Mermaid Violations

## Business Problem

Every contributor push to `ose-primer` runs `rhino-cli docs validate-mermaid` via the
pre-push hook. With 107 files failing and 247 violations outstanding, the hook fires on
any push that touches an affected file — blocking clean-code pushes unrelated to
diagrams and degrading CI confidence.

Beyond CI impact, wide diagrams render poorly on GitHub's markdown viewer and in
VS Code preview. Diagrams with span > 3 generate horizontal scrollbars or overflow
their containers, undermining the documentation's value as a learning resource.

## Business Goals

1. Restore clean pre-push hook execution — zero mermaid errors on `main`.
2. Improve diagram readability across GitHub, IDE previews, and any generated doc site.
3. Establish a baseline of zero violations so future violations are caught immediately
   at the PR boundary rather than accumulating.

## Stakeholders

| Stakeholder        | Interest                                            |
| ------------------ | --------------------------------------------------- |
| Contributors       | Unblocked pushes; no false-positive hook failures   |
| Readers / learners | Readable, non-overflowing diagrams in docs          |
| Platform team      | Clean CI baseline; zero violations in template repo |

## Scope

- **In scope**: All markdown files in `docs/` and `governance/` of `ose-primer`.
- **Out of scope**: App source code, specs, test files, generated files.
- **No dependency**: No other repo (ose-public, ose-infra) is affected by these changes;
  this is self-contained within ose-primer.

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
