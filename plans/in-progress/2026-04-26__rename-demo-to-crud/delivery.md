# Delivery Checklist: Rename `demo-*` → `crud-*`

## Phase 0 — Worktree setup

- [ ] `cd /Users/wkf/ose-projects/ose-primer`
- [ ] `git worktree add .claude/worktrees/rename-demo-to-crud -b worktree-rename-demo-to-crud origin/HEAD`
- [ ] `cd .claude/worktrees/rename-demo-to-crud`
- [ ] `npm install`
- [ ] `npm run doctor -- --fix`
- [ ] Verify `git status` is clean before proceeding

## Phase 1 — Rename app directories (`git mv`)

- [ ] `git mv apps/demo-be-clojure-pedestal apps/crud-be-clojure-pedestal`
- [ ] `git mv apps/demo-be-csharp-aspnetcore apps/crud-be-csharp-aspnetcore`
- [ ] `git mv apps/demo-be-elixir-phoenix apps/crud-be-elixir-phoenix`
- [ ] `git mv apps/demo-be-fsharp-giraffe apps/crud-be-fsharp-giraffe`
- [ ] `git mv apps/demo-be-golang-gin apps/crud-be-golang-gin`
- [ ] `git mv apps/demo-be-java-springboot apps/crud-be-java-springboot`
- [ ] `git mv apps/demo-be-java-vertx apps/crud-be-java-vertx`
- [ ] `git mv apps/demo-be-kotlin-ktor apps/crud-be-kotlin-ktor`
- [ ] `git mv apps/demo-be-python-fastapi apps/crud-be-python-fastapi`
- [ ] `git mv apps/demo-be-rust-axum apps/crud-be-rust-axum`
- [ ] `git mv apps/demo-be-ts-effect apps/crud-be-ts-effect`
- [ ] `git mv apps/demo-fe-dart-flutterweb apps/crud-fe-dart-flutterweb`
- [ ] `git mv apps/demo-fe-ts-nextjs apps/crud-fe-ts-nextjs`
- [ ] `git mv apps/demo-fe-ts-tanstack-start apps/crud-fe-ts-tanstack-start`
- [ ] `git mv apps/demo-fs-ts-nextjs apps/crud-fs-ts-nextjs`
- [ ] `git mv apps/demo-be-e2e apps/crud-be-e2e`
- [ ] `git mv apps/demo-fe-e2e apps/crud-fe-e2e`

## Phase 2 — Rename infra directories (`git mv`)

- [ ] `git mv infra/dev/demo-be-clojure-pedestal infra/dev/crud-be-clojure-pedestal` (if exists)
- [ ] `git mv infra/dev/demo-be-csharp-aspnetcore infra/dev/crud-be-csharp-aspnetcore` (if exists)
- [ ] `git mv infra/dev/demo-be-elixir-phoenix infra/dev/crud-be-elixir-phoenix` (if exists)
- [ ] `git mv infra/dev/demo-be-fsharp-giraffe infra/dev/crud-be-fsharp-giraffe` (if exists)
- [ ] `git mv infra/dev/demo-be-golang-gin infra/dev/crud-be-golang-gin` (if exists)
- [ ] `git mv infra/dev/demo-be-java-springboot infra/dev/crud-be-java-springboot` (if exists)
- [ ] `git mv infra/dev/demo-be-java-vertx infra/dev/crud-be-java-vertx` (if exists)
- [ ] `git mv infra/dev/demo-be-kotlin-ktor infra/dev/crud-be-kotlin-ktor` (if exists)
- [ ] `git mv infra/dev/demo-be-python-fastapi infra/dev/crud-be-python-fastapi` (if exists)
- [ ] `git mv infra/dev/demo-be-rust-axum infra/dev/crud-be-rust-axum` (if exists)
- [ ] `git mv infra/dev/demo-be-ts-effect infra/dev/crud-be-ts-effect` (if exists)
- [ ] `git mv infra/dev/demo-fe-ts-nextjs infra/dev/crud-fe-ts-nextjs` (if exists)
- [ ] `git mv infra/dev/demo-fe-ts-tanstack-start infra/dev/crud-fe-ts-tanstack-start` (if exists)
- [ ] `git mv infra/dev/demo-fe-dart-flutterweb infra/dev/crud-fe-dart-flutterweb` (if exists)
- [ ] `git mv infra/dev/demo-fs-ts-nextjs infra/dev/crud-fs-ts-nextjs` (if exists)

