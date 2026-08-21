# Backlog Plans

Planned projects for future implementation.

## Planned Projects

- [crud-kotlin-codegen-jdk-fix](./crud-kotlin-codegen-jdk-fix/README.md) — fix
  `crud-be-kotlin-ktor`'s `codegen` Nx target: the chained `./gradlew ktfmtFormatMain` sub-command is
  missing the `JAVA_HOME` override every sibling gradle target already carries, so it fails under an
  ambient JDK newer than 21 (Gradle 8.14 cannot run its Kotlin-DSL compiler on JDK 25).

Ideas awaiting promotion live as two-pagers in [ideas](../ideas/README.md); plans already underway
live in [in-progress](../in-progress/README.md).

## Instructions

**Idea Capture**: For ideas not ready for formal planning, write a two-pager in `../ideas/`.

**Naming**: Plans in `backlog/` use NO date prefix — just the slug (e.g., `add-investment-oracle-app/`).
A date prefix is applied only when a plan is archived to `done/`, where it records the completion date.

When creating a new plan:

1. Create folder: `[project-identifier]/`
2. Add standard files: README.md, brd.md, prd.md, tech-docs.md, delivery.md
3. Add the plan to this list
