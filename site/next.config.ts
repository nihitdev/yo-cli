import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  async headers() {
    return [{
      source: "/apt/:path*",
      headers: [
        { key: "Cache-Control", value: "public, max-age=0, must-revalidate" },
        { key: "X-Content-Type-Options", value: "nosniff" },
      ],
    }];
  },
};

export default nextConfig;
