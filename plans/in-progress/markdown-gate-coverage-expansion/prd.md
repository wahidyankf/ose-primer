# Product Requirements Document — Markdown Gate Coverage Expansion

## Product Overview

The product is the **markdown gate**: three validators (`docs validate-mermaid`,
`docs validate-links`, `docs validate-heading-hierarchy`) implemented identically in
`apps/rhino-cli-rust/` and `apps/rhino-cli-go/`, their Nx targets, and **three consistent
enforcement layers** — pre-commit staged-only via the `git pre-commit` suite (Layer 1), PR CI
(Layer 2), and push-to-`main` CI (Layer 3), with Layers 2 and 3 consolidated into a single
`.github/workflows/validate-markdown.yml`. This plan moves mermaid to pre-commit and widens it
repo-wide with `--exclude`, widens the link checker (scope + `--exclude` + anchors), builds the
heading-hierarchy checker from scratch in both CLIs under a prose-allowlist default-deny scope,
and cleans the resulting baseline.

## Personas

Solo-maintainer repository; personas are hats the maintainer wears and agents that consume the
surfaces.

- **Diagram author** — writes flowcharts; wants fast, accurate pre-commit feedback anywhere in
  the repo.
- **Link author** — writes `[text](target#anchor)` links; wants broken files AND broken anchors
  caught.
- **Prose author** — writes `docs/`, `repo-governance/`, `plans/`, `specs/`, README content;
  wants single-H1 and non-skipping heading nesting enforced.
- **Prompt/skill author** — writes `.claude/agents/`, `SKILL.md`, `.opencode/agents/` artifacts;
  must NEVER be tripped by heading rules.
- **Tooling maintainer** — owns both CLI implementations; wants existing tests green, new
  behavior covered in both, and byte parity preserved.
- **CI** — must block PRs and block direct `main` pushes, even when local hooks are skipped.

## User Stories

- **US-1** — As a tooling maintainer, I want the mermaid gate to run at pre-commit staged-only
  instead of pre-push, so diagram feedback arrives earlier and the pre-push hook is lighter.
- **US-2** — As a diagram author, I want the mermaid full scan to cover the whole repo minus
  excluded trees, so a malformed diagram anywhere outside the frozen archive is caught.
- **US-3** — As a link author, I want the link checker to scan the whole repo minus excluded
  trees, so a broken link in `apps/`, `libs/`, or `specs/` is caught.
- **US-4** — As a tooling maintainer, I want a repeatable `--exclude <path>` flag on all three
  validators, so the frozen `plans/done/` tree can be skipped explicitly at call sites.
- **US-5** — As a link author, I want a link to a non-existent `#fragment` to be flagged, so
  renamed sections do not leave silently-rotten anchors.
- **US-6** — As a link author, I want the slug algorithm to match GitHub's real behavior
  (underscores and Unicode kept, spaces not collapsed, `-N` collision suffixes), so anchor
  findings agree with what GitHub actually renders.
- **US-7** — As a prose author, I want heading-hierarchy enforced on `docs/`,
  `repo-governance/`, `plans/`(−`done/`), root `*.md`, `specs/`, and app/lib READMEs + `docs/`
  subtrees, so prose docs keep one H1 and non-skipping nesting.
- **US-8** — As a prompt/skill author, I want heading rules to NEVER fire on `.claude/**`,
  `.opencode/**`, or `.amazonq/**`, so my section-marker `#` usage is never a finding even at
  pre-commit.
- **US-9** — As CI, I want one consolidated workflow running all three gates on `pull_request`
  AND `push` to `main`, so the gate is unskippable and direct trunk pushes are covered for the
  first time.
- **US-10** — As a tooling maintainer, I want the anchor validator and the heading validator to
  share one fence-aware heading parser per CLI, so heading parsing is not duplicated across
  modules.
- **US-11** — As a tooling maintainer, I want every behavior change to land in BOTH CLIs in the
  same commits with shadow-diff byte parity green, so the dual-implementation convention holds.
- **US-12** — As a tooling maintainer, I want the rhino BDD specs under `specs/apps/rhino/` to
  gain scenarios for the new validator behavior and a `component-cli.md` command/flag inventory,
  so both `spec-coverage` gates stay green and the specs remain the source of the first failing
  tests.
