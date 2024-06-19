/** @type {import('next').NextConfig} */
const nextConfig = {
  webpack: (config, { isServer }) => {
    config.module.rules.push({
      test: /\.ts$/,
      exclude: /public\/OnceAndOnceAgain\/code\//,
      use: 'ignore-loader'
    });

    return config;
  },
};

export default nextConfig;
