// RED: src/env.ts does not exist yet — this import causes "Cannot find module".
// GREEN will create src/env.ts with @t3-oss/env-nextjs + zod validation of BACKEND_URL.
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