- **US-13** — As a maintainer, I want `diagrams.md`, `quality.md`, `linking.md`, and
  `repository-validation.md` to accurately describe the new enforcement, so governance docs match
  the tooling.
- **US-14** — As a governance maintainer, I want the convention updates propagated via
  `repo-rules-maker` and validated by `repo-rules-quality-gate` (strict, double-zero), so the
  rule change reaches every governance surface, not just the obvious files.

## Acceptance Criteria (Gherkin)

> Each scenario obeys the repo keyword-cardinality norm: at most one `Given`, one `When`, one
> `Then`; additional steps use `And`/`But`.

### Gate A — Mermaid scope and enforcement move

```gherkin
Scenario: Pre-push no longer triggers the mermaid gate
  Given the .husky/pre-push hook
  When a contributor inspects its trigger blocks
  Then no block runs the validate:mermaid target
  And the mermaid trigger has been removed
```

```gherkin
Scenario: Pre-commit runs the mermaid gate on staged markdown
  Given a staged markdown file containing a malformed flowchart
  When the pre-commit suite runs
  Then the mermaid gate reports the violation
  And the commit is blocked
```

```gherkin
Scenario: Pre-commit mermaid gate ignores unstaged markdown
  Given an unstaged markdown file containing a malformed flowchart
  When the pre-commit suite runs
  Then the mermaid gate does not report that file
  And the commit is allowed
```

```gherkin
Scenario: Mermaid full scan covers the whole repo minus exclusions
  Given a malformed flowchart in a markdown file under specs
  When docs validate-mermaid runs a full scan with --exclude plans/done
  Then the violation under specs is reported
  And files under plans/done and the noise-skip set are not scanned
```

### Gate B — Link checker scope, exclude flag, anchors

```gherkin
Scenario: Link checker scans the whole repo minus exclusions
  Given a broken relative link in a file under libs
  When docs validate-links runs a full scan
  Then the broken link is reported
  And files under the noise-skip set are not scanned
```

```gherkin
Scenario: A repeated exclude flag skips named trees
  Given a broken relative link in a file under plans/done
  When docs validate-links runs with --exclude plans/done
  Then the broken link under plans/done is not reported
  And links outside the excluded tree are still validated
```

```gherkin
Scenario: The .claude/skills tree stays hard-skipped
  Given a broken relative link in a SKILL.md under .claude/skills
  When docs validate-links runs a full scan
  Then the broken link is not reported
  And the hard-skip applies without an explicit exclude flag
```

```gherkin
Scenario: A link to a missing anchor is flagged
  Given a link [X](./target.md#missing-section) whose target file exists
  And target.md contains no heading that slugifies to missing-section
  When docs validate-links validates the link
  Then a broken-anchor finding is reported
  And the validator exits non-zero
```

```gherkin
Scenario: A link to an existing anchor passes
  Given a link [X](./target.md#real-section) whose target file exists
  And target.md contains a heading "## Real Section"
  When docs validate-links validates the link
  Then no broken-anchor finding is reported for that link
```

```gherkin
Scenario: GitHub slug collisions get numeric suffixes
  Given target.md contains two headings both titled "Setup"
  When the anchor validator slugifies the headings
  Then the first heading slug is setup
  And the second heading slug is setup-1
```

```gherkin
Scenario: Underscores and Unicode are kept in slugs
  Given target.md contains a heading "## snake_case naming für Domänen"
  When the anchor validator slugifies the heading
  Then the slug keeps the underscore and the Unicode letters
  And a link to #snake_case-naming-für-domänen passes validation
```

```gherkin
Scenario: A same-file anchor link is validated against its own headings
  Given a file linking to [Y](#own-section)
  And the file contains no heading that slugifies to own-section
  When docs validate-links validates the link
  Then a broken-anchor finding is reported for that file
```

### Gate C — Heading-hierarchy prose allowlist, default-deny

```gherkin
Scenario: Heading-hierarchy runs on the prose allowlist
  Given a prose file under docs with two H1 headings
  When docs validate-heading-hierarchy runs a full scan
  Then a duplicate-h1 finding is reported for that file
  And the validator exits non-zero
```

```gherkin
Scenario: An agent file is exempt from heading rules
  Given a .claude/agents file with zero H1 headings
  When docs validate-heading-hierarchy runs a full scan
  Then no missing-h1 finding is reported for that file
  And the file is excluded by the default-deny allowlist
```

