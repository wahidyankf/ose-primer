<!-- Knowledge Capture running log — append entries during execution. -->
<!-- Triage every entry (or record the explicit "none" escape) before archival. -->

# Learnings: crud-kotlin-codegen-jdk-fix

## Learning: chained gradlew sub-commands silently drop the JAVA_HOME override

- **Context**: Root-causing why `nx run crud-be-kotlin-ktor:typecheck` failed at its `codegen` step
  on a machine whose ambient JDK is 25 (Temurin), while every other gradle-invoking Nx target in the
  same `project.json` (`build`, `dev`, `test:coverage`, `test:unit`, `lint`, `typecheck`,
  `deps:audit`) succeeded.
- **Observation**: `codegen`'s command is `<openapi-generator-cli> && (cd apps/... && ./gradlew
ktfmtFormatMain)` — the second, chained `./gradlew` invocation has no `JAVA_HOME=` prefix, unlike
  every sibling target's single, un-chained `./gradlew ...` command. It silently inherits whatever
  `JAVA_HOME`/`java` is first on `PATH`. Gradle 8.14's bundled Kotlin-DSL script compiler throws
  `java.lang.IllegalArgumentException: 25` from `JavaVersion.parse` when the ambient JDK is 25,
  because Gradle 8.x's Kotlin-DSL compiler does not recognize that feature-version string (confirmed
  via [gradle/gradle#35111](https://github.com/gradle/gradle/issues/35111) — JDK 25 support lands in
  Gradle 9.1.0+, not 8.x).
- **Why it might generalize**: any Nx target whose `command` chains a second tool invocation with
  `&&` after a JAVA_HOME-prefixed one is a place this same silent-drop can recur — a shell `VAR=val
cmd` prefix binds only to the immediately following simple command, not to anything chained after
  `&&`. An `&&`-appended `./gradlew`/`java`/JVM-tool invocation needs its own explicit `JAVA_HOME=`
  prefix, not a shared one from earlier in the line. This plan's own Phase 1 already audits and fixes
  the one instance of this bug present in the repo (`crud-be-kotlin-ktor` is the only app with any
  `gradlew` invocation in `ose-primer`), so no separate cross-repo or cross-app follow-up is needed at
  authoring time — routed inline, terminal.
