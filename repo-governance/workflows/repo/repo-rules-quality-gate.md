---
name: repo-rules-quality-gate
title: "repo-rules-quality-gate"
goal: Validate repository consistency across all layers, apply fixes iteratively until zero findings achieved
termination: "Zero findings on two consecutive validations (max-iterations defaults to 7, escalation warning at 5)"
inputs:
  - name: mode
    type: enum
    values: [lax, normal, strict, ocd]
    description: "Quality threshold (lax: CRITICAL only, normal: CRITICAL/HIGH, strict: +MEDIUM, ocd: all levels)"
    required: false
    default: strict
  - name: min-iterations
    type: number
    description: Minimum check-fix cycles before allowing zero-finding termination (prevents premature success)
    required: false
  - name: max-iterations
    type: number
    description: Maximum check-fix cycles to prevent infinite loops
    required: false
    default: 7
  - name: max-concurrency
    type: number
    description: "Background agents run concurrently — the N in the N+1 model (1 main thread + N background agents = N+1 total). Raise only when independent work, machine capacity, and budget headroom all allow; lower under budget, runner, or disk pressure. Never self-promoted beyond the declared value."
    required: false
    default: 3
outputs:
  - name: final-status
    type: enum
    values: [pass, partial, fail]
    description: Final validation status
  - name: iterations-completed
    type: number
    description: Number of check-fix cycles executed
  - name: final-report
    type: file
    pattern: generated-reports/repo-rules__*__*__audit.md
    description: Final audit report (4-part format with UUID chain)
  - name: execution-scope
    type: string
    description: Scope identifier for UUID chain tracking (default "repo-rules")
    required: false
---

# Repository Rules Validation Workflow

**Purpose**: Automatically validate repository consistency across principles, conventions, development practices, agent and skill source definitions, and subdirectory README files, then apply fixes iteratively until all issues are resolved.

**IMPORTANT - Scope Clarification**:

This workflow validates **source definitions only** in `repo-governance/`. It does NOT validate generated directories:

- **Validates**: `repo-governance/` (principles, conventions, development practices)
- **Skips**: `.opencode/agents/` (auto-generated from `.claude/agents/` - validate via sync script)
- **Skips**: `.opencode/skills/` (Nx-native skills, not mirrored from `.claude/skills/`)

**Generated Output Validation**: Use CLI validation commands for validating generated content. This workflow ensures SOURCE is correct, then sync commands validate output generation.

**When to use**:

- After making changes to conventions, principles, or development practices
- Before major releases or deployments
- Periodically to ensure repository health
- After adding or modifying agents

## Execution Mode

**Preferred Mode**: Agent Delegation — invoke `repo-rules-checker` and
`repo-rules-fixer` via the Agent tool with `subagent_type`
(see [Workflow Execution Modes Convention](../meta/execution-modes.md)).

**Fallback Mode**: Manual Orchestration — execute workflow logic directly using
Read/Write/Edit tools when Agent Delegation is unavailable.

The Agent tool runs subagents that persist file changes to the actual filesystem, making it
the preferred approach when these agents exist as defined subagent types.

**How to Execute**:

```
User: "Run repository rules quality gate workflow in normal mode"
```

The AI will:

0. Build the canonical Rust binary if missing (`npx nx run rhino-cli:build`), then run
   the deterministic preflight (Step 0.5) capturing the four-category JSON envelope to
   `generated-reports/`.
1. Invoke `repo-rules-checker` via the Agent tool (reads governance files, writes audit)
2. Invoke `repo-rules-fixer` via the Agent tool (reads audit, applies fixes, writes fix report)
3. Iterate until zero findings achieved
4. Show git status with modified files
5. Wait for user commit approval

**Fallback (Manual Mode)**:

```
User: "Run repository rules quality gate workflow in manual mode"
```

The AI executes checker and fixer logic directly using Read/Write/Edit tools in the main
context — use this when agent delegation is unavailable.

## Steps

### 0.5. Deterministic Preflight (Sequential)

