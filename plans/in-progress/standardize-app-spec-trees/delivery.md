# Delivery — standardize-app-spec-trees (ose-primer)

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

<!-- separates adjacent blockquotes (markdownlint MD028) -->

> **Delivery mode**: main-to-main (commit + push directly to ose-primer `main`, no PR). This is a
> recorded deviation from the ose-primer Sync Convention Safety Invariant 6 (PR-only), accepted
> because this plan is docs-and-structure only. See `tech-docs.md` §Delivery-Mode Deviation.

## Worktree

Worktree path: `worktrees/standardize-app-spec-trees/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree standardize-app-spec-trees
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Parallelization Strategy (per-family worktrees)

Each family's restructure touches a **disjoint** `specs/apps/<family>/` subtree and a disjoint set of
app/e2e consumers, so the family phases can run **concurrently in their own sub-worktrees** off
`origin/main`, then converge for the shared-governance phase. This is the within-repo parallelism
layer; the three sibling repos (ose-public, ose-primer, ose-infra) parallelize independently via
their own canonical worktrees.

| Sub-worktree                                        | Branch                       | Covers                      | Independent? |
| --------------------------------------------------- | ---------------------------- | --------------------------- | ------------ |
| `worktrees/standardize-app-spec-trees--crud/`       | `spec-trees/crud`            | Phases 1, 2 (crud be + web) | yes          |
| `worktrees/standardize-app-spec-trees--rhino/`      | `spec-trees/rhino`           | Phase 3 (rhino cli)         | yes          |
| `worktrees/standardize-app-spec-trees/` (canonical) | `standardize-app-spec-trees` | Phase 0 + Phases 4, 5       | convergence  |

**Rules**:

- **Phases 1→2 are sequential within the `--crud` worktree** (both edit the `specs/apps/crud/` tree).
  The `--crud` chain and the `--rhino` worktree are **mutually independent** and run in parallel, each
  branched from the same `origin/main` baseline established in Phase 0. The per-family branches
  (`spec-trees/crud`, `spec-trees/rhino`) are local-only; they are merged into the canonical branch
  before the single `git push origin main` in Phase 5.
- **Shared-file edits are FORBIDDEN inside per-family worktrees.** Every file touched by more than one
  family — `specs/README.md`, the convention, `specs-checker.md`, `specs-maker.md` — is edited ONLY in
  **Phase 4** in the canonical worktree, after both family branches merge. Each per-family worktree
  edits only its own `specs/apps/<family>/` tree + that family's consumers.
- **Convergence**: when both family branches are green per their own Phase Gates, merge them into the
  canonical `standardize-app-spec-trees` branch (`git merge spec-trees/crud spec-trees/rhino`), then
  run Phase 4 (convention + agents + bindings re-sync + rationale) and Phase 5 (quality gates + push +
  archival) once over the unified tree.
- **Provisioning** (run from repo root, per family): `git worktree add
worktrees/standardize-app-spec-trees--<family> -b spec-trees/<family> origin/main`, then `cd` in and
  run `npm install && npm run doctor -- --fix` per
  [Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md).
- **Single-worktree fallback**: a sequential executor MAY run Phases 0→1→2→3→4→5 in the canonical
  worktree alone. Parallelism is an optimization, not a correctness requirement — the phase order is
  already a valid serial sequence.

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized.
- [ ] [AI] Converge the toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift.
- [ ] [AI] Record the live behavior-dir tree:
      `find specs/apps -type d -path '*/behavior/*'` — acceptance: output shows exactly
      `specs/apps/crud/behavior/{be,web}` (+ `gherkin`) and `specs/apps/rhino/behavior/cli`
      (+ `gherkin`); saved as the pre-rename baseline.
- [ ] [AI] Establish the affected-target baseline:
      `npx nx affected -t typecheck lint test:quick spec-coverage --base=origin/main 2>&1 | tee local-temp/spec-trees-baseline.txt`
      — acceptance: baseline pass/fail recorded; any preexisting failure documented.
- [ ] [AI] Resolve all preexisting failures before proceeding — acceptance: no preexisting failure
      remains unresolved.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
- [ ] [AI] Baseline recorded in `local-temp/spec-trees-baseline.txt`; every preexisting failure
      resolved (zero unresolved).

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no rename
> work exists yet. Safe to stop indefinitely. To resume: re-run the baseline command and confirm it
> is still clean.

## Phase 1: crud backend — `behavior/be` → `behavior/crud-be`

- [ ] [AI] Relocate the dir:
      `git mv specs/apps/crud/behavior/be specs/apps/crud/behavior/crud-be`
      — acceptance: `test -d specs/apps/crud/behavior/crud-be/gherkin && ! test -d specs/apps/crud/behavior/be`.
- [ ] [AI] Rewire all 11 `crud-be-*` backend `project.json` files + `crud-be-e2e` + fullstack BE
      inputs, replacing `specs/apps/crud/behavior/be/gherkin` with
      `specs/apps/crud/behavior/crud-be/gherkin` in:
      `apps/crud-be-clojure-pedestal/project.json` (56,72,100,102),
      `apps/crud-be-csharp-aspnetcore/project.json` (55,70,98,100),
      `apps/crud-be-elixir-phoenix/project.json` (54,69,104,106),
      `apps/crud-be-fsharp-giraffe/project.json` (58,73,106,108),
      `apps/crud-be-golang-gin/project.json` (52,68,103,105),
      `apps/crud-be-java-springboot/project.json` (59,74,106,108),
      `apps/crud-be-java-vertx/project.json` (59,74,106,108),
      `apps/crud-be-kotlin-ktor/project.json` (55,70,98,100),
      `apps/crud-be-python-fastapi/project.json` (56,71,99,101),
      `apps/crud-be-rust-axum/project.json` (59,77,110,112),
      `apps/crud-be-ts-effect/project.json` (46,57,66,76,104,106),
      `apps/crud-be-e2e/project.json` (27,36,74,76),
      `apps/crud-fs-ts-nextjs/project.json` (62,77).
      — command: `grep -rl "crud/behavior/be" apps/*/project.json` — acceptance: returns no files.
- [ ] [AI] Update the e2e playwright config `apps/crud-be-e2e/playwright.config.ts` (5,6):
      `featuresRoot` + `features` → `crud-be/gherkin`
      — acceptance: `grep -n "crud-be/gherkin" apps/crud-be-e2e/playwright.config.ts` shows both lines.
- [ ] [AI] Regenerate the e2e features tree (do NOT hand-edit `.features-gen/`):
      `npx nx run crud-be-e2e:test:e2e --configuration ci 2>&1 | tail -5` OR the project's
      feature-gen step — acceptance: `apps/crud-be-e2e/.features-gen/` regenerated against the new
      path; no stale `behavior/be` references remain (`grep -r "behavior/be" apps/crud-be-e2e/.features-gen` returns nothing).
- [ ] [AI] Update spec links in the 11 BE READMEs + `apps/crud-be-e2e/README.md` (11,22,153) +
      `apps/crud-fs-ts-nextjs/README.md` (64) from `crud/behavior/be/gherkin` to
      `crud/behavior/crud-be/gherkin`
      — command: `grep -rl "crud/behavior/be" apps/*/README.md` — acceptance: returns no files.
  - _Suggested executor: `readme-fixer`_
- [ ] [AI] Update specs-side READMEs that reference the BE path:
      `specs/README.md` (30), `specs/apps/crud/behavior/web/gherkin/README.md` (43),
      `specs/apps/crud/components/be/component-be.md` (128), plus path examples inside the moved
      `specs/apps/crud/behavior/crud-be/README.md` (108,163,179) and its `gherkin/README.md`
      — command: `grep -rln "crud/behavior/be" specs/` — acceptance: returns no files.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `grep -rln "crud/behavior/be\b" apps/ specs/ | grep -v node_modules | grep -v .features-gen`
      returns no files (no `crud-be` false positives because the pattern ends at a word boundary).
- [ ] [AI] `npx nx affected -t spec-coverage test:quick` for crud-be projects passes
      — expected: exit 0, no spec-coverage gate failure.

> **Pause Safety**: crud backend specs relocated and fully rewired; repo is green. Safe to stop. To
> resume: `npx nx affected -t spec-coverage test:quick`.

## Phase 2: crud web — `behavior/web` → `behavior/crud-web`

- [ ] [AI] Relocate the dir:
      `git mv specs/apps/crud/behavior/web specs/apps/crud/behavior/crud-web`
      — acceptance: `test -d specs/apps/crud/behavior/crud-web/gherkin && ! test -d specs/apps/crud/behavior/web`.
- [ ] [AI] Rewire web `project.json` consumers, replacing `specs/apps/crud/behavior/web/gherkin`
      with `specs/apps/crud/behavior/crud-web/gherkin` in:
      `apps/crud-fe-dart-flutterweb/project.json` (60,79,87,89),
      `apps/crud-fe-ts-nextjs/project.json` (62,76,85,87),
      `apps/crud-fe-ts-tanstack-start/project.json` (62,76,85,87),
      `apps/crud-fs-ts-nextjs/project.json` (63,78),
      `apps/crud-fe-e2e/project.json` (20,36,67,69).
      — command: `grep -rl "crud/behavior/web" apps/*/project.json` — acceptance: returns no files.
- [ ] [AI] Update `apps/crud-fe-e2e/playwright.config.ts` (5,6): `featuresRoot` + `features` →
      `crud-web/gherkin` — acceptance: `grep -n "crud-web/gherkin" apps/crud-fe-e2e/playwright.config.ts`
      shows both lines.
- [ ] [AI] Update web README spec links: `apps/crud-fe-e2e/README.md` (3,11,25,136),
      `apps/crud-fe-ts-nextjs/README.md`, `apps/crud-fe-ts-tanstack-start/README.md`,
      `apps/crud-fe-dart-flutterweb/README.md`, `apps/crud-fs-ts-nextjs/README.md` (65)
      — command: `grep -rl "crud/behavior/web" apps/*/README.md` — acceptance: returns no files.
  - _Suggested executor: `readme-fixer`_
- [ ] [AI] Confirm no `apps/crud-fe-ts-nextjs/test/unit/steps/**/*.steps.tsx` hardcodes the gherkin
      path (per tech-docs Open Question 2):
      `grep -rn "crud/behavior/web" apps/crud-fe-ts-nextjs/test` — acceptance: returns nothing; if it
      returns a match, rewrite that literal to `crud-web/gherkin`.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `grep -rln "crud/behavior/web\b" apps/ specs/ | grep -v node_modules | grep -v .features-gen`
      returns no files.
- [ ] [AI] `npx nx affected -t spec-coverage test:quick` for crud-fe/crud-fs/crud-fe-e2e passes
      — expected: exit 0.

> **Pause Safety**: crud web specs relocated and fully rewired; repo is green. Safe to stop. To
> resume: `npx nx affected -t spec-coverage test:quick`.

## Phase 3: rhino CLI — `behavior/cli` → `behavior/rhino-cli` (TDD-shaped)

- [ ] [AI] Relocate the dir:
      `git mv specs/apps/rhino/behavior/cli specs/apps/rhino/behavior/rhino-cli`
      — acceptance: `test -d specs/apps/rhino/behavior/rhino-cli/gherkin && ! test -d specs/apps/rhino/behavior/cli`.
- [ ] [AI] **RED**: run the rhino-cli integration tests against the moved tree to confirm they fail
      on the old hardcoded path: `npx nx run rhino-cli:test:quick`
      — acceptance: tests fail because `apps/rhino-cli/tests/*.rs` still `.join` the old
      `specs/apps/rhino/behavior/cli/gherkin/...` path (file-not-found).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: update every `apps/rhino-cli/tests/*.rs` path default + doc comment from
      `specs/apps/rhino/behavior/cli/gherkin` to `specs/apps/rhino/behavior/rhino-cli/gherkin`:
      `docs.rs` (5,919), `agents.rs` (5,695), `env_validate.rs` (204), `spec_coverage.rs` (4,261),
      `env.rs` (4,948), `test_coverage.rs` (4,532), `java.rs` (4,167), `git.rs` (4,287),
      `doctor.rs` (4,379), `repo_governance.rs` (5,298), `workflows.rs` (4,190),
      `contracts.rs` (5,254); then `npx nx run rhino-cli:test:quick`
      — acceptance: all rhino-cli tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Update `apps/rhino-cli/project.json` (103,106): spec-coverage command arg + inputs →
      `specs/apps/rhino/behavior/rhino-cli/gherkin`
      — command: `grep -n "rhino/behavior/cli" apps/rhino-cli/project.json` — acceptance: returns nothing.
- [ ] [AI] Confirm `apps/rhino-cli/src/internal/specs.rs` synthetic fixture (Open Question 1) is
      family-agnostic: `grep -n "behavior/cli" apps/rhino-cli/src` — acceptance: any match is a
      generic `specs/apps/x/behavior/cli/gherkin` placeholder needing no rename, OR is rewired if it
      names rhino specifically.
- [ ] [AI] Update specs-side rhino READMEs: `specs/apps/rhino/README.md` (71,84),
      `specs/apps/rhino/components/cli/component-cli.md` (291), plus self-relative links inside the
      moved `specs/apps/rhino/behavior/rhino-cli/gherkin/**/README.md`
      — command: `grep -rln "rhino/behavior/cli" specs/` — acceptance: returns no files.
- [ ] [AI] **REFACTOR**: re-run `npx nx run rhino-cli:test:quick` and
      `npx nx run rhino-cli:spec-coverage` — acceptance: both pass; the rhino path appears exactly
      once per former call-site, no leftover `behavior/cli` literal.
  - _Suggested executor: `swe-rust-dev`_

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `grep -rln "rhino/behavior/cli\b" apps/ specs/ | grep -v node_modules | grep -v target`
      returns no files (build artifacts under `apps/rhino-cli/target/**` excluded — they regenerate).
- [ ] [AI] `npx nx run rhino-cli:test:quick && npx nx run rhino-cli:spec-coverage`
      — expected: both exit 0.

> **Pause Safety**: all three behavior dirs relocated, all app/test/spec consumers rewired, repo
> green. Safe to stop. To resume: `npx nx affected -t test:quick spec-coverage`.

## Phase 4: promote to standard + governance/docs sweep + agents + rationale

- [ ] [AI] Amend `repo-governance/conventions/structure/specs-directory-structure.md`: copy the
      amended subsection VERBATIM from the ose-public sibling plan's Phase G amendment of the same
      file (replace bare-surface naming guidance around L77–120 with the flat product-surface
      subsection; add the `be`-over-`api` rule + worked examples)
      — acceptance: `diff <(amended ose-primer subsection) <(amended ose-public subsection)` is
      empty; `grep -n "product-surface" repo-governance/conventions/structure/specs-directory-structure.md`
      shows the new rule.
  - _Suggested executor: `repo-rules-fixer`_
- [ ] [AI] Sweep governance + docs cross-refs to the new flat product-surface paths in:
      `repo-governance/development/infra/ci-conventions.md` (190,192,387),
      `repo-governance/development/infra/nx-targets.md` (404,448,452,590,595–604),
      `repo-governance/development/infra/bdd-spec-test-mapping.md` (91–93,112,181,188,193,265),
      `repo-governance/development/quality/three-level-testing-standard.md` (18,43,80,119,135,151,400,414),
      `repo-governance/development/quality/specs-application-sync.md` (156,167,191,211,254,264),
      `repo-governance/development/quality/feature-change-completeness.md` (144,165),
      `repo-governance/workflows/specs/specs-quality-gate.md` (8,56,92–94,109,279,284,292,306,318,330),
      `repo-governance/conventions/writing/dynamic-collection-references.md` (165),
      `repo-governance/conventions/structure/specs-directory-structure.md` (25,100–102,115–117),
      `docs/explanation/software-engineering/automation-testing/tools/playwright/bdd.md` (86,87,294),
      `docs/how-to/update-api-contract.md` (84), `docs/how-to/add-new-app.md` (328),
      `docs/how-to/add-new-crud-backend.md` (133,177,183,191,202,229,340,709,732),
      `docs/how-to/add-gherkin-scenario.md` (22,31,46,104,131),
      `docs/how-to/run-crud-tests.md` (168),
      `docs/reference/project-dependency-graph.md` (50,210,281),
      `README.md` (24).
      — command: `grep -rln "crud/behavior/be\b\|crud/behavior/web\b\|rhino/behavior/cli\b" repo-governance/ docs/ README.md`
      — acceptance: returns no files.
  - _Suggested executor: `docs-fixer`_
- [ ] [AI] Update `.claude/agents/specs-checker.md` (37,40,51,221,238,256): rewrite example folder
      paths to flat product-surface; add enforcement rules — (1) one `specs/apps/<family>/` tree per
      family; (2) behavior dirs MUST be flat product-surface `behavior/<product>-<surface>/gherkin/`;
      (3) reject any bare-surface (`behavior/be`, `behavior/web`, `behavior/cli`) or `api` behavior
      dir — acceptance: `grep -n "product-surface" .claude/agents/specs-checker.md` shows the rule.
  - _Suggested executor: `repo-rules-fixer`_
- [ ] [AI] Update `.claude/agents/specs-maker.md` (42 + surface-profile templates): rewrite the
      example target path and document the flat product-surface scheme so scaffolds emit
      `behavior/<product>-<surface>/gherkin/`
      — acceptance: `grep -n "product-surface" .claude/agents/specs-maker.md` shows the documented scheme.
  - _Suggested executor: `repo-rules-fixer`_
- [ ] [AI] Update `.claude/agents/specs-fixer.md` (102) example file path
      `behavior/be/README.md` → `behavior/crud-be/README.md`
      — acceptance: `grep -n "behavior/be/README" .claude/agents/specs-fixer.md` returns nothing.
- [ ] [AI] Re-sync platform bindings: `npm run generate:bindings`
      — acceptance: exits 0; `git diff --name-only .opencode/ .amazonq/` shows the regenerated
      mirrors; `npm run validate:sync` passes.
- [ ] [AI] Write the rationale doc
      `docs/explanation/standardize-app-spec-trees-parity-decisions.md` (NEW FILE; siblings:
      `docs/explanation/standardize-secrets-and-env-parity-decisions.md`,
      `docs/explanation/plan-domain-parity-decisions.md`) recording the flat product-surface scheme,
      the `be`-over-`api` rule, the crud/rhino rename map, the byte-identical-convention decision,
      and the main-to-main delivery-mode deviation
      — acceptance: `test -f docs/explanation/standardize-app-spec-trees-parity-decisions.md` and it
      contains a "main-to-main" deviation section.
  - _Suggested executor: `docs-maker`_

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] Convention amendment diff against the ose-public sibling plan's amended subsection is
      empty (byte-identical).
- [ ] [AI] `grep -rln "crud/behavior/be\b\|crud/behavior/web\b\|rhino/behavior/cli\b" apps/ specs/ repo-governance/ docs/ README.md .claude/ | grep -v node_modules | grep -v .features-gen | grep -v target`
      returns no files (live consumers fully rewired).
- [ ] [AI] `npm run validate:sync` passes and `.opencode/`/`.amazonq/` mirrors match `.claude/` sources.

> **Pause Safety**: convention amended, agents updated and re-synced, rationale written; repo green.
> Safe to stop. To resume: `npm run validate:sync && npx nx affected -t spec-coverage test:quick`.

## Phase 5: quality gates, push, archival

### Local Quality Gates (Before Push)

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.
> This follows the root cause orientation principle — proactively fix preexisting errors encountered
> during work. Commit preexisting fixes separately with appropriate conventional commit messages.

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck` — acceptance: exit 0.
- [ ] [AI] Run affected linting: `npx nx affected -t lint` — acceptance: exit 0.
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick` — acceptance: exit 0.
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage` — acceptance: exit 0.
- [ ] [AI] Validate markdown links: `npx nx run rhino-cli:validate:links` — acceptance: exit 0, no broken link reported in touched docs.
- [ ] [AI] Validate markdown style: `npm run lint:md` — acceptance: exit 0, no markdownlint violations.
- [ ] [AI] Validate Mermaid diagrams: `npx nx run rhino-cli:validate:mermaid` — acceptance: exit 0.
- [ ] [AI] Validate heading hierarchy: `npx nx run rhino-cli:validate:heading-hierarchy` — acceptance: exit 0.
- [ ] [AI] Fix ALL failures (including preexisting) and re-run failing checks to confirm resolution
      — acceptance: zero failures before pushing.

### Commit Guidelines

- [ ] [AI] Commit thematically using Conventional Commits, splitting domains into separate commits,
      e.g.: (1) `refactor(specs): adopt flat product-surface dirs for crud`,
      (2) `refactor(specs)!: adopt flat product-surface dir for rhino-cli`,
      (3) `docs(governance): standardize flat product-surface spec layout`,
      (4) `chore(agents): re-sync bindings for flat product-surface scheme`,
      (5) `docs(explanation): add standardize-app-spec-trees parity rationale`.
      — acceptance: each commit is a single cohesive concern; preexisting fixes are their own commits.

### Post-Push CI Verification

- [ ] [AI] Push changes to `main` (main-to-main, no PR — see delivery-mode deviation):
      `git push origin main` — acceptance: push accepted.
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every 3 min via
      `gh run view --json status,conclusion`; do NOT use `gh run watch`)
      — acceptance: every workflow reaches `completed`/`success`.
- [ ] [AI] If any CI check fails, fix the root cause and push a follow-up commit; repeat until ALL
      GitHub Actions pass — acceptance: zero failing checks.
- [ ] [AI] Do NOT proceed to archival until CI is fully green.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI).
- [ ] [AI] Rename and move:
      `git mv plans/in-progress/standardize-app-spec-trees/ plans/done/YYYY-MM-DD__standardize-app-spec-trees/`
      using today's date as the completion date (NOT a creation date).
- [ ] [AI] Confirm `plans/in-progress/README.md` has no entry for `standardize-app-spec-trees`
      (grep returns nothing); if one exists, remove it — acceptance:
      `grep -c "standardize-app-spec-trees" plans/in-progress/README.md` returns `0`.
- [ ] [AI] Update `plans/done/README.md` — add the entry with completion date.
- [ ] [AI] Update any other READMEs that reference this plan (e.g. `plans/README.md`).
- [ ] [AI] Commit the archival: `chore(plans): move standardize-app-spec-trees to done`.

### Phase 5 Gate

> All checks below must pass to consider the plan complete.

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` all exit 0.
- [ ] [AI] All pushed CI workflows are green (`gh run list --limit 5` shows success for the push).
- [ ] [AI] Plan folder now lives under `plans/done/YYYY-MM-DD__standardize-app-spec-trees/` and the
      in-progress/done READMEs are updated.

> **Pause Safety**: all work delivered, pushed, CI green, plan archived. Terminal state — nothing
> further to resume. To re-verify: `npx nx affected -t test:quick spec-coverage`.
