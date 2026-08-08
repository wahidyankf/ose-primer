import { afterEach, describe, expect, it, vi } from "vitest";

async function loadBackendMode(value: string | undefined) {
  vi.resetModules();
  if (value === undefined) {
    vi.unstubAllEnvs();
  } else {
    vi.stubEnv("NEXT_PUBLIC_BACKEND_ENABLED", value);
  }

  const { default: config } = await import("./next.config.js");
  return config.env?.NEXT_PUBLIC_BACKEND_ENABLED;
}

describe("next.config", () => {
  afterEach(() => {
    vi.unstubAllEnvs();
  });

  it("defaults the built client to the safe frontend-only mode", async () => {
    await expect(loadBackendMode(undefined)).resolves.toBe("false");
  });

  it("enables built-client health only with the explicit public build flag", async () => {
    await expect(loadBackendMode("true")).resolves.toBe("true");
  });
});
