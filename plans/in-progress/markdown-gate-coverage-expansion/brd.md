# Business Requirements Document — Markdown Gate Coverage Expansion

## Business Goal

Make the repository's **structural markdown validation trustworthy and consistent** across three
validators — Mermaid diagrams, relative links (with anchors), and heading hierarchy — so that no
markdown file with a malformed diagram, a broken link or anchor, or (for generic prose) a broken
heading structure can reach `main` undetected, whether committed locally or pushed directly to
trunk — and keep the two CLI implementations (`rhino-cli-rust`, `rhino-cli-go`) in byte-identical
parity throughout.

## Business Rationale (WHY)

The repository runs two structural markdown validators, implemented in both CLIs, but they are
wired inconsistently — and the third validator the governance docs assume (heading hierarchy)
does not exist here at all. [Repo-grounded]

- **Mermaid** enforces readable, render-correct diagrams — but only at pre-push, after the work
  is already committed, and only over four hardcoded dirs. [Repo-grounded —
  `.husky/pre-push:22-24`, `apps/rhino-cli-rust/src/commands/docs.rs:291-308`]
- **Relative-link checker** verifies links resolve to real files — but scans only three trees
  plus root `*.md`, never validates `#fragment` anchors, and exposes no `--exclude` flag even
  though the `skip_paths` plumbing exists in both implementations. [Repo-grounded —
  `apps/rhino-cli-rust/src/internal/docs/scanner.rs:32-61`,
  `apps/rhino-cli-rust/src/commands/docs.rs:47-52,67`,
  `apps/rhino-cli-go/internal/docs/links_scanner.go:77`]
- **Heading-hierarchy** runs **nowhere because it exists nowhere** — no command in either CLI, no
  module, no hook, no Nx target, no CI. [Repo-grounded — grep across both CLIs] Prose docs get
  zero machine protection for single-H1 and non-skipping nesting, because markdownlint's MD025
  and MD001 are globally disabled to protect agent/skill prompt artifacts. [Repo-grounded —
  `.markdownlint-cli2.jsonc:61,69`]

Unifying enforcement turns two half-wired checks plus one missing check into one coherent gate.
The biggest correctness win is **anchor validation**: a link like
`[X](target#section-that-was-renamed)` resolves today because only the file is checked, never the
heading — so renamed-section links rot silently and mislead both human readers and the agents
that navigate governance docs by anchor. [Repo-grounded — both `resolve_link` implementations
strip the fragment]

### The heading-hierarchy non-breaking constraint (governance-critical)

`markdownlint` disables MD025 (multiple H1) and MD001 (heading increment) globally because agent
and skill prompt artifacts legitimately use `#` as section markers, not titles. [Repo-grounded —
`.markdownlint-cli2.jsonc:61,69`] Empirically the repo has many such files under
`.claude/agents/`, `.claude/skills/`, and `.opencode/agents/`. Re-enabling heading rules naively
would break all of them.

This plan re-enables heading rules **scoped to generic prose** via the rhino CLIs (which can
path-scope a rule, unlike markdownlint). The narrative the maintainer values: prose docs
(`docs/`, `repo-governance/`, `plans/`(−`done/`), root `*.md`, `specs/`, app/lib READMEs and
`docs/` subtrees) regain single-H1 and non-skipping enforcement, while prompt/skill artifacts
stay exempt
by a hard default-deny allowlist. This is the central business constraint of the plan — breaking
an agent or skill file is unacceptable. [Judgment call]

### The dual-CLI parity constraint (template-critical)

This repository is the MIT template extracted from `ose-public`, and the Go/Rust CLI pair is one
of its showcase assets: the parity convention requires every behavior change to land in both
implementations in the same commits, enforced by the shadow-diff harness and the `parity` CI job.
[Repo-grounded —
`repo-governance/conventions/structure/rhino-cli-dual-implementation-parity.md` Rule 1;
`.github/workflows/pr-quality-gate.yml` parity job] Shipping the markdown gate in only one CLI
would break the template's core promise. Gate C is therefore a **greenfield build in BOTH
languages**, which makes it the largest work item in this plan — upstream merely un-orphaned an
existing validator. [Judgment call]

## Business Impact

### Pain points addressed

- **Late diagram feedback** — mermaid findings surface at pre-push, after commits exist; and
  `apps/`, `libs/`, `specs/`, `.opencode/` markdown is never scanned at all. [Repo-grounded]
- **Rotting anchors** — section-renamed links resolve falsely because anchors are unchecked.
  [Repo-grounded]
- **Inflexible link scope** — without `--exclude`, the frozen `plans/done/` archive cannot be
  skipped, so the generic checker either misfires on frozen artifacts or stays repo-narrow.
  [Repo-grounded — no `--exclude` flag exists in either CLI]
- **No heading enforcement anywhere** — prose docs have zero machine protection for heading
  structure. [Repo-grounded]
- **Zero push-to-main CI** — every existing workflow is `pull_request`-only, yet the repo's
  default flow is direct trunk push, so the trunk receives no markdown CI coverage at all.
  [Repo-grounded — inspection of `.github/workflows/`]

### Expected benefits

- **One coherent markdown gate** — three validators, three consistent enforcement layers
  (pre-commit staged-only, PR CI, push CI), one consolidated CI workflow.
- **Anchor-safe links** — `#fragment` targets are validated against the destination file's actual
  headings using a GFM-correct slug algorithm, so renamed sections surface as `broken-anchor`
  findings.
- **Excludable frozen trees** — `--exclude` lets the link/mermaid gates skip `plans/done/`
  explicitly at call sites.
