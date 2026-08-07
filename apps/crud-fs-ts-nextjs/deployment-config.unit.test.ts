import { readFile } from "node:fs/promises";
import path from "node:path";
import { describe, expect, it } from "vitest";

const projectRoot = __dirname;

describe("full-stack frontend deployment mode", () => {
  it("keeps the default image frontend-only without exposing a credential", async () => {
    const dockerfile = await readFile(path.join(projectRoot, "Dockerfile"), "utf8");

    expect(dockerfile).toContain("ARG NEXT_PUBLIC_BACKEND_ENABLED=false");
    expect(dockerfile).toContain("ENV NEXT_PUBLIC_BACKEND_ENABLED=$NEXT_PUBLIC_BACKEND_ENABLED");
    expect(dockerfile).not.toContain("CRUD_FS_TS_NEXTJS_JWT_SECRET");
  });

  it("lets the full-stack Compose consumer opt into built-client health", async () => {
    const composePath = path.resolve(projectRoot, "../../infra/dev/crud-fs-ts-nextjs/docker-compose.yml");
    const compose = await readFile(composePath, "utf8");

    expect(compose).toContain('NEXT_PUBLIC_BACKEND_ENABLED: "true"');
  });
});
