---
title: "Delivery: Harness/Vendor Neutrality Blueprint — Phase 1 (ose-primer)"
---

# Delivery Checklist: Harness/Vendor Neutrality Blueprint — Phase 1

## Worktree

**Execution mode: direct on `main` in the primary working directory — no separate worktree.**

**plan-execution Step 0 worktree gate is explicitly WAIVED for this plan by user directive.** The
user directive is verbatim: "run plan-execution for the plan; do it in the current branch." The
calling context (orchestrator) executes in-place on `main` and does NOT refuse on the missing
`worktrees/<identifier>/` path. This is an authorized, documented exception — not a gate failure.

Justification:

- Per the [Trunk Based Development Convention](../../../repo-governance/development/workflow/trunk-based-development.md),
  a worktree is an optional isolation mechanism, not a requirement; work commits directly to `main`.
- This plan is a low-risk documentation + npm-script rename with no source-logic change.
- Declared execution path: repository root (`/Users/wkf/ose-projects/ose-primer`) on `main`
  (no `worktrees/` provisioning).
- The existing `worktrees/iterative-prancing-bentley/` belongs to unrelated work and is OUT OF
  SCOPE — its files (including any `sync:claude-to-opencode` references) must NOT be touched, and
  all grep-verify commands in this plan exclude `worktrees/`.

[Judgment call — documented, user-authorized override of the default `## Worktree` provisioning
and the plan-execution Step 0 gate.]

## Phase 0: Environment Setup

- [ ] Run `npm install` from repo root — must exit 0.

- [ ] Run `npm run doctor -- --fix` — verify required tools are present.

- [ ] Run `npm run sync:claude-to-opencode` as a baseline check — must exit 0 (confirms rhino-cli
      is buildable and `agents sync` runs cleanly before the rename). **This step intentionally uses
      the old script name and MUST run in Phase 0, before Phase 1 deletes it.** Once Phase 1 lands,
      this command no longer exists; do not re-run it later.

- [ ] Run `git diff --quiet .opencode/ .amazonq/` — must exit 0 (baseline is clean). If `.amazonq/`
      shows drift here, regenerate it first via
      `./apps/rhino-cli-rust/dist/rhino-cli agents emit-bindings` and commit the baseline separately
      so the rename diff stays clean.

## Phase 1: package.json — Add generate:bindings and Remove Old Script

- [ ] Edit `package.json`: add `"generate:bindings"` where `"sync:claude-to-opencode"` currently
      sits, with value
      `"nx run rhino-cli-rust:build --skip-nx-cache && ./apps/rhino-cli-rust/dist/rhino-cli agents sync && ./apps/rhino-cli-rust/dist/rhino-cli agents emit-bindings"`.
      Verify: `node -e "const p=require('./package.json'); console.log(p.scripts['generate:bindings'])"` —
      output must be the full nx-build + sync + emit-bindings chain.

- [ ] Edit `package.json`: **delete** `"sync:claude-to-opencode"` entirely (hard delete, no alias).
      Verify: `node -e "const p=require('./package.json'); console.log(p.scripts['sync:claude-to-opencode'])"` —
      output must be `undefined`.

- [ ] Edit `package.json`: change `"validate:config"` from
      `"npm run validate:claude && npm run sync:claude-to-opencode && npm run validate:opencode"` to
      `"npm run validate:claude && npm run generate:bindings && npm run validate:opencode"`.
      Verify: `node -e "const p=require('./package.json'); console.log(p.scripts['validate:config'])"` —
      must contain `generate:bindings`.

- [ ] Run `npm run generate:bindings` — must exit 0 with the build, `agents sync`, and
      `agents emit-bindings` all completing.

- [ ] Run `git diff --quiet .opencode/ .amazonq/` — must exit 0.

- [ ] Run `npm run validate:config` — must exit 0.

- [ ] **Do NOT commit yet** — all phases complete first; commits land together in Phase 4.

## Phase 2: Documentation Sweep (governance + docs + scripts)

### Governance files

- [ ] Edit `repo-governance/development/agents/ai-agents.md`: replace all 5 occurrences of
      `sync:claude-to-opencode` with `generate:bindings`.
      Verify: `grep -c "sync:claude-to-opencode" repo-governance/development/agents/ai-agents.md` → 0.

- [ ] Edit `repo-governance/development/agents/model-selection.md`: replace the 1 occurrence.
      Verify: zero matches.

- [ ] Edit `repo-governance/development/quality/code.md`: replace all 2 occurrences.
      Verify: zero matches.

- [ ] Edit `repo-governance/conventions/structure/multi-harness-binding.md`: replace the 1
      occurrence (AD8 regenerated-data note) with `generate:bindings`.
      Verify: zero matches.

