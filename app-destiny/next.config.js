/** @type {import('next').NextConfig} */

module.exports = {
  output: 'export',
  reactStrictMode: true,
  images: {
    domains: ['assets.levelcrush.com', 'assets.levelcrush.local', 'http.cat'],
  },
  optimizeFonts: false,
};
