# Dependency Bump — June 2026 Cycle

Security-first, policy-driven dependency-bump plan for the `ose-primer` polyglot Nx monorepo,
covering all dependency-bearing manifests across 11 language ecosystems plus Docker base images
and GitHub Actions.

## Completion status (2026-06-04)

**Executed and pushed to `origin main`** — all AI-executable work across Phases 0–15 is complete;
each ecosystem phase was committed thematically and pushed (npm → .NET → Spring Boot → JVM CVE
consumers → Python → Elixir → Clojure → Go → Rust → Kotlin/Java breaking migrations → Dart → Docker
→ GitHub Actions → re-audit/KEV/waiver-register → archival). Local pre-push gate (typecheck + lint +
test:quick + spec-coverage across the affected polyglot graph + markdownlint) was green on every
push. Per-app CI runs on the weekly `schedule` + `workflow_dispatch` (this repo's workflows are not
push-triggered), so the local affected gate is the effective per-push gate.

### Deferred `[HUMAN]` operator follow-ups (2 items)

These were intentionally NOT performed by the agent (toolchain mutation / supply-chain decision) and
are tracked for the operator:

1. **Flutter SDK floor raise** (Phase 11) — the proposed `>=3.44.0` floor was kept at `>=3.41.4`
   because raising it requires a host `flutter upgrade` (local Flutter is 3.41.5). A `# TODO [HUMAN]`
   comment in `apps/crud-fe-dart-flutterweb/pubspec.yaml` records it. The dependency pins (dio 5.9.2,
   web 1.1.1, flutter_lints 6.0.0) shipped. CI's `subosito/flutter-action` stable channel already
   satisfies `>=3.44.0`. **Moot for this bump** — no dependency required the raise.
2. **Flutter build-image migration** (Phase 12) — `ghcr.io/cirruslabs/flutter:stable` is discontinued
   upstream (EOL 2026-05-01). The agent must not pick a replacement image; the build-stage line is
   left untouched (its runtime `nginx:alpine` stage was pinned). **Operator action**: replace it with
   a maintained, exactly-pinned image (e.g. `instrumentisto/flutter` or a custom `dart:stable`-based
   image) and confirm `docker build` succeeds. Recorded in `delivery.md` Phase 12.

## Context

This plan operationalizes the clearance decisions in the
[June 2026 dependency-bump clearance report][clearance-report] [Repo-grounded] (report ID
`repo-dependency-bump-planning__be6560__2026-06-04--13-24`). Every version, CVE, EPSS, KEV, and
release-date claim in this plan is sourced from that report and tagged `[Web-cited]` (the report
itself delegated verification to `web-research-maker` against NVD, GHSA, Snyk, vendor pages, and
the CISA KEV catalog). Every manifest file path is tagged `[Repo-grounded]` and was confirmed to
resolve in the current commit before authoring.

The plan follows the
[Dependency Bump Stability & Safety Policy][policy] — the 60-day soak rule (Path B), the CVE
clearance process (Path A/B/C), and the `FUNCTIONAL-HOLD` rule (5b). It is **planning only**: no
manifest is edited here. All edits happen later under the
[plan-execution workflow][plan-execution].

### Snapshot caveat

This plan is a **snapshot as of cutoff 2026-04-05** (today − 60 days, where today = 2026-06-04)
[Web-cited]. Per the policy's _When the Plan Spans Many Days_ section, if promotion to execution
is delayed, re-run the eligibility check (the
[repo-dependency-bump-planning workflow][bump-workflow]) before execution to catch newly-eligible
versions or newly-disclosed CVEs.

## Scope

### In scope

- **npm**: `crud-fe-ts-nextjs`, `crud-fs-ts-nextjs`, `crud-fe-ts-tanstack-start`, `libs/ts-ui`,
  root `package.json` devDeps, Node.js Volta pin.
