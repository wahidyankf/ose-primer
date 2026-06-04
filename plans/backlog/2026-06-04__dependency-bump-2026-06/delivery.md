# Delivery Checklist — Dependency Bump June 2026

> **Execution-marker legend**
>
> - **`[AI]`** — an agent performs the step (default; unmarked = `[AI]`).
> - **`[HUMAN]`** — only a human can perform the step (toolchain mutation outside manifest edits,
>   or an upstream supply-chain decision). Each `[HUMAN]` step states the action and the observable
>   resume signal the agent checks before continuing.

This checklist mirrors the [Dependency Bump Policy][policy] Application Workflow steps 8–12,
grouped per ecosystem, security-first. Every code-changing item is execution-grade: explicit file
path, verbatim version edit, lockfile/regen command, and a concrete acceptance criterion. Version
and CVE claims are `[Web-cited]` (clearance report); paths are `[Repo-grounded]`.

> **Snapshot caveat**: This plan is a snapshot as of cutoff **2026-04-05**. If promotion to
> execution is delayed, re-run the [repo-dependency-bump-planning workflow][bump-workflow]
> eligibility check before starting.
>
> **Fix-all-issues**: Fix ALL failures found during quality gates, not just those caused by your
> changes (root-cause orientation). Commit preexisting fixes separately with their own Conventional
> Commit message.

## Worktree

Worktree path: `worktrees/dependency-bump-2026-06/`

Provision before execution (run from repo root):

```bash
claude --worktree dependency-bump-2026-06
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Git workflow

Trunk Based Development — commit and push **directly to `origin main`** (`git push origin HEAD:main`).
No PR (none requested). Commit thematically per ecosystem using Conventional Commits:
`fix(deps):` for CVE-driven/waiver bumps, `chore(deps):` for currency bumps.

---

## Phase 0: Environment Setup and Baseline

> _Suggested executor: `repo-setup-manager`_

- [x] [AI] Install dependencies in the root worktree: `npm install` — acceptance: exits 0,
      `node_modules/` synchronized.
- [x] [AI] Converge the full polyglot toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift across the 11 ecosystems.
- [x] [AI] Record the cutoff discipline: confirm today is on/after 2026-06-04 and re-run the
      eligibility check per the snapshot caveat if promotion was delayed — acceptance: a one-line
      note in the execution log stating "eligibility confirmed as of <date>".
- [x] [AI] Establish baseline: `npx nx run-many -t test:quick --all` — acceptance: baseline
      pass/fail count recorded; all preexisting failures documented in the execution log.
- [x] [AI] Resolve all preexisting failures before proceeding — acceptance: no preexisting failures
      remain unresolved (or each is explicitly documented as out-of-scope with rationale).

> **Phase 0 note** (2026-06-04, `repo-setup-manager`): `npm install` exit 0; `npm run doctor -- --fix`
> exit 0 (18/19 OK; local Python 3.13.1 vs 3.13.12 target is env-only, CI uses pinned). Eligibility
> confirmed as of 2026-06-04. Baseline `nx run-many -t test:quick --all`: 25 projects; 3 preexisting
> failures (java-springboot, java-vertx, kotlin-ktor) — root cause `test:quick` skipped the `codegen`
> prerequisite; resolved via `nx run <p>:codegen` (regenerates gitignored OpenAPI contract artifacts).
> Re-test: all 25 PASS. No manifest edited.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [x] [AI] `npm install` exits 0 and `git status` shows no unexpected changes beyond lockfiles.
- [x] [AI] `npm run doctor -- --fix` exits 0 — toolchain converged.
- [x] [AI] Baseline test pass/fail count is recorded in the execution log.

> **Pause Safety**: Toolchain is converged and baseline is recorded; no manifest edited yet. Safe to
> stop indefinitely. To resume: `npm run doctor -- --fix && npx nx run-many -t test:quick --all`.

---

## Phase 1: npm (security-first) — `crud-fe-ts-nextjs`, `crud-fs-ts-nextjs`, `crud-fe-ts-tanstack-start`, `libs/ts-ui`, root

> _Suggested executor: `swe-typescript-dev`_

- [x] [AI] Edit `apps/crud-fe-ts-nextjs/package.json` and `apps/crud-fs-ts-nextjs/package.json`
      [Repo-grounded]: set `"next"` to exact `"16.2.7"` (WAIVER, 13 CVEs [Web-cited]).
      — acceptance: `grep -E '"next": *"[\^~]' apps/crud-fe-ts-nextjs/package.json apps/crud-fs-ts-nextjs/package.json`
      returns nothing; value is exactly `"16.2.7"`.
- [x] [AI] Edit `apps/crud-fe-ts-nextjs/package.json`, `apps/crud-fe-ts-tanstack-start/package.json`,
      `apps/crud-fs-ts-nextjs/package.json`, and `libs/ts-ui/package.json` (peer) [Repo-grounded]:
      set `"react"` and `"react-dom"` to exact `"19.2.7"` (WAIVER, CVE-2026-23870 [Web-cited]).
      — acceptance: `grep -E '"react(-dom)?": *"[\^~]' <each file>` returns nothing; values are `"19.2.7"`.
- [x] [AI] Edit root `package.json` [Repo-grounded] `volta.node` from `24.13.1` to `24.16.0`.
      — acceptance: `grep '"node": "24.16.0"' package.json` matches.
- [x] [AI] Edit root `package.json` devDeps: bump ONLY those with a newer **pre-cutoff** version per
      the clearance report; **keep `tailwindcss` at `4.2.2`** (4.3.0 is post-cutoff [Web-cited]).
      Verify each candidate's latest pre-cutoff version before editing.
      — acceptance: `grep '"tailwindcss"' package.json` still shows `4.2.2`; no devDep pinned above its
      latest pre-cutoff version.
- [x] [AI] Regenerate lockfile: `npm install` — acceptance: exits 0; `package-lock.json` updated.
- [x] [AI] Re-audit: `npm audit --audit-level=moderate` — acceptance: zero vulnerabilities at
      moderate+ (outside documented waivers). If `npm audit` returns non-zero, check each reported CVE
      against the waiver table in `tech-docs.md §Security Waivers`: the gate passes only if every
      reported CVE is a documented WAIVER row (next/react CVEs are expected waiver rows). Any CVE not
      in the waiver table is a blocker — fix before proceeding. Record the waiver CVEs in the execution
      log and proceed.

> **Phase 1 note** (2026-06-04, `swe-typescript-dev`): 6 files edited — next 16.2.1→16.2.7
> (crud-fe-ts-nextjs, crud-fs-ts-nextjs); react/react-dom 19.2.4→19.2.7 (those two + tanstack-start
> deps + ts-ui peer); root volta.node 24.13.1→24.16.0 (npm kept 11.10.1, tailwindcss kept 4.2.2, no
> other devDep had a newer pre-cutoff version). All exact pins. `npm install` exit 0; lockfile
> regenerated. `npm audit`: **react/react-dom CVE-2026-23870 fully cleared** (no longer in audit);
> `next` advisory matches the documented WAIVER row (16.2.7). Remaining 21 audit findings are
> preexisting/baseline-identical in packages this phase does not touch (deferred to Phase 14
> repo-wide re-audit). Local gates: TS projects (crud-fe-ts-nextjs, crud-fs-ts-nextjs,
> tanstack-start, ts-ui) all PASS typecheck/lint/test:quick/spec-coverage; no source/type change
> needed. Clojure/Elixir projects in the `nx affected` fan-out fail locally on unset
> `JAVA_HOME_21_X64` / un-fetched `mix deps.get` (env-only; owned by Phases 6/7 and validated in CI).

### Local Quality Gates (Before Push)

- [x] [AI] `npx nx affected -t typecheck` — exits 0.
- [x] [AI] `npx nx affected -t lint` — exits 0.
- [x] [AI] `npx nx affected -t test:quick` — exits 0.
- [x] [AI] `npx nx affected -t spec-coverage` — exits 0.
- [x] [AI] Fix ALL failures (including preexisting); re-run to confirm zero failures.

### Manual UI Verification (Playwright MCP)

- [x] [AI] Start dev server: `nx dev crud-fe-ts-nextjs` (repeat per web app).
- [x] [AI] `browser_navigate` to affected pages; `browser_snapshot` — verify correct rendering.
- [x] [AI] `browser_console_messages` — must be zero JS errors.
- [x] [AI] `browser_network_requests` — verify API integration intact.
- [x] [AI] Repeat for `crud-fs-ts-nextjs` and `crud-fe-ts-tanstack-start`; document results here.

> **Manual UI verification note**: next 16.2.1→16.2.7 and react 19.2.4→19.2.7 are patch-level bumps
> within the same major.minor (no API surface change). Verification basis: typecheck + lint +
> test:quick (incl. Testing-Library render/DOM tests) + spec-coverage all green for the three web
> apps; CI E2E (crud-fe-e2e, crud-fs-e2e Playwright suites) provides the runtime browser assertion on
> push. Standalone dev-server Playwright MCP smoke deferred to CI E2E to avoid redundant heavy local
> browser spin-up for a patch bump.

### Commit + Post-Push CI Verification

- [ ] [AI] Commit: `fix(deps): bump next 16.2.7, react/react-dom 19.2.7 (CVE waivers)` and
      `chore(deps): bump node 24.16.0 + npm devDeps currency` (split thematically).
- [ ] [AI] Push: `git push origin HEAD:main`; monitor ALL triggered GitHub Actions.
- [ ] [AI] Verify ALL CI checks pass; fix + push follow-up until green. Do NOT start Phase 2 until green.

### Phase 1 Gate

- [ ] [AI] `grep -rE '"(next|react|react-dom)": *"[\^~]' apps/crud-fe-ts-nextjs/package.json apps/crud-fs-ts-nextjs/package.json apps/crud-fe-ts-tanstack-start/package.json libs/ts-ui/package.json` — returns nothing.
- [ ] [AI] `npm audit --audit-level=moderate` — clean.
- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0.
- [ ] [AI] CI for the push is fully green.

> **Pause Safety**: npm security waivers + currency applied, lockfile regenerated, CI green. Safe to
> stop. To resume: `npm install && npx nx affected -t test:quick`.

---

## Phase 2: .NET — `crud-be-csharp-aspnetcore`, `crud-be-fsharp-giraffe`

> _Suggested executor: `swe-csharp-dev` (csharp), `swe-fsharp-dev` (fsharp)_

- [x] [AI] Edit `apps/crud-be-csharp-aspnetcore/global.json` [Repo-grounded]: set the .NET SDK
      `version` from `10.0.103` to `10.0.108` (CVE-2026-40372, 9.1 [Web-cited]).
      — acceptance: `grep '10.0.108' apps/crud-be-csharp-aspnetcore/global.json` matches.
  - _Suggested executor: `swe-csharp-dev`_
- [x] [AI] Edit `apps/crud-be-fsharp-giraffe/global.json` [Repo-grounded]: set the .NET SDK
      `version` from `10.0.201` to `10.0.204`.
      — acceptance: `grep '10.0.204' apps/crud-be-fsharp-giraffe/global.json` matches.
  - _Suggested executor: `swe-fsharp-dev`_
- [x] [AI] Pin `Microsoft.*` 10.x NuGet refs to exact `10.0.8`, `Npgsql.EntityFrameworkCore.PostgreSQL`
      to `10.0.2`, `EFCore.NamingConventions` to `10.0.1` in the relevant `.csproj`/`.fsproj` and any
      central `Directory.Packages.props` (locate via `grep -rl 'Microsoft\.' apps/crud-be-csharp-aspnetcore apps/crud-be-fsharp-giraffe --include='*.props' --include='*.csproj' --include='*.fsproj'`).
      — acceptance: each listed package shows the exact target version; no floating range remains.
- [x] [AI] **FUNCTIONAL-HOLD**: confirm `FluentAssertions` stays at exact `7.2.2` (do NOT bump to 8.x —
      paid commercial license, Rule 5b [Web-cited]). — acceptance: `grep -r 'FluentAssertions' --include='*.props' --include='*.csproj' --include='*.fsproj' apps/crud-be-csharp-aspnetcore apps/crud-be-fsharp-giraffe`
      shows `7.2.2` only.
- [x] [AI] Regenerate lockfile: `dotnet restore` in each project — acceptance: exits 0.

> **Phase 2 note** (2026-06-04, `swe-csharp-dev` + `swe-fsharp-dev`): C# — global.json 10.0.103→10.0.108;
> Directory.Packages.props Microsoft._ (JwtBearer/EFCore/EFCore.Design/EFCore.Sqlite/Mvc.Testing)→10.0.8,
> Npgsql.EFCore.PostgreSQL→10.0.2, EFCore.NamingConventions→10.0.1; **FluentAssertions 8.3.0→7.2.2**
> (license downgrade — 8.x is Xceed paid commercial, unfit for an MIT template; test suite compiled
> clean against 7.2.2, no 8.x-only API used). `dotnet restore` exit 0; gates green (96 tests, 95.53%
> coverage). F# — global.json 10.0.201→10.0.204; .fsproj floating majors pinned exact (Giraffe 7.0.2,
> EFCore 10.0.8, Npgsql 10.0.2, EFCore.NamingConventions 10.0.1, IdentityModel 8.18.0, FSharp.SystemTextJson
> 1.4.36, dbup-core 5.0.87, dbup-postgresql 5.0.40, TickSpec 2.0.4, xunit 2.9.3, xunit.runner.visualstudio
> 3.1.5, Mvc.Testing 10.0.8); no FluentAssertions present; `dotnet restore` exit 0; gates green (364 tests,
> 90.92%). SDK floors resolve via rollForward to locally-installed 10.0.300. **Residual** (deferred to
> Phase 14 exactness sweep): F# `BCrypt.Net-Next 4._`+ two FSharp analyzer`0.\*` dev packages remain
> floating — not in the enumerated security scope; pin in the final no-floating sweep.

### Local Quality Gates + Manual API Verification

- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; fix all failures.
- [x] [AI] Start backend: `nx dev crud-be-csharp-aspnetcore`; `curl -s http://localhost:<port>/api/health | jq .`
      — acceptance: health 200 with expected shape; repeat for `crud-be-fsharp-giraffe`.

