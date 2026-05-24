# Delivery Checklist — Two Rhino Versions

> One checkbox = one concrete action. See [tech-docs.md](./tech-docs.md) for the
> architecture and [prd.md](./prd.md) for acceptance criteria.

## Worktree

Worktree path: `worktrees/have-two-rhino-versions/`

Provision before execution (run from repo root):

```bash
claude --worktree have-two-rhino-versions
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0 — Environment Setup

- [x] Provision worktree: `claude --worktree have-two-rhino-versions` (creates `worktrees/have-two-rhino-versions/`).
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: none (git worktree)
  - **Notes**: Provisioned via `git worktree add worktrees/have-two-rhino-versions -b have-two-rhino-versions HEAD` (git-level equivalent of `claude --worktree`, which the assistant cannot launch as it spawns a new interactive session). Worktree confirmed at `/Users/wkf/ose-projects/ose-primer/worktrees/have-two-rhino-versions` on branch `have-two-rhino-versions`.
- [x] Initialize toolchain in the **root** worktree (not the new one): `npm install && npm run doctor -- --fix`. Verify it exits 0 (see [Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md)).
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: none
  - **Notes**: `npm install` clean; `npm run doctor -- --fix` → **19/19 tools OK, 0 warning, 0 missing, nothing to fix**. Relevant toolchains: golang v1.26.1, rust v1.94.0, cargo-llvm-cov v0.8.5, node v24.13.1.
- [x] Confirm Rust toolchain present: `rustc --version && cargo --version && cargo llvm-cov --version` all succeed.
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: none
  - **Notes**: `rustc 1.94.0`, `cargo 1.94.0`, `cargo-llvm-cov 0.8.5` — all succeed.
- [x] Capture baseline: `npx nx run rhino-cli:build` _(pre-rename name — correct at this phase; rename happens in Phase 1)_ exits 0 and `./apps/rhino-cli/dist/rhino-cli --help` prints the command tree. Record the help output to `worktrees/have-two-rhino-versions/baseline-help.txt` for later parity reference.
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `baseline-help.txt` (worktree-local, gitignored)
  - **Notes**: Build succeeded. Help recorded (30 lines). Surface = 11 namespaces: agents, contracts, docs, doctor, env, git, java, repo-governance, spec-coverage, test-coverage, workflows. Global flags: `--no-color`, `-o/--output {text,json,markdown}`, `-q/--quiet`, `--say`, `-v/--verbose`, `--version`. Recorded at worktree-root `baseline-help.txt` (the `worktrees/...` prefix in the plan resolves to this path from repo root).
- [x] Confirm clean baseline gates: `npx nx affected -t typecheck lint test:quick spec-coverage --base=origin/main` exits 0.
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: none
  - **Notes**: Worktree HEAD == `origin/main` (683888ef3), so affected graph is empty → `NX No tasks were run` (exit 0). Clean baseline confirmed.

---

## Phase 1 — Rename Go `rhino-cli` → `rhino-cli-go` (CI stays green on Go)

> Goal: pure mechanical rename + repoint. No behavior change. End state: all
> gates green, CI still runs Go (just renamed). Rust does not exist yet.

- [x] `git mv apps/rhino-cli apps/rhino-cli-go`.
  - _Suggested executor: `swe-golang-dev`_ (executed directly — mechanical `git mv`, per workflow Agent-Selection rule 5)
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `apps/rhino-cli/` → `apps/rhino-cli-go/` (history preserved)
  - **Notes**: `git mv` exit 0; `apps/rhino-cli-go/` present (cmd/ internal/ main.go go.mod project.json scripts/), old path gone.
- [x] Edit `apps/rhino-cli-go/project.json`: set `"name": "rhino-cli-go"`, `"sourceRoot": "apps/rhino-cli-go"`, and rewrite every self-referencing path (`apps/rhino-cli` → `apps/rhino-cli-go`, `cwd`, `dist/rhino-cli` stays as basename, `go run -C apps/rhino-cli-go`, `spec-coverage validate … apps/rhino-cli-go`, `validate-cross-vendor-parity.sh` path). Verify: `npx nx show project rhino-cli-go --json` lists all targets and no path contains the old `apps/rhino-cli/`.
  - _Suggested executor: `swe-golang-dev`_ (executed directly — mechanical sed substitution)
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `apps/rhino-cli-go/project.json`
  - **Notes**: `name`→`rhino-cli-go`, `sourceRoot`→`apps/rhino-cli-go`; all `cwd`/`go run -C`/`spec-coverage` arg/`cover.out`/script paths → `apps/rhino-cli-go`; binary basename `dist/rhino-cli` preserved. `grep -c 'apps/rhino-cli[^-]'` = 0. The `nx show project` graph check is transiently blocked by stale `implicitDependencies: ["rhino-cli"]` in ~23 callers — resolved and re-verified in P1.5 (graph-wide dependency, expected during rename).
- [x] Check `apps/rhino-cli-go/go.mod` module path; if it encodes `rhino-cli`, decide whether to rename the module (Go imports are module-path based, not dir based — only rename if internal imports reference it). Verify: `cd apps/rhino-cli-go && CGO_ENABLED=0 go build ./... && CGO_ENABLED=0 go vet ./...` both exit 0.
  - _Suggested executor: `swe-golang-dev`_ (executed directly — mechanical)
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `go.work`
  - **Notes**: Module path is `github.com/wahidyankf/ose-public/apps/rhino-cli` — a **logical** path, unaffected by the dir rename; internal imports reference the module path (unchanged), not the dir, so **no module rename needed** (left as-is per the plan's guidance; the stale `ose-public` segment is preexisting and out of scope). Real blocker found + fixed: root **`go.work`** `use` directive listed `./apps/rhino-cli` → updated to `./apps/rhino-cli-go`. After fix, `go build ./...` and `go vet ./...` both exit 0. `go.work.sum` has no stale path refs.
- [x] `git mv infra/dev/rhino-cli infra/dev/rhino-cli-go` and update any path inside its `docker-compose.yml`. Verify: `test -f infra/dev/rhino-cli-go/docker-compose.yml`.
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `infra/dev/rhino-cli/` → `infra/dev/rhino-cli-go/`
  - **Notes**: `git mv` exit 0; `infra/dev/rhino-cli-go/docker-compose.yml` present; no stale bare `rhino-cli` path references inside the dir.
- [x] Enumerate all `project.json` callers: `grep -rln 'rhino-cli' apps libs --include=project.json`. For EACH hit, replace `implicitDependencies: ["rhino-cli"]` → `["rhino-cli-go"]` and command strings `go run -C apps/rhino-cli` → `go run -C apps/rhino-cli-go` (and any `apps/rhino-cli/` substring). Verify: `grep -rn 'rhino-cli\b' apps libs --include=project.json | grep -v 'rhino-cli-go'` returns nothing.
  - _Suggested executor: `swe-golang-dev`_ (executed directly — mechanical perl substitution)
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: 23 `project.json` files (11 crud-be, 5 crud-fe/fs, crud-be-e2e, crud-fe-e2e, + 6 libs: golang-commons, ts-ui, elixir-{gherkin,cabbage,openapi-codegen}, clojure-openapi-codegen)
  - **Notes**: Lookahead-safe perl across all callers (excluding the already-done `rhino-cli-go/project.json`): `apps/rhino-cli(?!-go)`→`apps/rhino-cli-go` (paths), `"rhino-cli"`→`"rhino-cli-go"` (implicitDependencies), `rhino-cli:`→`rhino-cli-go:` (nx target refs in java-springboot/vertx). Binary basename `dist/rhino-cli` preserved. Verify: `grep -rn 'rhino-cli' apps libs --include=project.json | grep -v 'rhino-cli-go'` → **CLEAN**. nx graph now resolves: `nx show project rhino-cli-go` OK; `crud-be-golang-gin` depends on `rhino-cli-go`. (This also satisfies P1.2's deferred `nx show project` graph check.)
- [x] Edit root `package.json` scripts (`dev:rhino-cli`, `sync:claude-to-opencode`, `sync:agents`, `sync:skills`, `sync:dry-run`, `validate:sync`, `validate:claude`, `doctor`): `nx run rhino-cli:build` → `nx run rhino-cli-go:build`; `./apps/rhino-cli/dist/rhino-cli` → `./apps/rhino-cli-go/dist/rhino-cli`; `infra/dev/rhino-cli/` → `infra/dev/rhino-cli-go/`. Verify: `npm run doctor` builds and runs; `npm run sync:claude-to-opencode` succeeds.
  - _Suggested executor: `swe-typescript-dev`_ (executed directly — mechanical)
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `package.json`
  - **Notes**: All 8 script commands repointed (`nx run rhino-cli-go:build`, `./apps/rhino-cli-go/dist/rhino-cli`, `infra/dev/rhino-cli-go/`). npm script _keys_ left unchanged (aliases, not project refs). `npm run doctor` → 19/19 OK; `npm run sync:claude-to-opencode` → SUCCESS (49 agents) with **0** `.opencode/` changes (no-op diff confirmed).
- [x] Edit `.husky/pre-commit`: `go run -C apps/rhino-cli` → `go run -C apps/rhino-cli-go`. Verify: `sh .husky/pre-commit` runs the git pre-commit gate without a path error.
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `.husky/pre-commit`
  - **Notes**: Now `CGO_ENABLED=0 go run -C apps/rhino-cli-go main.go git pre-commit`. Path resolves (dir exists, Go builds). Full hook fires at the actual commit in P1.14.
- [x] Edit `.husky/pre-push`: `rhino-cli:validate:naming-agents` → `rhino-cli-go:validate:naming-agents` (and `:naming-workflows`, `:mermaid`, `:cross-vendor-parity`). Verify: `npx nx run rhino-cli-go:validate:naming-agents` exits 0.
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `.husky/pre-push`
  - **Notes**: All 4 validate targets repointed to `rhino-cli-go:` (lines 17/20/23/28). `npx nx run rhino-cli-go:validate:naming-agents` → VALIDATION PASSED (0 violations), exit 0. One descriptive comment ("rhino-cli checks") left as generic prose.
- [x] Edit `.github/workflows/pr-quality-gate.yml` naming job (≈ lines 232–240): `rhino-cli:validate:naming-agents` → `rhino-cli-go:validate:naming-agents` and `:validate:naming-workflows`. Verify: `grep -n 'rhino-cli-go:validate' .github/workflows/pr-quality-gate.yml` shows both.
  - _Suggested executor: `ci-fixer`_ (executed directly — mechanical)
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `.github/workflows/pr-quality-gate.yml`
  - **Notes**: Lines 239–240 now `rhino-cli-go:validate:naming-agents` / `:validate:naming-workflows`. (setup-golang still correct — Go is the active impl until Phase 10 cutover.)
- [x] Edit `.github/workflows/pr-validate-links.yml` (≈ line 26): `go run -C apps/rhino-cli` → `go run -C apps/rhino-cli-go`. Verify: `grep -n 'apps/rhino-cli-go' .github/workflows/pr-validate-links.yml`.
  - _Suggested executor: `ci-fixer`_ (executed directly — mechanical)
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `.github/workflows/pr-validate-links.yml`
  - **Notes**: Line 26 now `CGO_ENABLED=0 go run -C apps/rhino-cli-go main.go docs validate-links`. No bare `rhino-cli` left in `.github/workflows/`.
- [x] Update `apps/rhino-cli-go/scripts/validate-cross-vendor-parity.sh`: any `apps/rhino-cli` path → `apps/rhino-cli-go`. Verify: `bash apps/rhino-cli-go/scripts/validate-cross-vendor-parity.sh` exits 0.
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `apps/rhino-cli-go/scripts/validate-cross-vendor-parity.sh`
  - **Notes**: Invocation paths (`cd apps/rhino-cli-go`, comment header path) repointed. Script run → "CROSS-VENDOR PARITY VALIDATION PASSED: all invariants hold." (exit 0). Remaining `rhino-cli` tokens are human-readable pass/fail log labels (prose, conceptual CLI) — left as-is.
- [x] Update textual references in docs: `grep -rln 'apps/rhino-cli\b\|rhino-cli:' repo-governance docs README.md AGENTS.md specs/apps/rhino` and repoint to `rhino-cli-go` where they name the project/target/path (leave generic prose "rhino-cli" only where it means the conceptual CLI — but for now Go is the only impl, so repoint paths/targets). Verify: no broken relative links — `npx nx run rhino-cli-go:validate:mermaid` passes and `go run -C apps/rhino-cli-go main.go docs validate-links` exits 0.
  - _Suggested executor: `docs-fixer`_ (executed directly — surgical path/target substitution, prose preserved)
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: 14 docs (repo-governance: ci-conventions, bdd-spec-test-mapping, post-push-ci-verification, code, ai-agents, reproducible-environments, infra-development-environment-setup, repo-cross-vendor-parity-quality-gate, diagrams, governance-vendor-independence; docs/: setup-development-environment, platform-bindings, project-dependency-graph, system-architecture/applications) + `.claude/agents/README.md` + `.claude/skills/README.md`
  - **Notes**: Repointed only concrete `apps/rhino-cli/` paths + `rhino-cli:`/`nx run rhino-cli` target refs; conceptual prose ("the rhino-cli tool", code spans) left intact. `specs/apps/rhino/` deferred to P1.13. Initial run flagged 1 broken link → traced to `.claude/agents/README.md:159` + `.claude/skills/README.md:104` (`../../apps/rhino-cli/README.md`); fixed both. Re-run: **✓ All links valid**; `validate:mermaid` **Successfully ran**. Historical `plans/done/**` archives left untouched (not in validator scan scope; records of past state).
- [x] Update `specs/apps/rhino/README.md` backlinks `../../../apps/rhino-cli/README.md` → `../../../apps/rhino-cli-go/README.md`. Verify: `go run -C apps/rhino-cli-go main.go docs validate-links` exits 0.
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `specs/apps/rhino/README.md`, `specs/apps/rhino/behavior/README.md`, `specs/apps/rhino/behavior/cli/gherkin/README.md`
  - **Notes**: All backlinks + `nx run rhino-cli-go:` targets + `cd apps/rhino-cli-go` + code-block paths repointed across the three specs READMEs; link label text `[rhino-cli]` kept (conceptual). `docs validate-links` → **✓ All links valid** (exit 0).
- [x] **Phase 1 gate**: `npx nx affected -t typecheck lint test:quick spec-coverage --base=origin/main` exits 0; `npm run lint:md` exits 0; `npm run sync:claude-to-opencode` is a no-op diff. Commit: `refactor(rhino-cli): rename rhino-cli to rhino-cli-go and repoint all callers`.
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: commit `87198b7e4` (258 files)
  - **Notes**: `rhino-cli-go` gates green (test:quick 90.00%); caller `crud-be-golang-gin:spec-coverage` resolves via repoint; `lint:md` 0 errors (fixed one MD049 in this delivery.md); sync no-op. Committed `87198b7e4` — **pre-commit hook passed** (git pre-commit gate, tree clean). Pushed to `main` (`683888ef3..87198b7e4`); the **pre-push hook ran `nx affected -t typecheck lint test:quick spec-coverage` across all affected projects + naming validators and passed** (push succeeded), satisfying the full affected-gate requirement. Post-push CI verification tracked in the Post-Push section.

---

## Phase 2 — Scaffold `rhino-cli-rust` (unwired)

> Goal: empty Rust crate with the full target set that builds, lints, type-checks,
> and runs unit tests — but no caller depends on it yet.

- [ ] Create `apps/rhino-cli-rust/Cargo.toml` modeled on ose-public `/Users/wkf/ose-projects/ose-public/apps/rhino-cli/Cargo.toml` _[Web-cited: ose-public `apps/rhino-cli/Cargo.toml` — sibling repo, verify structure at execution time]_ (`edition = 2024`, `[[bin]] name = "rhino-cli"`, `[lib] name = "rhino_cli"`, deps clap/serde/serde_json/walkdir/ignore/regex/pulldown-cmark/anyhow/thiserror/quick-xml/chrono/glob/sha2, dev-deps cucumber/assert_cmd/predicates/tempfile, lints `unsafe_code = "deny"` + clippy pedantic). Pin versions by running `cargo add` then `cargo update`; do NOT copy versions blind. Verify: `cargo metadata --manifest-path apps/rhino-cli-rust/Cargo.toml` succeeds.
  - _Suggested executor: `swe-rust-dev`_ ✅ delegated
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `apps/rhino-cli-rust/Cargo.toml`, `Cargo.lock`
  - **Notes**: `edition 2024`, `[[bin]] name="rhino-cli"`, `[lib] name="rhino_cli"`, MSRV 1.88. Deps pinned from registry (match ose-public exactly): clap 4.6.1, serde 1.0.228, serde_json 1.0.150, serde_norway 0.9.42, walkdir 2.5.0, ignore 0.4.25, regex 1.12.3, pulldown-cmark 0.13.4, anyhow 1.0.102, thiserror 2.0.18, quick-xml 0.40.1, chrono 0.4.44, glob 0.3.3, sha2 0.11.0; dev: cucumber 0.23.0, assert_cmd 2.2.2, predicates 3.1.4, tempfile 3.27.0. `unsafe_code = "forbid"` + clippy pedantic. `cargo metadata` OK. (tree-sitter deferred to the mermaid-port phase.)
- [x] Create `apps/rhino-cli-rust/rust-toolchain.toml` and `apps/rhino-cli-rust/deny.toml`. Verify: `cargo deny --manifest-path apps/rhino-cli-rust/Cargo.toml check` exits 0.
  - _Suggested executor: `swe-rust-dev`_ ✅ delegated
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `apps/rhino-cli-rust/rust-toolchain.toml`, `deny.toml`
  - **Notes**: toolchain pinned `1.95.0` (clippy/rustfmt/llvm-tools, profile minimal). `cargo deny check` → advisories/bans/licenses/sources **ok** (exit 0); 3 harmless `license-not-encountered` warnings (allow-listed for parity, unused).
- [x] Create `apps/rhino-cli-rust/src/{main.rs, lib.rs, cli.rs}`: clap derive root with global flags `--verbose --quiet --output --no-color` and an output-format validation hook, mirroring ose-public `src/cli.rs`. Create `src/internal/cliout/mod.rs` with the sealed `OutputFormat` enum (`Text|Json|Markdown`) + `parse()` + unit tests. Create `src/commands/mod.rs` (empty registry). Verify: `cargo run --manifest-path apps/rhino-cli-rust/Cargo.toml -- --help` prints a root help; `cargo test --manifest-path apps/rhino-cli-rust/Cargo.toml --lib` passes the cliout tests.
  - _Suggested executor: `swe-rust-dev`_ ✅ delegated
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `apps/rhino-cli-rust/src/{main.rs,lib.rs,cli.rs}`, `src/internal/mod.rs`, `src/internal/cliout/mod.rs`, `src/commands/mod.rs`
  - **Notes**: clap derive root with global flags `-v/--verbose`, `-q/--quiet`, `-o/--output` (default text), `--no-color`, `--say`; output-format validated against sealed `OutputFormat {Text,Json,Markdown}` enum (`parse()` + Display + round-trip). Empty command registry. `--help` exit 0; `cargo test --lib` → **5/5 pass**; clippy `-D warnings` clean; `fmt --check` clean. **Known parity item for later phases**: `--version` currently exits 2 (clap treats DisplayVersion as parse result via `run()`); the Go CLI exits 0 — to be reconciled when shadow-diff runs (flagged, not blocking the scaffold).
- [x] Create `apps/rhino-cli-rust/project.json` with name `rhino-cli-rust`, tags `["type:app","platform:cli","lang:rust","domain:tooling"]`, and the target set from [tech-docs §Nx target mapping](./tech-docs.md#nx-target-mapping-go-idiom--rust-idiom) (`build`, `install`, `run`, `typecheck`, `lint`, `test:unit`, `test:quick`, `test:integration`, plus stub `spec-coverage` + the `validate:*` targets the Go project exposes). Stub not-yet-ported `validate:*`/`spec-coverage` commands with `echo` placeholders. Verify: `npx nx show project rhino-cli-rust --json` lists every target.
  - _Suggested executor: `swe-rust-dev`_ (executed directly — Nx config)
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `apps/rhino-cli-rust/project.json`
  - **Notes**: name `rhino-cli-rust`, tags `[type:app, platform:cli, lang:rust, domain:tooling]`. Targets: build (`cargo build --release` + cp to `dist/rhino-cli`), install (`cargo fetch`), run, typecheck (`cargo check --all-targets`), lint (`cargo fmt --check && cargo clippy -- -D warnings`), test:unit (`cargo test --lib`), test:integration (`cargo test --tests`). **test:quick is a Phase-2 placeholder** (`cargo test --lib`) — Phase 3 swaps it to `cargo llvm-cov … --fail-under-lines 90`. spec-coverage + 5 validate:* targets are `echo` stubs (ported in Phases 3–6). `nx show project rhino-cli-rust` lists every target.
- [x] Add `apps/rhino-cli-rust/target` to the cache path list in `.github/actions/setup-rust/action.yml` (currently only `apps/crud-be-rust-axum/target`). Verify: `grep -n 'rhino-cli-rust/target' .github/actions/setup-rust/action.yml`.
  - _Suggested executor: `ci-fixer`_ (executed directly — one-line CI edit)
  - **Date**: 2026-05-24 · **Status**: Completed · **Files Changed**: `.github/actions/setup-rust/action.yml`
  - **Notes**: Added `apps/rhino-cli-rust/target` to the `actions/cache` path list (line 21, after `crud-be-rust-axum/target`). grep confirms presence.
- [ ] **Phase 2 gate**: `npx nx run rhino-cli-rust:build`, `:typecheck`, `:lint`, `:test:unit` each exit 0. Confirm no caller depends on it: `grep -rn 'rhino-cli-rust' apps libs package.json .husky .github --include='*' | grep -v 'apps/rhino-cli-rust/'` returns only the cache line. Commit: `feat(rhino-cli-rust): scaffold rust CLI crate with full target set`.

---

## Phase 3 — Port critical coverage path + introduce shadow-diff

> Commands first because every dependent project's `test:quick`/`spec-coverage`
> uses them. Introduces the shadow-diff harness used by all later phases.

- [ ] Create `apps/rhino-cli-rust/scripts/shadow-diff.sh` modeled on ose-public `apps/rhino-cli/scripts/shadow-diff.sh` _[Web-cited: ose-public `apps/rhino-cli/scripts/shadow-diff.sh` — sibling repo, verify structure at execution time]_: builds both binaries (`rhino-cli-go`, `rhino-cli-rust`), runs each on a per-command corpus (with `--no-color`, each `--output` format), diffs stdout/stderr/exit code, exits non-zero on any difference. Verify: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh --help` runs.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Write failing cucumber-rs scenarios for `test-coverage validate|merge|diff`: wire `specs/apps/rhino/behavior/cli/gherkin/test-coverage/` feature files into the integration test world in `apps/rhino-cli-rust/tests/`. Verify: `npx nx run rhino-cli-rust:test:integration` reports the test-coverage scenarios as failing (no implementation yet). _New test_
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port `apps/rhino-cli/internal/testcoverage/` (Go cover.out + LCOV + JaCoCo + Cobertura parse, classify covered/partial/missed, `pct = covered/(covered+partial+missed)`) into `apps/rhino-cli-rust/src/internal/testcoverage/`. Implement `test-coverage validate|merge|diff` commands in `apps/rhino-cli-rust/src/commands/testcoverage.rs`. Verify: `cargo test --manifest-path apps/rhino-cli-rust/Cargo.toml --lib` passes new unit tests; `npx nx run rhino-cli-rust:test:integration` passes the test-coverage scenarios.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Write failing cucumber-rs scenarios for `spec-coverage validate`: wire `specs/apps/rhino/behavior/cli/gherkin/spec-coverage/` feature files into the integration test world. Verify: `npx nx run rhino-cli-rust:test:integration` reports the spec-coverage scenarios as failing. _New test_
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port `apps/rhino-cli/internal/speccoverage/` + `spec-coverage validate` (with `--shared-steps`, `--exclude-dir`) into `apps/rhino-cli-rust/src/internal/speccoverage/`. Wire `cucumber-rs` integration tests reading `specs/apps/rhino/behavior/cli/gherkin/**/*.feature`. Verify: `npx nx run rhino-cli-rust:test:integration` passes; `npx nx run rhino-cli-rust:spec-coverage` reports full coverage.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Swap the `rhino-cli-rust:test:quick` target from the `--fail-under-lines` stub to real `cargo llvm-cov` with the 90% floor. Verify: `npx nx run rhino-cli-rust:test:quick` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] **Parity check**: run shadow-diff for `test-coverage validate|merge|diff` and `spec-coverage validate` against a corpus of real coverage files (use the repo's own `cover.out` fixtures + a crud app's `lcov.info`/`jacoco.xml`). Verify: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh test-coverage spec-coverage` exits 0 (byte-identical).
  - _Suggested executor: `swe-rust-dev`_
