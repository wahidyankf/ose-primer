import { describe, it, expect, afterEach } from "vitest";
import { Effect } from "effect";
import { loadConfig } from "../../src/config.js";

describe("loadConfig", () => {
  afterEach(() => {
    delete process.env["CRUD_BE_TS_EFFECT_JWT_SECRET"];
    delete process.env["CRUD_BE_TS_EFFECT_PORT"];
  });

  it("returns default config when env vars are not set", async () => {
    const config = await Effect.runPromise(loadConfig());
    expect(config.databaseUrl).toBe("sqlite::memory:");
    expect(config.jwtSecret).toBe("dev-jwt-secret-at-least-32-chars-long");
    expect(config.port).toBe(8201);
  });

  it("returns an object with required keys", async () => {
    const config = await Effect.runPromise(loadConfig());
    expect(config).toHaveProperty("databaseUrl");
    expect(config).toHaveProperty("jwtSecret");
    expect(config).toHaveProperty("port");
  });

  it("reads CRUD_BE_TS_EFFECT_JWT_SECRET and CRUD_BE_TS_EFFECT_PORT from the environment", async () => {
    process.env["CRUD_BE_TS_EFFECT_JWT_SECRET"] = "test-secret-value-at-least-32-characters-long";
    process.env["CRUD_BE_TS_EFFECT_PORT"] = "9999";

    const config = await Effect.runPromise(loadConfig());

    expect(config.jwtSecret).toBe("test-secret-value-at-least-32-characters-long");
    expect(config.port).toBe(9999);
  });
});
