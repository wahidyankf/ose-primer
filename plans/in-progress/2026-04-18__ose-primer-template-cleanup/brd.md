# Business Requirements Document — ose-primer Template Cleanup

## Business Goal

Turn `ose-primer` into a production-ready repository template so future OSE-style repos bootstrap in hours rather than days. The fork currently ships three live product websites, a production F#/Giraffe backend, deprecated Hugo tooling, and ~25 product-specific agents and skills — all of which leak brand-specific assumptions into any repo that starts from it. A clean template keeps the polyglot `a-demo-*` scaffolding, repo tooling (`rhino-cli`), governance, and generic agents/skills while removing every trace of AyoKoding, OSE Platform, OrganicLever, and Hugo.

## Business Impact

### Pain points today

- A contributor cloning `ose-primer` inherits live product codebases they must manually identify and delete before using the template.
- Product-specific agents and skills auto-load in cloned repos, silently steering AI workflows toward removed apps.
- Governance documents enumerate product apps in prose, making it non-obvious which examples are generic scaffolding vs brand-specific carryover.
- `package.json` scripts, CI workflows, and `infra/dev/` Docker composes reference apps that no longer exist, producing broken commands on first clone.
- `hugo-commons` lib and `swe-hugo-dev` agent stay live even though Hugo is formally deprecated across the org.

### Expected benefits

- Fresh template provides exactly one domain: polyglot demo apps (`a-demo-*` backends + frontends + contracts + E2E) plus repo tooling (`rhino-cli`) plus generic governance + agents + skills.
- Cloners can `git clone`, delete the `a-demo-*` variants they don't need, rename the repo, and start product work without further cleanup.
- Agent / skill catalog contains only generic automation that applies to any new repo.
- Command surface (`package.json` scripts, Nx targets, CI workflows) matches what exists on disk — no 404s.

## Affected Roles

Solo-maintainer repository; no sign-off ceremonies. The hats worn during and after this cleanup:

- **Template maintainer (primary hat)** — owns `ose-primer` going forward; decides which scaffolding survives.
- **Future cloner** — downstream user who clones `ose-primer` to bootstrap a new repo.
- **Agent consumers** — `plan-executor`, `plan-checker`, `plan-execution-checker`, `repo-rules-checker`, `repo-rules-fixer` read the cleaned `.claude/` and governance to validate subsequent work.

## Success Metrics

Metric honesty: observable facts, confirmable with a single shell command at the cleanup finish line; one judgment call is labelled as such. No fabricated KPIs.

### Observable (verifiable at cleanup completion)

1. **Product-reference grep is empty** outside historical plans:
   `rtk grep -R -in "ayokoding\|oseplatform\|organiclever\|hugo-commons" apps libs specs scripts infra archived .github .claude .opencode governance docs README.md CLAUDE.md AGENTS.md LICENSING-NOTICE.md package.json nx.json tsconfig.base.json | rg -v "plans/done/"` returns zero lines.
2. **Doctor passes**: `npm run doctor` exits 0.
3. **Affected quality gates pass**: `npx nx affected -t typecheck lint test:quick spec-coverage --base=HEAD~1` exits 0 after each phase commit.
4. **Full quality gates pass**: `npx nx run-many -t typecheck lint test:quick spec-coverage` exits 0 at the end of Phase 16.
5. **Markdown lint clean**: `npm run lint:md` exits 0.
6. **Nx project count drops correctly**: `nx show projects` lists only kept projects — 17 `a-demo-*` + `rhino-cli` + `a-demo-contracts` (if tracked) — and no `ayokoding-*`, `oseplatform-*`, or `organiclever-*`.
7. **`repo-rules-checker` reports zero CRITICAL and zero HIGH findings** (or a clean double-zero pass) at cleanup end.

### Judgment call (labelled)

