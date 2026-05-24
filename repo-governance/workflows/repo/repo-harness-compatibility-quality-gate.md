---
name: repo-harness-compatibility-quality-gate
goal: "Detect external drift between each supported coding-agent harness's current upstream conventions and the repository's platform-bindings catalog plus committed binding files, then apply fixes iteratively until zero findings achieved"
termination: "Zero drift findings on two consecutive validations (max-iterations defaults to 7, escalation warning at 5)"
inputs:
  - name: scope
    type: string
    description: 'Subset of harnesses to validate (e.g., "all", or a harness identifier). Defaults to all supported harnesses.'
    required: false
    default: all
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
    description: Maximum number of agents/tasks that can run concurrently during workflow execution
    required: false
    default: 2
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
    pattern: generated-reports/harness-compat__*__*__audit.md
    description: Final audit report (4-part format with UUID chain)
---

# Harness Compatibility Quality Gate Workflow

**Purpose**: Detect external drift between each supported coding-agent harness's current
upstream configuration conventions and the repository's platform-bindings catalog
(`docs/reference/platform-bindings.md`) and committed binding files, then apply validated
fixes iteratively until zero findings remain.

**Distinction from the deterministic pre-push guard**: The `rhino-cli agents validate-bindings`
command (run via `npm run validate:harness-bindings` in the pre-push hook) is a
**deterministic, agent-free** guard that detects **internal byte-drift** — it re-derives
expected binding file content from `AGENTS.md` and asserts byte-equality with the committed
files. This workflow is the **agent-backed, web-research-backed counterpart** that detects
**external convention drift** — changes in each harness's upstream documentation that have
not yet been reflected in the catalog or committed binding files. The two guards complement
each other: the pre-push guard is fast and offline; this workflow is comprehensive and
current.

**When to use**:

- On a scheduled cadence (e.g., monthly) to catch upstream harness convention changes
- After a supported harness ships a major release that may have changed its instruction-file model
- Before adding a new harness to the platform-bindings catalog
- When the catalog entry for a harness has not been re-verified within the last review period

## Execution Mode

**Preferred Mode**: Agent Delegation — invoke `repo-harness-compatibility-checker` and
`repo-harness-compatibility-fixer` via the Agent tool with `subagent_type`
(see [Workflow Execution Modes Convention](../meta/execution-modes.md)).

**Fallback Mode**: Manual Orchestration — execute workflow logic directly using
Read/Write/Edit tools when Agent Delegation is unavailable.

**How to Execute**:

```
User: "Run harness compatibility quality gate workflow"
```

The orchestrator will:

1. Invoke `repo-harness-compatibility-checker` via the Agent tool (fetches current harness
   conventions via `web-research-maker`, diffs against catalog and committed binding files,
   writes drift audit report to `generated-reports/`)
2. Invoke `repo-harness-compatibility-fixer` via the Agent tool (reads audit, applies
   validated catalog and binding-file updates)
3. Iterate until zero drift findings achieved on two consecutive validations
4. Show git status with modified files
5. Wait for user commit approval

**Fallback (Manual Mode)**:

```
User: "Run harness compatibility quality gate workflow in manual mode"
```

The orchestrator executes the checker and fixer logic directly using Read/Write/Edit tools
in the main context — use this when agent delegation is unavailable.

## Steps

### 1. Initial Validation (Sequential)

Run a harness-by-harness drift check to identify all external convention changes that have
not yet been reflected in the repository.

**Agent**: `repo-harness-compatibility-checker`

- **Args**: `scope: {input.scope}, mode: {input.mode}, EXECUTION_SCOPE: harness-compat`
- **Output**: `{audit-report-1}` — Initial drift audit in `generated-reports/` (4-part
  format: `harness-compat__{uuid-chain}__{timestamp}__audit.md`), citing web sources for
  every finding

**How the checker operates**:

- Reads the current platform-bindings catalog
  (`docs/reference/platform-bindings.md`) and committed binding files for each harness
  listed in scope
- Delegates multi-page web research to `web-research-maker` to fetch the current upstream
  conventions for each supported harness (instruction-file model, native `AGENTS.md`
  support, higher-precedence filename forms, binding directory paths, MCP config paths,
  custom-agent surfaces, skills surfaces, and tier classification)
- Diffs each harness's fetched conventions against the catalog row and any committed
  binding files
- Writes findings as a drift audit report with severity, evidence, and web citations

**UUID Chain Tracking**: Checker generates a 6-char UUID and writes to
`generated-reports/.execution-chain-harness-compat` before spawning `web-research-maker`
tasks. See the Temporary Files Convention for details.

**Success criteria**: Checker completes and generates audit report.