Step 0.5 runs the `rhino-cli` orchestrator to harvest the deterministic governance findings before invoking the AI checker. The `repo-governance audit` orchestrator runs exactly **four** governance categories in fixed order — `layer-coherence`, `traceability-audit`, `vendor-audit`, and `instruction-size` (byte budget check on all auto-loaded instruction surfaces per the `instruction-size:` section in `repo-config.yml`) — normalises their findings into one JSON envelope, and caches via Nx. The other deterministic markdown/convention/harness validators (file naming, frontmatter shape, license presence, README index integrity, emoji codepoints, heading hierarchy, agent-skill verbatim duplication, gherkin-keyword-cardinality) live under sibling `rhino-cli` subcommands (`md`, `convention`, `harness`, `specs`) and are enforced by the markdown and commit gates — they are not part of this preflight. The AI checker then runs only the AI-only categories (paraphrased duplication, semantic contradictions, terminology alignment, principle-appropriateness judgement).

**Why Step 0.5 (and not Step 1, renumbering everything down)**: this step was inserted
between the pre-existing "Execution Mode" material and Step 1 (Initial Validation).
Decimal numbering preserves the existing Step 1–6 references in the checker/fixer prompts
that pre-date the preflight, following the sibling-repo precedent for non-disruptive
sub-step insertions.

**Command**:

```bash
mkdir -p generated-reports
./apps/rhino-cli/dist/rhino-cli repo-governance audit -o json > generated-reports/repo-governance-audit__{uuid}__{timestamp}.json
```

The binary must be built first via `npx nx run rhino-cli:build`; the prebuilt path is
`apps/rhino-cli/dist/rhino-cli`.