```gherkin
Scenario: A staged skill file never trips heading rules at pre-commit
  Given a staged SKILL.md under .claude/skills with many H1 headings
  When the pre-commit suite runs the heading gate
  Then no heading finding is reported for the skill file
  And the allowlist filter excludes it inside the validator file selection
```

```gherkin
Scenario: plans/done is excluded from heading rules
  Given a frozen plan file under plans/done with a skipped heading level
  When docs validate-heading-hierarchy runs a full scan
  Then no skipped-level finding is reported for that file
  And plans/done is outside the allowlist
```

```gherkin
Scenario: specs prose and app READMEs are inside the heading scope
  Given an apps/example/README.md with a skipped heading level
  And a specs file with a duplicate H1
  When docs validate-heading-hierarchy runs a full scan
  Then a skipped-level finding is reported for the app README
  And a duplicate-h1 finding is reported for the specs file
```

```gherkin
Scenario: Deep app internals are outside the heading scope
  Given an apps/example/src/notes.md with zero H1 headings
  When docs validate-heading-hierarchy runs a full scan
  Then no missing-h1 finding is reported for that file
  And non-README non-docs app paths are outside the allowlist
```

```gherkin
Scenario: Heading-hierarchy honors a repeated exclude flag
  Given a prose file under docs with a duplicate H1
  When docs validate-heading-hierarchy runs with --exclude docs
  Then no finding is reported for the excluded docs tree
  And other allowlist trees are still validated
```

### Dual-CLI parity

```gherkin
Scenario: Both implementations produce byte-identical output
  Given the same markdown corpus and the same validator arguments
  When the shadow-diff harness runs the docs corpus against both CLIs
  Then the Rust and Go outputs are byte-identical
  And the harness exits zero
```

### Enforcement consolidation (three layers, one CI workflow)

```gherkin
Scenario: The consolidated workflow triggers on PR and push to main
  Given the validate-markdown workflow definition
  When a contributor inspects its on block
  Then it triggers on pull_request to main
  And it triggers on push to main
```

```gherkin
Scenario: The consolidated workflow runs all three gates
  Given the validate-markdown workflow definition
  When a contributor inspects its jobs
  Then it runs the mermaid validator
  And it runs the links validator
  And it runs the heading-hierarchy validator
```

```gherkin
Scenario: The legacy link workflow is migrated
  Given the .github/workflows directory after this plan
  When a contributor lists the workflows
  Then pr-validate-links.yml no longer exists as a standalone file
  And its link check now runs inside validate-markdown.yml
```

```gherkin
Scenario: Pre-commit blocks but --no-verify is the escape
  Given a staged markdown file with a broken link
  When the contributor commits without --no-verify
  Then the commit is blocked by the link gate
  But committing with --no-verify bypasses the local gate
```

### Per-tree cleanup and dogfooding

```gherkin
Scenario: Each tree reports zero findings after its cleanup phase
  Given a tree-specific cleanup phase is complete
  When all three gates run against that tree within scope
  Then zero blocking findings are reported for that tree
```

```gherkin
Scenario: This plan passes its own gates
  Given the five plan documents under plans/in-progress/markdown-gate-coverage-expansion
  When all three gates run against the plans tree
  Then every diagram, link, anchor, and prose heading in this plan is valid
  And zero blocking findings are reported for this plan
```

### Spec parity and governance propagation

```gherkin
Scenario: The rhino BDD specs cover the new validator behavior
  Given the spec files under specs/apps/rhino/behavior/cli/gherkin
  When both spec-coverage gates map scenarios to step definitions
  Then docs-validate-links.feature has scenarios for --exclude, repo-wide scan, and broken-anchor
  And docs-validate-heading-hierarchy.feature exists with scenarios for the prose allowlist and --exclude
  And docs-validate-mermaid.feature has scenarios for --exclude and the repo-wide scan
  And git-pre-commit.feature has scenarios for the staged mermaid and heading steps
```

```gherkin
Scenario: The CLI component doc inventories the new surface
  Given the specs/apps/rhino/components/cli directory after this plan
  When a contributor opens component-cli.md
  Then the docs validate-links, validate-mermaid, and validate-heading-hierarchy commands are documented
  And the --exclude flags appear in the flag inventory
```

```gherkin
Scenario: The convention change is propagated and gate-validated
  Given the related governance docs updated for the new enforcement
  When repo-rules-maker performs the governance propagation sweep
  Then every related surface reflects the change
  And a strict repo-rules-quality-gate run reaches double-zero
```

