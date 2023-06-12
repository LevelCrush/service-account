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
    url: '#',
    name: 'Tools',
    children: [
      {
        url: 'https://destinyitemmanager.com/en/',
        name: 'Destiny Item Manager',
      },
      {
        url: 'https://todayindestiny.com/',
        name: 'Today In Destiny',
      },
      {
        url: 'https://app.mobalytics.gg/destiny-2',
        name: 'Destiny 2 Mobalytics',
      },
      {
        url: 'https://destinyrecipes.com/',
        name: 'Destiny Recipes',
      },
      {
        url: 'https://d2armorpicker.com/',
        name: 'D2ArmorPicker',
      },
      {
        url: 'https://engram.blue/crafting',
        name: 'Engram.blue',
      },
    ],
  },
  {
    url: '/guides',
    name: 'Guides',
    children: [
      {
        url: '/guides',
        name: 'Guides',
      },
      {
        url: '/guides/destiny2/votd',
        name: 'Destiny 2 - VOTD',
      },
    ],
  },
  {
    url: '/profile',
    name: 'Profile',
    loginOnly: true,
    pullMenuOnly: true,
  },
  {
    url: '/admin',
    name: 'Admin Dashboard',
    loginOnly: true,
    pullMenuOnly: true,
    adminOnly: true,
  },
  {
    url:
      ENV.hosts.accounts +
      '/logout?redirect=' +
      encodeURIComponent(ENV.hosts.frontend),
    name: 'Logout',
    loginOnly: true,
    pullMenuOnly: true,
  },
] as RouteItem[];

export default Routes;
