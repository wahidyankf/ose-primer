import "./src/env";
import type { NextConfig } from "next";

const backendUrl = process.env.BACKEND_URL;

const nextConfig: NextConfig = {
  output: "standalone",
  allowedDevOrigins: ["127.0.0.1"],
  env: {
    NEXT_PUBLIC_BACKEND_ENABLED: backendUrl ? "true" : "false",
  },
  async rewrites() {
    if (!backendUrl) return [];

    return [
      { source: "/api/:path*", destination: `${backendUrl}/api/:path*` },
      { source: "/health", destination: `${backendUrl}/health` },
      {
        source: "/.well-known/:path*",
        destination: `${backendUrl}/.well-known/:path*`,
      },
    ];
  },
};

export default nextConfig;
