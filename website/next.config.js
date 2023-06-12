/** @type {import('next').NextConfig} */

module.exports = {
  reactStrictMode: true,
  images: {
    domains: ['assets.levelcrush.com', 'assets.levelcrush.local', 'http.cat'],
  },
  optimizeFonts: false,
  experimental: { images: { allowFutureImage: true } },
  async redirects() {
    return [
      {
        source: '/signups',
        destination: '/signup',
        permanent: true,
      },
      {
        source: '/tournament',
        destination: '/tournament/matchups',
        permanent: false,
      },

      {
        source: '/tournament/matchup',
        destination: '/tournament/matchups',
        permanent: false,
      },
      {
        source: '/tournament/rule',
        destination: '/tournament/rules',
        permanent: false,
      },
      {
        source: '/guides',
        destination: '/guides/destiny2/votd',
        permanent: false,
      },
      {
        source: '/guides/destiny2',
        destination: '/guides/destiny2/votd',
        permanent: false,
      },
    ];
  },
};
