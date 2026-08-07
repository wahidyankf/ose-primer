import { defineConfig, type PluginOption } from "vite";
import react from "@vitejs/plugin-react";
import { TanStackRouterVite } from "@tanstack/router-vite-plugin";
import tsconfigPaths from "vite-tsconfig-paths";

export function createViteConfig(backendUrl = process.env.BACKEND_URL) {
  return defineConfig({
    define: {
      "import.meta.env.VITE_BACKEND_ENABLED": JSON.stringify(backendUrl ? "true" : "false"),
    },
    plugins: [
      TanStackRouterVite({
        routesDirectory: "./src/routes",
        generatedRouteTree: "./src/routeTree.gen.ts",
      }) as PluginOption,
      react() as PluginOption,
      tsconfigPaths() as PluginOption,
    ],
    server: {
      proxy: backendUrl
        ? {
            "/api": {
              target: backendUrl,
              changeOrigin: true,
            },
            "/health": {
              target: backendUrl,
              changeOrigin: true,
            },
            "/.well-known": {
              target: backendUrl,
              changeOrigin: true,
            },
          }
        : undefined,
    },
  });
}

export default createViteConfig();
