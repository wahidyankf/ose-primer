# Business Requirements Document — Planning System Overhaul

## Problem Statement

Three planning-system gaps remain in ose-primer after adopting ose-public's
`2026-05-25__planning-and-dev-practice` and before this plan:

1. The TDD convention documents the RED/GREEN/REFACTOR shape but doesn't explicitly forbid
   collapsing all three into one checkbox. Agents and human contributors may write
   `- [ ] Implement X with TDD` and satisfy the convention's letter while violating its
   intent.

2. `AGENTS.md` is the canonical agent catalog but does not list `repo-setup-manager`,
   does not reference the `plan-establishment-execution.md` workflow, and does not document
   that `plan-maker` mandates grilling before and after plan creation. Contributors looking
   at `AGENTS.md` get an incomplete picture.

3. Markdown lint runs on `plans/done/` and `archived/` directories. These are frozen
   historical files whose internal links legitimately rot over time. Lint failures from
   archived content are noise — they block the quality gate on historical files that are
   not being actively maintained.

## Business Goals

| ID   | Goal                                                                                  | Priority |
| ---- | ------------------------------------------------------------------------------------- | -------- |
| BG-1 | Make TDD intent unambiguous: each of RED, GREEN, REFACTOR is its own checklist item   | HIGH     |
| BG-2 | Keep `AGENTS.md` accurate and complete as the canonical agent catalog                 | HIGH     |
| BG-3 | Eliminate lint noise from archived/frozen content to keep the quality gate meaningful | MEDIUM   |

## Success Criteria

- `test-driven-development.md` contains an explicit HARD RULE: never combine phases
- `AGENTS.md` lists `repo-setup-manager` and references `plan-establishment-execution.md`
- `plans/done/` and `archived/` are excluded from markdown lint in both config files
- `npm run lint:md` exits 0 after archive exclusions are in place
- `repo-rules-checker` reports zero CRITICAL and zero HIGH findings

## Non-Goals

- Enforcing the TDD HARD RULE in `plan-checker` (future plan)
- Any changes to already-correct files: `plan-execution.md`, `plan-maker.md`,
  `plan-establishment-execution.md`, `repo-setup-manager.md`
- Changing agent or skill behavior

## Affected Roles

| Role             | Interest                                         |
| ---------------- | ------------------------------------------------ |
| Plan authors     | Clear, unambiguous TDD rules                     |
| Plan executors   | Accurate AGENTS.md catalog                       |
| Repo maintainers | Clean lint gates, no noise from archived content |

## Business Impact

**Pain points**:

- Agents and contributors write `- [ ] Implement X with TDD` and satisfy the convention's
  letter while violating its intent — the TDD intent is ambiguous without an explicit
  prohibition.
- Contributors looking at `AGENTS.md` get an incomplete picture: `repo-setup-manager` is
  absent, the `plan-establishment-execution.md` workflow is unlinked, and the grill mandate
  is undocumented.
- `npm run lint:md` fails on `plans/done/` and `archived/` directories containing frozen
  historical content whose links legitimately rot after archival. This is noise that blocks
  the quality gate on files that cannot be meaningfully fixed.

**Expected benefits**:

- An explicit HARD RULE eliminates ambiguity about TDD phase separation, reducing
  checker false-negatives and human confusion.
- A complete `AGENTS.md` catalog gives contributors an accurate, single-source view of
  all agents and their relationships.
- Archive exclusions remove lint noise, keeping the quality gate signal-to-noise ratio high
  for actively maintained files.

## Risks

| Risk                                                                                        | Mitigation                                                                                                    |
| ------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| HARD RULE paragraph inserted at wrong location in `test-driven-development.md`              | Step 1.2 uses `grep -n "HARD RULE: Never combine"` to confirm exactly one match in the correct section        |
| AGENTS.md edit breaks markdown lint (line length, list formatting)                          | Step 2.2 runs `npm run lint:md -- AGENTS.md` immediately after the edit                                       |
| `.markdownlint-cli2.jsonc` JSONC syntax error from archive entries                          | Step 3.4 runs full `npm run lint:md` which fails fast if the JSONC is malformed                               |
| Archive exclusions are too broad and accidentally hide new active content placed in `done/` | Exclusions are path-scoped to `plans/done/` and `archived/` only; new active content goes into `in-progress/` |