- [ ] Commit: `feat(rhino-cli-rust): port test-coverage + spec-coverage with shadow-diff parity`.

---

## Phase 4 — Port `docs` (validate-links, validate-mermaid)

- [ ] Write failing cucumber-rs scenario for `docs validate-links`: wire `specs/apps/rhino/behavior/cli/gherkin/docs/` scenarios into the integration test world in `apps/rhino-cli-rust/tests/`. Verify: `npx nx run rhino-cli-rust:test:integration` reports the docs scenarios as failing (no implementation yet). _New test_
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port `apps/rhino-cli/internal/docs/` link validator into `apps/rhino-cli-rust/src/internal/docs/` using the same structural parsing approach as the Go implementation (custom line-by-line extractor and validator). Implement `docs validate-links` command in `apps/rhino-cli-rust/src/commands/docs.rs`. Verify: `cargo test --manifest-path apps/rhino-cli-rust/Cargo.toml --lib` passes new unit tests; `npx nx run rhino-cli-rust:test:integration` passes the docs validate-links scenarios.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Write failing cucumber-rs scenario for `docs validate-mermaid`: add remaining docs scenarios from `specs/apps/rhino/behavior/cli/gherkin/docs/` not yet covered. Verify: `npx nx run rhino-cli-rust:test:integration` reports the mermaid scenarios as failing. _New test_
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port `apps/rhino-cli/internal/mermaid/` mermaid validator into `apps/rhino-cli-rust/src/internal/mermaid/` using the same structural parsing approach as the Go implementation (custom line-by-line extractor and validator — no additional parsing crate required). Implement `docs validate-mermaid` command. Verify: `cargo test --manifest-path apps/rhino-cli-rust/Cargo.toml --lib` passes; `npx nx run rhino-cli-rust:test:integration` passes all docs scenarios.
  - _Suggested executor: `swe-rust-dev`_
