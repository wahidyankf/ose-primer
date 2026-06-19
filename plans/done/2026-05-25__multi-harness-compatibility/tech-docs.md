# Technical Documentation — Multi-Harness Compatibility

All harness-config facts below were established by `web-researcher` runs on **2026-05-24** (during
the upstream `ose-public` adoption) and are labelled `[Web-cited]` with their source. They describe
external-harness properties that do not change by repository, so they transfer to `ose-primer` unchanged;
the new compatibility-audit workflow re-verifies them over time. Repository facts are labelled
`[Repo-grounded]` and were re-verified against `ose-primer` on 2026-05-25. Subjective design choices are
labelled `[Judgment call]`. Some upstream facts carry the researcher's `[Needs Verification]` flag — these
are the items the compatibility-audit workflow exists to re-check.

## Harness Compatibility Matrix

The decisive question for each harness is **"does it read the root `AGENTS.md` natively?"** because the
repo already maintains a complete canonical `AGENTS.md`. [Repo-grounded — `AGENTS.md`]

| Harness                    | Reads root `AGENTS.md`?                             | Tool-specific instruction surface                                                                        | Project MCP config                          | Custom-agent surface                                                   | Skills surface                             | Current repo state (`ose-primer`)                                     |
| -------------------------- | --------------------------------------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------- | ---------------------------------------------------------------------- | ------------------------------------------ | --------------------------------------------------------------------- |
| **Claude Code**            | No — reads `CLAUDE.md` (bridged)                    | `CLAUDE.md` (shim `@AGENTS.md`), `.claude/`                                                              | `.mcp.json`                                 | `.claude/agents/*.md`                                                  | `.claude/skills/*/SKILL.md`                | **Active** [Repo-grounded]                                            |
| **OpenCode**               | Yes                                                 | `.opencode/agents/` (auto-synced); reads `.claude/skills/` natively                                      | `opencode.json`                             | `.opencode/agents/*.md`                                                | reads `.claude/skills/`                    | **Active** [Repo-grounded]                                            |
| **OpenAI Codex CLI**       | Yes (since Apr 2025)                                | `AGENTS.override.md` (overrides), `.codex/config.toml`                                                   | `.codex/config.toml` `[mcp_servers]` (TOML) | `[agents.<name>]` in `config.toml`                                     | `.agents/skills/`                          | **Partial** — `.codex/{config.toml,agents/}` exists [Repo-grounded]   |
| **GitHub Copilot**         | Yes (nearest file wins)                             | `.github/copilot-instructions.md`, `.github/instructions/*.instructions.md`                              | `.vscode/mcp.json` (VS Code)                | `.github/agents/*.agent.md` (also reads `.claude/agents/`)             | n/a                                        | **Partial** — `.github/{agents,prompts,skills}` exist [Repo-grounded] |
| **Cursor**                 | Yes                                                 | `.cursor/rules/*.mdc` (+ legacy `.cursorrules`)                                                          | `.cursor/mcp.json`                          | `.cursor/agents/*.md` (also reads `.claude/agents/`, `.codex/agents/`) | `.cursor/skills/`                          | **Absent** [Repo-grounded]                                            |
| **Windsurf**               | Yes                                                 | `.windsurf/rules/*.md` (+ legacy `.windsurfrules`), `.windsurf/workflows/`                               | global only (no documented project file)    | not officially documented                                              | `.windsurf/skills/` `[Needs Verification]` | **Absent** [Repo-grounded]                                            |
| **JetBrains Junie**        | Yes — `.junie/AGENTS.md` > root `AGENTS.md`         | `.junie/AGENTS.md`, `.junie/rules/*.md` (imports `.claude/agents/`, `.codex/agents/`, `.claude/skills/`) | `.junie/mcp/mcp.json`                       | `.junie/agents/`, `.agents/`                                           | `.junie/skills/<name>/SKILL.md`            | **Absent** [Repo-grounded]                                            |
| **Amazon Q Developer**     | **No** (open feature request #2712)                 | `.amazonq/rules/*.md` (consumed via agent JSON `resources`)                                              | `.amazonq/mcp.json`                         | JSON in `.amazonq/` (local) / `~/.aws/amazonq/cli-agents/`             | none                                       | **Absent** [Repo-grounded]                                            |
| **Google Antigravity CLI** | Yes (since v1.20.3) — **`GEMINI.md` > `AGENTS.md`** | `GEMINI.md` (overrides), `.agent/rules/*.md`                                                             | `mcp_config.json` (root or `.agents/`)      | runtime-orchestrated (no declarative file)                             | `.agents/skills/<name>/SKILL.md`           | **Absent** [Repo-grounded]                                            |
| **Pi (pi.dev)**            | Yes (also reads `CLAUDE.md`)                        | `.pi/settings.json`, `.pi/AGENTS.md`, `.pi/SYSTEM.md`                                                    | none (intentionally no native MCP)          | none built-in (extension-based)                                        | `.agents/skills/` or `.pi/skills/`         | **Absent** [Repo-grounded]                                            |

**Per-tool citations** `[Web-cited]`, accessed 2026-05-24:

- Copilot — [docs.github.com/copilot custom instructions](https://docs.github.com/en/copilot/how-tos/configure-custom-instructions-in-your-ide/add-repository-instructions-in-your-ide),
  [custom-instructions-support reference](https://docs.github.com/en/copilot/reference/custom-instructions-support);
  excerpt: GitHub Copilot reads `AGENTS.md` natively and also reads the nearest `.github/copilot-instructions.md`
  for per-repo custom instructions.
- Cursor — [cursor.com/docs/context/rules](https://cursor.com/docs/context/rules),
  [cursor.com/docs/subagents](https://cursor.com/docs/subagents);
  excerpt: Cursor reads `AGENTS.md` from the project root and applies `.cursor/rules/*.mdc` files as
  additional context rules.
- Windsurf — [docs.windsurf.com AGENTS.md](https://docs.windsurf.com/windsurf/cascade/agents-md),
  [workflows](https://docs.windsurf.com/windsurf/cascade/workflows);
  excerpt: Windsurf Cascade reads `AGENTS.md` natively and also supports `.windsurf/rules/*.md` for
  project-specific instructions.
- Junie — [junie.jetbrains.com guidelines-and-memory](https://junie.jetbrains.com/docs/guidelines-and-memory.html),
  [agent-skills](https://junie.jetbrains.com/docs/agent-skills.html);
  excerpt: Junie reads `.junie/AGENTS.md` first (takes precedence over root `AGENTS.md`) and also imports
  `.claude/agents/` and `.codex/agents/` definitions.
- Amazon Q — [docs.aws.amazon.com command-line-project-rules](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-project-rules.html),
  [feature request #2712](https://github.com/aws/amazon-q-developer-cli/issues/2712);
  excerpt: Amazon Q Developer CLI reads project rules from `.amazonq/rules/*.md` files via the agent JSON
  `resources` field; native `AGENTS.md` support is an open feature request (#2712, not yet shipped).
- Claude Code — [code.claude.com/docs/en/memory](https://code.claude.com/docs/en/memory);
  excerpt: Claude Code reads `CLAUDE.md` as its primary instruction file; this repo bridges it to `AGENTS.md`
  via a one-line `@AGENTS.md` import shim.
- Codex CLI — [developers.openai.com/codex/guides/agents-md](https://developers.openai.com/codex/guides/agents-md),
  [codex/skills](https://developers.openai.com/codex/skills);
  excerpt: OpenAI Codex CLI reads `AGENTS.md` natively (since April 2025) and applies `AGENTS.override.md`
  as a higher-precedence override when present.
- Antigravity CLI — [Google Developers Blog transition post](https://developers.googleblog.com/an-important-update-transitioning-gemini-cli-to-antigravity-cli/);
  excerpt: Google transitioned Gemini CLI to Antigravity CLI — "we can serve you best by pouring our energy
  into a single product built for today's multi-agent reality"; file-path details from third-party guides
  `[Needs Verification]` (official `antigravity.google/docs` was unreachable during research).
- Pi — [pi.dev](https://pi.dev/), [pi.dev/docs/latest/skills](https://pi.dev/docs/latest/skills),
  [github.com/earendil-works/pi](https://github.com/earendil-works/pi);
  excerpt: Pi reads `AGENTS.md` natively (also reads `CLAUDE.md`) and supports `.pi/AGENTS.md` and
  `.pi/settings.json` for project-specific configuration.

## Architecture Decisions

### AD1 — `AGENTS.md` stays the single canonical surface; bindings are thin

`[Judgment call]` Seven of nine harnesses read root `AGENTS.md` natively. The cheapest, lowest-drift design
keeps `AGENTS.md` as the only place instruction _content_ lives, and treats every tool-specific file as
either (a) unnecessary (native read suffices) or (b) a thin pointer that references `AGENTS.md` rather than
copying it. This mirrors the existing `CLAUDE.md` shim pattern (`@AGENTS.md` import). [Repo-grounded —
`CLAUDE.md`]

### AD2 — Two binding tiers

`[Judgment call]`

- **Tier 1 — native `AGENTS.md` readers** (Copilot, Cursor, Windsurf, Junie, Codex CLI, Antigravity, Pi,
  OpenCode): rely on the native read. Add a tool-specific file **only** when it materially improves
  discovery and can be a non-shadowing pointer. Default position: **add nothing** beyond what already
  exists, document the native-read status in the catalog.
- **Tier 2 — non-readers** (Amazon Q; Claude Code already handled): require an explicit committed bridge.
  For Amazon Q, generate `.amazonq/rules/00-agents-md.md` whose body points to `AGENTS.md`, plus a default
  agent JSON whose `resources` field globs `file://AGENTS.md` and `file://.amazonq/rules/**/*.md`.
  [Web-cited — Amazon Q project-rules + agent-format docs, 2026-05-24]

### AD3 — No-shadowing rule (hard)

`[Judgment call]` Three harnesses rank a tool-specific file **above** `AGENTS.md`:

- Codex CLI: `AGENTS.override.md` > `AGENTS.md`. [Web-cited]
- Junie: `.junie/AGENTS.md` > root `AGENTS.md`. [Web-cited]
- Antigravity: `GEMINI.md` > `AGENTS.md`. [Web-cited]

The repository **must not** commit any of these higher-precedence files with content that differs from
`AGENTS.md`, because doing so silently shadows the canonical surface for that tool only. The default is to
**not create them at all** (the native `AGENTS.md` read already applies). If a future need forces one to
exist, it must be a pure pointer/import of `AGENTS.md`. This rule is codified in the new multi-harness
binding convention.

### AD4 — Mechanical generation over hand-maintenance

`[Judgment call]` Binding files that must exist (Amazon Q bridge; any thin pointer) are generated by
`rhino-cli agents emit-bindings` from `AGENTS.md` so they cannot drift. This extends the existing generator
model where `rhino-cli agents sync` produces `.opencode/` from `.claude/`. Because `ose-primer` keeps two
CLI implementations, the emitter is implemented in **both** `apps/rhino-cli-rust/src/internal/agents/` and
`apps/rhino-cli-go/internal/agents/`, and the shadow-diff gate asserts they emit byte-identical output.
[Repo-grounded — `package.json` script `sync:claude-to-opencode`,
`apps/rhino-cli-rust/src/internal/agents/sync.rs`, `apps/rhino-cli-go/internal/agents/sync.go`]

### AD5 — Reconcile pre-existing partial bindings

`[Repo-grounded]` `.github/{agents,prompts,skills}` and `.codex/{config.toml,agents/}` already exist and
appear tool/Nx-provided rather than produced by our sync. The plan documents their ownership in the catalog
and does not clobber them; it only adds what is missing (the Amazon Q bridge) and records their provenance.

### AD6 — Compatibility-audit workflow as a quality gate

`[Judgment call]` Drift detection fits the maker-checker-fixer + `quality-gate` model: a checker fetches
current upstream conventions (via `web-researcher`) and diffs them against the catalog/binding files; a
fixer updates the catalog/bindings; iterate to zero drift. The workflow is named
`repo-harness-compatibility-quality-gate` to satisfy the `<scope>-<qualifier>-<type>` rule (scope `repo`,
qualifier `harness-compatibility`, type `quality-gate`). [Repo-grounded —
`repo-governance/conventions/structure/workflow-naming.md`]

### AD7 — Deterministic pre-push parity guard (no agent)

`[Judgment call]` AD6's workflow catches **external** drift (an upstream tool changing its conventions) and
is agent-backed and web-research-backed — too slow and non-deterministic for a git hook. A second, separate
mechanism catches **internal** drift: a committed binding file (the Amazon Q bridge, any thin pointer)
falling out of sync with its source (`AGENTS.md`) or the catalog missing a binding that exists on disk. This
guard MUST be **deterministic and agent-free** so it can run in `.husky/pre-push` in milliseconds.

Implementation: a new deterministic subcommand — `rhino-cli agents validate-bindings` — analogous to the
existing `rhino-cli agents validate-sync`, which already deterministically verifies `.opencode/` is a no-op
regenerate from `.claude/`. [Repo-grounded — `package.json` `validate:sync` →
`apps/rhino-cli-rust/src/internal/agents/sync_validator.rs`,
`apps/rhino-cli-go/internal/agents/sync_validator.go`] It (a) re-derives each generated binding file from
`AGENTS.md` in memory and asserts byte-equality with the committed file (sync no-op), and (b) asserts every
binding directory present on disk has a row in `docs/reference/platform-bindings.md`. It exits non-zero on
any mismatch. A **new** npm script `validate:harness-bindings` wraps it (against the canonical Rust binary),
modeled on the existing `validate:sync` script `[Repo-grounded — package.json validate:sync]`, and is wired
into `.husky/pre-push` alongside the existing conditional validators (`validate:naming-agents`,
`validate:cross-vendor-parity`) `[Repo-grounded — .husky/pre-push]`. The script and the underlying
`agents validate-bindings` subcommand are both new `[Judgment call]`.

This keeps the two concerns cleanly split: **deterministic guard in pre-push** (internal parity, fast, no
network, no agent) vs. **agent workflow on demand / scheduled** (external convention drift,
web-research-backed).

### AD8 — Dual-implementation byte-parity (ose-primer-specific)

`[Judgment call / Repo-grounded]` Unlike `ose-public` (which archived its Go CLI), `ose-primer` keeps two
co-equal CLI implementations and gates them with a shadow-diff harness that asserts byte-identical stdout,
stderr, and exit codes for every command corpus. CI runs it as the `parity` job.
[Repo-grounded — `apps/rhino-cli-rust/scripts/shadow-diff.sh`, `.github/workflows/pr-quality-gate.yml`,
`plans/done/2026-05-24__have-two-rhino-versions/`]

Consequences for this plan:

- The vendor-audit extension is applied to both `apps/rhino-cli-rust/src/internal/repo_governance/vendor_audit.rs`
  and `apps/rhino-cli-go/cmd/governance_vendor_audit.go`, identically.
- `agents emit-bindings` and `agents validate-bindings` are implemented in both CLIs.
- `apps/rhino-cli-rust/scripts/shadow-diff.sh` gains corpus cases for `emit-bindings --dry-run` (in the
  `agents` corpus) and `validate-bindings` (clean + drifted fixtures), so the parity gate exercises the new
  behavior.
- Each Rust change is paired with the identical Go change in the same delivery phase; `shadow-diff.sh
agents repo-governance` must pass before push.

## Vendor-Audit Extension

The audit is implemented twice and kept byte-identical: in
`apps/rhino-cli-rust/src/internal/repo_governance/vendor_audit.rs` (invoked via the Rust binary, Nx target
`rhino-cli-rust:validate:repo-governance-vendor-audit`) and in
`apps/rhino-cli-go/cmd/governance_vendor_audit.go` (Nx target
`rhino-cli-go:validate:repo-governance-vendor-audit`). [Repo-grounded — both `project.json` files] It scans
`repo-governance/` + `AGENTS.md` + `CLAUDE.md` prose, honoring allowlist regions (code fences,
`binding-example` fences, "Platform Binding Examples" headings, inline code, link URLs, HTML comments,
frontmatter). [Repo-grounded —
`repo-governance/conventions/structure/governance-vendor-independence.md` §Scope of the scanner]

**New vendor terms to add** (coding-agent / harness product names):

| Pattern (proposed)                      | Notes                                                                      |
| --------------------------------------- | -------------------------------------------------------------------------- |
| `\bJunie\b`                             | JetBrains product name                                                     |
| `\bJetBrains\b`                         | Vendor company name                                                        |
| `Amazon Q\b` / `\bAmazon Q Developer\b` | Use the qualified phrase, **never** bare `\bQ\b` (FP-prone single letter)  |
| `\bAntigravity\b`                       | Google product name                                                        |
| `Pi Coding Agent` / `pi\.dev`           | Use qualified forms, **never** bare `\bpi\b` (collides with math constant) |
| `\bEarendil\b`                          | Pi's vendor company                                                        |

**Binding directory paths to add** to the forbidden-paths table (governance prose must not name them in
load-bearing text): `\.junie/`, `\.amazonq/`, `\.pi/`, `\.gemini/`, `\.agent/`, `\.agents/`. Existing entries
already cover `\.cursor/`, `\.windsurf/`, `\.continue/`, `\.clinerules/`. [Repo-grounded — current
forbidden-paths table in `governance-vendor-independence.md`]

**False-positive handling** `[Judgment call]`:

- Do **not** add bare `\bQ\b`, `\bpi\b`, or `\bagy\b` — each collides with common English/math/identifier
  usage. Match only qualified phrases (`Amazon Q`, `pi.dev`, `Pi Coding Agent`).
- `\bGemini\b` is already forbidden as a model family; `GEMINI.md` references inside "Platform Binding
  Examples" sections remain allowlisted. [Repo-grounded]
- Add FP notes to the convention's "False-positive notes" block for each ambiguous term, mirroring the
  existing `Devin`/`Grok`/`Llama` notes. [Repo-grounded]
- The convention file itself and `docs/reference/platform-bindings.md` remain permanently allowlisted.
  [Repo-grounded]

## File Impact

**Governance (Layer 2/3) — authored/updated via `repo-rules-maker`:**

- `repo-governance/conventions/structure/governance-vendor-independence.md` — extend forbidden-terms tables,
  combined audit regex, FP notes, and vocabulary map. _Update_ [Repo-grounded]
- `repo-governance/conventions/structure/multi-harness-binding.md` — **new** convention: the two-tier
  binding strategy, AGENTS.md-canonical rule, no-shadowing rule (AD3), mechanical-generation rule. _New file_
- `repo-governance/workflows/repo/repo-harness-compatibility-quality-gate.md` — **new** workflow. _New file_
- `repo-governance/workflows/repo/README.md` — add the new workflow to the index. _Update_ [Repo-grounded]

**Reference docs:**

- `docs/reference/platform-bindings.md` — expand the catalog to all nine harnesses + OpenCode (+ the reserved
  Aider row) with the new columns (AGENTS.md-native, MCP path, skills path), replace the stale "Gemini CLI"
  row with "Antigravity CLI", document provenance of pre-existing `.github/`/`.codex/` bindings, and the
  no-shadowing note. _Update_ [Repo-grounded]

**Agents (Layer 4) — authored under `.claude/agents/`, synced to `.opencode/`:**

- `.claude/agents/repo-harness-compatibility-checker.md` — **new** checker; delegates to
  `web-researcher`, diffs findings vs catalog/bindings. _New file_
- `.claude/agents/repo-harness-compatibility-fixer.md` — **new** fixer; updates catalog/bindings from a
  validated audit. _New file_
- `.opencode/agents/repo-harness-compatibility-checker.md`, `.opencode/agents/repo-harness-compatibility-fixer.md`
  — generated by `npm run sync:claude-to-opencode`. _Generated_ [Repo-grounded — sync model]
- `.claude/agents/README.md` and `AGENTS.md` agent catalog list — add the two agents. _Update_ [Repo-grounded]

**rhino CLI (both implementations) — implemented by `swe-rust-dev` + `swe-golang-dev`:**

- `apps/rhino-cli-rust/src/internal/repo_governance/vendor_audit.rs` and
  `apps/rhino-cli-go/cmd/governance_vendor_audit.go` — extend term/path patterns + FP guards, identically.
  _Update_ [Repo-grounded]
- Rust: new `apps/rhino-cli-rust/src/internal/agents/bindings.rs` (single-source `expected_bindings()`
  derivation), new `src/commands/agents_emit_bindings.rs` + `src/commands/agents_validate_bindings.rs`, and
  new `EmitBindings` + `ValidateBindings` variants in `src/cli.rs` `AgentsCommands`. _New files / Update_
  [Repo-grounded — `src/cli.rs` `AgentsCommands` enum]
- Go: new `apps/rhino-cli-go/internal/agents/bindings.go` (mirror derivation), new
  `apps/rhino-cli-go/cmd/agents_emit_bindings.go` + `cmd/agents_validate_bindings.go`, registered in
  `cmd/agents.go`. _New files / Update_ [Repo-grounded — `cmd/agents.go`, mirrored module layout]
- `apps/rhino-cli-rust/scripts/shadow-diff.sh` — add `emit-bindings --dry-run` cases to the `agents` corpus
  and `validate-bindings` cases (clean + drifted). _Update_ [Repo-grounded — existing `agents` corpus]

**Pre-push parity guard (deterministic, no agent) — AD7:**

- `package.json` — add `validate:harness-bindings` script wrapping the Rust binary's
  `agents validate-bindings` (modeled on the existing `validate:sync` script). _New script_ [Judgment call;
  model script `validate:sync` is Repo-grounded]
- `apps/rhino-cli-rust/project.json` and `apps/rhino-cli-go/project.json` — add a
  `validate:harness-bindings` Nx target to **both** to preserve the identical-target-set invariant
  (mirroring the shape of the existing `validate:repo-governance-vendor-audit` target). _New target_ [Judgment
  call; model target `validate:repo-governance-vendor-audit` is Repo-grounded in both project.json]
- `.husky/pre-push` — wire `npm run validate:harness-bindings` into the existing conditional validator chain,
  fired when a binding surface changes (`.amazonq/`, `AGENTS.md`, `docs/reference/platform-bindings.md`,
  `.claude/`, `.opencode/`, `.codex/`, `.github/`). _Update_ [Repo-grounded — `.husky/pre-push` conditional
  blocks]

**Binding files (repo root) — generated where possible:**

- `.amazonq/rules/00-agents-md.md` + default agent JSON — **new** Tier-2 bridge (AD2). _New, generated_
- Optional thin pointers (decide during delivery, default = none per AD2): `.github/copilot-instructions.md`,
  `.cursor/rules/000-agents-md.mdc`, `.windsurf/rules/000-agents-md.md`. _New, generated (optional)_

**Specs — authored by `specs-maker`:**

- `specs/apps/rhino/behavior/cli/gherkin/repo-governance/repo-governance-vendor-audit.feature` — extend with
  scenarios for the new vendor terms and FP-safe tokens. _Update_ [Repo-grounded — file exists]
- `specs/apps/rhino/behavior/cli/gherkin/agents/agents-bindings.feature` — **new** feature covering both
  `emit-bindings` and `validate-bindings`. _New file_ [Repo-grounded — `agents/` dir exists]

**Config:**

- `.gitignore` — confirm new binding dirs (`.amazonq/`, any pointers) are tracked, not ignored.
  _Verify/Update_

**Related Markdown sweep (update ALL related `.md` files):**

Beyond the files named above, a closing sweep must update every Markdown doc that references the binding
catalog, the vendor vocabulary, the agent roster, or the workflow index, so nothing references a stale set.
The authoritative list is produced at delivery time by grep (see `delivery.md` Phase 5.5), but the known
targets are:

- `AGENTS.md` — add the two new agents to the agent-family roster (family 6, "Cross-Vendor Parity", or the
  Repository Governance family). `ose-primer`'s `AGENTS.md` has **no** "Platform Bindings Catalog sub-list"
  or "**Future**:" bindings line (those are `ose-public`-specific), so there is no stale binding list to
  trim here — only the agent roster to extend. _Update_ [Repo-grounded — `AGENTS.md` agent-family list]
- `CLAUDE.md` — dual-mode configuration section, only if a new binding affects the documented
  Claude↔OpenCode format-differences (expected: no change). _Verify_ [Repo-grounded]
- `.claude/agents/README.md` — add the two new agents. _Update_ [Repo-grounded]
- `repo-governance/workflows/repo/README.md` and `repo-governance/workflows/README.md` — add the new
  workflow. _Update_ [Repo-grounded]
- `repo-governance/conventions/README.md` and `repo-governance/conventions/structure/README.md` — add the
  new multi-harness-binding convention. _Update_ [Repo-grounded]
- `docs/reference/README.md` — ensure the platform-bindings catalog entry is current. _Verify_ [Repo-grounded]
- `repo-governance/conventions/structure/repository-ecosystem.md` — note that the new bindings propagate
  downstream to forks of `ose-primer`. (`ose-primer` has no `ose-primer-sync.md`; the ecosystem convention
  is the canonical downstream-propagation doc.) _Verify/Update_ [Repo-grounded]

## Dependencies

- Implementation order: vendor-audit + convention first (keeps governance green), then bindings + emitter,
  then the deterministic guard, then specs, then workflow + agents, then propagation + validation.
- Both CLIs change in lockstep; each Rust edit is paired with the identical Go edit and verified via
  `shadow-diff.sh` before push. [Repo-grounded — `apps/rhino-cli-rust/scripts/shadow-diff.sh`]
- `rhino-cli-rust` is Rust (`swe-rust-dev`); `rhino-cli-go` is Go (`swe-golang-dev`). Both must keep
  `nx run rhino-cli-rust:test:quick` and `nx run rhino-cli-go:test:quick` green. [Repo-grounded — both
  `project.json`]
- The compatibility workflow depends on `web-researcher` (exists) and the two new agents.
  [Repo-grounded — `.claude/agents/web-researcher.md`]

## Rollback

Every change is additive and reversible:

- Vendor-audit regex extension: revert the `vendor_audit.rs` + `governance_vendor_audit.go` patterns and the
  convention edit.
- Binding files: delete the generated files; native `AGENTS.md` readers are unaffected.
- Emitter / guard commands: remove the new subcommands from both CLIs, the shadow-diff corpus cases, the
  npm script, the Nx targets, and the `.husky/pre-push` line.
- Workflow + agents: delete the new files and remove their index entries.
- No data migrations, no deployment branches touched (governance/tooling only).
