# Technical Documentation — ose-primer Template Cleanup

## Architecture

No runtime architecture changes. The technical work is content surgery on a single-repo Nx monorepo. The relevant structures:

- **Nx workspace** — apps/libs/specs wired through `nx.json`, `project.json` per project, and `tsconfig.base.json` path aliases. Removing a project requires no `nx.json` edit (project discovery is directory-driven); removing a lib aliased in `tsconfig.base.json` may require a `paths` prune.
- **`.claude/` and `.opencode/` trees** — two parallel agent/skill trees. `.claude/` is source of truth; `.opencode/` is regenerated via `npm run sync:claude-to-opencode`. Sync tool is `rhino-cli agents sync`.
- **Governance and docs** — markdown-only trees; references to product apps are in prose enumerations and example code blocks.
- **CI workflows** — `.github/workflows/` contains per-project `test-*.yml` files and per-product `test-and-deploy-*.yml` files. Workflow discovery is file-driven; removing a file removes the workflow.
- **Pre-push hook** — Husky pre-push runs `nx affected -t typecheck lint test:quick spec-coverage` + markdown lint. Must stay green between every phase commit.

## File-Impact Analysis

### Phase 1 — Product apps (delete)

| Path                           | Action      | Notes                   |
| ------------------------------ | ----------- | ----------------------- |
| `apps/ayokoding-web/`          | `git rm -r` | Next.js content site    |
| `apps/ayokoding-web-be-e2e/`   | `git rm -r` | Playwright BE E2E       |
| `apps/ayokoding-web-fe-e2e/`   | `git rm -r` | Playwright FE E2E       |
| `apps/ayokoding-cli/`          | `git rm -r` | Go CLI link validator   |
| `apps/oseplatform-web/`        | `git rm -r` | Next.js marketing site  |
| `apps/oseplatform-web-be-e2e/` | `git rm -r` |                         |
| `apps/oseplatform-web-fe-e2e/` | `git rm -r` |                         |
| `apps/oseplatform-cli/`        | `git rm -r` | Go CLI site maintenance |
| `apps/organiclever-fe/`        | `git rm -r` | Next.js landing site    |
| `apps/organiclever-fe-e2e/`    | `git rm -r` |                         |
| `apps/organiclever-be/`        | `git rm -r` | F#/Giraffe backend      |
| `apps/organiclever-be-e2e/`    | `git rm -r` |                         |

### Phase 2 — Product specs (delete)

| Path                       | Action      |
| -------------------------- | ----------- |
| `specs/apps/ayokoding/`    | `git rm -r` |
| `specs/apps/organiclever/` | `git rm -r` |
| `specs/apps/oseplatform/`  | `git rm -r` |

Kept under `specs/apps/`: `a-demo/`, `rhino/`. Kept under `specs/libs/`: `golang-commons/`, `ts-ui/`. `specs/libs/hugo-commons/` is removed as part of Phase 3. (Nx projects `organiclever-contracts`, `ayokoding-*` E2E-spec projects if any, etc., disappear as their source directories are removed — Nx project discovery is directory-driven; verify with `nx show projects` after this phase.)

### Phase 3 — Deprecated lib (delete)

| Path                       | Action      | Notes                                      |
| -------------------------- | ----------- | ------------------------------------------ |
| `libs/hugo-commons/`       | `git rm -r` | No demo app imports it (verified via grep) |
| `specs/libs/hugo-commons/` | `git rm -r` | Paired spec tree                           |

### Phase 4 — Product agents (delete, both mirrors)

Per `.claude/agents/` and `.opencode/agent/` (20 agents × 2 trees = 40 files):

- `apps-ayokoding-web-by-example-{maker,checker,fixer}.md` (3)
- `apps-ayokoding-web-in-the-field-{maker,checker,fixer}.md` (3)
- `apps-ayokoding-web-general-{maker,checker,fixer}.md` (3)
- `apps-ayokoding-web-facts-{checker,fixer}.md` (2)
- `apps-ayokoding-web-link-{checker,fixer}.md` (2)
- `apps-ayokoding-web-deployer.md` (1)
- `apps-oseplatform-web-content-{maker,checker,fixer}.md` (3)
- `apps-oseplatform-web-deployer.md` (1)
- `apps-organiclever-fe-deployer.md` (1)
- `swe-hugo-dev.md` (1)

Deletion happens in `.claude/` only during Phase 4; `.opencode/` mirrors land via Phase 16 sync. Dry-running or direct `git rm` on `.opencode/` is fine as long as Phase 16 sync produces no diff afterwards.

### Phase 5 — Product skills (delete, both mirrors)

Per `.claude/skills/` and `.opencode/skill/` (3 skills × 2 trees = 6 directories):