- [ ] **Parity check**: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh docs` exits 0 across the repo's markdown corpus.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Commit: `feat(rhino-cli-rust): port docs validate-links + validate-mermaid`.

---

## Phase 5 — Port `agents` (sync, validate-naming, validate-claude, validate-sync)

- [ ] Write failing cucumber-rs scenarios for `agents` subcommands: wire `specs/apps/rhino/behavior/cli/gherkin/agents/` feature files into the integration test world. Verify: `npx nx run rhino-cli-rust:test:integration` reports all agents scenarios as failing. _New test_
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port `apps/rhino-cli/internal/agents/` internal library modules (converter, frontmatter, yaml_formatting) into `apps/rhino-cli-rust/src/internal/agents/`. Verify: `cargo test --manifest-path apps/rhino-cli-rust/Cargo.toml --lib` passes new unit tests for the agents internal library.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Implement `agents sync` command in `apps/rhino-cli-rust/src/commands/agents.rs`, wiring the sync, sync_validator, and skill_validator modules from `src/internal/agents/`. Verify: `npx nx run rhino-cli-rust:test:integration` passes the `agents sync` scenarios; `bash apps/rhino-cli-rust/scripts/shadow-diff.sh agents sync` exits 0 (byte-identical `.opencode/` tree verified with `git diff --exit-code` on a scratch checkout).
  - _Suggested executor: `swe-rust-dev`_
- [ ] Implement `agents validate-naming`, `agents validate-claude`, `agents validate-sync` commands. Verify: `npx nx run rhino-cli-rust:test:integration` passes all agents scenarios; `bash apps/rhino-cli-rust/scripts/shadow-diff.sh agents` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] **Parity check**: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh agents` exits 0 across all four subcommands. Critically verify `agents sync` produces a byte-identical `.opencode/` tree.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Commit: `feat(rhino-cli-rust): port agents sync + validators`.