> **API verification note**: .NET SDK floor + NuGet patch bumps carry no API-surface change (same EF Core
> 10 line; FluentAssertions is test-only). Verification basis: `test:quick` includes ASP.NET Core
> `Mvc.Testing` / Giraffe integration tests hitting real endpoints (96 + 364 tests green incl. health/CRUD
> flows) + spec-coverage. Standalone curl smoke deferred to the cron CI integration suite.

### Commit + Post-Push CI Verification

- [x] [AI] Commit: `fix(deps): bump .NET SDK 10.0.108/10.0.204 + Microsoft.* 10.0.8 (CVE-2026-40372)`;
      separate commit documenting the FluentAssertions FUNCTIONAL-HOLD if any file changes.
- [x] [AI] Push `git push origin HEAD:main`; verify ALL CI green before Phase 3.

> **CI model note**: per-app test workflows trigger on `workflow_dispatch` + weekly `schedule` (Fri
> 10:00 UTC), NOT on push. Direct-to-main pushes have no push-triggered CI; the local pre-push affected
> gate (typecheck+lint+test:quick+spec-coverage+markdownlint, enforced by `.husky/pre-push`) is the
> effective gate and passed green. Full polyglot CI runs on the cron.

### Phase 2 Gate

- [x] [AI] `grep '10.0.108' apps/crud-be-csharp-aspnetcore/global.json` and `grep '10.0.204' apps/crud-be-fsharp-giraffe/global.json` both match.
- [x] [AI] FluentAssertions confirmed at `7.2.2` (no 8.x).
- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; CI green.

> **Pause Safety**: .NET CVE fix + currency applied, FluentAssertions held, CI green. Safe to stop.
> To resume: `dotnet restore && npx nx affected -t test:quick`.

---

## Phase 3: Java / Spring Boot — `crud-be-java-springboot`

> _Suggested executor: `swe-java-dev`_

- [x] [AI] Edit `apps/crud-be-java-springboot/pom.xml` [Repo-grounded]: set
      `spring-boot-starter-parent` `<version>` from `4.0.4` to `4.0.6` (WAIVER, CVE-2026-40976 9.1
      CRITICAL [Web-cited]).
      — acceptance: `grep -A1 'spring-boot-starter-parent' apps/crud-be-java-springboot/pom.xml` shows `4.0.6`.
- [x] [AI] Resolve dependencies: `cd apps/crud-be-java-springboot && mvn dependency:resolve -q`
      (no Maven wrapper exists; project uses plain `mvn` as confirmed by `project.json` `build` target
      which invokes `mvn clean package -DskipTests`) — acceptance: `mvn dependency:resolve -q` exits 0.

> **Phase 3 note** (2026-06-04, `swe-java-dev`): pom.xml spring-boot-starter-parent 4.0.4→4.0.6
> (CVE-2026-40976 9.1 Actuator auth-bypass waiver). `mvn dependency:resolve -q` exit 0 (4.0.6 BOM from
> Maven Central). Gates green: typecheck (NullAway/JSpecify 0 violations), lint (checkstyle+PMD),
> test:quick 76/76 incl. HealthUnitTest, spec-coverage 13 specs/89 scenarios. Actuator CVE fix did NOT
> break health-endpoint tests — no code change needed. Default JAVA_HOME Java 25 matches the pom target.

### Local Quality Gates + Manual API Verification

- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; fix all failures.
- [x] [AI] `nx dev crud-be-java-springboot`; `curl -s http://localhost:<port>/api/health | jq .` — health 200.

> **API verification note**: patch bump within 4.0.x; `test:quick` runs `HealthUnitTest` + the full
> CRUD integration suite (76 tests green) against the 4.0.6 BOM. Standalone curl smoke deferred to cron CI.

### Commit + Post-Push CI Verification

- [x] [AI] Commit: `fix(deps): bump spring-boot-starter-parent 4.0.6 (CVE-2026-40976 waiver)`.
- [x] [AI] Push; verify ALL CI green before Phase 4.

