package com.demobejasb;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

/** Entry point for the demo Spring Boot application. */
@SpringBootApplication
public final class ADemoApplication {

    private ADemoApplication() {
    }

    /**
     * Starts the Spring Boot application.
     *
     * @param args command-line arguments passed to the application
     */
    public static void main(final String[] args) {
        SpringApplication.run(ADemoApplication.class, args);
    }
}
