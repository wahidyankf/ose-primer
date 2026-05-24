# Business Requirements — Multi-Harness Compatibility

## Business Goal

Make `ose-primer` a repository that any contributor can work in productively **regardless of which AI
coding agent they use**, while keeping the governance layer free of vendor lock-in. `ose-primer` is the
MIT-licensed template every downstream fork starts from, so its harness-agnostic posture propagates
downstream to every team that adopts the scaffolding. It is itself downstream of `ose-public`, from
which this multi-harness capability flows. [Repo-grounded —
`repo-governance/conventions/structure/repository-ecosystem.md`]

## Why This Matters

- **Contributor reach.** The coding-agent market is fragmenting fast (Gemini CLI was sunset into
  Antigravity CLI in 2026; new entrants like Pi appeared). [Web-cited — Google Developers Blog,
  "Transitioning Gemini CLI to Antigravity CLI",
  https://developers.googleblog.com/an-important-update-transitioning-gemini-cli-to-antigravity-cli/,
  accessed 2026-05-24; excerpt: "we can serve you best by pouring our energy into a single product built
  for today's multi-agent reality"] A template that hard-codes one tool's conventions silently excludes
  contributors on every other tool — and every fork inherits that exclusion.
- **Governance durability.** Vendor names and product lifecycles change. Keeping `repo-governance/`
  neutral means a vendor rename or shutdown never invalidates a governance rule — only a thin binding
  file needs updating. This is already the repo's stated convention; this plan closes the gaps the
  convention does not yet enforce (Junie, Amazon Q, Antigravity, Pi are absent from the audit vocabulary
  today). [Repo-grounded — `repo-governance/conventions/structure/governance-vendor-independence.md`
  forbidden-terms tables]
- **Drift is silent and expensive.** A harness can rename its instruction file or change its MCP config
  path in a point release. Without an automated re-verification loop, the binding catalog rots and
  contributors hit confusing failures. A research-backed compatibility audit turns silent rot into a
  tracked finding.
- **Single-source maintenance.** Because most harnesses read `AGENTS.md` natively, one canonical file
  plus generated thin pointers means near-zero duplicated instruction content to maintain.
- **Parity-safe tooling.** `ose-primer` keeps two co-equal CLI implementations (Rust + Go). Generating
  bindings mechanically from one source (`AGENTS.md`) — in both implementations, held byte-identical by
  the shadow-diff gate — means the harness scaffolding cannot drift either between bindings or between
  the two CLIs. [Repo-grounded — `plans/done/2026-05-24__have-two-rhino-versions/`]

## Affected Roles

- **Repository maintainer** — owns the vendor-audit, the binding files, and the compatibility workflow;
  benefits from mechanical enforcement instead of manual catalog upkeep.
- **External contributors / template adopters** — fork `ose-primer`, use whichever harness they prefer,
  and get the same governance instructions without manual setup.
- **AI coding agents (all nine)** — receive consistent, non-conflicting instructions from a single
  canonical surface, avoiding the shadowing hazards where a tool-specific file silently outranks
  `AGENTS.md`.

## Business-Level Success Metrics

- All nine named harnesses have a documented, verified binding entry in the platform-bindings catalog
  with an explicit "AGENTS.md native?" status. (Observable: catalog table rows = 9 named + OpenCode,
  plus the reserved Aider row.)
- The vendor-audit fails when a new vendor name leaks into governance prose for any of the nine harnesses,
  in **both** CLI implementations. (Observable: a deliberately-seeded test string is caught by
  `rhino-cli repo-governance vendor-audit`, and the shadow-diff parity gate stays green.)
- The compatibility-audit workflow can be run on demand and produces a drift report citing current
  upstream docs. (Observable: workflow run emits a `generated-reports/` audit referencing web sources.)
- `repo-rules-quality-gate` reaches its double-zero termination on the changed governance files.
  [Repo-grounded — `repo-governance/workflows/repo/repo-rules-quality-gate.md` termination criteria]

## Business-Scope Non-Goals

- This is not a marketing or onboarding-docs initiative; it does not produce per-tool "how to install X"
  tutorials beyond the catalog reference.
- It does not commit the project to _feature parity_ across harnesses (e.g., replicating Claude Code
  skills for tools that lack a skills concept) — only to _instruction compatibility_.

## Business Risks

- **Research staleness.** These tools change monthly; some findings carry `[Needs Verification]` flags.
  The compatibility-audit workflow is the mitigation — it re-checks rather than trusting a one-time
  snapshot.
- **Over-binding.** Creating tool-specific files that _outrank_ `AGENTS.md` (e.g., `GEMINI.md`,
  `.junie/AGENTS.md`) could silently shadow canonical instructions. Mitigated by the no-shadowing rule
  (see `tech-docs.md`).
- **False-positive fatigue.** Adding ambiguous short tokens (`Q`, `pi`, `agy`) to the vendor-audit could
  flood the audit with false positives and erode trust. Mitigated by qualified patterns and documented FP
  notes (see `tech-docs.md` §Vendor-Audit Extension).
- **Cross-implementation divergence.** A vendor-audit or emitter change applied to only one CLI would
  break the byte-parity gate and block every push. Mitigated by pairing every Rust change with the
  identical Go change in the same delivery phase and verifying via `shadow-diff.sh` before push.
  [Repo-grounded — `apps/rhino-cli-rust/scripts/shadow-diff.sh`]
