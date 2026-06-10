import { describe, it, expect, afterEach } from "vitest";
import { signAccessToken } from "./jwt";

describe("jwt - CRUD_FS_TS_NEXTJS_JWT_SECRET", () => {
  const SECRET_KEY = "CRUD_FS_TS_NEXTJS_JWT_SECRET";

  afterEach(() => {
    delete process.env[SECRET_KEY];
  });

  it("does not throw when CRUD_FS_TS_NEXTJS_JWT_SECRET is set", async () => {
    process.env[SECRET_KEY] = "test-secret-at-least-32-chars-long!";
    await expect(signAccessToken("user-1", "alice", "USER")).resolves.toBeTypeOf("string");
  });

  it("throws when CRUD_FS_TS_NEXTJS_JWT_SECRET is not set", async () => {
    delete process.env[SECRET_KEY];
    await expect(signAccessToken("user-1", "alice", "USER")).rejects.toThrow("CRUD_FS_TS_NEXTJS_JWT_SECRET is not set");
  });
});
