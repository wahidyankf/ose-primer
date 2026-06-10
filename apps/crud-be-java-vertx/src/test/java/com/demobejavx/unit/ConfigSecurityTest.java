package com.demobejavx.unit;

import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.assertFalse;

/**
 * RED: MainVerticle.java uses getOrDefault(..., DEFAULT_JWT_SECRET) so missing var falls back silently.
 * GREEN will replace with an explicit null-check that throws on missing CRUD_BE_JAVA_VERTX_JWT_SECRET.
 */
class ConfigSecurityTest {

    @Test
    void mainVerticleHasNoSoftDefaultForJwtSecret() throws IOException {
        String src = Files.readString(
            Path.of("src/main/java/com/demobejavx/MainVerticle.java")
        );
        assertFalse(
            src.contains("getOrDefault(\"CRUD_BE_JAVA_VERTX_JWT_SECRET\""),
            "MainVerticle must not use getOrDefault for JWT_SECRET — "
                + "absent var must throw, not silently fall back"
        );
    }
}