- **Go**: `crud-be-golang-gin`, `rhino-cli-go`, `libs/golang-commons`.
- **Rust**: `crud-be-rust-axum`, `rhino-cli-rust`.
- **.NET**: `crud-be-csharp-aspnetcore`, `crud-be-fsharp-giraffe`.
- **JVM**: `crud-be-java-springboot`, `crud-be-java-vertx`, `crud-be-kotlin-ktor`.
- **Python**: `crud-be-python-fastapi`.
- **Elixir**: `crud-be-elixir-phoenix` (+ root `.tool-versions` Erlang/Elixir pin, libs).
- **Clojure**: `crud-be-clojure-pedestal`, `libs/clojure-openapi-codegen`.
- **Dart/Flutter**: `crud-fe-dart-flutterweb`.
- **Docker**: all `apps/**/Dockerfile*` base images + all `infra/dev/*/docker-compose.yml`.
- **GitHub Actions**: `uses:` action majors + composite action default pins under
  `.github/actions/`.

### Out of scope (per policy)

- Lockfiles (`package-lock.json`, `Cargo.lock`, `go.sum`, `mix.lock`) are **regenerated** by the
  pinned-version edits but are never hand-edited.
- Workspace-internal `*` / `workspace:*` / `path:` refs.
- Type-only, zero-runtime-surface dev dependencies.
- The `crud-be-ts-effect`, `crud-be-e2e`, `crud-fe-e2e` projects carry no in-scope security or
  currency bumps in this cycle (already at latest pre-cutoff per the clearance report).

### Affected apps

`crud-fe-ts-nextjs`, `crud-fs-ts-nextjs`, `crud-fe-ts-tanstack-start`, `crud-be-golang-gin`,
`rhino-cli-go`, `crud-be-rust-axum`, `rhino-cli-rust`, `crud-be-csharp-aspnetcore`,
`crud-be-fsharp-giraffe`, `crud-be-java-springboot`, `crud-be-java-vertx`, `crud-be-kotlin-ktor`,
`crud-be-python-fastapi`, `crud-be-elixir-phoenix`, `crud-be-clojure-pedestal`,
`crud-fe-dart-flutterweb`, plus `libs/ts-ui`, `libs/golang-commons`, `libs/clojure-openapi-codegen`,
`libs/elixir-*`.

## Approach summary

1. **Phase 0** — environment setup and baseline (executor: `repo-setup-manager`).
2. **Security-first phases** (Path C waivers + Path A/B CVE-driven), one phase per ecosystem, in
   this order: npm → .NET → Java/Spring Boot → pgjdbc consumers (Vert.x, Ktor, Clojure) → Python
   → Elixir → Clojure/Pedestal.
3. **Pure-currency phases** — Go, Rust, Kotlin, Java currency, Dart.
4. **Infrastructure phases** — Docker exact-pin, GitHub Actions majors.
5. **Re-audit + waiver propagation + archival.**

Every in-scope manifest is pinned **EXACT** (no `^` / `~` / `latest` / floating major). Lockfiles
are regenerated. Post-bump re-audit (`npm audit`, `govulncheck`, per-ecosystem audit) must be
clean, and a post-bump CISA KEV cross-reference must be clean. All `WAIVER` / `FUNCTIONAL-HOLD`
entries are propagated to the long-lived [security-waivers register][waiver-register].

## Document map

| Document                         | Purpose                                                                  |
| -------------------------------- | ------------------------------------------------------------------------ |
| [`brd.md`](./brd.md)             | Business rationale — why this cycle runs, impact, risks                  |
| [`prd.md`](./prd.md)             | Product requirements — personas, user stories, Gherkin acceptance        |
| [`tech-docs.md`](./tech-docs.md) | Per-ecosystem clearance tables, security-waiver detail, design decisions |
| [`delivery.md`](./delivery.md)   | Phased, TDD-shaped executable checklist with phase gates                 |

## References

- [June 2026 clearance report][clearance-report]
- [Dependency Bump Stability & Safety Policy][policy]
- [Security Waivers Register][waiver-register]
- [repo-dependency-bump-planning workflow][bump-workflow]
- [plan-execution workflow][plan-execution]

[clearance-report]: ../../../generated-reports/repo-dependency-bump-planning__be6560__2026-06-04--13-24__report.md
[policy]: ../../../repo-governance/development/workflow/dependency-bump-policy.md
[waiver-register]: ../../../docs/reference/security-waivers.md
[bump-workflow]: ../../../repo-governance/workflows/repo/repo-dependency-bump-planning.md
[plan-execution]: ../../../repo-governance/workflows/plan/plan-execution.md