- [ ] Edit `repo-governance/workflows/repo/repo-cross-vendor-parity-quality-gate.md` in two steps:
  - Step A: replace all 3 occurrences of `sync:claude-to-opencode` with `generate:bindings`
    (Invariant 3 description + any other references). Verify: zero matches.
  - Step B: extend Invariant 3 so the diff check covers Amazon Q — wherever the Invariant 3
    description says regenerate `.opencode/`, make it read regenerate + diff `.opencode/` **and**
    `.amazonq/`. Verify: `grep -n "\.amazonq/" repo-governance/workflows/repo/repo-cross-vendor-parity-quality-gate.md`
    returns at least one Invariant-3-context match.

- [ ] Edit `repo-governance/workflows/repo/repo-harness-compatibility-quality-gate.md`: replace the
      1 occurrence (auto-fixable-scope note) with `generate:bindings`.
      Verify: zero matches.

- [ ] Edit `CLAUDE.md`: replace the 1 occurrence with `generate:bindings`.
      Verify: zero matches.

- [ ] Edit `AGENTS.md`: replace the 1 occurrence with `generate:bindings`.
      Verify: zero matches.

- [ ] Edit `README.md` (root): replace the 1 occurrence with `generate:bindings`.
      Verify: zero matches.

### Docs reference files

- [ ] Edit `docs/reference/platform-bindings.md`: replace the 1 occurrence with `generate:bindings`.
      Verify: zero matches.

### Shell scripts (dual-CLI parity pair)

- [ ] Edit `apps/rhino-cli-rust/scripts/validate-cross-vendor-parity.sh`: replace all 2 occurrences
      (invocation + error message) with `generate:bindings`.
      Verify: zero matches.

- [ ] Edit `apps/rhino-cli-go/scripts/validate-cross-vendor-parity.sh`: replace all 2 occurrences
      (invocation + error message) with `generate:bindings`.
      Verify: zero matches.

## Phase 3: Agent Definition and Skill Files Sweep

- [ ] Edit `.claude/agents/repo-parity-checker.md`: replace both occurrences. The Invariant 3 tool
      string changes from `npm run sync:claude-to-opencode && git diff --quiet .opencode/` to
      `npm run generate:bindings && git diff --quiet .opencode/ .amazonq/`. Also update the Pass/Fail
      lines so they mention `.amazonq/` drift alongside `.opencode/`.
      Verify: `grep -c "sync:claude-to-opencode" .claude/agents/repo-parity-checker.md` → 0; and
      `grep "git diff --quiet .opencode/ .amazonq/" .claude/agents/repo-parity-checker.md` → ≥1 match.

- [ ] Edit `.claude/agents/repo-parity-fixer.md`: replace all 3 occurrences (description + body).
      Verify: zero matches.

- [ ] Edit `.claude/agents/agent-maker.md`: replace the 1 occurrence in description frontmatter.
      Verify: zero matches.

- [ ] Edit `.claude/agents/README.md`: replace both occurrences.
      Verify: zero matches.

- [ ] Edit `.claude/agents/repo-harness-compatibility-fixer.md`: replace the 1 occurrence.
      Verify: zero matches.

- [ ] Edit `.claude/agents/repo-rules-fixer.md`: replace the 1 occurrence.
      Verify: zero matches.

- [ ] Edit `.claude/agents/web-research-maker.md`: replace the 1 occurrence.
      Verify: zero matches.

- [ ] Edit `.claude/skills/agent-developing-agents/SKILL.md`: replace the 1 occurrence.
      Verify: zero matches.

- [ ] Edit `.claude/skills/README.md`: replace the 1 occurrence.
      Verify: zero matches.

- [ ] Run `npm run generate:bindings` to sync all `.claude/agents/` edits to `.opencode/agents/`.
      Verify exits 0.

- [ ] Verify mirrors updated: `grep -rl "sync:claude-to-opencode" .opencode/` — zero files. If a
      non-synced mirror file (e.g. a `.opencode/**/README.md`) still contains the old name, fix it
      manually and re-verify.

## Phase 4: Coordinated Commit and Push

- [ ] Run comprehensive grep to confirm ZERO remaining occurrences:

```bash
grep -r "sync:claude-to-opencode" \
  --include="*.md" --include="*.json" --include="*.sh" --include="*.rs" --include="*.go" \
  . | grep -v "node_modules\|\.git/\|target/\|dist/\|generated-reports/\|plans/\|worktrees/\|/coverage/"
```

Expected: **zero matches**. Any match is a missed file — fix before committing. (`worktrees/` is
excluded: `worktrees/iterative-prancing-bentley/` is unrelated out-of-scope work.)

### Commit Guidelines