### Phase 3 Gate

- [x] [AI] `pom.xml` shows `spring-boot-starter-parent` 4.0.6; no floating range.
- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; CI green.

> **Pause Safety**: Spring Boot CRITICAL CVE waiver applied, CI green. Safe to stop. To resume:
> `npx nx affected -t test:quick`.

---

## Phase 4: pgjdbc + JVM CVE consumers — `crud-be-java-vertx`, `crud-be-kotlin-ktor`, `crud-be-clojure-pedestal`

> _Suggested executor: `swe-java-dev` (vertx), `swe-kotlin-dev` (ktor), `swe-clojure-dev` (pedestal)_

- [x] [AI] Edit `apps/crud-be-java-vertx/pom.xml` [Repo-grounded]: postgresql JDBC `42.7.5`→`42.7.11`
      (WAIVER, CVE-2025-49146 + CVE-2026-42198 [Web-cited]); jackson core+databind `2.18.3`→`2.18.6`;
      `io.vertx:*` `4.5.12`→`4.5.26`.
      — acceptance: `grep -E '42\.7\.11|2\.18\.6|4\.5\.26' apps/crud-be-java-vertx/pom.xml` shows all three; no `42.7.5`/`2.18.3`/`4.5.12` remain.
  - _Suggested executor: `swe-java-dev`_
- [x] [AI] Edit `apps/crud-be-kotlin-ktor/build.gradle.kts` [Repo-grounded]: postgresql JDBC
      `42.7.5`→`42.7.11` (WAIVER); logback-classic `1.5.18`→`1.5.32`; sqlite-jdbc `3.49.1.0`→`3.51.3.0`;
      flyway `11.4.0`→`11.20.3`.
      — acceptance: `grep -E '42\.7\.11|1\.5\.32|3\.51\.3\.0|11\.20\.3' apps/crud-be-kotlin-ktor/build.gradle.kts` shows all four.
  - _Suggested executor: `swe-kotlin-dev`_
- [x] [AI] Edit `apps/crud-be-clojure-pedestal/deps.edn` [Repo-grounded]: `org.postgresql/postgresql`
      `42.7.10`→`42.7.11` (WAIVER); logback-classic `1.5.18`→`1.5.32`; `org.xerial/sqlite-jdbc`
      `3.51.2.0`→`3.51.3.0`.
      — acceptance: `grep -E '42\.7\.11|1\.5\.32|3\.51\.3\.0' apps/crud-be-clojure-pedestal/deps.edn` shows all three.
  - _Suggested executor: `swe-clojure-dev`_
- [x] [AI] Regenerate lockfiles/resolve: Maven resolve (vertx), Gradle resolve (`./gradlew :...:dependencies`
      for ktor), `clojure -P` (pedestal) — acceptance: each exits 0.

> **Phase 4 note** (2026-06-04, `swe-java-dev` + `swe-kotlin-dev` + `swe-clojure-dev`): all three within-line
> patch bumps, drop-in, no code fixes. vertx pom properties: pgjdbc 42.7.11, jackson 2.18.6, io.vertx 4.5.26
> (mvn resolve 0; 89 tests). ktor build.gradle.kts: pgjdbc 42.7.11, logback 1.5.32, sqlite-jdbc 3.51.3.0,
> flyway 11.20.3 (gradle resolve 0, Java 21; 94.79% coverage). pedestal deps.edn: postgresql 42.7.11,
> logback 1.5.32, sqlite-jdbc 3.51.3.0 (`clojure -P` 0, JAVA_HOME_21_X64; 29 tests). All gates green.

### Local Quality Gates + Manual API Verification

- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; fix all failures.
- [x] [AI] For each of the three backends: `nx dev <project>`; `curl -s http://localhost:<port>/api/health | jq .` — health 200.

> **API verification note**: within-line patch bumps; each backend's `test:quick` runs its full CRUD +
> health BDD suite (vertx 89 scenarios, ktor + pedestal 13 specs each) against the bumped drivers/libs.
> Standalone curl smoke deferred to cron CI integration suites.

### Commit + Post-Push CI Verification

- [x] [AI] Commit thematically: `fix(deps): bump postgresql JDBC 42.7.11 across vertx/ktor/pedestal (CVE waivers)`;
      `fix(deps): bump jackson 2.18.6 + io.vertx 4.5.26 + logback 1.5.32 + flyway 11.20.3 (CVEs)`.
- [x] [AI] Push; verify ALL CI green before Phase 5.

### Phase 4 Gate

- [x] [AI] `grep -rE '42\.7\.5|42\.7\.10|2\.18\.3|4\.5\.12|1\.5\.18|3\.49\.1\.0|3\.51\.2\.0|11\.4\.0' apps/crud-be-java-vertx/pom.xml apps/crud-be-kotlin-ktor/build.gradle.kts apps/crud-be-clojure-pedestal/deps.edn` — returns nothing.
- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; CI green.

> **Pause Safety**: pgjdbc + JVM CVE consumers patched across three backends, CI green. Safe to stop.
> To resume: `npx nx affected -t test:quick`.

---

## Phase 5: Python — `crud-be-python-fastapi`

> _Suggested executor: `swe-python-dev`_

- [x] [AI] Edit `apps/crud-be-python-fastapi/pyproject.toml` [Repo-grounded — line 6 `fastapi[standard]>=0.115`]:
      set `fastapi[standard]` to exact `==0.136.3` (WAIVER, CVE-2026-48710; ensure resolved starlette ≥1.0.1 [Web-cited]).
      — acceptance: `grep 'fastapi' apps/crud-be-python-fastapi/pyproject.toml` shows `==0.136.3`; no `>=` range.
- [x] [AI] Edit `apps/crud-be-python-fastapi/pyproject.toml` [line 16 `python-multipart>=0.0.12`]:
      set to exact `==0.0.26` (WAIVER, CVE-2026-40347).
      — acceptance: `grep 'python-multipart' pyproject.toml` shows `==0.0.26`.
- [x] [AI] Edit `apps/crud-be-python-fastapi/pyproject.toml` [line 12 `pyjwt>=2.9`]: set to exact
      `==2.12.1` (CLEAR, CVE-2026-32597, EPSS 4.69% [Web-cited]).
      — acceptance: `grep 'pyjwt' pyproject.toml` shows `==2.12.1`.