---

## Phase 6 — Port `repo-governance vendor-audit`, `workflows validate-naming`, cross-vendor-parity

- [ ] Write failing cucumber-rs scenarios for `repo-governance vendor-audit` and `workflows validate-naming`: wire `specs/apps/rhino/behavior/cli/gherkin/repo-governance/` and `specs/apps/rhino/behavior/cli/gherkin/workflows/` feature files into the integration test world. Verify: `npx nx run rhino-cli-rust:test:integration` reports those scenarios as failing. _New test_
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port `apps/rhino-cli/internal/repo_governance/` vendor-audit logic into `apps/rhino-cli-rust/src/internal/repo_governance/` (mirroring Go's `internal/` layout) and implement `repo-governance vendor-audit` command in `apps/rhino-cli-rust/src/commands/repo_governance.rs`. Port `apps/rhino-cli/internal/naming/` workflow validator into `apps/rhino-cli-rust/src/internal/naming/` and implement `workflows validate-naming` command in `apps/rhino-cli-rust/src/commands/workflows.rs`. Verify: `cargo test --manifest-path apps/rhino-cli-rust/Cargo.toml --lib` passes; `npx nx run rhino-cli-rust:test:integration` passes the repo-governance and workflows scenarios.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port `apps/rhino-cli-go/scripts/validate-cross-vendor-parity.sh` semantics: create `apps/rhino-cli-rust/scripts/validate-cross-vendor-parity.sh` calling the Rust binary. Verify: `bash apps/rhino-cli-rust/scripts/validate-cross-vendor-parity.sh` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] **Parity check**: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh repo-governance workflows` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Commit: `feat(rhino-cli-rust): port repo-governance + workflows validators`.

---

## Phase 7 — Port `git pre-commit`, `contracts` (java-clean-imports, dart-scaffold), `java validate-annotations`

- [ ] Write failing cucumber-rs scenarios for `git pre-commit`: wire `specs/apps/rhino/behavior/cli/gherkin/git/` feature files into the integration test world. Verify: `npx nx run rhino-cli-rust:test:integration` reports the git scenarios as failing. _New test_
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port `apps/rhino-cli/internal/git/` + `git pre-commit` orchestrator into `apps/rhino-cli-rust/src/internal/git/` (mirroring Go's `internal/` layout). Implement `git pre-commit` command in `apps/rhino-cli-rust/src/commands/git.rs`. Verify: `npx nx run rhino-cli-rust:test:integration` passes the git scenarios; `bash apps/rhino-cli-rust/scripts/shadow-diff.sh git` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Write failing cucumber-rs scenarios for `contracts` and `java` subcommands: wire `specs/apps/rhino/behavior/cli/gherkin/contracts/` and `specs/apps/rhino/behavior/cli/gherkin/java/` feature files into the integration test world. Verify: `npx nx run rhino-cli-rust:test:integration` reports those scenarios as failing. _New test_
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port `contracts java-clean-imports` and `contracts dart-scaffold` into `apps/rhino-cli-rust/src/internal/contracts/` and implement in `apps/rhino-cli-rust/src/commands/contracts.rs`. Port `java validate-annotations` into `apps/rhino-cli-rust/src/internal/java/` and implement in `apps/rhino-cli-rust/src/commands/java.rs`. Verify: `cargo test --manifest-path apps/rhino-cli-rust/Cargo.toml --lib` passes; `npx nx run rhino-cli-rust:test:integration` passes contracts and java scenarios; `bash apps/rhino-cli-rust/scripts/shadow-diff.sh contracts java` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Commit: `feat(rhino-cli-rust): port git pre-commit + contracts + java validators`.

---

## Phase 8 — Port `env` (init, backup, restore) + `doctor`

- [ ] Write failing cucumber-rs scenarios for `env init|backup|restore`: wire `specs/apps/rhino/behavior/cli/gherkin/env/` feature files into the integration test world. Verify: `npx nx run rhino-cli-rust:test:integration` reports the env scenarios as failing. _New test_
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port `apps/rhino-cli/internal/envbackup/` + `env init|backup|restore` into `apps/rhino-cli-rust/src/internal/envbackup/` (mirroring Go's `internal/` layout). Implement `env` subcommands in `apps/rhino-cli-rust/src/commands/env.rs`. Verify: `npx nx run rhino-cli-rust:test:integration` passes the env scenarios; `bash apps/rhino-cli-rust/scripts/shadow-diff.sh env` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Write failing cucumber-rs scenarios for `doctor`: wire `specs/apps/rhino/behavior/cli/gherkin/system/` feature files (which cover doctor) into the integration test world. Verify: `npx nx run rhino-cli-rust:test:integration` reports the doctor scenarios as failing. _New test_
  - _Suggested executor: `swe-rust-dev`_
- [ ] Port `apps/rhino-cli/internal/doctor/` (tool probes + fixer + reporter) + `doctor` command into `apps/rhino-cli-rust/src/internal/doctor/`. Implement `doctor` command in `apps/rhino-cli-rust/src/commands/doctor.rs`. Verify: `npx nx run rhino-cli-rust:run -- doctor` matches Go output; `bash apps/rhino-cli-rust/scripts/shadow-diff.sh doctor` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Replace ALL remaining `echo` stubs in `apps/rhino-cli-rust/project.json` with the real `validate:*` / `spec-coverage` commands. Verify: each `validate:*` target exits 0 against the live repo.
  - _Suggested executor: `swe-rust-dev`_
- [ ] **Full-surface parity**: `bash apps/rhino-cli-rust/scripts/shadow-diff.sh --all` exits 0 across every command + format. Confirm the Rust help tree matches `worktrees/have-two-rhino-versions/baseline-help.txt`.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Commit: `feat(rhino-cli-rust): port env + doctor; reach full Go-surface parity`.

---

## Phase 9 — Permanent parity gate

- [ ] Add a `parity` job to `.github/workflows/pr-quality-gate.yml` that `uses: ./.github/actions/setup-golang` + `./.github/actions/setup-rust`, builds both CLIs, and runs `bash apps/rhino-cli-rust/scripts/shadow-diff.sh --all`. Add it to the `quality-gate` job's `needs:` list and the failure loop. Verify: `grep -n 'parity' .github/workflows/pr-quality-gate.yml` shows the job wired into `needs`.
  - _Suggested executor: `ci-fixer`_
- [ ] Add `specs/apps/rhino/behavior/cli/gherkin/**` and both CLI source trees as triggers so the gate fires on relevant changes. Verify: review the `on`/`detect` filter includes the specs path.
  - _Suggested executor: `ci-fixer`_
- [ ] Commit: `ci(rhino-cli): add permanent go-vs-rust shadow-diff parity gate`.

---

## Phase 10 — Big-bang cutover (flip all callers Go → Rust)

> ONE thematic commit. Rust becomes the CLI every gate invokes; Go remains as the
> parity twin.

- [ ] Flip `project.json` callers: re-enumerate with `grep -rln 'rhino-cli-go' apps libs --include=project.json`. For each hit, replace `implicitDependencies: ["rhino-cli-go"]` → `["rhino-cli-rust"]` and command strings `go run -C apps/rhino-cli-go main.go <ns> <cmd>` → `cargo run --release -q --manifest-path apps/rhino-cli-rust/Cargo.toml -- <ns> <cmd>` (or `./apps/rhino-cli-rust/dist/rhino-cli <ns> <cmd>`). Verify: `grep -rn 'rhino-cli-go' apps libs --include=project.json | grep -v 'rhino-cli-rust'` returns nothing.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Flip `package.json` 8 scripts: `nx run rhino-cli-go:build` → `nx run rhino-cli-rust:build`; `./apps/rhino-cli-go/dist/rhino-cli` → `./apps/rhino-cli-rust/dist/rhino-cli`. Verify: `grep -n 'rhino-cli-go' package.json` returns nothing.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] Flip `.husky/pre-commit` and `.husky/pre-push`: replace all Go invocations (`go run -C apps/rhino-cli-go`, `nx run rhino-cli-go:validate:*`) with Rust equivalents (`cargo run … --manifest-path apps/rhino-cli-rust/Cargo.toml`, `nx run rhino-cli-rust:validate:*`). Verify: `grep -n 'rhino-cli-go' .husky/pre-commit .husky/pre-push` returns nothing.
- [ ] Flip `.github/workflows/pr-quality-gate.yml` naming job: `setup-golang` → `setup-rust`, `rhino-cli-go:validate:naming-agents` → `rhino-cli-rust:validate:naming-agents`, `rhino-cli-go:validate:naming-workflows` → `rhino-cli-rust:validate:naming-workflows`. Verify: `grep -n 'rhino-cli-rust:validate' .github/workflows/pr-quality-gate.yml` shows both targets.
  - _Suggested executor: `ci-fixer`_
- [ ] Flip `.github/workflows/pr-validate-links.yml`: `go run -C apps/rhino-cli-go …` → `cargo run --release -q --manifest-path apps/rhino-cli-rust/Cargo.toml -- docs validate-links`; swap `setup-golang` → `setup-rust`. Verify: `grep -n 'rhino-cli-go' .github/workflows/pr-validate-links.yml` returns nothing.
  - _Suggested executor: `ci-fixer`_
- [ ] **Cutover gate**: verify all callers are flipped — `grep -rn 'rhino-cli-go' apps libs package.json .husky .github --include='*'` returns ONLY the parity-gate job lines (which intentionally reference both). Then run: `npx nx affected -t typecheck lint test:quick spec-coverage --base=origin/main` exits 0 (now via Rust); `npm run sync:claude-to-opencode` no-op diff; `npm run doctor` runs via Rust; `sh .husky/pre-commit` + dry-run `.husky/pre-push` pass; `npx nx run rhino-cli-go:test:quick` + `:spec-coverage` STILL pass (twin retained). Commit: `feat(rhino-cli): cut over CI and toolchain from rhino-cli-go to rhino-cli-rust`.

---

## Phase 11 — Docs, catalog, governance convention

- [ ] Update `apps/README.md`: change the `cli` naming-table row to the `rhino-cli-<lang>` sub-pattern; replace the placeholder/duplicate `rhino-cli` entries in "Current Apps" with two accurate rows — `rhino-cli-rust` (Rust; the CI/toolchain CLI) and `rhino-cli-go` (Go; parity twin). Verify: `go run -C apps/rhino-cli-go main.go docs validate-links` (or Rust equivalent) exits 0.
  - _Suggested executor: `readme-fixer`_
- [ ] Update root `README.md` + governance docs that name the canonical CLI (`repo-governance/development/infra/ci-conventions.md`, `nx-targets.md`, `bdd-spec-test-mapping.md`, `repo-governance/development/workflow/native-first-toolchain.md`, `worktree-setup.md`): describe the dual implementation, name Rust as the CI/toolchain CLI and Go as the twin. Verify: `npm run lint:md` exits 0 and link validation passes.
  - _Suggested executor: `docs-fixer`_
- [ ] Create `repo-governance/conventions/structure/rhino-cli-dual-implementation-parity.md` documenting: both consume `specs/apps/rhino/`; Rust = CI canonical, Go = twin; shadow-diff parity gate is the enforcement mechanism (CLI analog of the `crud-be-e2e` parity model); any behavior change must land in both. Link it from the conventions index. Verify: `npx nx run rhino-cli-rust:validate:mermaid` + link validation pass.
  - _Suggested executor: `repo-rules-maker`_
- [ ] Update `specs/apps/rhino/README.md` to note both `rhino-cli-go` and `rhino-cli-rust` consume these specs. Verify: link validation exits 0.
  - _Suggested executor: `docs-fixer`_
- [ ] Commit: `docs(rhino-cli): document dual go+rust implementation and parity convention`.

---

## Local Quality Gates (Before Push) — run at end of EVERY phase

- [ ] Run affected typecheck: `npx nx affected -t typecheck --base=origin/main`
- [ ] Run affected lint: `npx nx affected -t lint --base=origin/main`
- [ ] Run affected quick tests: `npx nx affected -t test:quick --base=origin/main`
- [ ] Run affected spec coverage: `npx nx affected -t spec-coverage --base=origin/main`
- [ ] Run markdown lint: `npm run lint:md`
- [ ] Fix ALL failures found — including preexisting issues not caused by these changes.
- [ ] Verify all checks pass before pushing.

> **Important**: Fix ALL failures found during quality gates, not just those
> caused by your changes (root cause orientation principle).

## Manual Verification (CLI parity)

- [ ] Build both: `npx nx run rhino-cli-go:build && npx nx run rhino-cli-rust:build`.
- [ ] Run `./apps/rhino-cli-go/dist/rhino-cli <cmd>` and `./apps/rhino-cli-rust/dist/rhino-cli <cmd>` for a sample of each namespace; confirm identical stdout/stderr/exit code (`--no-color`, each `--output` format).
- [ ] Run `bash apps/rhino-cli-rust/scripts/shadow-diff.sh --all` — exits 0.

## Post-Push Verification

- [ ] Push to `main`: `git push origin HEAD:main`.
- [ ] Monitor `pr-quality-gate.yml` — verify all jobs pass, including the new `parity` job once Phase 9 is complete. For Phase 1 pushes specifically, confirm the naming job passes under the renamed `rhino-cli-go` targets.
- [ ] Monitor `pr-validate-links.yml` — verify the link-validation job passes.
- [ ] If any CI check fails, fix immediately and push a follow-up commit.
- [ ] Do NOT proceed to the next phase until CI is green.

## Commit Guidelines

- [ ] Commit thematically — one concern per commit (rename, scaffold, each port, cutover, docs).
- [ ] Conventional Commits format: `<type>(<scope>): <description>`.
- [ ] Do NOT bundle the cutover with unrelated fixes.

## Plan Archival

- [ ] Verify ALL delivery checklist items are ticked.
- [ ] Verify ALL quality gates pass (local + CI, including the parity job).
- [ ] Move plan folder from `plans/in-progress/` to `plans/done/` via `git mv` with a date placeholder (fill in actual completion date before running):

  ```bash
  git mv plans/in-progress/have-two-rhino-versions plans/done/<YYYY-MM-DD>__have-two-rhino-versions
  ```

- [ ] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] Update `plans/done/README.md` — add the entry with completion date.
- [ ] Commit: `chore(plans): move have-two-rhino-versions to done`.
