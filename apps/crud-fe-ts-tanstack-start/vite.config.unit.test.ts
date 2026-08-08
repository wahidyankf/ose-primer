// @vitest-environment node
import { describe, expect, it } from "vitest";
import { createViteConfig } from "./vite.config";

describe("createViteConfig", () => {
  it("keeps the frontend-only reference quiet when no backend is configured", () => {
    const config = createViteConfig(undefined);

    expect(config.define).toMatchObject({
      "import.meta.env.VITE_BACKEND_ENABLED": '"false"',
    });
    expect(config.server?.proxy).toBeUndefined();
  });

  it("embeds the configured-backend state and proxy target for a full-stack build", () => {
    const config = createViteConfig("http://crud-be:8201");

    expect(config.define).toMatchObject({
      "import.meta.env.VITE_BACKEND_ENABLED": '"true"',
    });
    expect(config.server?.proxy).toMatchObject({
      "/health": {
        target: "http://crud-be:8201",
        changeOrigin: true,
      },
    });
  });
});