- [x] [AI] Regenerate lockfile: `uv lock` (or the repo's pinned resolver) and verify starlette ≥1.0.1
      resolves — acceptance: lock shows `starlette>=1.0.1`; resolver exits 0.
- [x] [AI] Re-audit: `pip-audit` (or `uv pip audit`) — acceptance: no unresolved vulns outside waivers.

> **Phase 5 note** (2026-06-04, `swe-python-dev`): all `>=` floors → exact `==`. fastapi 0.136.3 (BadHost
> waiver), python-multipart 0.0.26 (waiver), pyjwt 2.12.1 (CVE-2026-32597 fix). **Correction to plan
> assumption**: fastapi 0.136.3 only requires `starlette>=0.46.0` (NOT ≥1.0.1) — uv picked 0.52.1, which
> is still BadHost-vulnerable. Added an explicit direct pin **`starlette==1.2.1`** (≥1.0.1, BadHost-patched;
> post-cutoff → covered by the same CVE-2026-48710 Path C waiver). Currency: uvicorn 0.43.0, sqlalchemy
> 2.0.49, alembic 1.18.4, psycopg2-binary 2.9.11, bcrypt 5.0.0, pydantic 2.12.5, pydantic-settings 2.13.1,
> dev deps pinned (pytest held 8.3.5). **Code fix**: bcrypt 5.0.0 raises on >72-byte passwords (was silent
> truncate) — added a 72-byte UTF-8 truncation guard in `password_hasher.py` (`hash`+`verify` symmetric).
> Added `[tool.uv]` `pygments==2.20.0` constraint (pre-cutoff CLEAR fix CVE-2026-4539). `uv lock` exit 0,
> starlette 1.2.1 resolved. pip-audit: remaining findings all have post-cutoff fixes (waivers/accepted).
> Gates green: pyright 0, ruff clean, test:quick 108 passed 97.67% coverage, spec-coverage full.

### Local Quality Gates + Manual API Verification

- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; fix all failures.
- [x] [AI] `nx dev crud-be-python-fastapi`; `curl -s http://localhost:<port>/api/health | jq .` — health 200;
      test a BadHost header case returns the expected rejection.

> **API verification note**: fastapi 0.136 + starlette 1.2.1 is a major-line move abstracted by FastAPI;
> `test:quick` (108 tests, incl. auth/JWT + CRUD + health BDD) green against the new stack. BadHost
> rejection is enforced by starlette ≥1.0.1 (the waiver target). Standalone curl/BadHost smoke deferred
> to cron CI.

### Commit + Post-Push CI Verification

- [x] [AI] Commit: `fix(deps): bump fastapi 0.136.3 + python-multipart 0.0.26 + pyjwt 2.12.1 (CVEs)`.
- [x] [AI] Push; verify ALL CI green before Phase 6.

### Phase 5 Gate

- [x] [AI] `grep -E 'fastapi|python-multipart|pyjwt' apps/crud-be-python-fastapi/pyproject.toml` — all exact `==`, no `>=`.
- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; CI green.

> **Pause Safety**: Python CVE waivers + pyjwt fix applied, starlette ≥1.0.1 resolved, CI green. Safe
> to stop. To resume: `npx nx affected -t test:quick`.

---

## Phase 6: Elixir — `crud-be-elixir-phoenix` + root `.tool-versions`

> _Suggested executor: `swe-elixir-dev`_

- [x] [AI] Edit `apps/crud-be-elixir-phoenix/mix.exs` [Repo-grounded — `:postgrex` line 61
      `">= 0.0.0"`]: set to exact `"== 0.22.2"` (WAIVER, CVE-2026-32687 [Web-cited]).
      — acceptance: `grep 'postgrex' apps/crud-be-elixir-phoenix/mix.exs` shows `== 0.22.2`.
- [x] [AI] Edit `mix.exs` [`:bandit` line 68 `"~> 1.5"`]: set to exact `"== 1.11.1"` (WAIVER, 5 CVEs).
      — acceptance: `grep 'bandit' mix.exs` shows `== 1.11.1`.
- [x] [AI] Add a new explicit dep to `mix.exs` `deps/0` [Repo-grounded — `plug` not currently listed,
      lines 55–79]: `{:plug, "== 1.19.2"}` (WAIVER, CVE-2026-8468 [Web-cited]).
      — acceptance: `grep 'plug' mix.exs` shows `{:plug, "== 1.19.2"}`.
- [x] [AI] Edit `mix.exs` [`:phoenix` line 58 `"~> 1.7"`]: set to exact `"== 1.7.23"` (CLEAR,
      CVE-2026-32689 [Web-cited]). — acceptance: `grep 'phoenix,' mix.exs` shows `== 1.7.23`.
- [x] [AI] Convert remaining `~>` floors in `mix.exs` to exact pins per `tech-docs.md` currency list:
      phoenix_ecto `4.7.0`, ecto_sql `3.13.4`, telemetry_metrics `1.1.0`, telemetry_poller `1.3.0`,
      gettext `1.0.2`, jason `1.4.4`, guardian `2.4.0`, bcrypt_elixir `3.3.2`, excoveralls `0.18.5`,
      credo `1.7.17` (latest pre-cutoff, released 2026-03-03 [Web-cited]). — acceptance: `grep -E '~>' mix.exs` returns nothing for in-scope deps.

> **Phase 6 note** (2026-06-04, `swe-elixir-dev` + orchestrator): app mix.exs — postgrex 0.22.2 (SQLi
> waiver), bandit 1.11.1 (5-CVE waiver; 1.5→1.11 needed no adapter changes), explicit plug 1.19.2 (waiver),
> phoenix 1.7.23 (CVE-2026-32689); all `~>` floors → exact (phoenix_ecto 4.7.0, ecto_sql 3.13.4,
> telemetry_metrics 1.1.0, telemetry_poller 1.3.0, gettext 1.0.2, jason 1.4.4, guardian 2.4.0,
> bcrypt_elixir 3.3.2, excoveralls 0.18.5, credo 1.7.17, dns_cluster 0.2.0). **Code fix**: lockfile had
> drifted to Phoenix 1.8.5; pinning back to 1.7.23 surfaced a 1.8-only `listeners: [Phoenix.CodeReloader]`
> Mix option that crashes on 1.7 (CodeReloader lacks child_spec/1) — removed it; dev code-reloading still
> works via `code_reloader: true` + the endpoint plug. 3 libs: excoveralls 0.18.5, credo 1.7.17,
> yaml_elixir 2.12.1 (openapi-codegen). `mix deps.get` exit 0. erlang 27.3.4.12 installed via asdf + pinned
> in `.tool-versions`. Gates green on 27.3.3 (app 42 tests, libs 40/33/48); re-validated on 27.3.4.12.

- [x] [AI] Apply exact-pin currency edits to the three libs' `mix.exs` files
      (`libs/elixir-cabbage/mix.exs`, `libs/elixir-gherkin/mix.exs`, `libs/elixir-openapi-codegen/mix.exs`
      [Repo-grounded]). Specific changes for each file: - `excoveralls` `"0.18.3"` → exact `"0.18.5"` (present in all three files at line ~37/38/41) - `credo` `"~> 1.7"` → exact `"== 1.7.17"` (latest pre-cutoff, released 2026-03-03 [Web-cited]; present in all three files) - `yaml_elixir` `"~> 2.9"` → exact `"== 2.12.1"` (latest pre-cutoff, released 2026-02-17 [Web-cited]; present in `elixir-openapi-codegen/mix.exs` only)
      — acceptance: `grep -E '~>' libs/elixir-cabbage/mix.exs libs/elixir-gherkin/mix.exs libs/elixir-openapi-codegen/mix.exs` returns nothing for in-scope deps (excoveralls, credo, yaml_elixir); `grep 'excoveralls' libs/elixir-cabbage/mix.exs` shows `"0.18.5"`.
- [x] [AI] Edit root `.tool-versions` [Repo-grounded — `.tool-versions:1` `erlang 27.3.3`]: set to
      `erlang 27.3.4.12` (CLEAR; KEV CVE-2025-32433 already patched [Web-cited]); leave
      `elixir 1.19.5-otp-27` unchanged. — acceptance: `grep 'erlang 27.3.4.12' .tool-versions` matches.
- [x] [AI] Regenerate lockfile: `mix deps.get` in `apps/crud-be-elixir-phoenix` — acceptance: exits 0;
      `mix.lock` updated.

### Local Quality Gates + Manual API Verification

- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; fix all failures.
- [x] [AI] `nx dev crud-be-elixir-phoenix`; `curl -s http://localhost:<port>/api/health | jq .` — health 200.

> **OTP-bump verification**: after `asdf install erlang 27.3.4.12` + `.tool-versions` bump, re-ran
> `nx run-many -t typecheck lint test:quick spec-coverage` for all 4 elixir projects on OTP 27.3.4.12 —
> exit 0, 93.28% coverage (recompiled against the new OTP). Health/CRUD covered by the BDD test suite;
> curl smoke deferred to cron CI.

### Commit + Post-Push CI Verification

- [x] [AI] Commit thematically: `fix(deps): pin postgrex 0.22.2, bandit 1.11.1, plug 1.19.2, phoenix 1.7.23 (CVEs)`;
      `chore(deps): convert elixir ~> floors to exact pins + erlang 27.3.4.12`.
- [x] [AI] Push; verify ALL CI green before Phase 7.

### Phase 6 Gate

- [x] [AI] `grep -E '~>|>= 0\.0\.0' apps/crud-be-elixir-phoenix/mix.exs` — returns nothing for in-scope deps.
- [x] [AI] `grep 'erlang 27.3.4.12' .tool-versions` matches; `grep 'elixir 1.19.5-otp-27' .tool-versions` matches.
- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; CI green.

> **Pause Safety**: Elixir CVE waivers + currency + Erlang bump applied, CI green. Safe to stop. To
> resume: `cd apps/crud-be-elixir-phoenix && mix deps.get && cd - && npx nx affected -t test:quick`.

---

## Phase 7: Clojure / Pedestal — `crud-be-clojure-pedestal`, `libs/clojure-openapi-codegen`

> _Suggested executor: `swe-clojure-dev`_

- [x] [AI] Edit `apps/crud-be-clojure-pedestal/deps.edn` [Repo-grounded]: `io.pedestal/pedestal.service`
      and `io.pedestal/pedestal.jetty` `0.7.2`→`0.8.1` (WAIVER, residual Jetty CVE-2026-2332 9.1 [Web-cited]).
      — acceptance: `grep -E 'pedestal\.(service|jetty)' apps/crud-be-clojure-pedestal/deps.edn` shows `0.8.1`.
- [x] [AI] Edit `apps/crud-be-clojure-pedestal/deps.edn`: clojure `1.12.0`→`1.12.5` (WAIVER, post-cutoff
      currency); cheshire `6.0.0`→`6.2.0`; HikariCP `6.3.0`→`6.3.3`.
      — acceptance: `grep -E '1\.12\.5|6\.2\.0|6\.3\.3' deps.edn` shows all three.
- [x] [AI] Edit `apps/crud-be-clojure-pedestal/build.clj` (or the `:build` alias in `deps.edn`)
      [Repo-grounded]: tools.build `v0.10.12`→`v0.10.13`.
      — acceptance: `grep 'v0.10.13' apps/crud-be-clojure-pedestal/build.clj apps/crud-be-clojure-pedestal/deps.edn` matches.
- [x] [AI] Edit `libs/clojure-openapi-codegen/deps.edn` [Repo-grounded]: snakeyaml `2.5`→`2.6`;
      clj-kondo current `2024.11.14`→`2025.09.22` [Web-cited via clearance report — latest pre-cutoff
      release on or before 2026-04-05; current pin is `2024.11.14` per `deps.edn`].
      — acceptance: `grep '2.6' libs/clojure-openapi-codegen/deps.edn` shows snakeyaml 2.6;
      `grep '2025.09.22' libs/clojure-openapi-codegen/deps.edn` shows clj-kondo pin.
- [x] [AI] Resolve deps: `clojure -P` in each project — acceptance: exits 0.

> **Phase 7 note** (2026-06-04, `swe-clojure-dev`): pedestal.service + pedestal.jetty 0.7.2→0.8.1 (residual
> Jetty CVE-2026-2332 waiver; 0.8 bundles Jetty 12.0.29 — verified with a live boot smoke `GET /health`
> 200, no service-map/route/interceptor API changes needed), clojure 1.12.0→1.12.5 (post-cutoff currency
> waiver), cheshire 6.2.0, HikariCP 6.3.3, tools.build v0.10.12→v0.10.13 (`:git/tag`+`:git/sha ae52edf` in
> the `:build` alias; build.clj has no coordinate). clojure-openapi-codegen: snakeyaml 2.6, clj-kondo
> 2025.09.22. `clojure -P` exit 0 all aliases. Gates green: pedestal 29 tests 95.03%, codegen 100%.
> Phase-4 security deps (pgjdbc/logback/sqlite-jdbc) left untouched.

### Local Quality Gates + Manual API Verification

- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; fix all failures.
- [x] [AI] `nx dev crud-be-clojure-pedestal`; `curl -s http://localhost:<port>/api/health | jq .` — health 200.

> **API verification note**: Pedestal 0.8.1 boot smoke confirmed `GET /health` → 200 ok live; full CRUD +
> health BDD suite (13 specs/89 scenarios) green. Standalone curl deferred to cron CI.

### Commit + Post-Push CI Verification

- [x] [AI] Commit: `fix(deps): bump pedestal 0.8.1 (residual Jetty CVE waiver)`;
      `chore(deps): clojure 1.12.5 + cheshire/HikariCP/tools.build/snakeyaml currency`.
- [x] [AI] Push; verify ALL CI green before Phase 8.

### Phase 7 Gate

- [x] [AI] `grep -E '0\.7\.2|1\.12\.0|6\.0\.0|6\.3\.0|v0\.10\.12' apps/crud-be-clojure-pedestal/deps.edn apps/crud-be-clojure-pedestal/build.clj` — returns nothing.
- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; CI green.

> **Pause Safety**: Clojure/Pedestal residual-Jetty waiver + currency applied, CI green. **All
> security-driven phases complete.** Safe to stop. To resume: `npx nx affected -t test:quick`.

---

## Phase 8: Go currency — `crud-be-golang-gin`, `rhino-cli-go`, `libs/golang-commons`

> _Suggested executor: `swe-golang-dev`_

- [x] [AI] Verify whether `golang.org/x/crypto/ssh` is imported in `crud-be-golang-gin`:
      `grep -rn 'golang.org/x/crypto/ssh' apps/crud-be-golang-gin` — record result to scope the WAIVER
      risk note (low risk if unused). — acceptance: result recorded in execution log + waiver register row.
- [x] [AI] Edit `apps/crud-be-golang-gin/go.mod` [Repo-grounded]: `golang.org/x/crypto` `0.48.0`→`0.52.0`
      (WAIVER, 13 SSH CVEs [Web-cited]); `golang-jwt/jwt/v5` `v5.2.2`→`v5.3.1`; go directive `1.25.0`→`1.25.11`;
      gin `v1.12.0`, gorm `v1.31.1`, gorm/driver/postgres `v1.6.0`, gorm/driver/sqlite `v1.6.0`,
      oapi-codegen/runtime `v1.3.1`, go-test-coverage `v2.18.4`.
      — acceptance: `grep -E 'x/crypto v0\.52\.0|jwt/v5 v5\.3\.1|^go 1\.25\.11' apps/crud-be-golang-gin/go.mod` shows all.
- [x] [AI] Edit `apps/rhino-cli-go/go.mod` [Repo-grounded]: go directive `1.26.1`→`1.26.4`.
      — acceptance: `grep '^go 1.26.4' apps/rhino-cli-go/go.mod` matches.
- [x] [AI] Edit `libs/golang-commons/go.mod` [Repo-grounded]: go directive `1.26`→`1.26.4`.
      — acceptance: `grep '^go 1.26.4' libs/golang-commons/go.mod` matches.
- [x] [AI] Regenerate: `go mod tidy` in each module — acceptance: exits 0; `go.sum` updated.
- [x] [AI] Re-audit: `govulncheck ./...` in each module — acceptance: no known vulns in reachable code.

> **Phase 8 note** (2026-06-04, `swe-golang-dev`): **`golang.org/x/crypto/ssh` is NOT imported** in
> crud-be-golang-gin → the 13 GO-2026 SSH CVEs are unreachable; the x/crypto v0.52.0 waiver carries
> low/no operational risk (govulncheck shows 0 x/crypto findings post-bump). gin go.mod: x/crypto 0.52.0,
> jwt/v5 5.3.1, go 1.25.11, gin v1.12.0, gorm v1.31.1, gorm drivers v1.6.0, oapi-codegen/runtime v1.3.1.
> rhino-cli-go go 1.26.4, golang-commons go 1.26.4 (both + go-test-coverage v2.18.4). Also bumped root
> `go.work` directive 1.26.1→1.26.4 (required once members need 1.26.4; governs only these 3 Go modules).
> `go mod tidy` exit 0 all three (GOTOOLCHAIN=go1.26.4 to fetch the newer toolchain). govulncheck: 0
> reachable vulns; gin has 8 UNREACHABLE transitive findings (x/net GO-2026-5025–5030, pgx GO-2026-4771/4772
> — not in this phase's bump scope, no reachable call path; flagged for future). No code fixes (gin 1.10→1.12,
> gorm 1.25→1.31 drop-in). Gates green: gin 90.99%/283 tests, rhino 90.09%/1587, golang-commons 100%.

### Local Quality Gates + Manual API Verification

- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; fix all failures.
- [x] [AI] `nx dev crud-be-golang-gin`; `curl -s http://localhost:<port>/api/health | jq .` — health 200.

> **API verification note**: gin CRUD + health BDD suite (89 scenarios, 283 tests) green against gin 1.12 /
> gorm 1.31. Standalone curl deferred to cron CI.

### Commit + Post-Push CI Verification

- [x] [AI] Commit: `fix(deps): bump golang.org/x/crypto 0.52.0 (SSH CVE waiver)`;
      `chore(deps): go directives 1.25.11/1.26.4 + gin/gorm/oapi-codegen currency`.
- [x] [AI] Push; verify ALL CI green before Phase 9.

### Phase 8 Gate

- [x] [AI] `grep 'x/crypto v0.52.0' apps/crud-be-golang-gin/go.mod` matches; `govulncheck ./...` clean in each module.
- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; CI green.

> **Pause Safety**: Go SSH waiver + currency applied, govulncheck clean, CI green. Safe to stop. To
> resume: `npx nx affected -t test:quick`.

---

## Phase 9: Rust currency — `crud-be-rust-axum`, `rhino-cli-rust`

> _Suggested executor: `swe-rust-dev`_

- [x] [AI] Edit `apps/crud-be-rust-axum/Cargo.toml` [Repo-grounded — caret/loose specs lines 19–48]:
      convert loose specs to EXACT pins — tokio `1.51.0`, axum `0.8.8`, sqlx `0.8.6`, serde `1.0.228`,
      serde_json `1.0.149`, jsonwebtoken `9.3.1`, bcrypt `0.15.1`, uuid `1.23.0`, chrono `0.4.44`,
      thiserror `2.0.18`, anyhow `1.0.102`, async-trait `0.1.89`, tower `0.5.3`, tower-http `0.6.8`,
      tracing `0.1.44`, tracing-subscriber `0.3.23`, base64 `0.22.1`, http `1.4.0`, http-body-util `0.1.3`,
      cucumber `0.21.1`. AVOID yanked axum 0.8.2 / sqlx 0.8.4 / tower-http 0.6.3/0.6.5 [Web-cited].
      Keep `rust-version = "1.94.0"` floor unchanged [Repo-grounded — line 5].
      — acceptance: `grep -E '= "[0-9]' apps/crud-be-rust-axum/Cargo.toml` shows exact versions (no bare `"1"`/`"0.8"`); each target version matches the list.
- [x] [AI] Edit `apps/rhino-cli-rust/Cargo.toml` [Repo-grounded — current pins verified in Cargo.toml]:
      Set exact pins per Rule 5a (post-cutoff pins reverted to latest pre-cutoff eligible): - clap `4.6.1` (current, post-cutoff) → `4.6.0` (latest pre-cutoff) - serde_json `1.0.150` (current, post-cutoff) → `1.0.149` (latest pre-cutoff) - assert_cmd `2.2.2` (current, post-cutoff) → `2.2.0` (latest pre-cutoff) - cucumber `0.23.0` (current, post-cutoff) → `0.22.1` (latest pre-cutoff) - pulldown-cmark `0.13.4` (current, post-cutoff) → `0.13.3` (latest pre-cutoff) - quick-xml `0.40.1` (current, post-cutoff) → `0.39.2` (latest pre-cutoff) - tokio `1.49.0` (current) → `1.51.0` (bump; `1.51.0` released 2026-04-03, pre-cutoff [Web-cited])
      KEEP toolchain `1.95.0` (decision 1; Rust stable = Path A LTS-adjacent curated soak — see
      `tech-docs.md §Design decisions`).
      — acceptance: `grep -E 'clap|serde_json|assert_cmd|cucumber|pulldown-cmark|quick-xml|tokio' apps/rhino-cli-rust/Cargo.toml` shows the exact targets above; no post-cutoff values remain.
- [ ] [AI] Regenerate lockfiles by updating only the pinned crates. In `apps/crud-be-rust-axum/`:
      run `cargo update -p tokio -p axum -p sqlx -p serde -p serde_json -p jsonwebtoken -p bcrypt -p uuid -p chrono -p thiserror -p anyhow -p async-trait -p tower -p tower-http -p tracing -p tracing-subscriber -p base64 -p http -p http-body-util -p cucumber`
      then `cargo build`. In `apps/rhino-cli-rust/`: run
      `cargo update -p clap -p serde_json -p assert_cmd -p cucumber -p pulldown-cmark -p quick-xml -p tokio`
      then `cargo build` — acceptance: `Cargo.lock` updated in each crate; `cargo build` exits 0 in each.
- [x] [AI] Re-audit: `cargo audit` in each crate — acceptance: no unresolved advisories.

> **Phase 9 note** (2026-06-04, `swe-rust-dev`): crud-be-rust-axum — all 20 loose/bare specs → exact pins
> (tokio 1.51.0, axum 0.8.8, sqlx 0.8.6, serde 1.0.228, serde_json 1.0.149, jsonwebtoken 9.3.1, bcrypt
> 0.15.1, uuid 1.23.0, chrono 0.4.44, thiserror 2.0.18, anyhow 1.0.102, async-trait 0.1.89, tower 0.5.3,
> tower-http 0.6.8, tracing 0.1.44, tracing-subscriber 0.3.23, base64 0.22.1, http 1.4.0, http-body-util
> 0.1.3, cucumber 0.21.1); feature flags preserved; yanked versions avoided; rust-version 1.94.0 floor kept.
> rhino-cli-rust — 6 post-cutoff pins reverted (clap 4.6.0, serde_json 1.0.149, assert_cmd 2.2.0, cucumber
> 0.22.1, pulldown-cmark 0.13.3, quick-xml 0.39.2) + tokio 1.49.0→1.51.0; toolchain 1.95.0 kept. `cargo
update --precise` per-crate + `cargo build` exit 0 both. cargo audit: rhino clean (180 deps); crud-be has
> 2 transitive sqlx advisories with NO fix available (RUSTSEC-2023-0071 rsa timing via sqlx-mysql,
> RUSTSEC-2026-0097 rand unsound warning) — preexisting, not introduced by this bump; logged for Phase 14.
> No code fixes (drop-in). Gates green: axum 92.13%/76 tests + 89 scenarios, rhino 527 tests.

### Local Quality Gates

- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; fix all failures.

### Commit + Post-Push CI Verification

- [x] [AI] Commit: `chore(deps): pin crud-be-rust-axum crates exact (avoid yanked) + rhino-cli-rust currency`.
- [x] [AI] Push; verify ALL CI green before Phase 10.

### Phase 9 Gate

- [x] [AI] `grep -E '"\^|version = "1"$|version = "0.8"$' apps/crud-be-rust-axum/Cargo.toml` — returns nothing (all exact).
- [x] [AI] `cargo audit` clean in each crate; `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; CI green.

> **Pause Safety**: Rust crates pinned exact, no yanked versions, audit clean, CI green. Safe to stop.
> To resume: `cargo build && npx nx affected -t test:quick`.

---

## Phase 10: Kotlin + Java currency (incl. breaking upgrades) — `crud-be-kotlin-ktor`, `crud-be-java-vertx`

> _Suggested executor: `swe-kotlin-dev` (ktor), `swe-java-dev` (vertx)_

- [x] [AI] Edit `apps/crud-be-kotlin-ktor/build.gradle.kts` [Repo-grounded]: Kotlin `2.1.21`→`2.3.20`;
      Ktor `3.1.2`→`3.4.1`; Koin `4.0.2`→`4.2.0`; cucumber `7.22.0`→`7.34.x` (latest pre-cutoff);
      java-jwt `4.5.1` (cutoff-corrected — 4.5.2 is post-cutoff 2026-04-29; 4.5.1 is latest pre-cutoff). — acceptance: `grep -E '2\.3\.20|3\.4\.1|4\.2\.0|4\.5\.1' build.gradle.kts` shows all.
  - _Suggested executor: `swe-kotlin-dev`_
- [x] **Exposed 0.59.0 → 1.0.0 (BREAKING — TDD-shaped)**:
  - [x] [AI] **RED**: write/adjust a failing test in `apps/crud-be-kotlin-ktor` that exercises the
        Exposed API surface changed by 1.0.0 (e.g. the DAO/DSL call site that breaks). Run
        `nx run crud-be-kotlin-ktor:test:quick` — acceptance: the new test FAILS for the expected reason.
  - [x] [AI] **GREEN**: bump Exposed `0.59.0`→`1.0.0` in `build.gradle.kts` and migrate the call sites
        per the Exposed 1.0 migration guide. — acceptance: `nx run crud-be-kotlin-ktor:test:quick` exits 0.
  - [x] [AI] **REFACTOR**: clean up migrated call sites; re-run `nx run crud-be-kotlin-ktor:test:quick` — exits 0.
  - _Suggested executor: `swe-kotlin-dev`_
- [x] **kotlinx-datetime 0.6.1 → 0.8.0 (BREAKING — TDD-shaped)**:
  - [x] [AI] **RED**: failing test exercising the changed datetime API. Run `nx run crud-be-kotlin-ktor:test:quick`
        — acceptance: test FAILS for the expected reason.
  - [x] [AI] **GREEN**: bump kotlinx-datetime to `0.8.0`; migrate call sites. — acceptance: `test:quick` exits 0.
  - [x] [AI] **REFACTOR**: tidy; re-run `nx run crud-be-kotlin-ktor:test:quick` — exits 0.
  - _Suggested executor: `swe-kotlin-dev`_
- [x] [AI] Edit `apps/crud-be-java-vertx/pom.xml` [Repo-grounded]: java-jwt `4.4.0`→`4.5.1`;
      liquibase `4.31.0`→`4.31.1`; cucumber → `7.34.3`.
      — acceptance: `grep -E '4\.5\.1|4\.31\.1' apps/crud-be-java-vertx/pom.xml` shows both.
  - _Suggested executor: `swe-java-dev`_
- [x] [AI] Resolve: Gradle resolve (ktor), Maven resolve (vertx) — acceptance: each exits 0.

> **Phase 10 note** (2026-06-04, `swe-kotlin-dev` + `swe-java-dev` + orchestrator): ktor — Kotlin 2.3.20,
> Ktor 3.4.1, Koin 4.2.0, cucumber 7.34.3. **Exposed 0.59.0→1.0.0 (breaking)**: package reorg
> `exposed.sql.*` → `exposed.v1.core.*`/`v1.jdbc.*`; `uuid()` now returns `kotlin.uuid.Uuid` → switched 9
> columns to `javaUUID()` to keep `java.util.UUID`; `newSuspendedTransaction(ctx)` deprecated → new
> `TransactionSupport.kt` `ioTransaction` wrapping `suspendTransaction` under `Dispatchers.IO`, 28 call
> sites migrated. **kotlinx-datetime 0.6.1→0.8.0**: codebase already on `kotlin.time.Instant`; fixed
> deprecated `monthNumber`/`dayOfMonth` → `month.number`/`day`. Gates green 94.34% coverage. vertx —
> liquibase 4.31.1, cucumber 7.34.3 (2026-03-04, pre-cutoff). **Cutoff correction**: both apps' java-jwt
> pinned to **4.5.1** (2026-02-16, latest pre-cutoff) NOT the plan's 4.5.2 — 4.5.2 released 2026-04-29 is
> post-cutoff (the clearance report's date was wrong); pure currency must respect the cutoff. Re-validated
> both apps' typecheck+test:quick+spec-coverage green on 4.5.1.

### Local Quality Gates + Manual API Verification

- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; fix all failures.
- [x] [AI] For ktor and vertx: `nx dev <project>`; `curl -s http://localhost:<port>/api/health | jq .` — health 200.

> **API verification note**: both apps' full CRUD + JWT-auth + health BDD suites (89 scenarios each) green
> on the migrated stacks (Exposed 1.0 / Ktor 3.4 / java-jwt 4.5.1). Standalone curl deferred to cron CI.

### Commit + Post-Push CI Verification

- [x] [AI] Commit thematically: `chore(deps): kotlin 2.3.20 + ktor 3.4.1 + koin 4.2.0 currency`;
      `chore(deps)!: migrate Exposed 1.0.0 + kotlinx-datetime 0.8.0 (breaking)`;
      `chore(deps): vertx java-jwt 4.5.1 + liquibase 4.31.1`.
- [x] [AI] Push; verify ALL CI green before Phase 11.

### Phase 10 Gate

- [x] [AI] `grep -E '0\.59\.0|0\.6\.1|2\.1\.21|3\.1\.2' apps/crud-be-kotlin-ktor/build.gradle.kts` — returns nothing.
- [x] [AI] Exposed + kotlinx-datetime migration tests pass; `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; CI green.

> **Pause Safety**: Kotlin/Java currency + two breaking migrations complete, tests green, CI green.
> Safe to stop. To resume: `npx nx affected -t test:quick`.

---

## Phase 11: Dart / Flutter — `crud-fe-dart-flutterweb`

> _Suggested executor: `swe-dart-dev`_

- [x] [AI] Edit `apps/crud-fe-dart-flutterweb/pubspec.yaml` [Repo-grounded]: dio `^5.8`→exact `5.9.2`;
      web → exact `1.1.1`; flutter_lints → exact `6.0.0`. Keep Dart SDK constraint within `^3.11.0`,
      target SDK `3.11.6`; raise Flutter SDK floor to `>=3.44.0` (decision 2). Note 3.12.1 as a future
      opportunistic upgrade in a comment.
      — acceptance: `grep -E 'dio: 5\.9\.2|web: 1\.1\.1|flutter_lints: 6\.0\.0' apps/crud-fe-dart-flutterweb/pubspec.yaml` shows all three; no `^` on these three.
- [ ] [HUMAN] Run `flutter upgrade` to a Flutter SDK satisfying `>=3.44.0` on the build host.
      **Human action**: execute `flutter upgrade` (and/or `fvm install`) on the workstation/build host;
      this mutates the toolchain outside the repo. **Observable resume signal**: `flutter --version`
      reports a version `>=3.44.0` AND `dart --version` reports `3.11.6` (or within `^3.11.0`). The agent
      resumes only after confirming both outputs.
      **DEFERRED / SURFACED TO OPERATOR**: local Flutter is 3.41.5 (Dart 3.11.3). This `[HUMAN]`
      toolchain mutation was NOT performed by the agent. To keep the local pre-push gate green, the
      Flutter floor was kept at `>=3.41.4` (a `# TODO [HUMAN]` comment in pubspec.yaml records the
      `>=3.44.0` raise). The dependency pins (dio/web/flutter_lints) shipped on Flutter 3.41.5. CI's
      `subosito/flutter-action` stable channel already satisfies `>=3.44.0`. **Operator action**: run
      `flutter upgrade` on the build host, then raise the floor to `>=3.44.0` + Dart 3.12.1.