## Phase 3 — Rename specs tree (`git mv`)

- [ ] `git mv specs/apps/demo specs/apps/crud`
- [ ] Verify `specs/apps/crud/contracts/project.json` now exists (was `specs/apps/demo/contracts/project.json`)

## Phase 4 — Bulk string replacement (JSON / YAML / MD)

Run the bulk sweeps below. Each targets a specific string class to avoid
accidental over-replacement.

### 4a — App name prefixes in all JSON/YAML/MD (excludes plans/done, generated-reports)

- [ ] Run:

  ```bash
  find . \
    -not -path "./.git/*" -not -path "./node_modules/*" \
    -not -path "./plans/done/*" -not -path "./generated-reports/*" \
    -not -path "./.claude/worktrees/*" \
    -not -path "./plans/in-progress/*" \
    -type f \( -name "*.json" -o -name "*.yaml" -o -name "*.yml" -o -name "*.md" \) \
    | xargs sed -i '' \
      -e 's/demo-be-/crud-be-/g' \
      -e 's/demo-fe-/crud-fe-/g' \
      -e 's/demo-fs-/crud-fs-/g'
  ```

### 4b — Docker DB credential prefix (`demo_be_` → `crud_be_`)

- [ ] Run:

  ```bash
  find . \
    -not -path "./.git/*" -not -path "./node_modules/*" \
    -not -path "./plans/done/*" -not -path "./generated-reports/*" \
    -not -path "./.claude/worktrees/*" \
    -not -path "./plans/in-progress/*" \
    -type f \( -name "*.yml" -o -name "*.yaml" \) \
    | xargs sed -i '' 's/demo_be_/crud_be_/g'
  ```

### 4c — Nx project name string `"demo-contracts"` in JSON

- [ ] Run:

  ```bash
  find . \
    -not -path "./.git/*" -not -path "./node_modules/*" \
    -not -path "./plans/done/*" -not -path "./generated-reports/*" \
    -not -path "./.claude/worktrees/*" \
    -not -path "./plans/in-progress/*" \
    -type f -name "*.json" \
    | xargs sed -i '' 's/"demo-contracts"/"crud-contracts"/g'
  ```

### 4d — Spec path prefix `specs/apps/demo/` in all JSON/YAML/MD

- [ ] Run:

  ```bash
  find . \
    -not -path "./.git/*" -not -path "./node_modules/*" \
    -not -path "./plans/done/*" -not -path "./generated-reports/*" \
    -not -path "./.claude/worktrees/*" \
    -not -path "./plans/in-progress/*" \
    -type f \( -name "*.json" -o -name "*.yaml" -o -name "*.yml" -o -name "*.md" \) \
    | xargs sed -i '' 's|specs/apps/demo/|specs/apps/crud/|g'
  ```

## Phase 5 — Source-language app config files

> Prerequisite: Phase 1 (directory renames) must be complete. Apps are already at `apps/crud-be-*` and `apps/crud-fe-dart-flutterweb`.

Each backend stores its DB connection config in a language-specific file. Sweep all
source file types inside the renamed backend app directories:

- [ ] Run:

  ```bash
  find apps/crud-be-* -type f \
    \( -name "*.yml" -o -name "*.yaml" -o -name "*.py" \
       -o -name "*.exs" -o -name "*.ex" -o -name "*.conf" \
       -o -name "*.toml" -o -name "*.properties" \
       -o -name "*.clj" -o -name "*.edn" -o -name "*.kts" \
       -o -name "*.xml" -o -name "*.fs" -o -name "*.fsx" \
       -o -name "*.cs" -o -name "*.kt" -o -name "*.rs" \) \
    | xargs grep -l "demo_be_" 2>/dev/null \
    | xargs sed -i '' 's/demo_be_/crud_be_/g'
  ```

- [ ] Sweep Dart frontend for underscore-form package names:

  ```bash
  # pubspec.yaml: demo_fe_dart_flutterweb → crud_fe_dart_flutterweb, demo_contracts → crud_contracts
  sed -i '' 's/demo_fe_dart_flutterweb/crud_fe_dart_flutterweb/g; s/demo_contracts/crud_contracts/g' \
    apps/crud-fe-dart-flutterweb/pubspec.yaml
  ```

- [ ] Sweep `project.json` codegen command for `pubName=demo_contracts`:

  ```bash
  sed -i '' 's/pubName=demo_contracts/pubName=crud_contracts/g' \
    apps/crud-fe-dart-flutterweb/project.json
  ```

