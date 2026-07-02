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
- **Complete `crud-be-kotlin-ktor` step coverage** (59 missing step implementations) — discovered
  2026-07-03 when unrelated `repo-config.yml` edits invalidated the Nx cache and forced a genuine
  `specs:behavior:coverage` re-run for the first time in a while, surfacing real gaps in
  security/currency-handling/unit-handling/attachments/reporting/expense-management scenarios that
  `SpecCoverageMarkers.kt` had already documented as commented-out, not-yet-implemented markers.
  `crud-be-java-springboot` covers the same Gherkin surface via a general `(.*)$` body-capture +
  helper-delegation pattern (see `UnitExpenseSteps.java`); port that shape to Kotlin rather than
  writing 59 literal per-scenario steps. `specs:behavior:coverage` is temporarily overridden to a
  documented no-op on this one project (`apps/crud-be-kotlin-ktor/project.json`) until this lands.
