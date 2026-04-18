---
title: Code Coverage Reference
description: How code coverage is measured, validated, and reported across all projects in the monorepo
category: reference
tags:
  - coverage
  - testing
  - rhino-cli
  - quality
created: 2026-03-22
updated: 2026-04-18
---

# Code Coverage Reference

How code coverage is measured locally via `rhino-cli` across all projects in the monorepo.

## 📋 Coverage Algorithm

All projects use `rhino-cli test-coverage validate` which implements a 3-state line-based algorithm:

- **COVERED**: hit count > 0 AND all branches taken (or no branches)
- **PARTIAL**: hit count > 0 but some branches not taken
- **MISSED**: hit count = 0
- **Coverage %** = `covered / (covered + partial + missed)`

Partial lines count as NOT covered.

## Supported Formats

`rhino-cli` auto-detects the coverage format from the file:

| Format       | Detection                                                                | Used By                                                 |
| ------------ | ------------------------------------------------------------------------ | ------------------------------------------------------- |
| Go cover.out | Default (no other match)                                                 | Go projects                                             |
| LCOV (.info) | Filename ends in `.info` or contains `lcov`                              | TypeScript, Python, Rust, Elixir, F#, C#, Clojure, Dart |
| JaCoCo XML   | Filename ends in `.xml` containing `jacoco`, or XML with `<report>` root | Java (Spring Boot, Vert.x)                              |
| Kover XML    | JaCoCo-compatible XML                                                    | Kotlin                                                  |

## Thresholds

| Project Type           | Threshold | Rationale                                     |
| ---------------------- | --------- | --------------------------------------------- |
| Demo backends          | >= 90%    | Service-layer code with full Gherkin coverage |
| CLI tools              | >= 90%    | Core business logic                           |
| Go libraries           | >= 90%    | Shared utilities                              |
| Elixir libraries       | >= 90%    | Shared libraries                              |
| Clojure libraries      | >= 90%    | Codegen library                               |
| demo-fe-ts-nextjs      | >= 70%    | Frontend app with MSW integration tests       |
| demo-be-fsharp-giraffe | >= 90%    | F#/Giraffe backend API                        |
| Demo frontends         | >= 70%    | API/auth/query layers fully mocked by design  |

## 📊 Per-Project Coverage Details

### Go Projects

**Tool**: `go test -coverprofile=cover.out`
**Format**: Go cover.out (statement-based, mode: set)

| Project            | Coverage File               | Threshold | Exclusions                                          |
| ------------------ | --------------------------- | --------- | --------------------------------------------------- |
| rhino-cli          | `cover.out`                 | 90%       | None                                                |
| golang-commons     | `cover.out`                 | 90%       | None                                                |
| demo-be-golang-gin | `cover_unit.out` (filtered) | 90%       | gorm_store, server, cmd/server, generated-contracts |

**Go exclusion caveat**: Go's `go test -coverprofile` has no built-in
exclusion mechanism. `demo-be-golang-gin` uses `grep -v` to create a
filtered `cover_unit.out` that excludes infrastructure files.

### Java Projects

**Tool**: JaCoCo (Maven plugin)
**Format**: JaCoCo XML at `target/site/jacoco/jacoco.xml`

| Project                 | Threshold | Exclusions                                                                     |
| ----------------------- | --------- | ------------------------------------------------------------------------------ |
| demo-be-java-springboot | 90%       | JPA models (User, Expense), Application class, JpaAuditingConfig, package-info |
| demo-be-java-vertx      | 90%       | Main class, package-info, META-INF                                             |

Exclusions are configured in `pom.xml` via JaCoCo's `<excludes>` element.

### Kotlin Projects

**Tool**: Kover (Gradle plugin)
**Format**: JaCoCo-compatible XML at `build/reports/kover/report.xml`

| Project             | Threshold | Exclusions                       |
| ------------------- | --------- | -------------------------------- |
| demo-be-kotlin-ktor | 90%       | Configured in `build.gradle.kts` |

### TypeScript Projects

**Tool**: Vitest with `@vitest/coverage-v8`
**Format**: LCOV at `coverage/lcov.info`