- [ ] Verify no `demo_be_` remains in backend source configs:

  ```bash
  grep -r "demo_be_" apps/crud-be-* \
    --include="*.yml" --include="*.yaml" --include="*.properties" \
    --include="*.py" --include="*.exs" --include="*.toml" --include="*.xml" \
    --include="*.kt" --include="*.rs" --include="*.cs" --include="*.clj" \
    --include="*.ex" --include="*.fsx" --include="*.fs" --include="*.kts" \
    --include="*.conf" --include="*.edn"
  ```

- [ ] Verify no underscore-form `demo_` names remain in Dart frontend:
      `grep -r "demo_fe_\|pubName=demo_contracts" apps/crud-fe-dart-flutterweb/ --include="pubspec.yaml" --include="*.json"`

## Phase 6 — Verify Nx project.json files (manual spot-check)

- [ ] Open `apps/crud-be-golang-gin/project.json` — confirm `"name": "crud-be-golang-gin"`, `dependsOn: ["crud-contracts:bundle"]`, and all path strings use `crud-`
- [ ] Open `apps/crud-contracts/project.json` (if moved) OR `specs/apps/crud/contracts/project.json` — confirm `"name": "crud-contracts"` and `"root": "specs/apps/crud/contracts"`
- [ ] Open `apps/crud-be-e2e/project.json` — confirm all 11 `implicitDependencies` list `crud-*` names
- [ ] Open `apps/crud-fe-e2e/project.json` — confirm all 3 `implicitDependencies` list `crud-*` names
- [ ] Run stale-name check: `grep -r '"demo-' apps/ specs/ --include="*.json" | grep -v node_modules | grep -v ".git/"` → expect zero results

## Phase 7 — Root `package.json` npm scripts

- [ ] Open root `package.json`, verify all `dev:demo-*` scripts renamed to `dev:crud-*`
- [ ] Verify `demo-be:dev`, `demo-be:dev:restart`, `demo-be:clean` renamed to `crud-be:*`
- [ ] If bulk sweep missed any (check): `grep "demo-" package.json` → expect zero results

## Phase 8 — OpenAPI contract content

- [ ] Open `specs/apps/crud/contracts/openapi.yaml`
  - [ ] Update `info.title` if it reads "demo API" → "crud API"
  - [ ] Update `info.description` if it references "demo application" → "crud application"
- [ ] Open `specs/apps/crud/contracts/redocly.yaml` — audit for `demo` strings, update if present
- [ ] Open bundled outputs `specs/apps/crud/contracts/generated/openapi-bundled.json` and `openapi-bundled.yaml` — these are generated; delete stale bundles so next `lint` target regenerates them:
  - [ ] `rm -f specs/apps/crud/contracts/generated/openapi-bundled.json`
  - [ ] `rm -f specs/apps/crud/contracts/generated/openapi-bundled.yaml`

## Phase 9 — Gherkin feature files

- [ ] `grep -r "demo-" specs/apps/crud/be/gherkin/` — list any matches
- [ ] For each match: open file, replace `demo-` with `crud-` in scenario text or step definitions
- [ ] `grep -r "demo-" specs/apps/crud/fe/gherkin/` — list any matches
- [ ] For each match: open file, replace `demo-` with `crud-` in scenario text or step definitions
- [ ] Verify: `grep -r "demo" specs/apps/crud/` → expect zero results (or only legitimate English words like "demonstrate")

## Phase 10 — C4 diagram files

- [ ] Open `specs/apps/crud/c4/context.md` — replace any `demo-` app references
- [ ] Open `specs/apps/crud/c4/container.md` — replace any `demo-` app references
- [ ] Open `specs/apps/crud/c4/component-be.md` — replace any `demo-` app references
- [ ] Open `specs/apps/crud/c4/component-fe.md` — replace any `demo-` app references

## Phase 11 — Specs README files

- [ ] Open `specs/apps/crud/README.md` — replace `demo-` references
- [ ] Open `specs/apps/crud/be/README.md` — replace `demo-` references
- [ ] Open `specs/apps/crud/fe/README.md` — replace `demo-` references
- [ ] Open `specs/apps/crud/be/gherkin/README.md` — replace `demo-` references
- [ ] Open `specs/apps/crud/fe/gherkin/README.md` — replace `demo-` references
- [ ] Open `specs/apps/crud/c4/README.md` — replace `demo-` references
- [ ] Open `specs/apps/crud/contracts/README.md` — replace `demo-` references