**On failure**: Terminate workflow with status `fail`.

### 2. Check for Findings (Sequential)

Analyze the audit report to determine if fixes are needed.

**Condition check**: Count findings in `{audit-report-1}` based on the configured `mode`:

- `lax` — count CRITICAL only
- `normal` — count CRITICAL + HIGH
- `strict` (default) — count CRITICAL + HIGH + MEDIUM
- `ocd` — count CRITICAL + HIGH + MEDIUM + LOW

**Below-threshold findings**: Report but do not block success:

- `lax`: HIGH/MEDIUM/LOW reported, not counted
- `normal`: MEDIUM/LOW reported, not counted
- `strict`: LOW reported, not counted
- `ocd`: all findings counted

**Decision**:

- If threshold-level findings > 0: Proceed to step 3 (reset `consecutive_zero_count` to 0)
- If threshold-level findings = 0: Initialize `consecutive_zero_count` to 1, proceed to
  step 4 for confirmation re-check (consecutive pass requirement)

**Depends on**: Step 1 completion

### 3. Apply Fixes (Sequential, Conditional)

Apply validated drift fixes from the audit report based on mode level.

**Agent**: `repo-harness-compatibility-fixer`

- **Args**: `report: {audit-report-N}, approved: all, mode: {input.mode}, EXECUTION_SCOPE: harness-compat`
- **Output**: `{fix-report-N}` — Fix report with the same UUID chain as the source audit
- **Condition**: Threshold-level findings exist from step 2
- **Depends on**: Step 2 completion

**Auto-fixable scope** (fixer applies these automatically at HIGH confidence):

- Catalog field updates where web-research evidence is unambiguous (e.g., a harness ships
  native `AGENTS.md` support and the catalog still marks it Tier 2)
- Tier reclassification (Tier 2 → Tier 1) backed by a dated, cited web source
- Stale verification dates in the catalog (bumps to current date when content unchanged)
- Generated binding file updates where the content change is derivable from updated catalog
  facts and `rhino-cli agents emit-bindings` output matches

**Out-of-scope for automated fixing** (fixer flags and surfaces for human resolution):

- Tier 1 → Tier 2 reclassification (requires authoring a new generated bridge and updating
  the pre-push guard corpus)
- Higher-precedence filename discoveries (AD3 implications require human judgment per the
  [Multi-Harness Binding Convention](../../conventions/structure/multi-harness-binding.md))
- New harness additions (full onboarding involves catalog row, binding directory decision,
  and dual-CLI implementation per AD8)
- Evidence that conflicts across sources (checker must escalate to human with both sources)

**On out-of-scope findings**: Surface with full context in the orchestrator's user-visible
status; do not loop further until the human resolves.

**Success criteria**: Fixer applies all in-scope fixes without errors; out-of-scope findings
are surfaced clearly.

**On failure**: Log errors, proceed to step 4 for verification.

### 4. Re-Validate (Sequential)

Run the checker again to confirm fixes resolved drift and no new drift was introduced.

**Agent**: `repo-harness-compatibility-checker`

- **Args**: `scope: {input.scope}, mode: {input.mode}, EXECUTION_SCOPE: harness-compat`
- **Output**: `{audit-report-N+1}` — Verification audit report (continues the UUID chain from the prior iteration)

**Success criteria**: Checker completes validation.

**On failure**: Terminate workflow with status `fail`.

### 5. Iteration Control (Sequential)

Determine whether to continue fixing or terminate.

**Logic**:

- Count findings based on mode level in `{audit-report-N+1}` (same as step 2)
- Track `consecutive_zero_count` across iterations (resets to 0 when threshold-level
  findings > 0, increments when = 0)
- If `consecutive_zero_count >= 2` AND `iterations >= min-iterations` (or min not provided):
  Proceed to step 6 (Success — double-zero confirmed)
- If `consecutive_zero_count >= 2` AND `iterations < min-iterations`: Loop back to step 4
  (re-validate)
- If `consecutive_zero_count < 2` AND threshold-level findings = 0: Loop back to step 4
  (confirmation check — no fix needed, just re-verify)
- If threshold-level findings > 0 AND `max-iterations` provided AND
  `iterations >= max-iterations`: Proceed to step 6 (Partial)
- If threshold-level findings > 0 AND (`max-iterations` not provided OR
  `iterations < max-iterations`): Loop back to step 3
- At iteration 5: emit an escalation warning if not converging

**Below-threshold findings**: Continue to be reported in audit but do not affect iteration
logic.

**Depends on**: Step 4 completion

### 6. Finalization (Sequential)

Report final status and summary.

**Output**: `{final-status}`, `{iterations-completed}`, `{final-report}`

