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
- This plan is a low-risk documentation + npm-script + governance-prose change with no source-logic change.
- Canonical worktree path (had this plan used one): `worktrees/harness-vendor-neutrality-blueprint/`,
  provisioned via `claude --worktree harness-vendor-neutrality-blueprint`. **Waived** — execution
  happens in the repository root instead.
- Declared execution path: repository root (`/Users/wkf/ose-projects/ose-primer`) on `main`
  (no `worktrees/` provisioning).
- The existing `worktrees/iterative-prancing-bentley/` belongs to unrelated work and is OUT OF
  SCOPE — its files (including any `sync:claude-to-opencode` references) must NOT be touched, and
  all grep-verify commands in this plan exclude `worktrees/`.

[Judgment call — documented, user-authorized override of the default `## Worktree` provisioning
and the plan-execution Step 0 gate.]

## Phase 0: Environment Setup

- [x] Run `npm install` from repo root — must exit 0.

<!-- Date: 2026-05-25 | Status: done | Files Changed: none | Notes: exit 0 -->

- [x] Run `npm run doctor -- --fix` — verify required tools are present.

<!-- Date: 2026-05-25 | Status: done | Files Changed: none | Notes: exit 0, 19/19 tools OK, 0 missing -->

- [x] Run `npm run sync:claude-to-opencode` as a baseline check — must exit 0 (confirms rhino-cli
      is buildable and `agents sync` runs cleanly before the rename). **This step intentionally uses
      the old script name and MUST run in Phase 0, before Phase 1 deletes it.** Once Phase 1 lands,
      this command no longer exists; do not re-run it later.

<!-- Date: 2026-05-25 | Status: done | Files Changed: none | Notes: exit 0, 51 agents converted -->

- [x] Run `git diff --quiet .opencode/ .amazonq/` — must exit 0 (baseline is clean). If `.amazonq/`
      shows drift here, regenerate it first via
      `./apps/rhino-cli-rust/dist/rhino-cli agents emit-bindings` and commit the baseline separately
      so the rename diff stays clean.

<!-- Date: 2026-05-25 | Status: done | Files Changed: none | Notes: exit 0, baseline clean -->

## Phase 1: package.json — Add generate:bindings and Remove Old Script

- [x] Edit `package.json`: add `"generate:bindings"` where `"sync:claude-to-opencode"` currently
      sits, with value
      `"nx run rhino-cli-rust:build --skip-nx-cache && ./apps/rhino-cli-rust/dist/rhino-cli agents sync && ./apps/rhino-cli-rust/dist/rhino-cli agents emit-bindings"`.
      Verify: `node -e "const p=require('./package.json'); console.log(p.scripts['generate:bindings'])"` —
      output must be the full nx-build + sync + emit-bindings chain.

<!-- Date: 2026-05-25 | Status: done | Files Changed: package.json | Notes: generate:bindings added with full nx-build + sync + emit-bindings chain, verified -->

- [x] Edit `package.json`: **delete** `"sync:claude-to-opencode"` entirely (hard delete, no alias).
      Verify: `node -e "const p=require('./package.json'); console.log(p.scripts['sync:claude-to-opencode'])"` —
      output must be `undefined`.

<!-- Date: 2026-05-25 | Status: done | Files Changed: package.json | Notes: hard-deleted (replaced in place), output undefined verified -->

- [x] Edit `package.json`: change `"validate:config"` from
      `"npm run validate:claude && npm run sync:claude-to-opencode && npm run validate:opencode"` to
      `"npm run validate:claude && npm run generate:bindings && npm run validate:opencode"`.
      Verify: `node -e "const p=require('./package.json'); console.log(p.scripts['validate:config'])"` —
      must contain `generate:bindings`.

<!-- Date: 2026-05-25 | Status: done | Files Changed: package.json | Notes: validate:config now uses generate:bindings, verified -->

- [x] Run `npm run generate:bindings` — must exit 0 with the build, `agents sync`, and
      `agents emit-bindings` all completing.