## Phase 12 — Governance workflows

- [ ] `grep -r "demo-be-\|demo-fe-\|demo-fs-\|demo-contracts\|specs/apps/demo" governance/workflows/` — list all matches
- [ ] For each matched file: open, replace stale references with `crud-*` equivalents
- [ ] Verify: `grep -r "demo-be-\|demo-fe-\|demo-fs-\|demo-contracts" governance/workflows/` → expect zero results

## Phase 13 — Governance conventions

- [ ] `grep -r "demo-be-\|demo-fe-\|demo-fs-\|demo-contracts\|specs/apps/demo" governance/conventions/` — list all matches
- [ ] For each matched file: open, replace stale references
- [ ] Verify: `grep -r "demo-be-\|demo-fe-\|demo-fs-\|demo-contracts" governance/conventions/` → expect zero results

## Phase 14 — Governance development docs and principles

- [ ] `grep -rl "demo-be-\|demo-fe-\|demo-fs-\|demo-contracts" governance/development/ governance/principles/ governance/vision/` — list matches
- [ ] For each matched file: open, replace stale references

## Phase 15 — `docs/` tree

- [ ] `grep -rl "demo-be-\|demo-fe-\|demo-fs-\|demo-contracts\|specs/apps/demo" docs/` — list all matches
- [ ] For each matched file: open, replace stale `demo-*` app names with `crud-*`
- [ ] Verify: `grep -r "demo-be-\|demo-fe-\|demo-fs-\|demo-contracts" docs/` → expect zero results

## Phase 16 — Root workspace documentation files

- [ ] Open `CLAUDE.md`:
  - [ ] Replace all `demo-be-*` app names in the Tech Stack section
  - [ ] Replace all `demo-fe-*` and `demo-fs-*` app names
  - [ ] Replace `demo-contracts` references
  - [ ] Replace `specs/apps/demo/` path references
  - [ ] Replace npm script examples (`dev:demo-*` → `dev:crud-*`)
  - [ ] Replace coverage table project names
  - [ ] Replace `demo-be-e2e` and `demo-fe-e2e` references
  - [ ] Replace `test:integration` caching table app names
  - [ ] Verify: `grep "demo-be-\|demo-fe-\|demo-fs-\|demo-contracts" CLAUDE.md` → expect zero results
- [ ] Open root `README.md`:
  - [ ] Replace all `demo-*` app references
  - [ ] Verify: `grep "demo-be-\|demo-fe-\|demo-fs-" README.md` → expect zero results
- [ ] Open `AGENTS.md`:
  - [ ] `grep "demo-" AGENTS.md` — if any matches, update them
- [ ] Open `LICENSING-NOTICE.md`:
  - [ ] `grep "demo-" LICENSING-NOTICE.md` — update if present

## Phase 17 — Active plans (backlog, ideas)

- [ ] `grep -r "demo-be-\|demo-fe-\|demo-fs-\|demo-contracts" plans/ideas.md plans/backlog/` — list matches
- [ ] For each match in active plans: open, replace `demo-*` with `crud-*`
- [ ] Do **not** modify `plans/done/` or `generated-reports/` (historical records)

## Phase 18 — `.claude/` agent and skill files

- [ ] `grep -r "demo-be-\|demo-fe-\|demo-fs-\|demo-contracts" .claude/` — list matches
- [ ] For each match: open agent/skill file, replace stale references
- [ ] `.opencode/` mirrors will be regenerated by sync; skip manual edits there

## Phase 18b — GitHub Actions workflow file renames

Rename all 15 per-app test workflow files and update their internal references:

- [ ] `git mv .github/workflows/test-demo-be-clojure-pedestal.yml .github/workflows/test-crud-be-clojure-pedestal.yml`
- [ ] `git mv .github/workflows/test-demo-be-csharp-aspnetcore.yml .github/workflows/test-crud-be-csharp-aspnetcore.yml`
- [ ] `git mv .github/workflows/test-demo-be-elixir-phoenix.yml .github/workflows/test-crud-be-elixir-phoenix.yml`
- [ ] `git mv .github/workflows/test-demo-be-fsharp-giraffe.yml .github/workflows/test-crud-be-fsharp-giraffe.yml`
- [ ] `git mv .github/workflows/test-demo-be-golang-gin.yml .github/workflows/test-crud-be-golang-gin.yml`
- [ ] `git mv .github/workflows/test-demo-be-java-springboot.yml .github/workflows/test-crud-be-java-springboot.yml`
- [ ] `git mv .github/workflows/test-demo-be-java-vertx.yml .github/workflows/test-crud-be-java-vertx.yml`
- [ ] `git mv .github/workflows/test-demo-be-kotlin-ktor.yml .github/workflows/test-crud-be-kotlin-ktor.yml`
- [ ] `git mv .github/workflows/test-demo-be-python-fastapi.yml .github/workflows/test-crud-be-python-fastapi.yml`
- [ ] `git mv .github/workflows/test-demo-be-rust-axum.yml .github/workflows/test-crud-be-rust-axum.yml`
- [ ] `git mv .github/workflows/test-demo-be-ts-effect.yml .github/workflows/test-crud-be-ts-effect.yml`
- [ ] `git mv .github/workflows/test-demo-fe-ts-nextjs.yml .github/workflows/test-crud-fe-ts-nextjs.yml`
- [ ] `git mv .github/workflows/test-demo-fe-ts-tanstack-start.yml .github/workflows/test-crud-fe-ts-tanstack-start.yml`
- [ ] `git mv .github/workflows/test-demo-fe-dart-flutterweb.yml .github/workflows/test-crud-fe-dart-flutterweb.yml`
- [ ] `git mv .github/workflows/test-demo-fs-ts-nextjs.yml .github/workflows/test-crud-fs-ts-nextjs.yml`
- [ ] Update `name:` fields and `backend-name:` / `frontend-name:` with-block strings in each renamed workflow (Phase 4a bulk sweep already updated internal `demo-be-` → `crud-be-` strings in `.yml` files; verify with spot-check):
  - [ ] Open one renamed workflow (e.g., `test-crud-be-golang-gin.yml`) and confirm `backend-name: crud-be-golang-gin` and workflow `name:` reads `crud-`
  - [ ] `grep -r "demo-be-\|demo-fe-\|demo-fs-" .github/workflows/` → expect zero results
- [ ] Update reusable workflow files for any remaining hardcoded `demo-` references:
  - [ ] `grep "demo-" .github/workflows/_reusable-backend-lint.yml` — fix any matches
  - [ ] `grep "demo-" .github/workflows/_reusable-backend-typecheck.yml` — fix any matches
  - [ ] `grep "demo-" .github/workflows/_reusable-frontend-e2e.yml` — fix any matches (e.g., `infra/dev/demo-be-golang-gin`, `demo-be-golang-gin:codegen` default values)
- [ ] Verify: `ls .github/workflows/ | grep "test-demo-"` → expect zero results

## Phase 19 — Final stale-reference audit

Run all checks below; each must return zero results before proceeding to validation:

- [ ] `grep -r "demo-be-" . --include="*.json" --include="*.yaml" --include="*.yml" --include="*.md" | grep -v ".git/" | grep -v "node_modules/" | grep -v "plans/done/" | grep -v "generated-reports/" | grep -v ".claude/worktrees/" | grep -v "plans/in-progress/"`
- [ ] `grep -r "demo-fe-" . --include="*.json" --include="*.yaml" --include="*.yml" --include="*.md" | grep -v ".git/" | grep -v "node_modules/" | grep -v "plans/done/" | grep -v "generated-reports/" | grep -v ".claude/worktrees/" | grep -v "plans/in-progress/"`
- [ ] `grep -r "demo-fs-" . --include="*.json" --include="*.yaml" --include="*.yml" --include="*.md" | grep -v ".git/" | grep -v "node_modules/" | grep -v "plans/done/" | grep -v "generated-reports/" | grep -v ".claude/worktrees/" | grep -v "plans/in-progress/"`
- [ ] `grep -r '"demo-contracts"' . --include="*.json" | grep -v ".git/" | grep -v "node_modules/" | grep -v "plans/done/" | grep -v "plans/in-progress/"`
- [ ] `grep -r 'specs/apps/demo/' . --include="*.json" --include="*.yaml" --include="*.md" | grep -v ".git/" | grep -v "node_modules/" | grep -v "plans/done/" | grep -v "plans/in-progress/"`
- [ ] `grep -r 'apps/demo-' . --include="*.json" --include="*.yaml" --include="*.md" | grep -v ".git/" | grep -v "node_modules/" | grep -v "plans/done/" | grep -v "plans/in-progress/"`
- [ ] `grep -r 'demo_be_' . --include="*.json" --include="*.yaml" --include="*.yml" | grep -v ".git/" | grep -v "node_modules/" | grep -v "plans/done/" | grep -v "plans/in-progress/"`