- **Prose heading guarantees without collateral damage** — single-H1 and non-skipping nesting are
  enforced for prose, while agent/skill artifacts are provably exempt.
- **Unskippable CI** — PR and push-to-`main` CI both run all three gates; `--no-verify` only
  skips the local pre-commit layer. The repo gains its first push-to-`main` workflow.
- **Parity preserved** — both CLIs gain identical behavior in the same commits, with shadow-diff
  byte-parity green throughout, so the template's dual-implementation promise stays intact.
- **Spec + governance parity** — the rhino BDD specs (`specs/apps/rhino/`) gain scenarios and a
  component doc for the new behavior so both `spec-coverage` gates stay green, and the convention
  change is propagated via `repo-rules-maker` and validated by a strict `repo-rules-quality-gate`
  double-zero, so no governance surface is left describing the old enforcement.

## Affected Roles

Solo-maintainer repository; the maintainer wears several hats and several agents consume the
surfaces. No sign-off ceremonies apply.

- **Maintainer (tooling hat)** — owns both CLIs, the Nx targets, the hooks, the CI workflows, the
  shadow-diff harness.
- **Maintainer (governance hat)** — owns `diagrams.md`, `quality.md`, `linking.md`,
  `repository-validation.md`.
- **Maintainer (content hat)** — authors diagrams, links, and prose headings across `docs/`,
  `plans/`, `specs/`, governance.
- **Consuming agents** — `swe-rust-dev` and `swe-golang-dev` (validators/hooks), `repo-rules-maker`
  / `docs-maker` (convention docs and fix-all trees), `specs-maker` (spec surface),
  `repo-setup-manager` (Phase 0 baseline), and any agent that authors markdown now covered by the
  gates. [Repo-grounded — all named agents exist under `.claude/agents/`]

## Business-Level Success Metrics

- **Coverage completeness** (observable): the link and mermaid gates scan the whole repo minus
  `plans/done/` + noise dirs; heading-hierarchy scans exactly the prose allowlist — verifiable by
  running each validator and inspecting the scanned file set.
- **Anchor enforcement** (observable): a link to a non-existent `#fragment` produces a
  `broken-anchor` finding — verifiable by a unit test in each CLI.
- **Heading non-breakage** (observable): staging a `.claude/agents/*.md` or `SKILL.md` file
  produces ZERO heading findings even at pre-commit — verifiable by a unit test asserting the
  allowlist filter excludes those paths. [Judgment call: this is the guarantee the maintainer
  values most]
- **Byte parity** (observable): `bash apps/rhino-cli-rust/scripts/shadow-diff.sh docs` exits 0
  after every phase that touches validator behavior — verifiable by running it.
- **Unskippability** (observable): a single `validate-markdown.yml` workflow runs all three gates
  on `pull_request` AND `push` to `main` — verifiable by inspecting `.github/workflows/`.
- **Zero blocking findings** (observable): all three gates report zero findings repo-wide (within
  their scopes) at plan completion — verifiable by running them.
- **Pre-push no longer runs mermaid** (observable): `.husky/pre-push` contains no mermaid
  trigger — verifiable by inspection.

## Business-Scope Non-Goals

- Not building a markdown renderer, link-liveness checker, or external-URL validator.
- Not porting the upstream mermaid extras (inline exemptions, color-palette checks) — this repo's
  mermaid validator keeps its current check set.
- Not changing the markdownlint global config (MD025/MD001 stay disabled there; the rhino CLIs do
  the path-scoped prose enforcement instead).
- Not redesigning the linking convention syntax — only adding enforcement and correcting docs.

## Business Risks and Mitigations

| Risk                                                           | Likelihood | Mitigation                                                                                                                                                                                        |
| -------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Wider link scan surfaces a large broken-link/anchor backlog    | High       | Phase the cleanup one gate per tree; re-measure per tree at execution; fix or correct each before its gate                                                                                        |
| Greenfield Gate C diverges between the two implementations     | Medium     | Specs first (shared gherkin), identical unit fixtures in both, shadow-diff byte parity in every phase gate                                                                                        |
| Heading-hierarchy accidentally fires on an agent/skill file    | Medium     | Allowlist filter lives INSIDE the validator file selection + unit tests assert `.claude/**`/`SKILL.md` are excluded                                                                               |
| Prose heading backlog larger than expected                     | Medium     | Per-tree gates isolate scope; grep-based provisional estimate at Phase 0; re-measure at execution                                                                                                 |
| Anchor slug algorithm diverges from GitHub rendering           | Medium     | Implement the researched GFM algorithm (Unicode-aware, underscores kept, no space collapse, `-N` collisions); unit-test fixtures incl. underscore/Unicode                                         |
| Repo-wide walk descends into worktrees or vendored deps        | Medium     | Bake the standardized cross-repo noise-skip set (incl. `worktrees`) into the walkers; unit-test the skip set; gitignored vendored trees never reach CI and are excludable via `--exclude` locally |
| Moving mermaid to pre-commit slows commits                     | Low        | Staged-only scope keeps it light; per-file checks have no cross-file dependency                                                                                                                   |
| Consolidating the existing link workflow breaks PR link checks | Low        | Migrate `pr-validate-links.yml` into `validate-markdown.yml`; verify the link job still runs on PRs before deleting                                                                               |
| First push-to-main workflow misfires on the delivery push      | Low        | Behavioral acceptance step observes a deliberate RED then GREEN run before the plan is archived                                                                                                   |

See [prd.md](./prd.md) for the testable scenarios that verify each mitigation.
