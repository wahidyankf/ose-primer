// RED: src/env.ts does not exist yet — this import causes "Cannot find module".
// GREEN will create src/env.ts with zod validation of BACKEND_URL (and other public vars).
import { describe, it, expect } from "vitest";
import { env } from "./env.js";

describe("env", () => {
  it("exports validated env object", () => {
    expect(env).toBeDefined();
  });

  it("exposes BACKEND_URL", () => {
    expect(typeof env.BACKEND_URL).toBe("string");
  });
});
