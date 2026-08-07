import type { NextConfig } from "next";

const backendEnabled = process.env.NEXT_PUBLIC_BACKEND_ENABLED === "true";

const nextConfig: NextConfig = {
  output: "standalone",
  env: {
    NEXT_PUBLIC_BACKEND_ENABLED: backendEnabled ? "true" : "false",
  },
  turbopack: {
    root: "../../",
  },
};

export default nextConfig;
