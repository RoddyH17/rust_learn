import { withContentCollections } from "@content-collections/next";

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  // Static export for GitHub Pages, served at roddyh17.github.io/rust_learn
  output: "export",
  basePath: "/rust_learn",
  images: {
    unoptimized: true,
  },
  turbopack: {
    root: import.meta.dirname,
  },
};

// withContentCollections must be the outermost plugin
export default withContentCollections(nextConfig);
