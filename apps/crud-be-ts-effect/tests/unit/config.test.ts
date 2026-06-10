import { describe, it, expect, afterEach } from "vitest";
import { Effect, Exit } from "effect";
import { loadConfig } from "../../src/config.js";

describe("loadConfig", () => {
  afterEach(() => {
    delete process.env["CRUD_BE_TS_EFFECT_JWT_SECRET"];
    delete process.env["CRUD_BE_TS_EFFECT_PORT"];
  });

  it("returns an object with required keys when secret is set", async () => {
    process.env["CRUD_BE_TS_EFFECT_JWT_SECRET"] = "test-secret-value-at-least-32-characters-long";
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

  it("fails when CRUD_BE_TS_EFFECT_JWT_SECRET is not set", async () => {
    delete process.env["CRUD_BE_TS_EFFECT_JWT_SECRET"];
    const exit = await Effect.runPromiseExit(loadConfig());
    expect(Exit.isFailure(exit)).toBe(true);
  });
});
