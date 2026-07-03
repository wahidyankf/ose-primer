# Ideas

Quick ideas and todos that haven't been formalized into plans yet.

When an idea is ready for implementation, create a proper plan folder in `backlog/` and remove it from this list.

## Ideas List

- **Upgrade Rust MSRV to 1.94.1** (fixes CVE-2026-33056 in Cargo tar handling): deferred from
  `update-toolchain-versions` plan because local rustc was 1.94.0. Upgrade when Rust 1.94.1+ is
  available in the developer toolchain (`rustup update stable`).
- **Source-code credential scanning** — evaluate Betterleaks (gitleaks successor, MIT, v1.0.0 early 2026) for pre-commit + CI detection of hard-coded credentials in `.rs`/`.go`/`.ts`/`.tf` source
  files once it reaches stable production use. This public repo already has free GitHub Secret
  Scanning post-push coverage (700+ partner patterns + AI-backed generic detection). Gitleaks itself
  is feature-frozen with an unresolved entropy false-positive regression
  ([#1830](https://github.com/gitleaks/gitleaks/issues/1830)) affecting Rust/Go identifier names.
  Re-evaluate after Betterleaks has 60+ days of production soak.
- Fixed 2026-07-03 (`crud-be-kotlin-ktor` "59 missing step implementations"): the "gap" was never a
  real feature gap — `contexts/expenses/` (an empty DDD-layered scaffold, `.gitkeep` only) misled the
  earlier investigation; the real, fully-working implementation lives in the flat `routes/`/`domain/`
  tree (`ExpenseRoutes.kt`, `ReportRoutes.kt`, `AttachmentRoutes.kt`, `ExpenseDomain.kt`,
  `AttachmentDomain.kt`), and `nx run crud-be-kotlin-ktor:test:unit` already passed the whole time.
  Root cause was a genuine rhino-cli bug: `extract_jvm_step_texts` scanned Java/Kotlin source
  line-by-line, so any `@When(\n  "..."\n)` annotation with its string on its own line (a common
  formatter wrap for long step text) was silently invisible to the coverage tool — fixed to scan
  whole-file content instead (matching how `extract_dart_step_texts` already worked; no dotall flag
  needed, `jvm_step_re()` has no `.` metacharacter). Backported byte-identical to all 3 repos. The
  remaining 20 "orphan step implementation" reports were a separate, unrelated artifact: `--exclude-dir
test-support` only ever applied to the Gherkin-side scan, never the Kotlin-side scan (a real,
  still-open rhino-cli architecture gap — `extract_all_step_texts` uses a hardcoded `skip_dirs()`, not
  the CLI's `exclude_dirs`), so Kotlin's fully-working `test-support/test-api.feature` steps in
  `UnitTestSupportSteps.kt`/`IntegrationTestSupportSteps.kt` were flagged as orphaned once their
  Gherkin scenario was excluded. Fixed by dropping `--exclude-dir test-support` from
  `crud-be-kotlin-ktor`'s `specs:behavior:coverage` command specifically (Kotlin, unlike its siblings,
  actually implements those scenarios) rather than deferring to the more invasive rhino-cli
  architecture fix. `specs:behavior:coverage` re-enabled with its real command; `Spec coverage valid!
14 specs, 92 scenarios, 344 steps — all covered.`
- **rhino-cli**: `extract_all_step_texts` (shared-steps whole-app step-implementation scan) ignores
  the CLI's `--exclude-dir` flag entirely — it only filters via a hardcoded `skip_dirs()` build-artifact
  list, while `walk_feature_files` (the Gherkin-side scan) respects the user-supplied `exclude_dirs`.
  Discovered 2026-07-03 fixing the item above; worked around per-project rather than fixed at the
  rhino-cli level since nothing else currently depends on the asymmetry. Fix: thread `exclude_dirs`
  through to `extract_all_step_texts` too, so both sides of a `--shared-steps` comparison exclude the
  same directories consistently.
