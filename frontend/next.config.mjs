/** @type {import('next').NextConfig} */
const nextConfig = { compiler: {
    styledComponents: true,
  },
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
  output: "standalone"
};

export default nextConfig;