- `apps-ayokoding-web-developing-content/`
- `apps-organiclever-fe-developing-content/`
- `apps-oseplatform-web-developing-content/`

Same rule as agents: delete `.claude/` in Phase 5, let Phase 16 sync reconcile `.opencode/`.

### Phase 6 — Remove all other plans + clean ideas + generated-socials

- Rewrite `plans/ideas.md` to a minimal template-generic state — a single H1, a one-sentence description, and an empty bullet list (no inherited product ideas).
- Verify `plans/backlog/README.md` is untouched (already empty).
- `generated-socials/` removal remains the existing no-op verification.

### Phase 7 — Rewrite `CLAUDE.md`

Rewrite sections (do not delete the file):

- "Project Overview" paragraph — drop "Phase 1 (OrganicLever - Productivity Tracker)"; reframe as "Repository template".
- "Current Apps" list — keep only `a-demo-*`, `rhino-cli`, `a-demo-contracts`. Delete all `ayokoding-*`, `oseplatform-*`, `organiclever-*` bullets.
- "Project Structure" tree — strip the same apps.
- "Coverage thresholds" table — remove rows for `ayokoding-web`, `oseplatform-web`, `organiclever-fe`, `organiclever-be`.
- "Git Workflow" — remove the three `prod-*` branch bullets.
- "AI Agents" catalog — drop removed agents from each role grouping.
- "Web Sites" section — delete entirely (all three sub-sections were product-specific).
- Closing verification: `rtk grep -n 'ayokoding\|oseplatform\|organiclever\|hugo-commons' CLAUDE.md` returns empty.

### Phase 8 — Rewrite top-level `README.md`

Full reframing as template entry point. New section order with expected content per section:

1. **What this is** — 2-3 sentences framing `ose-primer` as a cloneable/cherry-pickable template for OSE-style polyglot monorepos; NOT a product repo.
2. **What it ships** — bullet list covering polyglot `a-demo-*` scaffolding (11 backends, 3 frontends, 1 fullstack, contracts, E2E), `rhino-cli` repo tooling, shared libs, governance, generic agents/skills, planning infrastructure.
3. **How to use this template** — step-by-step: `git clone`, choose `a-demo-*` variants to keep, delete unwanted variants, rename via search-and-replace or `rhino-cli` helpers, point `origin` at the new remote, push to `main`.
4. **Prerequisites** — Volta + Node pinned version; single-command setup `npm install && npm run doctor -- --fix`.
5. **Common commands** — `nx build`, `nx affected -t …`, `npm run lint:md`, `npm run doctor`, `npm run sync:claude-to-opencode`.
6. **Governance & conventions** — link to `governance/README.md` and list of top-level principle categories.
7. **Repository layout** — brief ASCII or bullet tree showing `apps/`, `libs/`, `specs/`, `governance/`, `docs/`, `plans/`, `.claude/`, `.opencode/`.
8. **License** — short statement: "MIT. See `LICENSE` and `LICENSING-NOTICE.md`."

The rewritten README is the first-read onboarding doc for a new cloner; it must explain what the template ships, how to use it, and where to look next, without assuming familiarity with the `ose-public` history.

### Phase 9 — `AGENTS.md` (OpenCode mirror)

Mirror the Phase 7 edits one-for-one. `AGENTS.md` is a parallel doc for OpenCode tooling.

### Phase 10 — `.claude/agents/README.md` + `.claude/settings.json`

- `.claude/agents/README.md` — drop removed agents from every table and role grouping.

### Phase 11 — Governance audit (`governance/**`)

Run `rtk grep -rn "ayokoding\|oseplatform\|organiclever\|hugo" governance/`. Expected hit sites (not exhaustive):

- `governance/conventions/structure/plans.md` — examples may reference removed apps; generalise.
- `governance/development/workflow/*.md` — env-branch conventions may reference `prod-ayokoding-web` etc.; delete those bullets.
- `governance/workflows/**` — may name removed agents; rewrite or delete per PR-4 risk.
- `governance/vision/**` — `Phase 1: OrganicLever` framing needs generalising.
- `governance/development/agents/**` — agent-workflow-orchestration examples.

**Decision rule**: if a file's sole subject is a removed product, `git rm` it. Otherwise, rewrite examples using generic placeholders (`<app-name>`, `your-app`). Commit per logical subgrouping (conventions, development, principles, workflows, vision) — at least one commit per subgrouping to keep diffs reviewable.

### Phase 12 — Docs audit (`docs/**`)

Same approach as Phase 11 but targeting `docs/tutorials/`, `docs/how-to/`, `docs/reference/`, `docs/explanation/`, `docs/metadata/`. Product-sole-subject tutorials (e.g., any tutorial walking through deploying AyoKoding) get deleted. Generic tutorials that happen to use a product example get the example generalised.

