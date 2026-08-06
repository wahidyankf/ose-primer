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
