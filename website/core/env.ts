export const ENV = {
  isBrowser: typeof window !== 'undefined',
  hosts: {
    frontend: process.env['NEXT_PUBLIC_HOST_FRONTEND'] || '',
    accounts: process.env['NEXT_PUBLIC_HOST_ACCOUNTS'] || '',
    feed: process.env['NEXT_PUBLIC_HOST_FEED'] || '',
    assets: process.env['NEXT_PUBLIC_HOST_ASSETS'] || '',
    destiny: process.env['NEXT_PUBLIC_HOST_DESTINY'] || '',
  },
  feed: {
    public_key: process.env['NEXT_PUBLIC_FEED_ACCESS_KEY'] || '',
    private_key: process.env['FEED_PRIVATE_KEY'] || '',
  },
};

export default ENV;