### Phase 12.5 — Audit every remaining markdown file under kept paths

All remaining `.md` files under `apps/`, `libs/`, `specs/`, `infra/`, `apps-labs/`, `archived/`, `.claude/`, `.opencode/`, `governance/`, `docs/`, and `plans/` must be scrubbed of product-brand references. Earlier phases cover governance and docs trees; this phase extends the audit explicitly to per-app READMEs, per-lib READMEs, kept agent bodies, kept skill bodies and their reference modules, spec READMEs, infra-dev READMEs, archived README, and apps-labs README. Generic placeholder substitutions are preferred over deletion; a file is deleted only when its sole subject is a removed product. This phase is a **safety-net audit** — if earlier phases caught everything, this phase is a no-op; if anything slipped, this phase catches it.

### Phase 13 — `LICENSE` + `LICENSING-NOTICE.md` + license metadata (MIT switch)

- Replace `LICENSE` file with the canonical MIT license text, preserving the existing copyright holder name and adjusting the year to 2026.
- Rewrite `LICENSING-NOTICE.md` to a short MIT-only statement: (1) the entire repo is MIT-licensed; (2) no FSL split; (3) pointer to `LICENSE`; (4) note that this is a policy shift from `ose-public`.
- Edit `package.json` top-level `license` field from `FSL-1.1-MIT` to `MIT`.
- For every kept app/lib, update any embedded license metadata (`package.json`, `pyproject.toml`, `Cargo.toml`, `.csproj`, `pom.xml`, `mix.exs`, `deps.edn`, `pubspec.yaml`, etc.) to MIT. Verify via `rtk grep -rn "FSL-1.1-MIT" apps libs`.

### Phase 14 — Tooling files

- `package.json` — delete scripts that reference removed apps: `demo-be:*` if they point at removed projects (verify — `a-demo-be-java-springboot` is kept, so `demo-be:dev` may stay), `organiclever:*`, `dev:ayokoding-web`, `dev:oseplatform-web`, `dev:organiclever`, `dev:ayokoding-cli`, `dev:oseplatform-cli`.
- `nx.json` — audit for explicit project references (should be project-agnostic; confirm).
- `tsconfig.base.json` — audit `compilerOptions.paths` for aliases pointing at removed libs.
- `.github/workflows/test-and-deploy-ayokoding-web.yml`, `test-and-deploy-oseplatform-web.yml`, `test-and-deploy-organiclever.yml` — `git rm`.
- `_reusable-test-and-deploy.yml` — the only callers are the three `test-and-deploy-*.yml` workflows deleted above. Once those are removed, this reusable has zero consumers and must be deleted in the same commit. Verify with `grep -l "_reusable-test-and-deploy" .github/workflows` before deleting.
- `.github/workflows/pr-quality-gate.yml`, `_reusable-*.yml`, and `codecov-upload.yml` — audit for orphan `needs:` / `uses:` references to deleted workflows and for product-specific matrix entries, job filters, or path triggers pointing at removed projects; fix or strip matches.
- `.github/workflows/test-a-demo-be-java-springboot.yml` — confirmed on 2026-04-18 to contain product-brand references via grep. Do NOT delete this file; the `a-demo-be-java-springboot` app is a KEPT app. Scrub only the stale product references (comments, matrix entries, job names) in place.

### Phase 15 — `.opencode/` sync

Run `npm run sync:claude-to-opencode`. Verify:

- No removed agent file remains in `.opencode/agent/`.
- No removed skill directory remains in `.opencode/skill/`.
- `rtk git diff .opencode/` after sync shows only expected changes.

### Phase 16 — Final validation

Full-workspace run: `npx nx run-many -t typecheck lint test:quick spec-coverage --parallel=<cores-1>`. Markdown lint. `repo-rules-checker` agent. Residual grep sweep per AC-2.

### Phase 17 — Archive

`git mv plans/in-progress/2026-04-18__ose-primer-template-cleanup plans/done/`. Update both `plans/*/README.md` files.

## Design Decisions

### D1 — Delete via `git rm`, never `rm -rf`

`git rm -r` preserves history. Every deletion must go through git. Rationale: the cleanup is permanent on `main`, but the historical ability to retrieve a file via `git log --all --oneline -- path/to/file` is valuable for template maintainers who later want to resurrect a pattern.

### D2 — One commit per phase category, not per file

Phases 1, 2, 4, 5 each bundle their mechanically identical deletions into one commit with a Conventional Commits message (`chore(cleanup): remove product apps`, etc.). Rationale: no reviewer benefit in separating the `ayokoding-web` deletion from the `oseplatform-web` deletion; both are the same class of change. Phase 11 (governance) splits commits by subgrouping because those changes are more semantic.