- **Output**: `{preflight-report}` — JSON envelope at the captured path; schema `rhino-cli/repo-governance-audit/v1`
- **Exit handling**:
  - Exit 0 (clean): All deterministic categories pass; pass JSON path to checker.
  - Exit 1 (findings): Deterministic findings present; pass JSON path to checker (the checker
    incorporates the deterministic findings verbatim into the final audit's "Deterministic
    Findings (rhino-cli preflight)" section). Deterministic findings are reported but do NOT
    count toward the mode threshold — they are fixed at their root (the offending file) rather
    than iterated through the AI loop.
  - Exit 2 (invocation error): Terminate the workflow with `fail` status. **Debugging hint**:
    re-run with `./apps/rhino-cli/dist/rhino-cli repo-governance audit -o text` for a
    human-readable diagnostic. Common causes: missing binary (rebuild via
    `npx nx run rhino-cli:build`); a broken category function (run the individual
    `./apps/rhino-cli/dist/rhino-cli repo-governance <category> validate` — one of
    `layer-coherence`, `traceability`, `vendor` — or
    `./apps/rhino-cli/dist/rhino-cli harness instruction-size validate` — to isolate); running
    outside a git repository.

> **Operator hatch**: The orchestrator accepts `--skip <category>` (one of `layer-coherence`,
> `traceability-audit`, `vendor-audit`, `instruction-size`) to bypass a whole category, and
> `--exclude <glob>` to drop findings whose path matches a glob.

**Success criteria**: Preflight completes; JSON file exists at expected path; JSON parses as
valid `AuditEnvelope` with `schema` field set to `rhino-cli/repo-governance-audit/v1`.

**Depends on**: None (first step in each iteration). Runs again before every re-validation
iteration; if the JSON SHA-256 is unchanged from the prior iteration, the checker reuses the
deterministic findings section unchanged and only re-evaluates AI-only categories.

### 1. Initial Validation (Sequential)

Run repository-wide consistency check to identify all issues.

**Agent**: `repo-rules-checker`

- **Args**: `scope: all, EXECUTION_SCOPE: repo-rules, preflight-report: {step0_5.outputs.preflight-report}`
- **Output**: `{audit-report-1}` - Initial audit report in `generated-reports/` (4-part format: `repo-rules__{uuid-chain}__{timestamp}__audit.md`)

**UUID Chain Tracking**: Checker generates 6-char UUID and writes to `generated-reports/.execution-chain-repo-rules` before spawning any child agents. See [Temporary Files Convention](../../development/infra/temporary-files.md#uuid-generation) for details.

**Note on preflight unavailability**: If the `preflight-report` argument is missing, the file does not exist, or the JSON fails schema validation, the AI checker falls back to full Steps 1-8 evaluation per its own Step 0.5 graceful-degradation rule (`.claude/agents/repo-rules-checker.md`). This is NOT a workflow failure — the checker logs a `[WARN]` in the audit report and the workflow proceeds. Only an Exit 2 from rhino-cli itself (broken binary, missing dependency) terminates the workflow with `fail`.

**Success criteria**: Checker completes and generates audit report.

**On failure**: Terminate workflow with status `fail`.

### 2. Check for Findings (Sequential)

Analyze audit report to determine if fixes are needed.

**Condition Check**: Count findings based on mode level in `{step1.outputs.audit-report-1}`

- **lax**: Count CRITICAL only
- **normal**: Count CRITICAL + HIGH
- **strict**: Count CRITICAL + HIGH + MEDIUM
- **ocd**: Count all levels (CRITICAL, HIGH, MEDIUM, LOW)

**Below-threshold findings**: Report but don't block success

- **lax**: HIGH/MEDIUM/LOW reported, not counted
- **normal**: MEDIUM/LOW reported, not counted
- **strict**: LOW reported, not counted
- **ocd**: All findings counted

Deterministic findings (those from the rhino-cli preflight) are reported in the `## Deterministic
Findings (rhino-cli preflight)` section of the audit but do NOT count toward the mode threshold.
Only AI-only findings count toward the mode threshold above.

**Decision**:

- If threshold-level findings > 0: Proceed to step 3 (reset `consecutive_zero_count` to 0)
- If threshold-level findings = 0: Initialize `consecutive_zero_count` to 1 (this check is the
  first zero), proceed to step 4 for confirmation re-check (consecutive pass requirement)

**Depends on**: Step 1 completion

**Notes**:

- Fix scope determined by mode level
- Below-threshold findings remain visible in audit reports
- Enables progressive quality improvement

### 3. Apply Fixes (Sequential, Conditional)

Apply validated fixes from the audit report based on mode level.

**Agent**: `repo-rules-fixer`

- **Args**: `report: {step1.outputs.audit-report-1}, approved: all, mode: {input.mode}, EXECUTION_SCOPE: repo-rules`
- **Output**: `{fixes-applied}` - Fix report with same UUID chain as source audit
- **Condition**: Threshold-level findings exist from step 2
- **Depends on**: Step 2 completion

**Success criteria**: Fixer successfully applies all threshold-level fixes without errors.

**On failure**: Log errors, proceed to step 4 for verification.

**Notes**:

- Fixer re-validates findings before applying (prevents false positives)
- **Fix scope based on mode**:
  - **lax**: Fix CRITICAL only (skip HIGH/MEDIUM/LOW)
  - **normal**: Fix CRITICAL + HIGH (skip MEDIUM/LOW)
  - **strict**: Fix CRITICAL + HIGH + MEDIUM (skip LOW)
  - **ocd**: Fix all levels (CRITICAL, HIGH, MEDIUM, LOW)
- Below-threshold findings remain untouched

### 4. Re-validate (Sequential)

Re-run the deterministic preflight (Step 0.5) first, then invoke the AI checker. If the preflight
JSON SHA-256 is unchanged from the prior iteration, the checker reuses the deterministic findings
section unchanged and only re-evaluates AI-only categories.

**Preflight re-run**:

```bash
mkdir -p generated-reports
./apps/rhino-cli/dist/rhino-cli repo-governance audit -o json > generated-reports/repo-governance-audit__{uuid}__{timestamp}.json
```

The binary must be built first via `npx nx run rhino-cli:build`; the prebuilt path is
`apps/rhino-cli/dist/rhino-cli`.

**Agent**: `repo-rules-checker`

- **Args**: `scope: all, preflight-report: {step4.preflight.outputs.preflight-report}`
- **Output**: `{audit-report-N}` - Verification audit report
- **Depends on**: Step 3 completion

**Note on preflight unavailability**: If the `preflight-report` argument is missing, the file does not exist, or the JSON fails schema validation, the AI checker falls back to full Steps 1-8 evaluation per its own Step 0.5 graceful-degradation rule (`.claude/agents/repo-rules-checker.md`). This is NOT a workflow failure — the checker logs a `[WARN]` in the audit report and the workflow proceeds. Only an Exit 2 from rhino-cli itself (broken binary, missing dependency) terminates the workflow with `fail`.

**Success criteria**: Checker completes validation.

**On failure**: Terminate workflow with status `fail`.

### 5. Iteration Control (Sequential)

Determine whether to continue fixing or terminate.

**Logic**:

- Count findings based on mode level in {step4.outputs.audit-report-N} (same as Step 2):
  - **lax**: Count CRITICAL only
  - **normal**: Count CRITICAL + HIGH
  - **strict**: Count CRITICAL + HIGH + MEDIUM
  - **ocd**: Count all levels
- Track `consecutive_zero_count` across iterations (resets to 0 when threshold-level findings > 0, increments when = 0)
- If consecutive_zero_count >= 2 AND iterations >= min-iterations (or min not provided): Proceed to step 6 (Success — double-zero confirmed)
- If consecutive_zero_count >= 2 AND iterations < min-iterations: Loop back to step 4 (re-validate)
- If consecutive_zero_count < 2 AND threshold-level findings = 0: Loop back to step 4 (confirmation check — no fix needed, just re-verify)
- If threshold-level findings > 0 AND max-iterations provided AND iterations >= max-iterations: Proceed to step 6 (Partial)
- If threshold-level findings > 0 AND (max-iterations not provided OR iterations < max-iterations): Loop back to step 3

**Below-threshold findings**: Continue to be reported in audit but don't affect iteration logic

Deterministic findings (those from the rhino-cli preflight) are reported in the audit but do NOT
count toward the mode threshold. Only AI-only findings count toward the mode threshold and the
`consecutive_zero_count` above.

**Depends on**: Step 4 completion

**Notes**:

- **Default behavior**: Runs up to 7 iterations (default max-iterations). Override with higher value for more attempts
- **Consecutive pass requirement**: Zero findings must be confirmed by a second independent check before declaring success
- **Optional min-iterations**: Prevents premature termination before sufficient iterations
- Each iteration uses the latest audit report
- Tracks iteration count for observability

### 6. Finalization (Sequential)

Report final status and summary.

**Output**: `{final-status}`, `{iterations-completed}`, `{final-report}`

**Status determination**:

- **Success** (`pass`): Zero findings after validation
- **Partial** (`partial`): Findings remain after max-iterations
- **Failure** (`fail`): Technical errors during check or fix

**Depends on**: Reaching this step from step 2, 4, or 5

## Termination Criteria

**Success** (`pass`):

- **lax**: Zero CRITICAL findings on 2 consecutive checks (HIGH/MEDIUM/LOW may exist)
- **normal**: Zero CRITICAL/HIGH findings on 2 consecutive checks (MEDIUM/LOW may exist)
- **strict**: Zero CRITICAL/HIGH/MEDIUM findings on 2 consecutive checks (LOW may exist)
- **ocd**: Zero findings at all levels on 2 consecutive checks

**Partial** (`partial`):

- Threshold-level findings remain after max-iterations safety limit

**Failure** (`fail`):

- Technical errors during check or fix

**Note on deterministic findings**: Deterministic findings from preflight are reported in the audit's
`## Deterministic Findings (rhino-cli preflight)` section but do NOT count toward any mode threshold.
Two consecutive zero-finding validations refers to AI-only findings only.

**Note**: Below-threshold findings are reported in final audit but don't prevent success status. Success requires two consecutive zero-finding validations (consecutive pass requirement).

## Example Usage

### Standard Invocation (Strict Mode — Default)

```
User: "Run repository rules quality gate workflow"
```

The AI will invoke specialized agents via the Agent tool:

- Validate repository consistency (`repo-rules-checker` subagent)
- Apply fixes for CRITICAL/HIGH/MEDIUM findings (`repo-rules-fixer` subagent)
- Iterate until zero CRITICAL/HIGH/MEDIUM findings achieved
- Report LOW findings without fixing them

### Pre-Release Validation (Strict Mode)

```
User: "Run repository rules quality gate workflow in strict mode"
```

The AI will invoke agents with stricter criteria:

- Fix CRITICAL/HIGH/MEDIUM findings
- Report LOW findings without fixing them
- Iterate until zero CRITICAL/HIGH/MEDIUM findings achieved

### Comprehensive Audit (OCD Mode)

```
User: "Run repository rules quality gate workflow in ocd mode"
```

The AI will invoke agents with zero-tolerance criteria:

- Fix ALL findings (CRITICAL, HIGH, MEDIUM, LOW)
- Iterate until zero findings at all levels
- Equivalent to pre-mode parameter behavior

### With Iteration Bounds

```
User: "Run repository rules quality gate workflow in strict mode with min-iterations=2 and max-iterations=7"
```

The AI will invoke agents with iteration controls:

- Require at least 2 check-fix cycles
- Cap at maximum 7 iterations to prevent infinite loops
- Report final status (pass/partial) after completion

## Iteration Example

Typical execution flow:

```
Iteration 1:
  Check → 15 findings → Fix → Re-check → 8 findings

Iteration 2:
  Check (reuse) → 8 findings → Fix → Re-check → 2 findings

Iteration 3:
  Check (reuse) → 2 findings → Fix → Re-check → 0 findings (consecutive_zero: 1)

Iteration 4 (confirmation):
  Re-check → 0 findings (consecutive_zero: 2 — double-zero confirmed)

Result: SUCCESS (4 iterations)
```

## Safety Features

**Infinite Loop Prevention**:

- max-iterations defaults to 7 (override with higher value for more attempts)
- When provided, workflow terminates with `partial` if limit reached
- Tracks iteration count for monitoring
- Escalation warning at iteration 5 if not converging

**Convergence Safeguards**:

- Checker loads `.known-false-positives.md` skip list at start of each iteration
- Fixer persists new FALSE_POSITIVEs to skip list after each run
- Re-validation uses scoped scan (changed files only) to prevent scope expansion
- Factual claims verified in iteration 1 are cached, not re-verified with WebSearch
- Escalation after repeated checker-fixer disagreements on the same finding

**False Positive Protection**:

- Fixer re-validates each finding before applying
- Skips FALSE_POSITIVE findings automatically
- Progressive writing ensures audit history survives

**Error Recovery**:

- Continues to verification even if some fixes fail
- Reports which fixes succeeded/failed
- Generates final report regardless of status

## Related Workflows

This workflow can be composed with:

- Deployment workflows (validate before deploy)
- Release workflows (audit before version bump)
- Content creation workflows (validate after bulk changes)

## Success Metrics

Track across executions:

- **Average iterations to completion**: How many cycles typically needed
- **Success rate**: Percentage reaching zero findings
- **Common finding categories**: What issues appear most often
- **Fix success rate**: Percentage of fixes applied without errors

## Notes

- **Fully automated**: No human checkpoints, runs to completion
- **Idempotent**: Safe to run multiple times, won't break working state
- **Conservative**: Fixer skips uncertain changes (preserves correctness)
- **Observable**: Generates audit reports for every iteration
- **Bounded**: Max-iterations prevents runaway execution

**Concurrency**: Currently validates and fixes sequentially. The `max-concurrency` parameter is reserved for future enhancements where multiple validation dimensions (principles, conventions, development, agents source in repo-governance/agents/) could run concurrently.

**Note**: "agents" in this context refers to agent SOURCE definitions in `.claude/agents/` (primary) - note: `.opencode/agents/` is auto-generated.

This workflow ensures repository consistency through iterative validation and fixing, making it ideal for maintenance and quality assurance.

## Principles Implemented/Respected

- **Explicit Over Implicit**: All steps, conditions, and termination criteria are explicit
- **Automation Over Manual**: Fully automated validation and fixing without human intervention
- **Simplicity Over Complexity**: Clear linear flow with loop control
- **Accessibility First**: Generates human-readable audit reports
- **Progressive Disclosure**: Can run with different iteration limits
- **No Time Estimates**: Focus on quality outcomes, not duration

## Conventions Implemented/Respected

- **[Deterministic vs AI Validation Split Convention](../../conventions/structure/deterministic-vs-ai-validation-split.md)**: Step 0.5 preflight enforces the deterministic-vs-AI category split and the JSON envelope contract this workflow consumes; Step 0.5 implements the deterministic validation tier; AI checker (Steps 1-8) implements the AI-only tier
- **[File Naming Convention](../../conventions/structure/file-naming.md)**: Workflow file follows plain name convention for workflows
- **[Linking Convention](../../conventions/formatting/linking.md)**: All cross-references use GitHub-compatible markdown with `.md` extensions
- **[Content Quality Principles](../../conventions/writing/quality.md)**: Active voice, proper heading hierarchy, single H1
