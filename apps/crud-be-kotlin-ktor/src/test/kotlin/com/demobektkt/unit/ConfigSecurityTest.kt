package com.demobektkt.unit

import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Test
import java.nio.file.Files
import java.nio.file.Path

/**
 * RED: Application.kt uses `?: "dev-jwt-secret-..."` so missing var falls back silently.
 * GREEN will replace with `error(...)` / `requireNotNull(...)` so absent var terminates startup.
 */
class ConfigSecurityTest {

    @Test
    fun `Application_kt has no soft default for jwt secret`() {
        val src = Files.readString(
            Path.of("src/main/kotlin/com/demobektkt/Application.kt")
        )
        assertFalse(
            src.contains("?: \"dev-jwt-secret"),
            "Application.kt must not use a soft default for JWT_SECRET — " +
                "absent var must call error() or requireNotNull(), not fall back"
        )
    }
}