## Phase 20 — Sync `.opencode/`

- [ ] `npm run sync:claude-to-opencode`
- [ ] Verify `npm run sync:claude-to-opencode` exits 0

## Phase 21 — Nx workspace validation

- [ ] `npm install` — ensure workspace loads with updated project names
- [ ] `npx nx graph --file=/tmp/nx-graph-output.json` — verify no broken references; confirm 18 `crud-*` projects appear, zero `demo-*`
- [ ] `npx nx run-many -t codegen --projects=crud-be-clojure-pedestal,crud-be-csharp-aspnetcore,crud-be-elixir-phoenix,crud-be-fsharp-giraffe,crud-be-golang-gin,crud-be-java-springboot,crud-be-java-vertx,crud-be-kotlin-ktor,crud-be-python-fastapi,crud-be-rust-axum,crud-be-ts-effect,crud-fe-ts-nextjs,crud-fe-ts-tanstack-start,crud-fe-dart-flutterweb,crud-fs-ts-nextjs` — regenerate all contracts
- [ ] Verify `generated-contracts/` directories exist under each `apps/crud-*/`

## Phase 22 — Quality gates

> **Important**: Fix ALL failures found during quality gates — including pre-existing
> issues not caused by this rename. This follows the root cause orientation principle.
> Do NOT skip or suppress any failure. Do NOT proceed to Phase 23 until all gates pass.

- [ ] `npx nx affected -t typecheck` — must pass (exit 0)
- [ ] `npx nx affected -t lint` — must pass (exit 0)
- [ ] `npx nx affected -t test:quick` — must pass with all coverage thresholds met
- [ ] `npx nx affected -t spec-coverage` — must pass for all affected projects (exit 0)
- [ ] `npx nx run rhino-cli:test:quick` — must pass (exit 0)

## Phase 23 — Markdown quality gate

- [ ] `npm run lint:md` — must pass (exit 0)
- [ ] `npm run lint:md:fix` — if violations found, auto-fix and re-run until clean

## Phase 24 — Commit and draft PR

### Commit strategy

This rename is a single logical refactor that may be committed as one thematic commit
after all phases pass quality gates. Intermediate commits per phase are acceptable
during execution to preserve progress and must also follow Conventional Commits format.
A final squash or single commit is preferred before opening the draft PR.

- Type: `refactor`
- Scope: `apps` (or omit scope given repo-wide change)
- Example message: `refactor(apps): rename demo-* to crud-* to clarify CRUD family scope`

Examples of acceptable intermediate commits (Conventional Commits format):

- `refactor(apps): git mv demo-* directories to crud-*`
- `refactor(ci): rename test-demo-*.yml workflow files to test-crud-*`
- `refactor(docs): update demo- references in governance and documentation`

- [ ] Review `git status` — confirm no unintended files staged
- [ ] Stage all changes: `git add -A`
- [ ] Commit with Conventional Commits message:
      `refactor(apps): rename demo-* to crud-* to clarify CRUD family scope`
- [ ] Push branch: `git push -u origin worktree-rename-demo-to-crud`
- [ ] Open draft PR against `main` via `gh pr create --draft --title "refactor(apps): rename demo-* to crud-*" --body "..."`
- [ ] Link PR URL in this plan's README.md under a "PR" section

### Post-push CI monitoring

- [ ] Monitor GitHub Actions workflows on the draft PR (visible at the PR URL)
- [ ] Verify `pr-quality-gate.yml` passes
- [ ] Verify each renamed per-app workflow (`test-crud-be-*.yml`, `test-crud-fe-*.yml`,
      `test-crud-fs-ts-nextjs.yml`) passes
- [ ] If any workflow fails, fix immediately and push a follow-up commit before
      moving to Phase 25

## Phase 25 — Post-merge cleanup and plan archival

- [ ] After PR is merged to `main`: `git worktree remove .claude/worktrees/rename-demo-to-crud`
- [ ] `git branch -d worktree-rename-demo-to-crud`
- [ ] `git mv plans/in-progress/2026-04-26__rename-demo-to-crud plans/done/2026-04-26__rename-demo-to-crud`
- [ ] Update `plans/in-progress/README.md` — remove this plan's entry
- [ ] Update `plans/done/README.md` — add this plan's entry with completion date
- [ ] Commit: `chore(plans): move rename-demo-to-crud to done`
