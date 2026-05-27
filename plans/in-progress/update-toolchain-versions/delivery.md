# Delivery Checklist — Update Polyglot Toolchain Versions

## Worktree

Worktree path: `worktrees/update-toolchain-versions/`

Provision before execution (run from repo root):

```bash
claude --worktree update-toolchain-versions
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] Install dependencies in the root worktree from the repo root:
      `npm install`
      — acceptance: exits 0, `node_modules/` synchronized with current lockfile

- [ ] Converge the polyglot toolchain:
      `npm run doctor -- --fix`
      — acceptance: exits 0, all required tools reported as OK or fixed

- [ ] Run baseline quality gates (skip-nx-cache to avoid stale results):

  ```bash
  npx nx affected -t typecheck --skip-nx-cache
  npx nx affected -t lint --skip-nx-cache
  npx nx affected -t test:quick --skip-nx-cache
  npm run lint:md
  npm run validate:harness-bindings
  npm run validate:config
  ```

  — acceptance: all exit 0 before any plan changes are applied; document any
  pre-existing failures and resolve them before proceeding

---

## Phase 1: Fix Doctor Go Path Bug

> _Executor: calling context (direct edits)_

Both doctor implementations reference `apps/rhino-cli/go.mod` which does not exist in
`ose-primer`. The correct path is `apps/rhino-cli-go/go.mod`.

### 1.1 — Fix Go implementation

**File**: `apps/rhino-cli-go/internal/doctor/tools.go`

- [ ] Change the `goModPath` variable declaration (line ~34):

  ```go
  // Before
  goModPath := filepath.Join(repoRoot, "apps", "rhino-cli", "go.mod")

  // After
  goModPath := filepath.Join(repoRoot, "apps", "rhino-cli-go", "go.mod")
  ```

- [ ] Change the `source` label string in the Go tool definition:

  ```go
  // Before
  source: "apps/rhino-cli/go.mod → go directive",

  // After
  source: "apps/rhino-cli-go/go.mod → go directive",
  ```

### 1.2 — Fix Rust implementation

**File**: `apps/rhino-cli-rust/src/internal/doctor/tools.rs`

- [ ] Change the `go_mod` path variable (line ~69):

  ```rust
  // Before
  let go_mod = repo_root.join("apps").join("rhino-cli").join("go.mod");

  // After
  let go_mod = repo_root.join("apps").join("rhino-cli-go").join("go.mod");
  ```

- [ ] Change the `source` label string in the Go tool definition:

  ```rust
  // Before
  "apps/rhino-cli/go.mod \u{2192} go directive"

  // After
  "apps/rhino-cli-go/go.mod \u{2192} go directive"
  ```

### 1.3 — Verify fix in tests

- [ ] Check that existing doctor tests reference the correct path and update if needed:

  ```bash
  grep -rn "rhino-cli/go.mod" apps/rhino-cli-go/ apps/rhino-cli-rust/
  ```

  — acceptance: grep returns no matches (all occurrences replaced)

---

## Phase 2: Update Config File Versions

> _Executor: calling context (direct edits)_

### 2.1 — Python version

**File**: `apps/crud-be-python-fastapi/.python-version`

- [ ] Replace content with:

  ```
  3.13.12
  ```

  — was: `3.13`

### 2.2 — .NET SDK version

**File**: `apps/crud-be-fsharp-giraffe/global.json`

- [ ] Update `sdk.version` from `10.0.103` to `10.0.201`:

  ```json
  {
    "sdk": {
      "version": "10.0.201",
      "rollForward": "latestMinor"
    }
  }
  ```

### 2.3 — Go minimum version directive

**File**: `apps/rhino-cli-go/go.mod`

- [ ] Change `go 1.26` to `go 1.26.1` in the module directive line

### 2.4 — Rust MSRV

**File**: `apps/crud-be-rust-axum/Cargo.toml`

- [ ] Change `rust-version = "1.80"` to `rust-version = "1.94.1"`

### 2.5 — Dart SDK and Flutter versions

**File**: `apps/crud-fe-dart-flutterweb/pubspec.yaml`

- [ ] Change `sdk: ^3.11.1` to `sdk: ^3.11.0`
- [ ] Change `flutter: ">=3.41.0"` to `flutter: ">=3.41.4"`

---

## Phase 3: Quality Gates

> _Executor: calling context_

Run all quality gates with `--skip-nx-cache` to ensure no stale cache hits.

### 3.1 — TypeScript type checking

- [ ] `npx nx affected -t typecheck --skip-nx-cache`
      — acceptance: exits 0; all projects pass; document and fix any failures before proceeding

### 3.2 — Linting

- [ ] `npx nx affected -t lint --skip-nx-cache`
      — acceptance: exits 0; all projects pass

### 3.3 — Unit tests

- [ ] `npx nx affected -t test:quick --skip-nx-cache`
      — acceptance: exits 0; all projects pass

### 3.4 — Markdown linting

- [ ] `npm run lint:md`
      — acceptance: exits 0; zero errors across all markdown files

### 3.5 — Harness binding validation

- [ ] `npm run validate:harness-bindings`
      — acceptance: exits 0; zero drift detected

### 3.6 — Config validation

- [ ] `npm run validate:config`
      — acceptance: exits 0; validate:claude + generate:bindings + validate:opencode all pass

### 3.7 — Spot-check doctor Go path

- [ ] Confirm no remaining `apps/rhino-cli/go.mod` references in source:

  ```bash
  grep -rn "apps/rhino-cli/go.mod" apps/rhino-cli-go/ apps/rhino-cli-rust/
  ```

  — acceptance: zero matches

---

## Phase 4: Commit

> _Executor: calling context_

- [ ] Commit the doctor path fixes:

  ```bash
  git add apps/rhino-cli-go/internal/doctor/tools.go \
          apps/rhino-cli-rust/src/internal/doctor/tools.rs
  git commit -m "fix(rhino-cli): correct go.mod path in doctor (rhino-cli → rhino-cli-go)"
  ```

- [ ] Commit the config file version updates:

  ```bash
  git add apps/crud-be-python-fastapi/.python-version \
          apps/crud-be-fsharp-giraffe/global.json \
          apps/rhino-cli-go/go.mod \
          apps/crud-be-rust-axum/Cargo.toml \
          apps/crud-fe-dart-flutterweb/pubspec.yaml
  git commit -m "chore(toolchain): update polyglot toolchain versions to safe targets"
  ```

---

## Phase 5: Archive Plan

> _Executor: calling context_

- [ ] Determine completion date (today's date in YYYY-MM-DD format)

- [ ] Move plan from `in-progress/` to `done/` with completion-date prefix:

  ```bash
  git mv plans/in-progress/update-toolchain-versions \
         plans/done/YYYY-MM-DD__update-toolchain-versions
  ```

- [ ] Update `plans/in-progress/README.md` — remove the entry for this plan

- [ ] Update `plans/done/README.md` — add an entry at the top of Completed Projects:

  ```markdown
  - [YYYY-MM-DD: Update Polyglot Toolchain Versions](./YYYY-MM-DD__update-toolchain-versions/README.md)
    — Updated Python, .NET, Go, Rust, Dart, Flutter toolchain version declarations; fixed
    rhino-cli doctor Go path bug (`apps/rhino-cli` → `apps/rhino-cli-go`).
  ```

- [ ] Update the plan's own `README.md` status from `In Progress` to `Completed`

- [ ] Commit the archive:

  ```bash
  git add plans/
  git commit -m "chore(plans): archive update-toolchain-versions plan to done/YYYY-MM-DD"
  ```

---

## Phase 6: Push

> _Executor: calling context_

- [ ] Push all commits to `origin/main`:

  ```bash
  git push origin main
  ```

  — acceptance: push succeeds; `git log --oneline origin/main..HEAD` returns nothing

- [ ] Final verification:

  ```bash
  git status --short
  ```

  — acceptance: clean working tree (no output)
