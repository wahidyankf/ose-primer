import { readFile } from "node:fs/promises";
import path from "node:path";
// RED: src/env.ts does not exist yet — this import causes "Cannot find module".
// GREEN will create src/env.ts with @t3-oss/env-nextjs + zod validation of required vars.
import { describe, it, expect } from "vitest";
import { env } from "./env.js";

describe("env", () => {
  it("exports validated env object", () => {
    expect(env).toBeDefined();
  });

  it("exposes CRUD_FS_TS_NEXTJS_JWT_SECRET", () => {
    expect(typeof env.CRUD_FS_TS_NEXTJS_JWT_SECRET).toBe("string");
  });

  it("uses an explicit non-secret public mode at build time", async () => {
    const configurationSource = await readFile(path.resolve(__dirname, "../next.config.ts"), "utf8");

    expect(configurationSource).not.toContain('import "./src/env"');
    expect(configurationSource).toContain("NEXT_PUBLIC_BACKEND_ENABLED");
    expect(configurationSource).not.toContain("CRUD_FS_TS_NEXTJS_JWT_SECRET");
  });
});