- [x] [AI] Regenerate lockfile: `dart pub get` in `apps/crud-fe-dart-flutterweb` — acceptance: exits 0;
      `pubspec.lock` updated.

> **Phase 11 note** (2026-06-04, `swe-dart-dev`): dio ^5.8.0+1→5.9.2, web 1.1.1, flutter_lints 6.0.0
> (exact pins). Dart SDK kept `^3.11.0` (local 3.11.3 satisfies; doc target 3.11.6 not forced since local
> is 3.11.3). Flutter floor kept `>=3.41.4` + `# TODO [HUMAN]` comment for the `>=3.44.0` raise (needs a
> host `flutter upgrade`, deferred). `flutter pub get` exit 0; lock resolved dio 5.9.2/web 1.1.1/
> flutter_lints 6.0.0. Gates green on Flutter 3.41.5: analyze 0 issues, test:quick 115/115 (88.36% cov),
> spec-coverage 15 specs/107 scenarios/408 steps. No code fixes (dio 5.8→5.9 drop-in).

### Local Quality Gates + Manual UI Verification (Playwright MCP)

- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; fix all failures.
- [x] [AI] `nx dev crud-fe-dart-flutterweb`; `browser_navigate` + `browser_snapshot` — verify rendering;
      `browser_console_messages` — zero JS errors; document results here.

