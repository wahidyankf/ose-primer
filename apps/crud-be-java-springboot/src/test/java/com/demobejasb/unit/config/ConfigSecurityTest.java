package com.demobejasb.unit.config;

import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.assertFalse;

/**
 * RED: application.yml has a soft default for CRUD_BE_JAVA_SPRINGBOOT_JWT_SECRET.
 * GREEN will remove the default so Spring fails to start when the var is absent.
 */
class ConfigSecurityTest {

    @Test
    void applicationYmlHasNoFallbackDefaultForJwtSecret() throws IOException {
        String yml = Files.readString(Path.of("src/main/resources/application.yml"));
        assertFalse(
            yml.contains(":change-me-in-production"),
            "application.yml must not contain a default fallback for jwt secret — "
                + "absent var must prevent startup"
        );
    }
}