## Product Scope

### In scope (features)

- Move the mermaid gate from pre-push to pre-commit staged-only (remove the `.husky/pre-push`
  mermaid trigger); expand its full scan repo-wide minus exclusions; add a repeatable
  `--exclude <path>` flag — in BOTH CLIs.
- `docs validate-links` (both CLIs): repeatable `--exclude <path>` flag appended to
  `ScanOptions.skip_paths`; repo-wide full scan minus the named exclusion (`plans/done`) + the
  noise-skip set; keep the `.claude/skills/` hard-skip and the `.opencode/skill/` baked-in skip;
  new `broken-anchor` finding category via GFM-correct slugify reusing the shared fence-aware
  heading parser; remove the pure-anchor extraction skip so same-file anchors are validated.
- `docs validate-heading-hierarchy` (both CLIs, NEW): three finding kinds (`missing-h1`,
  `duplicate-h1`, `skipped-level`); prose-allowlist default-deny scope enforced inside file
  selection; `--exclude` flag; wired into pre-commit staged-only + CI full-scan, blocking.
- Pre-commit suite (both CLIs' `git pre-commit` runners): staged-only mermaid + heading steps
  mirroring the existing link step; link step's `skip_paths` extended with `plans/done` while
  preserving the existing `.opencode/skill/` and `.claude/worktrees/` entries.
- Nx targets `validate:links` + `validate:heading-hierarchy` in BOTH `project.json` files;
  `validate:mermaid` commands updated with `--exclude plans/done` and repo-wide inputs.
- A single `.github/workflows/validate-markdown.yml` (push + PR to `main`) running the three
  Rust Nx targets; delete `pr-validate-links.yml`.
- Per-tree fix-all of surfaced violations (mermaid, broken links, broken anchors, prose
  headings), gated.
- BDD spec updates under `specs/apps/rhino/` (link/mermaid/heading/git-pre-commit `.feature`
  files + NEW `component-cli.md`) in lockstep with the code, keeping both `spec-coverage` gates
  green; shadow-diff corpus extended to cover the new command and flags.
- Governance propagation via `repo-rules-maker` (broad sweep), `npm run generate:bindings`
  re-sync, and a strict `repo-rules-quality-gate` double-zero validation.

### Out of scope (features)

- Upstream mermaid extras (inline `%%` exemptions, color-palette checks, additional structural
  checks) — the mermaid check set is unchanged.
- Markdown rendering / link-liveness / external-URL validation.
- Cross-file link-graph analysis beyond existence + anchor presence.
- Changing markdownlint's global MD025/MD001 config.

## Product Risks

| Risk                                                       | Mitigation                                                                                                                                                                         |
| ---------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Anchor slug algorithm diverges from GitHub rendering       | Implement the researched GFM slug (Unicode-aware keep-set, underscores kept, no space collapse, `-N` collisions); unit-test fixtures incl. underscore/Unicode/backtick/multi-space |
| Gate C behavior drifts between Rust and Go                 | Shared gherkin specs first; identical unit fixtures; shadow-diff byte parity in every phase gate                                                                                   |
| Heading allowlist leaks an agent/skill file                | Allowlist filter inside file selection; unit-test `.claude/**` and `SKILL.md` are excluded in both CLIs                                                                            |
| Wider scans produce a large backlog                        | Per-tree gated fix-all; re-measure each tree at execution                                                                                                                          |
| Migrating the link workflow drops PR link coverage         | Verify the link job runs in `validate-markdown.yml` on PRs before deleting the old file                                                                                            |
| Pre-commit mermaid staged-only misses a cross-file issue   | Mermaid checks are per-file with no cross-file dependency — staged-only loses nothing                                                                                              |
| `--exclude` prefix matching is too broad/narrow            | Reuse existing `filter_skip_paths`/`filterSkipPaths` prefix semantics; unit-test included/excluded paths                                                                           |
| Repo-wide walk explodes runtime or descends into worktrees | Name-based noise-dir skip (incl. `worktrees`, `deps`, `_build`, `.venv`) inside the walkers; unit-test the skip set                                                                |
| Rust 90% / Go 90% coverage gates fail on new code          | Keep validator logic in coverage-gated internal modules with thin CLI adapters (matches existing layout); test logic directly                                                      |