| Project                   | Threshold | Exclusions                                              |
| ------------------------- | --------- | ------------------------------------------------------- |
| demo-be-ts-effect         | 90%       | `main.ts`, `routes/test-api.ts` (in `vitest.config.ts`) |
| demo-fe-ts-nextjs         | 70%       | None                                                    |
| demo-fe-ts-tanstack-start | 70%       | None                                                    |

Exclusions are configured in `vitest.config.ts` via the `coverage.exclude` array.

### Python Projects

**Tool**: coverage.py via `uv run coverage`
**Format**: LCOV at `coverage/lcov.info`

| Project                | Threshold | Exclusions                                              |
| ---------------------- | --------- | ------------------------------------------------------- |
| demo-be-python-fastapi | 90%       | `tests/*`, `routers/*`, `main.py` (in `pyproject.toml`) |

Exclusions are configured in `[tool.coverage.run].omit` in `pyproject.toml`.

### Rust Projects

**Tool**: cargo-llvm-cov
**Format**: LCOV at `coverage/lcov.info`

| Project           | Threshold | Exclusions                                  |
| ----------------- | --------- | ------------------------------------------- |
| demo-be-rust-axum | 90%       | None (cargo-llvm-cov covers the full crate) |

### Elixir Projects

**Tool**: excoveralls (coveralls.json config)
**Format**: LCOV at `cover/lcov.info`

| Project                | Threshold | Exclusions                                                                                   |
| ---------------------- | --------- | -------------------------------------------------------------------------------------------- |
| demo-be-elixir-phoenix | 90%       | 19 files in `coveralls.json` (application, repo, behaviours, contexts, telemetry, CORS plug) |

### F# Projects

**Tool**: AltCover with `--linecover`
**Format**: LCOV at `coverage/altcov.info`

| Project                | Threshold | Exclusions                                                                                           |
| ---------------------- | --------- | ---------------------------------------------------------------------------------------------------- |
| demo-be-fsharp-giraffe | 90%       | Uses AltCover instead of XPlat Code Coverage to avoid F# `task{}` async state machine BRDA inflation |

### C# Projects

**Tool**: Coverlet (XPlat Code Coverage)
**Format**: LCOV at `coverage/**/coverage.info`

| Project                   | Threshold | Exclusions |
| ------------------------- | --------- | ---------- |
| demo-be-csharp-aspnetcore | 90%       | None       |

### Clojure Projects

**Tool**: cloverage with `--lcov`
**Format**: LCOV at `coverage/lcov.info`

| Project                  | Threshold | Exclusions |
| ------------------------ | --------- | ---------- |
| demo-be-clojure-pedestal | 90%       | None       |

### Dart Projects

**Tool**: `flutter test --coverage`
**Format**: LCOV at `coverage/lcov.info`

| Project                 | Threshold | Exclusions |
| ----------------------- | --------- | ---------- |
| demo-fe-dart-flutterweb | 70%       | None       |

## CI Integration

Coverage is measured during `test:quick` (part of the pre-push hook and main CI).

### Pipeline Flow

1. `test:unit` runs tests and generates the coverage file
2. `rhino-cli test-coverage validate <file> <threshold>` checks locally
3. Both steps are combined in `test:quick`

## 🔬 Troubleshooting

### Coverage drops after adding a new file

New source files with no test coverage appear as 0% in rhino-cli output. Either write tests or add the file to the appropriate exclusion config (language tool config).

### `rhino-cli --exclude` flag

`rhino-cli test-coverage validate` supports `--exclude` glob patterns for runtime exclusion without modifying the coverage file. Note: glob matching may not work with Go's full module paths in `cover.out` — use `grep -v` for Go projects instead.

## 🔗 Related Documentation

- [Three-Level Testing Standard](../../governance/development/quality/three-level-testing-standard.md) - Coverage thresholds and testing levels
- [Project Dependency Graph](./project-dependency-graph.md) - Which projects depend on rhino-cli
- [Nx Configuration](./nx-configuration.md) - How test:quick targets are configured