- **Template usability**: a fresh clone is "productive enough" that a new user can clone, delete unwanted `a-demo-*` variants, rename the repo, and start product work. No baseline was measured; this is a reasoned expectation based on what the `a-demo-*` suite already exercises (17 polyglot backends, 3 frontend variants, contracts codegen, E2E harness). Judgment call.

## Business-Scope Non-Goals

- **No new scaffolding** — this cleanup removes; it does not add new demo languages, new libs, new agents, or new governance.
- **No governance restructuring** — governance directory shape stays unchanged; only product-specific enumerations get generalised.
- **No retroactive license change to `ose-public` or other forks** — the MIT switch is scoped to `ose-primer` only; upstream repos retain their existing policies.
- **No remote env-branch cleanup** — `wahidyankf/ose-primer` is a fresh remote with only `main`; there are no `prod-*` branches to delete remotely.
- **No work on sibling repos** — `ose-public` and `ose-infra` are outside this plan's scope.

## Business Risks and Mitigations

### R1 — Hidden Nx target dependency on a removed project

- **Risk**: an `a-demo-*` app or a kept lib implicitly depends on a removed project (e.g., via `implicitDependencies`, a shared generator, or an Nx target input); removal breaks its build.
- **Likelihood**: Low (the `a-demo-*` suite is self-contained by design) but not zero.
- **Mitigation**: run `nx show projects` and `npx nx graph --file=local-temp/post-phase-nx-graph.json` after every deletion phase; run `npx nx affected -t typecheck lint --base=HEAD` before each phase commit. Fix any broken edge in the same phase before committing.

### R2 — Governance prose references slip past single-term greps

- **Risk**: governance or docs files mention a removed product in narrative form using wording a simple grep on "ayokoding" misses (e.g., "our primary content site").
- **Likelihood**: Medium.
- **Mitigation**: final validation step runs the multi-term grep sweep defined in the Observable metric above; `repo-rules-checker` runs before the final commit and is expected to flag residual references as findings.

### R3 — `.opencode/` drifts from `.claude/`

- **Risk**: `.opencode/` edits or deletes get forgotten, leaving OpenCode users with agents / skills that no longer exist on the Claude side.
- **Likelihood**: Medium (two parallel trees).
- **Mitigation**: dedicated Phase 16 runs `npm run sync:claude-to-opencode` after every `.claude/`-touching phase has landed, and commits sync artefacts in the same commit as any straggler change.

### R4 — CI pipeline references a workflow file that was deleted

- **Risk**: a removed workflow file is still wired into a scheduled or reusable workflow elsewhere.
- **Likelihood**: Low (workflows are file-scoped).
- **Mitigation**: audit `.github/workflows/_reusable-*.yml` and `pr-quality-gate.yml` in Phase 15 for `uses:` / `needs:` entries pointing at deleted files; fix in the same commit.

### R5 — Loss of template utility if too much is removed

- **Risk**: the cleanup strips a piece of scaffolding that was useful as template material (e.g., a generic Diátaxis tutorial incidentally using an OrganicLever example).
- **Likelihood**: Medium for docs/; Low for apps/.
- **Mitigation**: for governance and docs, the default action is **rewrite** (generalise the example), not **delete**. A file is deleted only when its sole subject is a removed product. When in doubt, generalise.

## Cross-Cutting Concerns

Post-cleanup, `ose-primer` is licensed uniformly under MIT across the entire repo — apps, libs, specs, governance, docs, tooling. This is a policy shift from `ose-public`, motivated by template-repo ergonomics: templates should be maximally permissive to support downstream relicensing. `ose-primer` is a repository template, not a product repo. MIT is standard across template repos in the ecosystem; FSL-1.1-MIT's Functional Source License semantics (delayed open-source grant, limited competitive use) add friction without value for a template. The testable scenarios for verifying the rewritten license files live in [prd.md](./prd.md) (AC-7). This BRD asserts only the business rationale: the template must carry the most permissive license so downstream cloners can relicense freely without encountering FSL constraints they did not choose.
