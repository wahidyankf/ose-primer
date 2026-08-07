import { readFile } from "node:fs/promises";
import path from "node:path";
import { describe, it, expect } from "vitest";
import { env } from "./env.js";

describe("env", () => {
  it("exports validated env object", () => {
    expect(env).toBeDefined();
  });

  it("exposes BACKEND_URL", () => {
    expect(typeof env.BACKEND_URL).toBe("string");
  });

  it("loads the environment module during Next.js startup", async () => {
    const configurationSource = await readFile(path.resolve(__dirname, "../next.config.ts"), "utf8");

    expect(configurationSource).toContain('import "./src/env"');
  });
});