Commit changes thematically using [Conventional Commits](https://www.conventionalcommits.org/)
format. The commits below are pre-split by domain — do not bundle into one commit.

- [ ] Commit 1 (package.json first):
      `chore(package.json): add generate:bindings, remove sync:claude-to-opencode`

- [ ] Commit 2 (governance + docs + scripts):
      `docs(governance): replace sync:claude-to-opencode with generate:bindings`

- [ ] Commit 3 (agent definitions + skills + regenerated mirrors):
      `chore(agents): replace sync:claude-to-opencode with generate:bindings`

- [ ] Run final quality gate. Fix ALL failures found — not only those caused by this plan's changes
      (root cause orientation principle).

```bash
npm run generate:bindings                    # exits 0
git diff --quiet .opencode/ .amazonq/        # exits 0
npm run validate:config                      # exits 0
npm run validate:harness-bindings            # exits 0
npm run lint:md                              # zero violations
npx nx affected -t typecheck lint test:quick # all affected projects pass
```

- [ ] Push all commits: `git push origin HEAD:main`

- [ ] Verify GitHub Actions CI passes. Monitor with `gh run list --branch main --limit 5` at
      ~3-minute intervals; confirm checks green before proceeding to Phase 5. Pre-existing
      infrastructure flakes that are demonstrably unrelated to these doc-only changes may be noted
      and proceeded past.

## Phase 5: Governance Propagation — repo-rules-maker + repo-rules-quality-gate

- [ ] Invoke `repo-rules-maker` via the Agent tool (`subagent_type: repo-rules-maker`) with this
      verbatim prompt:

      > Check whether `repo-governance/` needs a new or updated convention entry documenting the
      > harness-neutral npm script naming pattern: the `generate:` namespace, vendor-neutral script
      > names (name the operation not the vendor), and one script per logical operation
      > (`generate:bindings` regenerates all secondary bindings). First read
      > `repo-governance/conventions/structure/multi-harness-binding.md` and
      > `repo-governance/conventions/structure/governance-vendor-independence.md`. If they already
      > cover this pattern, record that determination — no new file needed. If a gap exists, add the
      > rule to the most appropriate existing convention. HARD CONSTRAINT: ose-primer's AD8 slot is
      > already "Dual-Implementation Byte-Parity" — do NOT reuse the AD8 number for any new rule.

      Verify: `npm run lint:md` exits 0 on any new/modified governance files.

- [ ] Run the Repository Rules Quality Gate workflow in **strict mode** by following
      [`repo-governance/workflows/repo/repo-rules-quality-gate.md`](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md):
      invoke `repo-rules-checker` (Agent tool, `subagent_type: repo-rules-checker`, scope
      `repo-governance/`, mode `strict`), then `repo-rules-fixer` on threshold findings, iterating
      until zero CRITICAL/HIGH/MEDIUM findings on two consecutive checks (double-zero).

- [ ] Confirm `repo-governance/` vendor-audit passes:

```bash
./apps/rhino-cli-rust/dist/rhino-cli repo-governance vendor-audit repo-governance/
```

Must exit 0. Any vendor-audit finding is blocking — fix prose to vendor-neutral terms first.

- [ ] Commit any governance files created or modified:
      `docs(governance): document harness-neutral npm script convention`
      (or `docs(governance): no new convention needed — coverage confirmed in multi-harness-binding.md`
      if the maker determined no new file was required).

- [ ] Push: `git push origin HEAD:main`.

## Phase 6: Plan Archival

- [ ] Verify all checklist items in Phases 0–5 are ticked.

- [ ] Rename and move the plan folder (replace `YYYY-MM-DD` with completion date):

```bash
git mv plans/in-progress/harness-vendor-neutrality-blueprint \
       plans/done/YYYY-MM-DD__harness-vendor-neutrality-blueprint
```

- [ ] Update `plans/in-progress/README.md`: remove this plan's entry.

- [ ] Update `plans/done/README.md`: add this plan's entry.

- [ ] Update the plan's `README.md` front matter status to `Done`.

- [ ] Commit: `chore(plans): move harness-vendor-neutrality-blueprint to done` and push to `main`.

## Quality Gates Summary

All of the following must pass before this plan is considered done:

```bash
npm run generate:bindings
git diff --quiet .opencode/ .amazonq/
npm run validate:config
npm run validate:harness-bindings
npm run lint:md
npx nx affected -t typecheck lint test:quick
grep -r "sync:claude-to-opencode" . \
  --include="*.md" --include="*.json" --include="*.sh" --include="*.rs" --include="*.go" \
  | grep -v "node_modules\|\.git/\|target/\|dist/\|generated-reports/\|plans/\|worktrees/\|/coverage/"
./apps/rhino-cli-rust/dist/rhino-cli repo-governance vendor-audit repo-governance/
# repo-rules-quality-gate: zero CRITICAL/HIGH/MEDIUM findings on two consecutive checks
```

## Post-Push CI Verification

After pushing to `origin main`:

1. Run `gh run list --branch main --limit 3` to get the latest workflow run ID
2. Poll every ~3 minutes with `gh run view <run-id> --json status,conclusion`
3. If any check fails, investigate root cause and fix — do not bypass hooks or skip checks
4. Confirm checks green (modulo demonstrably-unrelated infra flakes) before declaring complete
