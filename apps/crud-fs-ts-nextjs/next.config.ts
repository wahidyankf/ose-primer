import "./src/env.js";
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  output: "standalone",
  turbopack: {
    root: "../../",
  },
};

export default nextConfig;
