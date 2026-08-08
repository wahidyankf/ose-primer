# Kotlin/Gradle JDK toolchain convergence

One-line summary: converge `crud-be-kotlin-ktor`'s ad hoc per-target `JAVA_HOME_21_X64` pinning into
either a repo-wide asdf JDK pin or a Gradle 9.1+ bump, so JDK selection stops depending on manually
mirroring an env-var prefix onto every new gradle-invoking Nx target.

> Idea, added 2026-08-08. Spun out of the
> [`crud-kotlin-codegen-jdk-fix`](../../backlog/crud-kotlin-codegen-jdk-fix/README.md) backlog plan,
> which applies only the minimal parity fix for the one reproduced bug (a missing `JAVA_HOME`
> override on `codegen`'s chained `./gradlew ktfmtFormatMain`).

## Problem / context

`apps/crud-be-kotlin-ktor/project.json` pins every `./gradlew`-invoking Nx target to JDK 21 via a
repeated inline snippet:
`JAVA_HOME=${JAVA_HOME_21_X64:-$(ls -d ${SDKMAN_DIR:-$HOME/.sdkman}/candidates/java/21* 2>/dev/null | head -1)}`.
This exists because Gradle 8.14 (the pinned wrapper version, per
`apps/crud-be-kotlin-ktor/gradle/wrapper/gradle-wrapper.properties`) cannot run its bundled
Kotlin-DSL script compiler on JDK 25 — it throws `java.lang.IllegalArgumentException: 25` from
`JavaVersion.parse` (reproduced and confirmed 2026-08-08; root cause documented in the linked
backlog plan). The repo has no repo-wide JDK pin (`.tool-versions` pins only `erlang`/`elixir`), so
this snippet is the only thing standing between "works" and "breaks" for anyone whose ambient JDK is
newer than 21 — and it has to be copy-pasted onto every new gradlew-invoking target by hand, which is
exactly how the `codegen` target's chained `ktfmtFormatMain` call ended up missing it.

## Why now

Not urgent: the acute bug has a confirmed, shipped minimal fix. But the underlying fragility (a
hand-copied env-var snippet, one omission away from silently breaking on the next contributor's
newer default JDK) is a standing footgun that will resurface the next time someone adds a
gradle-invoking target or a teammate's machine defaults to JDK 26+.

## Prior art / precedents

- [gradle/gradle#35111 — Support Java 25 on Gradle 8](https://github.com/gradle/gradle/issues/35111)
  — confirms Gradle 9.1.0+ is the first version with JDK 25 support; Gradle 8.x tops out at JDK 24.
- [Gradle Compatibility Matrix](https://docs.gradle.org/current/userguide/compatibility.html) — the
  authoritative JVM-version-vs-Gradle-version table to re-check before picking a target Gradle
  version.
- **asdf** (`.tool-versions`, already used repo-wide for `erlang`/`elixir`) — the repo's own existing
  convention for pinning a language runtime via a plugin, rather than an inline shell snippet.
- [`crud-kotlin-codegen-jdk-fix`](../../backlog/crud-kotlin-codegen-jdk-fix/README.md) — the backlog
  plan that reproduced and fixed the acute instance of this class of bug.

## Proposed direction (sketch)

Two candidate convergence paths, not yet chosen between:

- **asdf JDK pin**: add a `java <version>` line to `.tool-versions` (asdf has a `java` plugin) so
  JDK 21 resolves automatically for any shell that has `direnv`/asdf hooked in, and drop the
  `JAVA_HOME_21_X64` fallback snippet from every Nx target once contributors' shells resolve it via
  asdf instead. Smaller version-compatibility surface, but shifts the dependency onto asdf shell
  integration being present.
- **Gradle 9.1+ bump**: upgrade the Gradle wrapper so it runs natively on any JDK from 17 through
  whatever ceiling Gradle 9.x supports, removing the JAVA_HOME pinning need entirely. Larger surface
  — needs its own compatibility pass across the Kotlin DSL, the `io.ktor.plugin`, Kover, detekt, and
  `com.ncorti.ktfmt.gradle` plugin versions currently pinned in `build.gradle.kts`.

## Rough scope & non-goals

In scope: choosing one convergence path (or documenting why neither is worth it yet) for JDK
selection across all `crud-be-kotlin-ktor` gradle-invoking Nx targets.

Out of scope (for now): any other app's toolchain; re-litigating the JDK 21 compile target itself
(`sourceCompatibility`/`jvmTarget` in `build.gradle.kts` stay at 21 either way — this idea is only
about which JDK runs Gradle, not which JDK the compiled bytecode targets).

## Risks & open questions

- Does every contributor's shell actually source asdf/direnv, or would an asdf pin be silently
  ignored on some machines the same way the current env-var snippet already is? (open)
- Does Gradle 9.1+ break any of the currently-pinned plugin versions (`io.ktor.plugin` 3.4.1, Kover
  0.9.1, detekt 1.23.8, ktfmt-gradle 0.22.0)? Needs its own verification pass. (open)
- Is this worth doing at all before the next JDK-related breakage actually happens, given the acute
  bug already has a working fix? (open — this is exactly the "not urgent" judgment call)

## What success looks like + promotion signal

Success: JDK selection for `crud-be-kotlin-ktor` no longer depends on a hand-copied inline env-var
snippet being present on every gradle-invoking Nx target. Ready to promote to a `backlog/` plan once
one of the two paths is chosen (or a third option surfaces) and the plugin-compatibility risk for a
Gradle 9 bump (if chosen) has been checked against the actual pinned versions.
