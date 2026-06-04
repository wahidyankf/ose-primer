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

### `[HUMAN]` operator follow-ups

1. **Flutter build-image migration** (Phase 12) — ✅ **DONE (2026-06-04, operator-approved)**.
   `ghcr.io/cirruslabs/flutter:stable` (discontinued upstream, EOL 2026-05-01) was replaced with the
   maintained community image **`instrumentisto/flutter:3.41.5`** (exact pin). The full
   `docker build -f apps/crud-fe-dart-flutterweb/Dockerfile …` **succeeded** locally (Flutter web
   compiles in the build stage; nginx runtime stage assembles). The build-verify also triggered a
   `docker manifest inspect` audit of every Phase 12 pin, which caught and fixed two invalid tags from
   the clearance report — see the recheck note below.
2. **Flutter SDK floor raise** (Phase 11) — still deferred (low value). The proposed `>=3.44.0` floor
   was kept at `>=3.41.4` because raising it requires a host `flutter upgrade` (local Flutter is
   3.41.5). A `# TODO [HUMAN]` comment in `apps/crud-fe-dart-flutterweb/pubspec.yaml` records it. The
   dependency pins (dio 5.9.2, web 1.1.1, flutter_lints 6.0.0) shipped. CI's `subosito/flutter-action`
   stable channel already satisfies `>=3.44.0`. **Moot for this bump** — no dependency required it.

### Docker tag-format correction (2026-06-04, from the build-verify audit)

Building the Flutter image revealed that the clearance report's `-alpine3.22` suffix is **invalid** for
two image families — they publish version-pinned tags with a bare `-alpine` suffix. Fixed:
`nginx:1.30.2-alpine3.22` → `nginx:1.30.2-alpine`; `eclipse-temurin:25.0.3_9-{jdk,jre}-alpine3.22` →
`…-{jdk,jre}-alpine` (5 Java Dockerfiles). The other pins (`golang:1.25.11-alpine3.22`,
`node:24.16.0-alpine3.22`, `postgres:17.10-alpine3.22`, `alpine:3.22.4`) were confirmed to exist via
`docker manifest inspect`. Without this fix the Java and Flutter Docker images would have failed to
build in CI.

## Post-execution recheck (2026-06-04)

A full recheck confirmed: HEAD == `origin/main`; full `nx run-many -t test:quick --all` green (all 25
projects, exit 0); every security-critical pin matches its claim; `docs/reference/security-waivers.md`
holds 13 WAIVER + 1 FUNCTIONAL-HOLD; no in-scope floating Docker base image or old GitHub Action major
remains. The recheck also caught and fixed two residuals: three `crud-be-fsharp-giraffe` packages still
floated (`BCrypt.Net-Next 4.*` resolved to a post-cutoff 4.2.1 — repinned exact to `4.0.3` matching the
C# app; analyzers pinned to `0.5.0` / `0.22.0`), and `go.work.sum` had an uncommitted checksum update.

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