### D3 — Governance and docs: rewrite by default, delete only product-sole-subject files

A file is kept + rewritten unless its **entire subject** is a removed product. This minimises template material loss (PR-5 mitigation).

### D4 — `.opencode/` sync is its own phase, not inlined

All `.claude/` edits land first (phases 4, 5, 10, 11, 12). Phase 15 runs the sync tool once and commits any delta. Rationale: avoids N small `chore(opencode): sync` commits interleaved throughout.

### D5 — Direct trunk-based work on `main`, no PR

Per the user's scope statement: this is parent-repo-only work; `ose-projects` parent-repo worktree rules do not apply because `ose-primer` is a standalone single-repo monorepo. Trunk-based default says: commit to `main`, push to `main`, no PR. See `governance/development/workflow/trunk-based-development.md`.

### D6 — No `--no-verify`, no `--force`

Every commit passes the Husky pre-push gate (`nx affected -t typecheck lint test:quick spec-coverage` + markdown lint). If a gate fires, fix root cause in the same phase before committing. Never bypass.

### D7 — Fix all issues encountered, even preexisting

Per the Root Cause Orientation principle and `governance/development/quality/ci-blocker-resolution.md`: any preexisting `typecheck` / `lint` / `test:quick` failure surfaced during cleanup gets fixed in its own thematic commit (e.g., `fix(a-demo-be-fsharp-giraffe): repair typecheck regression surfaced during template cleanup`), not deferred.

### D8 — Template ships with empty plans history

`plans/done/` is wiped of all 53 inherited archive entries because a template should not carry its source repo's development history. The cleanup plan itself archives into `plans/done/` at Phase 17, so by the time a cloner clones, `plans/done/` has exactly one entry — the plan that created the template.

### D9 — Explicit per-file markdown audit as safety net

A comprehensive sweep covers all product-brand references, but an explicit per-file audit of every surviving markdown file provides defense in depth. A cloner opening `apps/a-demo-be-golang-gin/README.md` first must not find a product reference left over from the source repo; the explicit audit guarantees this, rather than relying on the reviewer having inspected the sweep output.

## Dependencies

- Node 24.13.1, npm 11.10.1 (Volta-pinned).
- Nx 22.5.2.
- `rhino-cli` (kept) — source of `doctor`, `agents sync`, `spec-coverage validate`, `test-coverage validate`. Must remain buildable throughout.
- Husky pre-push hook — stays armed for the entire execution.
- GitHub Actions CI — pre-push gate is local; post-push CI runs on `main` push.

## Rollback Strategy

Every phase commits independently. To roll back a single phase:

```bash
rtk git log --oneline --grep "chore(cleanup)"
rtk git revert <commit-sha>
```

Reverting later phases does not require reverting earlier phases. The only cross-phase dependency is Phase 15 (`.opencode/` sync) — if Phases 4, 5, 10, 11, or 12 are reverted, Phase 15 should also be reverted (or Phase 15's work re-run).

No history rewrite (`rebase -i`, `push --force`) under any circumstance.

## Order Dependencies (why the phase order is not arbitrary)

- **Deletions first (1-6), rewrites second (7-14)**: rewriting `CLAUDE.md` to describe "only the `a-demo-*` apps" is only meaningful once the product apps are actually gone from disk.
- **`.opencode/` sync last (15)**: running it before `.claude/` is stable would produce a useless diff that then gets reverted.
- **Final validation after sync (16)**: the grep sweep must run against the fully-synced tree.
- **Archive absolutely last (17)**: the plan's own files have to stay readable to the executor through Phase 16.

## Validation Strategy

- **Per-phase local check**: `npx nx affected -t typecheck lint --base=HEAD` before each phase commit.
- **Per-phase post-commit check**: Husky pre-push gate runs automatically if `git push` is issued between phases; if not pushing between phases, that's fine — final push (Phase 16) runs the gate once at the end.
- **Full workspace sweep (Phase 16)**: `npx nx run-many -t typecheck lint test:quick spec-coverage`.
- **Governance sweep (Phase 16)**: `repo-rules-checker` agent produces a report under `generated-reports/`. If it flags CRITICAL or HIGH findings, `repo-rules-fixer` loop runs until double-zero.

## Out-of-Scope but Worth Noting

- **`generated-socials/` directory**: verified absent on 2026-04-18 via `ls`. Phase 6 keeps the deletion step as a no-op safety net in case a session creates it between now and execution.
- **`apps-labs/`**: contains `README.md` only (no removed-product content). Not touched.
- **`archived/` was intended for legacy product snapshots**: after Phase 14 removes all three product snapshots, the `archived/` directory is empty except for its `README.md`. Decision: keep the empty-ish directory + its README for now; future maintainers can decide whether the concept still applies to a template.