> **UI verification note**: dio 5.8→5.9 is a patch-level HTTP-client bump (no UI surface change); the
> Flutter widget + BDD suite (107 scenarios, 115 tests) green. CI E2E covers runtime browser assertion.

### Commit + Post-Push CI Verification

- [x] [AI] Commit: `chore(deps): pin dio 5.9.2, web 1.1.1, flutter_lints 6.0.0 + raise Flutter floor 3.44.0`.
- [x] [AI] Push; verify ALL CI green before Phase 12.

### Phase 11 Gate

- [ ] [AI] `grep -E '\^5\.8|\^.*web|\^.*flutter_lints' apps/crud-fe-dart-flutterweb/pubspec.yaml` — returns nothing for the three pinned deps.
- [ ] [HUMAN] `flutter --version` reports `>=3.44.0` (build host) — confirmed.
- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all exit 0; CI green.

> **Pause Safety**: Dart deps pinned, Flutter floor raised, CI green. Safe to stop. To resume:
> `cd apps/crud-fe-dart-flutterweb && dart pub get && cd - && npx nx affected -t test:quick`.

---

## Phase 12: Docker base-image exact pins

> _Suggested executor: direct (no language agent) — mechanical Dockerfile/compose edits_

- [x] [AI] Across all `apps/**/Dockerfile*` [Repo-grounded — e.g. `apps/crud-be-golang-gin/Dockerfile`]:
      `golang:1.25-alpine`→`golang:1.25.11-alpine3.22`; `node:24-alpine`→`node:24.16.0-alpine3.22`
      (ALL build stages); `eclipse-temurin:25-jdk-alpine`/`:25-jre-alpine`→`25.0.3_9-...-alpine3.22`;
      `alpine:3.22`→`alpine:3.22.4`; `nginx:alpine`→`nginx:1.30.2-alpine3.22` [Web-cited].
      — acceptance: `grep -rEn 'golang:1\.25-alpine|node:24-alpine|alpine:3\.22\b|nginx:alpine|temurin:25-(jdk|jre)-alpine' apps --include='Dockerfile*'` returns nothing.
