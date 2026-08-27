/** @type {import('next').NextConfig} */
const nextConfig = {
  // Standalone output keeps the production Docker image lean (no full
  // node_modules copy needed) — see Dockerfile.
  output: "standalone",
};

export default nextConfig;