**Status determination**:

- **Success** (`pass`): Zero threshold-level drift findings after double-zero confirmation
- **Partial** (`partial`): Drift findings remain after max-iterations, or out-of-scope
  findings require human resolution
- **Failure** (`fail`): Technical errors during check or fix

**Depends on**: Reaching this step from step 4 (confirmation re-check) or step 5 (iteration control)

## Termination Criteria

**Success** (`pass`):

- `lax`: Zero CRITICAL findings on 2 consecutive checks (HIGH/MEDIUM/LOW drift may exist)
- `normal`: Zero CRITICAL/HIGH findings on 2 consecutive checks (MEDIUM/LOW may exist)
- `strict`: Zero CRITICAL/HIGH/MEDIUM findings on 2 consecutive checks (LOW may exist)
- `ocd`: Zero findings at all levels on 2 consecutive checks

**Partial** (`partial`):

- Threshold-level drift findings remain after max-iterations, or out-of-scope findings
  surfaced and awaiting human resolution

**Failure** (`fail`):

- Technical errors during check or fix

**Note**: Below-threshold findings are reported in the final audit but do not prevent
success status. Success always requires two consecutive zero-finding validations (the
consecutive pass requirement).

## Success Criteria (Acceptance)

The following scenarios define what a correct workflow execution looks like:

```gherkin
Scenario: Checker delegates web research and produces a cited drift audit
  Given the workflow runs with scope "all"
  When repo-harness-compatibility-checker is invoked
  Then it delegates multi-page upstream research to web-research-maker
  And it fetches the current instruction-file model, tier, and binding paths for each supported harness
  And it diffs the fetched data against docs/reference/platform-bindings.md and committed binding files
  And it writes a drift audit to generated-reports/ citing the web sources for each finding
  And each finding identifies the affected harness, the stale field, and the upstream source URL

Scenario: Fixer updates catalog entries for unambiguous in-scope drift
  Given the audit contains a HIGH-confidence finding that a harness now reads AGENTS.md natively
  And the current catalog marks that harness as Tier 2
  When repo-harness-compatibility-fixer is invoked
  Then it updates the harness row in docs/reference/platform-bindings.md to Tier 1
  And it records the web citation and verification date in the catalog entry
  And it writes a fix report using the same UUID chain as the audit

Scenario: Out-of-scope findings escalate to human without looping
  Given the audit contains a finding that a harness introduced a new higher-precedence filename
  When repo-harness-compatibility-fixer encounters this finding
  Then it flags it as out-of-scope with a human-action annotation
  And the workflow terminates with status "partial" rather than looping further
  And the user-visible output surfaces the finding with full context

Scenario: Double-zero confirmation prevents premature success
  Given the first validation pass returns zero drift findings
  When the workflow reaches iteration control
  Then it increments consecutive_zero_count to 1 and loops to re-validate
  And only after a second consecutive zero-finding validation does it terminate with "pass"

Scenario: Scheduled execution stays within bounded iteration budget
  Given max-iterations is set to 7 (default)
  When drift findings persist through all 7 iterations
  Then the workflow terminates with status "partial"
  And the final audit report lists all remaining drift findings
  And an escalation warning was emitted at iteration 5
```

## Example Usage

### Standard Invocation (Strict Mode — Default)

```
User: "Run harness compatibility quality gate workflow"
```

The orchestrator invokes specialized agents:

- `repo-harness-compatibility-checker` fetches current upstream conventions for all
  supported harnesses and diffs against the catalog and committed binding files
- `repo-harness-compatibility-fixer` applies in-scope catalog updates (CRITICAL/HIGH/MEDIUM)
- Iterates until zero CRITICAL/HIGH/MEDIUM drift findings achieved on two consecutive checks
- Reports LOW-severity drift without fixing it

### Single Harness Scope

```
User: "Run harness compatibility quality gate workflow with scope=codex-cli"
```

Scopes the check to a single harness — useful when that harness ships a major release.

### With Iteration Bounds

```
User: "Run harness compatibility quality gate workflow in strict mode with min-iterations=2 and max-iterations=5"
```

Requires at least 2 check-fix cycles and caps at 5 iterations.

## Iteration Example

Typical execution flow:

```
Iteration 1:
  Check → 3 drift findings (2 tier updates, 1 stale date)
  Fix   → Apply 2 tier updates, bump stale date
  Re-check → 0 findings (consecutive_zero: 1)

Iteration 2 (confirmation):
  Re-check → 0 findings (consecutive_zero: 2 — double-zero confirmed)

Result: SUCCESS (2 iterations)
```

Typical flow when out-of-scope findings are present:

```
Iteration 1:
  Check → 1 finding (new higher-precedence filename discovered for a harness)
  Fix   → Flags as out-of-scope: human action required
Result: PARTIAL after 1 iteration; user must resolve before re-running.
```

## Safety Features

**Infinite Loop Prevention**:

- `max-iterations` defaults to 7 — override with a higher value for more attempts
- Workflow terminates with `partial` if the limit is reached
- Tracks iteration count for observability
- Escalation warning at iteration 5 if not converging

**False Positive Protection**:

- Fixer re-validates each finding before applying (prevents stale-audit edits)
- Skips FALSE_POSITIVE findings automatically
- Progressive writing ensures audit history survives fixer runs

**Web Research Integrity**:

- Checker cites the exact source URL and fetch date for every finding
- Out-of-scope findings surface both the checker's evidence and the catalog's current
  entry so the human reviewer can cross-check without re-doing the research

**Error Recovery**:

- Continues to verification even if some fixes fail
- Reports which fixes succeeded/failed
- Generates final report regardless of status

## Related Conventions

- [Multi-Harness Binding Convention](../../conventions/structure/multi-harness-binding.md) —
  Defines the two-tier classification, no-shadowing rule, and mechanical-generation
  requirement that this workflow audits for external drift
- [Platform Bindings Catalog](../../../docs/reference/platform-bindings.md) — The canonical
  catalog that this workflow reads and updates
- [Governance Vendor-Independence Convention](../../conventions/structure/governance-vendor-independence.md) —
  Vendor terms in workflow prose are restricted to the `## Platform Binding Examples`
  section; this file respects that rule

## Related Workflows

This workflow composes with:

- [Repository Rules Validation Workflow](./repo-rules-quality-gate.md) — Run after this
  workflow when catalog or binding-file updates require governance prose updates
- [Cross-Vendor Parity Quality Gate](./repo-cross-vendor-parity-quality-gate.md) — Run to
  verify that internal binding-file parity is maintained after catalog updates

## Notes

- **External focus**: This workflow detects drift in upstream harness documentation; it does
  not validate internal byte-level binding file consistency — use the deterministic
  `rhino-cli agents validate-bindings` pre-push guard for that
- **Fully automated** (in-scope fixes): No human checkpoints for catalog field updates and
  tier reclassifications backed by clear evidence; out-of-scope findings pause for human
  resolution
- **Idempotent**: Safe to run multiple times; each run produces a fresh audit and applies
  only findings confirmed by the current checker pass
- **Bounded**: `max-iterations` prevents runaway execution
- **Observable**: Generates audit reports with web citations for every iteration

## Principles Implemented/Respected

- **Explicit Over Implicit**: All steps, conditions, and termination criteria are explicit;
  the distinction between external convention drift (this workflow) and internal byte-drift
  (pre-push guard) is stated directly
- **Automation Over Manual**: Fully automated validation and fixing for in-scope findings;
  out-of-scope findings surface with enough context for efficient human resolution
- **Simplicity Over Complexity**: Clear linear flow with loop control mirrors the sibling
  quality-gate workflows
- **Accessibility First**: Generates human-readable audit reports with cited web sources
- **No Time Estimates**: Focus on quality outcomes, not duration

## Conventions Implemented/Respected

- **[Workflow Naming Convention](../../conventions/structure/workflow-naming.md)**: Filename
  `repo-harness-compatibility-quality-gate` follows the `<scope>-<qualifier>-<type>` rule
  with scope `repo`, qualifier `harness-compatibility`, and type `quality-gate`
- **[File Naming Convention](../../conventions/structure/file-naming.md)**: Workflow file
  uses plain kebab-case in its parent subdirectory
- **[Linking Convention](../../conventions/formatting/linking.md)**: All cross-references
  use GitHub-compatible markdown with `.md` extensions and relative paths
- **[Content Quality Principles](../../conventions/writing/quality.md)**: Active voice,
  proper heading hierarchy, single H1
- **[Governance Vendor-Independence Convention](../../conventions/structure/governance-vendor-independence.md)**:
  Vendor product names are absent from load-bearing prose; harnesses are referred to as
  "each supported harness" or "the coding agent" in non-example sections

## Platform Binding Examples

This section names concrete harness products and is excluded from the vendor-audit scan per
the Governance Vendor-Independence Convention.

The supported harnesses validated by this workflow are the same harnesses listed in the
[Platform Bindings Catalog](../../../docs/reference/platform-bindings.md): Claude Code,
OpenCode, OpenAI Codex CLI, GitHub Copilot, Cursor, Windsurf, JetBrains Junie, Amazon Q
Developer, Google Antigravity CLI, Pi (pi.dev), and Aider (Reserved).
