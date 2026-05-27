# Delivery Checklist — Update and Pin All npm Dependencies

## Worktree

Worktree path: `worktrees/update-dependencies-pinned/`

Provision before execution (run from repo root):

```bash
claude --worktree update-dependencies-pinned
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] Install dependencies in the root worktree from the repo root:
      `npm install`
      — acceptance: exits 0, `node_modules/` synchronized with current lockfile

- [ ] Converge the full polyglot toolchain in the root worktree:
      `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift reported

- [ ] Record baseline test results to establish a before-state:
      `npx nx run-many -t test:quick 2>&1 | tee /tmp/baseline-test-results.txt`
      — acceptance: output captured; note any pre-existing failures by name so they can be
      distinguished from regressions introduced by this plan

- [ ] Record baseline typecheck results:
      `npx nx run-many -t typecheck 2>&1 | tee /tmp/baseline-typecheck-results.txt`
      — acceptance: output captured

- [ ] Resolve all pre-existing failures before proceeding:
      If `/tmp/baseline-test-results.txt` or `/tmp/baseline-typecheck-results.txt` contains failures,
      identify each failing project by name from the output.
      For each failing project, run `npx nx run <project>:test:quick --verbose` to confirm the
      failure is pre-existing (not caused by this plan's changes).
      Fix each pre-existing failure in an isolated commit:
      `git commit -m "fix(<project>): resolve pre-existing <failure-description>"`
      Re-run `npx nx run-many -t test:quick` to confirm baseline is clean before Phase 1.
      — acceptance: `npx nx run-many -t test:quick` exits 0 with no failures, OR all failures
      are documented in delivery notes as pre-existing and will be tracked separately

---

## Phase 1: Update and Pin Root package.json

> _Executor: swe-typescript-dev_

Apply the pre-researched safe target versions from `tech-docs.md §Pre-Researched Safe Targets`.

- [ ] Edit `package.json` — update `devDependencies`
      to the following exact pins (remove all `^`/`~` prefixes):

  ```json
  "@commitlint/cli": "20.5.0",
  "@commitlint/config-conventional": "20.5.0",
  "@hey-api/client-fetch": "0.13.1",
  "@hey-api/openapi-ts": "0.94.5",
  "@openapitools/openapi-generator-cli": "2.31.0",
  "@redocly/cli": "2.25.1",
  "@stoplight/spectral-cli": "6.15.0",
  "eslint-plugin-jsx-a11y": "6.10.2",
  "husky": "9.1.7",
  "lint-staged": "16.4.0",
  "markdownlint-cli2": "0.22.0",
  "nx": "22.6.2",
  "prettier": "3.8.1",
  "prettier-plugin-tailwindcss": "0.7.2",
  "tsx": "4.21.0"
  ```

  — acceptance: `grep '"^' package.json` returns no output
  for the devDependencies block

- [ ] Edit `package.json` — update `dependencies` to:

  ```json
  "tailwindcss": "4.2.2"
  ```

  — acceptance: `grep '"^' package.json` returns no output
  for the dependencies block

- [ ] Verify the `volta` block is unchanged (node: 24.13.1, npm: 11.10.1 — already exact):
      `grep -A4 '"volta"' package.json`
      — acceptance: both entries present with exact version strings, no `^`/`~`

### Phase 1 — Commit

- [ ] Commit root `package.json` changes (lockfile not yet regenerated — commit manifest only):
      `git add package.json`
      `git commit -m "chore(deps): pin and upgrade root package.json dependencies to exact versions"`
      — acceptance: commit created, only `package.json` staged

---

## Phase 2: Update and Pin App-Level package.json Files

> _Executor: swe-typescript-dev_

For each app/lib `package.json`, follow the methodology in `tech-docs.md §App-Level Package
Update Methodology`:

1. Run `npm outdated --workspace <name>` to see what the registry considers outdated.
2. For each outdated package, check release date: `npm view <pkg>@<version> time --json | grep <version>`.
3. Accept a newer version only if its release date is on or before 2026-03-27 and CVE-free.
4. Pin ALL declared versions to exact strings (including packages not being upgraded).
5. Do NOT remove `*` from workspace-internal cross-references (e.g., `"@open-sharia-enterprise/ts-ui-tokens": "*"`).

### 2a — apps/crud-fe-ts-nextjs

- [ ] Edit `apps/crud-fe-ts-nextjs/package.json`:
      Remove `^`/`~` from every version string in `dependencies` and `devDependencies`.
      Apply any eligible upgrades found via `npm outdated --workspace crud-fe-ts-nextjs`.
      — acceptance: `grep -E '"[\^~]' apps/crud-fe-ts-nextjs/package.json`
      returns no output

### 2b — apps/crud-fe-ts-tanstack-start

- [ ] Edit `apps/crud-fe-ts-tanstack-start/package.json`:
      Remove `^`/`~` from every version string. Apply eligible upgrades.
      — acceptance: `grep -E '"[\^~]' apps/crud-fe-ts-tanstack-start/package.json`
      returns no output

### 2c — apps/crud-fs-ts-nextjs

- [ ] Edit `apps/crud-fs-ts-nextjs/package.json`:
      Remove `^`/`~` from every version string. Apply eligible upgrades.
      — acceptance: `grep -E '"[\^~]' apps/crud-fs-ts-nextjs/package.json`
      returns no output

### 2d — apps/crud-be-ts-effect

- [ ] Edit `apps/crud-be-ts-effect/package.json`:
      Remove `^`/`~` from every version string. Apply eligible upgrades.
      — acceptance: `grep -E '"[\^~]' apps/crud-be-ts-effect/package.json`
      returns no output

### 2e — apps/crud-fe-e2e

- [ ] Edit `apps/crud-fe-e2e/package.json`:
      Remove `^`/`~` from every version string. Apply eligible upgrades.
      Check `@playwright/test` release dates — if the latest eligible version predates 2026-03-27,
      pin to that version; otherwise pin to the current resolved version.
      — acceptance: `grep -E '"[\^~]' apps/crud-fe-e2e/package.json`
      returns no output

### 2f — apps/crud-be-e2e

- [ ] Edit `apps/crud-be-e2e/package.json`:
      Remove `^`/`~` from every version string. Apply eligible upgrades.
      — acceptance: `grep -E '"[\^~]' apps/crud-be-e2e/package.json`
      returns no output

### 2g — libs/ts-ui

- [ ] Edit `libs/ts-ui/package.json`:
      Remove `^`/`~` from every version string in `dependencies`, `devDependencies`, and
      `peerDependencies` (except `*` workspace references). Apply eligible upgrades.
      — acceptance: `grep -E '"[\^~]' libs/ts-ui/package.json`
      returns no output

### 2h — libs/ts-ui-tokens

- [ ] Verify `libs/ts-ui-tokens/package.json` has no
      declared dependencies (current content has none):
      `cat libs/ts-ui-tokens/package.json`
      — acceptance: no `dependencies` or `devDependencies` blocks present; file unchanged

### Phase 2 — Commit

- [ ] Commit all app/lib `package.json` changes:
      `git add apps/crud-fe-ts-nextjs/package.json apps/crud-fe-ts-tanstack-start/package.json apps/crud-fs-ts-nextjs/package.json apps/crud-be-ts-effect/package.json apps/crud-fe-e2e/package.json apps/crud-be-e2e/package.json libs/ts-ui/package.json`
      `git commit -m "chore(deps): pin all app and lib package.json dependencies to exact versions"`
      — acceptance: commit created; only the eight listed files staged

---

## Phase 3: Verify and Update .tool-versions

> _Executor: swe-typescript-dev_

- [ ] Check the current `.tool-versions` content:
      `cat .tool-versions`
      — acceptance: confirms `erlang 27.3` and `elixir 1.19.5-otp-27` are declared

- [ ] Verify erlang 27.3 release date via the Erlang/OTP GitHub releases page
      (https://github.com/erlang/otp/releases/tag/OTP-27.3):
      — acceptance: release date confirmed on or before 2026-03-27

- [ ] Verify elixir 1.19.5-otp-27 release date via the Elixir GitHub releases page
      (https://github.com/elixir-lang/elixir/releases):
      — acceptance: release date confirmed on or before 2026-03-27

- [ ] Check Erlang/OTP security advisories for version 27.3
      (https://www.erlang.org/news/tags/security):
      — acceptance: no high or critical CVEs affecting 27.3 documented

- [ ] Check Elixir security advisories for version 1.19.5
      (https://elixir-lang.org/blog/ and https://github.com/elixir-lang/elixir/security):
      — acceptance: no high or critical CVEs affecting 1.19.5 documented

- [ ] If a newer eligible version of erlang exists (released on or before 2026-03-27, CVE-free),
      update `.tool-versions` to declare it;
      otherwise leave the file unchanged.
      — acceptance: declared version satisfies cutoff and CVE criteria

- [ ] If a newer eligible version of elixir exists (released on or before 2026-03-27, CVE-free),
      update `.tool-versions` to declare it;
      otherwise leave the file unchanged.
      — acceptance: declared version satisfies cutoff and CVE criteria

- [ ] If `.tool-versions` was changed, commit:
      `git add .tool-versions`
      `git commit -m "chore(deps): update .tool-versions to latest eligible erlang/elixir versions"`
      — acceptance: commit created only if file was modified; skip if file was already correct

---

## Phase 4: Regenerate Lockfile and Audit

> _Executor: repo-setup-manager_

- [ ] Run `npm install` from the repository root to regenerate `package-lock.json`
      with all updated exact versions:
      `npm install`
      — acceptance: exits 0; `package-lock.json` is updated; no ERESOLVE peer-dependency
      conflict errors in stdout or stderr

- [ ] If ERESOLVE conflicts appear:
  - [ ] Read the npm install error output and identify the conflicting package and version requirement
  - [ ] Adjust the pinned version for the conflicting package to one that satisfies all peers
        (using the cutoff date and CVE criteria from `tech-docs.md §Pre-Researched Safe Targets`)
  - [ ] Re-run `npm install` — acceptance: exits 0 with no ERESOLVE errors

- [ ] Run `npm audit --audit-level=high`:
      `npm audit --audit-level=high`
      — acceptance: exits 0; output contains "found 0 vulnerabilities" or equivalent

- [ ] If audit findings appear:
  - [ ] Read the finding description to identify the affected package and recommended fix
  - [ ] Apply the fix (update the affected package to a CVE-free version within the cutoff date)
        by editing the relevant `package.json` file with the corrected version string
  - [ ] Re-run `npm install && npm audit --audit-level=high` — acceptance: both exit 0

- [ ] Verify no `^` or `~` prefixes remain anywhere in the npm package manifests:
      `grep -rn '"[\^~]' apps/ libs/ package.json`
      — acceptance: command returns no output (empty stdout, exit 0)

### Phase 4 — Commit

- [ ] Stage and commit the regenerated lockfile:
      `git add package-lock.json`
      `git commit -m "chore(deps): regenerate package-lock.json after pinning all dependencies"`
      — acceptance: commit created; only `package-lock.json` staged

---

## Phase 5: Quality Gates and Push

> _Executor: swe-typescript-dev_

### Local Quality Gates (Before Push)

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes. This follows the root cause orientation principle — proactively fix pre-existing
> errors encountered during work. Commit pre-existing fixes separately with appropriate
> conventional commit messages.

- [ ] Run affected typecheck:
      `npx nx affected -t typecheck`
      — acceptance: exits 0, no type errors reported for any affected project

- [ ] Run affected linting:
      `npx nx affected -t lint`
      — acceptance: exits 0, no lint errors reported for any affected project

- [ ] Run affected quick tests:
      `npx nx affected -t test:quick`
      — acceptance: exits 0, all tests pass; compare against `/tmp/baseline-test-results.txt`
      to confirm no regressions introduced by this plan

- [ ] Run affected spec coverage:
      `npx nx affected -t spec-coverage`
      — acceptance: exits 0, no coverage threshold violations reported

- [ ] If any failures appear: identify whether they are regressions from this plan or
      pre-existing issues; fix all failures before proceeding
      — acceptance: zero failures across typecheck, lint, and test:quick

### Commit Guidelines

- [ ] Commit any regression fixes discovered during quality gates:
      Each fix in its own commit with message format `fix(<scope>): <description>`
      — acceptance: regression fixes are not bundled with dependency pinning commits

### Post-Push CI Verification

- [ ] Push all commits to `main`:
      `git push origin HEAD:main`
      — acceptance: push succeeds; commits appear on `main`

- [ ] Monitor all GitHub Actions workflows triggered by the push
      — acceptance: GitHub Actions UI shows one or more workflow runs triggered by the push commit SHA

- [ ] Verify all CI checks pass — no exceptions:
      `gh run list --branch main --limit 5`
      — acceptance: all runs show `completed` status with `success` conclusion

- [ ] If any CI check fails: fix the root cause immediately, push a follow-up commit, and
      re-monitor until all checks pass
      — acceptance: zero workflow runs with `failure` or `cancelled` conclusion

- [ ] Do NOT proceed to plan archival until all GitHub Actions pass with zero failures

---

## Phase 6: Final Verification

> _Executor: swe-typescript-dev_

- [ ] Confirm no range prefixes remain in any package.json (AC-1):
      `grep -rn '"[\^~]' apps/ libs/ package.json`
      — acceptance: empty output

- [ ] Confirm `npm audit --audit-level=high` still passes (AC-4):
      `npm audit --audit-level=high`
      — acceptance: exits 0

- [ ] Confirm root nx version is 22.6.2 (AC-2 sample check):
      `grep '"nx"' package.json`
      — acceptance: output shows `"nx": "22.6.2"` with no prefix

- [ ] Confirm root tailwindcss version is 4.2.2 (AC-2 sample check):
      `grep '"tailwindcss"' package.json`
      — acceptance: output shows `"tailwindcss": "4.2.2"` with no prefix

---

### Plan Archival

- [ ] Verify ALL delivery checklist items above are ticked
- [ ] Verify ALL quality gates pass (local + CI)
- [ ] Rename and move: `git mv plans/in-progress/update-dependencies-pinned/ plans/done/YYYY-MM-DD__update-dependencies-pinned/`
      using today's date as the completion date (replace `YYYY-MM-DD` with the actual completion date)
      — acceptance: folder appears under `plans/done/` with correct date prefix; `plans/in-progress/update-dependencies-pinned/` no longer exists
- [ ] Update `plans/in-progress/README.md` — remove the entry for `update-dependencies-pinned`
      — acceptance: file no longer references `update-dependencies-pinned`
- [ ] Update `plans/done/README.md` — add entry for `update-dependencies-pinned` with completion date
      — acceptance: file contains a line referencing `YYYY-MM-DD__update-dependencies-pinned` with correct date
- [ ] Commit the archival:
      `git add plans/`
      `git commit -m "chore(plans): move update-dependencies-pinned to done"`
      — acceptance: commit created; push to `main` with `git push origin HEAD:main`
