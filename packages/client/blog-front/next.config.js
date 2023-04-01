/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    appDir: true,
  },
  async redirects() {
    return [
      {
        source: '/test',
        destination: '/test/redirect',
        permanent: true,
      },
    ]
  },
}

module.exports = nextConfig