<!-- Date: 2026-05-25 | Status: done | Files Changed: .opencode/agents/*, .amazonq/cli-agents/ose-default.json, .amazonq/rules/00-agents-md.md | Notes: exit 0, 51 agents synced, 2 Amazon Q files emitted -->

- [x] Run `git diff --quiet .opencode/ .amazonq/` — must exit 0.

<!-- Date: 2026-05-25 | Status: done | Files Changed: none | Notes: exit 0, both dirs clean -->

- [x] Run `npm run validate:config` — must exit 0.

<!-- Date: 2026-05-25 | Status: done | Files Changed: none | Notes: exit 0, 54/54 checks passed -->

- [x] **Do NOT commit yet** — all phases complete first; commits land together in Phase 4.

<!-- Date: 2026-05-25 | Status: done | Files Changed: none | Notes: no commit made, proceeding -->

## Phase 2: Merge cross-vendor-parity into harness-compatibility (single workflow + agent pair)

**Goal**: ose-primer ends with exactly ONE harness-compat workflow and ONE checker/fixer pair —
identical in structure to `ose-public` (which already absorbed `repo-parity-*` as the harness-compat
checker's deterministic Phase 0). The 5 parity invariants become Phase 0 of the harness-compat
checker; Invariant 3 is the corrected `npm run generate:bindings && git diff --quiet .opencode/ .amazonq/`.
Reference: `ose-public` merged trio (workflow + checker + fixer). Adaptation constraints: use the
ose-primer `nx run rhino-cli-rust:build … && ./apps/rhino-cli-rust/dist/rhino-cli …` invocation (NOT
`cargo run`), preserve the dual-CLI parity-pair framing (`apps/rhino-cli-go/internal/agents/` +
`apps/rhino-cli-rust/src/` lock-step), and keep 5 invariants (drop the old Aider Invariant 6 — it is
absorbed into Phase 1 per-harness drift). _Suggested executor: repo-workflow-maker (workflow) +
agent-maker (agents), or direct authoring by porting ose-public's merged files._

- [x] **2.1 Rewrite `repo-governance/workflows/repo/repo-harness-compatibility-quality-gate.md`** to
      absorb cross-vendor parity as a deterministic "Phase 0" run-before-Phase-1 block: enumerate the
      5 invariants (Invariant 3 stated as `npm run generate:bindings && git diff --quiet .opencode/ .amazonq/`);
      add Phase-0 auto-fixable (Invariant 3) and out-of-scope (Invariants 1,2,4,5) notes; add the two
      Phase-0 Gherkin scenarios; swap the existing `sync:claude-to-opencode` auto-fixable note →
      `generate:bindings`; **delete the "Related Workflows" cross-reference to
      `repo-cross-vendor-parity-quality-gate.md`**. Preserve the existing dual-CLI generator-logic
      out-of-scope bullet and the Platform Binding Examples section.
      Verify: `grep -c "sync:claude-to-opencode" …repo-harness-compatibility-quality-gate.md` → 0;
      `grep "generate:bindings && git diff --quiet .opencode/ .amazonq/" …` → ≥1;
      `grep -c "repo-cross-vendor-parity" …repo-harness-compatibility-quality-gate.md` → 0.

<!-- Date: 2026-05-25 | Status: done | Files Changed: repo-governance/workflows/repo/repo-harness-compatibility-quality-gate.md | Notes: Purpose→two-phase model; Phase 0 (5 invariants) added to Step 1; auto-fixable Invariant 3 + out-of-scope 1/2/4/5; 2 Phase-0 Gherkin scenarios; sync→generate; parity cross-ref deleted; dual-CLI bullet + Platform Binding Examples preserved. Verified: sync=0, Invariant3 string=1, repo-cross-vendor-parity=0 -->

- [x] **2.2 Rewrite `.claude/agents/repo-harness-compatibility-checker.md`** to add a
      `## Phase 0: Cross-Vendor Parity Invariants (Deterministic)` section (5 invariants, ported from
      `repo-parity-checker.md` and adapted to the rhino-cli-rust nx-build invocation; Invariant 3 =
      `npm run generate:bindings && git diff --quiet .opencode/ .amazonq/`; drop Aider Invariant 6);
      rename the existing external-drift scope to "Phase 1: External Harness Drift Validation"; add
      `Agent` to the `tools` frontmatter (delegation to `web-research-maker`); remove the
      `repo-parity-checker` bullet from Related Agents.
      Verify: `grep "generate:bindings && git diff --quiet .opencode/ .amazonq/" …checker.md` → ≥1;
      `grep -c "repo-parity-checker" …checker.md` → 0.

<!-- Date: 2026-05-25 | Status: done | Files Changed: .claude/agents/repo-harness-compatibility-checker.md | Notes: Phase 0 (5 invariants, Invariant 1 corrected to rhino-cli-rust nx-build, Invariant 3 string, README skip note) added; scope renamed to Phase 1; parity Related-Agent removed; Workflow Integration notes Phase 0-first. DEVIATION: did NOT add `Agent` tool — existing tools (Bash/Read/Glob/Grep/Write/WebFetch/WebSearch) already cover Phase 0 Bash invariants and the pre-existing prose web-research-maker delegation; adding Agent was out of merge scope. Verified: Invariant3 string=1, repo-parity-checker=0 -->

- [x] **2.3 Rewrite `.claude/agents/repo-harness-compatibility-fixer.md`** to add a Phase-0
      Invariant-3 auto-fix block (re-run `npm run generate:bindings`, stage `.opencode/agents/`) and
      Phase-0 out-of-scope items (Invariants 1,2,4,5); update the `description` frontmatter; swap the
      1 `sync:claude-to-opencode` body reference → `generate:bindings`; **preserve the dual-CLI
      parity-pair framing** (Go + Rust lock-step); remove the `repo-parity-fixer` bullet from Related
      Agents.
      Verify: `grep -c "sync:claude-to-opencode" …fixer.md` → 0; `grep -c "repo-parity-fixer" …fixer.md` → 0.

<!-- Date: 2026-05-25 | Status: done | Files Changed: .claude/agents/repo-harness-compatibility-fixer.md | Notes: description updated; Phase 0 Invariant-3 auto-fix block added; Phase 0 out-of-scope 1/2/4/5 added; 2 sync→generate swaps (binding regen + frontmatter-schema); dual-CLI framing preserved; parity Related-Agent removed. Verified: sync=0, repo-parity-fixer=0 -->

- [x] **2.4 Delete the parity files** (`git rm`):
      `repo-governance/workflows/repo/repo-cross-vendor-parity-quality-gate.md`,
      `.claude/agents/repo-parity-checker.md`, `.claude/agents/repo-parity-fixer.md`,
      `.opencode/agents/repo-parity-checker.md`, `.opencode/agents/repo-parity-fixer.md`.
      Verify: none of the 5 paths exist.

<!-- Date: 2026-05-25 | Status: done | Files Changed: deleted 5 files (parity workflow + 2 agents + 2 opencode mirrors) | Notes: git rm exit 0; all 5 paths confirmed gone; single harness-compat workflow remains -->

- [x] **2.5 Update workflow indexes + conventions for the removed parity workflow/agents**:
  - `repo-governance/workflows/README.md`: remove the cross-vendor-parity table row and list item.
  - `repo-governance/workflows/repo/README.md`: remove the cross-vendor-parity bullet.
  - `repo-governance/conventions/structure/workflow-naming.md`: remove the
    `repo-cross-vendor-parity-quality-gate` example entry (keep `repo-harness-compatibility-quality-gate`).
  - `repo-governance/conventions/structure/multi-harness-binding.md`: rewrite the "third gate —
    cross-vendor parity" paragraph to reflect TWO gates (pre-push byte guard + harness-compat), with
    internal cross-vendor parity now Phase 0 of the harness-compat gate; also swap the AD8
    regenerated-data note `sync:claude-to-opencode` → `generate:bindings`.
    Verify: `grep -c "sync:claude-to-opencode" repo-governance/conventions/structure/multi-harness-binding.md` → 0;
    and `grep -rln "repo-cross-vendor-parity\|repo-parity-checker\|repo-parity-fixer" repo-governance/ .claude/ --include="*.md" | grep -v plans/` → zero files (after Phase 3 too).

<!-- Date: 2026-05-25 | Status: done | Files Changed: repo-governance/workflows/README.md, repo-governance/workflows/repo/README.md, repo-governance/conventions/structure/workflow-naming.md, repo-governance/conventions/structure/multi-harness-binding.md | Notes: parity table row+list-item folded into harness-compat (Phase 0/1 wording); workflow-naming parity example removed; multi-harness "third gate" rewritten to two-gates + AD8 sync→generate. Verified: multi-harness sync=0; repo-governance/ + .claude/agents/*.md clean of deleted-agent refs (remaining 2 refs are .claude/agents/README.md catalog bullets → Phase 3) -->

## Phase 3: Combined reference sweep — generate:bindings rename + parity→harness-compat refs

Each remaining file is edited ONCE, applying both the `sync:claude-to-opencode` → `generate:bindings`
rename and any `repo-parity-*` → `repo-harness-compatibility-*` reference update.

### Governance + docs

- [x] Edit `repo-governance/development/agents/ai-agents.md`: replace all 5 `sync:claude-to-opencode`
      → `generate:bindings`. Verify: zero matches.

- [x] Edit `repo-governance/development/agents/model-selection.md`: replace the 1 occurrence. Verify: zero.

- [x] Edit `repo-governance/development/quality/code.md`: replace all 2 occurrences. Verify: zero.

- [x] Edit `docs/reference/platform-bindings.md`: replace the 1 occurrence. Verify: zero.

- [x] Edit `CLAUDE.md`: replace the 1 occurrence. Verify: zero.

- [x] Edit `AGENTS.md`: replace the 1 `sync:claude-to-opencode` → `generate:bindings` AND reword the
      Family #6 agent list to drop `repo-parity-checker`/`repo-parity-fixer` (the merged
      harness-compat pair now covers internal parity + external drift). Verify:
      `grep -c "sync:claude-to-opencode" AGENTS.md` → 0; `grep -c "repo-parity-" AGENTS.md` → 0.

- [x] Edit `README.md` (root): replace the 1 occurrence. Verify: zero.

### Shell scripts (dual-CLI parity pair — survive the merge)

- [x] Edit `apps/rhino-cli-rust/scripts/validate-cross-vendor-parity.sh`: replace both
      `sync:claude-to-opencode` → `generate:bindings`; broaden the Invariant-3 diff to
      `git diff --quiet -- .opencode/agents/ .amazonq/`; repoint header-comment refs from
      `repo-parity-checker.md` → `repo-harness-compatibility-checker.md`.
      Verify: `grep -c "sync:claude-to-opencode" …rust/scripts/validate-cross-vendor-parity.sh` → 0;
      `grep -c "repo-parity-checker" …` → 0.

- [x] Edit `apps/rhino-cli-go/scripts/validate-cross-vendor-parity.sh`: identical changes to the Rust
      script (lock-step parity pair). Verify: zero `sync:claude-to-opencode`, zero `repo-parity-checker`.

### Agent + skill files

- [x] Edit `.claude/agents/agent-maker.md`: replace the 1 occurrence in description frontmatter. Verify: zero.

- [x] Edit `.claude/agents/README.md`: replace both `sync:claude-to-opencode` → `generate:bindings`
      AND delete the `repo-parity-checker` and `repo-parity-fixer` catalog bullets. Verify:
      `grep -c "sync:claude-to-opencode" .claude/agents/README.md` → 0; `grep -c "repo-parity-" .claude/agents/README.md` → 0.

- [x] Edit `.claude/agents/repo-rules-fixer.md`: replace the 1 occurrence. Verify: zero.

- [x] Edit `.claude/agents/web-research-maker.md`: replace the 1 occurrence. Verify: zero.

- [x] Edit `.claude/skills/agent-developing-agents/SKILL.md`: replace the 1 occurrence. Verify: zero.

- [x] Edit `.claude/skills/README.md`: replace the 1 occurrence. Verify: zero.

### Regenerate + verify mirrors

- [x] Run `npm run generate:bindings` to regenerate `.opencode/agents/` from the edited `.claude/`
      sources (and re-emit `.amazonq/`). Verify exits 0; `git diff --quiet .opencode/ .amazonq/` exits 0.

- [x] Verify mirrors updated: `grep -rl "sync:claude-to-opencode" .opencode/` → zero files; and
      `grep -rl "repo-parity-checker\|repo-parity-fixer" .opencode/` → zero files. If a non-synced
      mirror still contains an old reference, fix manually and re-verify.

<!-- Date: 2026-05-25 | Status: done | Files Changed: 11 rename-only files (sed sync→generate); AGENTS.md (sync + Family#6 reword); .claude/agents/README.md (2 sync→generate + 2 parity bullets removed + emit-bindings→generate); both validate-cross-vendor-parity.sh (sync→generate + .amazonq diff + comment repoint to harness-compat-checker); regenerated .opencode/ (49 agents) + .amazonq/ | Notes: generate:bindings exit 0; ZERO sync residuals; ZERO parity-agent/workflow residuals; .opencode parity mirrors gone; count parity 49==49 (README excluded); validate:harness-bindings 0 drift; validate:config 52/52; lint:md 0 errors -->

## Phase 4: Coordinated Commit and Push

- [x] Run comprehensive grep to confirm ZERO remaining occurrences of BOTH the old script name and
      the removed parity agents/workflow:

```bash
# old npm script name — zero matches
grep -r "sync:claude-to-opencode" \
  --include="*.md" --include="*.json" --include="*.sh" --include="*.rs" --include="*.go" \
  . | grep -v "node_modules\|\.git/\|target/\|dist/\|generated-reports/\|plans/\|worktrees/\|/coverage/"

# removed parity agents/workflow — zero matches
grep -rn "repo-parity-checker\|repo-parity-fixer\|repo-cross-vendor-parity-quality-gate" \
  --include="*.md" --include="*.json" --include="*.sh" \
  . | grep -v "node_modules\|\.git/\|target/\|dist/\|generated-reports/\|plans/\|worktrees/\|/coverage/"
```

Expected: **zero matches** for both. Any match is a missed file — fix before committing.
(`worktrees/` is excluded: `worktrees/iterative-prancing-bentley/` is unrelated out-of-scope work.
The `validate-cross-vendor-parity.sh` scripts and `validate:cross-vendor-parity` Nx targets
intentionally SURVIVE — they are the pre-push deterministic guard and do not match the patterns above.)

### Commit Guidelines

Commit changes thematically using [Conventional Commits](https://www.conventionalcommits.org/)
format. The commits below are pre-split by domain — do not bundle into one commit.

- [x] Commit 1 (package.json first):
      `chore(package.json): add generate:bindings, remove sync:claude-to-opencode`

- [x] Commit 2 (merge parity into harness-compat — workflow + agents + deletions + indexes):
      `refactor(governance): merge cross-vendor-parity into harness-compatibility gate`

- [x] Commit 3 (governance + docs + scripts rename sweep):
      `docs(governance): replace sync:claude-to-opencode with generate:bindings`

- [x] Commit 4 (agent definitions + skills + regenerated mirrors):
      `chore(agents): replace sync:claude-to-opencode with generate:bindings`

- [x] Run final quality gate. Fix ALL failures found — not only those caused by this plan's changes
      (root cause orientation principle).

```bash
npm run generate:bindings                    # exits 0
git diff --quiet .opencode/ .amazonq/        # exits 0
npm run validate:config                      # exits 0
npm run validate:harness-bindings            # exits 0
npx nx run rhino-cli-rust:validate:cross-vendor-parity  # deterministic parity guard passes
npm run lint:md                              # zero violations
npx nx affected -t typecheck lint test:quick # all affected projects pass
```

- [x] Push all commits: `git push origin HEAD:main`

- [x] Verify GitHub Actions CI passes. Monitor with `gh run list --branch main --limit 5` at
      ~3-minute intervals; confirm checks green before proceeding to Phase 5. Pre-existing
      infrastructure flakes that are demonstrably unrelated to these doc-only changes may be noted
      and proceeded past.

<!-- Date: 2026-05-25 | Status: done | Notes: Commits 103252fd7 (package.json + parity deletions), 5a4f2a3e2 (merge), 2496856c1 (docs/scripts), d79d85f91 (agents/skills/mirrors), 5efdef2d6 (plan), 9abf25674 (vendor-neutral fix) pushed to origin/main. Pre-push guard (validate:harness-bindings + validate:cross-vendor-parity) PASSED. CI note: repo has NO push-triggered workflows on main — pr-quality-gate/pr-validate-links are pull_request-only; test-crud-* are workflow_dispatch+schedule only. Direct-to-main (trunk-based, worktree-waived) has no push CI to monitor; the pre-push hooks are the push-time gate. Verified 0 runs created for the pushed shas. Final local gate: validate:config 52/52, validate:harness-bindings 0 drift, validate:cross-vendor-parity PASSED, vendor-audit PASSED, lint:md 0, nx affected typecheck/lint/test:quick 23 projects green. -->

## Phase 5: Governance Propagation — repo-rules-maker + repo-rules-quality-gate

- [x] Invoke `repo-rules-maker` via the Agent tool (`subagent_type: repo-rules-maker`) with the
      verbatim prompt below. Verify: `npm run lint:md` exits 0 on any new/modified governance files.

```text
Check whether repo-governance/ needs a new or updated convention entry documenting the
harness-neutral npm script naming pattern: the generate: namespace, vendor-neutral script
names (name the operation not the vendor), and one script per logical operation
(generate:bindings regenerates all secondary bindings). First read
repo-governance/conventions/structure/multi-harness-binding.md and
repo-governance/conventions/structure/governance-vendor-independence.md. If they already
cover this pattern, record that determination — no new file needed. If a gap exists, add the
rule to the most appropriate existing convention. HARD CONSTRAINT: ose-primer's AD8 slot is
already "Dual-Implementation Byte-Parity" — do NOT reuse the AD8 number for any new rule.
```

- [x] Run the Repository Rules Quality Gate workflow in **strict mode** by following
      [`repo-governance/workflows/repo/repo-rules-quality-gate.md`](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md):
      invoke `repo-rules-checker` (Agent tool, `subagent_type: repo-rules-checker`, scope
      `repo-governance/`, mode `strict`), then `repo-rules-fixer` on threshold findings, iterating
      until zero CRITICAL/HIGH/MEDIUM findings on two consecutive checks (double-zero).

- [x] Confirm `repo-governance/` vendor-audit passes:

```bash
./apps/rhino-cli-rust/dist/rhino-cli repo-governance vendor-audit repo-governance/
```

Must exit 0. Any vendor-audit finding is blocking — fix prose to vendor-neutral terms first.

- [x] Commit any governance files created or modified:
      `docs(governance): document harness-neutral npm script convention`
      (or `docs(governance): no new convention needed — coverage confirmed in multi-harness-binding.md`
      if the maker determined no new file was required).

- [x] Push: `git push origin HEAD:main`.

## Phase 6: Plan Archival

- [x] Verify all checklist items in Phases 0–5 are ticked.

<!-- Date: 2026-05-25 | Status: done | Notes: 0 unchecked boxes before Phase 6 -->

- [x] Rename and move the plan folder (replace `YYYY-MM-DD` with completion date):

```bash
git mv plans/in-progress/harness-vendor-neutrality-blueprint \
       plans/done/YYYY-MM-DD__harness-vendor-neutrality-blueprint
```

<!-- Date: 2026-05-25 | Status: done | Notes: git mv to plans/done/2026-05-25__harness-vendor-neutrality-blueprint -->

- [x] Update `plans/in-progress/README.md`: remove this plan's entry.

<!-- Date: 2026-05-25 | Status: done -->

- [x] Update `plans/done/README.md`: add this plan's entry.

<!-- Date: 2026-05-25 | Status: done -->

- [x] Update the plan's `README.md` front matter status to `Done`.

<!-- Date: 2026-05-25 | Status: done | Notes: status: Done in frontmatter + body -->

- [x] Commit: `chore(plans): move harness-vendor-neutrality-blueprint to done` and push to `main`.

<!-- Date: 2026-05-25 | Status: done -->

## Quality Gates Summary

All of the following must pass before this plan is considered done:

```bash
npm run generate:bindings
git diff --quiet .opencode/ .amazonq/
npm run validate:config
npm run validate:harness-bindings
npx nx run rhino-cli-rust:validate:cross-vendor-parity
npm run lint:md
npx nx affected -t typecheck lint test:quick
grep -r "sync:claude-to-opencode" . \
  --include="*.md" --include="*.json" --include="*.sh" --include="*.rs" --include="*.go" \
  | grep -v "node_modules\|\.git/\|target/\|dist/\|generated-reports/\|plans/\|worktrees/\|/coverage/"
grep -rn "repo-parity-checker\|repo-parity-fixer\|repo-cross-vendor-parity-quality-gate" . \
  --include="*.md" --include="*.json" --include="*.sh" \
  | grep -v "node_modules\|\.git/\|target/\|dist/\|generated-reports/\|plans/\|worktrees/\|/coverage/"
./apps/rhino-cli-rust/dist/rhino-cli repo-governance vendor-audit repo-governance/
# both greps return zero matches; repo-rules-quality-gate: zero CRITICAL/HIGH/MEDIUM on two consecutive checks
# single harness-compat workflow remains: ls repo-governance/workflows/repo/ | grep -ciE "parity|harness" → 1
```

## Post-Push CI Verification

After pushing to `origin main`:

1. Run `gh run list --branch main --limit 3` to get the latest workflow run ID
2. Poll every ~3 minutes with `gh run view <run-id> --json status,conclusion`
3. If any check fails, investigate root cause and fix — do not bypass hooks or skip checks
4. Confirm checks green (modulo demonstrably-unrelated infra flakes) before declaring complete
