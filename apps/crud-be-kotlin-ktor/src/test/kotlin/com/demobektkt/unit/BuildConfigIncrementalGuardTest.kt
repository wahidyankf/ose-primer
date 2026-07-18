package com.demobektkt.unit

import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import java.nio.file.Files
import java.nio.file.Path

/**
 * Regression guard for the CI flakiness fixed by setting `kotlin.incremental=false`.
 *
 * The main-ci "JVM quality gate" intermittently failed with a Kotlin daemon
 * `AssertionError: Could not close incremental caches in .../caches-jvm` during
 * `:compileKotlin`. The root cause is Kotlin incremental compilation, which yields
 * no benefit on CI (every build runs from a fresh checkout) yet is the sole source
 * of the non-deterministic cache-close race.
 *
 * A race-reproducing test is infeasible (a green run proves nothing), so this guard
 * instead pins the deterministic remedy: if `kotlin.incremental=false` is ever
 * silently removed from gradle.properties, this test fails deterministically rather
 * than letting the flaky failure quietly return. See the Regression Test Mandate
 * (repo-governance/development/quality/regression-test-mandate.md): the test form
 * adapts to the defect type — for a build-determinism config fix, a config-presence
 * guard makes the fix "impossible to silently reintroduce".
 */
class BuildConfigIncrementalGuardTest {

    @Test
    fun `gradle_properties keeps Kotlin incremental compilation disabled`() {
        val props = Files.readString(Path.of("gradle.properties"))
        assertTrue(
            props.lineSequence().any { it.trim() == "kotlin.incremental=false" },
            "gradle.properties must set `kotlin.incremental=false` to keep the CI " +
                "Kotlin build deterministic (avoids the flaky 'Could not close " +
                "incremental caches' daemon failure). Do not remove this property.",
        )
    }
}
