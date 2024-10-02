/** @type {import('next').NextConfig} */
const nextConfig = {
  compiler: {
    styledComponents: true,
  },
  output: "standalone",
  images: {
    remotePatterns: [
      {
        protocol: "https",
        hostname: "lh3.googleusercontent.com",
        port: "",
        pathname: "**",
      },
    ],
  },
};

// If it breaks then we'll come back to it
// async headers() {
//   return [
//     {
//       source: "/(.*)",
//       headers: [
//         {
//           key: "Cross-Origin-Opener-Policy",
//           value: "same-origin", // "same-origin-allow-popups"
//         },
//       ],
//     },
//   ];
// },

export default nextConfig;