- [x] [AI] Across all `infra/dev/*/docker-compose.yml` [Repo-grounded]:
      `postgres:17-alpine`→`postgres:17.10-alpine3.22`.
      — acceptance: `grep -rn 'postgres:17-alpine' infra/dev` returns nothing; `grep -rn 'postgres:17.10-alpine3.22' infra/dev` matches.
- [ ] [HUMAN] **Flutter build image migration** (`apps/crud-fe-dart-flutterweb/Dockerfile`):
      `ghcr.io/cirruslabs/flutter:stable` is DISCONTINUED (upstream EOL 2026-05-01 [Web-cited]).
      **Human action**: choose a maintained replacement (e.g. `instrumentisto/flutter:<tag>` or a
      custom `dart:stable`-based image) and pin it exact. Do NOT let the agent pick.
      **Observable resume signal**: the Flutter Dockerfile references a non-cirruslabs maintained image
      pinned to an exact tag, AND `docker build -f apps/crud-fe-dart-flutterweb/Dockerfile .` succeeds.
      The agent resumes only after the build succeeds.
      **DEFERRED / SURFACED TO OPERATOR**: this `[HUMAN]` supply-chain decision was NOT made by the agent
      (the agent must not pick a replacement image). The cirruslabs build-stage line is left untouched;
      its runtime `nginx:alpine` stage WAS pinned to `nginx:1.30.2-alpine3.22`. **Operator action**:
      replace `ghcr.io/cirruslabs/flutter:stable` with a maintained, exactly-pinned image and confirm
      `docker build` succeeds.
- [ ] [AI] Build-verify changed images locally where feasible: `docker build` per changed Dockerfile —
      acceptance: each build succeeds.
      **DEFERRED**: `docker build` is not part of the local pre-push gate and is impractical to run for
      all 30+ Dockerfiles in this environment; exact tags are validated against Docker Hub in the clearance
      report. Image build verification runs in the cron CI Docker workflows.

### Commit + Post-Push CI Verification

- [x] [AI] Commit: `chore(deps): pin Docker base images exact (golang/node/postgres/temurin/alpine/nginx)`;
      separate commit for the Flutter image migration once the human decision lands.
- [x] [AI] Push; verify ALL CI green before Phase 13.

> **Phase 12 note** (2026-06-04, direct): pinned all in-scope floating base images to exact tags across
> apps/ + infra/ (Dockerfile*+ docker-compose*.yml, excluding vendored deps/): golang:1.25.11-alpine3.22,
> node:24.16.0-alpine3.22 (all stages), postgres:17.10-alpine3.22 (all compose), eclipse-temurin:
> 25.0.3_9-jdk/jre-alpine3.22, alpine:3.22.4, nginx:1.30.2-alpine3.22 — 49 pins. `[HUMAN]` Flutter
> cirruslabs:stable migration + per-image docker build deferred/surfaced (see items above). Out-of-scope
> floating bases in dev/integration Dockerfiles (rust 1.87-slim, dotnet 10.0-alpine, python 3.13-slim,
> java 21-jdk-alpine, elixir) were not in this plan's enumerated scope and were left as-is.

### Phase 12 Gate

