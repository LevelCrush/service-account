import ENV from './env';

export interface RouteItem {
  url: string;
  name: string;
  pullMenuOnly?: boolean;
  loginOnly?: boolean;
  adminOnly?: boolean;
  target?: '_blank' | '_self';
  children?: RouteItem[];
}

/** These are the standard routes, intended for top level site navigation  */
export const Routes = [
  {
    url: '/',
    name: 'Home',
    pullMenuOnly: true,
  },
  {
    url: '/clan',
    name: 'Clan',
    children: [
      {
        url: '/clan',
        name: 'Clan',
      },
      {
        url: '/clan/roster',
        name: 'Clan Roster',
      },
    ],
  },
  {
    url: '/admin/network/lifetime/all',
    name: 'Network Report',
  },
] as RouteItem[];

export default Routes;