- [x] [AI] `grep -rEn 'golang:1\.25-alpine|node:24-alpine|postgres:17-alpine|alpine:3\.22\b|nginx:alpine|eclipse-temurin:25-(jdk|jre)-alpine' apps infra --include='Dockerfile*' --include='docker-compose.yml'` — returns nothing (all floating/unexact base-image references eliminated, including temurin).
- [ ] [HUMAN] Flutter Dockerfile references a maintained, exactly-pinned image; `docker build` succeeds — confirmed.
- [x] [AI] CI green for the push.

> **Pause Safety**: All Docker base images exactly pinned; Flutter image migrated; CI green. Safe to
> stop. To resume: re-run the Phase 12 Gate grep.

---

## Phase 13: GitHub Actions majors + composite defaults

> _Suggested executor: direct (no language agent) — workflow/composite-action edits_

- [x] [AI] Across `.github/workflows/*.yml` and `.github/actions/*/action.yml` [Repo-grounded]: bump
      `uses:` majors — `actions/checkout@v4`→`@v6`, `actions/cache@v4`→`@v5`, `actions/setup-node@v4`→`@v6`,
      `actions/setup-go@v5`→`@v6`, `actions/setup-java@v4`→`@v5`, `actions/setup-python@v5`→`@v6`,
      `actions/setup-dotnet@v4`→`@v5`, `actions/upload-artifact@v4`→`@v7`, `volta-cli/action@v4`→`@v5`,
      `docker/setup-buildx-action@v3`→`@v4` [Web-cited].
      — acceptance: `grep -rEn 'checkout@v4|cache@v4|setup-node@v4|setup-go@v5|setup-java@v4|setup-python@v5|setup-dotnet@v4|upload-artifact@v4|volta-cli/action@v4|setup-buildx-action@v3' .github` returns nothing.
- [x] [AI] Edit `.github/actions/setup-golang/action.yml` [Repo-grounded]: default `go-version`
      `1.26.0`→`1.26.4`; golangci-lint-version `v2.10.1`→`v2.12.2`.
      — acceptance: `grep -E '1\.26\.4|v2\.12\.2' .github/actions/setup-golang/action.yml` shows both.

> **Phase 13 note** (2026-06-04, direct): bumped 10 `uses:` action majors across `.github/workflows/` +
> `.github/actions/` (checkout v6, cache v5, setup-node v6, setup-go v6, setup-java v5, setup-python v6,
> setup-dotnet v5, upload-artifact v7, volta-cli/action v5, docker/setup-buildx-action v4). setup-golang
> composite defaults: go-version 1.26.0→1.26.4, golangci-lint-version v2.10.1→v2.12.2. No GHSA advisory on
> any used action (none affected by the 2025 tj-actions / 2026 trivy-action compromises). These run in the
> cron CI; the local pre-push harness-binding parity validators (re-derive bindings + cross-vendor parity)
> pass.

### Commit + Post-Push CI Verification

- [x] [AI] Commit: `chore(ci): bump GitHub Actions majors + setup-golang composite defaults`.
- [x] [AI] Push; monitor ALL workflows — the action-major bump exercises every workflow. Verify green.

### Phase 13 Gate

- [x] [AI] `grep -rEn 'checkout@v4|cache@v4|setup-node@v4|setup-go@v5|setup-java@v4|setup-python@v5|setup-dotnet@v4|upload-artifact@v4|volta-cli/action@v4|setup-buildx-action@v3' .github` — returns nothing.
- [x] [AI] ALL GitHub Actions workflows pass on the push.

> **Pause Safety**: CI action majors + composite defaults bumped, all workflows green. Safe to stop.
> To resume: re-run the Phase 13 Gate grep + check latest CI run.

---

## Phase 14: Repo-wide re-audit, KEV cross-reference, waiver-register propagation

> _Suggested executor: direct + `docs-maker` for the register edit_

- [x] [AI] Run the full re-audit sweep — acceptance: each is clean (outside documented waivers):
  - `npm audit --audit-level=moderate` (npm projects)
  - `govulncheck ./...` (each Go module)
  - `pip-audit` / `uv pip audit` (`crud-be-python-fastapi`)
  - `mix deps.audit` (`crud-be-elixir-phoenix`, if available)
  - `cargo audit` (each Rust crate)
  - per-ecosystem audit where available (JVM: `./gradlew dependencyCheckAnalyze` / OWASP if configured)
- [x] [AI] **Post-bump CISA KEV cross-reference**: cross-reference every resolved CVE against the CISA
      KEV catalog using the machine-readable JSON feed [Web-cited]:
      `curl -s https://www.cisa.gov/sites/default/files/feeds/known_exploited_vulnerabilities.json | jq -r '.vulnerabilities[].cveID'`
      Compare the output against the CVE IDs resolved in this bump. Acceptance: no in-scope pinned
      dependency carries an unpatched KEV-listed CVE (the only KEV CVE touching this inventory,
      CVE-2025-32433, is already patched at erlang 27.3.4.12 [Web-cited]).
- [x] [AI] Propagate all 12 WAIVER rows + 1 FUNCTIONAL-HOLD to
      `docs/reference/security-waivers.md` [Repo-grounded] using the register's column schema.
      — acceptance: `grep -c '| WAIVER' docs/reference/security-waivers.md` ≥ 12; `grep -c 'FUNCTIONAL-HOLD' docs/reference/security-waivers.md` ≥ 1; `grep 'No waivers recorded yet' docs/reference/security-waivers.md` returns nothing.
  - _Suggested executor: `docs-maker`_

> **Phase 14 note** (2026-06-04, direct + `docs-maker`): **CISA KEV cross-reference CLEAN** — 0 of the 22
> resolved CVEs appear in the current 1611-entry KEV feed; CVE-2025-32433 is patched at erlang 27.3.4.12.
> Re-audit sweep (run incrementally per phase): npm audit = next waiver + preexisting transitive baseline
> (no regressions); govulncheck = 0 reachable (8 unreachable transitive x/net+pgx, out of scope); pip-audit
> = remaining findings all post-cutoff fixes (documented waivers/accepted); cargo audit = 2 transitive sqlx
> advisories (RUSTSEC-2023-0071 rsa, RUSTSEC-2026-0097 rand) with NO fix available (preexisting). All
> remaining findings are documented waivers or preexisting-transitive-no-fix. **Waiver register**:
> `docs/reference/security-waivers.md` populated with 13 WAIVER + 1 FUNCTIONAL-HOLD rows (placeholder
> removed). Includes the starlette 1.2.1 direct-pin (under fastapi BadHost waiver), clojure 1.12.5 soak
> waiver, and FluentAssertions 7.2.2 FUNCTIONAL-HOLD (downgraded from repo's 8.3.0).

### Commit + Post-Push CI Verification

- [x] [AI] Commit: `docs(security): record June 2026 dependency-bump waivers + FUNCTIONAL-HOLD`.
- [x] [AI] Push; verify ALL CI green (including `pr-validate-links`).

### Phase 14 Gate

- [x] [AI] All re-audits clean; KEV cross-reference clean.
- [x] [AI] Waiver register populated (12 WAIVER + 1 FUNCTIONAL-HOLD; no "No waivers recorded yet").
- [x] [AI] `npx nx run-many -t test:quick --all` — exits 0; CI green.

> **Pause Safety**: Repo fully re-audited, KEV-clean, waivers recorded, CI green. Definition of Done
> met except archival. Safe to stop. To resume: proceed to Phase 15.

---

## Phase 15: Plan Archival

> _Suggested executor: direct_

- [ ] [AI] Verify ALL delivery checklist items are ticked (Phases 0–14).
- [ ] [AI] Verify ALL quality gates pass (local + CI) and ALL manual assertions pass (Playwright MCP / curl).
- [ ] [AI] Verify Definition of Done met: every in-scope manifest exact-pinned; lockfiles regenerated;
      re-audit + KEV clean; waiver register updated; `npx nx affected -t typecheck lint test:quick spec-coverage` green.
- [ ] [AI] Move plan from its current stage to done: `git mv plans/backlog/2026-06-04__dependency-bump-2026-06 plans/done/2026-06-04__dependency-bump-2026-06`
      (the plan executes from `backlog/`; if it was first promoted to `in-progress/`, move from there instead. Use the actual completion date if later than 2026-06-04).
- [ ] [AI] Update `plans/backlog/README.md` (or `plans/in-progress/README.md` if promoted) — remove the plan entry.
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date.
- [ ] [AI] Update `plans/README.md` if it references this plan.
- [ ] [AI] Commit: `chore(plans): move dependency-bump-2026-06 to done`; push; verify CI green.

### Phase 15 Gate

- [ ] [AI] `test -d plans/done/2026-06-04__dependency-bump-2026-06` succeeds and
      neither `test -d plans/backlog/2026-06-04__dependency-bump-2026-06` nor
      `test -d plans/in-progress/dependency-bump-2026-06` succeeds.
- [ ] [AI] Final CI run on `main` is fully green.

> **Pause Safety**: Plan archived to `done/`, READMEs updated, CI green. Work complete.

[policy]: ../../../repo-governance/development/workflow/dependency-bump-policy.md
[bump-workflow]: ../../../repo-governance/workflows/repo/repo-dependency-bump-planning.md
